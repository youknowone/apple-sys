#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use libc::{id_t, pid_t, size_t};

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct Float32Point {
    pub x: Float32,
    pub y: Float32,
}
pub type URefCon = *mut ::std::os::raw::c_void;
pub type Str32 = [::std::os::raw::c_uchar; 33usize];
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ProcessSerialNumber {
    pub highLongOfPSN: UInt32,
    pub lowLongOfPSN: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FixedPoint {
    pub x: Fixed,
    pub y: Fixed,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NumVersion {
    pub nonRelRev: UInt8,
    pub stage: UInt8,
    pub minorAndBugRev: UInt8,
    pub majorRev: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cups_array_s {
    _unused: [u8; 0],
}
pub type cups_array_t = _cups_array_s;
pub type ppd_ui_e = ::std::os::raw::c_uint;
pub use self::ppd_ui_e as ppd_ui_t;
pub type ppd_section_e = ::std::os::raw::c_uint;
pub use self::ppd_section_e as ppd_section_t;
pub type ppd_cs_e = ::std::os::raw::c_int;
pub use self::ppd_cs_e as ppd_cs_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_attr_s {
    pub name: [::std::os::raw::c_char; 41usize],
    pub spec: [::std::os::raw::c_char; 41usize],
    pub text: [::std::os::raw::c_char; 81usize],
    pub value: *mut ::std::os::raw::c_char,
}
pub type ppd_attr_t = ppd_attr_s;
pub type ppd_option_t = ppd_option_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_choice_s {
    pub marked: ::std::os::raw::c_char,
    pub choice: [::std::os::raw::c_char; 41usize],
    pub text: [::std::os::raw::c_char; 81usize],
    pub code: *mut ::std::os::raw::c_char,
    pub option: *mut ppd_option_t,
}
pub type ppd_choice_t = ppd_choice_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_option_s {
    pub conflicted: ::std::os::raw::c_char,
    pub keyword: [::std::os::raw::c_char; 41usize],
    pub defchoice: [::std::os::raw::c_char; 41usize],
    pub text: [::std::os::raw::c_char; 81usize],
    pub ui: ppd_ui_t,
    pub section: ppd_section_t,
    pub order: f32,
    pub num_choices: ::std::os::raw::c_int,
    pub choices: *mut ppd_choice_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_group_s {
    pub text: [::std::os::raw::c_char; 40usize],
    pub name: [::std::os::raw::c_char; 41usize],
    pub num_options: ::std::os::raw::c_int,
    pub options: *mut ppd_option_t,
    pub num_subgroups: ::std::os::raw::c_int,
    pub subgroups: *mut ppd_group_s,
}
pub type ppd_group_t = ppd_group_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_const_s {
    pub option1: [::std::os::raw::c_char; 41usize],
    pub choice1: [::std::os::raw::c_char; 41usize],
    pub option2: [::std::os::raw::c_char; 41usize],
    pub choice2: [::std::os::raw::c_char; 41usize],
}
pub type ppd_const_t = ppd_const_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_size_s {
    pub marked: ::std::os::raw::c_int,
    pub name: [::std::os::raw::c_char; 41usize],
    pub width: f32,
    pub length: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}
pub type ppd_size_t = ppd_size_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_emul_s {
    pub name: [::std::os::raw::c_char; 41usize],
    pub start: *mut ::std::os::raw::c_char,
    pub stop: *mut ::std::os::raw::c_char,
}
pub type ppd_emul_t = ppd_emul_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_profile_s {
    pub resolution: [::std::os::raw::c_char; 41usize],
    pub media_type: [::std::os::raw::c_char; 41usize],
    pub density: f32,
    pub gamma: f32,
    pub matrix: [[f32; 3usize]; 3usize],
}
pub type ppd_profile_t = ppd_profile_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ppd_cache_s {
    _unused: [u8; 0],
}
pub type _ppd_cache_t = _ppd_cache_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ppd_file_s {
    pub language_level: ::std::os::raw::c_int,
    pub color_device: ::std::os::raw::c_int,
    pub variable_sizes: ::std::os::raw::c_int,
    pub accurate_screens: ::std::os::raw::c_int,
    pub contone_only: ::std::os::raw::c_int,
    pub landscape: ::std::os::raw::c_int,
    pub model_number: ::std::os::raw::c_int,
    pub manual_copies: ::std::os::raw::c_int,
    pub throughput: ::std::os::raw::c_int,
    pub colorspace: ppd_cs_t,
    pub patches: *mut ::std::os::raw::c_char,
    pub num_emulations: ::std::os::raw::c_int,
    pub emulations: *mut ppd_emul_t,
    pub jcl_begin: *mut ::std::os::raw::c_char,
    pub jcl_ps: *mut ::std::os::raw::c_char,
    pub jcl_end: *mut ::std::os::raw::c_char,
    pub lang_encoding: *mut ::std::os::raw::c_char,
    pub lang_version: *mut ::std::os::raw::c_char,
    pub modelname: *mut ::std::os::raw::c_char,
    pub ttrasterizer: *mut ::std::os::raw::c_char,
    pub manufacturer: *mut ::std::os::raw::c_char,
    pub product: *mut ::std::os::raw::c_char,
    pub nickname: *mut ::std::os::raw::c_char,
    pub shortnickname: *mut ::std::os::raw::c_char,
    pub num_groups: ::std::os::raw::c_int,
    pub groups: *mut ppd_group_t,
    pub num_sizes: ::std::os::raw::c_int,
    pub sizes: *mut ppd_size_t,
    pub custom_min: [f32; 2usize],
    pub custom_max: [f32; 2usize],
    pub custom_margins: [f32; 4usize],
    pub num_consts: ::std::os::raw::c_int,
    pub consts: *mut ppd_const_t,
    pub num_fonts: ::std::os::raw::c_int,
    pub fonts: *mut *mut ::std::os::raw::c_char,
    pub num_profiles: ::std::os::raw::c_int,
    pub profiles: *mut ppd_profile_t,
    pub num_filters: ::std::os::raw::c_int,
    pub filters: *mut *mut ::std::os::raw::c_char,
    pub flip_duplex: ::std::os::raw::c_int,
    pub protocols: *mut ::std::os::raw::c_char,
    pub pcfilename: *mut ::std::os::raw::c_char,
    pub num_attrs: ::std::os::raw::c_int,
    pub cur_attr: ::std::os::raw::c_int,
    pub attrs: *mut *mut ppd_attr_t,
    pub sorted_attrs: *mut cups_array_t,
    pub options: *mut cups_array_t,
    pub coptions: *mut cups_array_t,
    pub marked: *mut cups_array_t,
    pub cups_uiconstraints: *mut cups_array_t,
    pub cache: *mut _ppd_cache_t,
}
pub type ATSPoint = CGPoint;
pub type FMGeneration = UInt32;
pub type FMFontFamily = SInt16;
pub type FMFontStyle = SInt16;
pub type FMFontSize = SInt16;
pub type FMFont = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FMFontFamilyInstance {
    pub fontFamily: FMFontFamily,
    pub fontStyle: FMFontStyle,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FMFontFamilyIterator {
    pub reserved: [UInt32; 16usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FMFontIterator {
    pub reserved: [UInt32; 16usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FMFontFamilyInstanceIterator {
    pub reserved: [UInt32; 16usize],
}
pub type FMFilterSelector = UInt32;
pub type ATSOptionFlags = OptionBits;
pub type ATSGeneration = UInt32;
pub type ATSFontContainerRef = UInt32;
pub type ATSFontFamilyRef = UInt32;
pub type ATSGlyphRef = UInt16;
pub type ATSFontSize = CGFloat;
pub type ATSFontFormat = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFontMetrics {
    pub version: UInt32,
    pub ascent: CGFloat,
    pub descent: CGFloat,
    pub leading: CGFloat,
    pub avgAdvanceWidth: CGFloat,
    pub maxAdvanceWidth: CGFloat,
    pub minLeftSideBearing: CGFloat,
    pub minRightSideBearing: CGFloat,
    pub stemWidth: CGFloat,
    pub stemHeight: CGFloat,
    pub capHeight: CGFloat,
    pub xHeight: CGFloat,
    pub italicAngle: CGFloat,
    pub underlinePosition: CGFloat,
    pub underlineThickness: CGFloat,
}
pub type ATSCurveType = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSUCurvePath {
    pub vectors: UInt32,
    pub controlBits: [UInt32; 1usize],
    pub vector: [ATSPoint; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSUCurvePaths {
    pub contours: UInt32,
    pub contour: [ATSUCurvePath; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSGlyphIdealMetrics {
    pub advance: ATSPoint,
    pub sideBearing: ATSPoint,
    pub otherSideBearing: ATSPoint,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSGlyphScreenMetrics {
    pub deviceAdvance: ATSPoint,
    pub topLeft: ATSPoint,
    pub height: UInt32,
    pub width: UInt32,
    pub sideBearing: ATSPoint,
    pub otherSideBearing: ATSPoint,
}
pub type GlyphID = ATSGlyphRef;
pub type ATSULayoutOperationSelector = UInt32;
pub type ATSULayoutOperationCallbackStatus = UInt32;
pub type ATSLineLayoutOptions = UInt32;
pub type ATSStyleRenderingOptions = UInt32;
pub type ATSGlyphInfoFlags = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSLayoutRecord {
    pub glyphID: ATSGlyphRef,
    pub flags: ATSGlyphInfoFlags,
    pub originalOffset: ByteCount,
    pub realPos: Fixed,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ATSTrapezoid {
    pub upperLeft: FixedPoint,
    pub upperRight: FixedPoint,
    pub lowerRight: FixedPoint,
    pub lowerLeft: FixedPoint,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSJustWidthDeltaEntryOverride {
    pub beforeGrowLimit: Fixed,
    pub beforeShrinkLimit: Fixed,
    pub afterGrowLimit: Fixed,
    pub afterShrinkLimit: Fixed,
    pub growFlags: JustificationFlags,
    pub shrinkFlags: JustificationFlags,
}
pub type ATSFontContext = UInt32;
pub type ATSFontFamilyApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(
        iFamily: ATSFontFamilyRef,
        iRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type ATSFontApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(iFont: ATSFontRef, iRefCon: *mut ::std::os::raw::c_void) -> OSStatus,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ATSFontFamilyIterator_ {
    _unused: [u8; 0],
}
pub type ATSFontFamilyIterator = *mut ATSFontFamilyIterator_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ATSFontIterator_ {
    _unused: [u8; 0],
}
pub type ATSFontIterator = *mut ATSFontIterator_;
pub type ATSFontFilterSelector = ::std::os::raw::c_uint;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct ATSFontFilter {
    pub __bindgen_anon_1: ATSFontFilter__bindgen_ty_1,
    pub version: UInt32,
    pub filterSelector: ATSFontFilterSelector,
    pub filter: ATSFontFilter__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union ATSFontFilter__bindgen_ty_1 {
    pub generationFilter: ATSGeneration,
    pub fontFamilyFilter: ATSFontFamilyRef,
    pub fontFamilyApplierFunctionFilter: ATSFontFamilyApplierFunction,
    pub fontApplierFunctionFilter: ATSFontApplierFunction,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ATSFontNotificationRef_ {
    _unused: [u8; 0],
}
pub type ATSFontNotificationRef = *mut ATSFontNotificationRef_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ATSFontNotificationInfoRef_ {
    _unused: [u8; 0],
}
pub type ATSFontNotificationInfoRef = *mut ATSFontNotificationInfoRef_;
pub type ATSFontNotifyOption = ::std::os::raw::c_uint;
pub type ATSFontNotifyAction = ::std::os::raw::c_uint;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFontQuerySourceContext {
    pub version: UInt32,
    pub refCon: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
}
pub type ATSFontQueryMessageID = ::std::os::raw::c_uint;
pub type ATSFontAutoActivationSetting = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMDateTime {
    pub year: UInt16,
    pub month: UInt16,
    pub dayOfTheMonth: UInt16,
    pub hours: UInt16,
    pub minutes: UInt16,
    pub seconds: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMFixedXYColor {
    pub x: Fixed,
    pub y: Fixed,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMFixedXYZColor {
    pub X: Fixed,
    pub Y: Fixed,
    pub Z: Fixed,
}
pub type CMXYZComponent = UInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMXYZColor {
    pub X: CMXYZComponent,
    pub Y: CMXYZComponent,
    pub Z: CMXYZComponent,
}
pub type CMProfileMD5 = [::std::os::raw::c_uchar; 16usize];
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CM2Header {
    pub size: UInt32,
    pub CMMType: OSType,
    pub profileVersion: UInt32,
    pub profileClass: OSType,
    pub dataColorSpace: OSType,
    pub profileConnectionSpace: OSType,
    pub dateTime: CMDateTime,
    pub CS2profileSignature: OSType,
    pub platform: OSType,
    pub flags: UInt32,
    pub deviceManufacturer: OSType,
    pub deviceModel: UInt32,
    pub deviceAttributes: [UInt32; 2usize],
    pub renderingIntent: UInt32,
    pub white: CMFixedXYZColor,
    pub creator: OSType,
    pub reserved: [::std::os::raw::c_char; 44usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CM4Header {
    pub size: UInt32,
    pub CMMType: OSType,
    pub profileVersion: UInt32,
    pub profileClass: OSType,
    pub dataColorSpace: OSType,
    pub profileConnectionSpace: OSType,
    pub dateTime: CMDateTime,
    pub CS2profileSignature: OSType,
    pub platform: OSType,
    pub flags: UInt32,
    pub deviceManufacturer: OSType,
    pub deviceModel: UInt32,
    pub deviceAttributes: [UInt32; 2usize],
    pub renderingIntent: UInt32,
    pub white: CMFixedXYZColor,
    pub creator: OSType,
    pub digest: CMProfileMD5,
    pub reserved: [::std::os::raw::c_char; 28usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMTagRecord {
    pub tag: OSType,
    pub elementOffset: UInt32,
    pub elementSize: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMTagElemTable {
    pub count: UInt32,
    pub tagList: [CMTagRecord; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CM2Profile {
    pub header: CM2Header,
    pub tagTable: CMTagElemTable,
    pub elemData: [::std::os::raw::c_char; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMAdaptationMatrixType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub adaptationMatrix: [Fixed; 9usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMCurveType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub countValue: UInt32,
    pub data: [UInt16; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMDataType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub dataFlag: UInt32,
    pub data: [::std::os::raw::c_char; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMDateTimeType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub dateTime: CMDateTime,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMLut16Type {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub inputChannels: UInt8,
    pub outputChannels: UInt8,
    pub gridPoints: UInt8,
    pub reserved2: UInt8,
    pub matrix: [[Fixed; 3usize]; 3usize],
    pub inputTableEntries: UInt16,
    pub outputTableEntries: UInt16,
    pub inputTable: [UInt16; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMLut8Type {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub inputChannels: UInt8,
    pub outputChannels: UInt8,
    pub gridPoints: UInt8,
    pub reserved2: UInt8,
    pub matrix: [[Fixed; 3usize]; 3usize],
    pub inputTable: [UInt8; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMMultiFunctLutType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub inputChannels: UInt8,
    pub outputChannels: UInt8,
    pub reserved2: UInt16,
    pub offsetBcurves: UInt32,
    pub offsetMatrix: UInt32,
    pub offsetMcurves: UInt32,
    pub offsetCLUT: UInt32,
    pub offsetAcurves: UInt32,
    pub data: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMMultiFunctCLUTType {
    pub gridPoints: [UInt8; 16usize],
    pub entrySize: UInt8,
    pub reserved: [UInt8; 3usize],
    pub data: [UInt8; 2usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMMeasurementType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub standardObserver: UInt32,
    pub backingXYZ: CMFixedXYZColor,
    pub geometry: UInt32,
    pub flare: UInt32,
    pub illuminant: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMNamedColorType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub vendorFlag: UInt32,
    pub count: UInt32,
    pub prefixName: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMNamedColor2EntryType {
    pub rootName: [UInt8; 32usize],
    pub PCSColorCoords: [UInt16; 3usize],
    pub DeviceColorCoords: [UInt16; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMNamedColor2Type {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub vendorFlag: UInt32,
    pub count: UInt32,
    pub deviceChannelCount: UInt32,
    pub prefixName: [UInt8; 32usize],
    pub suffixName: [UInt8; 32usize],
    pub data: [::std::os::raw::c_char; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMNativeDisplayInfo {
    pub dataSize: UInt32,
    pub redPhosphor: CMFixedXYColor,
    pub greenPhosphor: CMFixedXYColor,
    pub bluePhosphor: CMFixedXYColor,
    pub whitePoint: CMFixedXYColor,
    pub redGammaValue: Fixed,
    pub greenGammaValue: Fixed,
    pub blueGammaValue: Fixed,
    pub gammaChannels: UInt16,
    pub gammaEntryCount: UInt16,
    pub gammaEntrySize: UInt16,
    pub gammaData: [::std::os::raw::c_char; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMNativeDisplayInfoType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub nativeDisplayInfo: CMNativeDisplayInfo,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMParametricCurveType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub functionType: UInt16,
    pub reserved2: UInt16,
    pub value: [Fixed; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMTextDescriptionType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub ASCIICount: UInt32,
    pub ASCIIName: [UInt8; 2usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMTextType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub text: [UInt8; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMUnicodeTextType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub text: [UniChar; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMScreeningChannelRec {
    pub frequency: Fixed,
    pub angle: Fixed,
    pub spotFunction: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMScreeningType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub screeningFlag: UInt32,
    pub channelCount: UInt32,
    pub channelInfo: [CMScreeningChannelRec; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMSignatureType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub signature: OSType,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMS15Fixed16ArrayType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub value: [Fixed; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMU16Fixed16ArrayType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub value: [UInt32; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMUInt8ArrayType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub value: [UInt8; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMUInt16ArrayType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub value: [UInt16; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMUInt32ArrayType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub value: [UInt32; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMUInt64ArrayType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub value: [UInt32; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMViewingConditionsType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub illuminant: CMFixedXYZColor,
    pub surround: CMFixedXYZColor,
    pub stdIlluminant: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMXYZType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub XYZ: [CMFixedXYZColor; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMProfileSequenceDescType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub count: UInt32,
    pub data: [::std::os::raw::c_char; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMUcrBgType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub ucrCount: UInt32,
    pub ucrValues: [UInt16; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMIntentCRDVMSize {
    pub renderingIntent: UInt32,
    pub VMSize: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMPS2CRDVMSizeType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub count: UInt32,
    pub intentCRD: [CMIntentCRDVMSize; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMVideoCardGammaTable {
    pub channels: UInt16,
    pub entryCount: UInt16,
    pub entrySize: UInt16,
    pub data: [::std::os::raw::c_char; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMVideoCardGammaFormula {
    pub redGamma: Fixed,
    pub redMin: Fixed,
    pub redMax: Fixed,
    pub greenGamma: Fixed,
    pub greenMin: Fixed,
    pub greenMax: Fixed,
    pub blueGamma: Fixed,
    pub blueMin: Fixed,
    pub blueMax: Fixed,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct CMVideoCardGamma {
    pub __bindgen_anon_1: CMVideoCardGamma__bindgen_ty_1,
    pub tagType: UInt32,
    pub u: CMVideoCardGamma__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CMVideoCardGamma__bindgen_ty_1 {
    pub table: CMVideoCardGammaTable,
    pub formula: CMVideoCardGammaFormula,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct CMVideoCardGammaType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub gamma: CMVideoCardGamma,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMMakeAndModel {
    pub manufacturer: OSType,
    pub model: UInt32,
    pub serialNumber: UInt32,
    pub manufactureDate: UInt32,
    pub reserved1: UInt32,
    pub reserved2: UInt32,
    pub reserved3: UInt32,
    pub reserved4: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMMakeAndModelType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub makeAndModel: CMMakeAndModel,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMMultiLocalizedUniCodeEntryRec {
    pub languageCode: [::std::os::raw::c_char; 2usize],
    pub regionCode: [::std::os::raw::c_char; 2usize],
    pub textLength: UInt32,
    pub textOffset: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CMMultiLocalizedUniCodeType {
    pub typeDescriptor: OSType,
    pub reserved: UInt32,
    pub entryCount: UInt32,
    pub entrySize: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCMProfileRef {
    _unused: [u8; 0],
}
pub type CMProfileRef = *mut OpaqueCMProfileRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCMWorldRef {
    _unused: [u8; 0],
}
pub type CMFlattenProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        command: SInt32,
        size: *mut ::std::os::raw::c_long,
        data: *mut ::std::os::raw::c_void,
        refCon: *mut ::std::os::raw::c_void,
    ) -> OSErr,
>;
pub type CMFlattenUPP = CMFlattenProcPtr;
#[repr(C)]
#[derive(Copy, Clone)]
pub union CMAppleProfileHeader {
    pub cm2: CM2Header,
    pub cm4: CM4Header,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMConcatProfileSet {
    pub keyIndex: UInt16,
    pub count: UInt16,
    pub profileSet: [CMProfileRef; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NCMConcatProfileSpec {
    pub renderingIntent: UInt32,
    pub transformTag: UInt32,
    pub profile: CMProfileRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NCMConcatProfileSet {
    pub cmm: OSType,
    pub flags: UInt32,
    pub flagsMask: UInt32,
    pub profileCount: UInt32,
    pub profileSpecs: [NCMConcatProfileSpec; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMRGBColor {
    pub red: UInt16,
    pub green: UInt16,
    pub blue: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMCMYKColor {
    pub cyan: UInt16,
    pub magenta: UInt16,
    pub yellow: UInt16,
    pub black: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMCMYColor {
    pub cyan: UInt16,
    pub magenta: UInt16,
    pub yellow: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMHLSColor {
    pub hue: UInt16,
    pub lightness: UInt16,
    pub saturation: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMHSVColor {
    pub hue: UInt16,
    pub saturation: UInt16,
    pub value: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMLabColor {
    pub L: UInt16,
    pub a: UInt16,
    pub b: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMLuvColor {
    pub L: UInt16,
    pub u: UInt16,
    pub v: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMYxyColor {
    pub capY: UInt16,
    pub x: UInt16,
    pub y: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMGrayColor {
    pub gray: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMMultichannel5Color {
    pub components: [UInt8; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMMultichannel6Color {
    pub components: [UInt8; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMMultichannel7Color {
    pub components: [UInt8; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMMultichannel8Color {
    pub components: [UInt8; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMNamedColor {
    pub namedColorIndex: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CMColor {
    pub rgb: CMRGBColor,
    pub hsv: CMHSVColor,
    pub hls: CMHLSColor,
    pub XYZ: CMXYZColor,
    pub Lab: CMLabColor,
    pub Luv: CMLuvColor,
    pub Yxy: CMYxyColor,
    pub cmyk: CMCMYKColor,
    pub cmy: CMCMYColor,
    pub gray: CMGrayColor,
    pub mc5: CMMultichannel5Color,
    pub mc6: CMMultichannel6Color,
    pub mc7: CMMultichannel7Color,
    pub mc8: CMMultichannel8Color,
    pub namedColor: CMNamedColor,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMMInfo {
    pub dataSize: usize,
    pub CMMType: OSType,
    pub CMMMfr: OSType,
    pub CMMVersion: UInt32,
    pub ASCIIName: [::std::os::raw::c_uchar; 32usize],
    pub ASCIIDesc: [::std::os::raw::c_uchar; 256usize],
    pub UniCodeNameCount: UniCharCount,
    pub UniCodeName: [UniChar; 32usize],
    pub UniCodeDescCount: UniCharCount,
    pub UniCodeDesc: [UniChar; 256usize],
}
pub type CMBitmapColorSpace = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMBitmap {
    pub image: *mut ::std::os::raw::c_char,
    pub width: usize,
    pub height: usize,
    pub rowBytes: usize,
    pub pixelSize: usize,
    pub space: CMBitmapColorSpace,
    pub user1: UInt32,
    pub user2: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMHandleLocation {
    pub h: Handle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMPathLocation {
    pub path: [::std::os::raw::c_char; 1024usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMBufferLocation {
    pub buffer: *mut ::std::os::raw::c_void,
    pub size: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CMProfLoc {
    pub handleLoc: CMHandleLocation,
    pub pathLoc: CMPathLocation,
    pub bufferLoc: CMBufferLocation,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CMProfileLocation {
    pub locType: ::std::os::raw::c_short,
    pub u: CMProfLoc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CMProfileIterateData {
    pub dataVersion: UInt32,
    pub header: CM2Header,
    pub code: ScriptCode,
    pub name: Str255,
    pub location: CMProfileLocation,
    pub uniCodeNameCount: UniCharCount,
    pub uniCodeName: *mut UniChar,
    pub asciiName: *mut ::std::os::raw::c_uchar,
    pub makeAndModel: *mut CMMakeAndModel,
    pub digest: *mut CMProfileMD5,
}
pub type CMFloatBitmapFlags = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMFloatBitmap {
    pub version: ::std::os::raw::c_ulong,
    pub buffers: [*mut f32; 16usize],
    pub height: usize,
    pub width: usize,
    pub rowStride: isize,
    pub colStride: isize,
    pub space: OSType,
    pub flags: CMFloatBitmapFlags,
}
pub type CMDeviceState = UInt32;
pub type CMDeviceID = UInt32;
pub type CMDeviceProfileID = UInt32;
pub type CMDeviceClass = OSType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMDeviceScope {
    pub deviceUser: CFStringRef,
    pub deviceHost: CFStringRef,
}
pub type CMDeviceProfileScope = CMDeviceScope;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMDeviceInfo {
    pub dataVersion: UInt32,
    pub deviceClass: CMDeviceClass,
    pub deviceID: CMDeviceID,
    pub deviceScope: CMDeviceScope,
    pub deviceState: CMDeviceState,
    pub defaultProfileID: CMDeviceProfileID,
    pub deviceName: *mut CFDictionaryRef,
    pub profileCount: UInt32,
    pub reserved: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CMDeviceProfileInfo {
    pub dataVersion: UInt32,
    pub profileID: CMDeviceProfileID,
    pub profileLoc: CMProfileLocation,
    pub profileName: CFDictionaryRef,
    pub reserved: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NCMDeviceProfileInfo {
    pub dataVersion: UInt32,
    pub profileID: CMDeviceProfileID,
    pub profileLoc: CMProfileLocation,
    pub profileName: CFDictionaryRef,
    pub profileScope: CMDeviceProfileScope,
    pub reserved: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CMDeviceProfileArray {
    pub profileCount: UInt32,
    pub profiles: [CMDeviceProfileInfo; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueGrafPtr {
    _unused: [u8; 0],
}
pub type GrafPtr = *mut OpaqueGrafPtr;
pub type CGrafPtr = GrafPtr;
pub type GWorldPtr = CGrafPtr;
pub type QDErr = ::std::os::raw::c_short;
pub type GWorldFlags = ::std::os::raw::c_ulong;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct BitMap {
    pub baseAddr: Ptr,
    pub rowBytes: ::std::os::raw::c_short,
    pub bounds: Rect,
}
pub type BitMapPtr = *mut BitMap;
pub type BitMapHandle = *mut BitMapPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    pub red: ::std::os::raw::c_ushort,
    pub green: ::std::os::raw::c_ushort,
    pub blue: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColorSpec {
    pub value: ::std::os::raw::c_short,
    pub rgb: RGBColor,
}
pub type ColorSpecPtr = *mut ColorSpec;
pub type CSpecArray = [ColorSpec; 1usize];
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ColorTable {
    pub ctSeed: SInt32,
    pub ctFlags: ::std::os::raw::c_short,
    pub ctSize: ::std::os::raw::c_short,
    pub ctTable: CSpecArray,
}
pub type CTabPtr = *mut ColorTable;
pub type CTabHandle = *mut CTabPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PixMap {
    pub baseAddr: Ptr,
    pub rowBytes: ::std::os::raw::c_short,
    pub bounds: Rect,
    pub pmVersion: ::std::os::raw::c_short,
    pub packType: ::std::os::raw::c_short,
    pub packSize: SInt32,
    pub hRes: Fixed,
    pub vRes: Fixed,
    pub pixelType: ::std::os::raw::c_short,
    pub pixelSize: ::std::os::raw::c_short,
    pub cmpCount: ::std::os::raw::c_short,
    pub cmpSize: ::std::os::raw::c_short,
    pub pixelFormat: OSType,
    pub pmTable: CTabHandle,
    pub pmExt: *mut ::std::os::raw::c_void,
}
pub type PixMapPtr = *mut PixMap;
pub type PixMapHandle = *mut PixMapPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pattern {
    pub pat: [UInt8; 8usize],
}
pub type PatPtr = *mut Pattern;
pub type PatHandle = *mut PatPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PixPat {
    pub patType: ::std::os::raw::c_short,
    pub patMap: PixMapHandle,
    pub patData: Handle,
    pub patXData: Handle,
    pub patXValid: ::std::os::raw::c_short,
    pub patXMap: Handle,
    pub pat1Data: Pattern,
}
pub type PixPatPtr = *mut PixPat;
pub type PixPatHandle = *mut PixPatPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueRgnHandle {
    _unused: [u8; 0],
}
pub type RgnHandle = *mut OpaqueRgnHandle;
pub type GDPtr = *mut GDevice;
pub type GDHandle = *mut GDPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct GDevice {
    pub gdRefNum: ::std::os::raw::c_short,
    pub gdID: ::std::os::raw::c_short,
    pub gdType: ::std::os::raw::c_short,
    pub gdITable: Handle,
    pub gdResPref: ::std::os::raw::c_short,
    pub gdSearchProc: Handle,
    pub gdCompProc: Handle,
    pub gdFlags: ::std::os::raw::c_short,
    pub gdPMap: PixMapHandle,
    pub gdRefCon: SInt32,
    pub gdNextGD: GDHandle,
    pub gdRect: Rect,
    pub gdMode: SInt32,
    pub gdCCBytes: ::std::os::raw::c_short,
    pub gdCCDepth: ::std::os::raw::c_short,
    pub gdCCXData: Handle,
    pub gdCCXMask: Handle,
    pub gdExt: Handle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Picture {
    pub picSize: ::std::os::raw::c_short,
    pub picFrame: Rect,
}
pub type PicPtr = *mut Picture;
pub type PicHandle = *mut PicPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct OpenCPicParams {
    pub srcRect: Rect,
    pub hRes: Fixed,
    pub vRes: Fixed,
    pub version: ::std::os::raw::c_short,
    pub reserved1: ::std::os::raw::c_short,
    pub reserved2: SInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FontInfo {
    pub ascent: ::std::os::raw::c_short,
    pub descent: ::std::os::raw::c_short,
    pub widMax: ::std::os::raw::c_short,
    pub leading: ::std::os::raw::c_short,
}
pub type QDRegionParseDirection = SInt32;
pub type RegionToRectsProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        message: UInt16,
        rgn: RgnHandle,
        rect: *const Rect,
        refCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type RegionToRectsUPP = RegionToRectsProcPtr;
pub type TruncCode = ::std::os::raw::c_short;
pub type DragConstraint = UInt16;
pub type DragGrayRgnProcPtr = ::std::option::Option<unsafe extern "C" fn()>;
pub type ColorSearchProcPtr = ::std::option::Option<
    unsafe extern "C" fn(rgb: *mut RGBColor, position: *mut ::std::os::raw::c_long) -> Boolean,
>;
pub type ColorComplementProcPtr =
    ::std::option::Option<unsafe extern "C" fn(rgb: *mut RGBColor) -> Boolean>;
pub type DragGrayRgnUPP = DragGrayRgnProcPtr;
pub type ColorSearchUPP = ColorSearchProcPtr;
pub type ColorComplementUPP = ColorComplementProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueWindowPtr {
    _unused: [u8; 0],
}
pub type WindowPtr = *mut OpaqueWindowPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueDialogPtr {
    _unused: [u8; 0],
}
pub type DialogPtr = *mut OpaqueDialogPtr;
pub type WindowRef = WindowPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct VDGammaRecord {
    pub csGTable: Ptr,
}
pub type VDGamRecPtr = *mut VDGammaRecord;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MacPolygon {
    pub polySize: ::std::os::raw::c_short,
    pub polyBBox: Rect,
    pub polyPoints: [Point; 1usize],
}
pub type Polygon = MacPolygon;
pub type PolyPtr = *mut MacPolygon;
pub type PolyHandle = *mut PolyPtr;
pub type GrafVerb = SInt8;
pub type PrinterStatusOpcode = SInt32;
pub type QDTextProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        byteCount: ::std::os::raw::c_short,
        textBuf: *const ::std::os::raw::c_void,
        numer: Point,
        denom: Point,
    ),
>;
pub type QDLineProcPtr = ::std::option::Option<unsafe extern "C" fn(newPt: Point)>;
pub type QDRectProcPtr =
    ::std::option::Option<unsafe extern "C" fn(verb: GrafVerb, r: *const Rect)>;
pub type QDRRectProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        verb: GrafVerb,
        r: *const Rect,
        ovalWidth: ::std::os::raw::c_short,
        ovalHeight: ::std::os::raw::c_short,
    ),
>;
pub type QDOvalProcPtr =
    ::std::option::Option<unsafe extern "C" fn(verb: GrafVerb, r: *const Rect)>;
pub type QDArcProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        verb: GrafVerb,
        r: *const Rect,
        startAngle: ::std::os::raw::c_short,
        arcAngle: ::std::os::raw::c_short,
    ),
>;
pub type QDPolyProcPtr =
    ::std::option::Option<unsafe extern "C" fn(verb: GrafVerb, poly: PolyHandle)>;
pub type QDRgnProcPtr = ::std::option::Option<unsafe extern "C" fn(verb: GrafVerb, rgn: RgnHandle)>;
pub type QDBitsProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        srcBits: *const BitMap,
        srcRect: *const Rect,
        dstRect: *const Rect,
        mode: ::std::os::raw::c_short,
        maskRgn: RgnHandle,
    ),
>;
pub type QDCommentProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        kind: ::std::os::raw::c_short,
        dataSize: ::std::os::raw::c_short,
        dataHandle: Handle,
    ),
>;
pub type QDTxMeasProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        byteCount: ::std::os::raw::c_short,
        textAddr: *const ::std::os::raw::c_void,
        numer: *mut Point,
        denom: *mut Point,
        info: *mut FontInfo,
    ) -> ::std::os::raw::c_short,
>;
pub type QDGetPicProcPtr = ::std::option::Option<
    unsafe extern "C" fn(dataPtr: *mut ::std::os::raw::c_void, byteCount: ::std::os::raw::c_short),
>;
pub type QDPutPicProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        dataPtr: *const ::std::os::raw::c_void,
        byteCount: ::std::os::raw::c_short,
    ),
>;
pub type QDOpcodeProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        fromRect: *const Rect,
        toRect: *const Rect,
        opcode: UInt16,
        version: SInt16,
    ),
>;
pub type QDStdGlyphsProcPtr = ::std::option::Option<
    unsafe extern "C" fn(dataStream: *mut ::std::os::raw::c_void, size: ByteCount) -> OSStatus,
>;
pub type QDJShieldCursorProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        left: ::std::os::raw::c_short,
        top: ::std::os::raw::c_short,
        right: ::std::os::raw::c_short,
        bottom: ::std::os::raw::c_short,
    ),
>;
pub type QDPrinterStatusProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        opcode: PrinterStatusOpcode,
        currentPort: CGrafPtr,
        printerStatus: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type QDTextUPP = QDTextProcPtr;
pub type QDLineUPP = QDLineProcPtr;
pub type QDRectUPP = QDRectProcPtr;
pub type QDRRectUPP = QDRRectProcPtr;
pub type QDOvalUPP = QDOvalProcPtr;
pub type QDArcUPP = QDArcProcPtr;
pub type QDPolyUPP = QDPolyProcPtr;
pub type QDRgnUPP = QDRgnProcPtr;
pub type QDBitsUPP = QDBitsProcPtr;
pub type QDCommentUPP = QDCommentProcPtr;
pub type QDTxMeasUPP = QDTxMeasProcPtr;
pub type QDGetPicUPP = QDGetPicProcPtr;
pub type QDPutPicUPP = QDPutPicProcPtr;
pub type QDOpcodeUPP = QDOpcodeProcPtr;
pub type QDStdGlyphsUPP = QDStdGlyphsProcPtr;
pub type QDJShieldCursorUPP = QDJShieldCursorProcPtr;
pub type QDPrinterStatusUPP = QDPrinterStatusProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CQDProcs {
    pub textProc: QDTextUPP,
    pub lineProc: QDLineUPP,
    pub rectProc: QDRectUPP,
    pub rRectProc: QDRRectUPP,
    pub ovalProc: QDOvalUPP,
    pub arcProc: QDArcUPP,
    pub polyProc: QDPolyUPP,
    pub rgnProc: QDRgnUPP,
    pub bitsProc: QDBitsUPP,
    pub commentProc: QDCommentUPP,
    pub txMeasProc: QDTxMeasUPP,
    pub getPicProc: QDGetPicUPP,
    pub putPicProc: QDPutPicUPP,
    pub opcodeProc: QDOpcodeUPP,
    pub newProc1: UniversalProcPtr,
    pub glyphsProc: QDStdGlyphsUPP,
    pub printerStatusProc: QDPrinterStatusUPP,
    pub newProc4: UniversalProcPtr,
    pub newProc5: UniversalProcPtr,
    pub newProc6: UniversalProcPtr,
}
pub type CQDProcsPtr = *mut CQDProcs;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GrafPort {
    pub whatever: [::std::os::raw::c_short; 87usize],
}
pub type CGrafPort = GrafPort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __HIShape {
    _unused: [u8; 0],
}
pub type HIShapeRef = *const __HIShape;
pub type HIMutableShapeRef = *mut __HIShape;
pub type IconAlignmentType = SInt16;
pub type IconTransformType = SInt16;
pub type IconSelectorValue = UInt32;
pub type IconActionProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theType: ResType,
        theIcon: *mut Handle,
        yourDataPtr: *mut ::std::os::raw::c_void,
    ) -> OSErr,
>;
pub type IconGetterProcPtr = ::std::option::Option<
    unsafe extern "C" fn(theType: ResType, yourDataPtr: *mut ::std::os::raw::c_void) -> Handle,
>;
pub type IconActionUPP = IconActionProcPtr;
pub type IconGetterUPP = IconGetterProcPtr;
pub type PlotIconRefFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueICInstance {
    _unused: [u8; 0],
}
pub type ICInstance = *mut OpaqueICInstance;
pub type ICAttr = UInt32;
pub type ICPerm = UInt8;
pub type ICProfileID = SInt32;
pub type ICProfileIDPtr = *mut ICProfileID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ICFontRecord {
    pub size: SInt16,
    pub face: Style,
    pub pad: ::std::os::raw::c_char,
    pub font: Str255,
}
pub type ICFontRecordPtr = *mut ICFontRecord;
pub type ICFontRecordHandle = *mut ICFontRecordPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ICCharTable {
    pub netToMac: [::std::os::raw::c_uchar; 256usize],
    pub macToNet: [::std::os::raw::c_uchar; 256usize],
}
pub type ICCharTablePtr = *mut ICCharTable;
pub type ICCharTableHandle = *mut ICCharTablePtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAppSpec {
    pub fCreator: OSType,
    pub name: Str63,
}
pub type ICAppSpecPtr = *mut ICAppSpec;
pub type ICAppSpecHandle = *mut ICAppSpecPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ICAppSpecList {
    pub numberOfItems: SInt16,
    pub appSpecs: [ICAppSpec; 1usize],
}
pub type ICAppSpecListPtr = *mut ICAppSpecList;
pub type ICAppSpecListHandle = *mut ICAppSpecListPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICFileSpec {
    pub volName: Str31,
    pub volCreationDate: SInt32,
    pub fss: FSSpec,
    pub alias: AliasRecord,
}
pub type ICFileSpecPtr = *mut ICFileSpec;
pub type ICFileSpecHandle = *mut ICFileSpecPtr;
pub type ICMapEntryFlags = SInt32;
pub type ICFixedLength = SInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICMapEntry {
    pub totalLength: SInt16,
    pub fixedLength: ICFixedLength,
    pub version: SInt16,
    pub fileType: OSType,
    pub fileCreator: OSType,
    pub postCreator: OSType,
    pub flags: ICMapEntryFlags,
    pub extension: Str255,
    pub creatorAppName: Str255,
    pub postAppName: Str255,
    pub MIMEType: Str255,
    pub entryName: Str255,
}
pub type ICMapEntryPtr = *mut ICMapEntry;
pub type ICMapEntryHandle = *mut ICMapEntryPtr;
pub type ICServiceEntryFlags = SInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ICServiceEntry {
    pub name: Str255,
    pub port: SInt16,
    pub flags: ICServiceEntryFlags,
}
pub type ICServiceEntryPtr = *mut ICServiceEntry;
pub type ICServiceEntryHandle = *mut ICServiceEntryPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ICServices {
    pub count: SInt16,
    pub services: [ICServiceEntry; 1usize],
}
pub type ICServicesPtr = *mut ICServices;
pub type ICServicesHandle = *mut ICServicesPtr;
pub type LaunchFlags = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AppParameters {
    pub theMsgEvent: AppParameters__bindgen_ty_1,
    pub eventRefCon: UInt32,
    pub messageLength: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AppParameters__bindgen_ty_1 {
    pub what: UInt16,
    pub message: UInt32,
    pub when: UInt32,
    pub where_: Point,
    pub modifiers: UInt16,
}
pub type AppParametersPtr = *mut AppParameters;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LaunchParamBlockRec {
    pub reserved1: UInt32,
    pub reserved2: UInt16,
    pub launchBlockID: UInt16,
    pub launchEPBLength: UInt32,
    pub launchFileFlags: UInt16,
    pub launchControlFlags: LaunchFlags,
    pub launchAppRef: FSRefPtr,
    pub launchProcessSN: ProcessSerialNumber,
    pub launchPreferredSize: UInt32,
    pub launchMinimumSize: UInt32,
    pub launchAvailableSize: UInt32,
    pub launchAppParameters: AppParametersPtr,
}
pub type LaunchPBPtr = *mut LaunchParamBlockRec;
pub type ProcessApplicationTransformState = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ProcessInfoRec {
    pub processInfoLength: UInt32,
    pub processName: StringPtr,
    pub processNumber: ProcessSerialNumber,
    pub processType: UInt32,
    pub processSignature: OSType,
    pub processMode: UInt32,
    pub processLocation: Ptr,
    pub processSize: UInt32,
    pub processFreeMem: UInt32,
    pub processLauncher: ProcessSerialNumber,
    pub processLaunchDate: UInt32,
    pub processActiveTime: UInt32,
    pub processAppRef: FSRefPtr,
}
pub type ProcessInfoRecPtr = *mut ProcessInfoRec;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ProcessInfoExtendedRec {
    pub processInfoLength: UInt32,
    pub processName: StringPtr,
    pub processNumber: ProcessSerialNumber,
    pub processType: UInt32,
    pub processSignature: OSType,
    pub processMode: UInt32,
    pub processLocation: Ptr,
    pub processSize: UInt32,
    pub processFreeMem: UInt32,
    pub processLauncher: ProcessSerialNumber,
    pub processLaunchDate: UInt32,
    pub processActiveTime: UInt32,
    pub processAppRef: FSRefPtr,
    pub processTempMemTotal: UInt32,
    pub processPurgeableTempMemTotal: UInt32,
}
pub type ProcessInfoExtendedRecPtr = *mut ProcessInfoExtendedRec;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SizeResourceRec {
    pub flags: UInt16,
    pub preferredHeapSize: UInt32,
    pub minimumHeapSize: UInt32,
}
pub type SizeResourceRecPtr = *mut SizeResourceRec;
pub type SizeResourceRecHandle = *mut SizeResourceRecPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePasteboardRef {
    _unused: [u8; 0],
}
pub type PasteboardRef = *mut OpaquePasteboardRef;
pub type PasteboardItemID = *mut ::std::os::raw::c_void;
pub type PasteboardSyncFlags = OptionBits;
pub type PasteboardFlavorFlags = OptionBits;
pub type PasteboardStandardLocation = OSType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueTranslationRef {
    _unused: [u8; 0],
}
pub type TranslationRef = *mut OpaqueTranslationRef;
pub type TranslationFlags = OptionBits;
pub type AXError = SInt32;
pub type AXMenuItemModifiers = UInt32;
pub type AXPriority = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __AXUIElement {
    _unused: [u8; 0],
}
pub type AXUIElementRef = *const __AXUIElement;
pub type AXCopyMultipleAttributeOptions = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __AXTextMarker {
    _unused: [u8; 0],
}
pub type AXTextMarkerRef = *const __AXTextMarker;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __AXTextMarkerRange {
    _unused: [u8; 0],
}
pub type AXTextMarkerRangeRef = *const __AXTextMarkerRange;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __AXObserver {
    _unused: [u8; 0],
}
pub type AXObserverRef = *mut __AXObserver;
pub type AXValueType = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __AXValue {
    _unused: [u8; 0],
}
pub type AXValueRef = *const __AXValue;
pub type AXUnderlineStyle = UInt32;
pub type UAZoomChangeFocusType = UInt32;
pub type PMObject = *const ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePMPrintSettings {
    _unused: [u8; 0],
}
pub type PMPrintSettings = *mut OpaquePMPrintSettings;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePMPageFormat {
    _unused: [u8; 0],
}
pub type PMPageFormat = *mut OpaquePMPageFormat;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePMPrintSession {
    _unused: [u8; 0],
}
pub type PMPrintSession = *mut OpaquePMPrintSession;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePMPrinter {
    _unused: [u8; 0],
}
pub type PMPrinter = *mut OpaquePMPrinter;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePMServer {
    _unused: [u8; 0],
}
pub type PMServer = *mut OpaquePMServer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePMPreset {
    _unused: [u8; 0],
}
pub type PMPreset = *mut OpaquePMPreset;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePMPaper {
    _unused: [u8; 0],
}
pub type PMPaper = *mut OpaquePMPaper;
pub type PMDestinationType = UInt16;
pub type PMOrientation = UInt16;
pub type PMPrinterState = UInt16;
pub type PMColorSpaceModel = UInt32;
pub type PMQualityMode = UInt32;
pub type PMPaperType = UInt32;
pub type PMScalingAlignment = UInt16;
pub type PMDuplexMode = UInt32;
pub type PMLayoutDirection = UInt16;
pub type PMBorderType = UInt16;
pub type PMPrintDialogOptionFlags = OptionBits;
pub type PMPPDDomain = UInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PMRect {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PMResolution {
    pub hRes: f64,
    pub vRes: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PMLanguageInfo {
    pub level: Str32,
    pub version: Str32,
    pub release: Str32,
}
pub type PMPaperMargins = PMRect;
pub type PMDataFormat = ::std::os::raw::c_uint;
pub trait PPDEPlugIn: Sized + std::ops::Deref {
    unsafe fn initWithBundle_(&self, theBundle: NSBundle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBundle : theBundle)
    }
    unsafe fn PDEPanelsForType_withHostInfo_(&self, pdeType: NSString, host: *mut u64) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PDEPanelsForType : pdeType, withHostInfo : host)
    }
}
pub trait PPDEPanel: Sized + std::ops::Deref {
    unsafe fn willShow(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willShow)
    }
    unsafe fn shouldHide(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldHide)
    }
    unsafe fn saveValuesAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveValuesAndReturnError : error)
    }
    unsafe fn restoreValuesAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreValuesAndReturnError : error)
    }
    unsafe fn supportedPPDOptionKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedPPDOptionKeys)
    }
    unsafe fn PPDOptionKeyValueDidChange_ppdChoice_(&self, option: NSString, choice: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PPDOptionKeyValueDidChange : option, ppdChoice : choice)
    }
    unsafe fn panelView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, panelView)
    }
    unsafe fn panelName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, panelName)
    }
    unsafe fn panelKind(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, panelKind)
    }
    unsafe fn summaryInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summaryInfo)
    }
    unsafe fn shouldShowHelp(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldShowHelp)
    }
    unsafe fn shouldPrint(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldPrint)
    }
    unsafe fn printWindowWillClose_(&self, userCanceled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, printWindowWillClose : userCanceled)
    }
}
pub trait PPDEPlugInCallbackProtocol: Sized + std::ops::Deref {
    unsafe fn printSession(&self) -> PMPrintSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, printSession)
    }
    unsafe fn printSettings(&self) -> PMPrintSettings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, printSettings)
    }
    unsafe fn pageFormat(&self) -> PMPageFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageFormat)
    }
    unsafe fn PMPrinter(&self) -> PMPrinter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PMPrinter)
    }
    unsafe fn ppdFile(&self) -> *mut ppd_file_s
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ppdFile)
    }
    unsafe fn willChangePPDOptionKeyValue_ppdChoice_(
        &self,
        option: NSString,
        choice: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willChangePPDOptionKeyValue : option, ppdChoice : choice)
    }
}
pub type PMPageToPaperMappingType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FontRec {
    pub fontType: SInt16,
    pub firstChar: SInt16,
    pub lastChar: SInt16,
    pub widMax: SInt16,
    pub kernMax: SInt16,
    pub nDescent: SInt16,
    pub fRectWidth: SInt16,
    pub fRectHeight: SInt16,
    pub owTLoc: UInt16,
    pub ascent: SInt16,
    pub descent: SInt16,
    pub leading: SInt16,
    pub rowWords: SInt16,
}
pub type FontRecPtr = *mut FontRec;
pub type FontRecHdl = *mut FontRecPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FMInput {
    pub family: ::std::os::raw::c_short,
    pub size: ::std::os::raw::c_short,
    pub face: Style,
    pub needBits: Boolean,
    pub device: ::std::os::raw::c_short,
    pub numer: Point,
    pub denom: Point,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FamRec {
    pub ffFlags: SInt16,
    pub ffFamID: SInt16,
    pub ffFirstChar: SInt16,
    pub ffLastChar: SInt16,
    pub ffAscent: SInt16,
    pub ffDescent: SInt16,
    pub ffLeading: SInt16,
    pub ffWidMax: SInt16,
    pub ffWTabOff: SInt32,
    pub ffKernOff: SInt32,
    pub ffStylOff: SInt32,
    pub ffProperty: [SInt16; 9usize],
    pub ffIntl: [SInt16; 2usize],
    pub ffVersion: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsscEntry {
    pub fontSize: SInt16,
    pub fontStyle: SInt16,
    pub fontID: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FontAssoc {
    pub numAssoc: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct StyleTable {
    pub fontClass: SInt16,
    pub offset: SInt32,
    pub reserved: SInt32,
    pub indexes: [::std::os::raw::c_char; 48usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NameTable {
    pub stringCount: SInt16,
    pub baseFontName: Str255,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernPair {
    pub kernFirst: ::std::os::raw::c_char,
    pub kernSecond: ::std::os::raw::c_char,
    pub kernWidth: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernEntry {
    pub kernStyle: SInt16,
    pub kernLength: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernTable {
    pub numKerns: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueATSUTextLayout {
    _unused: [u8; 0],
}
pub type ATSUTextLayout = *mut OpaqueATSUTextLayout;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueATSUStyle {
    _unused: [u8; 0],
}
pub type ATSUStyle = *mut OpaqueATSUStyle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueATSUFontFallbacks {
    _unused: [u8; 0],
}
pub type ATSUFontFallbacks = *mut OpaqueATSUFontFallbacks;
pub type ATSUTextMeasurement = Fixed;
pub type ATSUFontID = FMFont;
pub type ATSUFontFeatureType = UInt16;
pub type ATSUFontFeatureSelector = UInt16;
pub type ATSUFontVariationAxis = FourCharCode;
pub type ATSUFontVariationValue = Fixed;
pub type ATSUAttributeTag = UInt32;
pub type ATSUAttributeValuePtr = *mut ::std::os::raw::c_void;
pub type ConstATSUAttributeValuePtr = *const ::std::os::raw::c_void;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSUAttributeInfo {
    pub fTag: ATSUAttributeTag,
    pub fValueSize: ByteCount,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSUCaret {
    pub fX: Fixed,
    pub fY: Fixed,
    pub fDeltaX: Fixed,
    pub fDeltaY: Fixed,
}
pub type ATSUCursorMovementType = UInt16;
pub type ATSULineTruncation = UInt32;
pub type ATSUStyleLineCountType = UInt16;
pub type ATSUVerticalCharacterType = UInt16;
pub type ATSUStyleComparison = UInt16;
pub type ATSUFontFallbackMethod = UInt16;
pub type ATSUTabType = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSUTab {
    pub tabPosition: ATSUTextMeasurement,
    pub tabType: ATSUTabType,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSURGBAlphaColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
pub type GlyphCollection = UInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ATSUGlyphSelector {
    pub collection: GlyphCollection,
    pub glyphID: GlyphID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSUGlyphInfo {
    pub glyphID: GlyphID,
    pub reserved: UInt16,
    pub layoutFlags: UInt32,
    pub charIndex: UniCharArrayOffset,
    pub style: ATSUStyle,
    pub deltaY: Float32,
    pub idealX: Float32,
    pub screenX: SInt16,
    pub caretX: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSUGlyphInfoArray {
    pub layout: ATSUTextLayout,
    pub numGlyphs: ItemCount,
    pub glyphs: [ATSUGlyphInfo; 1usize],
}
pub type ATSUHighlightMethod = UInt32;
pub type ATSUBackgroundDataType = UInt32;
pub type ATSUBackgroundColor = ATSURGBAlphaColor;
pub type RedrawBackgroundProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        iLayout: ATSUTextLayout,
        iTextOffset: UniCharArrayOffset,
        iTextLength: UniCharCount,
        iUnhighlightArea: *mut ATSTrapezoid,
        iTrapezoidCount: ItemCount,
    ) -> Boolean,
>;
pub type RedrawBackgroundUPP = RedrawBackgroundProcPtr;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union ATSUBackgroundData {
    pub backgroundColor: ATSUBackgroundColor,
    pub backgroundUPP: RedrawBackgroundUPP,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct ATSUUnhighlightData {
    pub dataType: ATSUBackgroundDataType,
    pub unhighlightData: ATSUBackgroundData,
}
pub type ATSQuadraticNewPathProcPtr = ::std::option::Option<
    unsafe extern "C" fn(callBackDataPtr: *mut ::std::os::raw::c_void) -> OSStatus,
>;
pub type ATSQuadraticNewPathUPP = ATSQuadraticNewPathProcPtr;
pub type ATSQuadraticLineProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        pt1: *const Float32Point,
        pt2: *const Float32Point,
        callBackDataPtr: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type ATSQuadraticLineUPP = ATSQuadraticLineProcPtr;
pub type ATSQuadraticCurveProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        pt1: *const Float32Point,
        controlPt: *const Float32Point,
        pt2: *const Float32Point,
        callBackDataPtr: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type ATSQuadraticCurveUPP = ATSQuadraticCurveProcPtr;
pub type ATSQuadraticClosePathProcPtr = ::std::option::Option<
    unsafe extern "C" fn(callBackDataPtr: *mut ::std::os::raw::c_void) -> OSStatus,
>;
pub type ATSQuadraticClosePathUPP = ATSQuadraticClosePathProcPtr;
pub type ATSCubicMoveToProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        pt: *const Float32Point,
        callBackDataPtr: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type ATSCubicMoveToUPP = ATSCubicMoveToProcPtr;
pub type ATSCubicLineToProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        pt: *const Float32Point,
        callBackDataPtr: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type ATSCubicLineToUPP = ATSCubicLineToProcPtr;
pub type ATSCubicCurveToProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        pt1: *const Float32Point,
        pt2: *const Float32Point,
        pt3: *const Float32Point,
        callBackDataPtr: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type ATSCubicCurveToUPP = ATSCubicCurveToProcPtr;
pub type ATSCubicClosePathProcPtr = ::std::option::Option<
    unsafe extern "C" fn(callBackDataPtr: *mut ::std::os::raw::c_void) -> OSStatus,
>;
pub type ATSCubicClosePathUPP = ATSCubicClosePathProcPtr;
pub type ATSUFlattenedDataStreamFormat = UInt32;
pub type ATSUFlattenStyleRunOptions = UInt32;
pub type ATSUUnFlattenStyleRunOptions = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSUStyleRunInfo {
    pub runLength: UInt32,
    pub styleObjectIndex: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataMainHeaderBlock {
    pub version: UInt32,
    pub sizeOfDataBlock: UInt32,
    pub offsetToTextLayouts: UInt32,
    pub offsetToStyleRuns: UInt32,
    pub offsetToStyleList: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataTextLayoutDataHeader {
    pub sizeOfLayoutData: UInt32,
    pub textLayoutLength: UInt32,
    pub offsetToLayoutControls: UInt32,
    pub offsetToLineInfo: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataTextLayoutHeader {
    pub numFlattenedTextLayouts: UInt32,
    pub flattenedTextLayouts: [ATSFlatDataTextLayoutDataHeader; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataLayoutControlsDataHeader {
    pub numberOfLayoutControls: UInt32,
    pub controlArray: [ATSUAttributeInfo; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataLineInfoData {
    pub lineLength: UInt32,
    pub numberOfLineControls: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataLineInfoHeader {
    pub numberOfLines: UInt32,
    pub lineInfoArray: [ATSFlatDataLineInfoData; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataStyleRunDataHeader {
    pub numberOfStyleRuns: UInt32,
    pub styleRunArray: [ATSUStyleRunInfo; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataStyleListStyleDataHeader {
    pub sizeOfStyleInfo: UInt32,
    pub numberOfSetAttributes: UInt32,
    pub numberOfSetFeatures: UInt32,
    pub numberOfSetVariations: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataStyleListHeader {
    pub numberOfStyles: UInt32,
    pub styleDataArray: [ATSFlatDataStyleListStyleDataHeader; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataStyleListFeatureData {
    pub theFeatureType: ATSUFontFeatureType,
    pub theFeatureSelector: ATSUFontFeatureSelector,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataStyleListVariationData {
    pub theVariationAxis: ATSUFontVariationAxis,
    pub theVariationValue: ATSUFontVariationValue,
}
pub type ATSFlatDataFontSpeciferType = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataFontNameDataHeader {
    pub nameSpecifierType: ATSFlatDataFontSpeciferType,
    pub nameSpecifierSize: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataFontSpecRawNameData {
    pub fontNameType: FontNameCode,
    pub fontNamePlatform: FontPlatformCode,
    pub fontNameScript: FontScriptCode,
    pub fontNameLanguage: FontLanguageCode,
    pub fontNameLength: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ATSFlatDataFontSpecRawNameDataHeader {
    pub numberOfFlattenedNames: UInt32,
    pub nameDataArray: [ATSFlatDataFontSpecRawNameData; 1usize],
}
pub type ATSUDirectDataSelector = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LLCStyleInfo {
    _unused: [u8; 0],
}
pub type ATSUStyleSettingRef = *mut LLCStyleInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SpeechChannelRecord {
    pub data: [::std::os::raw::c_long; 1usize],
}
pub type SpeechChannel = *mut SpeechChannelRecord;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct VoiceSpec {
    pub creator: OSType,
    pub id: OSType,
}
pub type VoiceSpecPtr = *mut VoiceSpec;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct VoiceDescription {
    pub length: SInt32,
    pub voice: VoiceSpec,
    pub version: SInt32,
    pub name: Str63,
    pub comment: Str255,
    pub gender: SInt16,
    pub age: SInt16,
    pub script: SInt16,
    pub language: SInt16,
    pub region: SInt16,
    pub reserved: [SInt32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoiceFileInfo {
    pub fileSpec: FSSpec,
    pub resID: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SpeechStatusInfo {
    pub outputBusy: Boolean,
    pub outputPaused: Boolean,
    pub inputBytesLeft: ::std::os::raw::c_long,
    pub phonemeCode: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SpeechErrorInfo {
    pub count: SInt16,
    pub oldest: OSErr,
    pub oldPos: ::std::os::raw::c_long,
    pub newest: OSErr,
    pub newPos: ::std::os::raw::c_long,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SpeechVersionInfo {
    pub synthType: OSType,
    pub synthSubType: OSType,
    pub synthManufacturer: OSType,
    pub synthFlags: SInt32,
    pub synthVersion: NumVersion,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhonemeInfo {
    pub opcode: SInt16,
    pub phStr: Str15,
    pub exampleStr: Str31,
    pub hiliteStart: SInt16,
    pub hiliteEnd: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhonemeDescriptor {
    pub phonemeCount: SInt16,
    pub thePhonemes: [PhonemeInfo; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SpeechXtndData {
    pub synthCreator: OSType,
    pub synthData: [Byte; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DelimiterInfo {
    pub startDelimiter: [Byte; 2usize],
    pub endDelimiter: [Byte; 2usize],
}
pub type SpeechTextDoneProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        chan: SpeechChannel,
        refCon: SRefCon,
        nextBuf: *mut *const ::std::os::raw::c_void,
        byteLen: *mut ::std::os::raw::c_ulong,
        controlFlags: *mut SInt32,
    ),
>;
pub type SpeechDoneProcPtr =
    ::std::option::Option<unsafe extern "C" fn(chan: SpeechChannel, refCon: SRefCon)>;
pub type SpeechSyncProcPtr = ::std::option::Option<
    unsafe extern "C" fn(chan: SpeechChannel, refCon: SRefCon, syncMessage: OSType),
>;
pub type SpeechErrorProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        chan: SpeechChannel,
        refCon: SRefCon,
        theError: OSErr,
        bytePos: ::std::os::raw::c_long,
    ),
>;
pub type SpeechPhonemeProcPtr = ::std::option::Option<
    unsafe extern "C" fn(chan: SpeechChannel, refCon: SRefCon, phonemeOpcode: SInt16),
>;
pub type SpeechWordProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        chan: SpeechChannel,
        refCon: SRefCon,
        wordPos: ::std::os::raw::c_ulong,
        wordLen: UInt16,
    ),
>;
pub type SpeechTextDoneUPP = SpeechTextDoneProcPtr;
pub type SpeechDoneUPP = SpeechDoneProcPtr;
pub type SpeechSyncUPP = SpeechSyncProcPtr;
pub type SpeechErrorUPP = SpeechErrorProcPtr;
pub type SpeechPhonemeUPP = SpeechPhonemeProcPtr;
pub type SpeechWordUPP = SpeechWordProcPtr;
unsafe extern "C" {
    pub fn ATSGetGeneration() -> ATSGeneration;
}
unsafe extern "C" {
    pub fn ATSFontActivateFromMemory(
        iData: LogicalAddress,
        iLength: ByteCount,
        iContext: ATSFontContext,
        iFormat: ATSFontFormat,
        iReserved: *mut ::std::os::raw::c_void,
        iOptions: ATSOptionFlags,
        oContainer: *mut ATSFontContainerRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontDeactivate(
        iContainer: ATSFontContainerRef,
        iRefCon: *mut ::std::os::raw::c_void,
        iOptions: ATSOptionFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetContainer(
        iFont: ATSFontRef,
        iOptions: ATSOptionFlags,
        oContainer: *mut ATSFontContainerRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontSetEnabled(
        iFont: ATSFontRef,
        iOptions: ATSOptionFlags,
        iEnabled: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontIsEnabled(iFont: ATSFontRef) -> Boolean;
}
unsafe extern "C" {
    pub fn ATSFontFamilyApplyFunction(
        iFunction: ATSFontFamilyApplierFunction,
        iRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontFamilyIteratorCreate(
        iContext: ATSFontContext,
        iFilter: *const ATSFontFilter,
        iRefCon: *mut ::std::os::raw::c_void,
        iOptions: ATSOptionFlags,
        ioIterator: *mut ATSFontFamilyIterator,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontFamilyIteratorRelease(ioIterator: *mut ATSFontFamilyIterator) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontFamilyIteratorReset(
        iContext: ATSFontContext,
        iFilter: *const ATSFontFilter,
        iRefCon: *mut ::std::os::raw::c_void,
        iOptions: ATSOptionFlags,
        ioIterator: *mut ATSFontFamilyIterator,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontFamilyIteratorNext(
        iIterator: ATSFontFamilyIterator,
        oFamily: *mut ATSFontFamilyRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontFamilyFindFromName(
        iName: CFStringRef,
        iOptions: ATSOptionFlags,
    ) -> ATSFontFamilyRef;
}
unsafe extern "C" {
    pub fn ATSFontFamilyGetGeneration(iFamily: ATSFontFamilyRef) -> ATSGeneration;
}
unsafe extern "C" {
    pub fn ATSFontFamilyGetName(
        iFamily: ATSFontFamilyRef,
        iOptions: ATSOptionFlags,
        oName: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontApplyFunction(
        iFunction: ATSFontApplierFunction,
        iRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontIteratorCreate(
        iContext: ATSFontContext,
        iFilter: *const ATSFontFilter,
        iRefCon: *mut ::std::os::raw::c_void,
        iOptions: ATSOptionFlags,
        ioIterator: *mut ATSFontIterator,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontIteratorRelease(ioIterator: *mut ATSFontIterator) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontIteratorReset(
        iContext: ATSFontContext,
        iFilter: *const ATSFontFilter,
        iRefCon: *mut ::std::os::raw::c_void,
        iOptions: ATSOptionFlags,
        ioIterator: *mut ATSFontIterator,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontIteratorNext(iIterator: ATSFontIterator, oFont: *mut ATSFontRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontFindFromName(iName: CFStringRef, iOptions: ATSOptionFlags) -> ATSFontRef;
}
unsafe extern "C" {
    pub fn ATSFontFindFromPostScriptName(
        iName: CFStringRef,
        iOptions: ATSOptionFlags,
    ) -> ATSFontRef;
}
unsafe extern "C" {
    pub fn ATSFontFindFromContainer(
        iContainer: ATSFontContainerRef,
        iOptions: ATSOptionFlags,
        iCount: ItemCount,
        ioArray: *mut ATSFontRef,
        oCount: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetGeneration(iFont: ATSFontRef) -> ATSGeneration;
}
unsafe extern "C" {
    pub fn ATSFontGetName(
        iFont: ATSFontRef,
        iOptions: ATSOptionFlags,
        oName: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetPostScriptName(
        iFont: ATSFontRef,
        iOptions: ATSOptionFlags,
        oName: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetTableDirectory(
        iFont: ATSFontRef,
        iBufferSize: ByteCount,
        ioBuffer: *mut ::std::os::raw::c_void,
        oSize: *mut ByteCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetTable(
        iFont: ATSFontRef,
        iTag: FourCharCode,
        iOffset: ByteOffset,
        iBufferSize: ByteCount,
        ioBuffer: *mut ::std::os::raw::c_void,
        oSize: *mut ByteCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetHorizontalMetrics(
        iFont: ATSFontRef,
        iOptions: ATSOptionFlags,
        oMetrics: *mut ATSFontMetrics,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetVerticalMetrics(
        iFont: ATSFontRef,
        iOptions: ATSOptionFlags,
        oMetrics: *mut ATSFontMetrics,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontFamilyFindFromQuickDrawName(iName: ConstStr255Param) -> ATSFontFamilyRef;
}
unsafe extern "C" {
    pub fn ATSFontFamilyGetQuickDrawName(
        iFamily: ATSFontFamilyRef,
        oName: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetFontFamilyResource(
        iFont: ATSFontRef,
        iBufferSize: ByteCount,
        ioBuffer: *mut ::std::os::raw::c_void,
        oSize: *mut ByteCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontNotify(
        action: ATSFontNotifyAction,
        info: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontNotificationUnsubscribe(notificationRef: ATSFontNotificationRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontSetGlobalAutoActivationSetting(
        iSetting: ATSFontAutoActivationSetting,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetGlobalAutoActivationSetting() -> ATSFontAutoActivationSetting;
}
unsafe extern "C" {
    pub fn ATSFontSetAutoActivationSettingForApplication(
        iSetting: ATSFontAutoActivationSetting,
        iApplicationFileURL: CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ATSFontGetAutoActivationSettingForApplication(
        iApplicationFileURL: CFURLRef,
    ) -> ATSFontAutoActivationSetting;
}
unsafe extern "C" {
    pub fn HIShapeGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn HIShapeCreateEmpty() -> HIShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateWithQDRgn(inRgn: RgnHandle) -> HIShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateWithRect(inRect: *const CGRect) -> HIShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateCopy(inShape: HIShapeRef) -> HIShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateIntersection(inShape1: HIShapeRef, inShape2: HIShapeRef) -> HIShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateDifference(inShape1: HIShapeRef, inShape2: HIShapeRef) -> HIShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateUnion(inShape1: HIShapeRef, inShape2: HIShapeRef) -> HIShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateXor(inShape1: HIShapeRef, inShape2: HIShapeRef) -> HIShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeIsEmpty(inShape: HIShapeRef) -> Boolean;
}
unsafe extern "C" {
    pub fn HIShapeIsRectangular(inShape: HIShapeRef) -> Boolean;
}
unsafe extern "C" {
    pub fn HIShapeContainsPoint(inShape: HIShapeRef, inPoint: *const CGPoint) -> Boolean;
}
unsafe extern "C" {
    pub fn HIShapeIntersectsRect(inShape: HIShapeRef, inRect: *const CGRect) -> Boolean;
}
unsafe extern "C" {
    pub fn HIShapeGetBounds(inShape: HIShapeRef, outRect: *mut CGRect) -> *mut CGRect;
}
unsafe extern "C" {
    pub fn HIShapeGetAsQDRgn(inShape: HIShapeRef, outRgn: RgnHandle) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeReplacePathInCGContext(inShape: HIShapeRef, inContext: CGContextRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeCreateMutable() -> HIMutableShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateMutableCopy(inOrig: HIShapeRef) -> HIMutableShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeCreateMutableWithRect(inRect: *const CGRect) -> HIMutableShapeRef;
}
unsafe extern "C" {
    pub fn HIShapeSetEmpty(inShape: HIMutableShapeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeSetWithShape(inDestShape: HIMutableShapeRef, inSrcShape: HIShapeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeIntersect(
        inShape1: HIShapeRef,
        inShape2: HIShapeRef,
        outResult: HIMutableShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeDifference(
        inShape1: HIShapeRef,
        inShape2: HIShapeRef,
        outResult: HIMutableShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeUnion(
        inShape1: HIShapeRef,
        inShape2: HIShapeRef,
        outResult: HIMutableShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeXor(
        inShape1: HIShapeRef,
        inShape2: HIShapeRef,
        outResult: HIMutableShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeOffset(inShape: HIMutableShapeRef, inDX: CGFloat, inDY: CGFloat) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeInset(inShape: HIMutableShapeRef, inDX: CGFloat, inDY: CGFloat) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIShapeUnionWithRect(inShape: HIMutableShapeRef, inRect: *const CGRect) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewIconActionUPP(userRoutine: IconActionProcPtr) -> IconActionUPP;
}
unsafe extern "C" {
    pub fn NewIconGetterUPP(userRoutine: IconGetterProcPtr) -> IconGetterUPP;
}
unsafe extern "C" {
    pub fn DisposeIconActionUPP(userUPP: IconActionUPP);
}
unsafe extern "C" {
    pub fn DisposeIconGetterUPP(userUPP: IconGetterUPP);
}
unsafe extern "C" {
    pub fn InvokeIconActionUPP(
        theType: ResType,
        theIcon: *mut Handle,
        yourDataPtr: *mut ::std::os::raw::c_void,
        userUPP: IconActionUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeIconGetterUPP(
        theType: ResType,
        yourDataPtr: *mut ::std::os::raw::c_void,
        userUPP: IconGetterUPP,
    ) -> Handle;
}
unsafe extern "C" {
    pub fn IconRefToIconFamily(
        theIconRef: IconRef,
        whichIcons: IconSelectorValue,
        iconFamily: *mut IconFamilyHandle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SetIconFamilyData(iconFamily: IconFamilyHandle, iconType: OSType, h: Handle) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIconFamilyData(iconFamily: IconFamilyHandle, iconType: OSType, h: Handle) -> OSErr;
}
unsafe extern "C" {
    pub fn PlotIconRefInContext(
        inContext: CGContextRef,
        inRect: *const CGRect,
        inAlign: IconAlignmentType,
        inTransform: IconTransformType,
        inLabelColor: *const RGBColor,
        inFlags: PlotIconRefFlags,
        inIconRef: IconRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn IconRefContainsCGPoint(
        testPt: *const CGPoint,
        iconRect: *const CGRect,
        align: IconAlignmentType,
        iconServicesUsageFlags: IconServicesUsageFlags,
        theIconRef: IconRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IconRefIntersectsCGRect(
        testRect: *const CGRect,
        iconRect: *const CGRect,
        align: IconAlignmentType,
        iconServicesUsageFlags: IconServicesUsageFlags,
        theIconRef: IconRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IconRefToHIShape(
        iconRect: *const CGRect,
        align: IconAlignmentType,
        iconServicesUsageFlags: IconServicesUsageFlags,
        theIconRef: IconRef,
    ) -> HIShapeRef;
}
unsafe extern "C" {
    pub fn IsIconRefMaskEmpty(iconRef: IconRef) -> Boolean;
}
unsafe extern "C" {
    pub fn GetIconRefVariant(
        inIconRef: IconRef,
        inVariant: OSType,
        outTransform: *mut IconTransformType,
    ) -> IconRef;
}
unsafe extern "C" {
    pub fn ICStart(inst: *mut ICInstance, signature: OSType) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICStop(inst: ICInstance) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetVersion(
        inst: ICInstance,
        whichVersion: ::std::os::raw::c_long,
        version: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetConfigName(
        inst: ICInstance,
        longname: Boolean,
        name: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetSeed(inst: ICInstance, seed: *mut ::std::os::raw::c_long) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetPerm(inst: ICInstance, perm: *mut ICPerm) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICBegin(inst: ICInstance, perm: ICPerm) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetPref(
        inst: ICInstance,
        key: ConstStr255Param,
        attr: *mut ICAttr,
        buf: *mut ::std::os::raw::c_void,
        size: *mut ::std::os::raw::c_long,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICSetPref(
        inst: ICInstance,
        key: ConstStr255Param,
        attr: ICAttr,
        buf: *const ::std::os::raw::c_void,
        size: ::std::os::raw::c_long,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICFindPrefHandle(
        inst: ICInstance,
        key: ConstStr255Param,
        attr: *mut ICAttr,
        prefh: Handle,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetPrefHandle(
        inst: ICInstance,
        key: ConstStr255Param,
        attr: *mut ICAttr,
        prefh: *mut Handle,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICSetPrefHandle(
        inst: ICInstance,
        key: ConstStr255Param,
        attr: ICAttr,
        prefh: Handle,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICCountPref(inst: ICInstance, count: *mut ::std::os::raw::c_long) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetIndPref(
        inst: ICInstance,
        index: ::std::os::raw::c_long,
        key: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICDeletePref(inst: ICInstance, key: ConstStr255Param) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICEnd(inst: ICInstance) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetDefaultPref(inst: ICInstance, key: ConstStr255Param, prefH: Handle) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICEditPreferences(inst: ICInstance, key: ConstStr255Param) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICLaunchURL(
        inst: ICInstance,
        hint: ConstStr255Param,
        data: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_long,
        selStart: *mut ::std::os::raw::c_long,
        selEnd: *mut ::std::os::raw::c_long,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICParseURL(
        inst: ICInstance,
        hint: ConstStr255Param,
        data: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_long,
        selStart: *mut ::std::os::raw::c_long,
        selEnd: *mut ::std::os::raw::c_long,
        url: Handle,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICCreateGURLEvent(
        inst: ICInstance,
        helperCreator: OSType,
        urlH: Handle,
        theEvent: *mut AppleEvent,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICSendGURLEvent(inst: ICInstance, theEvent: *mut AppleEvent) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICMapFilename(
        inst: ICInstance,
        filename: ConstStr255Param,
        entry: *mut ICMapEntry,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICMapTypeCreator(
        inst: ICInstance,
        fType: OSType,
        fCreator: OSType,
        filename: ConstStr255Param,
        entry: *mut ICMapEntry,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICMapEntriesFilename(
        inst: ICInstance,
        entries: Handle,
        filename: ConstStr255Param,
        entry: *mut ICMapEntry,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICMapEntriesTypeCreator(
        inst: ICInstance,
        entries: Handle,
        fType: OSType,
        fCreator: OSType,
        filename: ConstStr255Param,
        entry: *mut ICMapEntry,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICCountMapEntries(
        inst: ICInstance,
        entries: Handle,
        count: *mut ::std::os::raw::c_long,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetIndMapEntry(
        inst: ICInstance,
        entries: Handle,
        index: ::std::os::raw::c_long,
        pos: *mut ::std::os::raw::c_long,
        entry: *mut ICMapEntry,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetMapEntry(
        inst: ICInstance,
        entries: Handle,
        pos: ::std::os::raw::c_long,
        entry: *mut ICMapEntry,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICSetMapEntry(
        inst: ICInstance,
        entries: Handle,
        pos: ::std::os::raw::c_long,
        entry: *const ICMapEntry,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICDeleteMapEntry(
        inst: ICInstance,
        entries: Handle,
        pos: ::std::os::raw::c_long,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICAddMapEntry(inst: ICInstance, entries: Handle, entry: *const ICMapEntry) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetCurrentProfile(inst: ICInstance, currentID: *mut ICProfileID) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICSetCurrentProfile(inst: ICInstance, newID: ICProfileID) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICCountProfiles(inst: ICInstance, count: *mut ::std::os::raw::c_long) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetIndProfile(
        inst: ICInstance,
        index: ::std::os::raw::c_long,
        thisID: *mut ICProfileID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICGetProfileName(
        inst: ICInstance,
        thisID: ICProfileID,
        name: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICSetProfileName(
        inst: ICInstance,
        thisID: ICProfileID,
        name: ConstStr255Param,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICAddProfile(
        inst: ICInstance,
        prototypeID: ICProfileID,
        newID: *mut ICProfileID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ICDeleteProfile(inst: ICInstance, thisID: ICProfileID) -> OSStatus;
}
unsafe extern "C" {
    pub fn LaunchApplication(LaunchParams: LaunchPBPtr) -> OSErr;
}
unsafe extern "C" {
    pub fn GetCurrentProcess(pPSN: *mut ProcessSerialNumber) -> OSErr;
}
unsafe extern "C" {
    pub fn GetFrontProcess(pPSN: *mut ProcessSerialNumber) -> OSErr;
}
unsafe extern "C" {
    pub fn GetNextProcess(pPSN: *mut ProcessSerialNumber) -> OSErr;
}
unsafe extern "C" {
    pub fn GetProcessInformation(
        PSN: *const ProcessSerialNumber,
        info: *mut ProcessInfoRec,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn ProcessInformationCopyDictionary(
        PSN: *const ProcessSerialNumber,
        infoToReturn: UInt32,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SetFrontProcess(pPSN: *const ProcessSerialNumber) -> OSErr;
}
unsafe extern "C" {
    pub fn SetFrontProcessWithOptions(
        inProcess: *const ProcessSerialNumber,
        inOptions: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn WakeUpProcess(PSN: *const ProcessSerialNumber) -> OSErr;
}
unsafe extern "C" {
    pub fn SameProcess(
        PSN1: *const ProcessSerialNumber,
        PSN2: *const ProcessSerialNumber,
        result: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn ExitToShell() -> !;
}
unsafe extern "C" {
    pub fn KillProcess(inProcess: *const ProcessSerialNumber) -> OSErr;
}
unsafe extern "C" {
    pub fn GetProcessBundleLocation(
        psn: *const ProcessSerialNumber,
        location: *mut FSRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CopyProcessName(psn: *const ProcessSerialNumber, name: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetProcessPID(psn: *const ProcessSerialNumber, pid: *mut pid_t) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetProcessForPID(pid: pid_t, psn: *mut ProcessSerialNumber) -> OSStatus;
}
unsafe extern "C" {
    pub fn IsProcessVisible(psn: *const ProcessSerialNumber) -> Boolean;
}
unsafe extern "C" {
    pub fn ShowHideProcess(psn: *const ProcessSerialNumber, visible: Boolean) -> OSErr;
}
unsafe extern "C" {
    pub fn TransformProcessType(
        psn: *const ProcessSerialNumber,
        transformState: ProcessApplicationTransformState,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn PasteboardCreate(inName: CFStringRef, outPasteboard: *mut PasteboardRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardSynchronize(inPasteboard: PasteboardRef) -> PasteboardSyncFlags;
}
unsafe extern "C" {
    pub fn PasteboardClear(inPasteboard: PasteboardRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardCopyName(inPasteboard: PasteboardRef, outName: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardGetItemCount(
        inPasteboard: PasteboardRef,
        outItemCount: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardGetItemIdentifier(
        inPasteboard: PasteboardRef,
        inIndex: CFIndex,
        outItem: *mut PasteboardItemID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardCopyItemFlavors(
        inPasteboard: PasteboardRef,
        inItem: PasteboardItemID,
        outFlavorTypes: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardGetItemFlavorFlags(
        inPasteboard: PasteboardRef,
        inItem: PasteboardItemID,
        inFlavorType: CFStringRef,
        outFlags: *mut PasteboardFlavorFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardCopyItemFlavorData(
        inPasteboard: PasteboardRef,
        inItem: PasteboardItemID,
        inFlavorType: CFStringRef,
        outData: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardPutItemFlavor(
        inPasteboard: PasteboardRef,
        inItem: PasteboardItemID,
        inFlavorType: CFStringRef,
        inData: CFDataRef,
        inFlags: PasteboardFlavorFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardCopyPasteLocation(
        inPasteboard: PasteboardRef,
        outPasteLocation: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardSetPasteLocation(
        inPasteboard: PasteboardRef,
        inPasteLocation: CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PasteboardResolvePromises(inPasteboard: PasteboardRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TranslationGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn TranslationCreate(
        inSourceType: CFStringRef,
        inDestinationType: CFStringRef,
        inTranslationFlags: TranslationFlags,
        outTranslation: *mut TranslationRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TranslationCreateWithSourceArray(
        inSourceTypes: CFArrayRef,
        inTranslationFlags: TranslationFlags,
        outDestinationTypes: *mut CFArrayRef,
        outTranslations: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TranslationPerformForData(
        inTranslation: TranslationRef,
        inSourceData: CFDataRef,
        outDestinationData: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TranslationPerformForFile(
        inTranslation: TranslationRef,
        inSourceFile: *const FSRef,
        inDestinationDirectory: *const FSRef,
        inDestinationName: CFStringRef,
        outTranslatedFile: *mut FSRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TranslationPerformForURL(
        inTranslation: TranslationRef,
        inSourceURL: CFURLRef,
        inDestinationURL: CFURLRef,
        outTranslatedURL: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TranslationCopySourceType(
        inTranslation: TranslationRef,
        outSourceType: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TranslationCopyDestinationType(
        inTranslation: TranslationRef,
        outDestinationType: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TranslationGetTranslationFlags(
        inTranslation: TranslationRef,
        outTranslationFlags: *mut TranslationFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AXAPIEnabled() -> Boolean;
}
unsafe extern "C" {
    pub fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> Boolean;
}
unsafe extern "C" {
    pub static mut kAXTrustedCheckOptionPrompt: CFStringRef;
}
unsafe extern "C" {
    pub fn AXIsProcessTrusted() -> Boolean;
}
unsafe extern "C" {
    pub fn AXMakeProcessTrusted(executablePath: CFStringRef) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn AXUIElementCopyAttributeNames(
        element: AXUIElementRef,
        names: *mut CFArrayRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: *mut CFTypeRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementGetAttributeValueCount(
        element: AXUIElementRef,
        attribute: CFStringRef,
        count: *mut CFIndex,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCopyAttributeValues(
        element: AXUIElementRef,
        attribute: CFStringRef,
        index: CFIndex,
        maxValues: CFIndex,
        values: *mut CFArrayRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementIsAttributeSettable(
        element: AXUIElementRef,
        attribute: CFStringRef,
        settable: *mut Boolean,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementSetAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: CFTypeRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCopyMultipleAttributeValues(
        element: AXUIElementRef,
        attributes: CFArrayRef,
        options: AXCopyMultipleAttributeOptions,
        values: *mut CFArrayRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCopyParameterizedAttributeNames(
        element: AXUIElementRef,
        names: *mut CFArrayRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCopyParameterizedAttributeValue(
        element: AXUIElementRef,
        parameterizedAttribute: CFStringRef,
        parameter: CFTypeRef,
        result: *mut CFTypeRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCopyActionNames(element: AXUIElementRef, names: *mut CFArrayRef) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCopyActionDescription(
        element: AXUIElementRef,
        action: CFStringRef,
        description: *mut CFStringRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementPerformAction(element: AXUIElementRef, action: CFStringRef) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCopyElementAtPosition(
        application: AXUIElementRef,
        x: f32,
        y: f32,
        element: *mut AXUIElementRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementCreateApplication(pid: pid_t) -> AXUIElementRef;
}
unsafe extern "C" {
    pub fn AXUIElementCreateSystemWide() -> AXUIElementRef;
}
unsafe extern "C" {
    pub fn AXUIElementGetPid(element: AXUIElementRef, pid: *mut pid_t) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementSetMessagingTimeout(
        element: AXUIElementRef,
        timeoutInSeconds: f32,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXUIElementPostKeyboardEvent(
        application: AXUIElementRef,
        keyChar: CGCharCode,
        virtualKey: CGKeyCode,
        keyDown: Boolean,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXTextMarkerGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn AXTextMarkerCreate(
        allocator: CFAllocatorRef,
        bytes: *const UInt8,
        length: CFIndex,
    ) -> AXTextMarkerRef;
}
unsafe extern "C" {
    pub fn AXTextMarkerGetLength(marker: AXTextMarkerRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn AXTextMarkerGetBytePtr(theTextMarker: AXTextMarkerRef) -> *const UInt8;
}
unsafe extern "C" {
    pub fn AXTextMarkerRangeGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn AXTextMarkerRangeCreate(
        allocator: CFAllocatorRef,
        startMarker: AXTextMarkerRef,
        endMarker: AXTextMarkerRef,
    ) -> AXTextMarkerRangeRef;
}
unsafe extern "C" {
    pub fn AXTextMarkerRangeCreateWithBytes(
        allocator: CFAllocatorRef,
        startMarkerBytes: *const UInt8,
        startMarkerLength: CFIndex,
        endMarkerBytes: *const UInt8,
        endMarkerLength: CFIndex,
    ) -> AXTextMarkerRangeRef;
}
unsafe extern "C" {
    pub fn AXTextMarkerRangeCopyStartMarker(
        textMarkerRange: AXTextMarkerRangeRef,
    ) -> AXTextMarkerRef;
}
unsafe extern "C" {
    pub fn AXTextMarkerRangeCopyEndMarker(textMarkerRange: AXTextMarkerRangeRef)
        -> AXTextMarkerRef;
}
unsafe extern "C" {
    pub fn AXObserverGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn AXObserverAddNotification(
        observer: AXObserverRef,
        element: AXUIElementRef,
        notification: CFStringRef,
        refcon: *mut ::std::os::raw::c_void,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXObserverRemoveNotification(
        observer: AXObserverRef,
        element: AXUIElementRef,
        notification: CFStringRef,
    ) -> AXError;
}
unsafe extern "C" {
    pub fn AXObserverGetRunLoopSource(observer: AXObserverRef) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn AXValueGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn AXValueCreate(
        theType: AXValueType,
        valuePtr: *const ::std::os::raw::c_void,
    ) -> AXValueRef;
}
unsafe extern "C" {
    pub fn AXValueGetType(value: AXValueRef) -> AXValueType;
}
unsafe extern "C" {
    pub fn AXValueGetValue(
        value: AXValueRef,
        theType: AXValueType,
        valuePtr: *mut ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static mut kAXFontTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXForegroundColorTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXBackgroundColorTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXUnderlineColorTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXStrikethroughColorTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXUnderlineTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXSuperscriptTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXStrikethroughTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXShadowTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXAttachmentTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXLinkTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXNaturalLanguageTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXReplacementStringTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXMisspelledTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXMarkedMisspelledTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXAutocorrectedTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXListItemPrefixTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXListItemIndexTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXListItemLevelTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXFontNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXFontFamilyKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXVisibleNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXFontSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kAXForegoundColorTextAttribute: CFStringRef;
}
unsafe extern "C" {
    pub fn UAZoomEnabled() -> Boolean;
}
unsafe extern "C" {
    pub fn UAZoomChangeFocus(
        inRect: *const CGRect,
        inHighlightRect: *const CGRect,
        inType: UAZoomChangeFocusType,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMRetain(object: PMObject) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMRelease(object: PMObject) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCreateSession(printSession: *mut PMPrintSession) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionError(printSession: PMPrintSession) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionSetError(printSession: PMPrintSession, printError: OSStatus) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionBeginCGDocumentNoDialog(
        printSession: PMPrintSession,
        printSettings: PMPrintSettings,
        pageFormat: PMPageFormat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionEndDocumentNoDialog(printSession: PMPrintSession) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionBeginPageNoDialog(
        printSession: PMPrintSession,
        pageFormat: PMPageFormat,
        pageFrame: *const PMRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionEndPageNoDialog(printSession: PMPrintSession) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionGetCGGraphicsContext(
        printSession: PMPrintSession,
        context: *mut CGContextRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionGetDestinationType(
        printSession: PMPrintSession,
        printSettings: PMPrintSettings,
        destTypeP: *mut PMDestinationType,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionCopyDestinationFormat(
        printSession: PMPrintSession,
        printSettings: PMPrintSettings,
        destFormatP: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionCopyDestinationLocation(
        printSession: PMPrintSession,
        printSettings: PMPrintSettings,
        destLocationP: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionSetDestination(
        printSession: PMPrintSession,
        printSettings: PMPrintSettings,
        destType: PMDestinationType,
        destFormat: CFStringRef,
        destLocation: CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionCopyOutputFormatList(
        printSession: PMPrintSession,
        destType: PMDestinationType,
        documentFormatP: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionCreatePageFormatList(
        printSession: PMPrintSession,
        printer: PMPrinter,
        pageFormatList: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionCreatePrinterList(
        printSession: PMPrintSession,
        printerList: *mut CFArrayRef,
        currentIndex: *mut CFIndex,
        currentPrinter: *mut PMPrinter,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionGetCurrentPrinter(
        printSession: PMPrintSession,
        currentPrinter: *mut PMPrinter,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionSetCurrentPMPrinter(session: PMPrintSession, printer: PMPrinter) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionGetDataFromSession(
        printSession: PMPrintSession,
        key: CFStringRef,
        data: *mut CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionSetDataInSession(
        printSession: PMPrintSession,
        key: CFStringRef,
        data: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCreatePageFormat(pageFormat: *mut PMPageFormat) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionDefaultPageFormat(
        printSession: PMPrintSession,
        pageFormat: PMPageFormat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionValidatePageFormat(
        printSession: PMPrintSession,
        pageFormat: PMPageFormat,
        changed: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCopyPageFormat(formatSrc: PMPageFormat, formatDest: PMPageFormat) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCreatePageFormatWithPMPaper(pageFormat: *mut PMPageFormat, paper: PMPaper)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPageFormatCreateDataRepresentation(
        pageFormat: PMPageFormat,
        data: *mut CFDataRef,
        format: PMDataFormat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPageFormatCreateWithDataRepresentation(
        data: CFDataRef,
        pageFormat: *mut PMPageFormat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetAdjustedPageRect(pageFormat: PMPageFormat, pageRect: *mut PMRect) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetAdjustedPaperRect(pageFormat: PMPageFormat, paperRect: *mut PMRect) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetOrientation(pageFormat: PMPageFormat, orientation: *mut PMOrientation) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetPageFormatExtendedData(
        pageFormat: PMPageFormat,
        dataID: OSType,
        size: *mut UInt32,
        extendedData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPageFormatGetPrinterID(
        pageFormat: PMPageFormat,
        printerID: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetScale(pageFormat: PMPageFormat, scale: *mut f64) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetUnadjustedPageRect(pageFormat: PMPageFormat, pageRect: *mut PMRect) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetUnadjustedPaperRect(pageFormat: PMPageFormat, paperRect: *mut PMRect) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetOrientation(
        pageFormat: PMPageFormat,
        orientation: PMOrientation,
        lock: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetPageFormatExtendedData(
        pageFormat: PMPageFormat,
        dataID: OSType,
        size: UInt32,
        extendedData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetScale(pageFormat: PMPageFormat, scale: f64) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCreatePrintSettings(printSettings: *mut PMPrintSettings) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionDefaultPrintSettings(
        printSession: PMPrintSession,
        printSettings: PMPrintSettings,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSessionValidatePrintSettings(
        printSession: PMPrintSession,
        printSettings: PMPrintSettings,
        changed: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCopyPrintSettings(
        settingSrc: PMPrintSettings,
        settingDest: PMPrintSettings,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsCreateDataRepresentation(
        printSettings: PMPrintSettings,
        data: *mut CFDataRef,
        format: PMDataFormat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsCreateWithDataRepresentation(
        data: CFDataRef,
        printSettings: *mut PMPrintSettings,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetCollate(printSettings: PMPrintSettings, collate: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetCopies(printSettings: PMPrintSettings, copies: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetDuplex(
        printSettings: PMPrintSettings,
        duplexSetting: *mut PMDuplexMode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetFirstPage(printSettings: PMPrintSettings, first: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetLastPage(printSettings: PMPrintSettings, last: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetPageRange(
        printSettings: PMPrintSettings,
        minPage: *mut UInt32,
        maxPage: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsGetJobName(
        printSettings: PMPrintSettings,
        name: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsGetValue(
        printSettings: PMPrintSettings,
        key: CFStringRef,
        value: *mut CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetCollate(printSettings: PMPrintSettings, collate: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetCopies(printSettings: PMPrintSettings, copies: UInt32, lock: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetDuplex(printSettings: PMPrintSettings, duplexSetting: PMDuplexMode) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetFirstPage(printSettings: PMPrintSettings, first: UInt32, lock: Boolean)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetLastPage(printSettings: PMPrintSettings, last: UInt32, lock: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMSetPageRange(
        printSettings: PMPrintSettings,
        minPage: UInt32,
        maxPage: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsSetJobName(printSettings: PMPrintSettings, name: CFStringRef)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsSetValue(
        printSettings: PMPrintSettings,
        key: CFStringRef,
        value: CFTypeRef,
        locked: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsCopyAsDictionary(
        printSettings: PMPrintSettings,
        settingsDictionary: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsCopyKeys(
        printSettings: PMPrintSettings,
        settingsKeys: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCreateGenericPrinter(printer: *mut PMPrinter) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMServerCreatePrinterList(server: PMServer, printerList: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMServerLaunchPrinterBrowser(server: PMServer, options: CFDictionaryRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterCreateFromPrinterID(printerID: CFStringRef) -> PMPrinter;
}
unsafe extern "C" {
    pub fn PMPrinterCopyDescriptionURL(
        printer: PMPrinter,
        descriptionType: CFStringRef,
        fileURL: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterCopyDeviceURI(printer: PMPrinter, deviceURI: *mut CFURLRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterCopyHostName(printer: PMPrinter, hostNameP: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterCopyPresets(printer: PMPrinter, presetList: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetCommInfo(
        printer: PMPrinter,
        supportsControlCharRangeP: *mut Boolean,
        supportsEightBitP: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetID(printer: PMPrinter) -> CFStringRef;
}
unsafe extern "C" {
    pub fn PMPrinterGetLocation(printer: PMPrinter) -> CFStringRef;
}
unsafe extern "C" {
    pub fn PMPrinterGetDriverCreator(printer: PMPrinter, creator: *mut OSType) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetPrinterResolutionCount(printer: PMPrinter, countP: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetIndexedPrinterResolution(
        printer: PMPrinter,
        index: UInt32,
        resolutionP: *mut PMResolution,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetOutputResolution(
        printer: PMPrinter,
        printSettings: PMPrintSettings,
        resolutionP: *mut PMResolution,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterSetOutputResolution(
        printer: PMPrinter,
        printSettings: PMPrintSettings,
        resolutionP: *const PMResolution,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetLanguageInfo(printer: PMPrinter, info: *mut PMLanguageInfo) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetMakeAndModelName(
        printer: PMPrinter,
        makeAndModel: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetMimeTypes(
        printer: PMPrinter,
        settings: PMPrintSettings,
        mimeTypes: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetName(printer: PMPrinter) -> CFStringRef;
}
unsafe extern "C" {
    pub fn PMPrinterGetPaperList(printer: PMPrinter, paperList: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterGetState(printer: PMPrinter, state: *mut PMPrinterState) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterIsDefault(printer: PMPrinter) -> Boolean;
}
unsafe extern "C" {
    pub fn PMPrinterIsFavorite(printer: PMPrinter) -> Boolean;
}
unsafe extern "C" {
    pub fn PMPrinterIsPostScriptCapable(printer: PMPrinter) -> Boolean;
}
unsafe extern "C" {
    pub fn PMPrinterIsPostScriptPrinter(printer: PMPrinter, isPSPrinter: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterIsRemote(printer: PMPrinter, isRemoteP: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterSetDefault(printer: PMPrinter) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPresetCopyName(preset: PMPreset, name: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPresetCreatePrintSettings(
        preset: PMPreset,
        session: PMPrintSession,
        printSettings: *mut PMPrintSettings,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPresetGetAttributes(preset: PMPreset, attributes: *mut CFDictionaryRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMGetPageFormatPaper(format: PMPageFormat, paper: *mut PMPaper) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperCreateCustom(
        printer: PMPrinter,
        id: CFStringRef,
        name: CFStringRef,
        width: f64,
        height: f64,
        margins: *const PMPaperMargins,
        paperP: *mut PMPaper,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperGetWidth(paper: PMPaper, paperWidth: *mut f64) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperGetHeight(paper: PMPaper, paperHeight: *mut f64) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperGetMargins(paper: PMPaper, paperMargins: *mut PMPaperMargins) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperGetID(paper: PMPaper, paperID: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperGetPPDPaperName(paper: PMPaper, paperName: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperCreateLocalizedName(
        paper: PMPaper,
        printer: PMPrinter,
        paperName: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperGetPrinterID(paper: PMPaper, printerID: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPaperIsCustom(paper: PMPaper) -> Boolean;
}
unsafe extern "C" {
    pub fn PMWorkflowCopyItems(workflowItems: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMWorkflowSubmitPDFWithOptions(
        workflowItem: CFURLRef,
        title: CFStringRef,
        options: *const ::std::os::raw::c_char,
        pdfFile: CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMWorkflowSubmitPDFWithSettings(
        workflowItem: CFURLRef,
        settings: PMPrintSettings,
        pdfFile: CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterPrintWithProvider(
        printer: PMPrinter,
        settings: PMPrintSettings,
        format: PMPageFormat,
        mimeType: CFStringRef,
        provider: CGDataProviderRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterPrintWithFile(
        printer: PMPrinter,
        settings: PMPrintSettings,
        format: PMPageFormat,
        mimeType: CFStringRef,
        fileURL: CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterWritePostScriptToURL(
        printer: PMPrinter,
        settings: PMPrintSettings,
        format: PMPageFormat,
        mimeType: CFStringRef,
        sourceFileURL: CFURLRef,
        destinationFileURL: CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsToOptions(
        settings: PMPrintSettings,
        options: *mut *mut ::std::os::raw::c_char,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrintSettingsToOptionsWithPrinterAndPageFormat(
        settings: PMPrintSettings,
        printer: PMPrinter,
        pageFormat: PMPageFormat,
        options: *mut *mut ::std::os::raw::c_char,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterSendCommand(
        printer: PMPrinter,
        commandString: CFStringRef,
        jobTitle: CFStringRef,
        options: CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMPrinterCopyState(printer: PMPrinter, stateDict: *mut CFDictionaryRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCopyAvailablePPDs(domain: PMPPDDomain, ppds: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCopyLocalizedPPD(ppd: CFURLRef, localizedPPD: *mut CFURLRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCopyPPDData(ppd: CFURLRef, data: *mut CFDataRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PMCGImageCreateWithEPSDataProvider(
        epsDataProvider: CGDataProviderRef,
        epsPreview: CGImageRef,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub static kSpeechStatusProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorsProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechInputModeProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechCharacterModeProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechNumberModeProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechRateProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPitchBaseProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPitchModProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechVolumeProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechSynthesizerInfoProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechRecentSyncProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPhonemeSymbolsProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechCurrentVoiceProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechCommandDelimiterProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechResetProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechOutputToFileURLProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechOutputToExtAudioFileProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechOutputToAudioDeviceProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechOutputToFileDescriptorProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechAudioOutputFormatProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechOutputChannelMapProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechRefConProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechTextDoneCallBack: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechSpeechDoneCallBack: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechSyncCallBack: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPhonemeCallBack: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorCFCallBack: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechWordCFCallBack: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPhonemeOptionsProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechAudioUnitProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechAudioGraphProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechSynthExtensionProperty: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechModeText: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechModePhoneme: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechModeTune: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechModeNormal: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechModeLiteral: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechNoEndingProsody: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechNoSpeechInterrupt: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPreflightThenPause: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechStatusOutputBusy: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechStatusOutputPaused: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechStatusNumberOfCharactersLeft: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechStatusPhonemeCode: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorCount: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorOldest: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorOldestCharacterOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorNewest: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorNewestCharacterOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechSynthesizerInfoIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechSynthesizerInfoManufacturer: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechSynthesizerInfoVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPhonemeInfoOpcode: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPhonemeInfoSymbol: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPhonemeInfoExample: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPhonemeInfoHiliteStart: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechPhonemeInfoHiliteEnd: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechVoiceCreator: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechVoiceID: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechCommandPrefix: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechCommandSuffix: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechDictionaryLocaleIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechDictionaryModificationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechDictionaryPronunciations: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechDictionaryAbbreviations: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechDictionaryEntrySpelling: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechDictionaryEntryPhonemes: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorCallbackSpokenString: CFStringRef;
}
unsafe extern "C" {
    pub static kSpeechErrorCallbackCharacterOffset: CFStringRef;
}
unsafe extern "C" {
    pub fn NewSpeechTextDoneUPP(userRoutine: SpeechTextDoneProcPtr) -> SpeechTextDoneUPP;
}
unsafe extern "C" {
    pub fn NewSpeechDoneUPP(userRoutine: SpeechDoneProcPtr) -> SpeechDoneUPP;
}
unsafe extern "C" {
    pub fn NewSpeechSyncUPP(userRoutine: SpeechSyncProcPtr) -> SpeechSyncUPP;
}
unsafe extern "C" {
    pub fn NewSpeechErrorUPP(userRoutine: SpeechErrorProcPtr) -> SpeechErrorUPP;
}
unsafe extern "C" {
    pub fn NewSpeechPhonemeUPP(userRoutine: SpeechPhonemeProcPtr) -> SpeechPhonemeUPP;
}
unsafe extern "C" {
    pub fn NewSpeechWordUPP(userRoutine: SpeechWordProcPtr) -> SpeechWordUPP;
}
unsafe extern "C" {
    pub fn DisposeSpeechTextDoneUPP(userUPP: SpeechTextDoneUPP);
}
unsafe extern "C" {
    pub fn DisposeSpeechDoneUPP(userUPP: SpeechDoneUPP);
}
unsafe extern "C" {
    pub fn DisposeSpeechSyncUPP(userUPP: SpeechSyncUPP);
}
unsafe extern "C" {
    pub fn DisposeSpeechErrorUPP(userUPP: SpeechErrorUPP);
}
unsafe extern "C" {
    pub fn DisposeSpeechPhonemeUPP(userUPP: SpeechPhonemeUPP);
}
unsafe extern "C" {
    pub fn DisposeSpeechWordUPP(userUPP: SpeechWordUPP);
}
unsafe extern "C" {
    pub fn InvokeSpeechTextDoneUPP(
        chan: SpeechChannel,
        refCon: SRefCon,
        nextBuf: *mut *const ::std::os::raw::c_void,
        byteLen: *mut ::std::os::raw::c_ulong,
        controlFlags: *mut SInt32,
        userUPP: SpeechTextDoneUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeSpeechDoneUPP(chan: SpeechChannel, refCon: SRefCon, userUPP: SpeechDoneUPP);
}
unsafe extern "C" {
    pub fn InvokeSpeechSyncUPP(
        chan: SpeechChannel,
        refCon: SRefCon,
        syncMessage: OSType,
        userUPP: SpeechSyncUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeSpeechErrorUPP(
        chan: SpeechChannel,
        refCon: SRefCon,
        theError: OSErr,
        bytePos: ::std::os::raw::c_long,
        userUPP: SpeechErrorUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeSpeechPhonemeUPP(
        chan: SpeechChannel,
        refCon: SRefCon,
        phonemeOpcode: SInt16,
        userUPP: SpeechPhonemeUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeSpeechWordUPP(
        chan: SpeechChannel,
        refCon: SRefCon,
        wordPos: ::std::os::raw::c_ulong,
        wordLen: UInt16,
        userUPP: SpeechWordUPP,
    );
}
unsafe extern "C" {
    pub fn SpeechManagerVersion() -> NumVersion;
}
unsafe extern "C" {
    pub fn MakeVoiceSpec(creator: OSType, id: OSType, voice: *mut VoiceSpec) -> OSErr;
}
unsafe extern "C" {
    pub fn CountVoices(numVoices: *mut SInt16) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIndVoice(index: SInt16, voice: *mut VoiceSpec) -> OSErr;
}
unsafe extern "C" {
    pub fn GetVoiceDescription(
        voice: *const VoiceSpec,
        info: *mut VoiceDescription,
        infoLength: ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetVoiceInfo(
        voice: *const VoiceSpec,
        selector: OSType,
        voiceInfo: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn NewSpeechChannel(voice: *mut VoiceSpec, chan: *mut SpeechChannel) -> OSErr;
}
unsafe extern "C" {
    pub fn DisposeSpeechChannel(chan: SpeechChannel) -> OSErr;
}
unsafe extern "C" {
    pub fn SpeakString(textToBeSpoken: ConstStr255Param) -> OSErr;
}
unsafe extern "C" {
    pub fn SpeakText(
        chan: SpeechChannel,
        textBuf: *const ::std::os::raw::c_void,
        textBytes: ::std::os::raw::c_ulong,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SpeakBuffer(
        chan: SpeechChannel,
        textBuf: *const ::std::os::raw::c_void,
        textBytes: ::std::os::raw::c_ulong,
        controlFlags: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn StopSpeech(chan: SpeechChannel) -> OSErr;
}
unsafe extern "C" {
    pub fn StopSpeechAt(chan: SpeechChannel, whereToStop: SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn PauseSpeechAt(chan: SpeechChannel, whereToPause: SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn ContinueSpeech(chan: SpeechChannel) -> OSErr;
}
unsafe extern "C" {
    pub fn SpeechBusy() -> SInt16;
}
unsafe extern "C" {
    pub fn SpeechBusySystemWide() -> SInt16;
}
unsafe extern "C" {
    pub fn SetSpeechRate(chan: SpeechChannel, rate: Fixed) -> OSErr;
}
unsafe extern "C" {
    pub fn GetSpeechRate(chan: SpeechChannel, rate: *mut Fixed) -> OSErr;
}
unsafe extern "C" {
    pub fn SetSpeechPitch(chan: SpeechChannel, pitch: Fixed) -> OSErr;
}
unsafe extern "C" {
    pub fn GetSpeechPitch(chan: SpeechChannel, pitch: *mut Fixed) -> OSErr;
}
unsafe extern "C" {
    pub fn SetSpeechInfo(
        chan: SpeechChannel,
        selector: OSType,
        speechInfo: *const ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetSpeechInfo(
        chan: SpeechChannel,
        selector: OSType,
        speechInfo: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn TextToPhonemes(
        chan: SpeechChannel,
        textBuf: *const ::std::os::raw::c_void,
        textBytes: ::std::os::raw::c_ulong,
        phonemeBuf: Handle,
        phonemeBytes: *mut ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn UseDictionary(chan: SpeechChannel, dictionary: Handle) -> OSErr;
}
unsafe extern "C" {
    pub fn SpeakCFString(
        chan: SpeechChannel,
        aString: CFStringRef,
        options: CFDictionaryRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn UseSpeechDictionary(chan: SpeechChannel, speechDictionary: CFDictionaryRef) -> OSErr;
}
unsafe extern "C" {
    pub fn CopyPhonemesFromText(
        chan: SpeechChannel,
        text: CFStringRef,
        phonemes: *mut CFStringRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CopySpeechProperty(
        chan: SpeechChannel,
        property: CFStringRef,
        object: *mut CFTypeRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SetSpeechProperty(
        chan: SpeechChannel,
        property: CFStringRef,
        object: CFTypeRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SpeechSynthesisRegisterModuleURL(url: CFURLRef) -> OSErr;
}
unsafe extern "C" {
    pub fn SpeechSynthesisUnregisterModuleURL(url: CFURLRef) -> OSErr;
}

unsafe impl objc2::encode::RefEncode for Float32Point {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Float32Point {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Float32Point", &[]);
}
unsafe impl objc2::encode::RefEncode for ProcessSerialNumber {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ProcessSerialNumber {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ProcessSerialNumber", &[]);
}
unsafe impl objc2::encode::RefEncode for FixedPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FixedPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FixedPoint", &[]);
}
unsafe impl objc2::encode::RefEncode for NumVersion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NumVersion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NumVersion", &[]);
}
unsafe impl objc2::encode::RefEncode for _cups_array_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cups_array_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cups_array_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_attr_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_attr_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_attr_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_choice_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_choice_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_choice_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_option_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_option_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_option_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_group_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_group_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_group_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_const_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_const_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_const_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_size_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_size_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_size_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_emul_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_emul_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_emul_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_profile_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_profile_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_profile_s", &[]);
}
unsafe impl objc2::encode::RefEncode for _ppd_cache_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _ppd_cache_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_ppd_cache_s", &[]);
}
unsafe impl objc2::encode::RefEncode for ppd_file_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ppd_file_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ppd_file_s", &[]);
}
unsafe impl objc2::encode::RefEncode for FMFontFamilyInstance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FMFontFamilyInstance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FMFontFamilyInstance", &[]);
}
unsafe impl objc2::encode::RefEncode for FMFontFamilyIterator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FMFontFamilyIterator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FMFontFamilyIterator", &[]);
}
unsafe impl objc2::encode::RefEncode for FMFontIterator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FMFontIterator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FMFontIterator", &[]);
}
unsafe impl objc2::encode::RefEncode for FMFontFamilyInstanceIterator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FMFontFamilyInstanceIterator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FMFontFamilyInstanceIterator", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFontMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFontMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFontMetrics", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUCurvePath {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUCurvePath {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUCurvePath", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUCurvePaths {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUCurvePaths {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUCurvePaths", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSGlyphIdealMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSGlyphIdealMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSGlyphIdealMetrics", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSGlyphScreenMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSGlyphScreenMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSGlyphScreenMetrics", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSLayoutRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSLayoutRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSLayoutRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSTrapezoid {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSTrapezoid {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSTrapezoid", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSJustWidthDeltaEntryOverride {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSJustWidthDeltaEntryOverride {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSJustWidthDeltaEntryOverride", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFontFamilyIterator_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFontFamilyIterator_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFontFamilyIterator_", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFontIterator_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFontIterator_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFontIterator_", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFontFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFontFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFontFilter", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFontFilter__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFontFilter__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFontFilter__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFontNotificationRef_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFontNotificationRef_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFontNotificationRef_", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFontNotificationInfoRef_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFontNotificationInfoRef_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFontNotificationInfoRef_", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFontQuerySourceContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFontQuerySourceContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFontQuerySourceContext", &[]);
}
unsafe impl objc2::encode::RefEncode for CMDateTime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDateTime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMDateTime", &[]);
}
unsafe impl objc2::encode::RefEncode for CMFixedXYColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMFixedXYColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMFixedXYColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMFixedXYZColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMFixedXYZColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMFixedXYZColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMXYZColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMXYZColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMXYZColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CM2Header {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CM2Header {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CM2Header", &[]);
}
unsafe impl objc2::encode::RefEncode for CM4Header {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CM4Header {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CM4Header", &[]);
}
unsafe impl objc2::encode::RefEncode for CMTagRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTagRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMTagRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for CMTagElemTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTagElemTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMTagElemTable", &[]);
}
unsafe impl objc2::encode::RefEncode for CM2Profile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CM2Profile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CM2Profile", &[]);
}
unsafe impl objc2::encode::RefEncode for CMAdaptationMatrixType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAdaptationMatrixType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMAdaptationMatrixType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMCurveType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMCurveType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMCurveType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMDataType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDataType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMDataType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMDateTimeType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDateTimeType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMDateTimeType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMLut16Type {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMLut16Type {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMLut16Type", &[]);
}
unsafe impl objc2::encode::RefEncode for CMLut8Type {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMLut8Type {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMLut8Type", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMultiFunctLutType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMultiFunctLutType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMultiFunctLutType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMultiFunctCLUTType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMultiFunctCLUTType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMultiFunctCLUTType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMeasurementType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMeasurementType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMeasurementType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMNamedColorType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMNamedColorType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMNamedColorType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMNamedColor2EntryType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMNamedColor2EntryType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMNamedColor2EntryType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMNamedColor2Type {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMNamedColor2Type {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMNamedColor2Type", &[]);
}
unsafe impl objc2::encode::RefEncode for CMNativeDisplayInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMNativeDisplayInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMNativeDisplayInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CMNativeDisplayInfoType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMNativeDisplayInfoType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMNativeDisplayInfoType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMParametricCurveType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMParametricCurveType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMParametricCurveType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMTextDescriptionType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTextDescriptionType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMTextDescriptionType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMTextType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTextType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMTextType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMUnicodeTextType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMUnicodeTextType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMUnicodeTextType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMScreeningChannelRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMScreeningChannelRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMScreeningChannelRec", &[]);
}
unsafe impl objc2::encode::RefEncode for CMScreeningType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMScreeningType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMScreeningType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMSignatureType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMSignatureType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMSignatureType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMS15Fixed16ArrayType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMS15Fixed16ArrayType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMS15Fixed16ArrayType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMU16Fixed16ArrayType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMU16Fixed16ArrayType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMU16Fixed16ArrayType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMUInt8ArrayType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMUInt8ArrayType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMUInt8ArrayType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMUInt16ArrayType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMUInt16ArrayType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMUInt16ArrayType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMUInt32ArrayType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMUInt32ArrayType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMUInt32ArrayType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMUInt64ArrayType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMUInt64ArrayType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMUInt64ArrayType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMViewingConditionsType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMViewingConditionsType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMViewingConditionsType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMXYZType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMXYZType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMXYZType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMProfileSequenceDescType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMProfileSequenceDescType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMProfileSequenceDescType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMUcrBgType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMUcrBgType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMUcrBgType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMIntentCRDVMSize {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIntentCRDVMSize {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIntentCRDVMSize", &[]);
}
unsafe impl objc2::encode::RefEncode for CMPS2CRDVMSizeType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMPS2CRDVMSizeType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMPS2CRDVMSizeType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMVideoCardGammaTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMVideoCardGammaTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMVideoCardGammaTable", &[]);
}
unsafe impl objc2::encode::RefEncode for CMVideoCardGammaFormula {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMVideoCardGammaFormula {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMVideoCardGammaFormula", &[]);
}
unsafe impl objc2::encode::RefEncode for CMVideoCardGamma {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMVideoCardGamma {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMVideoCardGamma", &[]);
}
unsafe impl objc2::encode::RefEncode for CMVideoCardGamma__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMVideoCardGamma__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMVideoCardGamma__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for CMVideoCardGammaType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMVideoCardGammaType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMVideoCardGammaType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMakeAndModel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMakeAndModel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMakeAndModel", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMakeAndModelType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMakeAndModelType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMakeAndModelType", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMultiLocalizedUniCodeEntryRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMultiLocalizedUniCodeEntryRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMultiLocalizedUniCodeEntryRec", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMultiLocalizedUniCodeType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMultiLocalizedUniCodeType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMultiLocalizedUniCodeType", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCMProfileRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCMProfileRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCMProfileRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCMWorldRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCMWorldRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCMWorldRef", &[]);
}
unsafe impl objc2::encode::RefEncode for CMAppleProfileHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAppleProfileHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMAppleProfileHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for CMConcatProfileSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMConcatProfileSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMConcatProfileSet", &[]);
}
unsafe impl objc2::encode::RefEncode for NCMConcatProfileSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NCMConcatProfileSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NCMConcatProfileSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for NCMConcatProfileSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NCMConcatProfileSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NCMConcatProfileSet", &[]);
}
unsafe impl objc2::encode::RefEncode for CMRGBColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMRGBColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMRGBColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMCMYKColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMCMYKColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMCMYKColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMCMYColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMCMYColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMCMYColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMHLSColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMHLSColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMHLSColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMHSVColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMHSVColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMHSVColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMLabColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMLabColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMLabColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMLuvColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMLuvColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMLuvColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMYxyColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMYxyColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMYxyColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMGrayColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMGrayColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMGrayColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMultichannel5Color {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMultichannel5Color {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMultichannel5Color", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMultichannel6Color {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMultichannel6Color {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMultichannel6Color", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMultichannel7Color {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMultichannel7Color {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMultichannel7Color", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMultichannel8Color {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMultichannel8Color {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMultichannel8Color", &[]);
}
unsafe impl objc2::encode::RefEncode for CMNamedColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMNamedColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMNamedColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CMBitmap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMBitmap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMBitmap", &[]);
}
unsafe impl objc2::encode::RefEncode for CMHandleLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMHandleLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMHandleLocation", &[]);
}
unsafe impl objc2::encode::RefEncode for CMPathLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMPathLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMPathLocation", &[]);
}
unsafe impl objc2::encode::RefEncode for CMBufferLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMBufferLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMBufferLocation", &[]);
}
unsafe impl objc2::encode::RefEncode for CMProfLoc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMProfLoc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMProfLoc", &[]);
}
unsafe impl objc2::encode::RefEncode for CMProfileLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMProfileLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMProfileLocation", &[]);
}
unsafe impl objc2::encode::RefEncode for CMProfileIterateData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMProfileIterateData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMProfileIterateData", &[]);
}
unsafe impl objc2::encode::RefEncode for CMFloatBitmap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMFloatBitmap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMFloatBitmap", &[]);
}
unsafe impl objc2::encode::RefEncode for CMDeviceScope {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDeviceScope {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMDeviceScope", &[]);
}
unsafe impl objc2::encode::RefEncode for CMDeviceInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDeviceInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMDeviceInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CMDeviceProfileInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDeviceProfileInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMDeviceProfileInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for NCMDeviceProfileInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NCMDeviceProfileInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NCMDeviceProfileInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CMDeviceProfileArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDeviceProfileArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMDeviceProfileArray", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueGrafPtr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueGrafPtr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueGrafPtr", &[]);
}
unsafe impl objc2::encode::RefEncode for BitMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BitMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BitMap", &[]);
}
unsafe impl objc2::encode::RefEncode for RGBColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RGBColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RGBColor", &[]);
}
unsafe impl objc2::encode::RefEncode for ColorSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ColorSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ColorSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for ColorTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ColorTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ColorTable", &[]);
}
unsafe impl objc2::encode::RefEncode for PixMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PixMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PixMap", &[]);
}
unsafe impl objc2::encode::RefEncode for Pattern {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Pattern {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Pattern", &[]);
}
unsafe impl objc2::encode::RefEncode for PixPat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PixPat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PixPat", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueRgnHandle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueRgnHandle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueRgnHandle", &[]);
}
unsafe impl objc2::encode::RefEncode for GDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GDevice", &[]);
}
unsafe impl objc2::encode::RefEncode for Picture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Picture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Picture", &[]);
}
unsafe impl objc2::encode::RefEncode for OpenCPicParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpenCPicParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpenCPicParams", &[]);
}
unsafe impl objc2::encode::RefEncode for FontInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FontInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FontInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueWindowPtr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueWindowPtr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueWindowPtr", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueDialogPtr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueDialogPtr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueDialogPtr", &[]);
}
unsafe impl objc2::encode::RefEncode for VDGammaRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VDGammaRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VDGammaRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for MacPolygon {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MacPolygon {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MacPolygon", &[]);
}
unsafe impl objc2::encode::RefEncode for CQDProcs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CQDProcs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CQDProcs", &[]);
}
unsafe impl objc2::encode::RefEncode for GrafPort {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GrafPort {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GrafPort", &[]);
}
unsafe impl objc2::encode::RefEncode for __HIShape {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __HIShape {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__HIShape", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueICInstance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueICInstance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueICInstance", &[]);
}
unsafe impl objc2::encode::RefEncode for ICFontRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICFontRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICFontRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for ICCharTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICCharTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICCharTable", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAppSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAppSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAppSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAppSpecList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAppSpecList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAppSpecList", &[]);
}
unsafe impl objc2::encode::RefEncode for ICFileSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICFileSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICFileSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for ICMapEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICMapEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICMapEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for ICServiceEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICServiceEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICServiceEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for ICServices {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICServices {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICServices", &[]);
}
unsafe impl objc2::encode::RefEncode for AppParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AppParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AppParameters", &[]);
}
unsafe impl objc2::encode::RefEncode for AppParameters__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AppParameters__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AppParameters__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for LaunchParamBlockRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LaunchParamBlockRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LaunchParamBlockRec", &[]);
}
unsafe impl objc2::encode::RefEncode for ProcessInfoRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ProcessInfoRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ProcessInfoRec", &[]);
}
unsafe impl objc2::encode::RefEncode for ProcessInfoExtendedRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ProcessInfoExtendedRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ProcessInfoExtendedRec", &[]);
}
unsafe impl objc2::encode::RefEncode for SizeResourceRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SizeResourceRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SizeResourceRec", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePasteboardRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePasteboardRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePasteboardRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueTranslationRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueTranslationRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueTranslationRef", &[]);
}
unsafe impl objc2::encode::RefEncode for __AXUIElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __AXUIElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__AXUIElement", &[]);
}
unsafe impl objc2::encode::RefEncode for __AXTextMarker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __AXTextMarker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__AXTextMarker", &[]);
}
unsafe impl objc2::encode::RefEncode for __AXTextMarkerRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __AXTextMarkerRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__AXTextMarkerRange", &[]);
}
unsafe impl objc2::encode::RefEncode for __AXObserver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __AXObserver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__AXObserver", &[]);
}
unsafe impl objc2::encode::RefEncode for __AXValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __AXValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__AXValue", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePMPrintSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePMPrintSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePMPrintSettings", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePMPageFormat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePMPageFormat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePMPageFormat", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePMPrintSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePMPrintSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePMPrintSession", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePMPrinter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePMPrinter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePMPrinter", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePMServer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePMServer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePMServer", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePMPreset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePMPreset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePMPreset", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePMPaper {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePMPaper {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePMPaper", &[]);
}
unsafe impl objc2::encode::RefEncode for PMRect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PMRect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PMRect", &[]);
}
unsafe impl objc2::encode::RefEncode for PMResolution {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PMResolution {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PMResolution", &[]);
}
unsafe impl objc2::encode::RefEncode for PMLanguageInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PMLanguageInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PMLanguageInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FontRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FontRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FontRec", &[]);
}
unsafe impl objc2::encode::RefEncode for FMInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FMInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FMInput", &[]);
}
unsafe impl objc2::encode::RefEncode for FamRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FamRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FamRec", &[]);
}
unsafe impl objc2::encode::RefEncode for AsscEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AsscEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AsscEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for FontAssoc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FontAssoc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FontAssoc", &[]);
}
unsafe impl objc2::encode::RefEncode for StyleTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for StyleTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("StyleTable", &[]);
}
unsafe impl objc2::encode::RefEncode for NameTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NameTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NameTable", &[]);
}
unsafe impl objc2::encode::RefEncode for KernPair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernPair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernPair", &[]);
}
unsafe impl objc2::encode::RefEncode for KernEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for KernTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernTable", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueATSUTextLayout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueATSUTextLayout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueATSUTextLayout", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueATSUStyle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueATSUStyle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueATSUStyle", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueATSUFontFallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueATSUFontFallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueATSUFontFallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUAttributeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUAttributeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUAttributeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUCaret {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUCaret {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUCaret", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUTab {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUTab {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUTab", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSURGBAlphaColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSURGBAlphaColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSURGBAlphaColor", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUGlyphSelector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUGlyphSelector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUGlyphSelector", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUGlyphInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUGlyphInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUGlyphInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUGlyphInfoArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUGlyphInfoArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUGlyphInfoArray", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUBackgroundData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUBackgroundData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUBackgroundData", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUUnhighlightData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUUnhighlightData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUUnhighlightData", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSUStyleRunInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSUStyleRunInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSUStyleRunInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataMainHeaderBlock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataMainHeaderBlock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataMainHeaderBlock", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataTextLayoutDataHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataTextLayoutDataHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataTextLayoutDataHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataTextLayoutHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataTextLayoutHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataTextLayoutHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataLayoutControlsDataHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataLayoutControlsDataHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataLayoutControlsDataHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataLineInfoData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataLineInfoData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataLineInfoData", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataLineInfoHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataLineInfoHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataLineInfoHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataStyleRunDataHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataStyleRunDataHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataStyleRunDataHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataStyleListStyleDataHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataStyleListStyleDataHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataStyleListStyleDataHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataStyleListHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataStyleListHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataStyleListHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataStyleListFeatureData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataStyleListFeatureData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataStyleListFeatureData", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataStyleListVariationData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataStyleListVariationData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataStyleListVariationData", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataFontNameDataHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataFontNameDataHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataFontNameDataHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataFontSpecRawNameData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataFontSpecRawNameData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataFontSpecRawNameData", &[]);
}
unsafe impl objc2::encode::RefEncode for ATSFlatDataFontSpecRawNameDataHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATSFlatDataFontSpecRawNameDataHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ATSFlatDataFontSpecRawNameDataHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for LLCStyleInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LLCStyleInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LLCStyleInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for SpeechChannelRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SpeechChannelRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SpeechChannelRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for VoiceSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VoiceSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VoiceSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for VoiceDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VoiceDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VoiceDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for VoiceFileInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VoiceFileInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VoiceFileInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for SpeechStatusInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SpeechStatusInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SpeechStatusInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for SpeechErrorInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SpeechErrorInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SpeechErrorInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for SpeechVersionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SpeechVersionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SpeechVersionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for PhonemeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PhonemeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PhonemeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for PhonemeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PhonemeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PhonemeDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for SpeechXtndData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SpeechXtndData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SpeechXtndData", &[]);
}
unsafe impl objc2::encode::RefEncode for DelimiterInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DelimiterInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DelimiterInfo", &[]);
}
