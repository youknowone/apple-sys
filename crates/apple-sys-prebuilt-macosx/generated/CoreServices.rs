#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::ApplicationServices::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::DiskArbitration::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use libc::{dev_t, id_t, mach_port_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type double_t = f64;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UnsignedWide {
    pub lo: UInt32,
    pub hi: UInt32,
}
pub type Fract = SInt32;
pub type UnsignedFixed = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Float80 {
    pub exp: SInt16,
    pub man: [UInt16; 4usize],
}
pub type Ptr = *mut ::std::os::raw::c_char;
pub type Handle = *mut Ptr;
pub type Size = ::std::os::raw::c_long;
pub type LogicalAddress = *mut ::std::os::raw::c_void;
pub type ByteCount = ::std::os::raw::c_ulong;
pub type ByteOffset = ::std::os::raw::c_ulong;
pub type Duration = SInt32;
pub type AbsoluteTime = UnsignedWide;
pub type OptionBits = UInt32;
pub type ItemCount = ::std::os::raw::c_ulong;
pub type PBVersion = UInt32;
pub type ResType = FourCharCode;
pub type ProcPtr = ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_long>;
pub type UniversalProcPtr = ProcPtr;
pub type SRefCon = *mut ::std::os::raw::c_void;
pub type Str255 = [::std::os::raw::c_uchar; 256usize];
pub type Str63 = [::std::os::raw::c_uchar; 64usize];
pub type Str31 = [::std::os::raw::c_uchar; 32usize];
pub type Str15 = [::std::os::raw::c_uchar; 16usize];
pub type StrFileName = Str63;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub v: ::std::os::raw::c_short,
    pub h: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub top: ::std::os::raw::c_short,
    pub left: ::std::os::raw::c_short,
    pub bottom: ::std::os::raw::c_short,
    pub right: ::std::os::raw::c_short,
}
pub type Style = ::std::os::raw::c_uchar;
pub type extended80 = Float80;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HFSUniStr255 {
    pub length: u16,
    pub unicode: [u16; 255usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CustomBadgeResource {
    pub version: SInt16,
    pub customBadgeResourceID: SInt16,
    pub customBadgeType: OSType,
    pub customBadgeCreator: OSType,
    pub windowBadgeType: OSType,
    pub windowBadgeCreator: OSType,
    pub overrideType: OSType,
    pub overrideCreator: OSType,
}
pub type CustomBadgeResourcePtr = *mut CustomBadgeResource;
pub type CustomBadgeResourceHandle = *mut CustomBadgeResourcePtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct RoutingResourceEntry {
    pub creator: OSType,
    pub fileType: OSType,
    pub targetFolder: OSType,
    pub destinationFolder: OSType,
    pub reservedField: OSType,
}
pub type RoutingResourcePtr = *mut RoutingResourceEntry;
pub type RoutingResourceHandle = *mut RoutingResourcePtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FileInfo {
    pub fileType: OSType,
    pub fileCreator: OSType,
    pub finderFlags: UInt16,
    pub location: Point,
    pub reservedField: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FolderInfo {
    pub windowBounds: Rect,
    pub finderFlags: UInt16,
    pub location: Point,
    pub reservedField: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ExtendedFileInfo {
    pub reserved1: [SInt16; 4usize],
    pub extendedFinderFlags: UInt16,
    pub reserved2: SInt16,
    pub putAwayFolderID: SInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ExtendedFolderInfo {
    pub scrollPosition: Point,
    pub reserved1: SInt32,
    pub extendedFinderFlags: UInt16,
    pub reserved2: SInt16,
    pub putAwayFolderID: SInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FInfo {
    pub fdType: OSType,
    pub fdCreator: OSType,
    pub fdFlags: UInt16,
    pub fdLocation: Point,
    pub fdFldr: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FXInfo {
    pub fdIconID: SInt16,
    pub fdReserved: [SInt16; 3usize],
    pub fdScript: SInt8,
    pub fdXFlags: SInt8,
    pub fdComment: SInt16,
    pub fdPutAway: SInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DInfo {
    pub frRect: Rect,
    pub frFlags: UInt16,
    pub frLocation: Point,
    pub frView: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct DXInfo {
    pub frScroll: Point,
    pub frOpenChain: SInt32,
    pub frScript: SInt8,
    pub frXFlags: SInt8,
    pub frComment: SInt16,
    pub frPutAway: SInt32,
}
pub type DateOrders = SInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OffPair {
    pub offFirst: ::std::os::raw::c_short,
    pub offSecond: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Intl0Rec {
    pub decimalPt: ::std::os::raw::c_char,
    pub thousSep: ::std::os::raw::c_char,
    pub listSep: ::std::os::raw::c_char,
    pub currSym1: ::std::os::raw::c_char,
    pub currSym2: ::std::os::raw::c_char,
    pub currSym3: ::std::os::raw::c_char,
    pub currFmt: UInt8,
    pub dateOrder: UInt8,
    pub shrtDateFmt: UInt8,
    pub dateSep: ::std::os::raw::c_char,
    pub timeCycle: UInt8,
    pub timeFmt: UInt8,
    pub mornStr: [::std::os::raw::c_char; 4usize],
    pub eveStr: [::std::os::raw::c_char; 4usize],
    pub timeSep: ::std::os::raw::c_char,
    pub time1Suff: ::std::os::raw::c_char,
    pub time2Suff: ::std::os::raw::c_char,
    pub time3Suff: ::std::os::raw::c_char,
    pub time4Suff: ::std::os::raw::c_char,
    pub time5Suff: ::std::os::raw::c_char,
    pub time6Suff: ::std::os::raw::c_char,
    pub time7Suff: ::std::os::raw::c_char,
    pub time8Suff: ::std::os::raw::c_char,
    pub metricSys: UInt8,
    pub intl0Vers: ::std::os::raw::c_short,
}
pub type Intl0Ptr = *mut Intl0Rec;
pub type Intl0Hndl = *mut Intl0Ptr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Intl1Rec {
    pub days: [Str15; 7usize],
    pub months: [Str15; 12usize],
    pub suppressDay: UInt8,
    pub lngDateFmt: UInt8,
    pub dayLeading0: UInt8,
    pub abbrLen: UInt8,
    pub st0: [::std::os::raw::c_char; 4usize],
    pub st1: [::std::os::raw::c_char; 4usize],
    pub st2: [::std::os::raw::c_char; 4usize],
    pub st3: [::std::os::raw::c_char; 4usize],
    pub st4: [::std::os::raw::c_char; 4usize],
    pub intl1Vers: ::std::os::raw::c_short,
    pub localRtn: [::std::os::raw::c_short; 1usize],
}
pub type Intl1Ptr = *mut Intl1Rec;
pub type Intl1Hndl = *mut Intl1Ptr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct Itl1ExtRec {
    pub base: Intl1Rec,
    pub version: ::std::os::raw::c_short,
    pub format: ::std::os::raw::c_short,
    pub calendarCode: ::std::os::raw::c_short,
    pub extraDaysTableOffset: SInt32,
    pub extraDaysTableLength: SInt32,
    pub extraMonthsTableOffset: SInt32,
    pub extraMonthsTableLength: SInt32,
    pub abbrevDaysTableOffset: SInt32,
    pub abbrevDaysTableLength: SInt32,
    pub abbrevMonthsTableOffset: SInt32,
    pub abbrevMonthsTableLength: SInt32,
    pub extraSepsTableOffset: SInt32,
    pub extraSepsTableLength: SInt32,
    pub tables: [::std::os::raw::c_short; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UntokenTable {
    pub len: ::std::os::raw::c_short,
    pub lastToken: ::std::os::raw::c_short,
    pub index: [::std::os::raw::c_short; 256usize],
}
pub type UntokenTablePtr = *mut UntokenTable;
pub type UntokenTableHandle = *mut UntokenTablePtr;
#[repr(C)]
#[derive(Copy, Clone)]
pub union WideChar {
    pub a: WideChar__bindgen_ty_1,
    pub b: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WideChar__bindgen_ty_1 {
    pub lo: ::std::os::raw::c_char,
    pub hi: ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WideCharArr {
    pub size: ::std::os::raw::c_short,
    pub data: [WideChar; 10usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NumberParts {
    pub version: ::std::os::raw::c_short,
    pub data: [WideChar; 31usize],
    pub pePlus: WideCharArr,
    pub peMinus: WideCharArr,
    pub peMinusPlus: WideCharArr,
    pub altNumTable: WideCharArr,
    pub reserved: [::std::os::raw::c_char; 20usize],
}
pub type NumberPartsPtr = *mut NumberParts;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct Itl4Rec {
    pub flags: ::std::os::raw::c_short,
    pub resourceType: SInt32,
    pub resourceNum: ::std::os::raw::c_short,
    pub version: ::std::os::raw::c_short,
    pub resHeader1: SInt32,
    pub resHeader2: SInt32,
    pub numTables: ::std::os::raw::c_short,
    pub mapOffset: SInt32,
    pub strOffset: SInt32,
    pub fetchOffset: SInt32,
    pub unTokenOffset: SInt32,
    pub defPartsOffset: SInt32,
    pub resOffset6: SInt32,
    pub resOffset7: SInt32,
    pub resOffset8: SInt32,
}
pub type Itl4Ptr = *mut Itl4Rec;
pub type Itl4Handle = *mut Itl4Ptr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct NItl4Rec {
    pub flags: ::std::os::raw::c_short,
    pub resourceType: SInt32,
    pub resourceNum: ::std::os::raw::c_short,
    pub version: ::std::os::raw::c_short,
    pub format: ::std::os::raw::c_short,
    pub resHeader: ::std::os::raw::c_short,
    pub resHeader2: SInt32,
    pub numTables: ::std::os::raw::c_short,
    pub mapOffset: SInt32,
    pub strOffset: SInt32,
    pub fetchOffset: SInt32,
    pub unTokenOffset: SInt32,
    pub defPartsOffset: SInt32,
    pub whtSpListOffset: SInt32,
    pub resOffset7: SInt32,
    pub resOffset8: SInt32,
    pub resLength1: ::std::os::raw::c_short,
    pub resLength2: ::std::os::raw::c_short,
    pub resLength3: ::std::os::raw::c_short,
    pub unTokenLength: ::std::os::raw::c_short,
    pub defPartsLength: ::std::os::raw::c_short,
    pub whtSpListLength: ::std::os::raw::c_short,
    pub resLength7: ::std::os::raw::c_short,
    pub resLength8: ::std::os::raw::c_short,
}
pub type NItl4Ptr = *mut NItl4Rec;
pub type NItl4Handle = *mut NItl4Ptr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TableDirectoryRecord {
    pub tableSignature: OSType,
    pub reserved: UInt32,
    pub tableStartOffset: UInt32,
    pub tableSize: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct Itl5Record {
    pub versionNumber: Fixed,
    pub numberOfTables: ::std::os::raw::c_ushort,
    pub reserved: [::std::os::raw::c_ushort; 3usize],
    pub tableDirectory: [TableDirectoryRecord; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RuleBasedTrslRecord {
    pub sourceType: ::std::os::raw::c_short,
    pub targetType: ::std::os::raw::c_short,
    pub formatNumber: ::std::os::raw::c_short,
    pub propertyFlag: ::std::os::raw::c_short,
    pub numberOfRules: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ItlcRecord {
    pub itlcSystem: ::std::os::raw::c_short,
    pub itlcReserved: ::std::os::raw::c_short,
    pub itlcFontForce: SInt8,
    pub itlcIntlForce: SInt8,
    pub itlcOldKybd: SInt8,
    pub itlcFlags: SInt8,
    pub itlcIconOffset: ::std::os::raw::c_short,
    pub itlcIconSide: SInt8,
    pub itlcIconRsvd: SInt8,
    pub itlcRegionCode: ::std::os::raw::c_short,
    pub itlcSysFlags: ::std::os::raw::c_short,
    pub itlcReserved4: [SInt8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ItlbRecord {
    pub itlbNumber: ::std::os::raw::c_short,
    pub itlbDate: ::std::os::raw::c_short,
    pub itlbSort: ::std::os::raw::c_short,
    pub itlbFlags: ::std::os::raw::c_short,
    pub itlbToken: ::std::os::raw::c_short,
    pub itlbEncoding: ::std::os::raw::c_short,
    pub itlbLang: ::std::os::raw::c_short,
    pub itlbNumRep: SInt8,
    pub itlbDateRep: SInt8,
    pub itlbKeys: ::std::os::raw::c_short,
    pub itlbIcon: ::std::os::raw::c_short,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ItlbExtRecord {
    pub base: ItlbRecord,
    pub itlbLocalSize: SInt32,
    pub itlbMonoFond: ::std::os::raw::c_short,
    pub itlbMonoSize: ::std::os::raw::c_short,
    pub itlbPrefFond: ::std::os::raw::c_short,
    pub itlbPrefSize: ::std::os::raw::c_short,
    pub itlbSmallFond: ::std::os::raw::c_short,
    pub itlbSmallSize: ::std::os::raw::c_short,
    pub itlbSysFond: ::std::os::raw::c_short,
    pub itlbSysSize: ::std::os::raw::c_short,
    pub itlbAppFond: ::std::os::raw::c_short,
    pub itlbAppSize: ::std::os::raw::c_short,
    pub itlbHelpFond: ::std::os::raw::c_short,
    pub itlbHelpSize: ::std::os::raw::c_short,
    pub itlbValidStyles: Style,
    pub itlbAliasStyle: Style,
}
pub type ScriptTokenType = ::std::os::raw::c_short;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TokenRec {
    pub theToken: ScriptTokenType,
    pub position: Ptr,
    pub length: ::std::os::raw::c_long,
    pub stringPosition: StringPtr,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TokenBlock {
    pub source: Ptr,
    pub sourceLength: ::std::os::raw::c_long,
    pub tokenList: Ptr,
    pub tokenLength: ::std::os::raw::c_long,
    pub tokenCount: ::std::os::raw::c_long,
    pub stringList: Ptr,
    pub stringLength: ::std::os::raw::c_long,
    pub stringCount: ::std::os::raw::c_long,
    pub doString: Boolean,
    pub doAppend: Boolean,
    pub doAlphanumeric: Boolean,
    pub doNest: Boolean,
    pub leftDelims: [ScriptTokenType; 2usize],
    pub rightDelims: [ScriptTokenType; 2usize],
    pub leftComment: [ScriptTokenType; 4usize],
    pub rightComment: [ScriptTokenType; 4usize],
    pub escapeCode: ScriptTokenType,
    pub decimalCode: ScriptTokenType,
    pub itlResource: Handle,
    pub reserved: [::std::os::raw::c_long; 8usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UTCDateTime {
    pub highSeconds: UInt16,
    pub lowSeconds: UInt32,
    pub fraction: UInt16,
}
pub type UTCDateTimePtr = *mut UTCDateTime;
pub type UTCDateTimeHandle = *mut UTCDateTimePtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LocalDateTime {
    pub highSeconds: UInt16,
    pub lowSeconds: UInt32,
    pub fraction: UInt16,
}
pub type LocalDateTimePtr = *mut LocalDateTime;
pub type LocalDateTimeHandle = *mut LocalDateTimePtr;
pub type TextEncodingBase = UInt32;
pub type TextEncodingVariant = UInt32;
pub type TextEncodingFormat = UInt32;
pub type TextEncoding = UInt32;
pub type TextEncodingNameSelector = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TextEncodingRun {
    pub offset: ByteOffset,
    pub textEncoding: TextEncoding,
}
pub type TextEncodingRunPtr = *mut TextEncodingRun;
pub type ConstTextEncodingRunPtr = *const TextEncodingRun;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ScriptCodeRun {
    pub offset: ByteOffset,
    pub script: ScriptCode,
}
pub type ScriptCodeRunPtr = *mut ScriptCodeRun;
pub type ConstScriptCodeRunPtr = *const ScriptCodeRun;
pub type TextPtr = *mut UInt8;
pub type ConstTextPtr = *const UInt8;
pub type UniCharArrayPtr = *mut UniChar;
pub type ConstUniCharArrayPtr = *const UniChar;
pub type UniCharArrayHandle = *mut UniCharArrayPtr;
pub type UniCharArrayOffset = ::std::os::raw::c_ulong;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TECInfo {
    pub format: UInt16,
    pub tecVersion: UInt16,
    pub tecTextConverterFeatures: UInt32,
    pub tecUnicodeConverterFeatures: UInt32,
    pub tecTextCommonFeatures: UInt32,
    pub tecTextEncodingsFolderName: Str31,
    pub tecExtensionFileName: Str31,
    pub tecLowestTEFileVersion: UInt16,
    pub tecHighestTEFileVersion: UInt16,
}
pub type TECInfoPtr = *mut TECInfo;
pub type TECInfoHandle = *mut TECInfoPtr;
pub type UCCharPropertyType = SInt32;
pub type UCCharPropertyValue = UInt32;
pub type CallingConventionType = ::std::os::raw::c_ushort;
pub type ISAType = SInt8;
pub type RTAType = SInt8;
pub type registerSelectorType = ::std::os::raw::c_ushort;
pub type ProcInfoType = ::std::os::raw::c_ulong;
pub type RoutineFlagsType = ::std::os::raw::c_ushort;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct RoutineRecord {
    pub procInfo: ProcInfoType,
    pub reserved1: SInt8,
    pub ISA: ISAType,
    pub routineFlags: RoutineFlagsType,
    pub procDescriptor: ProcPtr,
    pub reserved2: UInt32,
    pub selector: UInt32,
}
pub type RoutineRecordPtr = *mut RoutineRecord;
pub type RoutineRecordHandle = *mut RoutineRecordPtr;
pub type RDFlagsType = UInt8;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct RoutineDescriptor {
    pub goMixedModeTrap: UInt16,
    pub version: SInt8,
    pub routineDescriptorFlags: RDFlagsType,
    pub reserved1: UInt32,
    pub reserved2: UInt8,
    pub selectorInfo: UInt8,
    pub routineCount: UInt16,
    pub routineRecords: [RoutineRecord; 1usize],
}
pub type RoutineDescriptorPtr = *mut RoutineDescriptor;
pub type RoutineDescriptorHandle = *mut RoutineDescriptorPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MixedModeStateRecord {
    pub state1: UInt32,
    pub state2: UInt32,
    pub state3: UInt32,
    pub state4: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCollection {
    _unused: [u8; 0],
}
pub type Collection = *mut OpaqueCollection;
pub type CollectionTag = FourCharCode;
pub type CollectionFlattenProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        size: SInt32,
        data: *mut ::std::os::raw::c_void,
        refCon: *mut ::std::os::raw::c_void,
    ) -> OSErr,
>;
pub type CollectionExceptionProcPtr =
    ::std::option::Option<unsafe extern "C" fn(c: Collection, status: OSErr) -> OSErr>;
pub type CollectionFlattenUPP = CollectionFlattenProcPtr;
pub type CollectionExceptionUPP = CollectionExceptionProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct BigEndianUInt32 {
    pub bigEndianValue: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct BigEndianLong {
    pub bigEndianValue: ::std::os::raw::c_long,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct BigEndianUnsignedLong {
    pub bigEndianValue: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BigEndianShort {
    pub bigEndianValue: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BigEndianUnsignedShort {
    pub bigEndianValue: ::std::os::raw::c_ushort,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct BigEndianFixed {
    pub bigEndianValue: Fixed,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct BigEndianUnsignedFixed {
    pub bigEndianValue: UnsignedFixed,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct BigEndianOSType {
    pub bigEndianValue: OSType,
}
pub type SelectorFunctionProcPtr =
    ::std::option::Option<unsafe extern "C" fn(selector: OSType, response: *mut SInt32) -> OSErr>;
pub type SelectorFunctionUPP = SelectorFunctionProcPtr;
pub type CSDiskSpaceRecoveryOptions = ::std::os::raw::c_int;
pub type ToggleResults = SInt16;
pub type LongDateField = SInt8;
pub type DateForm = SInt8;
pub type StringToDateStatus = ::std::os::raw::c_short;
pub type String2DateStatus = StringToDateStatus;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DateCacheRecord {
    pub hidden: [::std::os::raw::c_short; 256usize],
}
pub type DateCachePtr = *mut DateCacheRecord;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DateTimeRec {
    pub year: ::std::os::raw::c_short,
    pub month: ::std::os::raw::c_short,
    pub day: ::std::os::raw::c_short,
    pub hour: ::std::os::raw::c_short,
    pub minute: ::std::os::raw::c_short,
    pub second: ::std::os::raw::c_short,
    pub dayOfWeek: ::std::os::raw::c_short,
}
pub type LongDateTime = SInt64;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union LongDateCvt {
    pub __bindgen_anon_1: LongDateCvt__bindgen_ty_1,
    pub c: SInt64,
    pub hl: LongDateCvt__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LongDateCvt__bindgen_ty_1 {
    pub lLow: UInt32,
    pub lHigh: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union LongDateRec {
    pub __bindgen_anon_1: LongDateRec__bindgen_ty_1,
    pub __bindgen_anon_2: LongDateRec__bindgen_ty_2,
    pub ld: LongDateRec__bindgen_ty_1,
    pub list: [::std::os::raw::c_short; 14usize],
    pub od: LongDateRec__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LongDateRec__bindgen_ty_1 {
    pub era: ::std::os::raw::c_short,
    pub year: ::std::os::raw::c_short,
    pub month: ::std::os::raw::c_short,
    pub day: ::std::os::raw::c_short,
    pub hour: ::std::os::raw::c_short,
    pub minute: ::std::os::raw::c_short,
    pub second: ::std::os::raw::c_short,
    pub dayOfWeek: ::std::os::raw::c_short,
    pub dayOfYear: ::std::os::raw::c_short,
    pub weekOfYear: ::std::os::raw::c_short,
    pub pm: ::std::os::raw::c_short,
    pub res1: ::std::os::raw::c_short,
    pub res2: ::std::os::raw::c_short,
    pub res3: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LongDateRec__bindgen_ty_2 {
    pub eraAlt: ::std::os::raw::c_short,
    pub oldDate: DateTimeRec,
}
pub type DateDelta = SInt8;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TogglePB {
    pub togFlags: ::std::os::raw::c_long,
    pub amChars: ResType,
    pub pmChars: ResType,
    pub reserved: [::std::os::raw::c_long; 4usize],
}
pub type QTypes = SignedByte;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct QElem {
    pub qLink: *mut QElem,
    pub qType: ::std::os::raw::c_short,
    pub qData: [::std::os::raw::c_short; 1usize],
}
pub type QElemPtr = *mut QElem;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct QHdr {
    pub qFlags: ::std::os::raw::c_short,
    pub qHead: QElemPtr,
    pub qTail: QElemPtr,
}
pub type QHdrPtr = *mut QHdr;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct MachineLocation {
    pub __bindgen_anon_1: MachineLocation__bindgen_ty_1,
    pub latitude: Fract,
    pub longitude: Fract,
    pub u: MachineLocation__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union MachineLocation__bindgen_ty_1 {
    pub __bindgen_anon_1: MachineLocation__bindgen_ty_1__bindgen_ty_1,
    pub gmtDelta: ::std::os::raw::c_long,
    pub dls: MachineLocation__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MachineLocation__bindgen_ty_1__bindgen_ty_1 {
    pub pad: [SInt8; 3usize],
    pub Delta: SInt8,
}
pub type SysPPtr = *mut ::std::os::raw::c_void;
pub type DeferredTaskProcPtr =
    ::std::option::Option<unsafe extern "C" fn(dtParam: ::std::os::raw::c_long)>;
pub type DeferredTaskUPP = DeferredTaskProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct DeferredTask {
    pub qLink: QElemPtr,
    pub qType: ::std::os::raw::c_short,
    pub dtFlags: ::std::os::raw::c_short,
    pub dtAddr: DeferredTaskUPP,
    pub dtParam: ::std::os::raw::c_long,
    pub dtReserved: ::std::os::raw::c_long,
}
pub type DeferredTaskPtr = *mut DeferredTask;
pub type FSVolumeRefNum = SInt16;
pub type FSIORefNum = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FSRef {
    pub hidden: [UInt8; 80usize],
}
pub type FSRefPtr = *mut FSRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __FSFileSecurity {
    _unused: [u8; 0],
}
pub type FSFileSecurityRef = *mut __FSFileSecurity;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CatPositionRec {
    pub initialize: SInt32,
    pub priv_: [SInt16; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FSSpec {
    pub hidden: [UInt8; 70usize],
}
pub type FSSpecPtr = *mut FSSpec;
pub type FSSpecHandle = *mut FSSpecPtr;
pub type FSSpecArrayPtr = FSSpecPtr;
pub type ConstFSSpecPtr = *const FSSpec;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParamBlockRec {
    _unused: [u8; 0],
}
pub type ParmBlkPtr = *mut ::std::os::raw::c_void;
pub type IOCompletionProcPtr = ::std::option::Option<unsafe extern "C" fn(paramBlock: ParmBlkPtr)>;
pub type IOCompletionUPP = IOCompletionProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSPermissionInfo {
    pub userID: UInt32,
    pub groupID: UInt32,
    pub reserved1: UInt8,
    pub userAccess: UInt8,
    pub mode: UInt16,
    pub fileSec: FSFileSecurityRef,
}
pub type FSCatalogInfoBitmap = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSCatalogInfo {
    pub nodeFlags: UInt16,
    pub volume: FSVolumeRefNum,
    pub parentDirID: UInt32,
    pub nodeID: UInt32,
    pub sharingFlags: UInt8,
    pub userPrivileges: UInt8,
    pub reserved1: UInt8,
    pub reserved2: UInt8,
    pub createDate: UTCDateTime,
    pub contentModDate: UTCDateTime,
    pub attributeModDate: UTCDateTime,
    pub accessDate: UTCDateTime,
    pub backupDate: UTCDateTime,
    pub permissions: FSPermissionInfo,
    pub finderInfo: [UInt8; 16usize],
    pub extFinderInfo: [UInt8; 16usize],
    pub dataLogicalSize: UInt64,
    pub dataPhysicalSize: UInt64,
    pub rsrcLogicalSize: UInt64,
    pub rsrcPhysicalSize: UInt64,
    pub valence: UInt32,
    pub textEncodingHint: TextEncoding,
}
pub type FSCatalogInfoPtr = *mut FSCatalogInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSRefParam {
    pub qLink: QElemPtr,
    pub qType: SInt16,
    pub ioTrap: SInt16,
    pub ioCmdAddr: Ptr,
    pub ioCompletion: IOCompletionUPP,
    pub ioResult: OSErr,
    pub ioNamePtr: ConstStringPtr,
    pub ioVRefNum: FSVolumeRefNum,
    pub reserved1: SInt16,
    pub reserved2: UInt8,
    pub reserved3: UInt8,
    pub ref_: *const FSRef,
    pub whichInfo: FSCatalogInfoBitmap,
    pub catInfo: *mut FSCatalogInfo,
    pub nameLength: UniCharCount,
    pub name: *const UniChar,
    pub ioDirID: UInt32,
    pub spec: FSSpecPtr,
    pub parentRef: *mut FSRef,
    pub newRef: *mut FSRef,
    pub textEncodingHint: TextEncoding,
    pub outName: *mut HFSUniStr255,
}
pub type FSRefParamPtr = *mut FSRefParam;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSRefForkIOParam {
    pub qLink: QElemPtr,
    pub qType: SInt16,
    pub ioTrap: SInt16,
    pub ioCmdAddr: Ptr,
    pub ioCompletion: IOCompletionUPP,
    pub ioResult: OSErr,
    pub parentRef: *const FSRef,
    pub nameLength: UniCharCount,
    pub name: *const UniChar,
    pub whichInfo: FSCatalogInfoBitmap,
    pub catInfo: *const FSCatalogInfo,
    pub forkNameLength: UniCharCount,
    pub forkName: *const UniChar,
    pub permissions: SInt8,
    pub reserved1: UInt8,
    pub forkRefNum: FSIORefNum,
    pub newRef: *mut FSRef,
}
pub type FSRefForkIOParamPtr = *mut FSRefForkIOParam;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueFSIterator {
    _unused: [u8; 0],
}
pub type FSIterator = *mut OpaqueFSIterator;
pub type FSIteratorFlags = OptionBits;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSSearchParams {
    pub searchTime: Duration,
    pub searchBits: OptionBits,
    pub searchNameLength: UniCharCount,
    pub searchName: *const UniChar,
    pub searchInfo1: *mut FSCatalogInfo,
    pub searchInfo2: *mut FSCatalogInfo,
}
pub type FSSearchParamsPtr = *mut FSSearchParams;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSCatalogBulkParam {
    pub qLink: QElemPtr,
    pub qType: SInt16,
    pub ioTrap: SInt16,
    pub ioCmdAddr: Ptr,
    pub ioCompletion: IOCompletionUPP,
    pub ioResult: OSErr,
    pub containerChanged: Boolean,
    pub reserved: UInt8,
    pub iteratorFlags: FSIteratorFlags,
    pub iterator: FSIterator,
    pub container: *const FSRef,
    pub maximumItems: ItemCount,
    pub actualItems: ItemCount,
    pub whichInfo: FSCatalogInfoBitmap,
    pub catalogInfo: *mut FSCatalogInfo,
    pub refs: *mut FSRef,
    pub specs: FSSpecPtr,
    pub names: *mut HFSUniStr255,
    pub searchParams: *const FSSearchParams,
}
pub type FSCatalogBulkParamPtr = *mut FSCatalogBulkParam;
pub type FSAllocationFlags = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSForkIOParam {
    pub qLink: QElemPtr,
    pub qType: SInt16,
    pub ioTrap: SInt16,
    pub ioCmdAddr: Ptr,
    pub ioCompletion: IOCompletionUPP,
    pub ioResult: OSErr,
    pub reserved1: *mut ::std::os::raw::c_void,
    pub reserved2: SInt16,
    pub forkRefNum: FSIORefNum,
    pub reserved3: UInt8,
    pub permissions: SInt8,
    pub ref_: *const FSRef,
    pub buffer: Ptr,
    pub requestCount: UInt32,
    pub actualCount: UInt32,
    pub positionMode: UInt16,
    pub positionOffset: SInt64,
    pub allocationFlags: FSAllocationFlags,
    pub allocationAmount: UInt64,
    pub forkNameLength: UniCharCount,
    pub forkName: *const UniChar,
    pub forkIterator: CatPositionRec,
    pub outForkName: *mut HFSUniStr255,
}
pub type FSForkIOParamPtr = *mut FSForkIOParam;
pub type FSForkInfoFlags = UInt8;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSForkInfo {
    pub flags: FSForkInfoFlags,
    pub permissions: SInt8,
    pub volume: FSVolumeRefNum,
    pub reserved2: UInt32,
    pub nodeID: UInt32,
    pub forkID: UInt32,
    pub currentPosition: UInt64,
    pub logicalEOF: UInt64,
    pub physicalEOF: UInt64,
    pub process: UInt64,
}
pub type FSForkInfoPtr = *mut FSForkInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSForkCBInfoParam {
    pub qLink: QElemPtr,
    pub qType: SInt16,
    pub ioTrap: SInt16,
    pub ioCmdAddr: Ptr,
    pub ioCompletion: IOCompletionUPP,
    pub ioResult: OSErr,
    pub desiredRefNum: FSIORefNum,
    pub volumeRefNum: FSVolumeRefNum,
    pub iterator: FSIORefNum,
    pub actualRefNum: FSVolumeRefNum,
    pub ref_: *mut FSRef,
    pub forkInfo: *mut FSForkInfo,
    pub forkName: *mut HFSUniStr255,
}
pub type FSForkCBInfoParamPtr = *mut FSForkCBInfoParam;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSRangeLockParam {
    pub qLink: QElemPtr,
    pub qType: SInt16,
    pub ioTrap: SInt16,
    pub ioCmdAddr: Ptr,
    pub ioCompletion: IOCompletionUPP,
    pub ioResult: OSErr,
    pub forkRefNum: FSIORefNum,
    pub requestCount: UInt64,
    pub positionMode: UInt16,
    pub positionOffset: SInt64,
    pub rangeStart: UInt64,
}
pub type FSRangeLockParamPtr = *mut FSRangeLockParam;
pub type FSVolumeInfoBitmap = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSVolumeInfo {
    pub createDate: UTCDateTime,
    pub modifyDate: UTCDateTime,
    pub backupDate: UTCDateTime,
    pub checkedDate: UTCDateTime,
    pub fileCount: UInt32,
    pub folderCount: UInt32,
    pub totalBytes: UInt64,
    pub freeBytes: UInt64,
    pub blockSize: UInt32,
    pub totalBlocks: UInt32,
    pub freeBlocks: UInt32,
    pub nextAllocation: UInt32,
    pub rsrcClumpSize: UInt32,
    pub dataClumpSize: UInt32,
    pub nextCatalogID: UInt32,
    pub finderInfo: [UInt8; 32usize],
    pub flags: UInt16,
    pub filesystemID: UInt16,
    pub signature: UInt16,
    pub driveNumber: UInt16,
    pub driverRefNum: FSIORefNum,
}
pub type FSVolumeInfoPtr = *mut FSVolumeInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSVolumeInfoParam {
    pub qLink: QElemPtr,
    pub qType: SInt16,
    pub ioTrap: SInt16,
    pub ioCmdAddr: Ptr,
    pub ioCompletion: IOCompletionUPP,
    pub ioResult: OSErr,
    pub ioNamePtr: StringPtr,
    pub ioVRefNum: FSVolumeRefNum,
    pub volumeIndex: UInt32,
    pub whichInfo: FSVolumeInfoBitmap,
    pub volumeInfo: *mut FSVolumeInfo,
    pub volumeName: *mut HFSUniStr255,
    pub ref_: *mut FSRef,
}
pub type FSVolumeInfoParamPtr = *mut FSVolumeInfoParam;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct GetVolParmsInfoBuffer {
    pub vMVersion: SInt16,
    pub vMAttrib: SInt32,
    pub vMLocalHand: Handle,
    pub vMServerAdr: SInt32,
    pub vMVolumeGrade: SInt32,
    pub vMForeignPrivID: SInt16,
    pub vMExtendedAttributes: SInt32,
    pub vMDeviceID: *mut ::std::os::raw::c_void,
    pub vMMaxNameLength: UniCharCount,
}
pub type VolumeType = OSType;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct VolMountInfoHeader {
    pub length: SInt16,
    pub media: VolumeType,
}
pub type VolMountInfoPtr = *mut VolMountInfoHeader;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct VolumeMountInfoHeader {
    pub length: SInt16,
    pub media: VolumeType,
    pub flags: SInt16,
}
pub type VolumeMountInfoHeaderPtr = *mut VolumeMountInfoHeader;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AFPVolMountInfo {
    pub length: SInt16,
    pub media: VolumeType,
    pub flags: SInt16,
    pub nbpInterval: SInt8,
    pub nbpCount: SInt8,
    pub uamType: SInt16,
    pub zoneNameOffset: SInt16,
    pub serverNameOffset: SInt16,
    pub volNameOffset: SInt16,
    pub userNameOffset: SInt16,
    pub userPasswordOffset: SInt16,
    pub volPasswordOffset: SInt16,
    pub AFPData: [::std::os::raw::c_char; 144usize],
}
pub type AFPVolMountInfoPtr = *mut AFPVolMountInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AFPXVolMountInfo {
    pub length: SInt16,
    pub media: VolumeType,
    pub flags: SInt16,
    pub nbpInterval: SInt8,
    pub nbpCount: SInt8,
    pub uamType: SInt16,
    pub zoneNameOffset: SInt16,
    pub serverNameOffset: SInt16,
    pub volNameOffset: SInt16,
    pub userNameOffset: SInt16,
    pub userPasswordOffset: SInt16,
    pub volPasswordOffset: SInt16,
    pub extendedFlags: SInt16,
    pub uamNameOffset: SInt16,
    pub alternateAddressOffset: SInt16,
    pub AFPData: [::std::os::raw::c_char; 176usize],
}
pub type AFPXVolMountInfoPtr = *mut AFPXVolMountInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AFPTagData {
    pub fLength: UInt8,
    pub fType: UInt8,
    pub fData: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AFPAlternateAddress {
    pub fVersion: UInt8,
    pub fAddressCount: UInt8,
    pub fAddressList: [UInt8; 1usize],
}
pub type FNMessage = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueFNSubscriptionRef {
    _unused: [u8; 0],
}
pub type FNSubscriptionRef = *mut OpaqueFNSubscriptionRef;
pub type FNSubscriptionProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        message: FNMessage,
        flags: OptionBits,
        refcon: *mut ::std::os::raw::c_void,
        subscription: FNSubscriptionRef,
    ),
>;
pub type FNSubscriptionUPP = FNSubscriptionProcPtr;
pub type FSMountStatus = UInt32;
pub type FSEjectStatus = UInt32;
pub type FSUnmountStatus = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueFSVolumeOperation {
    _unused: [u8; 0],
}
pub type FSVolumeOperation = *mut OpaqueFSVolumeOperation;
pub type FSVolumeMountProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        err: OSStatus,
        mountedVolumeRefNum: FSVolumeRefNum,
    ),
>;
pub type FSVolumeUnmountProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        err: OSStatus,
        volumeRefNum: FSVolumeRefNum,
        dissenter: pid_t,
    ),
>;
pub type FSVolumeEjectProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        err: OSStatus,
        volumeRefNum: FSVolumeRefNum,
        dissenter: pid_t,
    ),
>;
pub type FSVolumeMountUPP = FSVolumeMountProcPtr;
pub type FSVolumeUnmountUPP = FSVolumeUnmountProcPtr;
pub type FSVolumeEjectUPP = FSVolumeEjectProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __FSFileOperation {
    _unused: [u8; 0],
}
pub type FSFileOperationRef = *mut __FSFileOperation;
pub type FSFileOperationStage = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSFileOperationClientContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
}
pub type ResID = SInt16;
pub type ResAttributes = SInt16;
pub type ResFileAttributes = SInt16;
pub type ResourceCount = SInt16;
pub type ResourceIndex = SInt16;
pub type ResFileRefNum = FSIORefNum;
pub type ResErrProcPtr = ::std::option::Option<unsafe extern "C" fn(thErr: OSErr)>;
pub type ResErrUPP = ResErrProcPtr;
pub type RsrcChainLocation = SInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentDescription {
    pub componentType: OSType,
    pub componentSubType: OSType,
    pub componentManufacturer: OSType,
    pub componentFlags: UInt32,
    pub componentFlagsMask: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ResourceSpec {
    pub resType: OSType,
    pub resID: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ComponentResource {
    pub cd: ComponentDescription,
    pub component: ResourceSpec,
    pub componentName: ResourceSpec,
    pub componentInfo: ResourceSpec,
    pub componentIcon: ResourceSpec,
}
pub type ComponentResourcePtr = *mut ComponentResource;
pub type ComponentResourceHandle = *mut ComponentResourcePtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentPlatformInfo {
    pub componentFlags: SInt32,
    pub component: ResourceSpec,
    pub platformType: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentResourceExtension {
    pub componentVersion: SInt32,
    pub componentRegisterFlags: SInt32,
    pub componentIconFamily: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentPlatformInfoArray {
    pub count: SInt32,
    pub platformArray: [ComponentPlatformInfo; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ExtComponentResource {
    pub cd: ComponentDescription,
    pub component: ResourceSpec,
    pub componentName: ResourceSpec,
    pub componentInfo: ResourceSpec,
    pub componentIcon: ResourceSpec,
    pub componentVersion: SInt32,
    pub componentRegisterFlags: SInt32,
    pub componentIconFamily: SInt16,
    pub count: SInt32,
    pub platformArray: [ComponentPlatformInfo; 1usize],
}
pub type ExtComponentResourcePtr = *mut ExtComponentResource;
pub type ExtComponentResourceHandle = *mut ExtComponentResourcePtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ComponentAliasResource {
    pub cr: ComponentResource,
    pub aliasCD: ComponentDescription,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentParameters {
    pub flags: UInt8,
    pub paramSize: UInt8,
    pub what: SInt16,
    pub padding: UInt32,
    pub params: [::std::os::raw::c_long; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentRecord {
    pub data: [::std::os::raw::c_long; 1usize],
}
pub type Component = *mut ComponentRecord;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentInstanceRecord {
    pub data: [::std::os::raw::c_long; 1usize],
}
pub type ComponentInstance = *mut ComponentInstanceRecord;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct RegisteredComponentRecord {
    pub data: [::std::os::raw::c_long; 1usize],
}
pub type RegisteredComponentRecordPtr = *mut RegisteredComponentRecord;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct RegisteredComponentInstanceRecord {
    pub data: [::std::os::raw::c_long; 1usize],
}
pub type RegisteredComponentInstanceRecordPtr = *mut RegisteredComponentInstanceRecord;
pub type ComponentResult = SInt32;
pub type CSComponentsThreadMode = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentMPWorkFunctionHeaderRecord {
    pub headerSize: UInt32,
    pub recordSize: UInt32,
    pub workFlags: UInt32,
    pub processorCount: UInt16,
    pub unused: UInt8,
    pub isRunning: UInt8,
}
pub type ComponentMPWorkFunctionHeaderRecordPtr = *mut ComponentMPWorkFunctionHeaderRecord;
pub type ComponentMPWorkFunctionProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        globalRefCon: *mut ::std::os::raw::c_void,
        header: ComponentMPWorkFunctionHeaderRecordPtr,
    ) -> ComponentResult,
>;
pub type ComponentRoutineProcPtr = ::std::option::Option<
    unsafe extern "C" fn(cp: *mut ComponentParameters, componentStorage: Handle) -> ComponentResult,
>;
pub type GetMissingComponentResourceProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        c: Component,
        resType: OSType,
        resID: SInt16,
        refCon: *mut ::std::os::raw::c_void,
        resource: *mut Handle,
    ) -> OSErr,
>;
pub type ComponentMPWorkFunctionUPP = ComponentMPWorkFunctionProcPtr;
pub type ComponentRoutineUPP = ComponentRoutineProcPtr;
pub type GetMissingComponentResourceUPP = GetMissingComponentResourceProcPtr;
pub type ComponentFunctionUPP = UniversalProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPProcessID {
    _unused: [u8; 0],
}
pub type MPProcessID = *mut OpaqueMPProcessID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPTaskID {
    _unused: [u8; 0],
}
pub type MPTaskID = *mut OpaqueMPTaskID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPQueueID {
    _unused: [u8; 0],
}
pub type MPQueueID = *mut OpaqueMPQueueID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPSemaphoreID {
    _unused: [u8; 0],
}
pub type MPSemaphoreID = *mut OpaqueMPSemaphoreID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPCriticalRegionID {
    _unused: [u8; 0],
}
pub type MPCriticalRegionID = *mut OpaqueMPCriticalRegionID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPTimerID {
    _unused: [u8; 0],
}
pub type MPTimerID = *mut OpaqueMPTimerID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPEventID {
    _unused: [u8; 0],
}
pub type MPEventID = *mut OpaqueMPEventID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPAddressSpaceID {
    _unused: [u8; 0],
}
pub type MPAddressSpaceID = *mut OpaqueMPAddressSpaceID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPNotificationID {
    _unused: [u8; 0],
}
pub type MPNotificationID = *mut OpaqueMPNotificationID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPCoherenceID {
    _unused: [u8; 0],
}
pub type MPCoherenceID = *mut OpaqueMPCoherenceID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPCpuID {
    _unused: [u8; 0],
}
pub type MPCpuID = *mut OpaqueMPCpuID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPAreaID {
    _unused: [u8; 0],
}
pub type MPAreaID = *mut OpaqueMPAreaID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPConsoleID {
    _unused: [u8; 0],
}
pub type MPConsoleID = *mut OpaqueMPConsoleID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMPOpaqueID {
    _unused: [u8; 0],
}
pub type MPOpaqueID = *mut OpaqueMPOpaqueID;
pub type MPOpaqueIDClass = UInt32;
pub type MPTaskOptions = OptionBits;
pub type TaskStorageIndex = ItemCount;
pub type TaskStorageValue = LogicalAddress;
pub type MPSemaphoreCount = ItemCount;
pub type MPTaskWeight = UInt32;
pub type MPEventFlags = UInt32;
pub type MPExceptionKind = UInt32;
pub type MPTaskStateKind = UInt32;
pub type MPPageSizeClass = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MPTaskInfoVersion2 {
    pub version: PBVersion,
    pub name: OSType,
    pub queueName: OSType,
    pub runState: UInt16,
    pub lastCPU: UInt16,
    pub weight: UInt32,
    pub processID: MPProcessID,
    pub cpuTime: AbsoluteTime,
    pub schedTime: AbsoluteTime,
    pub creationTime: AbsoluteTime,
    pub codePageFaults: ItemCount,
    pub dataPageFaults: ItemCount,
    pub preemptions: ItemCount,
    pub cpuID: MPCpuID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MPTaskInfo {
    pub version: PBVersion,
    pub name: OSType,
    pub queueName: OSType,
    pub runState: UInt16,
    pub lastCPU: UInt16,
    pub weight: UInt32,
    pub processID: MPProcessID,
    pub cpuTime: AbsoluteTime,
    pub schedTime: AbsoluteTime,
    pub creationTime: AbsoluteTime,
    pub codePageFaults: ItemCount,
    pub dataPageFaults: ItemCount,
    pub preemptions: ItemCount,
    pub cpuID: MPCpuID,
    pub blockedObject: MPOpaqueID,
    pub spaceID: MPAddressSpaceID,
    pub stackBase: LogicalAddress,
    pub stackLimit: LogicalAddress,
    pub stackCurr: LogicalAddress,
}
pub type MPDebuggerLevel = UInt32;
pub type MPRemoteContext = UInt8;
pub type FSAliasInfoBitmap = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AliasRecord {
    pub hidden: [UInt8; 6usize],
}
pub type AliasPtr = *mut AliasRecord;
pub type AliasHandle = *mut AliasPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSAliasInfo {
    pub volumeCreateDate: UTCDateTime,
    pub targetCreateDate: UTCDateTime,
    pub fileType: OSType,
    pub fileCreator: OSType,
    pub parentDirID: UInt32,
    pub nodeID: UInt32,
    pub filesystemID: UInt16,
    pub signature: UInt16,
    pub volumeIsBootVolume: Boolean,
    pub volumeIsAutomounted: Boolean,
    pub volumeIsEjectable: Boolean,
    pub volumeHasPersistentFileIDs: Boolean,
    pub isDirectory: Boolean,
}
pub type FSAliasInfoPtr = *mut FSAliasInfo;
pub type AliasInfoType = ::std::os::raw::c_short;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueLocaleRef {
    _unused: [u8; 0],
}
pub type LocaleRef = *mut OpaqueLocaleRef;
pub type LocalePartMask = UInt32;
pub type LocaleOperationClass = FourCharCode;
pub type LocaleOperationVariant = FourCharCode;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LocaleAndVariant {
    pub locale: LocaleRef,
    pub opVariant: LocaleOperationVariant,
}
pub type LocaleNameMask = UInt32;
pub type DebugComponentCallbackProcPtr = ::std::option::Option<
    unsafe extern "C" fn(optionSelectorNum: SInt32, command: UInt32, optionSetting: *mut Boolean),
>;
pub type DebugComponentCallbackUPP = DebugComponentCallbackProcPtr;
pub type DebugAssertOutputHandlerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        componentSignature: OSType,
        options: UInt32,
        assertionString: *const ::std::os::raw::c_char,
        exceptionLabelString: *const ::std::os::raw::c_char,
        errorString: *const ::std::os::raw::c_char,
        fileName: *const ::std::os::raw::c_char,
        lineNumber: ::std::os::raw::c_long,
        value: *mut ::std::os::raw::c_void,
        outputMsg: ConstStr255Param,
    ),
>;
pub type DebugAssertOutputHandlerUPP = DebugAssertOutputHandlerProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAreaID {
    _unused: [u8; 0],
}
pub type AreaID = *mut OpaqueAreaID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MachineInformationPowerPC {
    pub CTR: UnsignedWide,
    pub LR: UnsignedWide,
    pub PC: UnsignedWide,
    pub CRRegister: ::std::os::raw::c_ulong,
    pub XER: ::std::os::raw::c_ulong,
    pub MSR: ::std::os::raw::c_ulong,
    pub MQ: ::std::os::raw::c_ulong,
    pub ExceptKind: ::std::os::raw::c_ulong,
    pub DSISR: ::std::os::raw::c_ulong,
    pub DAR: UnsignedWide,
    pub Reserved: UnsignedWide,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RegisterInformationPowerPC {
    pub R0: UnsignedWide,
    pub R1: UnsignedWide,
    pub R2: UnsignedWide,
    pub R3: UnsignedWide,
    pub R4: UnsignedWide,
    pub R5: UnsignedWide,
    pub R6: UnsignedWide,
    pub R7: UnsignedWide,
    pub R8: UnsignedWide,
    pub R9: UnsignedWide,
    pub R10: UnsignedWide,
    pub R11: UnsignedWide,
    pub R12: UnsignedWide,
    pub R13: UnsignedWide,
    pub R14: UnsignedWide,
    pub R15: UnsignedWide,
    pub R16: UnsignedWide,
    pub R17: UnsignedWide,
    pub R18: UnsignedWide,
    pub R19: UnsignedWide,
    pub R20: UnsignedWide,
    pub R21: UnsignedWide,
    pub R22: UnsignedWide,
    pub R23: UnsignedWide,
    pub R24: UnsignedWide,
    pub R25: UnsignedWide,
    pub R26: UnsignedWide,
    pub R27: UnsignedWide,
    pub R28: UnsignedWide,
    pub R29: UnsignedWide,
    pub R30: UnsignedWide,
    pub R31: UnsignedWide,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPUInformationPowerPC {
    pub Registers: [UnsignedWide; 32usize],
    pub FPSCR: ::std::os::raw::c_ulong,
    pub Reserved: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Vector128 {
    pub l: [::std::os::raw::c_ulong; 4usize],
    pub s: [::std::os::raw::c_ushort; 8usize],
    pub c: [::std::os::raw::c_uchar; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VectorInformationPowerPC {
    pub Registers: [Vector128; 32usize],
    pub VSCR: Vector128,
    pub VRsave: UInt32,
}
pub type MemoryReferenceKind = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryExceptionInformation {
    pub theArea: AreaID,
    pub theAddress: LogicalAddress,
    pub theError: OSStatus,
    pub theReference: MemoryReferenceKind,
}
pub type ExceptionKind = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ExceptionInfo {
    pub memoryInfo: *mut MemoryExceptionInformation,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExceptionInformationPowerPC {
    pub theKind: ExceptionKind,
    pub machineState: *mut MachineInformationPowerPC,
    pub registerImage: *mut RegisterInformationPowerPC,
    pub FPUImage: *mut FPUInformationPowerPC,
    pub info: ExceptionInfo,
    pub vectorImage: *mut VectorInformationPowerPC,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MachineInformation {
    pub __unusedMachineInformationField: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RegisterInformation {
    pub __unusedRegisterInformationField: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FPUInformation {
    pub __unusedFPUInformationField: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VectorInformation {
    pub __unusedVectorInformationField: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExceptionInformation {
    pub theKind: ExceptionKind,
    pub machineState: *mut MachineInformation,
    pub registerImage: *mut RegisterInformation,
    pub FPUImage: *mut FPUInformation,
    pub info: ExceptionInfo,
    pub vectorImage: *mut VectorInformation,
}
pub type ExceptionHandlerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(theException: *mut ExceptionInformation) -> OSStatus,
>;
pub type ExceptionHandlerUPP = ExceptionHandlerProcPtr;
pub type ExceptionHandlerTPP = ExceptionHandlerUPP;
pub type ExceptionHandler = ExceptionHandlerTPP;
pub type Nanoseconds = UnsignedWide;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NumFormatString {
    pub fLength: UInt8,
    pub fVersion: UInt8,
    pub data: [::std::os::raw::c_char; 254usize],
}
pub type NumFormatStringRec = NumFormatString;
pub type FormatStatus = ::std::os::raw::c_short;
pub type FormatClass = SInt8;
pub type FormatResultType = SInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FVector {
    pub start: ::std::os::raw::c_short,
    pub length: ::std::os::raw::c_short,
}
pub type UCKeyOutput = UInt16;
pub type UCKeyCharSeq = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyStateRecord {
    pub stateZeroCharData: UCKeyCharSeq,
    pub stateZeroNextState: UInt16,
    pub stateEntryCount: UInt16,
    pub stateEntryFormat: UInt16,
    pub stateEntryData: [UInt32; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyStateEntryTerminal {
    pub curState: UInt16,
    pub charData: UCKeyCharSeq,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyStateEntryRange {
    pub curStateStart: UInt16,
    pub curStateRange: UInt8,
    pub deltaMultiplier: UInt8,
    pub charData: UCKeyCharSeq,
    pub nextState: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyboardTypeHeader {
    pub keyboardTypeFirst: UInt32,
    pub keyboardTypeLast: UInt32,
    pub keyModifiersToTableNumOffset: UInt32,
    pub keyToCharTableIndexOffset: UInt32,
    pub keyStateRecordsIndexOffset: UInt32,
    pub keyStateTerminatorsOffset: UInt32,
    pub keySequenceDataIndexOffset: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyboardLayout {
    pub keyLayoutHeaderFormat: UInt16,
    pub keyLayoutDataVersion: UInt16,
    pub keyLayoutFeatureInfoOffset: UInt32,
    pub keyboardTypeCount: UInt32,
    pub keyboardTypeList: [UCKeyboardTypeHeader; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyLayoutFeatureInfo {
    pub keyLayoutFeatureInfoFormat: UInt16,
    pub reserved: UInt16,
    pub maxOutputStringLength: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyModifiersToTableNum {
    pub keyModifiersToTableNumFormat: UInt16,
    pub defaultTableNum: UInt16,
    pub modifiersCount: UInt32,
    pub tableNum: [UInt8; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyToCharTableIndex {
    pub keyToCharTableIndexFormat: UInt16,
    pub keyToCharTableSize: UInt16,
    pub keyToCharTableCount: UInt32,
    pub keyToCharTableOffsets: [UInt32; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyStateRecordsIndex {
    pub keyStateRecordsIndexFormat: UInt16,
    pub keyStateRecordCount: UInt16,
    pub keyStateRecordOffsets: [UInt32; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UCKeyStateTerminators {
    pub keyStateTerminatorsFormat: UInt16,
    pub keyStateTerminatorCount: UInt16,
    pub keyStateTerminators: [UCKeyCharSeq; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UCKeySequenceDataIndex {
    pub keySequenceDataIndexFormat: UInt16,
    pub charSequenceCount: UInt16,
    pub charSequenceOffsets: [UInt16; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCollatorRef {
    _unused: [u8; 0],
}
pub type CollatorRef = *mut OpaqueCollatorRef;
pub type UCCollateOptions = UInt32;
pub type UCCollationValue = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueUCTypeSelectRef {
    _unused: [u8; 0],
}
pub type UCTypeSelectRef = *mut OpaqueUCTypeSelectRef;
pub type UCTypeSelectCompareResult = SInt32;
pub type UCTSWalkDirection = UInt16;
pub type UCTypeSelectOptions = UInt16;
pub type IndexToUCStringProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        index: UInt32,
        listDataPtr: *mut ::std::os::raw::c_void,
        refcon: *mut ::std::os::raw::c_void,
        outString: *mut CFStringRef,
        tsOptions: *mut UCTypeSelectOptions,
    ) -> Boolean,
>;
pub type IndexToUCStringUPP = IndexToUCStringProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueTextBreakLocatorRef {
    _unused: [u8; 0],
}
pub type TextBreakLocatorRef = *mut OpaqueTextBreakLocatorRef;
pub type UCTextBreakType = UInt32;
pub type UCTextBreakOptions = UInt32;
pub type relop = ::std::os::raw::c_short;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct decimal {
    pub __bindgen_anon_1: decimal__bindgen_ty_1,
    pub sgn: ::std::os::raw::c_char,
    pub unused: ::std::os::raw::c_char,
    pub exp: ::std::os::raw::c_short,
    pub sig: decimal__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct decimal__bindgen_ty_1 {
    pub length: ::std::os::raw::c_uchar,
    pub text: [::std::os::raw::c_uchar; 36usize],
    pub unused: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct decform {
    pub style: ::std::os::raw::c_char,
    pub unused: ::std::os::raw::c_char,
    pub digits: ::std::os::raw::c_short,
}
pub type TECPluginSignature = OSType;
pub type TECPluginVersion = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueTECObjectRef {
    _unused: [u8; 0],
}
pub type TECObjectRef = *mut OpaqueTECObjectRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueTECSnifferObjectRef {
    _unused: [u8; 0],
}
pub type TECSnifferObjectRef = *mut OpaqueTECSnifferObjectRef;
pub type TECPluginSig = OSType;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TECConversionInfo {
    pub sourceEncoding: TextEncoding,
    pub destinationEncoding: TextEncoding,
    pub reserved1: UInt16,
    pub reserved2: UInt16,
}
pub type TECInternetNameUsageMask = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueTextToUnicodeInfo {
    _unused: [u8; 0],
}
pub type TextToUnicodeInfo = *mut OpaqueTextToUnicodeInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueUnicodeToTextInfo {
    _unused: [u8; 0],
}
pub type UnicodeToTextInfo = *mut OpaqueUnicodeToTextInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueUnicodeToTextRunInfo {
    _unused: [u8; 0],
}
pub type UnicodeToTextRunInfo = *mut OpaqueUnicodeToTextRunInfo;
pub type ConstTextToUnicodeInfo = TextToUnicodeInfo;
pub type ConstUnicodeToTextInfo = UnicodeToTextInfo;
pub type UnicodeMapVersion = SInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct UnicodeMapping {
    pub unicodeEncoding: TextEncoding,
    pub otherEncoding: TextEncoding,
    pub mappingVersion: UnicodeMapVersion,
}
pub type UnicodeMappingPtr = *mut UnicodeMapping;
pub type ConstUnicodeMappingPtr = *const UnicodeMapping;
pub type UnicodeToTextFallbackProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        iSrcUniStr: *mut UniChar,
        iSrcUniStrLen: ByteCount,
        oSrcConvLen: *mut ByteCount,
        oDestStr: TextPtr,
        iDestStrLen: ByteCount,
        oDestConvLen: *mut ByteCount,
        iInfoPtr: LogicalAddress,
        iUnicodeMappingPtr: ConstUnicodeMappingPtr,
    ) -> OSStatus,
>;
pub type UnicodeToTextFallbackUPP = UnicodeToTextFallbackProcPtr;
pub type ThreadState = UInt16;
pub type ThreadTaskRef = *mut ::std::os::raw::c_void;
pub type ThreadStyle = UInt32;
pub type ThreadID = ::std::os::raw::c_ulong;
pub type ThreadOptions = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SchedulerInfoRec {
    pub InfoRecSize: UInt32,
    pub CurrentThreadID: ThreadID,
    pub SuggestedThreadID: ThreadID,
    pub InterruptedCoopThreadID: ThreadID,
}
pub type SchedulerInfoRecPtr = *mut SchedulerInfoRec;
pub type voidPtr = *mut ::std::os::raw::c_void;
pub type ThreadEntryProcPtr = ::std::option::Option<
    unsafe extern "C" fn(threadParam: *mut ::std::os::raw::c_void) -> voidPtr,
>;
pub type ThreadSchedulerProcPtr =
    ::std::option::Option<unsafe extern "C" fn(schedulerInfo: SchedulerInfoRecPtr) -> ThreadID>;
pub type ThreadSwitchProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        threadBeingSwitched: ThreadID,
        switchProcParam: *mut ::std::os::raw::c_void,
    ),
>;
pub type ThreadTerminationProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        threadTerminated: ThreadID,
        terminationProcParam: *mut ::std::os::raw::c_void,
    ),
>;
pub type DebuggerNewThreadProcPtr =
    ::std::option::Option<unsafe extern "C" fn(threadCreated: ThreadID)>;
pub type DebuggerDisposeThreadProcPtr =
    ::std::option::Option<unsafe extern "C" fn(threadDeleted: ThreadID)>;
pub type DebuggerThreadSchedulerProcPtr =
    ::std::option::Option<unsafe extern "C" fn(schedulerInfo: SchedulerInfoRecPtr) -> ThreadID>;
pub type ThreadEntryUPP = ThreadEntryProcPtr;
pub type ThreadSchedulerUPP = ThreadSchedulerProcPtr;
pub type ThreadSwitchUPP = ThreadSwitchProcPtr;
pub type ThreadTerminationUPP = ThreadTerminationProcPtr;
pub type DebuggerNewThreadUPP = DebuggerNewThreadProcPtr;
pub type DebuggerDisposeThreadUPP = DebuggerDisposeThreadProcPtr;
pub type DebuggerThreadSchedulerUPP = DebuggerThreadSchedulerProcPtr;
pub type ThreadEntryTPP = ThreadEntryUPP;
pub type ThreadSchedulerTPP = ThreadSchedulerUPP;
pub type ThreadSwitchTPP = ThreadSwitchUPP;
pub type ThreadTerminationTPP = ThreadTerminationUPP;
pub type DebuggerNewThreadTPP = DebuggerNewThreadUPP;
pub type DebuggerDisposeThreadTPP = DebuggerDisposeThreadUPP;
pub type DebuggerThreadSchedulerTPP = DebuggerThreadSchedulerUPP;
pub type FolderDescFlags = UInt32;
pub type FolderClass = OSType;
pub type FolderType = OSType;
pub type FolderLocation = OSType;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FolderDesc {
    pub descSize: Size,
    pub foldType: FolderType,
    pub flags: FolderDescFlags,
    pub foldClass: FolderClass,
    pub foldLocation: FolderType,
    pub badgeSignature: OSType,
    pub badgeType: OSType,
    pub reserved: UInt32,
    pub name: StrFileName,
}
pub type FolderDescPtr = *mut FolderDesc;
pub type RoutingFlags = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FolderRouting {
    pub descSize: Size,
    pub fileType: OSType,
    pub routeFromFolder: FolderType,
    pub routeToFolder: FolderType,
    pub flags: RoutingFlags,
}
pub type FolderRoutingPtr = *mut FolderRouting;
pub type FolderManagerNotificationProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        message: OSType,
        arg: *mut ::std::os::raw::c_void,
        userRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type FolderManagerNotificationUPP = FolderManagerNotificationProcPtr;
pub type TMTaskPtr = *mut TMTask;
pub type TimerProcPtr = ::std::option::Option<unsafe extern "C" fn(tmTaskPtr: TMTaskPtr)>;
pub type TimerUPP = TimerProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TMTask {
    pub qLink: QElemPtr,
    pub qType: ::std::os::raw::c_short,
    pub tmAddr: TimerUPP,
    pub tmCount: ::std::os::raw::c_long,
    pub tmWakeUp: ::std::os::raw::c_long,
    pub tmReserved: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MPQueueInfo {
    pub version: PBVersion,
    pub processID: MPProcessID,
    pub queueName: OSType,
    pub nWaiting: ItemCount,
    pub waitingTaskID: MPTaskID,
    pub nMessages: ItemCount,
    pub nReserved: ItemCount,
    pub p1: *mut ::std::os::raw::c_void,
    pub p2: *mut ::std::os::raw::c_void,
    pub p3: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MPSemaphoreInfo {
    pub version: PBVersion,
    pub processID: MPProcessID,
    pub semaphoreName: OSType,
    pub nWaiting: ItemCount,
    pub waitingTaskID: MPTaskID,
    pub maximum: ItemCount,
    pub count: ItemCount,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MPEventInfo {
    pub version: PBVersion,
    pub processID: MPProcessID,
    pub eventName: OSType,
    pub nWaiting: ItemCount,
    pub waitingTaskID: MPTaskID,
    pub events: MPEventFlags,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MPCriticalRegionInfo {
    pub version: PBVersion,
    pub processID: MPProcessID,
    pub regionName: OSType,
    pub nWaiting: ItemCount,
    pub waitingTaskID: MPTaskID,
    pub owningTask: MPTaskID,
    pub count: ItemCount,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MPNotificationInfo {
    pub version: PBVersion,
    pub processID: MPProcessID,
    pub notificationName: OSType,
    pub queueID: MPQueueID,
    pub p1: *mut ::std::os::raw::c_void,
    pub p2: *mut ::std::os::raw::c_void,
    pub p3: *mut ::std::os::raw::c_void,
    pub eventID: MPEventID,
    pub events: MPEventFlags,
    pub semaphoreID: MPSemaphoreID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MPAddressSpaceInfo {
    pub version: PBVersion,
    pub processID: MPProcessID,
    pub groupID: MPCoherenceID,
    pub nTasks: ItemCount,
    pub vsid: [UInt32; 16usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PEFContainerHeader {
    pub tag1: OSType,
    pub tag2: OSType,
    pub architecture: OSType,
    pub formatVersion: UInt32,
    pub dateTimeStamp: UInt32,
    pub oldDefVersion: UInt32,
    pub oldImpVersion: UInt32,
    pub currentVersion: UInt32,
    pub sectionCount: UInt16,
    pub instSectionCount: UInt16,
    pub reservedA: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PEFSectionHeader {
    pub nameOffset: SInt32,
    pub defaultAddress: UInt32,
    pub totalLength: UInt32,
    pub unpackedLength: UInt32,
    pub containerLength: UInt32,
    pub containerOffset: UInt32,
    pub sectionKind: UInt8,
    pub shareKind: UInt8,
    pub alignment: UInt8,
    pub reservedA: UInt8,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PEFLoaderInfoHeader {
    pub mainSection: SInt32,
    pub mainOffset: UInt32,
    pub initSection: SInt32,
    pub initOffset: UInt32,
    pub termSection: SInt32,
    pub termOffset: UInt32,
    pub importedLibraryCount: UInt32,
    pub totalImportedSymbolCount: UInt32,
    pub relocSectionCount: UInt32,
    pub relocInstrOffset: UInt32,
    pub loaderStringsOffset: UInt32,
    pub exportHashOffset: UInt32,
    pub exportHashTablePower: UInt32,
    pub exportedSymbolCount: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PEFImportedLibrary {
    pub nameOffset: UInt32,
    pub oldImpVersion: UInt32,
    pub currentVersion: UInt32,
    pub importedSymbolCount: UInt32,
    pub firstImportedSymbol: UInt32,
    pub options: UInt8,
    pub reservedA: UInt8,
    pub reservedB: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PEFImportedSymbol {
    pub classAndName: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PEFExportedSymbolHashSlot {
    pub countAndStart: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PEFSplitHashWord {
    pub nameLength: UInt16,
    pub hashValue: UInt16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFExportedSymbolKey {
    pub u: PEFExportedSymbolKey__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union PEFExportedSymbolKey__bindgen_ty_1 {
    pub fullHashWord: UInt32,
    pub splitHashWord: PEFSplitHashWord,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PEFExportedSymbol {
    pub classAndName: UInt32,
    pub symbolValue: UInt32,
    pub sectionIndex: SInt16,
}
pub type PEFRelocChunk = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PEFLoaderRelocationHeader {
    pub sectionIndex: UInt16,
    pub reservedA: UInt16,
    pub relocCount: UInt32,
    pub firstRelocOffset: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct XLibContainerHeader {
    pub tag1: OSType,
    pub tag2: OSType,
    pub currentFormat: UInt32,
    pub containerStringsOffset: UInt32,
    pub exportHashOffset: UInt32,
    pub exportKeyOffset: UInt32,
    pub exportSymbolOffset: UInt32,
    pub exportNamesOffset: UInt32,
    pub exportHashTablePower: UInt32,
    pub exportedSymbolCount: UInt32,
    pub fragNameOffset: UInt32,
    pub fragNameLength: UInt32,
    pub dylibPathOffset: UInt32,
    pub dylibPathLength: UInt32,
    pub cpuFamily: OSType,
    pub cpuModel: OSType,
    pub dateTimeStamp: UInt32,
    pub currentVersion: UInt32,
    pub oldDefVersion: UInt32,
    pub oldImpVersion: UInt32,
}
pub type XLibExportedSymbolHashSlot = PEFExportedSymbolHashSlot;
pub type XLibExportedSymbolKey = PEFExportedSymbolKey;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct XLibExportedSymbol {
    pub classAndName: UInt32,
    pub bpOffset: UInt32,
}
pub type HFSCatalogNodeID = UInt32;
pub type MarkerIdType = SInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ChunkHeader {
    pub ckID: UInt32,
    pub ckSize: SInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ContainerChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub formType: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FormatVersionChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub timestamp: UInt32,
}
pub type FormatVersionChunkPtr = *mut FormatVersionChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CommonChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub numChannels: SInt16,
    pub numSampleFrames: UInt32,
    pub sampleSize: SInt16,
    pub sampleRate: extended80,
}
pub type CommonChunkPtr = *mut CommonChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ExtCommonChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub numChannels: SInt16,
    pub numSampleFrames: UInt32,
    pub sampleSize: SInt16,
    pub sampleRate: extended80,
    pub compressionType: UInt32,
    pub compressionName: [::std::os::raw::c_char; 1usize],
}
pub type ExtCommonChunkPtr = *mut ExtCommonChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SoundDataChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub offset: UInt32,
    pub blockSize: UInt32,
}
pub type SoundDataChunkPtr = *mut SoundDataChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct Marker {
    pub id: MarkerIdType,
    pub position: UInt32,
    pub markerName: Str255,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MarkerChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub numMarkers: UInt16,
    pub Markers: [Marker; 1usize],
}
pub type MarkerChunkPtr = *mut MarkerChunk;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AIFFLoop {
    pub playMode: SInt16,
    pub beginLoop: MarkerIdType,
    pub endLoop: MarkerIdType,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct InstrumentChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub baseFrequency: UInt8,
    pub detune: UInt8,
    pub lowFrequency: UInt8,
    pub highFrequency: UInt8,
    pub lowVelocity: UInt8,
    pub highVelocity: UInt8,
    pub gain: SInt16,
    pub sustainLoop: AIFFLoop,
    pub releaseLoop: AIFFLoop,
}
pub type InstrumentChunkPtr = *mut InstrumentChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MIDIDataChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub MIDIdata: [UInt8; 1usize],
}
pub type MIDIDataChunkPtr = *mut MIDIDataChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AudioRecordingChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub AESChannelStatus: [UInt8; 24usize],
}
pub type AudioRecordingChunkPtr = *mut AudioRecordingChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ApplicationSpecificChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub applicationSignature: OSType,
    pub data: [UInt8; 1usize],
}
pub type ApplicationSpecificChunkPtr = *mut ApplicationSpecificChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct Comment {
    pub timeStamp: UInt32,
    pub marker: MarkerIdType,
    pub count: UInt16,
    pub text: [::std::os::raw::c_char; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CommentsChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub numComments: UInt16,
    pub comments: [Comment; 1usize],
}
pub type CommentsChunkPtr = *mut CommentsChunk;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TextChunk {
    pub ckID: UInt32,
    pub ckSize: SInt32,
    pub text: [::std::os::raw::c_char; 1usize],
}
pub type TextChunkPtr = *mut TextChunk;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TextEncodingRec {
    pub base: UInt32,
    pub variant: UInt32,
    pub format: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECEncodingsListRec {
    pub count: UInt32,
    pub encodings: TextEncodingRec,
}
pub type TECEncodingsListPtr = *mut TECEncodingsListRec;
pub type TECEncodingsListHandle = *mut TECEncodingsListPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECSubTextEncodingRec {
    pub offset: UInt32,
    pub searchEncoding: TextEncodingRec,
    pub count: UInt32,
    pub subEncodings: TextEncodingRec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECSubTextEncodingsRec {
    pub count: UInt32,
    pub subTextEncodingRec: TECSubTextEncodingRec,
}
pub type TECSubTextEncodingsPtr = *mut TECSubTextEncodingsRec;
pub type TECSubTextEncodingsHandle = *mut TECSubTextEncodingsPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECEncodingPairRec {
    pub source: TextEncodingRec,
    pub dest: TextEncodingRec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECEncodingPairs {
    pub encodingPair: TECEncodingPairRec,
    pub flags: UInt32,
    pub speed: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECEncodingPairsRec {
    pub count: UInt32,
    pub encodingPairs: TECEncodingPairs,
}
pub type TECEncodingPairsPtr = *mut TECEncodingPairsRec;
pub type TECEncodingPairsHandle = *mut TECEncodingPairsPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECLocaleListToEncodingListRec {
    pub offset: UInt32,
    pub count: UInt32,
    pub locales: RegionCode,
}
pub type TECLocaleListToEncodingListPtr = *mut TECLocaleListToEncodingListRec;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECLocaleToEncodingsListRec {
    pub count: UInt32,
    pub localeListToEncodingList: TECLocaleListToEncodingListRec,
}
pub type TECLocaleToEncodingsListPtr = *mut TECLocaleToEncodingsListRec;
pub type TECLocaleToEncodingsListHandle = *mut TECLocaleToEncodingsListPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECInternetNameRec {
    pub offset: UInt32,
    pub searchEncoding: TextEncodingRec,
    pub encodingNameLength: UInt8,
    pub encodingName: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECInternetNamesRec {
    pub count: UInt32,
    pub InternetNames: TECInternetNameRec,
}
pub type TECInternetNamesPtr = *mut TECInternetNamesRec;
pub type TECInternetNamesHandle = *mut TECInternetNamesPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECBufferContextRec {
    pub textInputBuffer: ConstTextPtr,
    pub textInputBufferEnd: ConstTextPtr,
    pub textOutputBuffer: TextPtr,
    pub textOutputBufferEnd: TextPtr,
    pub encodingInputBuffer: ConstTextEncodingRunPtr,
    pub encodingInputBufferEnd: ConstTextEncodingRunPtr,
    pub encodingOutputBuffer: TextEncodingRunPtr,
    pub encodingOutputBufferEnd: TextEncodingRunPtr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECPluginStateRec {
    pub state1: UInt8,
    pub state2: UInt8,
    pub state3: UInt8,
    pub state4: UInt8,
    pub longState1: UInt32,
    pub longState2: UInt32,
    pub longState3: UInt32,
    pub longState4: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECConverterContextRec {
    pub pluginRec: Ptr,
    pub sourceEncoding: TextEncoding,
    pub destEncoding: TextEncoding,
    pub reserved1: UInt32,
    pub reserved2: UInt32,
    pub bufferContext: TECBufferContextRec,
    pub contextRefCon: URefCon,
    pub conversionProc: ProcPtr,
    pub flushProc: ProcPtr,
    pub clearContextInfoProc: ProcPtr,
    pub options1: UInt32,
    pub options2: UInt32,
    pub pluginState: TECPluginStateRec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECSnifferContextRec {
    pub pluginRec: Ptr,
    pub encoding: TextEncoding,
    pub maxErrors: ItemCount,
    pub maxFeatures: ItemCount,
    pub textInputBuffer: ConstTextPtr,
    pub textInputBufferEnd: ConstTextPtr,
    pub numFeatures: ItemCount,
    pub numErrors: ItemCount,
    pub contextRefCon: URefCon,
    pub sniffProc: ProcPtr,
    pub clearContextInfoProc: ProcPtr,
    pub pluginState: TECPluginStateRec,
}
pub type TECPluginNewEncodingConverterPtr = ::std::option::Option<
    unsafe extern "C" fn(
        newEncodingConverter: *mut TECObjectRef,
        plugContext: *mut TECConverterContextRec,
        inputEncoding: TextEncoding,
        outputEncoding: TextEncoding,
    ) -> OSStatus,
>;
pub type TECPluginClearContextInfoPtr = ::std::option::Option<
    unsafe extern "C" fn(
        encodingConverter: TECObjectRef,
        plugContext: *mut TECConverterContextRec,
    ) -> OSStatus,
>;
pub type TECPluginConvertTextEncodingPtr = ::std::option::Option<
    unsafe extern "C" fn(
        encodingConverter: TECObjectRef,
        plugContext: *mut TECConverterContextRec,
    ) -> OSStatus,
>;
pub type TECPluginFlushConversionPtr = ::std::option::Option<
    unsafe extern "C" fn(
        encodingConverter: TECObjectRef,
        plugContext: *mut TECConverterContextRec,
    ) -> OSStatus,
>;
pub type TECPluginDisposeEncodingConverterPtr = ::std::option::Option<
    unsafe extern "C" fn(
        newEncodingConverter: TECObjectRef,
        plugContext: *mut TECConverterContextRec,
    ) -> OSStatus,
>;
pub type TECPluginNewEncodingSnifferPtr = ::std::option::Option<
    unsafe extern "C" fn(
        encodingSniffer: *mut TECSnifferObjectRef,
        snifContext: *mut TECSnifferContextRec,
        inputEncoding: TextEncoding,
    ) -> OSStatus,
>;
pub type TECPluginClearSnifferContextInfoPtr = ::std::option::Option<
    unsafe extern "C" fn(
        encodingSniffer: TECSnifferObjectRef,
        snifContext: *mut TECSnifferContextRec,
    ) -> OSStatus,
>;
pub type TECPluginSniffTextEncodingPtr = ::std::option::Option<
    unsafe extern "C" fn(
        encodingSniffer: TECSnifferObjectRef,
        snifContext: *mut TECSnifferContextRec,
    ) -> OSStatus,
>;
pub type TECPluginDisposeEncodingSnifferPtr = ::std::option::Option<
    unsafe extern "C" fn(
        encodingSniffer: TECSnifferObjectRef,
        snifContext: *mut TECSnifferContextRec,
    ) -> OSStatus,
>;
pub type TECPluginGetCountAvailableTextEncodingsPtr = ::std::option::Option<
    unsafe extern "C" fn(
        availableEncodings: *mut TextEncoding,
        maxAvailableEncodings: ItemCount,
        actualAvailableEncodings: *mut ItemCount,
    ) -> OSStatus,
>;
pub type TECPluginGetCountAvailableTextEncodingPairsPtr = ::std::option::Option<
    unsafe extern "C" fn(
        availableEncodings: *mut TECConversionInfo,
        maxAvailableEncodings: ItemCount,
        actualAvailableEncodings: *mut ItemCount,
    ) -> OSStatus,
>;
pub type TECPluginGetCountDestinationTextEncodingsPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inputEncoding: TextEncoding,
        destinationEncodings: *mut TextEncoding,
        maxDestinationEncodings: ItemCount,
        actualDestinationEncodings: *mut ItemCount,
    ) -> OSStatus,
>;
pub type TECPluginGetCountSubTextEncodingsPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inputEncoding: TextEncoding,
        subEncodings: *mut TextEncoding,
        maxSubEncodings: ItemCount,
        actualSubEncodings: *mut ItemCount,
    ) -> OSStatus,
>;
pub type TECPluginGetCountAvailableSniffersPtr = ::std::option::Option<
    unsafe extern "C" fn(
        availableEncodings: *mut TextEncoding,
        maxAvailableEncodings: ItemCount,
        actualAvailableEncodings: *mut ItemCount,
    ) -> OSStatus,
>;
pub type TECPluginGetTextEncodingInternetNamePtr = ::std::option::Option<
    unsafe extern "C" fn(
        textEncoding: TextEncoding,
        encodingName: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus,
>;
pub type TECPluginGetTextEncodingFromInternetNamePtr = ::std::option::Option<
    unsafe extern "C" fn(
        textEncoding: *mut TextEncoding,
        encodingName: ConstStr255Param,
    ) -> OSStatus,
>;
pub type TECPluginGetCountWebEncodingsPtr = ::std::option::Option<
    unsafe extern "C" fn(
        availableEncodings: *mut TextEncoding,
        maxAvailableEncodings: ItemCount,
        actualAvailableEncodings: *mut ItemCount,
    ) -> OSStatus,
>;
pub type TECPluginGetCountMailEncodingsPtr = ::std::option::Option<
    unsafe extern "C" fn(
        availableEncodings: *mut TextEncoding,
        maxAvailableEncodings: ItemCount,
        actualAvailableEncodings: *mut ItemCount,
    ) -> OSStatus,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TECPluginDispatchTable {
    pub version: TECPluginVersion,
    pub compatibleVersion: TECPluginVersion,
    pub PluginID: TECPluginSignature,
    pub PluginNewEncodingConverter: TECPluginNewEncodingConverterPtr,
    pub PluginClearContextInfo: TECPluginClearContextInfoPtr,
    pub PluginConvertTextEncoding: TECPluginConvertTextEncodingPtr,
    pub PluginFlushConversion: TECPluginFlushConversionPtr,
    pub PluginDisposeEncodingConverter: TECPluginDisposeEncodingConverterPtr,
    pub PluginNewEncodingSniffer: TECPluginNewEncodingSnifferPtr,
    pub PluginClearSnifferContextInfo: TECPluginClearSnifferContextInfoPtr,
    pub PluginSniffTextEncoding: TECPluginSniffTextEncodingPtr,
    pub PluginDisposeEncodingSniffer: TECPluginDisposeEncodingSnifferPtr,
    pub PluginGetCountAvailableTextEncodings: TECPluginGetCountAvailableTextEncodingsPtr,
    pub PluginGetCountAvailableTextEncodingPairs: TECPluginGetCountAvailableTextEncodingPairsPtr,
    pub PluginGetCountDestinationTextEncodings: TECPluginGetCountDestinationTextEncodingsPtr,
    pub PluginGetCountSubTextEncodings: TECPluginGetCountSubTextEncodingsPtr,
    pub PluginGetCountAvailableSniffers: TECPluginGetCountAvailableSniffersPtr,
    pub PluginGetCountWebTextEncodings: TECPluginGetCountWebEncodingsPtr,
    pub PluginGetCountMailTextEncodings: TECPluginGetCountMailEncodingsPtr,
    pub PluginGetTextEncodingInternetName: TECPluginGetTextEncodingInternetNamePtr,
    pub PluginGetTextEncodingFromInternetName: TECPluginGetTextEncodingFromInternetNamePtr,
}
pub type DescType = ResType;
pub type AEKeyword = FourCharCode;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAEDataStorageType {
    _unused: [u8; 0],
}
pub type AEDataStorageType = *mut OpaqueAEDataStorageType;
pub type AEDataStorage = *mut AEDataStorageType;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AEDesc {
    pub descriptorType: DescType,
    pub dataHandle: AEDataStorage,
}
pub type AEDescPtr = *mut AEDesc;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AEKeyDesc {
    pub descKey: AEKeyword,
    pub descContent: AEDesc,
}
pub type AEDescList = AEDesc;
pub type AERecord = AEDescList;
pub type AEAddressDesc = AEDesc;
pub type AppleEvent = AERecord;
pub type AppleEventPtr = *mut AppleEvent;
pub type AEReturnID = SInt16;
pub type AETransactionID = SInt32;
pub type AEEventClass = FourCharCode;
pub type AEEventID = FourCharCode;
pub type AEArrayType = SInt8;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union AEArrayData {
    pub kAEDataArray: [SInt16; 1usize],
    pub kAEPackedArray: [::std::os::raw::c_char; 1usize],
    pub kAEHandleArray: [Handle; 1usize],
    pub kAEDescArray: [AEDesc; 1usize],
    pub kAEKeyDescArray: [AEKeyDesc; 1usize],
}
pub type AEArrayDataPointer = *mut AEArrayData;
pub type AESendPriority = SInt16;
pub type AESendMode = SInt32;
pub type AECoerceDescProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        fromDesc: *const AEDesc,
        toType: DescType,
        handlerRefcon: SRefCon,
        toDesc: *mut AEDesc,
    ) -> OSErr,
>;
pub type AECoercePtrProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        typeCode: DescType,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: Size,
        toType: DescType,
        handlerRefcon: SRefCon,
        result: *mut AEDesc,
    ) -> OSErr,
>;
pub type AECoerceDescUPP = AECoerceDescProcPtr;
pub type AECoercePtrUPP = AECoercePtrProcPtr;
pub type AECoercionHandlerUPP = AECoerceDescUPP;
pub type AEDisposeExternalProcPtr = ::std::option::Option<
    unsafe extern "C" fn(dataPtr: *const ::std::os::raw::c_void, dataLength: Size, refcon: SRefCon),
>;
pub type AEDisposeExternalUPP = AEDisposeExternalProcPtr;
pub type AEEventHandlerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theAppleEvent: *const AppleEvent,
        reply: *mut AppleEvent,
        handlerRefcon: SRefCon,
    ) -> OSErr,
>;
pub type AEEventHandlerUPP = AEEventHandlerProcPtr;
pub type AEEventSource = SInt8;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AERemoteProcessResolverContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AERemoteProcessResolver {
    _unused: [u8; 0],
}
pub type AERemoteProcessResolverRef = *mut AERemoteProcessResolver;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ccntTokenRecord {
    pub tokenClass: DescType,
    pub token: AEDesc,
}
pub type ccntTokenRecPtr = *mut ccntTokenRecord;
pub type ccntTokenRecHandle = *mut ccntTokenRecPtr;
pub type OSLAccessorProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        desiredClass: DescType,
        container: *const AEDesc,
        containerClass: DescType,
        form: DescType,
        selectionData: *const AEDesc,
        value: *mut AEDesc,
        accessorRefcon: SRefCon,
    ) -> OSErr,
>;
pub type OSLCompareProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        oper: DescType,
        obj1: *const AEDesc,
        obj2: *const AEDesc,
        result: *mut Boolean,
    ) -> OSErr,
>;
pub type OSLCountProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        desiredType: DescType,
        containerClass: DescType,
        container: *const AEDesc,
        result: *mut ::std::os::raw::c_long,
    ) -> OSErr,
>;
pub type OSLDisposeTokenProcPtr =
    ::std::option::Option<unsafe extern "C" fn(unneededToken: *mut AEDesc) -> OSErr>;
pub type OSLGetMarkTokenProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        dContainerToken: *const AEDesc,
        containerClass: DescType,
        result: *mut AEDesc,
    ) -> OSErr,
>;
pub type OSLGetErrDescProcPtr =
    ::std::option::Option<unsafe extern "C" fn(appDescPtr: *mut *mut AEDesc) -> OSErr>;
pub type OSLMarkProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        dToken: *const AEDesc,
        markToken: *const AEDesc,
        index: ::std::os::raw::c_long,
    ) -> OSErr,
>;
pub type OSLAdjustMarksProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        newStart: ::std::os::raw::c_long,
        newStop: ::std::os::raw::c_long,
        markToken: *const AEDesc,
    ) -> OSErr,
>;
pub type OSLAccessorUPP = OSLAccessorProcPtr;
pub type OSLCompareUPP = OSLCompareProcPtr;
pub type OSLCountUPP = OSLCountProcPtr;
pub type OSLDisposeTokenUPP = OSLDisposeTokenProcPtr;
pub type OSLGetMarkTokenUPP = OSLGetMarkTokenProcPtr;
pub type OSLGetErrDescUPP = OSLGetErrDescProcPtr;
pub type OSLMarkUPP = OSLMarkProcPtr;
pub type OSLAdjustMarksUPP = OSLAdjustMarksProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TextRange {
    pub fStart: SInt32,
    pub fEnd: SInt32,
    pub fHiliteStyle: SInt16,
}
pub type TextRangePtr = *mut TextRange;
pub type TextRangeHandle = *mut TextRangePtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TextRangeArray {
    pub fNumOfRanges: SInt16,
    pub fRange: [TextRange; 1usize],
}
pub type TextRangeArrayPtr = *mut TextRangeArray;
pub type TextRangeArrayHandle = *mut TextRangeArrayPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct OffsetArray {
    pub fNumOfOffsets: SInt16,
    pub fOffset: [SInt32; 1usize],
}
pub type OffsetArrayPtr = *mut OffsetArray;
pub type OffsetArrayHandle = *mut OffsetArrayPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WritingCode {
    pub theScriptCode: ScriptCode,
    pub theLangCode: LangCode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IntlText {
    pub theScriptCode: ScriptCode,
    pub theLangCode: LangCode,
    pub theText: [::std::os::raw::c_char; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TScriptingSizeResource {
    pub scriptingSizeFlags: SInt16,
    pub minStackSize: UInt32,
    pub preferredStackSize: UInt32,
    pub maxStackSize: UInt32,
    pub minHeapSize: UInt32,
    pub preferredHeapSize: UInt32,
    pub maxHeapSize: UInt32,
}
pub type AEBuildErrorCode = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AEBuildError {
    pub fError: AEBuildErrorCode,
    pub fErrorPos: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAEStreamRef {
    _unused: [u8; 0],
}
pub type AEStreamRef = *mut OpaqueAEStreamRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DCSDictionary {
    _unused: [u8; 0],
}
pub type DCSDictionaryRef = *const __DCSDictionary;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CSIdentityAuthority {
    _unused: [u8; 0],
}
pub type CSIdentityAuthorityRef = *mut __CSIdentityAuthority;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CSIdentity {
    _unused: [u8; 0],
}
pub type CSIdentityRef = *mut __CSIdentity;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CSIdentityQuery {
    _unused: [u8; 0],
}
pub type CSIdentityQueryRef = *mut __CSIdentityQuery;
pub type CSIdentityClass = CFIndex;
pub type CSIdentityFlags = CFOptionFlags;
pub type CSIdentityStatusUpdatedCallback = ::std::option::Option<
    unsafe extern "C" fn(
        identity: CSIdentityRef,
        status: CFIndex,
        error: CFErrorRef,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CSIdentityClientContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
    pub statusUpdated: CSIdentityStatusUpdatedCallback,
}
pub type CSIdentityQueryFlags = CFOptionFlags;
pub type CSIdentityQueryStringComparisonMethod = CFIndex;
pub type CSIdentityQueryEvent = CFIndex;
pub type CSIdentityQueryReceiveEventCallback = ::std::option::Option<
    unsafe extern "C" fn(
        query: CSIdentityQueryRef,
        event: CSIdentityQueryEvent,
        identities: CFArrayRef,
        error: CFErrorRef,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CSIdentityQueryClientContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retainInfo: CFAllocatorRetainCallBack,
    pub releaseInfo: CFAllocatorReleaseCallBack,
    pub copyInfoDescription: CFAllocatorCopyDescriptionCallBack,
    pub receiveEvent: CSIdentityQueryReceiveEventCallback,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct IconFamilyElement {
    pub elementType: OSType,
    pub elementSize: SInt32,
    pub elementData: [::std::os::raw::c_uchar; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct IconFamilyResource {
    pub resourceType: OSType,
    pub resourceSize: SInt32,
    pub elements: [IconFamilyElement; 1usize],
}
pub type IconFamilyPtr = *mut IconFamilyResource;
pub type IconFamilyHandle = *mut IconFamilyPtr;
pub type SleepQRecPtr = *mut SleepQRec;
pub type SleepQProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        message: ::std::os::raw::c_long,
        qRecPtr: SleepQRecPtr,
    ) -> ::std::os::raw::c_long,
>;
pub type SleepQUPP = SleepQProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SleepQRec {
    pub sleepQLink: SleepQRecPtr,
    pub sleepQType: ::std::os::raw::c_short,
    pub sleepQProc: SleepQUPP,
    pub sleepQFlags: ::std::os::raw::c_short,
}
pub type KCRef = SecKeychainRef;
pub type KCItemRef = SecKeychainItemRef;
pub type KCSearchRef = SecKeychainSearchRef;
pub type KCAttribute = SecKeychainAttribute;
pub type KCAttributeList = SecKeychainAttributeList;
pub type KCAttrType = SecKeychainAttrType;
pub type KCStatus = SecKeychainStatus;
pub type KCEvent = UInt16;
pub type KCEventMask = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct KCCallbackInfo {
    pub version: UInt32,
    pub item: KCItemRef,
    pub processID: [SInt32; 2usize],
    pub event: [SInt32; 4usize],
    pub keychain: KCRef,
}
pub type KCItemClass = FourCharCode;
pub type KCItemAttr = FourCharCode;
pub type KCAuthType = FourCharCode;
pub type KCProtocolType = FourCharCode;
pub type KCCertAddOptions = UInt32;
pub type KCVerifyStopOn = UInt16;
pub type KCCertSearchOptions = UInt32;
pub type KCCallbackProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        keychainEvent: KCEvent,
        info: *mut KCCallbackInfo,
        userContext: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type KCCallbackUPP = KCCallbackProcPtr;
pub type WSTypeID = ::std::os::raw::c_uint;
pub type WSClientContextRetainCallBackProcPtr = ::std::option::Option<
    unsafe extern "C" fn(info: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
>;
pub type WSClientContextReleaseCallBackProcPtr =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
pub type WSClientContextCopyDescriptionCallBackProcPtr =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void) -> CFStringRef>;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct WSClientContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: WSClientContextRetainCallBackProcPtr,
    pub release: WSClientContextReleaseCallBackProcPtr,
    pub copyDescription: WSClientContextCopyDescriptionCallBackProcPtr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueWSMethodInvocationRef {
    _unused: [u8; 0],
}
pub type WSMethodInvocationRef = *mut OpaqueWSMethodInvocationRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueWSProtocolHandlerRef {
    _unused: [u8; 0],
}
pub type WSProtocolHandlerRef = *mut OpaqueWSProtocolHandlerRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueIconRef {
    _unused: [u8; 0],
}
pub type IconRef = *mut OpaqueIconRef;
pub type IconServicesUsageFlags = UInt32;
pub type LSRolesMask = OptionBits;
pub type LSAcceptanceFlags = OptionBits;
pub type LSRequestedInfo = OptionBits;
pub type LSItemInfoFlags = OptionBits;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LSItemInfoRecord {
    pub flags: LSItemInfoFlags,
    pub filetype: OSType,
    pub creator: OSType,
    pub extension: CFStringRef,
}
pub type LSHandlerOptions = OptionBits;
pub type LSLaunchFlags = OptionBits;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LSLaunchURLSpec {
    pub appURL: CFURLRef,
    pub itemURLs: CFArrayRef,
    pub passThruParams: *const AEDesc,
    pub launchFlags: LSLaunchFlags,
    pub asyncRefCon: *mut ::std::os::raw::c_void,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LSLaunchFSRefSpec {
    pub appRef: *const FSRef,
    pub numDocs: ItemCount,
    pub itemRefs: *const FSRef,
    pub passThruParams: *const AEDesc,
    pub launchFlags: LSLaunchFlags,
    pub asyncRefCon: *mut ::std::os::raw::c_void,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LSApplicationParameters {
    pub version: CFIndex,
    pub flags: LSLaunchFlags,
    pub application: *const FSRef,
    pub asyncLaunchRefCon: *mut ::std::os::raw::c_void,
    pub environment: CFDictionaryRef,
    pub argv: CFArrayRef,
    pub initialEvent: *mut AppleEvent,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __MDItem {
    _unused: [u8; 0],
}
pub type MDItemRef = *mut __MDItem;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __MDQuery {
    _unused: [u8; 0],
}
pub type MDQueryRef = *mut __MDQuery;
pub type MDQueryOptionFlags = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MDQueryBatchingParams {
    pub first_max_num: usize,
    pub first_max_ms: usize,
    pub progress_max_num: usize,
    pub progress_max_ms: usize,
    pub update_max_num: usize,
    pub update_max_ms: usize,
}
pub type MDQuerySortOptionFlags = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __MDLabel {
    _unused: [u8; 0],
}
pub type MDLabelRef = *mut __MDLabel;
pub type MDLabelDomain = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MDImporterInterfaceStruct {
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
    pub ImporterImportData: ::std::option::Option<
        unsafe extern "C" fn(
            thisInterface: *mut ::std::os::raw::c_void,
            attributes: CFMutableDictionaryRef,
            contentTypeUTI: CFStringRef,
            pathToFile: CFStringRef,
        ) -> Boolean,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MDExporterInterfaceStruct {
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
    pub ImporterExportData: ::std::option::Option<
        unsafe extern "C" fn(
            thisInterface: *mut ::std::os::raw::c_void,
            attributes: CFDictionaryRef,
            contentTypeUTI: CFStringRef,
            pathToFile: CFStringRef,
        ) -> Boolean,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MDImporterURLInterfaceStruct {
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
    pub ImporterImportURLData: ::std::option::Option<
        unsafe extern "C" fn(
            thisInterface: *mut ::std::os::raw::c_void,
            attributes: CFMutableDictionaryRef,
            contentTypeUTI: CFStringRef,
            urlForFile: CFURLRef,
        ) -> Boolean,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MDImporterBundleWrapperURLInterfaceStruct {
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
    pub ImporterImportBundleWrapperURLData: ::std::option::Option<
        unsafe extern "C" fn(
            thisInterface: *mut ::std::os::raw::c_void,
            attributes: CFMutableDictionaryRef,
            contentTypeUTI: CFStringRef,
            urlForFile: CFURLRef,
        ) -> Boolean,
    >,
}
pub type SKDocumentRef = CFTypeRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SKIndex {
    _unused: [u8; 0],
}
pub type SKIndexRef = *mut __SKIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SKIndexDocumentIterator {
    _unused: [u8; 0],
}
pub type SKIndexDocumentIteratorRef = *mut __SKIndexDocumentIterator;
pub type SKIndexType = ::std::os::raw::c_uint;
pub type SKDocumentIndexState = ::std::os::raw::c_uint;
pub type SKDocumentID = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SKSearch {
    _unused: [u8; 0],
}
pub type SKSearchRef = *mut __SKSearch;
pub type SKSearchOptions = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SKSearchGroup {
    _unused: [u8; 0],
}
pub type SKSearchGroupRef = *mut __SKSearchGroup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SKSearchResults {
    _unused: [u8; 0],
}
pub type SKSearchResultsRef = *mut __SKSearchResults;
pub type SKSearchType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SKSummary {
    _unused: [u8; 0],
}
pub type SKSummaryRef = *mut __SKSummary;
pub type FSEventStreamCreateFlags = UInt32;
pub type FSEventStreamEventFlags = UInt32;
pub type FSEventStreamEventId = UInt64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __FSEventStream {
    _unused: [u8; 0],
}
pub type FSEventStreamRef = *mut __FSEventStream;
pub type ConstFSEventStreamRef = *const __FSEventStream;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSEventStreamContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueLSSharedFileListRef {
    _unused: [u8; 0],
}
pub type LSSharedFileListRef = *mut OpaqueLSSharedFileListRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueLSSharedFileListItemRef {
    _unused: [u8; 0],
}
pub type LSSharedFileListItemRef = *mut OpaqueLSSharedFileListItemRef;
pub type LSSharedFileListResolutionFlags = UInt32;
unsafe extern "C" {
    pub fn Debugger();
}
unsafe extern "C" {
    pub fn DebugStr(debuggerMsg: ConstStr255Param);
}
unsafe extern "C" {
    pub fn SysBreak();
}
unsafe extern "C" {
    pub fn SysBreakStr(debuggerMsg: ConstStr255Param);
}
unsafe extern "C" {
    pub fn SysBreakFunc(debuggerMsg: ConstStr255Param);
}
unsafe extern "C" {
    pub fn FixRatio(numer: ::std::os::raw::c_short, denom: ::std::os::raw::c_short) -> Fixed;
}
unsafe extern "C" {
    pub fn FixMul(a: Fixed, b: Fixed) -> Fixed;
}
unsafe extern "C" {
    pub fn FixRound(x: Fixed) -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn Fix2Frac(x: Fixed) -> Fract;
}
unsafe extern "C" {
    pub fn Fix2Long(x: Fixed) -> SInt32;
}
unsafe extern "C" {
    pub fn Long2Fix(x: SInt32) -> Fixed;
}
unsafe extern "C" {
    pub fn Frac2Fix(x: Fract) -> Fixed;
}
unsafe extern "C" {
    pub fn FracMul(x: Fract, y: Fract) -> Fract;
}
unsafe extern "C" {
    pub fn FixDiv(x: Fixed, y: Fixed) -> Fixed;
}
unsafe extern "C" {
    pub fn FracDiv(x: Fract, y: Fract) -> Fract;
}
unsafe extern "C" {
    pub fn FracSqrt(x: Fract) -> Fract;
}
unsafe extern "C" {
    pub fn FracSin(x: Fixed) -> Fract;
}
unsafe extern "C" {
    pub fn FracCos(x: Fixed) -> Fract;
}
unsafe extern "C" {
    pub fn FixATan2(x: SInt32, y: SInt32) -> Fixed;
}
unsafe extern "C" {
    pub fn Frac2X(x: Fract) -> f64;
}
unsafe extern "C" {
    pub fn Fix2X(x: Fixed) -> f64;
}
unsafe extern "C" {
    pub fn X2Fix(x: f64) -> Fixed;
}
unsafe extern "C" {
    pub fn X2Frac(x: f64) -> Fract;
}
unsafe extern "C" {
    pub fn UnsignedFixedMulDiv(
        value: UnsignedFixed,
        multiplier: UnsignedFixed,
        divisor: UnsignedFixed,
    ) -> UnsignedFixed;
}
unsafe extern "C" {
    pub fn GetScriptManagerVariable(selector: ::std::os::raw::c_short) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn SetScriptManagerVariable(
        selector: ::std::os::raw::c_short,
        param: ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SysError(errorCode: ::std::os::raw::c_short);
}
unsafe extern "C" {
    pub fn CreateTextEncoding(
        encodingBase: TextEncodingBase,
        encodingVariant: TextEncodingVariant,
        encodingFormat: TextEncodingFormat,
    ) -> TextEncoding;
}
unsafe extern "C" {
    pub fn GetTextEncodingBase(encoding: TextEncoding) -> TextEncodingBase;
}
unsafe extern "C" {
    pub fn GetTextEncodingVariant(encoding: TextEncoding) -> TextEncodingVariant;
}
unsafe extern "C" {
    pub fn GetTextEncodingFormat(encoding: TextEncoding) -> TextEncodingFormat;
}
unsafe extern "C" {
    pub fn ResolveDefaultTextEncoding(encoding: TextEncoding) -> TextEncoding;
}
unsafe extern "C" {
    pub fn GetTextEncodingName(
        iEncoding: TextEncoding,
        iNamePartSelector: TextEncodingNameSelector,
        iPreferredRegion: RegionCode,
        iPreferredEncoding: TextEncoding,
        iOutputBufLen: ByteCount,
        oNameLength: *mut ByteCount,
        oActualRegion: *mut RegionCode,
        oActualEncoding: *mut TextEncoding,
        oEncodingName: TextPtr,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetInfo(tecInfo: *mut TECInfoHandle) -> OSStatus;
}
unsafe extern "C" {
    pub fn UpgradeScriptInfoToTextEncoding(
        iTextScriptID: ScriptCode,
        iTextLanguageID: LangCode,
        iRegionID: RegionCode,
        iTextFontname: ConstStr255Param,
        oEncoding: *mut TextEncoding,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn RevertTextEncodingToScriptInfo(
        iEncoding: TextEncoding,
        oTextScriptID: *mut ScriptCode,
        oTextLanguageID: *mut LangCode,
        oTextFontname: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetTextEncodingFromScriptInfo(
        iTextScriptID: ScriptCode,
        iTextLanguageID: LangCode,
        iTextRegionID: RegionCode,
        oEncoding: *mut TextEncoding,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetScriptInfoFromTextEncoding(
        iEncoding: TextEncoding,
        oTextScriptID: *mut ScriptCode,
        oTextLanguageID: *mut LangCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NearestMacTextEncodings(
        generalEncoding: TextEncoding,
        bestMacEncoding: *mut TextEncoding,
        alternateMacEncoding: *mut TextEncoding,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCGetCharProperty(
        charPtr: *const UniChar,
        textLength: UniCharCount,
        propType: UCCharPropertyType,
        propValue: *mut UCCharPropertyValue,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewCollectionFlattenUPP(userRoutine: CollectionFlattenProcPtr) -> CollectionFlattenUPP;
}
unsafe extern "C" {
    pub fn NewCollectionExceptionUPP(
        userRoutine: CollectionExceptionProcPtr,
    ) -> CollectionExceptionUPP;
}
unsafe extern "C" {
    pub fn DisposeCollectionFlattenUPP(userUPP: CollectionFlattenUPP);
}
unsafe extern "C" {
    pub fn DisposeCollectionExceptionUPP(userUPP: CollectionExceptionUPP);
}
unsafe extern "C" {
    pub fn InvokeCollectionFlattenUPP(
        size: SInt32,
        data: *mut ::std::os::raw::c_void,
        refCon: *mut ::std::os::raw::c_void,
        userUPP: CollectionFlattenUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeCollectionExceptionUPP(
        c: Collection,
        status: OSErr,
        userUPP: CollectionExceptionUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn NewCollection() -> Collection;
}
unsafe extern "C" {
    pub fn DisposeCollection(c: Collection);
}
unsafe extern "C" {
    pub fn CloneCollection(c: Collection) -> Collection;
}
unsafe extern "C" {
    pub fn CountCollectionOwners(c: Collection) -> SInt32;
}
unsafe extern "C" {
    pub fn RetainCollection(c: Collection) -> OSStatus;
}
unsafe extern "C" {
    pub fn ReleaseCollection(c: Collection) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetCollectionRetainCount(c: Collection) -> ItemCount;
}
unsafe extern "C" {
    pub fn CopyCollection(srcCollection: Collection, dstCollection: Collection) -> Collection;
}
unsafe extern "C" {
    pub fn GetCollectionDefaultAttributes(c: Collection) -> SInt32;
}
unsafe extern "C" {
    pub fn SetCollectionDefaultAttributes(
        c: Collection,
        whichAttributes: SInt32,
        newAttributes: SInt32,
    );
}
unsafe extern "C" {
    pub fn CountCollectionItems(c: Collection) -> SInt32;
}
unsafe extern "C" {
    pub fn AddCollectionItem(
        c: Collection,
        tag: CollectionTag,
        id: SInt32,
        itemSize: SInt32,
        itemData: *const ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetCollectionItem(
        c: Collection,
        tag: CollectionTag,
        id: SInt32,
        itemSize: *mut SInt32,
        itemData: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn RemoveCollectionItem(c: Collection, tag: CollectionTag, id: SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn SetCollectionItemInfo(
        c: Collection,
        tag: CollectionTag,
        id: SInt32,
        whichAttributes: SInt32,
        newAttributes: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetCollectionItemInfo(
        c: Collection,
        tag: CollectionTag,
        id: SInt32,
        itemIndex: *mut SInt32,
        itemSize: *mut SInt32,
        attributes: *mut SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn ReplaceIndexedCollectionItem(
        c: Collection,
        itemIndex: SInt32,
        itemSize: SInt32,
        itemData: *const ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIndexedCollectionItem(
        c: Collection,
        itemIndex: SInt32,
        itemSize: *mut SInt32,
        itemData: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn RemoveIndexedCollectionItem(c: Collection, itemIndex: SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn SetIndexedCollectionItemInfo(
        c: Collection,
        itemIndex: SInt32,
        whichAttributes: SInt32,
        newAttributes: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIndexedCollectionItemInfo(
        c: Collection,
        itemIndex: SInt32,
        tag: *mut CollectionTag,
        id: *mut SInt32,
        itemSize: *mut SInt32,
        attributes: *mut SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CollectionTagExists(c: Collection, tag: CollectionTag) -> Boolean;
}
unsafe extern "C" {
    pub fn CountCollectionTags(c: Collection) -> SInt32;
}
unsafe extern "C" {
    pub fn GetIndexedCollectionTag(
        c: Collection,
        tagIndex: SInt32,
        tag: *mut CollectionTag,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CountTaggedCollectionItems(c: Collection, tag: CollectionTag) -> SInt32;
}
unsafe extern "C" {
    pub fn GetTaggedCollectionItem(
        c: Collection,
        tag: CollectionTag,
        whichItem: SInt32,
        itemSize: *mut SInt32,
        itemData: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetTaggedCollectionItemInfo(
        c: Collection,
        tag: CollectionTag,
        whichItem: SInt32,
        id: *mut SInt32,
        itemIndex: *mut SInt32,
        itemSize: *mut SInt32,
        attributes: *mut SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PurgeCollection(c: Collection, whichAttributes: SInt32, matchingAttributes: SInt32);
}
unsafe extern "C" {
    pub fn PurgeCollectionTag(c: Collection, tag: CollectionTag);
}
unsafe extern "C" {
    pub fn EmptyCollection(c: Collection);
}
unsafe extern "C" {
    pub fn FlattenCollection(
        c: Collection,
        flattenProc: CollectionFlattenUPP,
        refCon: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FlattenPartialCollection(
        c: Collection,
        flattenProc: CollectionFlattenUPP,
        refCon: *mut ::std::os::raw::c_void,
        whichAttributes: SInt32,
        matchingAttributes: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn UnflattenCollection(
        c: Collection,
        flattenProc: CollectionFlattenUPP,
        refCon: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetCollectionExceptionProc(c: Collection) -> CollectionExceptionUPP;
}
unsafe extern "C" {
    pub fn SetCollectionExceptionProc(c: Collection, exceptionProc: CollectionExceptionUPP);
}
unsafe extern "C" {
    pub fn GetNewCollection(collectionID: SInt16) -> Collection;
}
unsafe extern "C" {
    pub fn AddCollectionItemHdl(
        aCollection: Collection,
        tag: CollectionTag,
        id: SInt32,
        itemData: Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetCollectionItemHdl(
        aCollection: Collection,
        tag: CollectionTag,
        id: SInt32,
        itemData: Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn ReplaceIndexedCollectionItemHdl(
        aCollection: Collection,
        itemIndex: SInt32,
        itemData: Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIndexedCollectionItemHdl(
        aCollection: Collection,
        itemIndex: SInt32,
        itemData: Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FlattenCollectionToHdl(aCollection: Collection, flattened: Handle) -> OSErr;
}
unsafe extern "C" {
    pub fn UnflattenCollectionFromHdl(aCollection: Collection, flattened: Handle) -> OSErr;
}
unsafe extern "C" {
    pub fn CoreEndianFlipData(
        dataDomain: OSType,
        dataType: OSType,
        id: SInt16,
        data: *mut ::std::os::raw::c_void,
        dataLen: ByteCount,
        currentlyNative: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn Gestalt(selector: OSType, response: *mut SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn NewGestaltValue(selector: OSType, newValue: SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn ReplaceGestaltValue(selector: OSType, replacementValue: SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn SetGestaltValue(selector: OSType, newValue: SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn DeleteGestaltValue(selector: OSType) -> OSErr;
}
unsafe extern "C" {
    pub fn NewSelectorFunctionUPP(userRoutine: SelectorFunctionProcPtr) -> SelectorFunctionUPP;
}
unsafe extern "C" {
    pub fn DisposeSelectorFunctionUPP(userUPP: SelectorFunctionUPP);
}
unsafe extern "C" {
    pub fn InvokeSelectorFunctionUPP(
        selector: OSType,
        response: *mut SInt32,
        userUPP: SelectorFunctionUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn MemError() -> OSErr;
}
unsafe extern "C" {
    pub fn LMGetMemErr() -> SInt16;
}
unsafe extern "C" {
    pub fn LMSetMemErr(value: SInt16);
}
unsafe extern "C" {
    pub fn NewHandle(byteCount: Size) -> Handle;
}
unsafe extern "C" {
    pub fn NewHandleClear(byteCount: Size) -> Handle;
}
unsafe extern "C" {
    pub fn RecoverHandle(p: Ptr) -> Handle;
}
unsafe extern "C" {
    pub fn NewPtr(byteCount: Size) -> Ptr;
}
unsafe extern "C" {
    pub fn NewPtrClear(byteCount: Size) -> Ptr;
}
unsafe extern "C" {
    pub fn NewEmptyHandle() -> Handle;
}
unsafe extern "C" {
    pub fn HLock(h: Handle);
}
unsafe extern "C" {
    pub fn HLockHi(h: Handle);
}
unsafe extern "C" {
    pub fn HUnlock(h: Handle);
}
unsafe extern "C" {
    pub fn TempNewHandle(logicalSize: Size, resultCode: *mut OSErr) -> Handle;
}
unsafe extern "C" {
    pub fn DisposePtr(p: Ptr);
}
unsafe extern "C" {
    pub fn GetPtrSize(p: Ptr) -> Size;
}
unsafe extern "C" {
    pub fn SetPtrSize(p: Ptr, newSize: Size);
}
unsafe extern "C" {
    pub fn DisposeHandle(h: Handle);
}
unsafe extern "C" {
    pub fn SetHandleSize(h: Handle, newSize: Size);
}
unsafe extern "C" {
    pub fn GetHandleSize(h: Handle) -> Size;
}
unsafe extern "C" {
    pub fn ReallocateHandle(h: Handle, byteCount: Size);
}
unsafe extern "C" {
    pub fn EmptyHandle(h: Handle);
}
unsafe extern "C" {
    pub fn HSetRBit(h: Handle);
}
unsafe extern "C" {
    pub fn HClrRBit(h: Handle);
}
unsafe extern "C" {
    pub fn HGetState(h: Handle) -> SInt8;
}
unsafe extern "C" {
    pub fn HSetState(h: Handle, flags: SInt8);
}
unsafe extern "C" {
    pub fn HandToHand(theHndl: *mut Handle) -> OSErr;
}
unsafe extern "C" {
    pub fn PtrToXHand(
        srcPtr: *const ::std::os::raw::c_void,
        dstHndl: Handle,
        size: ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PtrToHand(
        srcPtr: *const ::std::os::raw::c_void,
        dstHndl: *mut Handle,
        size: ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn HandAndHand(hand1: Handle, hand2: Handle) -> OSErr;
}
unsafe extern "C" {
    pub fn PtrAndHand(
        ptr1: *const ::std::os::raw::c_void,
        hand2: Handle,
        size: ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn IsHeapValid() -> Boolean;
}
unsafe extern "C" {
    pub fn IsHandleValid(h: Handle) -> Boolean;
}
unsafe extern "C" {
    pub fn IsPointerValid(p: Ptr) -> Boolean;
}
unsafe extern "C" {
    pub fn S64Absolute(value: SInt64) -> SInt64;
}
unsafe extern "C" {
    pub fn U64Compare(left: UInt64, right: UInt64) -> SInt32;
}
unsafe extern "C" {
    pub fn CSBackupSetItemExcluded(
        item: CFURLRef,
        exclude: Boolean,
        excludeByPath: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CSBackupIsItemExcluded(item: CFURLRef, excludeByPath: *mut Boolean) -> Boolean;
}
unsafe extern "C" {
    pub fn CSDiskSpaceCancelRecovery(operationUUID: CFUUIDRef);
}
unsafe extern "C" {
    pub fn CSDiskSpaceGetRecoveryEstimate(volumeURL: CFURLRef) -> UInt64;
}
unsafe extern "C" {
    pub fn UCConvertUTCDateTimeToCFAbsoluteTime(
        iUTCDate: *const UTCDateTime,
        oCFTime: *mut CFAbsoluteTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCConvertSecondsToCFAbsoluteTime(
        iSeconds: UInt32,
        oCFTime: *mut CFAbsoluteTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCConvertLongDateTimeToCFAbsoluteTime(
        iLongTime: LongDateTime,
        oCFTime: *mut CFAbsoluteTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCConvertCFAbsoluteTimeToUTCDateTime(
        iCFTime: CFAbsoluteTime,
        oUTCDate: *mut UTCDateTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCConvertCFAbsoluteTimeToSeconds(
        iCFTime: CFAbsoluteTime,
        oSeconds: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCConvertCFAbsoluteTimeToLongDateTime(
        iCFTime: CFAbsoluteTime,
        oLongDate: *mut LongDateTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn IsMetric() -> Boolean;
}
unsafe extern "C" {
    pub fn Delay(numTicks: ::std::os::raw::c_ulong, finalTicks: *mut ::std::os::raw::c_ulong);
}
unsafe extern "C" {
    pub fn Enqueue(qElement: QElemPtr, qHeader: QHdrPtr);
}
unsafe extern "C" {
    pub fn Dequeue(qElement: QElemPtr, qHeader: QHdrPtr) -> OSErr;
}
unsafe extern "C" {
    pub fn ReadLocation(loc: *mut MachineLocation);
}
unsafe extern "C" {
    pub fn TickCount() -> UInt32;
}
unsafe extern "C" {
    pub fn CSCopyUserName(useShortName: Boolean) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CSCopyMachineName() -> CFStringRef;
}
unsafe extern "C" {
    pub fn NewDeferredTaskUPP(userRoutine: DeferredTaskProcPtr) -> DeferredTaskUPP;
}
unsafe extern "C" {
    pub fn DisposeDeferredTaskUPP(userUPP: DeferredTaskUPP);
}
unsafe extern "C" {
    pub fn InvokeDeferredTaskUPP(dtParam: ::std::os::raw::c_long, userUPP: DeferredTaskUPP);
}
unsafe extern "C" {
    pub fn NewIOCompletionUPP(userRoutine: IOCompletionProcPtr) -> IOCompletionUPP;
}
unsafe extern "C" {
    pub fn DisposeIOCompletionUPP(userUPP: IOCompletionUPP);
}
unsafe extern "C" {
    pub fn InvokeIOCompletionUPP(paramBlock: ParmBlkPtr, userUPP: IOCompletionUPP);
}
unsafe extern "C" {
    pub fn FSMakeFSRefUnicode(
        parentRef: *const FSRef,
        nameLength: UniCharCount,
        name: *const UniChar,
        textEncodingHint: TextEncoding,
        newRef: *mut FSRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBMakeFSRefUnicodeSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBMakeFSRefUnicodeAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSCompareFSRefs(ref1: *const FSRef, ref2: *const FSRef) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCompareFSRefsSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCompareFSRefsAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSCreateFileUnicode(
        parentRef: *const FSRef,
        nameLength: UniCharCount,
        name: *const UniChar,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfo: *const FSCatalogInfo,
        newRef: *mut FSRef,
        newSpec: FSSpecPtr,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCreateFileUnicodeSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCreateFileUnicodeAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSCreateDirectoryUnicode(
        parentRef: *const FSRef,
        nameLength: UniCharCount,
        name: *const UniChar,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfo: *const FSCatalogInfo,
        newRef: *mut FSRef,
        newSpec: FSSpecPtr,
        newDirID: *mut UInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCreateDirectoryUnicodeSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCreateDirectoryUnicodeAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSDeleteObject(ref_: *const FSRef) -> OSErr;
}
unsafe extern "C" {
    pub fn PBDeleteObjectSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBDeleteObjectAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSUnlinkObject(ref_: *const FSRef) -> OSErr;
}
unsafe extern "C" {
    pub fn PBUnlinkObjectSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBUnlinkObjectAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSMoveObject(
        ref_: *const FSRef,
        destDirectory: *const FSRef,
        newRef: *mut FSRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBMoveObjectSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBMoveObjectAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSExchangeObjects(ref_: *const FSRef, destRef: *const FSRef) -> OSErr;
}
unsafe extern "C" {
    pub fn PBExchangeObjectsSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBExchangeObjectsAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSReplaceObject(
        originalObject: *const FSRef,
        replacementObject: *const FSRef,
        newName: CFStringRef,
        temporaryName: CFStringRef,
        temporaryDirectory: *const FSRef,
        flags: OptionBits,
        resultObject: *mut FSRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSPathReplaceObject(
        originalObjectPath: *const ::std::os::raw::c_char,
        replacementObjectPath: *const ::std::os::raw::c_char,
        newName: CFStringRef,
        temporaryName: CFStringRef,
        temporaryDirectoryPath: *const ::std::os::raw::c_char,
        flags: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetTemporaryDirectoryForReplaceObject(
        originalObject: *const FSRef,
        temporaryDirectory: *mut FSRef,
        flags: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSPathGetTemporaryDirectoryForReplaceObject(
        originalObjectPath: *const ::std::os::raw::c_char,
        temporaryDirectoryPath: *mut ::std::os::raw::c_char,
        maxPathSize: UInt32,
        flags: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSRenameUnicode(
        ref_: *const FSRef,
        nameLength: UniCharCount,
        name: *const UniChar,
        textEncodingHint: TextEncoding,
        newRef: *mut FSRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBRenameUnicodeSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBRenameUnicodeAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSGetCatalogInfo(
        ref_: *const FSRef,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfo: *mut FSCatalogInfo,
        outName: *mut HFSUniStr255,
        fsSpec: FSSpecPtr,
        parentRef: *mut FSRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetCatalogInfoSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetCatalogInfoAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSSetCatalogInfo(
        ref_: *const FSRef,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfo: *const FSCatalogInfo,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBSetCatalogInfoSync(paramBlock: *mut FSRefParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBSetCatalogInfoAsync(paramBlock: *mut FSRefParam);
}
unsafe extern "C" {
    pub fn FSOpenIterator(
        container: *const FSRef,
        iteratorFlags: FSIteratorFlags,
        iterator: *mut FSIterator,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBOpenIteratorSync(paramBlock: *mut FSCatalogBulkParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBOpenIteratorAsync(paramBlock: *mut FSCatalogBulkParam);
}
unsafe extern "C" {
    pub fn FSCloseIterator(iterator: FSIterator) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCloseIteratorSync(paramBlock: *mut FSCatalogBulkParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCloseIteratorAsync(paramBlock: *mut FSCatalogBulkParam);
}
unsafe extern "C" {
    pub fn FSGetCatalogInfoBulk(
        iterator: FSIterator,
        maximumObjects: ItemCount,
        actualObjects: *mut ItemCount,
        containerChanged: *mut Boolean,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfos: *mut FSCatalogInfo,
        refs: *mut FSRef,
        specs: FSSpecPtr,
        names: *mut HFSUniStr255,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetCatalogInfoBulkSync(paramBlock: *mut FSCatalogBulkParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetCatalogInfoBulkAsync(paramBlock: *mut FSCatalogBulkParam);
}
unsafe extern "C" {
    pub fn FSCatalogSearch(
        iterator: FSIterator,
        searchCriteria: *const FSSearchParams,
        maximumObjects: ItemCount,
        actualObjects: *mut ItemCount,
        containerChanged: *mut Boolean,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfos: *mut FSCatalogInfo,
        refs: *mut FSRef,
        specs: FSSpecPtr,
        names: *mut HFSUniStr255,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCatalogSearchSync(paramBlock: *mut FSCatalogBulkParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCatalogSearchAsync(paramBlock: *mut FSCatalogBulkParam);
}
unsafe extern "C" {
    pub fn FSCreateFileAndOpenForkUnicode(
        parentRef: *const FSRef,
        nameLength: UniCharCount,
        name: *const UniChar,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfo: *const FSCatalogInfo,
        forkNameLength: UniCharCount,
        forkName: *const UniChar,
        permissions: SInt8,
        forkRefNum: *mut FSIORefNum,
        newRef: *mut FSRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBCreateFileAndOpenForkUnicodeSync(paramBlock: FSRefForkIOParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBCreateFileAndOpenForkUnicodeAsync(paramBlock: FSRefForkIOParamPtr);
}
unsafe extern "C" {
    pub fn FSCreateFork(
        ref_: *const FSRef,
        forkNameLength: UniCharCount,
        forkName: *const UniChar,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCreateForkSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCreateForkAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSDeleteFork(
        ref_: *const FSRef,
        forkNameLength: UniCharCount,
        forkName: *const UniChar,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBDeleteForkSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBDeleteForkAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSIterateForks(
        ref_: *const FSRef,
        forkIterator: *mut CatPositionRec,
        forkName: *mut HFSUniStr255,
        forkSize: *mut SInt64,
        forkPhysicalSize: *mut UInt64,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBIterateForksSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBIterateForksAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSOpenFork(
        ref_: *const FSRef,
        forkNameLength: UniCharCount,
        forkName: *const UniChar,
        permissions: SInt8,
        forkRefNum: *mut FSIORefNum,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBOpenForkSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBOpenForkAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSReadFork(
        forkRefNum: FSIORefNum,
        positionMode: UInt16,
        positionOffset: SInt64,
        requestCount: ByteCount,
        buffer: *mut ::std::os::raw::c_void,
        actualCount: *mut ByteCount,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBReadForkSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBReadForkAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSWriteFork(
        forkRefNum: FSIORefNum,
        positionMode: UInt16,
        positionOffset: SInt64,
        requestCount: ByteCount,
        buffer: *const ::std::os::raw::c_void,
        actualCount: *mut ByteCount,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBWriteForkSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBWriteForkAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSGetForkPosition(forkRefNum: FSIORefNum, position: *mut SInt64) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetForkPositionSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetForkPositionAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSSetForkPosition(
        forkRefNum: FSIORefNum,
        positionMode: UInt16,
        positionOffset: SInt64,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBSetForkPositionSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBSetForkPositionAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSGetForkSize(forkRefNum: FSIORefNum, forkSize: *mut SInt64) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetForkSizeSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetForkSizeAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSSetForkSize(
        forkRefNum: FSIORefNum,
        positionMode: UInt16,
        positionOffset: SInt64,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBSetForkSizeSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBSetForkSizeAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSAllocateFork(
        forkRefNum: FSIORefNum,
        flags: FSAllocationFlags,
        positionMode: UInt16,
        positionOffset: SInt64,
        requestCount: UInt64,
        actualCount: *mut UInt64,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBAllocateForkSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBAllocateForkAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSFlushFork(forkRefNum: FSIORefNum) -> OSErr;
}
unsafe extern "C" {
    pub fn PBFlushForkSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBFlushForkAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSCloseFork(forkRefNum: FSIORefNum) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCloseForkSync(paramBlock: *mut FSForkIOParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBCloseForkAsync(paramBlock: *mut FSForkIOParam);
}
unsafe extern "C" {
    pub fn FSGetForkCBInfo(
        desiredRefNum: FSIORefNum,
        volume: FSVolumeRefNum,
        iterator: *mut ::std::os::raw::c_short,
        actualRefNum: *mut FSIORefNum,
        forkInfo: *mut FSForkInfo,
        ref_: *mut FSRef,
        outForkName: *mut HFSUniStr255,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetForkCBInfoSync(paramBlock: *mut FSForkCBInfoParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetForkCBInfoAsync(paramBlock: *mut FSForkCBInfoParam);
}
unsafe extern "C" {
    pub fn FSLockRange(
        forkRefNum: FSIORefNum,
        positionMode: UInt16,
        positionOffset: SInt64,
        requestCount: UInt64,
        rangeStart: *mut UInt64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBXLockRangeSync(paramBlock: FSRangeLockParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBXLockRangeAsync(paramBlock: FSRangeLockParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSUnlockRange(
        forkRefNum: FSIORefNum,
        positionMode: UInt16,
        positionOffset: SInt64,
        requestCount: UInt64,
        rangeStart: *mut UInt64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBXUnlockRangeSync(paramBlock: FSRangeLockParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBXUnlockRangeAsync(paramBlock: FSRangeLockParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetVolumeInfo(
        volume: FSVolumeRefNum,
        volumeIndex: ItemCount,
        actualVolume: *mut FSVolumeRefNum,
        whichInfo: FSVolumeInfoBitmap,
        info: *mut FSVolumeInfo,
        volumeName: *mut HFSUniStr255,
        rootDirectory: *mut FSRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetVolumeInfoSync(paramBlock: *mut FSVolumeInfoParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBGetVolumeInfoAsync(paramBlock: *mut FSVolumeInfoParam);
}
unsafe extern "C" {
    pub fn FSSetVolumeInfo(
        volume: FSVolumeRefNum,
        whichInfo: FSVolumeInfoBitmap,
        info: *const FSVolumeInfo,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn PBSetVolumeInfoSync(paramBlock: *mut FSVolumeInfoParam) -> OSErr;
}
unsafe extern "C" {
    pub fn PBSetVolumeInfoAsync(paramBlock: *mut FSVolumeInfoParam);
}
unsafe extern "C" {
    pub fn FSGetDataForkName(dataForkName: *mut HFSUniStr255) -> OSErr;
}
unsafe extern "C" {
    pub fn FSGetResourceForkName(resourceForkName: *mut HFSUniStr255) -> OSErr;
}
unsafe extern "C" {
    pub fn FSRefMakePath(ref_: *const FSRef, path: *mut UInt8, pathBufferSize: UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSPathMakeRef(
        path: *const UInt8,
        ref_: *mut FSRef,
        isDirectory: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSPathMakeRefWithOptions(
        path: *const UInt8,
        options: OptionBits,
        ref_: *mut FSRef,
        isDirectory: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSIsFSRefValid(ref_: *const FSRef) -> Boolean;
}
unsafe extern "C" {
    pub fn FNNotify(ref_: *const FSRef, message: FNMessage, flags: OptionBits) -> OSStatus;
}
unsafe extern "C" {
    pub fn FNNotifyByPath(path: *const UInt8, message: FNMessage, flags: OptionBits) -> OSStatus;
}
unsafe extern "C" {
    pub fn FNNotifyAll(message: FNMessage, flags: OptionBits) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewFNSubscriptionUPP(userRoutine: FNSubscriptionProcPtr) -> FNSubscriptionUPP;
}
unsafe extern "C" {
    pub fn DisposeFNSubscriptionUPP(userUPP: FNSubscriptionUPP);
}
unsafe extern "C" {
    pub fn InvokeFNSubscriptionUPP(
        message: FNMessage,
        flags: OptionBits,
        refcon: *mut ::std::os::raw::c_void,
        subscription: FNSubscriptionRef,
        userUPP: FNSubscriptionUPP,
    );
}
unsafe extern "C" {
    pub fn FNSubscribe(
        directoryRef: *const FSRef,
        callback: FNSubscriptionUPP,
        refcon: *mut ::std::os::raw::c_void,
        flags: OptionBits,
        subscription: *mut FNSubscriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FNSubscribeByPath(
        directoryPath: *const UInt8,
        callback: FNSubscriptionUPP,
        refcon: *mut ::std::os::raw::c_void,
        flags: OptionBits,
        subscription: *mut FNSubscriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FNUnsubscribe(subscription: FNSubscriptionRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn FNGetDirectoryForSubscription(
        subscription: FNSubscriptionRef,
        ref_: *mut FSRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewFSVolumeMountUPP(userRoutine: FSVolumeMountProcPtr) -> FSVolumeMountUPP;
}
unsafe extern "C" {
    pub fn NewFSVolumeUnmountUPP(userRoutine: FSVolumeUnmountProcPtr) -> FSVolumeUnmountUPP;
}
unsafe extern "C" {
    pub fn NewFSVolumeEjectUPP(userRoutine: FSVolumeEjectProcPtr) -> FSVolumeEjectUPP;
}
unsafe extern "C" {
    pub fn DisposeFSVolumeMountUPP(userUPP: FSVolumeMountUPP);
}
unsafe extern "C" {
    pub fn DisposeFSVolumeUnmountUPP(userUPP: FSVolumeUnmountUPP);
}
unsafe extern "C" {
    pub fn DisposeFSVolumeEjectUPP(userUPP: FSVolumeEjectUPP);
}
unsafe extern "C" {
    pub fn InvokeFSVolumeMountUPP(
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        err: OSStatus,
        mountedVolumeRefNum: FSVolumeRefNum,
        userUPP: FSVolumeMountUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeFSVolumeUnmountUPP(
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        err: OSStatus,
        volumeRefNum: FSVolumeRefNum,
        dissenter: pid_t,
        userUPP: FSVolumeUnmountUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeFSVolumeEjectUPP(
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        err: OSStatus,
        volumeRefNum: FSVolumeRefNum,
        dissenter: pid_t,
        userUPP: FSVolumeEjectUPP,
    );
}
unsafe extern "C" {
    pub fn FSCreateVolumeOperation(volumeOp: *mut FSVolumeOperation) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSDisposeVolumeOperation(volumeOp: FSVolumeOperation) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSMountLocalVolumeSync(
        diskID: CFStringRef,
        mountDir: CFURLRef,
        mountedVolumeRefNum: *mut FSVolumeRefNum,
        flags: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSMountLocalVolumeAsync(
        diskID: CFStringRef,
        mountDir: CFURLRef,
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        flags: OptionBits,
        callback: FSVolumeMountUPP,
        runloop: CFRunLoopRef,
        runloopMode: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSMountServerVolumeSync(
        url: CFURLRef,
        mountDir: CFURLRef,
        user: CFStringRef,
        password: CFStringRef,
        mountedVolumeRefNum: *mut FSVolumeRefNum,
        flags: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSMountServerVolumeAsync(
        url: CFURLRef,
        mountDir: CFURLRef,
        user: CFStringRef,
        password: CFStringRef,
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        flags: OptionBits,
        callback: FSVolumeMountUPP,
        runloop: CFRunLoopRef,
        runloopMode: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetAsyncMountStatus(
        volumeOp: FSVolumeOperation,
        status: *mut FSMountStatus,
        volumeOpStatus: *mut OSStatus,
        mountedVolumeRefNum: *mut FSVolumeRefNum,
        clientData: *mut *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSUnmountVolumeSync(
        vRefNum: FSVolumeRefNum,
        flags: OptionBits,
        dissenter: *mut pid_t,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSUnmountVolumeAsync(
        vRefNum: FSVolumeRefNum,
        flags: OptionBits,
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        callback: FSVolumeUnmountUPP,
        runloop: CFRunLoopRef,
        runloopMode: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetAsyncUnmountStatus(
        volumeOp: FSVolumeOperation,
        status: *mut FSUnmountStatus,
        volumeOpStatus: *mut OSStatus,
        volumeRefNum: *mut FSVolumeRefNum,
        dissenter: *mut pid_t,
        clientData: *mut *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSCancelVolumeOperation(volumeOp: FSVolumeOperation) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSEjectVolumeSync(
        vRefNum: FSVolumeRefNum,
        flags: OptionBits,
        dissenter: *mut pid_t,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSEjectVolumeAsync(
        vRefNum: FSVolumeRefNum,
        flags: OptionBits,
        volumeOp: FSVolumeOperation,
        clientData: *mut ::std::os::raw::c_void,
        callback: FSVolumeEjectUPP,
        runloop: CFRunLoopRef,
        runloopMode: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetAsyncEjectStatus(
        volumeOp: FSVolumeOperation,
        status: *mut FSEjectStatus,
        volumeOpStatus: *mut OSStatus,
        volumeRefNum: *mut FSVolumeRefNum,
        dissenter: *mut pid_t,
        clientData: *mut *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSCopyDiskIDForVolume(vRefNum: FSVolumeRefNum, diskID: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSCopyURLForVolume(vRefNum: FSVolumeRefNum, url: *mut CFURLRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetVolumeForDiskID(diskID: CFStringRef, vRefNum: *mut FSVolumeRefNum) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSCopyDADiskForVolume(vRefNum: FSVolumeRefNum, disk: *mut DADiskRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetVolumeForDADisk(disk: DADiskRef, vRefNum: *mut FSVolumeRefNum) -> OSStatus;
}
unsafe extern "C" {
    pub static kFSOperationTotalBytesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationBytesCompleteKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationBytesRemainingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationTotalObjectsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationObjectsCompleteKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationObjectsRemainingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationTotalUserVisibleObjectsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationUserVisibleObjectsCompleteKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationUserVisibleObjectsRemainingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFSOperationThroughputKey: CFStringRef;
}
unsafe extern "C" {
    pub fn FSCopyObjectSync(
        source: *const FSRef,
        destDir: *const FSRef,
        destName: CFStringRef,
        target: *mut FSRef,
        options: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSMoveObjectSync(
        source: *const FSRef,
        destDir: *const FSRef,
        destName: CFStringRef,
        target: *mut FSRef,
        options: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSMoveObjectToTrashSync(
        source: *const FSRef,
        target: *mut FSRef,
        options: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSPathCopyObjectSync(
        sourcePath: *const ::std::os::raw::c_char,
        destDirPath: *const ::std::os::raw::c_char,
        destName: CFStringRef,
        targetPath: *mut *mut ::std::os::raw::c_char,
        options: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSPathMoveObjectSync(
        sourcePath: *const ::std::os::raw::c_char,
        destDirPath: *const ::std::os::raw::c_char,
        destName: CFStringRef,
        targetPath: *mut *mut ::std::os::raw::c_char,
        options: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSPathMoveObjectToTrashSync(
        sourcePath: *const ::std::os::raw::c_char,
        targetPath: *mut *mut ::std::os::raw::c_char,
        options: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileOperationGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn FSFileOperationCreate(alloc: CFAllocatorRef) -> FSFileOperationRef;
}
unsafe extern "C" {
    pub fn FSFileOperationScheduleWithRunLoop(
        fileOp: FSFileOperationRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileOperationUnscheduleFromRunLoop(
        fileOp: FSFileOperationRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileOperationCancel(fileOp: FSFileOperationRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileOperationCopyStatus(
        fileOp: FSFileOperationRef,
        currentItem: *mut FSRef,
        stage: *mut FSFileOperationStage,
        error: *mut OSStatus,
        statusDictionary: *mut CFDictionaryRef,
        info: *mut *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSPathFileOperationCopyStatus(
        fileOp: FSFileOperationRef,
        currentItem: *mut *mut ::std::os::raw::c_char,
        stage: *mut FSFileOperationStage,
        error: *mut OSStatus,
        statusDictionary: *mut CFDictionaryRef,
        info: *mut *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSCreateStringFromHFSUniStr(
        alloc: CFAllocatorRef,
        uniStr: *const HFSUniStr255,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn FSGetHFSUniStrFromString(theString: CFStringRef, uniStr: *mut HFSUniStr255) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecurityGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn FSFileSecurityCreate(alloc: CFAllocatorRef) -> FSFileSecurityRef;
}
unsafe extern "C" {
    pub fn FSFileSecurityCreateWithFSPermissionInfo(
        alloc: CFAllocatorRef,
        permissions: *const FSPermissionInfo,
    ) -> FSFileSecurityRef;
}
unsafe extern "C" {
    pub fn FSFileSecurityRefCreateCopy(
        alloc: CFAllocatorRef,
        fileSec: FSFileSecurityRef,
    ) -> FSFileSecurityRef;
}
unsafe extern "C" {
    pub fn FSFileSecurityGetOwnerUUID(
        fileSec: FSFileSecurityRef,
        owner: *mut CFUUIDBytes,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecuritySetOwnerUUID(
        fileSec: FSFileSecurityRef,
        owner: *const CFUUIDBytes,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecurityGetGroupUUID(
        fileSec: FSFileSecurityRef,
        group: *mut CFUUIDBytes,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecuritySetGroupUUID(
        fileSec: FSFileSecurityRef,
        group: *const CFUUIDBytes,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecurityCopyAccessControlList(
        fileSec: FSFileSecurityRef,
        accessControlList: *mut acl_t,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecuritySetAccessControlList(
        fileSec: FSFileSecurityRef,
        accessControlList: acl_t,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecurityGetOwner(fileSec: FSFileSecurityRef, owner: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecuritySetOwner(fileSec: FSFileSecurityRef, owner: UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecurityGetGroup(fileSec: FSFileSecurityRef, group: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecuritySetGroup(fileSec: FSFileSecurityRef, group: UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecurityGetMode(fileSec: FSFileSecurityRef, mode: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFileSecuritySetMode(fileSec: FSFileSecurityRef, mode: UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetVolumeParms(
        volume: FSVolumeRefNum,
        buffer: *mut GetVolParmsInfoBuffer,
        bufferSize: ByteCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSGetVolumeMountInfoSize(volume: FSVolumeRefNum, size: *mut ByteCount) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSFlushVolume(vRefNum: FSVolumeRefNum) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBFlushVolumeSync(paramBlock: FSRefParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBFlushVolumeAsync(paramBlock: FSRefParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBFSCopyFileSync(paramBlock: FSRefParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBFSCopyFileAsync(paramBlock: FSRefParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSResolveNodeID(volume: FSVolumeRefNum, nodeID: UInt32, newRef: FSRefPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBFSResolveNodeIDSync(paramBlock: FSRefParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn PBFSResolveNodeIDAsync(paramBlock: FSRefParamPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewResErrUPP(userRoutine: ResErrProcPtr) -> ResErrUPP;
}
unsafe extern "C" {
    pub fn DisposeResErrUPP(userUPP: ResErrUPP);
}
unsafe extern "C" {
    pub fn InvokeResErrUPP(thErr: OSErr, userUPP: ResErrUPP);
}
unsafe extern "C" {
    pub fn CloseResFile(refNum: ResFileRefNum);
}
unsafe extern "C" {
    pub fn ResError() -> OSErr;
}
unsafe extern "C" {
    pub fn CurResFile() -> ResFileRefNum;
}
unsafe extern "C" {
    pub fn HomeResFile(theResource: Handle) -> ResFileRefNum;
}
unsafe extern "C" {
    pub fn UseResFile(refNum: ResFileRefNum);
}
unsafe extern "C" {
    pub fn CountTypes() -> ResourceCount;
}
unsafe extern "C" {
    pub fn Count1Types() -> ResourceCount;
}
unsafe extern "C" {
    pub fn GetIndType(theType: *mut ResType, itemIndex: ResourceIndex);
}
unsafe extern "C" {
    pub fn Get1IndType(theType: *mut ResType, itemIndex: ResourceIndex);
}
unsafe extern "C" {
    pub fn SetResLoad(load: Boolean);
}
unsafe extern "C" {
    pub fn CountResources(theType: ResType) -> ResourceCount;
}
unsafe extern "C" {
    pub fn Count1Resources(theType: ResType) -> ResourceCount;
}
unsafe extern "C" {
    pub fn GetIndResource(theType: ResType, itemIndex: ResourceIndex) -> Handle;
}
unsafe extern "C" {
    pub fn Get1IndResource(theType: ResType, itemIndex: ResourceIndex) -> Handle;
}
unsafe extern "C" {
    pub fn GetResource(theType: ResType, theID: ResID) -> Handle;
}
unsafe extern "C" {
    pub fn Get1Resource(theType: ResType, theID: ResID) -> Handle;
}
unsafe extern "C" {
    pub fn GetNamedResource(theType: ResType, name: ConstStr255Param) -> Handle;
}
unsafe extern "C" {
    pub fn Get1NamedResource(theType: ResType, name: ConstStr255Param) -> Handle;
}
unsafe extern "C" {
    pub fn LoadResource(theResource: Handle);
}
unsafe extern "C" {
    pub fn ReleaseResource(theResource: Handle);
}
unsafe extern "C" {
    pub fn DetachResource(theResource: Handle);
}
unsafe extern "C" {
    pub fn UniqueID(theType: ResType) -> ResID;
}
unsafe extern "C" {
    pub fn Unique1ID(theType: ResType) -> ResID;
}
unsafe extern "C" {
    pub fn GetResAttrs(theResource: Handle) -> ResAttributes;
}
unsafe extern "C" {
    pub fn GetResInfo(
        theResource: Handle,
        theID: *mut ResID,
        theType: *mut ResType,
        name: *mut ::std::os::raw::c_uchar,
    );
}
unsafe extern "C" {
    pub fn SetResInfo(theResource: Handle, theID: ResID, name: ConstStr255Param);
}
unsafe extern "C" {
    pub fn AddResource(theData: Handle, theType: ResType, theID: ResID, name: ConstStr255Param);
}
unsafe extern "C" {
    pub fn GetResourceSizeOnDisk(theResource: Handle) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetMaxResourceSize(theResource: Handle) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn SetResAttrs(theResource: Handle, attrs: ResAttributes);
}
unsafe extern "C" {
    pub fn ChangedResource(theResource: Handle);
}
unsafe extern "C" {
    pub fn RemoveResource(theResource: Handle);
}
unsafe extern "C" {
    pub fn UpdateResFile(refNum: ResFileRefNum);
}
unsafe extern "C" {
    pub fn WriteResource(theResource: Handle);
}
unsafe extern "C" {
    pub fn SetResPurge(install: Boolean);
}
unsafe extern "C" {
    pub fn GetResFileAttrs(refNum: ResFileRefNum) -> ResFileAttributes;
}
unsafe extern "C" {
    pub fn SetResFileAttrs(refNum: ResFileRefNum, attrs: ResFileAttributes);
}
unsafe extern "C" {
    pub fn ReadPartialResource(
        theResource: Handle,
        offset: ::std::os::raw::c_long,
        buffer: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
    );
}
unsafe extern "C" {
    pub fn WritePartialResource(
        theResource: Handle,
        offset: ::std::os::raw::c_long,
        buffer: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
    );
}
unsafe extern "C" {
    pub fn SetResourceSize(theResource: Handle, newSize: ::std::os::raw::c_long);
}
unsafe extern "C" {
    pub fn GetNextFOND(fondHandle: Handle) -> Handle;
}
unsafe extern "C" {
    pub fn InsertResourceFile(refNum: ResFileRefNum, where_: RsrcChainLocation) -> OSErr;
}
unsafe extern "C" {
    pub fn DetachResourceFile(refNum: ResFileRefNum) -> OSErr;
}
unsafe extern "C" {
    pub fn GetTopResourceFile(refNum: *mut ResFileRefNum) -> OSErr;
}
unsafe extern "C" {
    pub fn GetNextResourceFile(curRefNum: ResFileRefNum, nextRefNum: *mut ResFileRefNum) -> OSErr;
}
unsafe extern "C" {
    pub fn FSOpenResFile(ref_: *const FSRef, permission: SInt8) -> ResFileRefNum;
}
unsafe extern "C" {
    pub fn FSCreateResFile(
        parentRef: *const FSRef,
        nameLength: UniCharCount,
        name: *const UniChar,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfo: *const FSCatalogInfo,
        newRef: *mut FSRef,
        newSpec: FSSpecPtr,
    );
}
unsafe extern "C" {
    pub fn FSResourceFileAlreadyOpen(
        resourceFileRef: *const FSRef,
        inChain: *mut Boolean,
        refNum: *mut ResFileRefNum,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn FSOpenOrphanResFile(
        ref_: *const FSRef,
        permission: SignedByte,
        refNum: *mut ResFileRefNum,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSCreateResourceFile(
        parentRef: *const FSRef,
        nameLength: UniCharCount,
        name: *const UniChar,
        whichInfo: FSCatalogInfoBitmap,
        catalogInfo: *const FSCatalogInfo,
        forkNameLength: UniCharCount,
        forkName: *const UniChar,
        newRef: *mut FSRef,
        newSpec: FSSpecPtr,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSCreateResourceFork(
        ref_: *const FSRef,
        forkNameLength: UniCharCount,
        forkName: *const UniChar,
        flags: UInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSOpenResourceFile(
        ref_: *const FSRef,
        forkNameLength: UniCharCount,
        forkName: *const UniChar,
        permissions: SInt8,
        refNum: *mut ResFileRefNum,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CSSetComponentsThreadMode(mode: CSComponentsThreadMode);
}
unsafe extern "C" {
    pub fn CSGetComponentsThreadMode() -> CSComponentsThreadMode;
}
unsafe extern "C" {
    pub fn NewComponentFunctionUPP(
        userRoutine: ProcPtr,
        procInfo: ProcInfoType,
    ) -> ComponentFunctionUPP;
}
unsafe extern "C" {
    pub fn DisposeComponentFunctionUPP(userUPP: ComponentFunctionUPP);
}
unsafe extern "C" {
    pub fn RegisterComponent(
        cd: *mut ComponentDescription,
        componentEntryPoint: ComponentRoutineUPP,
        global: SInt16,
        componentName: Handle,
        componentInfo: Handle,
        componentIcon: Handle,
    ) -> Component;
}
unsafe extern "C" {
    pub fn RegisterComponentResource(cr: ComponentResourceHandle, global: SInt16) -> Component;
}
unsafe extern "C" {
    pub fn UnregisterComponent(aComponent: Component) -> OSErr;
}
unsafe extern "C" {
    pub fn FindNextComponent(
        aComponent: Component,
        looking: *mut ComponentDescription,
    ) -> Component;
}
unsafe extern "C" {
    pub fn CountComponents(looking: *mut ComponentDescription) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetComponentInfo(
        aComponent: Component,
        cd: *mut ComponentDescription,
        componentName: Handle,
        componentInfo: Handle,
        componentIcon: Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetComponentListModSeed() -> SInt32;
}
unsafe extern "C" {
    pub fn GetComponentTypeModSeed(componentType: OSType) -> SInt32;
}
unsafe extern "C" {
    pub fn OpenAComponent(aComponent: Component, ci: *mut ComponentInstance) -> OSErr;
}
unsafe extern "C" {
    pub fn OpenComponent(aComponent: Component) -> ComponentInstance;
}
unsafe extern "C" {
    pub fn CloseComponent(aComponentInstance: ComponentInstance) -> OSErr;
}
unsafe extern "C" {
    pub fn GetComponentInstanceError(aComponentInstance: ComponentInstance) -> OSErr;
}
unsafe extern "C" {
    pub fn ResolveComponentAlias(aComponent: Component) -> Component;
}
unsafe extern "C" {
    pub fn GetComponentPublicResource(
        aComponent: Component,
        resourceType: OSType,
        resourceID: SInt16,
        theResource: *mut Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetComponentPublicResourceList(
        resourceType: OSType,
        resourceID: SInt16,
        flags: SInt32,
        cd: *mut ComponentDescription,
        missingProc: GetMissingComponentResourceUPP,
        refCon: *mut ::std::os::raw::c_void,
        atomContainerPtr: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetComponentPublicIndString(
        aComponent: Component,
        theString: *mut ::std::os::raw::c_uchar,
        strListID: SInt16,
        index: SInt16,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SetComponentInstanceError(aComponentInstance: ComponentInstance, theError: OSErr);
}
unsafe extern "C" {
    pub fn GetComponentRefcon(aComponent: Component) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn SetComponentRefcon(aComponent: Component, theRefcon: ::std::os::raw::c_long);
}
unsafe extern "C" {
    pub fn OpenComponentResFile(aComponent: Component) -> ResFileRefNum;
}
unsafe extern "C" {
    pub fn OpenAComponentResFile(aComponent: Component, resRef: *mut ResFileRefNum) -> OSErr;
}
unsafe extern "C" {
    pub fn CloseComponentResFile(refnum: ResFileRefNum) -> OSErr;
}
unsafe extern "C" {
    pub fn GetComponentResource(
        aComponent: Component,
        resType: OSType,
        resID: SInt16,
        theResource: *mut Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetComponentIndString(
        aComponent: Component,
        theString: *mut ::std::os::raw::c_uchar,
        strListID: SInt16,
        index: SInt16,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetComponentInstanceStorage(aComponentInstance: ComponentInstance) -> Handle;
}
unsafe extern "C" {
    pub fn SetComponentInstanceStorage(aComponentInstance: ComponentInstance, theStorage: Handle);
}
unsafe extern "C" {
    pub fn CountComponentInstances(aComponent: Component) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn CallComponentFunction(
        params: *mut ComponentParameters,
        func: ComponentFunctionUPP,
    ) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentFunctionWithStorage(
        storage: Handle,
        params: *mut ComponentParameters,
        func: ComponentFunctionUPP,
    ) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentFunctionWithStorageProcInfo(
        storage: Handle,
        params: *mut ComponentParameters,
        func: ProcPtr,
        funcProcInfo: ProcInfoType,
    ) -> ComponentResult;
}
unsafe extern "C" {
    pub fn DelegateComponentCall(
        originalParams: *mut ComponentParameters,
        ci: ComponentInstance,
    ) -> ComponentResult;
}
unsafe extern "C" {
    pub fn SetDefaultComponent(aComponent: Component, flags: SInt16) -> OSErr;
}
unsafe extern "C" {
    pub fn OpenDefaultComponent(
        componentType: OSType,
        componentSubType: OSType,
    ) -> ComponentInstance;
}
unsafe extern "C" {
    pub fn OpenADefaultComponent(
        componentType: OSType,
        componentSubType: OSType,
        ci: *mut ComponentInstance,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CaptureComponent(
        capturedComponent: Component,
        capturingComponent: Component,
    ) -> Component;
}
unsafe extern "C" {
    pub fn UncaptureComponent(aComponent: Component) -> OSErr;
}
unsafe extern "C" {
    pub fn RegisterComponentResourceFile(resRefNum: SInt16, global: SInt16) -> SInt32;
}
unsafe extern "C" {
    pub fn RegisterComponentFileRef(ref_: *const FSRef, global: SInt16) -> OSErr;
}
unsafe extern "C" {
    pub fn RegisterComponentFileRefEntries(
        ref_: *const FSRef,
        global: SInt16,
        toRegister: *const ComponentDescription,
        registerCount: UInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CallComponentOpen(ci: ComponentInstance, self_: ComponentInstance) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentClose(ci: ComponentInstance, self_: ComponentInstance) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentCanDo(ci: ComponentInstance, ftnNumber: SInt16) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentVersion(ci: ComponentInstance) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentRegister(ci: ComponentInstance) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentTarget(ci: ComponentInstance, target: ComponentInstance)
        -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentUnregister(ci: ComponentInstance) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentGetMPWorkFunction(
        ci: ComponentInstance,
        workFunction: *mut ComponentMPWorkFunctionUPP,
        refCon: *mut *mut ::std::os::raw::c_void,
    ) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentGetPublicResource(
        ci: ComponentInstance,
        resourceType: OSType,
        resourceID: SInt16,
        resource: *mut Handle,
    ) -> ComponentResult;
}
unsafe extern "C" {
    pub fn CallComponentDispatch(cp: *mut ComponentParameters) -> ComponentResult;
}
unsafe extern "C" {
    pub fn NewComponentMPWorkFunctionUPP(
        userRoutine: ComponentMPWorkFunctionProcPtr,
    ) -> ComponentMPWorkFunctionUPP;
}
unsafe extern "C" {
    pub fn NewComponentRoutineUPP(userRoutine: ComponentRoutineProcPtr) -> ComponentRoutineUPP;
}
unsafe extern "C" {
    pub fn NewGetMissingComponentResourceUPP(
        userRoutine: GetMissingComponentResourceProcPtr,
    ) -> GetMissingComponentResourceUPP;
}
unsafe extern "C" {
    pub fn DisposeComponentMPWorkFunctionUPP(userUPP: ComponentMPWorkFunctionUPP);
}
unsafe extern "C" {
    pub fn DisposeComponentRoutineUPP(userUPP: ComponentRoutineUPP);
}
unsafe extern "C" {
    pub fn DisposeGetMissingComponentResourceUPP(userUPP: GetMissingComponentResourceUPP);
}
unsafe extern "C" {
    pub fn InvokeComponentMPWorkFunctionUPP(
        globalRefCon: *mut ::std::os::raw::c_void,
        header: ComponentMPWorkFunctionHeaderRecordPtr,
        userUPP: ComponentMPWorkFunctionUPP,
    ) -> ComponentResult;
}
unsafe extern "C" {
    pub fn InvokeComponentRoutineUPP(
        cp: *mut ComponentParameters,
        componentStorage: Handle,
        userUPP: ComponentRoutineUPP,
    ) -> ComponentResult;
}
unsafe extern "C" {
    pub fn InvokeGetMissingComponentResourceUPP(
        c: Component,
        resType: OSType,
        resID: SInt16,
        refCon: *mut ::std::os::raw::c_void,
        resource: *mut Handle,
        userUPP: GetMissingComponentResourceUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn MPProcessors() -> ItemCount;
}
unsafe extern "C" {
    pub fn MPProcessorsScheduled() -> ItemCount;
}
unsafe extern "C" {
    pub fn MPTerminateTask(task: MPTaskID, terminationStatus: OSStatus) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPSetTaskWeight(task: MPTaskID, weight: MPTaskWeight) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPTaskIsPreemptive(taskID: MPTaskID) -> Boolean;
}
unsafe extern "C" {
    pub fn MPExit(status: OSStatus);
}
unsafe extern "C" {
    pub fn MPYield();
}
unsafe extern "C" {
    pub fn MPCurrentTaskID() -> MPTaskID;
}
unsafe extern "C" {
    pub fn MPSetTaskType(task: MPTaskID, taskType: OSType) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPAllocateTaskStorageIndex(taskIndex: *mut TaskStorageIndex) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDeallocateTaskStorageIndex(taskIndex: TaskStorageIndex) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPSetTaskStorageValue(taskIndex: TaskStorageIndex, value: TaskStorageValue) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPGetTaskStorageValue(taskIndex: TaskStorageIndex) -> TaskStorageValue;
}
unsafe extern "C" {
    pub fn MPCreateQueue(queue: *mut MPQueueID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDeleteQueue(queue: MPQueueID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPNotifyQueue(
        queue: MPQueueID,
        param1: *mut ::std::os::raw::c_void,
        param2: *mut ::std::os::raw::c_void,
        param3: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPWaitOnQueue(
        queue: MPQueueID,
        param1: *mut *mut ::std::os::raw::c_void,
        param2: *mut *mut ::std::os::raw::c_void,
        param3: *mut *mut ::std::os::raw::c_void,
        timeout: Duration,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPSetQueueReserve(queue: MPQueueID, count: ItemCount) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPCreateSemaphore(
        maximumValue: MPSemaphoreCount,
        initialValue: MPSemaphoreCount,
        semaphore: *mut MPSemaphoreID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDeleteSemaphore(semaphore: MPSemaphoreID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPSignalSemaphore(semaphore: MPSemaphoreID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPWaitOnSemaphore(semaphore: MPSemaphoreID, timeout: Duration) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPCreateCriticalRegion(criticalRegion: *mut MPCriticalRegionID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDeleteCriticalRegion(criticalRegion: MPCriticalRegionID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPEnterCriticalRegion(criticalRegion: MPCriticalRegionID, timeout: Duration)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn MPExitCriticalRegion(criticalRegion: MPCriticalRegionID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPCreateEvent(event: *mut MPEventID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDeleteEvent(event: MPEventID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPSetEvent(event: MPEventID, flags: MPEventFlags) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPWaitForEvent(
        event: MPEventID,
        flags: *mut MPEventFlags,
        timeout: Duration,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPCreateNotification(notificationID: *mut MPNotificationID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDeleteNotification(notificationID: MPNotificationID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPModifyNotification(
        notificationID: MPNotificationID,
        anID: MPOpaqueID,
        notifyParam1: *mut ::std::os::raw::c_void,
        notifyParam2: *mut ::std::os::raw::c_void,
        notifyParam3: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPModifyNotificationParameters(
        notificationID: MPNotificationID,
        kind: MPOpaqueIDClass,
        notifyParam1: *mut ::std::os::raw::c_void,
        notifyParam2: *mut ::std::os::raw::c_void,
        notifyParam3: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPCauseNotification(notificationID: MPNotificationID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDelayUntil(expirationTime: *mut AbsoluteTime) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPCreateTimer(timerID: *mut MPTimerID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDeleteTimer(timerID: MPTimerID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPSetTimerNotify(
        timerID: MPTimerID,
        anID: MPOpaqueID,
        notifyParam1: *mut ::std::os::raw::c_void,
        notifyParam2: *mut ::std::os::raw::c_void,
        notifyParam3: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPArmTimer(
        timerID: MPTimerID,
        expirationTime: *mut AbsoluteTime,
        options: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPCancelTimer(timerID: MPTimerID, timeRemaining: *mut AbsoluteTime) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPAllocateAligned(
        size: ByteCount,
        alignment: UInt8,
        options: OptionBits,
    ) -> LogicalAddress;
}
unsafe extern "C" {
    pub fn MPAllocate(size: ByteCount) -> LogicalAddress;
}
unsafe extern "C" {
    pub fn MPFree(object: LogicalAddress);
}
unsafe extern "C" {
    pub fn MPGetAllocatedBlockSize(object: LogicalAddress) -> ByteCount;
}
unsafe extern "C" {
    pub fn MPBlockCopy(source: LogicalAddress, destination: LogicalAddress, size: ByteCount);
}
unsafe extern "C" {
    pub fn MPBlockClear(address: LogicalAddress, size: ByteCount);
}
unsafe extern "C" {
    pub fn MPSetExceptionHandler(task: MPTaskID, exceptionQ: MPQueueID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPDisposeTaskException(task: MPTaskID, action: OptionBits) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPExtractTaskState(
        task: MPTaskID,
        kind: MPTaskStateKind,
        info: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPSetTaskState(
        task: MPTaskID,
        kind: MPTaskStateKind,
        info: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPThrowException(task: MPTaskID, kind: MPExceptionKind) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPRegisterDebugger(queue: MPQueueID, level: MPDebuggerLevel) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPUnregisterDebugger(queue: MPQueueID) -> OSStatus;
}
unsafe extern "C" {
    pub fn _MPIsFullyInitialized() -> Boolean;
}
unsafe extern "C" {
    pub fn _MPLibraryVersion(
        versionCString: *mut *const ::std::os::raw::c_char,
        major: *mut UInt32,
        minor: *mut UInt32,
        release: *mut UInt32,
        revision: *mut UInt32,
    );
}
unsafe extern "C" {
    pub fn _MPLibraryIsCompatible(
        versionCString: *const ::std::os::raw::c_char,
        major: UInt32,
        minor: UInt32,
        release: UInt32,
        revision: UInt32,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn FSNewAlias(
        fromFile: *const FSRef,
        target: *const FSRef,
        inAlias: *mut AliasHandle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSNewAliasMinimal(target: *const FSRef, inAlias: *mut AliasHandle) -> OSErr;
}
unsafe extern "C" {
    pub fn FSIsAliasFile(
        fileRef: *const FSRef,
        aliasFileFlag: *mut Boolean,
        folderFlag: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSResolveAliasWithMountFlags(
        fromFile: *const FSRef,
        inAlias: AliasHandle,
        target: *mut FSRef,
        wasChanged: *mut Boolean,
        mountFlags: ::std::os::raw::c_ulong,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSResolveAlias(
        fromFile: *const FSRef,
        alias: AliasHandle,
        target: *mut FSRef,
        wasChanged: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSResolveAliasFileWithMountFlags(
        theRef: *mut FSRef,
        resolveAliasChains: Boolean,
        targetIsFolder: *mut Boolean,
        wasAliased: *mut Boolean,
        mountFlags: ::std::os::raw::c_ulong,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSResolveAliasFile(
        theRef: *mut FSRef,
        resolveAliasChains: Boolean,
        targetIsFolder: *mut Boolean,
        wasAliased: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSFollowFinderAlias(
        fromFile: *mut FSRef,
        alias: AliasHandle,
        logon: Boolean,
        target: *mut FSRef,
        wasChanged: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSUpdateAlias(
        fromFile: *const FSRef,
        target: *const FSRef,
        alias: AliasHandle,
        wasChanged: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSNewAliasUnicode(
        fromFile: *const FSRef,
        targetParentRef: *const FSRef,
        targetNameLength: UniCharCount,
        targetName: *const UniChar,
        inAlias: *mut AliasHandle,
        isDirectory: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSNewAliasMinimalUnicode(
        targetParentRef: *const FSRef,
        targetNameLength: UniCharCount,
        targetName: *const UniChar,
        inAlias: *mut AliasHandle,
        isDirectory: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSNewAliasFromPath(
        fromFilePath: *const ::std::os::raw::c_char,
        targetPath: *const ::std::os::raw::c_char,
        flags: OptionBits,
        inAlias: *mut AliasHandle,
        isDirectory: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FSCopyAliasInfo(
        inAlias: AliasHandle,
        targetName: *mut HFSUniStr255,
        volumeName: *mut HFSUniStr255,
        pathString: *mut CFStringRef,
        whichInfo: *mut FSAliasInfoBitmap,
        info: *mut FSAliasInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetAliasSize(alias: AliasHandle) -> Size;
}
unsafe extern "C" {
    pub fn GetAliasUserType(alias: AliasHandle) -> OSType;
}
unsafe extern "C" {
    pub fn SetAliasUserType(alias: AliasHandle, userType: OSType);
}
unsafe extern "C" {
    pub fn GetAliasSizeFromPtr(alias: *const AliasRecord) -> Size;
}
unsafe extern "C" {
    pub fn GetAliasUserTypeFromPtr(alias: *const AliasRecord) -> OSType;
}
unsafe extern "C" {
    pub fn SetAliasUserTypeWithPtr(alias: AliasPtr, userType: OSType);
}
unsafe extern "C" {
    pub fn LocaleRefFromLangOrRegionCode(
        lang: LangCode,
        region: RegionCode,
        locale: *mut LocaleRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleRefFromLocaleString(
        localeString: *const ::std::os::raw::c_char,
        locale: *mut LocaleRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleRefGetPartString(
        locale: LocaleRef,
        partMask: LocalePartMask,
        maxStringLen: ByteCount,
        partString: *mut ::std::os::raw::c_char,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleStringToLangAndRegionCodes(
        localeString: *const ::std::os::raw::c_char,
        lang: *mut LangCode,
        region: *mut RegionCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleOperationCountLocales(
        opClass: LocaleOperationClass,
        localeCount: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleOperationGetLocales(
        opClass: LocaleOperationClass,
        maxLocaleCount: ItemCount,
        actualLocaleCount: *mut ItemCount,
        localeVariantList: *mut LocaleAndVariant,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleGetName(
        locale: LocaleRef,
        opVariant: LocaleOperationVariant,
        nameMask: LocaleNameMask,
        displayLocale: LocaleRef,
        maxNameLen: UniCharCount,
        actualNameLen: *mut UniCharCount,
        displayName: *mut UniChar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleCountNames(
        locale: LocaleRef,
        opVariant: LocaleOperationVariant,
        nameMask: LocaleNameMask,
        nameCount: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleGetIndName(
        locale: LocaleRef,
        opVariant: LocaleOperationVariant,
        nameMask: LocaleNameMask,
        nameIndex: ItemCount,
        maxNameLen: UniCharCount,
        actualNameLen: *mut UniCharCount,
        displayName: *mut UniChar,
        displayLocale: *mut LocaleRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleOperationGetName(
        opClass: LocaleOperationClass,
        displayLocale: LocaleRef,
        maxNameLen: UniCharCount,
        actualNameLen: *mut UniCharCount,
        displayName: *mut UniChar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleOperationCountNames(
        opClass: LocaleOperationClass,
        nameCount: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LocaleOperationGetIndName(
        opClass: LocaleOperationClass,
        nameIndex: ItemCount,
        maxNameLen: UniCharCount,
        actualNameLen: *mut UniCharCount,
        displayName: *mut UniChar,
        displayLocale: *mut LocaleRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DebugAssert(
        componentSignature: OSType,
        options: UInt32,
        assertionString: *const ::std::os::raw::c_char,
        exceptionLabelString: *const ::std::os::raw::c_char,
        errorString: *const ::std::os::raw::c_char,
        fileName: *const ::std::os::raw::c_char,
        lineNumber: ::std::os::raw::c_long,
        value: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn TaskLevel() -> UInt32;
}
unsafe extern "C" {
    pub fn NewDebugComponent(
        componentSignature: OSType,
        componentName: ConstStr255Param,
        componentCallback: DebugComponentCallbackUPP,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewDebugOption(
        componentSignature: OSType,
        optionSelectorNum: SInt32,
        optionName: ConstStr255Param,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DisposeDebugComponent(componentSignature: OSType) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetDebugComponentInfo(
        itemIndex: UInt32,
        componentSignature: *mut OSType,
        componentName: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetDebugOptionInfo(
        itemIndex: UInt32,
        componentSignature: OSType,
        optionSelectorNum: *mut SInt32,
        optionName: *mut ::std::os::raw::c_uchar,
        optionSetting: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SetDebugOptionValue(
        componentSignature: OSType,
        optionSelectorNum: SInt32,
        newOptionSetting: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn InstallDebugAssertOutputHandler(handler: DebugAssertOutputHandlerUPP);
}
unsafe extern "C" {
    pub fn GetMacOSStatusErrorString(err: OSStatus) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn GetMacOSStatusCommentString(err: OSStatus) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn NewDebugComponentCallbackUPP(
        userRoutine: DebugComponentCallbackProcPtr,
    ) -> DebugComponentCallbackUPP;
}
unsafe extern "C" {
    pub fn NewDebugAssertOutputHandlerUPP(
        userRoutine: DebugAssertOutputHandlerProcPtr,
    ) -> DebugAssertOutputHandlerUPP;
}
unsafe extern "C" {
    pub fn DisposeDebugComponentCallbackUPP(userUPP: DebugComponentCallbackUPP);
}
unsafe extern "C" {
    pub fn DisposeDebugAssertOutputHandlerUPP(userUPP: DebugAssertOutputHandlerUPP);
}
unsafe extern "C" {
    pub fn InvokeDebugComponentCallbackUPP(
        optionSelectorNum: SInt32,
        command: UInt32,
        optionSetting: *mut Boolean,
        userUPP: DebugComponentCallbackUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeDebugAssertOutputHandlerUPP(
        componentSignature: OSType,
        options: UInt32,
        assertionString: *const ::std::os::raw::c_char,
        exceptionLabelString: *const ::std::os::raw::c_char,
        errorString: *const ::std::os::raw::c_char,
        fileName: *const ::std::os::raw::c_char,
        lineNumber: ::std::os::raw::c_long,
        value: *mut ::std::os::raw::c_void,
        outputMsg: ConstStr255Param,
        userUPP: DebugAssertOutputHandlerUPP,
    );
}
unsafe extern "C" {
    pub fn PLstrcmp(str1: ConstStr255Param, str2: ConstStr255Param) -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn PLstrncmp(
        str1: ConstStr255Param,
        str2: ConstStr255Param,
        num: ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn PLstrcpy(dest: StringPtr, source: ConstStr255Param) -> StringPtr;
}
unsafe extern "C" {
    pub fn PLstrncpy(
        dest: StringPtr,
        source: ConstStr255Param,
        num: ::std::os::raw::c_short,
    ) -> StringPtr;
}
unsafe extern "C" {
    pub fn PLstrcat(str_: StringPtr, append: ConstStr255Param) -> StringPtr;
}
unsafe extern "C" {
    pub fn PLstrncat(
        str1: StringPtr,
        append: ConstStr255Param,
        num: ::std::os::raw::c_short,
    ) -> StringPtr;
}
unsafe extern "C" {
    pub fn PLstrchr(str1: ConstStr255Param, ch1: ::std::os::raw::c_short) -> Ptr;
}
unsafe extern "C" {
    pub fn PLstrrchr(str1: ConstStr255Param, ch1: ::std::os::raw::c_short) -> Ptr;
}
unsafe extern "C" {
    pub fn PLstrpbrk(str1: ConstStr255Param, charSet: ConstStr255Param) -> Ptr;
}
unsafe extern "C" {
    pub fn PLstrspn(str1: ConstStr255Param, charSet: ConstStr255Param) -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn PLstrstr(str1: ConstStr255Param, searchStr: ConstStr255Param) -> Ptr;
}
unsafe extern "C" {
    pub fn PLstrlen(str_: ConstStr255Param) -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn PLpos(str1: ConstStr255Param, searchStr: ConstStr255Param) -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn CompareAndSwap(oldValue: UInt32, newValue: UInt32, address: *mut UInt32) -> Boolean;
}
unsafe extern "C" {
    pub fn TestAndClear(bit: UInt32, address: *mut UInt8) -> Boolean;
}
unsafe extern "C" {
    pub fn TestAndSet(bit: UInt32, address: *mut UInt8) -> Boolean;
}
unsafe extern "C" {
    pub fn IncrementAtomic8(address: *mut SInt8) -> SInt8;
}
unsafe extern "C" {
    pub fn DecrementAtomic8(address: *mut SInt8) -> SInt8;
}
unsafe extern "C" {
    pub fn AddAtomic8(amount: SInt32, address: *mut SInt8) -> SInt8;
}
unsafe extern "C" {
    pub fn BitAndAtomic8(mask: UInt32, address: *mut UInt8) -> UInt8;
}
unsafe extern "C" {
    pub fn BitOrAtomic8(mask: UInt32, address: *mut UInt8) -> UInt8;
}
unsafe extern "C" {
    pub fn BitXorAtomic8(mask: UInt32, address: *mut UInt8) -> UInt8;
}
unsafe extern "C" {
    pub fn IncrementAtomic16(address: *mut SInt16) -> SInt16;
}
unsafe extern "C" {
    pub fn DecrementAtomic16(address: *mut SInt16) -> SInt16;
}
unsafe extern "C" {
    pub fn AddAtomic16(amount: SInt32, address: *mut SInt16) -> SInt16;
}
unsafe extern "C" {
    pub fn BitAndAtomic16(mask: UInt32, address: *mut UInt16) -> UInt16;
}
unsafe extern "C" {
    pub fn BitOrAtomic16(mask: UInt32, address: *mut UInt16) -> UInt16;
}
unsafe extern "C" {
    pub fn BitXorAtomic16(mask: UInt32, address: *mut UInt16) -> UInt16;
}
unsafe extern "C" {
    pub fn IncrementAtomic(address: *mut SInt32) -> SInt32;
}
unsafe extern "C" {
    pub fn DecrementAtomic(address: *mut SInt32) -> SInt32;
}
unsafe extern "C" {
    pub fn AddAtomic(amount: SInt32, address: *mut SInt32) -> SInt32;
}
unsafe extern "C" {
    pub fn BitAndAtomic(mask: UInt32, address: *mut UInt32) -> UInt32;
}
unsafe extern "C" {
    pub fn BitOrAtomic(mask: UInt32, address: *mut UInt32) -> UInt32;
}
unsafe extern "C" {
    pub fn BitXorAtomic(mask: UInt32, address: *mut UInt32) -> UInt32;
}
unsafe extern "C" {
    pub fn NewExceptionHandlerUPP(userRoutine: ExceptionHandlerProcPtr) -> ExceptionHandlerUPP;
}
unsafe extern "C" {
    pub fn DisposeExceptionHandlerUPP(userUPP: ExceptionHandlerUPP);
}
unsafe extern "C" {
    pub fn InvokeExceptionHandlerUPP(
        theException: *mut ExceptionInformation,
        userUPP: ExceptionHandlerUPP,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn InstallExceptionHandler(theHandler: ExceptionHandlerTPP) -> ExceptionHandlerTPP;
}
unsafe extern "C" {
    pub fn UpTime() -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn AbsoluteToNanoseconds(absoluteTime: AbsoluteTime) -> Nanoseconds;
}
unsafe extern "C" {
    pub fn AbsoluteToDuration(absoluteTime: AbsoluteTime) -> Duration;
}
unsafe extern "C" {
    pub fn NanosecondsToAbsolute(nanoseconds: Nanoseconds) -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn DurationToAbsolute(duration: Duration) -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn AddAbsoluteToAbsolute(
        absoluteTime1: AbsoluteTime,
        absoluteTime2: AbsoluteTime,
    ) -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn SubAbsoluteFromAbsolute(
        leftAbsoluteTime: AbsoluteTime,
        rightAbsoluteTime: AbsoluteTime,
    ) -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn AddNanosecondsToAbsolute(
        nanoseconds: Nanoseconds,
        absoluteTime: AbsoluteTime,
    ) -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn AddDurationToAbsolute(duration: Duration, absoluteTime: AbsoluteTime) -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn SubNanosecondsFromAbsolute(
        nanoseconds: Nanoseconds,
        absoluteTime: AbsoluteTime,
    ) -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn SubDurationFromAbsolute(duration: Duration, absoluteTime: AbsoluteTime) -> AbsoluteTime;
}
unsafe extern "C" {
    pub fn AbsoluteDeltaToNanoseconds(
        leftAbsoluteTime: AbsoluteTime,
        rightAbsoluteTime: AbsoluteTime,
    ) -> Nanoseconds;
}
unsafe extern "C" {
    pub fn AbsoluteDeltaToDuration(
        leftAbsoluteTime: AbsoluteTime,
        rightAbsoluteTime: AbsoluteTime,
    ) -> Duration;
}
unsafe extern "C" {
    pub fn DurationToNanoseconds(theDuration: Duration) -> Nanoseconds;
}
unsafe extern "C" {
    pub fn NanosecondsToDuration(theNanoseconds: Nanoseconds) -> Duration;
}
unsafe extern "C" {
    pub fn numtostring(theNum: ::std::os::raw::c_long, theString: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Munger(
        h: Handle,
        offset: ::std::os::raw::c_long,
        ptr1: *const ::std::os::raw::c_void,
        len1: ::std::os::raw::c_long,
        ptr2: *const ::std::os::raw::c_void,
        len2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn BitTst(
        bytePtr: *const ::std::os::raw::c_void,
        bitNum: ::std::os::raw::c_long,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn BitSet(bytePtr: *mut ::std::os::raw::c_void, bitNum: ::std::os::raw::c_long);
}
unsafe extern "C" {
    pub fn BitClr(bytePtr: *mut ::std::os::raw::c_void, bitNum: ::std::os::raw::c_long);
}
unsafe extern "C" {
    pub fn BitAnd(
        value1: ::std::os::raw::c_long,
        value2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn BitOr(
        value1: ::std::os::raw::c_long,
        value2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn BitXor(
        value1: ::std::os::raw::c_long,
        value2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn BitNot(value: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn BitShift(
        value: ::std::os::raw::c_long,
        count: ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn NewIndexToUCStringUPP(userRoutine: IndexToUCStringProcPtr) -> IndexToUCStringUPP;
}
unsafe extern "C" {
    pub fn DisposeIndexToUCStringUPP(userUPP: IndexToUCStringUPP);
}
unsafe extern "C" {
    pub fn InvokeIndexToUCStringUPP(
        index: UInt32,
        listDataPtr: *mut ::std::os::raw::c_void,
        refcon: *mut ::std::os::raw::c_void,
        outString: *mut CFStringRef,
        tsOptions: *mut UCTypeSelectOptions,
        userUPP: IndexToUCStringUPP,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn UCKeyTranslate(
        keyLayoutPtr: *const UCKeyboardLayout,
        virtualKeyCode: UInt16,
        keyAction: UInt16,
        modifierKeyState: UInt32,
        keyboardType: UInt32,
        keyTranslateOptions: OptionBits,
        deadKeyState: *mut UInt32,
        maxStringLength: UniCharCount,
        actualStringLength: *mut UniCharCount,
        unicodeString: *mut UniChar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCCreateCollator(
        locale: LocaleRef,
        opVariant: LocaleOperationVariant,
        options: UCCollateOptions,
        collatorRef: *mut CollatorRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCGetCollationKey(
        collatorRef: CollatorRef,
        textPtr: *const UniChar,
        textLength: UniCharCount,
        maxKeySize: ItemCount,
        actualKeySize: *mut ItemCount,
        collationKey: *mut UCCollationValue,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCCompareCollationKeys(
        key1Ptr: *const UCCollationValue,
        key1Length: ItemCount,
        key2Ptr: *const UCCollationValue,
        key2Length: ItemCount,
        equivalent: *mut Boolean,
        order: *mut SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCCompareText(
        collatorRef: CollatorRef,
        text1Ptr: *const UniChar,
        text1Length: UniCharCount,
        text2Ptr: *const UniChar,
        text2Length: UniCharCount,
        equivalent: *mut Boolean,
        order: *mut SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCDisposeCollator(collatorRef: *mut CollatorRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCCompareTextDefault(
        options: UCCollateOptions,
        text1Ptr: *const UniChar,
        text1Length: UniCharCount,
        text2Ptr: *const UniChar,
        text2Length: UniCharCount,
        equivalent: *mut Boolean,
        order: *mut SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCCompareTextNoLocale(
        options: UCCollateOptions,
        text1Ptr: *const UniChar,
        text1Length: UniCharCount,
        text2Ptr: *const UniChar,
        text2Length: UniCharCount,
        equivalent: *mut Boolean,
        order: *mut SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCCreateTextBreakLocator(
        locale: LocaleRef,
        opVariant: LocaleOperationVariant,
        breakTypes: UCTextBreakType,
        breakRef: *mut TextBreakLocatorRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCFindTextBreak(
        breakRef: TextBreakLocatorRef,
        breakType: UCTextBreakType,
        options: UCTextBreakOptions,
        textPtr: *const UniChar,
        textLength: UniCharCount,
        startOffset: UniCharArrayOffset,
        breakOffset: *mut UniCharArrayOffset,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCDisposeTextBreakLocator(breakRef: *mut TextBreakLocatorRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCTypeSelectCreateSelector(
        locale: LocaleRef,
        opVariant: LocaleOperationVariant,
        options: UCCollateOptions,
        newSelector: *mut UCTypeSelectRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCTypeSelectFlushSelectorData(ref_: UCTypeSelectRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCTypeSelectReleaseSelector(ref_: *mut UCTypeSelectRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCTypeSelectWouldResetBuffer(
        inRef: UCTypeSelectRef,
        inText: CFStringRef,
        inEventTime: f64,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn UCTypeSelectAddKeyToSelector(
        inRef: UCTypeSelectRef,
        inText: CFStringRef,
        inEventTime: f64,
        updateFlag: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCTypeSelectCompare(
        ref_: UCTypeSelectRef,
        inText: CFStringRef,
        result: *mut UCTypeSelectCompareResult,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCTypeSelectFindItem(
        ref_: UCTypeSelectRef,
        listSize: UInt32,
        listDataPtr: *mut ::std::os::raw::c_void,
        refcon: *mut ::std::os::raw::c_void,
        userUPP: IndexToUCStringUPP,
        closestItem: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UCTypeSelectWalkList(
        ref_: UCTypeSelectRef,
        currSelect: CFStringRef,
        direction: UCTSWalkDirection,
        listSize: UInt32,
        listDataPtr: *mut ::std::os::raw::c_void,
        refcon: *mut ::std::os::raw::c_void,
        userUPP: IndexToUCStringUPP,
        closestItem: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static pi: double_t;
}
unsafe extern "C" {
    pub fn compound(rate: f64, periods: f64) -> f64;
}
unsafe extern "C" {
    pub fn annuity(rate: f64, periods: f64) -> f64;
}
unsafe extern "C" {
    pub fn randomx(x: *mut double_t) -> double_t;
}
unsafe extern "C" {
    pub fn relation(x: double_t, y: double_t) -> relop;
}
unsafe extern "C" {
    pub fn num2dec(f: *const decform, x: double_t, d: *mut decimal);
}
unsafe extern "C" {
    pub fn dec2num(d: *const decimal) -> double_t;
}
unsafe extern "C" {
    pub fn dec2str(f: *const decform, d: *const decimal, s: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn str2dec(
        s: *const ::std::os::raw::c_char,
        ix: *mut ::std::os::raw::c_short,
        d: *mut decimal,
        vp: *mut ::std::os::raw::c_short,
    );
}
unsafe extern "C" {
    pub fn dec2f(d: *const decimal) -> f32;
}
unsafe extern "C" {
    pub fn dec2s(d: *const decimal) -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn dec2l(d: *const decimal) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn relationl(x: f64, y: f64) -> relop;
}
unsafe extern "C" {
    pub fn num2decl(f: *const decform, x: f64, d: *mut decimal);
}
unsafe extern "C" {
    pub fn dec2numl(d: *const decimal) -> f64;
}
unsafe extern "C" {
    pub fn x80tod(x80: *const extended80) -> f64;
}
unsafe extern "C" {
    pub fn dtox80(x: *const f64, x80: *mut extended80);
}
unsafe extern "C" {
    pub fn x80told(x80: *const extended80, x: *mut f64);
}
unsafe extern "C" {
    pub fn ldtox80(x: *const f64, x80: *mut extended80);
}
unsafe extern "C" {
    pub fn TECCountAvailableTextEncodings(numberEncodings: *mut ItemCount) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetAvailableTextEncodings(
        availableEncodings: *mut TextEncoding,
        maxAvailableEncodings: ItemCount,
        actualAvailableEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCountDirectTextEncodingConversions(numberOfEncodings: *mut ItemCount) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetDirectTextEncodingConversions(
        availableConversions: *mut TECConversionInfo,
        maxAvailableConversions: ItemCount,
        actualAvailableConversions: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCountDestinationTextEncodings(
        inputEncoding: TextEncoding,
        numberOfEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetDestinationTextEncodings(
        inputEncoding: TextEncoding,
        destinationEncodings: *mut TextEncoding,
        maxDestinationEncodings: ItemCount,
        actualDestinationEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetTextEncodingInternetName(
        textEncoding: TextEncoding,
        encodingName: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetTextEncodingFromInternetName(
        textEncoding: *mut TextEncoding,
        encodingName: ConstStr255Param,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCreateConverter(
        newEncodingConverter: *mut TECObjectRef,
        inputEncoding: TextEncoding,
        outputEncoding: TextEncoding,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCreateConverterFromPath(
        newEncodingConverter: *mut TECObjectRef,
        inPath: *const TextEncoding,
        inEncodings: ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECDisposeConverter(newEncodingConverter: TECObjectRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECClearConverterContextInfo(encodingConverter: TECObjectRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECConvertText(
        encodingConverter: TECObjectRef,
        inputBuffer: ConstTextPtr,
        inputBufferLength: ByteCount,
        actualInputLength: *mut ByteCount,
        outputBuffer: TextPtr,
        outputBufferLength: ByteCount,
        actualOutputLength: *mut ByteCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECFlushText(
        encodingConverter: TECObjectRef,
        outputBuffer: TextPtr,
        outputBufferLength: ByteCount,
        actualOutputLength: *mut ByteCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCountSubTextEncodings(
        inputEncoding: TextEncoding,
        numberOfEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetSubTextEncodings(
        inputEncoding: TextEncoding,
        subEncodings: *mut TextEncoding,
        maxSubEncodings: ItemCount,
        actualSubEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetEncodingList(
        encodingConverter: TECObjectRef,
        numEncodings: *mut ItemCount,
        encodingList: *mut Handle,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCreateOneToManyConverter(
        newEncodingConverter: *mut TECObjectRef,
        inputEncoding: TextEncoding,
        numOutputEncodings: ItemCount,
        outputEncodings: *const TextEncoding,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECConvertTextToMultipleEncodings(
        encodingConverter: TECObjectRef,
        inputBuffer: ConstTextPtr,
        inputBufferLength: ByteCount,
        actualInputLength: *mut ByteCount,
        outputBuffer: TextPtr,
        outputBufferLength: ByteCount,
        actualOutputLength: *mut ByteCount,
        outEncodingsBuffer: *mut TextEncodingRun,
        maxOutEncodingRuns: ItemCount,
        actualOutEncodingRuns: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECFlushMultipleEncodings(
        encodingConverter: TECObjectRef,
        outputBuffer: TextPtr,
        outputBufferLength: ByteCount,
        actualOutputLength: *mut ByteCount,
        outEncodingsBuffer: *mut TextEncodingRun,
        maxOutEncodingRuns: ItemCount,
        actualOutEncodingRuns: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCountWebTextEncodings(
        locale: RegionCode,
        numberEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetWebTextEncodings(
        locale: RegionCode,
        availableEncodings: *mut TextEncoding,
        maxAvailableEncodings: ItemCount,
        actualAvailableEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCountMailTextEncodings(
        locale: RegionCode,
        numberEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetMailTextEncodings(
        locale: RegionCode,
        availableEncodings: *mut TextEncoding,
        maxAvailableEncodings: ItemCount,
        actualAvailableEncodings: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCountAvailableSniffers(numberOfEncodings: *mut ItemCount) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetAvailableSniffers(
        availableSniffers: *mut TextEncoding,
        maxAvailableSniffers: ItemCount,
        actualAvailableSniffers: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCreateSniffer(
        encodingSniffer: *mut TECSnifferObjectRef,
        testEncodings: *const TextEncoding,
        numTextEncodings: ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECSniffTextEncoding(
        encodingSniffer: TECSnifferObjectRef,
        inputBuffer: ConstTextPtr,
        inputBufferLength: ByteCount,
        testEncodings: *mut TextEncoding,
        numTextEncodings: ItemCount,
        numErrsArray: *mut ItemCount,
        maxErrs: ItemCount,
        numFeaturesArray: *mut ItemCount,
        maxFeatures: ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECDisposeSniffer(encodingSniffer: TECSnifferObjectRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECClearSnifferContextInfo(encodingSniffer: TECSnifferObjectRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECSetBasicOptions(
        encodingConverter: TECObjectRef,
        controlFlags: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECCopyTextEncodingInternetNameAndMIB(
        textEncoding: TextEncoding,
        usage: TECInternetNameUsageMask,
        encodingNamePtr: *mut CFStringRef,
        mibEnumPtr: *mut SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TECGetTextEncodingFromInternetNameOrMIB(
        textEncodingPtr: *mut TextEncoding,
        usage: TECInternetNameUsageMask,
        encodingName: CFStringRef,
        mibEnum: SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewUnicodeToTextFallbackUPP(
        userRoutine: UnicodeToTextFallbackProcPtr,
    ) -> UnicodeToTextFallbackUPP;
}
unsafe extern "C" {
    pub fn DisposeUnicodeToTextFallbackUPP(userUPP: UnicodeToTextFallbackUPP);
}
unsafe extern "C" {
    pub fn InvokeUnicodeToTextFallbackUPP(
        iSrcUniStr: *mut UniChar,
        iSrcUniStrLen: ByteCount,
        oSrcConvLen: *mut ByteCount,
        oDestStr: TextPtr,
        iDestStrLen: ByteCount,
        oDestConvLen: *mut ByteCount,
        iInfoPtr: LogicalAddress,
        iUnicodeMappingPtr: ConstUnicodeMappingPtr,
        userUPP: UnicodeToTextFallbackUPP,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateTextToUnicodeInfo(
        iUnicodeMapping: ConstUnicodeMappingPtr,
        oTextToUnicodeInfo: *mut TextToUnicodeInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateTextToUnicodeInfoByEncoding(
        iEncoding: TextEncoding,
        oTextToUnicodeInfo: *mut TextToUnicodeInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateUnicodeToTextInfo(
        iUnicodeMapping: ConstUnicodeMappingPtr,
        oUnicodeToTextInfo: *mut UnicodeToTextInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateUnicodeToTextInfoByEncoding(
        iEncoding: TextEncoding,
        oUnicodeToTextInfo: *mut UnicodeToTextInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateUnicodeToTextRunInfo(
        iNumberOfMappings: ItemCount,
        iUnicodeMappings: *const UnicodeMapping,
        oUnicodeToTextInfo: *mut UnicodeToTextRunInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateUnicodeToTextRunInfoByEncoding(
        iNumberOfEncodings: ItemCount,
        iEncodings: *const TextEncoding,
        oUnicodeToTextInfo: *mut UnicodeToTextRunInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateUnicodeToTextRunInfoByScriptCode(
        iNumberOfScriptCodes: ItemCount,
        iScripts: *const ScriptCode,
        oUnicodeToTextInfo: *mut UnicodeToTextRunInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ChangeTextToUnicodeInfo(
        ioTextToUnicodeInfo: TextToUnicodeInfo,
        iUnicodeMapping: ConstUnicodeMappingPtr,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ChangeUnicodeToTextInfo(
        ioUnicodeToTextInfo: UnicodeToTextInfo,
        iUnicodeMapping: ConstUnicodeMappingPtr,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DisposeTextToUnicodeInfo(ioTextToUnicodeInfo: *mut TextToUnicodeInfo) -> OSStatus;
}
unsafe extern "C" {
    pub fn DisposeUnicodeToTextInfo(ioUnicodeToTextInfo: *mut UnicodeToTextInfo) -> OSStatus;
}
unsafe extern "C" {
    pub fn DisposeUnicodeToTextRunInfo(
        ioUnicodeToTextRunInfo: *mut UnicodeToTextRunInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ConvertFromUnicodeToText(
        iUnicodeToTextInfo: UnicodeToTextInfo,
        iUnicodeLen: ByteCount,
        iUnicodeStr: *const UniChar,
        iControlFlags: OptionBits,
        iOffsetCount: ItemCount,
        iOffsetArray: *const ByteOffset,
        oOffsetCount: *mut ItemCount,
        oOffsetArray: *mut ByteOffset,
        iOutputBufLen: ByteCount,
        oInputRead: *mut ByteCount,
        oOutputLen: *mut ByteCount,
        oOutputStr: LogicalAddress,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ConvertFromUnicodeToTextRun(
        iUnicodeToTextInfo: UnicodeToTextRunInfo,
        iUnicodeLen: ByteCount,
        iUnicodeStr: *const UniChar,
        iControlFlags: OptionBits,
        iOffsetCount: ItemCount,
        iOffsetArray: *const ByteOffset,
        oOffsetCount: *mut ItemCount,
        oOffsetArray: *mut ByteOffset,
        iOutputBufLen: ByteCount,
        oInputRead: *mut ByteCount,
        oOutputLen: *mut ByteCount,
        oOutputStr: LogicalAddress,
        iEncodingRunBufLen: ItemCount,
        oEncodingRunOutLen: *mut ItemCount,
        oEncodingRuns: *mut TextEncodingRun,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ConvertFromUnicodeToScriptCodeRun(
        iUnicodeToTextInfo: UnicodeToTextRunInfo,
        iUnicodeLen: ByteCount,
        iUnicodeStr: *const UniChar,
        iControlFlags: OptionBits,
        iOffsetCount: ItemCount,
        iOffsetArray: *const ByteOffset,
        oOffsetCount: *mut ItemCount,
        oOffsetArray: *mut ByteOffset,
        iOutputBufLen: ByteCount,
        oInputRead: *mut ByteCount,
        oOutputLen: *mut ByteCount,
        oOutputStr: LogicalAddress,
        iScriptRunBufLen: ItemCount,
        oScriptRunOutLen: *mut ItemCount,
        oScriptCodeRuns: *mut ScriptCodeRun,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TruncateForUnicodeToText(
        iUnicodeToTextInfo: ConstUnicodeToTextInfo,
        iSourceLen: ByteCount,
        iSourceStr: *const UniChar,
        iControlFlags: OptionBits,
        iMaxLen: ByteCount,
        oTruncatedLen: *mut ByteCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ConvertFromPStringToUnicode(
        iTextToUnicodeInfo: TextToUnicodeInfo,
        iPascalStr: ConstStr255Param,
        iOutputBufLen: ByteCount,
        oUnicodeLen: *mut ByteCount,
        oUnicodeStr: *mut UniChar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ConvertFromUnicodeToPString(
        iUnicodeToTextInfo: UnicodeToTextInfo,
        iUnicodeLen: ByteCount,
        iUnicodeStr: *const UniChar,
        oPascalStr: *mut ::std::os::raw::c_uchar,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CountUnicodeMappings(
        iFilter: OptionBits,
        iFindMapping: ConstUnicodeMappingPtr,
        oActualCount: *mut ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn QueryUnicodeMappings(
        iFilter: OptionBits,
        iFindMapping: ConstUnicodeMappingPtr,
        iMaxCount: ItemCount,
        oActualCount: *mut ItemCount,
        oReturnedMappings: *mut UnicodeMapping,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SetFallbackUnicodeToText(
        iUnicodeToTextInfo: UnicodeToTextInfo,
        iFallback: UnicodeToTextFallbackUPP,
        iControlFlags: OptionBits,
        iInfoPtr: LogicalAddress,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SetFallbackUnicodeToTextRun(
        iUnicodeToTextRunInfo: UnicodeToTextRunInfo,
        iFallback: UnicodeToTextFallbackUPP,
        iControlFlags: OptionBits,
        iInfoPtr: LogicalAddress,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn ResetTextToUnicodeInfo(ioTextToUnicodeInfo: TextToUnicodeInfo) -> OSStatus;
}
unsafe extern "C" {
    pub fn ResetUnicodeToTextInfo(ioUnicodeToTextInfo: UnicodeToTextInfo) -> OSStatus;
}
unsafe extern "C" {
    pub fn ResetUnicodeToTextRunInfo(ioUnicodeToTextRunInfo: UnicodeToTextRunInfo) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewThreadEntryUPP(userRoutine: ThreadEntryProcPtr) -> ThreadEntryUPP;
}
unsafe extern "C" {
    pub fn NewThreadSchedulerUPP(userRoutine: ThreadSchedulerProcPtr) -> ThreadSchedulerUPP;
}
unsafe extern "C" {
    pub fn NewThreadSwitchUPP(userRoutine: ThreadSwitchProcPtr) -> ThreadSwitchUPP;
}
unsafe extern "C" {
    pub fn NewThreadTerminationUPP(userRoutine: ThreadTerminationProcPtr) -> ThreadTerminationUPP;
}
unsafe extern "C" {
    pub fn NewDebuggerNewThreadUPP(userRoutine: DebuggerNewThreadProcPtr) -> DebuggerNewThreadUPP;
}
unsafe extern "C" {
    pub fn NewDebuggerDisposeThreadUPP(
        userRoutine: DebuggerDisposeThreadProcPtr,
    ) -> DebuggerDisposeThreadUPP;
}
unsafe extern "C" {
    pub fn NewDebuggerThreadSchedulerUPP(
        userRoutine: DebuggerThreadSchedulerProcPtr,
    ) -> DebuggerThreadSchedulerUPP;
}
unsafe extern "C" {
    pub fn DisposeThreadEntryUPP(userUPP: ThreadEntryUPP);
}
unsafe extern "C" {
    pub fn DisposeThreadSchedulerUPP(userUPP: ThreadSchedulerUPP);
}
unsafe extern "C" {
    pub fn DisposeThreadSwitchUPP(userUPP: ThreadSwitchUPP);
}
unsafe extern "C" {
    pub fn DisposeThreadTerminationUPP(userUPP: ThreadTerminationUPP);
}
unsafe extern "C" {
    pub fn DisposeDebuggerNewThreadUPP(userUPP: DebuggerNewThreadUPP);
}
unsafe extern "C" {
    pub fn DisposeDebuggerDisposeThreadUPP(userUPP: DebuggerDisposeThreadUPP);
}
unsafe extern "C" {
    pub fn DisposeDebuggerThreadSchedulerUPP(userUPP: DebuggerThreadSchedulerUPP);
}
unsafe extern "C" {
    pub fn InvokeThreadEntryUPP(
        threadParam: *mut ::std::os::raw::c_void,
        userUPP: ThreadEntryUPP,
    ) -> voidPtr;
}
unsafe extern "C" {
    pub fn InvokeThreadSchedulerUPP(
        schedulerInfo: SchedulerInfoRecPtr,
        userUPP: ThreadSchedulerUPP,
    ) -> ThreadID;
}
unsafe extern "C" {
    pub fn InvokeThreadSwitchUPP(
        threadBeingSwitched: ThreadID,
        switchProcParam: *mut ::std::os::raw::c_void,
        userUPP: ThreadSwitchUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeThreadTerminationUPP(
        threadTerminated: ThreadID,
        terminationProcParam: *mut ::std::os::raw::c_void,
        userUPP: ThreadTerminationUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeDebuggerNewThreadUPP(threadCreated: ThreadID, userUPP: DebuggerNewThreadUPP);
}
unsafe extern "C" {
    pub fn InvokeDebuggerDisposeThreadUPP(
        threadDeleted: ThreadID,
        userUPP: DebuggerDisposeThreadUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeDebuggerThreadSchedulerUPP(
        schedulerInfo: SchedulerInfoRecPtr,
        userUPP: DebuggerThreadSchedulerUPP,
    ) -> ThreadID;
}
unsafe extern "C" {
    pub fn NewThread(
        threadStyle: ThreadStyle,
        threadEntry: ThreadEntryTPP,
        threadParam: *mut ::std::os::raw::c_void,
        stackSize: Size,
        options: ThreadOptions,
        threadResult: *mut *mut ::std::os::raw::c_void,
        threadMade: *mut ThreadID,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SetThreadScheduler(threadScheduler: ThreadSchedulerTPP) -> OSErr;
}
unsafe extern "C" {
    pub fn SetThreadSwitcher(
        thread: ThreadID,
        threadSwitcher: ThreadSwitchTPP,
        switchProcParam: *mut ::std::os::raw::c_void,
        inOrOut: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SetThreadTerminator(
        thread: ThreadID,
        threadTerminator: ThreadTerminationTPP,
        terminationProcParam: *mut ::std::os::raw::c_void,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SetDebuggerNotificationProcs(
        notifyNewThread: DebuggerNewThreadTPP,
        notifyDisposeThread: DebuggerDisposeThreadTPP,
        notifyThreadScheduler: DebuggerThreadSchedulerTPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CreateThreadPool(
        threadStyle: ThreadStyle,
        numToCreate: SInt16,
        stackSize: Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetDefaultThreadStackSize(threadStyle: ThreadStyle, stackSize: *mut Size) -> OSErr;
}
unsafe extern "C" {
    pub fn ThreadCurrentStackSpace(thread: ThreadID, freeStack: *mut ByteCount) -> OSErr;
}
unsafe extern "C" {
    pub fn DisposeThread(
        threadToDump: ThreadID,
        threadResult: *mut ::std::os::raw::c_void,
        recycleThread: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn YieldToThread(suggestedThread: ThreadID) -> OSErr;
}
unsafe extern "C" {
    pub fn YieldToAnyThread() -> OSErr;
}
unsafe extern "C" {
    pub fn GetCurrentThread(currentThreadID: *mut ThreadID) -> OSErr;
}
unsafe extern "C" {
    pub fn GetThreadState(threadToGet: ThreadID, threadState: *mut ThreadState) -> OSErr;
}
unsafe extern "C" {
    pub fn SetThreadState(
        threadToSet: ThreadID,
        newState: ThreadState,
        suggestedThread: ThreadID,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SetThreadStateEndCritical(
        threadToSet: ThreadID,
        newState: ThreadState,
        suggestedThread: ThreadID,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn ThreadBeginCritical() -> OSErr;
}
unsafe extern "C" {
    pub fn ThreadEndCritical() -> OSErr;
}
unsafe extern "C" {
    pub fn GetThreadCurrentTaskRef(threadTRef: *mut ThreadTaskRef) -> OSErr;
}
unsafe extern "C" {
    pub fn GetThreadStateGivenTaskRef(
        threadTRef: ThreadTaskRef,
        threadToGet: ThreadID,
        threadState: *mut ThreadState,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SetThreadReadyGivenTaskRef(threadTRef: ThreadTaskRef, threadToSet: ThreadID) -> OSErr;
}
unsafe extern "C" {
    pub fn FindFolder(
        vRefNum: FSVolumeRefNum,
        folderType: OSType,
        createFolder: Boolean,
        foundVRefNum: *mut FSVolumeRefNum,
        foundDirID: *mut SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn ReleaseFolder(vRefNum: FSVolumeRefNum, folderType: OSType) -> OSErr;
}
unsafe extern "C" {
    pub fn FSFindFolder(
        vRefNum: FSVolumeRefNum,
        folderType: OSType,
        createFolder: Boolean,
        foundRef: *mut FSRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetFolderTypes(
        requestedTypeCount: UInt32,
        totalTypeCount: *mut UInt32,
        theTypes: *mut FolderType,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn RemoveFolderDescriptor(foldType: FolderType) -> OSErr;
}
unsafe extern "C" {
    pub fn GetFolderNameUnicode(
        vRefNum: FSVolumeRefNum,
        foldType: OSType,
        foundVRefNum: *mut FSVolumeRefNum,
        name: *mut HFSUniStr255,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn InvalidateFolderDescriptorCache(vRefNum: FSVolumeRefNum, dirID: SInt32) -> OSErr;
}
unsafe extern "C" {
    pub fn IdentifyFolder(
        vRefNum: FSVolumeRefNum,
        dirID: SInt32,
        foldType: *mut FolderType,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn FSDetermineIfRefIsEnclosedByFolder(
        domainOrVRefNum: FSVolumeRefNum,
        folderType: OSType,
        inRef: *const FSRef,
        outResult: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn DetermineIfPathIsEnclosedByFolder(
        domainOrVRefNum: FSVolumeRefNum,
        folderType: OSType,
        utf8Path: *const UInt8,
        pathIsRealPath: Boolean,
        outResult: *mut Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn NewFolderManagerNotificationUPP(
        userRoutine: FolderManagerNotificationProcPtr,
    ) -> FolderManagerNotificationUPP;
}
unsafe extern "C" {
    pub fn DisposeFolderManagerNotificationUPP(userUPP: FolderManagerNotificationUPP);
}
unsafe extern "C" {
    pub fn InvokeFolderManagerNotificationUPP(
        message: OSType,
        arg: *mut ::std::os::raw::c_void,
        userRefCon: *mut ::std::os::raw::c_void,
        userUPP: FolderManagerNotificationUPP,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn Microseconds(microTickCount: *mut UnsignedWide);
}
unsafe extern "C" {
    pub fn InsTime(tmTaskPtr: QElemPtr);
}
unsafe extern "C" {
    pub fn InsXTime(tmTaskPtr: QElemPtr);
}
unsafe extern "C" {
    pub fn PrimeTime(tmTaskPtr: QElemPtr, count: ::std::os::raw::c_long);
}
unsafe extern "C" {
    pub fn RmvTime(tmTaskPtr: QElemPtr);
}
unsafe extern "C" {
    pub fn InstallTimeTask(tmTaskPtr: QElemPtr) -> OSErr;
}
unsafe extern "C" {
    pub fn InstallXTimeTask(tmTaskPtr: QElemPtr) -> OSErr;
}
unsafe extern "C" {
    pub fn PrimeTimeTask(tmTaskPtr: QElemPtr, count: ::std::os::raw::c_long) -> OSErr;
}
unsafe extern "C" {
    pub fn RemoveTimeTask(tmTaskPtr: QElemPtr) -> OSErr;
}
unsafe extern "C" {
    pub fn NewTimerUPP(userRoutine: TimerProcPtr) -> TimerUPP;
}
unsafe extern "C" {
    pub fn DisposeTimerUPP(userUPP: TimerUPP);
}
unsafe extern "C" {
    pub fn InvokeTimerUPP(tmTaskPtr: TMTaskPtr, userUPP: TimerUPP);
}
unsafe extern "C" {
    pub fn MPGetNextCpuID(owningCoherenceID: MPCoherenceID, cpuID: *mut MPCpuID) -> OSStatus;
}
unsafe extern "C" {
    pub fn MPGetNextTaskID(owningProcessID: MPProcessID, taskID: *mut MPTaskID) -> OSStatus;
}
unsafe extern "C" {
    pub fn LMGetBootDrive() -> SInt16;
}
unsafe extern "C" {
    pub fn LMSetBootDrive(value: SInt16);
}
unsafe extern "C" {
    pub fn LMGetApFontID() -> SInt16;
}
unsafe extern "C" {
    pub fn LMSetApFontID(value: SInt16);
}
unsafe extern "C" {
    pub fn LMGetSysMap() -> SInt16;
}
unsafe extern "C" {
    pub fn LMSetSysMap(value: SInt16);
}
unsafe extern "C" {
    pub fn LMGetResLoad() -> UInt8;
}
unsafe extern "C" {
    pub fn LMSetResLoad(value: UInt8);
}
unsafe extern "C" {
    pub fn LMGetResErr() -> SInt16;
}
unsafe extern "C" {
    pub fn LMSetResErr(value: SInt16);
}
unsafe extern "C" {
    pub fn LMGetTmpResLoad() -> UInt8;
}
unsafe extern "C" {
    pub fn LMSetTmpResLoad(value: UInt8);
}
unsafe extern "C" {
    pub fn LMGetIntlSpec() -> Ptr;
}
unsafe extern "C" {
    pub fn LMSetIntlSpec(value: Ptr);
}
unsafe extern "C" {
    pub fn LMSetSysFontFam(value: SInt16);
}
unsafe extern "C" {
    pub fn LMGetSysFontSize() -> SInt16;
}
unsafe extern "C" {
    pub fn LMSetSysFontSize(value: SInt16);
}
unsafe extern "C" {
    pub fn NewAECoerceDescUPP(userRoutine: AECoerceDescProcPtr) -> AECoerceDescUPP;
}
unsafe extern "C" {
    pub fn NewAECoercePtrUPP(userRoutine: AECoercePtrProcPtr) -> AECoercePtrUPP;
}
unsafe extern "C" {
    pub fn DisposeAECoerceDescUPP(userUPP: AECoerceDescUPP);
}
unsafe extern "C" {
    pub fn DisposeAECoercePtrUPP(userUPP: AECoercePtrUPP);
}
unsafe extern "C" {
    pub fn InvokeAECoerceDescUPP(
        fromDesc: *const AEDesc,
        toType: DescType,
        handlerRefcon: SRefCon,
        toDesc: *mut AEDesc,
        userUPP: AECoerceDescUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeAECoercePtrUPP(
        typeCode: DescType,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: Size,
        toType: DescType,
        handlerRefcon: SRefCon,
        result: *mut AEDesc,
        userUPP: AECoercePtrUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEInstallCoercionHandler(
        fromType: DescType,
        toType: DescType,
        handler: AECoercionHandlerUPP,
        handlerRefcon: SRefCon,
        fromTypeIsDesc: Boolean,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AERemoveCoercionHandler(
        fromType: DescType,
        toType: DescType,
        handler: AECoercionHandlerUPP,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetCoercionHandler(
        fromType: DescType,
        toType: DescType,
        handler: *mut AECoercionHandlerUPP,
        handlerRefcon: *mut SRefCon,
        fromTypeIsDesc: *mut Boolean,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AECoercePtr(
        typeCode: DescType,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: Size,
        toType: DescType,
        result: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AECoerceDesc(theAEDesc: *const AEDesc, toType: DescType, result: *mut AEDesc) -> OSErr;
}
unsafe extern "C" {
    pub fn AEInitializeDesc(desc: *mut AEDesc);
}
unsafe extern "C" {
    pub fn AECreateDesc(
        typeCode: DescType,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: Size,
        result: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEDisposeDesc(theAEDesc: *mut AEDesc) -> OSErr;
}
unsafe extern "C" {
    pub fn AEDuplicateDesc(theAEDesc: *const AEDesc, result: *mut AEDesc) -> OSErr;
}
unsafe extern "C" {
    pub fn AECreateDescFromExternalPtr(
        descriptorType: OSType,
        dataPtr: *const ::std::os::raw::c_void,
        dataLength: Size,
        disposeCallback: AEDisposeExternalUPP,
        disposeRefcon: SRefCon,
        theDesc: *mut AEDesc,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AECompareDesc(
        desc1: *const AEDesc,
        desc2: *const AEDesc,
        resultP: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AECreateList(
        factoringPtr: *const ::std::os::raw::c_void,
        factoredSize: Size,
        isRecord: Boolean,
        resultList: *mut AEDescList,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AECountItems(
        theAEDescList: *const AEDescList,
        theCount: *mut ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEPutPtr(
        theAEDescList: *mut AEDescList,
        index: ::std::os::raw::c_long,
        typeCode: DescType,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEPutDesc(
        theAEDescList: *mut AEDescList,
        index: ::std::os::raw::c_long,
        theAEDesc: *const AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetNthPtr(
        theAEDescList: *const AEDescList,
        index: ::std::os::raw::c_long,
        desiredType: DescType,
        theAEKeyword: *mut AEKeyword,
        typeCode: *mut DescType,
        dataPtr: *mut ::std::os::raw::c_void,
        maximumSize: Size,
        actualSize: *mut Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetNthDesc(
        theAEDescList: *const AEDescList,
        index: ::std::os::raw::c_long,
        desiredType: DescType,
        theAEKeyword: *mut AEKeyword,
        result: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AESizeOfNthItem(
        theAEDescList: *const AEDescList,
        index: ::std::os::raw::c_long,
        typeCode: *mut DescType,
        dataSize: *mut Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetArray(
        theAEDescList: *const AEDescList,
        arrayType: AEArrayType,
        arrayPtr: AEArrayDataPointer,
        maximumSize: Size,
        itemType: *mut DescType,
        itemSize: *mut Size,
        itemCount: *mut ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEPutArray(
        theAEDescList: *mut AEDescList,
        arrayType: AEArrayType,
        arrayPtr: *const AEArrayData,
        itemType: DescType,
        itemSize: Size,
        itemCount: ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEDeleteItem(theAEDescList: *mut AEDescList, index: ::std::os::raw::c_long) -> OSErr;
}
unsafe extern "C" {
    pub fn AECheckIsRecord(theDesc: *const AEDesc) -> Boolean;
}
unsafe extern "C" {
    pub fn AECreateAppleEvent(
        theAEEventClass: AEEventClass,
        theAEEventID: AEEventID,
        target: *const AEAddressDesc,
        returnID: AEReturnID,
        transactionID: AETransactionID,
        result: *mut AppleEvent,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEPutParamPtr(
        theAppleEvent: *mut AppleEvent,
        theAEKeyword: AEKeyword,
        typeCode: DescType,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEPutParamDesc(
        theAppleEvent: *mut AppleEvent,
        theAEKeyword: AEKeyword,
        theAEDesc: *const AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetParamPtr(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        desiredType: DescType,
        actualType: *mut DescType,
        dataPtr: *mut ::std::os::raw::c_void,
        maximumSize: Size,
        actualSize: *mut Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetParamDesc(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        desiredType: DescType,
        result: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AESizeOfParam(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        typeCode: *mut DescType,
        dataSize: *mut Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEDeleteParam(theAppleEvent: *mut AppleEvent, theAEKeyword: AEKeyword) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetAttributePtr(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        desiredType: DescType,
        typeCode: *mut DescType,
        dataPtr: *mut ::std::os::raw::c_void,
        maximumSize: Size,
        actualSize: *mut Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetAttributeDesc(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        desiredType: DescType,
        result: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AESizeOfAttribute(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        typeCode: *mut DescType,
        dataSize: *mut Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEPutAttributePtr(
        theAppleEvent: *mut AppleEvent,
        theAEKeyword: AEKeyword,
        typeCode: DescType,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEPutAttributeDesc(
        theAppleEvent: *mut AppleEvent,
        theAEKeyword: AEKeyword,
        theAEDesc: *const AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AESizeOfFlattenedDesc(theAEDesc: *const AEDesc) -> Size;
}
unsafe extern "C" {
    pub fn AEFlattenDesc(
        theAEDesc: *const AEDesc,
        buffer: Ptr,
        bufferSize: Size,
        actualSize: *mut Size,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEUnflattenDesc(buffer: *const ::std::os::raw::c_void, result: *mut AEDesc) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEUnflattenDescFromBytes(
        buffer: *const ::std::os::raw::c_void,
        bufferLen: usize,
        result: *mut AEDesc,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEGetDescData(
        theAEDesc: *const AEDesc,
        dataPtr: *mut ::std::os::raw::c_void,
        maximumSize: Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetDescDataSize(theAEDesc: *const AEDesc) -> Size;
}
unsafe extern "C" {
    pub fn AEReplaceDescData(
        typeCode: DescType,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: Size,
        theAEDesc: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetDescDataRange(
        dataDesc: *const AEDesc,
        buffer: *mut ::std::os::raw::c_void,
        offset: Size,
        length: Size,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewAEDisposeExternalUPP(userRoutine: AEDisposeExternalProcPtr) -> AEDisposeExternalUPP;
}
unsafe extern "C" {
    pub fn NewAEEventHandlerUPP(userRoutine: AEEventHandlerProcPtr) -> AEEventHandlerUPP;
}
unsafe extern "C" {
    pub fn DisposeAEDisposeExternalUPP(userUPP: AEDisposeExternalUPP);
}
unsafe extern "C" {
    pub fn DisposeAEEventHandlerUPP(userUPP: AEEventHandlerUPP);
}
unsafe extern "C" {
    pub fn InvokeAEDisposeExternalUPP(
        dataPtr: *const ::std::os::raw::c_void,
        dataLength: Size,
        refcon: SRefCon,
        userUPP: AEDisposeExternalUPP,
    );
}
unsafe extern "C" {
    pub fn InvokeAEEventHandlerUPP(
        theAppleEvent: *const AppleEvent,
        reply: *mut AppleEvent,
        handlerRefcon: SRefCon,
        userUPP: AEEventHandlerUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEInstallEventHandler(
        theAEEventClass: AEEventClass,
        theAEEventID: AEEventID,
        handler: AEEventHandlerUPP,
        handlerRefcon: SRefCon,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AERemoveEventHandler(
        theAEEventClass: AEEventClass,
        theAEEventID: AEEventID,
        handler: AEEventHandlerUPP,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetEventHandler(
        theAEEventClass: AEEventClass,
        theAEEventID: AEEventID,
        handler: *mut AEEventHandlerUPP,
        handlerRefcon: *mut SRefCon,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEInstallSpecialHandler(
        functionClass: AEKeyword,
        handler: AEEventHandlerUPP,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AERemoveSpecialHandler(
        functionClass: AEKeyword,
        handler: AEEventHandlerUPP,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetSpecialHandler(
        functionClass: AEKeyword,
        handler: *mut AEEventHandlerUPP,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEManagerInfo(keyWord: AEKeyword, result: *mut ::std::os::raw::c_long) -> OSErr;
}
unsafe extern "C" {
    pub static kAERemoteProcessURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kAERemoteProcessNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kAERemoteProcessUserIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kAERemoteProcessProcessIDKey: CFStringRef;
}
unsafe extern "C" {
    pub fn AECreateRemoteProcessResolver(
        allocator: CFAllocatorRef,
        url: CFURLRef,
    ) -> AERemoteProcessResolverRef;
}
unsafe extern "C" {
    pub fn AEDisposeRemoteProcessResolver(ref_: AERemoteProcessResolverRef);
}
unsafe extern "C" {
    pub fn AERemoteProcessResolverGetProcesses(
        ref_: AERemoteProcessResolverRef,
        outError: *mut CFStreamError,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn AEDeterminePermissionToAutomateTarget(
        target: *const AEAddressDesc,
        theAEEventClass: AEEventClass,
        theAEEventID: AEEventID,
        askUserIfNeeded: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateOffsetDescriptor(
        theOffset: ::std::os::raw::c_long,
        theDescriptor: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CreateCompDescriptor(
        comparisonOperator: DescType,
        operand1: *mut AEDesc,
        operand2: *mut AEDesc,
        disposeInputs: Boolean,
        theDescriptor: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CreateLogicalDescriptor(
        theLogicalTerms: *mut AEDescList,
        theLogicOperator: DescType,
        disposeInputs: Boolean,
        theDescriptor: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CreateObjSpecifier(
        desiredClass: DescType,
        theContainer: *mut AEDesc,
        keyForm: DescType,
        keyData: *mut AEDesc,
        disposeInputs: Boolean,
        objSpecifier: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn CreateRangeDescriptor(
        rangeStart: *mut AEDesc,
        rangeStop: *mut AEDesc,
        disposeInputs: Boolean,
        theDescriptor: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn NewOSLAccessorUPP(userRoutine: OSLAccessorProcPtr) -> OSLAccessorUPP;
}
unsafe extern "C" {
    pub fn NewOSLCompareUPP(userRoutine: OSLCompareProcPtr) -> OSLCompareUPP;
}
unsafe extern "C" {
    pub fn NewOSLCountUPP(userRoutine: OSLCountProcPtr) -> OSLCountUPP;
}
unsafe extern "C" {
    pub fn NewOSLDisposeTokenUPP(userRoutine: OSLDisposeTokenProcPtr) -> OSLDisposeTokenUPP;
}
unsafe extern "C" {
    pub fn NewOSLGetMarkTokenUPP(userRoutine: OSLGetMarkTokenProcPtr) -> OSLGetMarkTokenUPP;
}
unsafe extern "C" {
    pub fn NewOSLGetErrDescUPP(userRoutine: OSLGetErrDescProcPtr) -> OSLGetErrDescUPP;
}
unsafe extern "C" {
    pub fn NewOSLMarkUPP(userRoutine: OSLMarkProcPtr) -> OSLMarkUPP;
}
unsafe extern "C" {
    pub fn NewOSLAdjustMarksUPP(userRoutine: OSLAdjustMarksProcPtr) -> OSLAdjustMarksUPP;
}
unsafe extern "C" {
    pub fn DisposeOSLAccessorUPP(userUPP: OSLAccessorUPP);
}
unsafe extern "C" {
    pub fn DisposeOSLCompareUPP(userUPP: OSLCompareUPP);
}
unsafe extern "C" {
    pub fn DisposeOSLCountUPP(userUPP: OSLCountUPP);
}
unsafe extern "C" {
    pub fn DisposeOSLDisposeTokenUPP(userUPP: OSLDisposeTokenUPP);
}
unsafe extern "C" {
    pub fn DisposeOSLGetMarkTokenUPP(userUPP: OSLGetMarkTokenUPP);
}
unsafe extern "C" {
    pub fn DisposeOSLGetErrDescUPP(userUPP: OSLGetErrDescUPP);
}
unsafe extern "C" {
    pub fn DisposeOSLMarkUPP(userUPP: OSLMarkUPP);
}
unsafe extern "C" {
    pub fn DisposeOSLAdjustMarksUPP(userUPP: OSLAdjustMarksUPP);
}
unsafe extern "C" {
    pub fn InvokeOSLAccessorUPP(
        desiredClass: DescType,
        container: *const AEDesc,
        containerClass: DescType,
        form: DescType,
        selectionData: *const AEDesc,
        value: *mut AEDesc,
        accessorRefcon: SRefCon,
        userUPP: OSLAccessorUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeOSLCompareUPP(
        oper: DescType,
        obj1: *const AEDesc,
        obj2: *const AEDesc,
        result: *mut Boolean,
        userUPP: OSLCompareUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeOSLCountUPP(
        desiredType: DescType,
        containerClass: DescType,
        container: *const AEDesc,
        result: *mut ::std::os::raw::c_long,
        userUPP: OSLCountUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeOSLDisposeTokenUPP(
        unneededToken: *mut AEDesc,
        userUPP: OSLDisposeTokenUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeOSLGetMarkTokenUPP(
        dContainerToken: *const AEDesc,
        containerClass: DescType,
        result: *mut AEDesc,
        userUPP: OSLGetMarkTokenUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeOSLGetErrDescUPP(appDescPtr: *mut *mut AEDesc, userUPP: OSLGetErrDescUPP)
        -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeOSLMarkUPP(
        dToken: *const AEDesc,
        markToken: *const AEDesc,
        index: ::std::os::raw::c_long,
        userUPP: OSLMarkUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn InvokeOSLAdjustMarksUPP(
        newStart: ::std::os::raw::c_long,
        newStop: ::std::os::raw::c_long,
        markToken: *const AEDesc,
        userUPP: OSLAdjustMarksUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEObjectInit() -> OSErr;
}
unsafe extern "C" {
    pub fn AESetObjectCallbacks(
        myCompareProc: OSLCompareUPP,
        myCountProc: OSLCountUPP,
        myDisposeTokenProc: OSLDisposeTokenUPP,
        myGetMarkTokenProc: OSLGetMarkTokenUPP,
        myMarkProc: OSLMarkUPP,
        myAdjustMarksProc: OSLAdjustMarksUPP,
        myGetErrDescProcPtr: OSLGetErrDescUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEResolve(
        objectSpecifier: *const AEDesc,
        callbackFlags: ::std::os::raw::c_short,
        theToken: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEInstallObjectAccessor(
        desiredClass: DescType,
        containerType: DescType,
        theAccessor: OSLAccessorUPP,
        accessorRefcon: SRefCon,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AERemoveObjectAccessor(
        desiredClass: DescType,
        containerType: DescType,
        theAccessor: OSLAccessorUPP,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetObjectAccessor(
        desiredClass: DescType,
        containerType: DescType,
        accessor: *mut OSLAccessorUPP,
        accessorRefcon: *mut SRefCon,
        isSysHandler: Boolean,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEDisposeToken(theToken: *mut AEDesc) -> OSErr;
}
unsafe extern "C" {
    pub fn AECallObjectAccessor(
        desiredClass: DescType,
        containerToken: *const AEDesc,
        containerClass: DescType,
        keyForm: DescType,
        keyData: *const AEDesc,
        token: *mut AEDesc,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEBuildDesc(
        dst: *mut AEDesc,
        error: *mut AEBuildError,
        src: *const ::std::os::raw::c_char,
        ...
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn vAEBuildDesc(
        dst: *mut AEDesc,
        error: *mut AEBuildError,
        src: *const ::std::os::raw::c_char,
        args: va_list,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEBuildParameters(
        event: *mut AppleEvent,
        error: *mut AEBuildError,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn vAEBuildParameters(
        event: *mut AppleEvent,
        error: *mut AEBuildError,
        format: *const ::std::os::raw::c_char,
        args: va_list,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEBuildAppleEvent(
        theClass: AEEventClass,
        theID: AEEventID,
        addressType: DescType,
        addressData: *const ::std::os::raw::c_void,
        addressLength: Size,
        returnID: SInt16,
        transactionID: SInt32,
        result: *mut AppleEvent,
        error: *mut AEBuildError,
        paramsFmt: *const ::std::os::raw::c_char,
        ...
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn vAEBuildAppleEvent(
        theClass: AEEventClass,
        theID: AEEventID,
        addressType: DescType,
        addressData: *const ::std::os::raw::c_void,
        addressLength: Size,
        returnID: SInt16,
        transactionID: SInt32,
        resultEvt: *mut AppleEvent,
        error: *mut AEBuildError,
        paramsFmt: *const ::std::os::raw::c_char,
        args: va_list,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEPrintDescToHandle(desc: *const AEDesc, result: *mut Handle) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamOpen() -> AEStreamRef;
}
unsafe extern "C" {
    pub fn AEStreamClose(ref_: AEStreamRef, desc: *mut AEDesc) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamOpenDesc(ref_: AEStreamRef, newType: DescType) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamWriteData(
        ref_: AEStreamRef,
        data: *const ::std::os::raw::c_void,
        length: Size,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamCloseDesc(ref_: AEStreamRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamWriteDesc(
        ref_: AEStreamRef,
        newType: DescType,
        data: *const ::std::os::raw::c_void,
        length: Size,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamWriteAEDesc(ref_: AEStreamRef, desc: *const AEDesc) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamOpenList(ref_: AEStreamRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamCloseList(ref_: AEStreamRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamOpenRecord(ref_: AEStreamRef, newType: DescType) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamSetRecordType(ref_: AEStreamRef, newType: DescType) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamCloseRecord(ref_: AEStreamRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamWriteKeyDesc(
        ref_: AEStreamRef,
        key: AEKeyword,
        newType: DescType,
        data: *const ::std::os::raw::c_void,
        length: Size,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamOpenKeyDesc(ref_: AEStreamRef, key: AEKeyword, newType: DescType) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamWriteKey(ref_: AEStreamRef, key: AEKeyword) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEStreamCreateEvent(
        clazz: AEEventClass,
        id: AEEventID,
        targetType: DescType,
        targetData: *const ::std::os::raw::c_void,
        targetLength: Size,
        returnID: SInt16,
        transactionID: SInt32,
    ) -> AEStreamRef;
}
unsafe extern "C" {
    pub fn AEStreamOpenEvent(event: *mut AppleEvent) -> AEStreamRef;
}
unsafe extern "C" {
    pub fn AEStreamOptionalParam(ref_: AEStreamRef, key: AEKeyword) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEGetRegisteredMachPort() -> mach_port_t;
}
unsafe extern "C" {
    pub fn AEDecodeMessage(
        header: *mut mach_msg_header_t,
        event: *mut AppleEvent,
        reply: *mut AppleEvent,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEProcessMessage(header: *mut mach_msg_header_t) -> OSStatus;
}
unsafe extern "C" {
    pub fn AESendMessage(
        event: *const AppleEvent,
        reply: *mut AppleEvent,
        sendMode: AESendMode,
        timeOutInTicks: ::std::os::raw::c_long,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DCSGetTermRangeInString(
        dictionary: DCSDictionaryRef,
        textString: CFStringRef,
        offset: CFIndex,
    ) -> CFRange;
}
unsafe extern "C" {
    pub fn DCSCopyTextDefinition(
        dictionary: DCSDictionaryRef,
        textString: CFStringRef,
        range: CFRange,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub static kCSIdentityErrorDomain: CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityAuthorityGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CSGetDefaultIdentityAuthority() -> CSIdentityAuthorityRef;
}
unsafe extern "C" {
    pub fn CSGetLocalIdentityAuthority() -> CSIdentityAuthorityRef;
}
unsafe extern "C" {
    pub fn CSGetManagedIdentityAuthority() -> CSIdentityAuthorityRef;
}
unsafe extern "C" {
    pub fn CSIdentityAuthorityCopyLocalizedName(authority: CSIdentityAuthorityRef) -> CFStringRef;
}
unsafe extern "C" {
    pub static kCSIdentityGeneratePosixName: CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CSIdentityCreate(
        allocator: CFAllocatorRef,
        identityClass: CSIdentityClass,
        fullName: CFStringRef,
        posixName: CFStringRef,
        flags: CSIdentityFlags,
        authority: CSIdentityAuthorityRef,
    ) -> CSIdentityRef;
}
unsafe extern "C" {
    pub fn CSIdentityCreateCopy(
        allocator: CFAllocatorRef,
        identity: CSIdentityRef,
    ) -> CSIdentityRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetClass(identity: CSIdentityRef) -> CSIdentityClass;
}
unsafe extern "C" {
    pub fn CSIdentityGetAuthority(identity: CSIdentityRef) -> CSIdentityAuthorityRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetUUID(identity: CSIdentityRef) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetFullName(identity: CSIdentityRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetPosixID(identity: CSIdentityRef) -> id_t;
}
unsafe extern "C" {
    pub fn CSIdentityGetPosixName(identity: CSIdentityRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetEmailAddress(identity: CSIdentityRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetImageURL(identity: CSIdentityRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetImageData(identity: CSIdentityRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetImageDataType(identity: CSIdentityRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetAliases(identity: CSIdentityRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CSIdentityIsMemberOfGroup(identity: CSIdentityRef, group: CSIdentityRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityIsHidden(identity: CSIdentityRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityCreatePersistentReference(
        allocator: CFAllocatorRef,
        identity: CSIdentityRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CSIdentityIsEnabled(user: CSIdentityRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityAuthenticateUsingPassword(
        user: CSIdentityRef,
        password: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityGetCertificate(user: CSIdentityRef) -> SecCertificateRef;
}
unsafe extern "C" {
    pub fn CSIdentityCreateGroupMembershipQuery(
        allocator: CFAllocatorRef,
        group: CSIdentityRef,
    ) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentitySetFullName(identity: CSIdentityRef, fullName: CFStringRef);
}
unsafe extern "C" {
    pub fn CSIdentitySetEmailAddress(identity: CSIdentityRef, emailAddress: CFStringRef);
}
unsafe extern "C" {
    pub fn CSIdentitySetImageURL(identity: CSIdentityRef, url: CFURLRef);
}
unsafe extern "C" {
    pub fn CSIdentitySetImageData(
        identity: CSIdentityRef,
        imageData: CFDataRef,
        imageDataType: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CSIdentityAddAlias(identity: CSIdentityRef, alias: CFStringRef);
}
unsafe extern "C" {
    pub fn CSIdentityRemoveAlias(identity: CSIdentityRef, alias: CFStringRef);
}
unsafe extern "C" {
    pub fn CSIdentityAddMember(group: CSIdentityRef, member: CSIdentityRef);
}
unsafe extern "C" {
    pub fn CSIdentityRemoveMember(group: CSIdentityRef, member: CSIdentityRef);
}
unsafe extern "C" {
    pub fn CSIdentitySetIsEnabled(user: CSIdentityRef, isEnabled: Boolean);
}
unsafe extern "C" {
    pub fn CSIdentitySetPassword(user: CSIdentityRef, password: CFStringRef);
}
unsafe extern "C" {
    pub fn CSIdentitySetCertificate(user: CSIdentityRef, certificate: SecCertificateRef);
}
unsafe extern "C" {
    pub fn CSIdentityDelete(identity: CSIdentityRef);
}
unsafe extern "C" {
    pub fn CSIdentityCommit(
        identity: CSIdentityRef,
        authorization: AuthorizationRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityCommitAsynchronously(
        identity: CSIdentityRef,
        clientContext: *const CSIdentityClientContext,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
        authorization: AuthorizationRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityIsCommitting(identity: CSIdentityRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityRemoveClient(identity: CSIdentityRef);
}
unsafe extern "C" {
    pub fn CSIdentityQueryGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCreate(
        allocator: CFAllocatorRef,
        identityClass: CSIdentityClass,
        authority: CSIdentityAuthorityRef,
    ) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCreateForName(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        comparisonMethod: CSIdentityQueryStringComparisonMethod,
        identityClass: CSIdentityClass,
        authority: CSIdentityAuthorityRef,
    ) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCreateForUUID(
        allocator: CFAllocatorRef,
        uuid: CFUUIDRef,
        authority: CSIdentityAuthorityRef,
    ) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCreateForPosixID(
        allocator: CFAllocatorRef,
        posixID: id_t,
        identityClass: CSIdentityClass,
        authority: CSIdentityAuthorityRef,
    ) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCreateForPersistentReference(
        allocator: CFAllocatorRef,
        referenceData: CFDataRef,
    ) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCreateForCurrentUser(allocator: CFAllocatorRef) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCopyResults(query: CSIdentityQueryRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryExecute(
        query: CSIdentityQueryRef,
        flags: CSIdentityQueryFlags,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityQueryExecuteAsynchronously(
        query: CSIdentityQueryRef,
        flags: CSIdentityQueryFlags,
        clientContext: *const CSIdentityQueryClientContext,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityQueryStop(query: CSIdentityQueryRef);
}
unsafe extern "C" {
    pub fn NewSleepQUPP(userRoutine: SleepQProcPtr) -> SleepQUPP;
}
unsafe extern "C" {
    pub fn DisposeSleepQUPP(userUPP: SleepQUPP);
}
unsafe extern "C" {
    pub fn InvokeSleepQUPP(
        message: ::std::os::raw::c_long,
        qRecPtr: SleepQRecPtr,
        userUPP: SleepQUPP,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetCPUSpeed() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn SleepQInstall(qRecPtr: SleepQRecPtr);
}
unsafe extern "C" {
    pub fn SleepQRemove(qRecPtr: SleepQRecPtr);
}
unsafe extern "C" {
    pub fn MaximumProcessorSpeed() -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn MinimumProcessorSpeed() -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn CurrentProcessorSpeed() -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn BatteryCount() -> ::std::os::raw::c_short;
}
unsafe extern "C" {
    pub fn UpdateSystemActivity(activity: UInt8) -> OSErr;
}
unsafe extern "C" {
    pub fn KCGetKeychainManagerVersion(returnVers: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCSetInteractionAllowed(state: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCIsInteractionAllowed() -> Boolean;
}
unsafe extern "C" {
    pub fn KCMakeKCRefFromFSRef(keychainFSRef: *mut FSRef, keychain: *mut KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCMakeKCRefFromAlias(keychainAlias: AliasHandle, keychain: *mut KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCMakeAliasFromKCRef(keychain: KCRef, keychainAlias: *mut AliasHandle) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCReleaseKeychain(keychain: *mut KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCGetDefaultKeychain(keychain: *mut KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCSetDefaultKeychain(keychain: KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCGetStatus(keychain: KCRef, keychainStatus: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCGetKeychain(item: KCItemRef, keychain: *mut KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCGetKeychainName(keychain: KCRef, keychainName: StringPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCCountKeychains() -> UInt16;
}
unsafe extern "C" {
    pub fn KCGetIndKeychain(index: UInt16, keychain: *mut KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewKCCallbackUPP(userRoutine: KCCallbackProcPtr) -> KCCallbackUPP;
}
unsafe extern "C" {
    pub fn DisposeKCCallbackUPP(userUPP: KCCallbackUPP);
}
unsafe extern "C" {
    pub fn InvokeKCCallbackUPP(
        keychainEvent: KCEvent,
        info: *mut KCCallbackInfo,
        userContext: *mut ::std::os::raw::c_void,
        userUPP: KCCallbackUPP,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCFindInternetPassword(
        serverName: ConstStringPtr,
        securityDomain: ConstStringPtr,
        accountName: ConstStringPtr,
        port: UInt16,
        protocol: OSType,
        authType: OSType,
        maxLength: UInt32,
        passwordData: *mut ::std::os::raw::c_void,
        actualLength: *mut UInt32,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCFindInternetPasswordWithPath(
        serverName: ConstStringPtr,
        securityDomain: ConstStringPtr,
        accountName: ConstStringPtr,
        path: ConstStringPtr,
        port: UInt16,
        protocol: OSType,
        authType: OSType,
        maxLength: UInt32,
        passwordData: *mut ::std::os::raw::c_void,
        actualLength: *mut UInt32,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCFindGenericPassword(
        serviceName: ConstStringPtr,
        accountName: ConstStringPtr,
        maxLength: UInt32,
        passwordData: *mut ::std::os::raw::c_void,
        actualLength: *mut UInt32,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCAddCallback(
        callbackProc: KCCallbackUPP,
        eventMask: KCEventMask,
        userContext: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCRemoveCallback(callbackProc: KCCallbackUPP) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCNewItem(
        itemClass: KCItemClass,
        itemCreator: OSType,
        length: UInt32,
        data: *const ::std::os::raw::c_void,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCSetAttribute(item: KCItemRef, attr: *mut KCAttribute) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCGetAttribute(
        item: KCItemRef,
        attr: *mut KCAttribute,
        actualLength: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCSetData(
        item: KCItemRef,
        length: UInt32,
        data: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCUpdateItem(item: KCItemRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCReleaseItem(item: *mut KCItemRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCCopyItem(item: KCItemRef, destKeychain: KCRef, copy: *mut KCItemRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCFindFirstItem(
        keychain: KCRef,
        attrList: *const KCAttributeList,
        search: *mut KCSearchRef,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCFindNextItem(search: KCSearchRef, item: *mut KCItemRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCReleaseSearch(search: *mut KCSearchRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCDeleteItem(item: KCItemRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCGetData(
        item: KCItemRef,
        maxLength: UInt32,
        data: *mut ::std::os::raw::c_void,
        actualLength: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCLock(keychain: KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn kcgetkeychainname(
        keychain: KCRef,
        keychainName: *mut ::std::os::raw::c_char,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn kcfindinternetpassword(
        serverName: *const ::std::os::raw::c_char,
        securityDomain: *const ::std::os::raw::c_char,
        accountName: *const ::std::os::raw::c_char,
        port: UInt16,
        protocol: OSType,
        authType: OSType,
        maxLength: UInt32,
        passwordData: *mut ::std::os::raw::c_void,
        actualLength: *mut UInt32,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn kcfindinternetpasswordwithpath(
        serverName: *const ::std::os::raw::c_char,
        securityDomain: *const ::std::os::raw::c_char,
        accountName: *const ::std::os::raw::c_char,
        path: *const ::std::os::raw::c_char,
        port: UInt16,
        protocol: OSType,
        authType: OSType,
        maxLength: UInt32,
        passwordData: *mut ::std::os::raw::c_void,
        actualLength: *mut UInt32,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn kcfindgenericpassword(
        serviceName: *const ::std::os::raw::c_char,
        accountName: *const ::std::os::raw::c_char,
        maxLength: UInt32,
        passwordData: *mut ::std::os::raw::c_void,
        actualLength: *mut UInt32,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static mut kWSXMLRPCProtocol: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSSOAP1999Protocol: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSSOAP2001Protocol: CFStringRef;
}
unsafe extern "C" {
    pub fn WSGetWSTypeIDFromCFType(ref_: CFTypeRef) -> WSTypeID;
}
unsafe extern "C" {
    pub fn WSGetCFTypeIDFromWSTypeID(typeID: WSTypeID) -> CFTypeID;
}
unsafe extern "C" {
    pub static mut kWSMethodInvocationResult: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSFaultString: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSFaultCode: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSFaultExtra: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSNetworkStreamFaultString: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSStreamErrorMessage: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSStreamErrorDomain: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSStreamErrorError: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSHTTPMessage: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSHTTPResponseMessage: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSHTTPExtraHeaders: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSHTTPVersion: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSHTTPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSHTTPFollowsRedirects: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSDebugOutgoingHeaders: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSDebugOutgoingBody: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSDebugIncomingHeaders: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSDebugIncomingBody: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSSOAPBodyEncodingStyle: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSSOAPMethodNamespaceURI: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSSOAPStyleDoc: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSSOAPStyleRPC: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSSOAPMessageHeaders: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSRecordParameterOrder: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSRecordNamespaceURI: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSRecordType: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSMethodInvocationResultParameterName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kWSMethodInvocationTimeoutValue: CFStringRef;
}
unsafe extern "C" {
    pub fn WSMethodInvocationGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn WSMethodInvocationCreate(
        url: CFURLRef,
        methodName: CFStringRef,
        protocol: CFStringRef,
    ) -> WSMethodInvocationRef;
}
unsafe extern "C" {
    pub fn WSMethodInvocationCreateFromSerialization(contract: CFDataRef) -> WSMethodInvocationRef;
}
unsafe extern "C" {
    pub fn WSMethodInvocationCopySerialization(invocation: WSMethodInvocationRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn WSMethodInvocationSetParameters(
        invocation: WSMethodInvocationRef,
        parameters: CFDictionaryRef,
        parameterOrder: CFArrayRef,
    );
}
unsafe extern "C" {
    pub fn WSMethodInvocationCopyParameters(
        invocation: WSMethodInvocationRef,
        parameterOrder: *mut CFArrayRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn WSMethodInvocationSetProperty(
        invocation: WSMethodInvocationRef,
        propertyName: CFStringRef,
        propertyValue: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn WSMethodInvocationCopyProperty(
        invocation: WSMethodInvocationRef,
        propertyName: CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn WSMethodInvocationInvoke(invocation: WSMethodInvocationRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn WSMethodInvocationScheduleWithRunLoop(
        invocation: WSMethodInvocationRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn WSMethodInvocationUnscheduleFromRunLoop(
        invocation: WSMethodInvocationRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn WSMethodResultIsFault(methodResult: CFDictionaryRef) -> Boolean;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerCreate(
        allocator: CFAllocatorRef,
        protocol: CFStringRef,
    ) -> WSProtocolHandlerRef;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerCopyRequestDictionary(
        ref_: WSProtocolHandlerRef,
        data: CFDataRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerCopyReplyDictionary(
        ref_: WSProtocolHandlerRef,
        methodName: CFStringRef,
        data: CFDataRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerCopyReplyDocument(
        ref_: WSProtocolHandlerRef,
        methodContext: CFDictionaryRef,
        resultValue: CFTypeRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerCopyFaultDocument(
        ref_: WSProtocolHandlerRef,
        methodContext: CFDictionaryRef,
        faultDict: CFDictionaryRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerCopyRequestDocument(
        ref_: WSProtocolHandlerRef,
        methodName: CFStringRef,
        methodParams: CFDictionaryRef,
        methodParamOrder: CFArrayRef,
        methodExtras: CFDictionaryRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerCopyProperty(
        ref_: WSProtocolHandlerRef,
        propertyName: CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn WSProtocolHandlerSetProperty(
        ref_: WSProtocolHandlerRef,
        propertyName: CFStringRef,
        propertyValue: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn GetIconRefOwners(theIconRef: IconRef, owners: *mut UInt16) -> OSErr;
}
unsafe extern "C" {
    pub fn AcquireIconRef(theIconRef: IconRef) -> OSErr;
}
unsafe extern "C" {
    pub fn ReleaseIconRef(theIconRef: IconRef) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIconRef(
        vRefNum: SInt16,
        creator: OSType,
        iconType: OSType,
        theIconRef: *mut IconRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIconRefFromFolder(
        vRefNum: SInt16,
        parentFolderID: SInt32,
        folderID: SInt32,
        attributes: SInt8,
        accessPrivileges: SInt8,
        theIconRef: *mut IconRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIconRefFromFileInfo(
        inRef: *const FSRef,
        inFileNameLength: UniCharCount,
        inFileName: *const UniChar,
        inWhichInfo: FSCatalogInfoBitmap,
        inCatalogInfo: *const FSCatalogInfo,
        inUsageFlags: IconServicesUsageFlags,
        outIconRef: *mut IconRef,
        outLabel: *mut SInt16,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetIconRefFromTypeInfo(
        inCreator: OSType,
        inType: OSType,
        inExtension: CFStringRef,
        inMIMEType: CFStringRef,
        inUsageFlags: IconServicesUsageFlags,
        outIconRef: *mut IconRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn GetIconRefFromIconFamilyPtr(
        inIconFamilyPtr: *const IconFamilyResource,
        inSize: Size,
        outIconRef: *mut IconRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetIconRefFromComponent(inComponent: Component, outIconRef: *mut IconRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn RegisterIconRefFromIconFamily(
        creator: OSType,
        iconType: OSType,
        iconFamily: IconFamilyHandle,
        theIconRef: *mut IconRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn RegisterIconRefFromFSRef(
        creator: OSType,
        iconType: OSType,
        iconFile: *const FSRef,
        theIconRef: *mut IconRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UnregisterIconRef(creator: OSType, iconType: OSType) -> OSErr;
}
unsafe extern "C" {
    pub fn UpdateIconRef(theIconRef: IconRef) -> OSErr;
}
unsafe extern "C" {
    pub fn OverrideIconRef(oldIconRef: IconRef, newIconRef: IconRef) -> OSErr;
}
unsafe extern "C" {
    pub fn RemoveIconRefOverride(theIconRef: IconRef) -> OSErr;
}
unsafe extern "C" {
    pub fn CompositeIconRef(
        backgroundIconRef: IconRef,
        foregroundIconRef: IconRef,
        compositeIconRef: *mut IconRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn IsIconRefComposite(
        compositeIconRef: IconRef,
        backgroundIconRef: *mut IconRef,
        foregroundIconRef: *mut IconRef,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn IsValidIconRef(theIconRef: IconRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IsDataAvailableInIconRef(inIconKind: OSType, inIconRef: IconRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SetCustomIconsEnabled(vRefNum: SInt16, enableCustomIcons: Boolean) -> OSErr;
}
unsafe extern "C" {
    pub fn GetCustomIconsEnabled(vRefNum: SInt16, customIconsEnabled: *mut Boolean) -> OSErr;
}
unsafe extern "C" {
    pub fn ReadIconFromFSRef(ref_: *const FSRef, iconFamily: *mut IconFamilyHandle) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyDefaultApplicationURLForURL(
        inURL: CFURLRef,
        inRoleMask: LSRolesMask,
        outError: *mut CFErrorRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn LSCopyDefaultApplicationURLForContentType(
        inContentType: CFStringRef,
        inRoleMask: LSRolesMask,
        outError: *mut CFErrorRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn LSCopyApplicationURLsForBundleIdentifier(
        inBundleIdentifier: CFStringRef,
        outError: *mut CFErrorRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn LSCopyApplicationURLsForURL(inURL: CFURLRef, inRoleMask: LSRolesMask) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn LSCanURLAcceptURL(
        inItemURL: CFURLRef,
        inTargetURL: CFURLRef,
        inRoleMask: LSRolesMask,
        inFlags: LSAcceptanceFlags,
        outAcceptsItem: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSRegisterURL(inURL: CFURLRef, inUpdate: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyDefaultRoleHandlerForContentType(
        inContentType: CFStringRef,
        inRole: LSRolesMask,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn LSCopyAllRoleHandlersForContentType(
        inContentType: CFStringRef,
        inRole: LSRolesMask,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn LSSetDefaultRoleHandlerForContentType(
        inContentType: CFStringRef,
        inRole: LSRolesMask,
        inHandlerBundleID: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyDefaultHandlerForURLScheme(inURLScheme: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn LSCopyAllHandlersForURLScheme(inURLScheme: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn LSSetDefaultHandlerForURLScheme(
        inURLScheme: CFStringRef,
        inHandlerBundleID: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyItemInfoForURL(
        inURL: CFURLRef,
        inWhichInfo: LSRequestedInfo,
        outItemInfo: *mut LSItemInfoRecord,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyItemInfoForRef(
        inItemRef: *const FSRef,
        inWhichInfo: LSRequestedInfo,
        outItemInfo: *mut LSItemInfoRecord,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSGetExtensionInfo(
        inNameLen: UniCharCount,
        inNameBuffer: *const UniChar,
        outExtStartIndex: *mut UniCharCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyDisplayNameForRef(
        inRef: *const FSRef,
        outDisplayName: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyDisplayNameForURL(inURL: CFURLRef, outDisplayName: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSetExtensionHiddenForRef(inRef: *const FSRef, inHide: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSetExtensionHiddenForURL(inURL: CFURLRef, inHide: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyKindStringForRef(
        inFSRef: *const FSRef,
        outKindString: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyKindStringForURL(inURL: CFURLRef, outKindString: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyKindStringForTypeInfo(
        inType: OSType,
        inCreator: OSType,
        inExtension: CFStringRef,
        outKindString: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyKindStringForMIMEType(
        inMIMEType: CFStringRef,
        outKindString: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSGetApplicationForItem(
        inItemRef: *const FSRef,
        inRoleMask: LSRolesMask,
        outAppRef: *mut FSRef,
        outAppURL: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSGetApplicationForInfo(
        inType: OSType,
        inCreator: OSType,
        inExtension: CFStringRef,
        inRoleMask: LSRolesMask,
        outAppRef: *mut FSRef,
        outAppURL: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyApplicationForMIMEType(
        inMIMEType: CFStringRef,
        inRoleMask: LSRolesMask,
        outAppURL: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSGetApplicationForURL(
        inURL: CFURLRef,
        inRoleMask: LSRolesMask,
        outAppRef: *mut FSRef,
        outAppURL: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSFindApplicationForInfo(
        inCreator: OSType,
        inBundleID: CFStringRef,
        inName: CFStringRef,
        outAppRef: *mut FSRef,
        outAppURL: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCanRefAcceptItem(
        inItemFSRef: *const FSRef,
        inTargetRef: *const FSRef,
        inRoleMask: LSRolesMask,
        inFlags: LSAcceptanceFlags,
        outAcceptsItem: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSRegisterFSRef(inRef: *const FSRef, inUpdate: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub static kLSItemContentType: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemFileType: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemFileCreator: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemExtension: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemDisplayName: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemDisplayKind: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemRoleHandlerDisplayName: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemIsInvisible: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemExtensionIsHidden: CFStringRef;
}
unsafe extern "C" {
    pub static kLSItemQuarantineProperties: CFStringRef;
}
unsafe extern "C" {
    pub fn LSCopyItemAttribute(
        inItem: *const FSRef,
        inRoles: LSRolesMask,
        inAttributeName: CFStringRef,
        outValue: *mut CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSCopyItemAttributes(
        inItem: *const FSRef,
        inRoles: LSRolesMask,
        inAttributeNames: CFArrayRef,
        outValues: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSetItemAttribute(
        inItem: *const FSRef,
        inRoles: LSRolesMask,
        inAttributeName: CFStringRef,
        inValue: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSGetHandlerOptionsForContentType(inContentType: CFStringRef) -> LSHandlerOptions;
}
unsafe extern "C" {
    pub fn LSSetHandlerOptionsForContentType(
        inContentType: CFStringRef,
        inOptions: LSHandlerOptions,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSOpenCFURLRef(inURL: CFURLRef, outLaunchedURL: *mut CFURLRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSOpenFromURLSpec(
        inLaunchSpec: *const LSLaunchURLSpec,
        outLaunchedURL: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSOpenFSRef(inRef: *const FSRef, outLaunchedRef: *mut FSRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSOpenFromRefSpec(
        inLaunchSpec: *const LSLaunchFSRefSpec,
        outLaunchedRef: *mut FSRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kLSQuarantineAgentNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineAgentBundleIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineTimeStampKey: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineTypeWebDownload: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineTypeOtherDownload: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineTypeEmailAttachment: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineTypeInstantMessageAttachment: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineTypeCalendarEventAttachment: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineTypeOtherAttachment: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineOriginURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kLSQuarantineDataURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeItem: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeContent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCompositeContent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMessage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeContact: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDiskImage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAliasRecord: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeData: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDirectory: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeResolvable: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSymLink: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeExecutable: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMountPoint: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAliasFile: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeURLBookmarkData: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeURL: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFileURL: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePlainText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUTF8PlainText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUTF16ExternalPlainText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUTF16PlainText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDelimitedText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCommaSeparatedText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeTabSeparatedText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUTF8TabSeparatedText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeRTF: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeHTML: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeXML: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJavaSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSourceCode: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAssemblyLanguageSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeObjectiveCSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSwiftSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCPlusPlusSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeObjectiveCPlusPlusSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCHeader: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCPlusPlusHeader: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAppleScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeOSAScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeOSAScriptBundle: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJavaScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeShellScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePerlScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePythonScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeRubyScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePHPScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJSON: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePropertyList: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeXMLPropertyList: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBinaryPropertyList: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeTXNTextAndMultimediaData: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePDF: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeRTFD: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFlatRTFD: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeWebArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJPEG2000: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeQuickTimeImage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeImage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJPEG: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeTIFF: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePICT: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeGIF: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePNG: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAppleICNS: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBMP: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeICO: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeRawImage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeScalableVectorGraphics: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeLivePhoto: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAudiovisualContent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMovie: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeVideo: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAudio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeQuickTimeMovie: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG2Video: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG2TransportStream: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMP3: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG4: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG4Audio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAppleProtectedMPEG4Audio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAppleProtectedMPEG4Video: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAVIMovie: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAudioInterchangeFileFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeWaveformAudio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMIDIAudio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePlaylist: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeM3UPlaylist: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFolder: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeVolume: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePackage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBundle: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePluginBundle: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSpotlightImporter: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeQuickLookGenerator: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeXPCService: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFramework: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeApplication: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeApplicationBundle: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeApplicationFile: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUnixExecutable: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJavaClass: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJavaArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeWindowsExecutable: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSystemPreferencesPane: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeGNUZipArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBzip2Archive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeZipArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSpreadsheet: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePresentation: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDatabase: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeVCard: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeToDoItem: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCalendarEvent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeEmailMessage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeInternetLocation: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeInkText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFont: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBookmark: CFStringRef;
}
unsafe extern "C" {
    pub static kUTType3DContent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePKCS12: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeX509Certificate: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeElectronicPublication: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeLog: CFStringRef;
}
unsafe extern "C" {
    pub static kUTExportedTypeDeclarationsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTImportedTypeDeclarationsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeTagSpecificationKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeConformsToKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDescriptionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeIconFileKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeReferenceURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeVersionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTagClassFilenameExtension: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTagClassMIMEType: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTagClassNSPboardType: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTagClassOSType: CFStringRef;
}
unsafe extern "C" {
    pub fn UTTypeCreatePreferredIdentifierForTag(
        inTagClass: CFStringRef,
        inTag: CFStringRef,
        inConformingToUTI: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn UTTypeCreateAllIdentifiersForTag(
        inTagClass: CFStringRef,
        inTag: CFStringRef,
        inConformingToUTI: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn UTTypeCopyPreferredTagWithClass(
        inUTI: CFStringRef,
        inTagClass: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn UTTypeCopyAllTagsWithClass(inUTI: CFStringRef, inTagClass: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn UTTypeEqual(inUTI1: CFStringRef, inUTI2: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn UTTypeConformsTo(inUTI: CFStringRef, inConformsToUTI: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn UTTypeCopyDescription(inUTI: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn UTTypeIsDeclared(inUTI: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn UTTypeIsDynamic(inUTI: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn UTTypeCopyDeclaration(inUTI: CFStringRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn UTTypeCopyDeclaringBundleURL(inUTI: CFStringRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn UTCreateStringForOSType(inOSType: OSType) -> CFStringRef;
}
unsafe extern "C" {
    pub fn UTGetOSTypeFromString(inString: CFStringRef) -> OSType;
}
unsafe extern "C" {
    pub fn MDItemGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn MDItemCreate(allocator: CFAllocatorRef, path: CFStringRef) -> MDItemRef;
}
unsafe extern "C" {
    pub fn MDItemCreateWithURL(allocator: CFAllocatorRef, url: CFURLRef) -> MDItemRef;
}
unsafe extern "C" {
    pub fn MDItemsCreateWithURLs(allocator: CFAllocatorRef, urls: CFArrayRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDItemCopyAttribute(item: MDItemRef, name: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn MDItemCopyAttributes(item: MDItemRef, names: CFArrayRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn MDItemCopyAttributeNames(item: MDItemRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDItemsCopyAttributes(items: CFArrayRef, names: CFArrayRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDItemGetCacheFileDescriptors(
        items: CFArrayRef,
        completionHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub static kMDItemAttributeChangeDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemContentType: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemContentTypeTree: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemKeywords: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemTitle: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAuthors: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemEditors: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemParticipants: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemProjects: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemDownloadedDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemWhereFroms: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemComment: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemCopyright: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLastUsedDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemContentCreationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemContentModificationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemDateAdded: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemDurationSeconds: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemContactKeywords: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPixelHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemXMPCredit: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemXMPDigitalSourceType: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPixelWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPixelCount: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemBitsPerSample: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFlashOnOff: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFocalLength: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAcquisitionMake: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAcquisitionModel: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemISOSpeed: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemOrientation: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLayerNames: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemWhiteBalance: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAperture: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemProfileName: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemResolutionWidthDPI: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemResolutionHeightDPI: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemExposureMode: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemExposureTimeSeconds: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemEXIFVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemCameraOwner: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFocalLength35mm: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLensModel: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemEXIFGPSVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAltitude: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLatitude: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLongitude: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemSpeed: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemTimestamp: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSTrack: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemImageDirection: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemNamedLocation: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSMeasureMode: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSDOP: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSMapDatum: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSDestLatitude: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSDestLongitude: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSDestBearing: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSDestDistance: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSProcessingMethod: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSAreaInformation: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSDateStamp: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGPSDifferental: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemMediaExtensions: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemCodecs: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemMediaTypes: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemStreamable: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemTotalBitRate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemVideoBitRate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAudioBitRate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemDeliveryType: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAlbum: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemHasAlphaChannel: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemRedEyeOnOff: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemMeteringMode: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemMaxAperture: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemExposureProgram: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemExposureTimeString: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemHeadline: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemInstructions: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemCity: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemStateOrProvince: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemCountry: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSName: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemDisplayName: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPath: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSSize: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSCreationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSContentChangeDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSOwnerUserID: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSOwnerGroupID: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSExists: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSIsReadable: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSIsWriteable: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSHasCustomIcon: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSIsExtensionHidden: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSIsStationery: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSInvisible: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSLabel: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFSNodeCount: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemHTMLContent: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemTextContent: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAudioSampleRate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAudioChannelCount: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemTempo: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemKeySignature: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemTimeSignature: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAudioEncodingApplication: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemComposer: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLyricist: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAudioTrackNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemRecordingDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemMusicalGenre: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemIsGeneralMIDISequence: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemRecordingYear: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemOrganizations: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLanguages: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemRights: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPublishers: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemContributors: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemCoverage: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemSubject: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemTheme: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAudiences: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemNumberOfPages: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPageWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPageHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemSecurityMethod: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemCreator: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemEncodingApplications: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemDueDate: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemStarRating: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPhoneNumbers: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemEmailAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemInstantMessageAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemKind: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemRecipients: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFinderComment: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemFonts: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAppleLoopsRootKey: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAppleLoopsKeyFilterType: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAppleLoopsLoopMode: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAppleLoopDescriptors: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemMusicalInstrumentCategory: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemMusicalInstrumentName: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemCFBundleIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemSupportFileType: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemInformation: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemDirector: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemProducer: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemGenre: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemPerformers: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemOriginalFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemOriginalSource: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAuthorEmailAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemRecipientEmailAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemAuthorAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemRecipientAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemURL: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLabelIcon: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLabelID: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLabelKind: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemLabelUUID: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemIsLikelyJunk: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemExecutableArchitectures: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemExecutablePlatform: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemApplicationCategories: CFStringRef;
}
unsafe extern "C" {
    pub static kMDItemIsApplicationManaged: CFStringRef;
}
unsafe extern "C" {
    pub fn __MDItemCopyAttributesEllipsis1(item: MDItemRef, ...) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn MDQueryGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn MDQueryCreate(
        allocator: CFAllocatorRef,
        queryString: CFStringRef,
        valueListAttrs: CFArrayRef,
        sortingAttrs: CFArrayRef,
    ) -> MDQueryRef;
}
unsafe extern "C" {
    pub fn MDQueryCreateSubset(
        allocator: CFAllocatorRef,
        query: MDQueryRef,
        queryString: CFStringRef,
        valueListAttrs: CFArrayRef,
        sortingAttrs: CFArrayRef,
    ) -> MDQueryRef;
}
unsafe extern "C" {
    pub fn MDQueryCreateForItems(
        allocator: CFAllocatorRef,
        queryString: CFStringRef,
        valueListAttrs: CFArrayRef,
        sortingAttrs: CFArrayRef,
        items: CFArrayRef,
    ) -> MDQueryRef;
}
unsafe extern "C" {
    pub fn MDQueryCopyQueryString(query: MDQueryRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn MDQueryCopyValueListAttributes(query: MDQueryRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDQueryCopySortingAttributes(query: MDQueryRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDQueryGetBatchingParameters(query: MDQueryRef) -> MDQueryBatchingParams;
}
unsafe extern "C" {
    pub fn MDQuerySetBatchingParameters(query: MDQueryRef, params: MDQueryBatchingParams);
}
unsafe extern "C" {
    pub fn MDQuerySetDispatchQueue(query: MDQueryRef, queue: NSObject);
}
unsafe extern "C" {
    pub fn MDQueryExecute(query: MDQueryRef, optionFlags: CFOptionFlags) -> Boolean;
}
unsafe extern "C" {
    pub fn MDQueryStop(query: MDQueryRef);
}
unsafe extern "C" {
    pub fn MDQueryDisableUpdates(query: MDQueryRef);
}
unsafe extern "C" {
    pub fn MDQueryEnableUpdates(query: MDQueryRef);
}
unsafe extern "C" {
    pub fn MDQueryIsGatheringComplete(query: MDQueryRef) -> Boolean;
}
unsafe extern "C" {
    pub fn MDQueryGetResultCount(query: MDQueryRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn MDQueryGetResultAtIndex(
        query: MDQueryRef,
        idx: CFIndex,
    ) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn MDQueryGetIndexOfResult(
        query: MDQueryRef,
        result: *const ::std::os::raw::c_void,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn MDQueryGetAttributeValueOfResultAtIndex(
        query: MDQueryRef,
        name: CFStringRef,
        idx: CFIndex,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn MDQueryCopyValuesOfAttribute(query: MDQueryRef, name: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDQueryGetCountOfResultsWithAttributeValue(
        query: MDQueryRef,
        name: CFStringRef,
        value: CFTypeRef,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn MDQuerySetSortOrder(query: MDQueryRef, sortingAttrs: CFArrayRef) -> Boolean;
}
unsafe extern "C" {
    pub fn MDQuerySetSortOptionFlagsForAttribute(
        query: MDQueryRef,
        fieldName: CFStringRef,
        flags: u32,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn MDQuerySetSortComparatorBlock(
        query: MDQueryRef,
        comparator: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub static kMDQueryProgressNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryDidFinishNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryDidUpdateNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryUpdateAddedItems: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryUpdateChangedItems: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryUpdateRemovedItems: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryResultContentRelevance: CFStringRef;
}
unsafe extern "C" {
    pub fn MDQuerySetSearchScope(
        query: MDQueryRef,
        scopeDirectories: CFArrayRef,
        scopeOptions: OptionBits,
    );
}
unsafe extern "C" {
    pub static kMDQueryScopeHome: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryScopeComputer: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryScopeNetwork: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryScopeAllIndexed: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryScopeComputerIndexed: CFStringRef;
}
unsafe extern "C" {
    pub static kMDQueryScopeNetworkIndexed: CFStringRef;
}
unsafe extern "C" {
    pub fn MDQuerySetMaxCount(query: MDQueryRef, size: CFIndex);
}
unsafe extern "C" {
    pub fn MDLabelGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn MDItemCopyLabels(item: MDItemRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDItemSetLabel(item: MDItemRef, label: MDLabelRef) -> Boolean;
}
unsafe extern "C" {
    pub fn MDItemRemoveLabel(item: MDItemRef, label: MDLabelRef) -> Boolean;
}
unsafe extern "C" {
    pub fn MDLabelCreate(
        allocator: CFAllocatorRef,
        displayName: CFStringRef,
        kind: CFStringRef,
        domain: MDLabelDomain,
    ) -> MDLabelRef;
}
unsafe extern "C" {
    pub fn MDLabelCopyAttribute(label: MDLabelRef, name: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn MDLabelCopyAttributeName(label: MDLabelRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn MDLabelDelete(label: MDLabelRef) -> Boolean;
}
unsafe extern "C" {
    pub fn MDLabelSetAttributes(label: MDLabelRef, attrs: CFDictionaryRef) -> Boolean;
}
unsafe extern "C" {
    pub fn MDCopyLabelKinds() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDCopyLabelsMatchingExpression(simpleQueryString: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDCopyLabelsWithKind(kind: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDCopyLabelWithUUID(labelUUID: CFUUIDRef) -> MDLabelRef;
}
unsafe extern "C" {
    pub static mut kMDLabelBundleURL: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelContentChangeDate: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelDisplayName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelIconData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelIconUUID: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelIsMutuallyExclusiveSetMember: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelKind: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelSetsFinderColor: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelUUID: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelVisibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelKindIsMutuallyExclusiveSetKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDLabelKindVisibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDPrivateVisibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kMDPublicVisibility: CFStringRef;
}
unsafe extern "C" {
    pub static kMDLabelAddedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kMDLabelChangedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kMDLabelRemovedNotification: CFStringRef;
}
unsafe extern "C" {
    pub fn MDSchemaCopyAttributesForContentType(contentTypeUTI: CFStringRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn MDSchemaCopyMetaAttributesForAttribute(name: CFStringRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn MDSchemaCopyAllAttributes() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MDSchemaCopyDisplayNameForAttribute(name: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn MDSchemaCopyDisplayDescriptionForAttribute(name: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub static kMDAttributeDisplayValues: CFStringRef;
}
unsafe extern "C" {
    pub static kMDAttributeAllValues: CFStringRef;
}
unsafe extern "C" {
    pub static kMDAttributeReadOnlyValues: CFStringRef;
}
unsafe extern "C" {
    pub static kMDExporterAvaliable: CFStringRef;
}
unsafe extern "C" {
    pub static kMDAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kMDAttributeType: CFStringRef;
}
unsafe extern "C" {
    pub static kMDAttributeMultiValued: CFStringRef;
}
unsafe extern "C" {
    pub fn SKDocumentGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SKDocumentCreateWithURL(inURL: CFURLRef) -> SKDocumentRef;
}
unsafe extern "C" {
    pub fn SKDocumentCopyURL(inDocument: SKDocumentRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn SKDocumentCreate(
        inScheme: CFStringRef,
        inParent: SKDocumentRef,
        inName: CFStringRef,
    ) -> SKDocumentRef;
}
unsafe extern "C" {
    pub fn SKDocumentGetSchemeName(inDocument: SKDocumentRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SKDocumentGetName(inDocument: SKDocumentRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SKDocumentGetParent(inDocument: SKDocumentRef) -> SKDocumentRef;
}
unsafe extern "C" {
    pub static kSKMinTermLength: CFStringRef;
}
unsafe extern "C" {
    pub static kSKSubstitutions: CFStringRef;
}
unsafe extern "C" {
    pub static kSKStopWords: CFStringRef;
}
unsafe extern "C" {
    pub static kSKProximityIndexing: CFStringRef;
}
unsafe extern "C" {
    pub static kSKMaximumTerms: CFStringRef;
}
unsafe extern "C" {
    pub static kSKTermChars: CFStringRef;
}
unsafe extern "C" {
    pub static kSKStartTermChars: CFStringRef;
}
unsafe extern "C" {
    pub static kSKEndTermChars: CFStringRef;
}
unsafe extern "C" {
    pub static kSKLanguageTypes: CFStringRef;
}
unsafe extern "C" {
    pub fn SKIndexGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SKIndexDocumentIteratorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SKIndexCreateWithURL(
        inURL: CFURLRef,
        inIndexName: CFStringRef,
        inIndexType: SKIndexType,
        inAnalysisProperties: CFDictionaryRef,
    ) -> SKIndexRef;
}
unsafe extern "C" {
    pub fn SKIndexOpenWithURL(
        inURL: CFURLRef,
        inIndexName: CFStringRef,
        inWriteAccess: Boolean,
    ) -> SKIndexRef;
}
unsafe extern "C" {
    pub fn SKIndexCreateWithMutableData(
        inData: CFMutableDataRef,
        inIndexName: CFStringRef,
        inIndexType: SKIndexType,
        inAnalysisProperties: CFDictionaryRef,
    ) -> SKIndexRef;
}
unsafe extern "C" {
    pub fn SKIndexOpenWithData(inData: CFDataRef, inIndexName: CFStringRef) -> SKIndexRef;
}
unsafe extern "C" {
    pub fn SKIndexOpenWithMutableData(
        inData: CFMutableDataRef,
        inIndexName: CFStringRef,
    ) -> SKIndexRef;
}
unsafe extern "C" {
    pub fn SKIndexFlush(inIndex: SKIndexRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SKIndexSetMaximumBytesBeforeFlush(inIndex: SKIndexRef, inBytesForUpdate: CFIndex);
}
unsafe extern "C" {
    pub fn SKIndexGetMaximumBytesBeforeFlush(inIndex: SKIndexRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKIndexCompact(inIndex: SKIndexRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SKIndexGetIndexType(inIndex: SKIndexRef) -> SKIndexType;
}
unsafe extern "C" {
    pub fn SKIndexGetAnalysisProperties(inIndex: SKIndexRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SKIndexGetDocumentCount(inIndex: SKIndexRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKIndexClose(inIndex: SKIndexRef);
}
unsafe extern "C" {
    pub fn SKIndexAddDocumentWithText(
        inIndex: SKIndexRef,
        inDocument: SKDocumentRef,
        inDocumentText: CFStringRef,
        inCanReplace: Boolean,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SKIndexAddDocument(
        inIndex: SKIndexRef,
        inDocument: SKDocumentRef,
        inMIMETypeHint: CFStringRef,
        inCanReplace: Boolean,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SKIndexRemoveDocument(inIndex: SKIndexRef, inDocument: SKDocumentRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SKIndexCopyDocumentProperties(
        inIndex: SKIndexRef,
        inDocument: SKDocumentRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SKIndexSetDocumentProperties(
        inIndex: SKIndexRef,
        inDocument: SKDocumentRef,
        inProperties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn SKIndexGetDocumentState(
        inIndex: SKIndexRef,
        inDocument: SKDocumentRef,
    ) -> SKDocumentIndexState;
}
unsafe extern "C" {
    pub fn SKIndexGetDocumentID(inIndex: SKIndexRef, inDocument: SKDocumentRef) -> SKDocumentID;
}
unsafe extern "C" {
    pub fn SKIndexCopyDocumentForDocumentID(
        inIndex: SKIndexRef,
        inDocumentID: SKDocumentID,
    ) -> SKDocumentRef;
}
unsafe extern "C" {
    pub fn SKIndexRenameDocument(
        inIndex: SKIndexRef,
        inDocument: SKDocumentRef,
        inNewName: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SKIndexMoveDocument(
        inIndex: SKIndexRef,
        inDocument: SKDocumentRef,
        inNewParent: SKDocumentRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SKIndexDocumentIteratorCreate(
        inIndex: SKIndexRef,
        inParentDocument: SKDocumentRef,
    ) -> SKIndexDocumentIteratorRef;
}
unsafe extern "C" {
    pub fn SKIndexDocumentIteratorCopyNext(inIterator: SKIndexDocumentIteratorRef)
        -> SKDocumentRef;
}
unsafe extern "C" {
    pub fn SKIndexGetMaximumDocumentID(inIndex: SKIndexRef) -> SKDocumentID;
}
unsafe extern "C" {
    pub fn SKIndexGetDocumentTermCount(inIndex: SKIndexRef, inDocumentID: SKDocumentID) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKIndexCopyTermIDArrayForDocumentID(
        inIndex: SKIndexRef,
        inDocumentID: SKDocumentID,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SKIndexGetDocumentTermFrequency(
        inIndex: SKIndexRef,
        inDocumentID: SKDocumentID,
        inTermID: CFIndex,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKIndexGetMaximumTermID(inIndex: SKIndexRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKIndexGetTermDocumentCount(inIndex: SKIndexRef, inTermID: CFIndex) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKIndexCopyDocumentIDArrayForTermID(
        inIndex: SKIndexRef,
        inTermID: CFIndex,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SKIndexCopyTermStringForTermID(inIndex: SKIndexRef, inTermID: CFIndex) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SKIndexGetTermIDForTermString(inIndex: SKIndexRef, inTermString: CFStringRef)
        -> CFIndex;
}
unsafe extern "C" {
    pub fn SKLoadDefaultExtractorPlugIns();
}
unsafe extern "C" {
    pub fn SKSearchGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SKSearchCreate(
        inIndex: SKIndexRef,
        inQuery: CFStringRef,
        inSearchOptions: SKSearchOptions,
    ) -> SKSearchRef;
}
unsafe extern "C" {
    pub fn SKSearchCancel(inSearch: SKSearchRef);
}
unsafe extern "C" {
    pub fn SKSearchFindMatches(
        inSearch: SKSearchRef,
        inMaximumCount: CFIndex,
        outDocumentIDsArray: *mut SKDocumentID,
        outScoresArray: *mut f32,
        maximumTime: CFTimeInterval,
        outFoundCount: *mut CFIndex,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SKIndexCopyInfoForDocumentIDs(
        inIndex: SKIndexRef,
        inCount: CFIndex,
        inDocumentIDsArray: *mut SKDocumentID,
        outNamesArray: *mut CFStringRef,
        outParentIDsArray: *mut SKDocumentID,
    );
}
unsafe extern "C" {
    pub fn SKIndexCopyDocumentRefsForDocumentIDs(
        inIndex: SKIndexRef,
        inCount: CFIndex,
        inDocumentIDsArray: *mut SKDocumentID,
        outDocumentRefsArray: *mut SKDocumentRef,
    );
}
unsafe extern "C" {
    pub fn SKIndexCopyDocumentURLsForDocumentIDs(
        inIndex: SKIndexRef,
        inCount: CFIndex,
        inDocumentIDsArray: *mut SKDocumentID,
        outDocumentURLsArray: *mut CFURLRef,
    );
}
unsafe extern "C" {
    pub fn SKSearchGroupGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SKSearchResultsGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SKSearchGroupCreate(inArrayOfInIndexes: CFArrayRef) -> SKSearchGroupRef;
}
unsafe extern "C" {
    pub fn SKSearchGroupCopyIndexes(inSearchGroup: SKSearchGroupRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SKSearchResultsGetCount(inSearchResults: SKSearchResultsRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKSearchResultsGetInfoInRange(
        inSearchResults: SKSearchResultsRef,
        inRange: CFRange,
        outDocumentsArray: *mut SKDocumentRef,
        outIndexesArray: *mut SKIndexRef,
        outScoresArray: *mut f32,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKSearchResultsCopyMatchingTerms(
        inSearchResults: SKSearchResultsRef,
        inItem: CFIndex,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SKSummaryGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SKSummaryCreateWithString(inString: CFStringRef) -> SKSummaryRef;
}
unsafe extern "C" {
    pub fn SKSummaryGetSentenceCount(summary: SKSummaryRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKSummaryGetParagraphCount(summary: SKSummaryRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKSummaryCopySentenceAtIndex(summary: SKSummaryRef, i: CFIndex) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SKSummaryCopyParagraphAtIndex(summary: SKSummaryRef, i: CFIndex) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SKSummaryCopySentenceSummaryString(
        summary: SKSummaryRef,
        numSentences: CFIndex,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SKSummaryCopyParagraphSummaryString(
        summary: SKSummaryRef,
        numParagraphs: CFIndex,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SKSummaryGetSentenceSummaryInfo(
        summary: SKSummaryRef,
        numSentencesInSummary: CFIndex,
        outRankOrderOfSentences: *mut CFIndex,
        outSentenceIndexOfSentences: *mut CFIndex,
        outParagraphIndexOfSentences: *mut CFIndex,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn SKSummaryGetParagraphSummaryInfo(
        summary: SKSummaryRef,
        numParagraphsInSummary: CFIndex,
        outRankOrderOfParagraphs: *mut CFIndex,
        outParagraphIndexOfParagraphs: *mut CFIndex,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn FSEventStreamGetLatestEventId(streamRef: ConstFSEventStreamRef) -> FSEventStreamEventId;
}
unsafe extern "C" {
    pub fn FSEventStreamGetDeviceBeingWatched(streamRef: ConstFSEventStreamRef) -> dev_t;
}
unsafe extern "C" {
    pub fn FSEventStreamCopyPathsBeingWatched(streamRef: ConstFSEventStreamRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn FSEventsGetCurrentEventId() -> FSEventStreamEventId;
}
unsafe extern "C" {
    pub fn FSEventsCopyUUIDForDevice(dev: dev_t) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn FSEventsGetLastEventIdForDeviceBeforeTime(
        dev: dev_t,
        time: CFAbsoluteTime,
    ) -> FSEventStreamEventId;
}
unsafe extern "C" {
    pub fn FSEventsPurgeEventsForDeviceUpToEventId(
        dev: dev_t,
        eventId: FSEventStreamEventId,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn FSEventStreamRetain(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamRelease(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamScheduleWithRunLoop(
        streamRef: FSEventStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn FSEventStreamUnscheduleFromRunLoop(
        streamRef: FSEventStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn FSEventStreamSetDispatchQueue(streamRef: FSEventStreamRef, q: NSObject);
}
unsafe extern "C" {
    pub fn FSEventStreamInvalidate(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamStart(streamRef: FSEventStreamRef) -> Boolean;
}
unsafe extern "C" {
    pub fn FSEventStreamFlushAsync(streamRef: FSEventStreamRef) -> FSEventStreamEventId;
}
unsafe extern "C" {
    pub fn FSEventStreamFlushSync(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamStop(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamShow(streamRef: ConstFSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamCopyDescription(streamRef: ConstFSEventStreamRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn FSEventStreamSetExclusionPaths(
        streamRef: FSEventStreamRef,
        pathsToExclude: CFArrayRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListFavoriteVolumes: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListFavoriteItems: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListRecentApplicationItems: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListRecentDocumentItems: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListRecentServerItems: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListSessionLoginItems: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListGlobalLoginItems: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListRecentItemsMaxAmount: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListVolumesComputerVisible: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListVolumesIDiskVisible: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListVolumesNetworkVisible: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListItemBeforeFirst: LSSharedFileListItemRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListItemLast: LSSharedFileListItemRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListItemHidden: CFStringRef;
}
unsafe extern "C" {
    pub static mut kLSSharedFileListLoginItemHidden: CFStringRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn LSSharedFileListCreate(
        inAllocator: CFAllocatorRef,
        inListType: CFStringRef,
        listOptions: CFTypeRef,
    ) -> LSSharedFileListRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListSetAuthorization(
        inList: LSSharedFileListRef,
        inAuthorization: AuthorizationRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSharedFileListGetSeedValue(inList: LSSharedFileListRef) -> UInt32;
}
unsafe extern "C" {
    pub fn LSSharedFileListCopyProperty(
        inList: LSSharedFileListRef,
        inPropertyName: CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListSetProperty(
        inList: LSSharedFileListRef,
        inPropertyName: CFStringRef,
        inPropertyData: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSharedFileListCopySnapshot(
        inList: LSSharedFileListRef,
        outSnapshotSeed: *mut UInt32,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListInsertItemURL(
        inList: LSSharedFileListRef,
        insertAfterThisItem: LSSharedFileListItemRef,
        inDisplayName: CFStringRef,
        inIconRef: IconRef,
        inURL: CFURLRef,
        inPropertiesToSet: CFDictionaryRef,
        inPropertiesToClear: CFArrayRef,
    ) -> LSSharedFileListItemRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListInsertItemFSRef(
        inList: LSSharedFileListRef,
        insertAfterThisItem: LSSharedFileListItemRef,
        inDisplayName: CFStringRef,
        inIconRef: IconRef,
        inFSRef: *const FSRef,
        inPropertiesToSet: CFDictionaryRef,
        inPropertiesToClear: CFArrayRef,
    ) -> LSSharedFileListItemRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemMove(
        inList: LSSharedFileListRef,
        inItem: LSSharedFileListItemRef,
        inMoveAfterItem: LSSharedFileListItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemRemove(
        inList: LSSharedFileListRef,
        inItem: LSSharedFileListItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSharedFileListRemoveAllItems(inList: LSSharedFileListRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemGetID(inItem: LSSharedFileListItemRef) -> UInt32;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemCopyIconRef(inItem: LSSharedFileListItemRef) -> IconRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemCopyDisplayName(inItem: LSSharedFileListItemRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemResolve(
        inItem: LSSharedFileListItemRef,
        inFlags: LSSharedFileListResolutionFlags,
        outURL: *mut CFURLRef,
        outRef: *mut FSRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemCopyResolvedURL(
        inItem: LSSharedFileListItemRef,
        inFlags: LSSharedFileListResolutionFlags,
        outError: *mut CFErrorRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemCopyProperty(
        inItem: LSSharedFileListItemRef,
        inPropertyName: CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn LSSharedFileListItemSetProperty(
        inItem: LSSharedFileListItemRef,
        inPropertyName: CFStringRef,
        inPropertyData: CFTypeRef,
    ) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for UnsignedWide {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UnsignedWide {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UnsignedWide", &[]);
}
unsafe impl objc2::encode::RefEncode for Float80 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Float80 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Float80", &[]);
}
unsafe impl objc2::encode::RefEncode for Point {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Point {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Point", &[]);
}
unsafe impl objc2::encode::RefEncode for Rect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Rect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Rect", &[]);
}
unsafe impl objc2::encode::RefEncode for HFSUniStr255 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HFSUniStr255 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HFSUniStr255", &[]);
}
unsafe impl objc2::encode::RefEncode for CustomBadgeResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CustomBadgeResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CustomBadgeResource", &[]);
}
unsafe impl objc2::encode::RefEncode for RoutingResourceEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RoutingResourceEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RoutingResourceEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for FileInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FileInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FileInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FolderInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FolderInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FolderInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ExtendedFileInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExtendedFileInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExtendedFileInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ExtendedFolderInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExtendedFolderInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExtendedFolderInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FXInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FXInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FXInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for DInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for DXInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DXInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DXInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OffPair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OffPair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OffPair", &[]);
}
unsafe impl objc2::encode::RefEncode for Intl0Rec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Intl0Rec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Intl0Rec", &[]);
}
unsafe impl objc2::encode::RefEncode for Intl1Rec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Intl1Rec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Intl1Rec", &[]);
}
unsafe impl objc2::encode::RefEncode for Itl1ExtRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Itl1ExtRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Itl1ExtRec", &[]);
}
unsafe impl objc2::encode::RefEncode for UntokenTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UntokenTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UntokenTable", &[]);
}
unsafe impl objc2::encode::RefEncode for WideChar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WideChar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WideChar", &[]);
}
unsafe impl objc2::encode::RefEncode for WideChar__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WideChar__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WideChar__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for WideCharArr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WideCharArr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WideCharArr", &[]);
}
unsafe impl objc2::encode::RefEncode for NumberParts {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NumberParts {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NumberParts", &[]);
}
unsafe impl objc2::encode::RefEncode for Itl4Rec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Itl4Rec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Itl4Rec", &[]);
}
unsafe impl objc2::encode::RefEncode for NItl4Rec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NItl4Rec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NItl4Rec", &[]);
}
unsafe impl objc2::encode::RefEncode for TableDirectoryRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TableDirectoryRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TableDirectoryRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for Itl5Record {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Itl5Record {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Itl5Record", &[]);
}
unsafe impl objc2::encode::RefEncode for RuleBasedTrslRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RuleBasedTrslRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RuleBasedTrslRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for ItlcRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ItlcRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ItlcRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for ItlbRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ItlbRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ItlbRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for ItlbExtRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ItlbExtRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ItlbExtRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for TokenRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TokenRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TokenRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TokenBlock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TokenBlock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TokenBlock", &[]);
}
unsafe impl objc2::encode::RefEncode for UTCDateTime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UTCDateTime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UTCDateTime", &[]);
}
unsafe impl objc2::encode::RefEncode for LocalDateTime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LocalDateTime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LocalDateTime", &[]);
}
unsafe impl objc2::encode::RefEncode for TextEncodingRun {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TextEncodingRun {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TextEncodingRun", &[]);
}
unsafe impl objc2::encode::RefEncode for ScriptCodeRun {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScriptCodeRun {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScriptCodeRun", &[]);
}
unsafe impl objc2::encode::RefEncode for TECInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for RoutineRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RoutineRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RoutineRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for RoutineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RoutineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RoutineDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for MixedModeStateRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MixedModeStateRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MixedModeStateRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCollection", &[]);
}
unsafe impl objc2::encode::RefEncode for BigEndianUInt32 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BigEndianUInt32 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BigEndianUInt32", &[]);
}
unsafe impl objc2::encode::RefEncode for BigEndianLong {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BigEndianLong {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BigEndianLong", &[]);
}
unsafe impl objc2::encode::RefEncode for BigEndianUnsignedLong {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BigEndianUnsignedLong {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BigEndianUnsignedLong", &[]);
}
unsafe impl objc2::encode::RefEncode for BigEndianShort {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BigEndianShort {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BigEndianShort", &[]);
}
unsafe impl objc2::encode::RefEncode for BigEndianUnsignedShort {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BigEndianUnsignedShort {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BigEndianUnsignedShort", &[]);
}
unsafe impl objc2::encode::RefEncode for BigEndianFixed {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BigEndianFixed {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BigEndianFixed", &[]);
}
unsafe impl objc2::encode::RefEncode for BigEndianUnsignedFixed {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BigEndianUnsignedFixed {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BigEndianUnsignedFixed", &[]);
}
unsafe impl objc2::encode::RefEncode for BigEndianOSType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BigEndianOSType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BigEndianOSType", &[]);
}
unsafe impl objc2::encode::RefEncode for DateCacheRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DateCacheRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DateCacheRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for DateTimeRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DateTimeRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DateTimeRec", &[]);
}
unsafe impl objc2::encode::RefEncode for LongDateCvt {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LongDateCvt {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LongDateCvt", &[]);
}
unsafe impl objc2::encode::RefEncode for LongDateCvt__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LongDateCvt__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LongDateCvt__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for LongDateRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LongDateRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LongDateRec", &[]);
}
unsafe impl objc2::encode::RefEncode for LongDateRec__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LongDateRec__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LongDateRec__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for LongDateRec__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LongDateRec__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LongDateRec__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for TogglePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TogglePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TogglePB", &[]);
}
unsafe impl objc2::encode::RefEncode for QElem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QElem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("QElem", &[]);
}
unsafe impl objc2::encode::RefEncode for QHdr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QHdr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("QHdr", &[]);
}
unsafe impl objc2::encode::RefEncode for MachineLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MachineLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MachineLocation", &[]);
}
unsafe impl objc2::encode::RefEncode for MachineLocation__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MachineLocation__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MachineLocation__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MachineLocation__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MachineLocation__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MachineLocation__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for DeferredTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DeferredTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DeferredTask", &[]);
}
unsafe impl objc2::encode::RefEncode for FSRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSRef", &[]);
}
unsafe impl objc2::encode::RefEncode for __FSFileSecurity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __FSFileSecurity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__FSFileSecurity", &[]);
}
unsafe impl objc2::encode::RefEncode for CatPositionRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CatPositionRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CatPositionRec", &[]);
}
unsafe impl objc2::encode::RefEncode for FSSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for ParamBlockRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ParamBlockRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ParamBlockRec", &[]);
}
unsafe impl objc2::encode::RefEncode for FSPermissionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSPermissionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSPermissionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FSCatalogInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSCatalogInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSCatalogInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FSRefParam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSRefParam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSRefParam", &[]);
}
unsafe impl objc2::encode::RefEncode for FSRefForkIOParam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSRefForkIOParam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSRefForkIOParam", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueFSIterator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueFSIterator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueFSIterator", &[]);
}
unsafe impl objc2::encode::RefEncode for FSSearchParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSSearchParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSSearchParams", &[]);
}
unsafe impl objc2::encode::RefEncode for FSCatalogBulkParam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSCatalogBulkParam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSCatalogBulkParam", &[]);
}
unsafe impl objc2::encode::RefEncode for FSForkIOParam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSForkIOParam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSForkIOParam", &[]);
}
unsafe impl objc2::encode::RefEncode for FSForkInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSForkInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSForkInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FSForkCBInfoParam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSForkCBInfoParam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSForkCBInfoParam", &[]);
}
unsafe impl objc2::encode::RefEncode for FSRangeLockParam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSRangeLockParam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSRangeLockParam", &[]);
}
unsafe impl objc2::encode::RefEncode for FSVolumeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSVolumeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSVolumeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FSVolumeInfoParam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSVolumeInfoParam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSVolumeInfoParam", &[]);
}
unsafe impl objc2::encode::RefEncode for GetVolParmsInfoBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GetVolParmsInfoBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GetVolParmsInfoBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for VolMountInfoHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VolMountInfoHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VolMountInfoHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for VolumeMountInfoHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VolumeMountInfoHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VolumeMountInfoHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for AFPVolMountInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AFPVolMountInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AFPVolMountInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AFPXVolMountInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AFPXVolMountInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AFPXVolMountInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AFPTagData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AFPTagData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AFPTagData", &[]);
}
unsafe impl objc2::encode::RefEncode for AFPAlternateAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AFPAlternateAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AFPAlternateAddress", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueFNSubscriptionRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueFNSubscriptionRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueFNSubscriptionRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueFSVolumeOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueFSVolumeOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueFSVolumeOperation", &[]);
}
unsafe impl objc2::encode::RefEncode for __FSFileOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __FSFileOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__FSFileOperation", &[]);
}
unsafe impl objc2::encode::RefEncode for FSFileOperationClientContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSFileOperationClientContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSFileOperationClientContext", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for ResourceSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ResourceSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ResourceSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentResource", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentPlatformInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentPlatformInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentPlatformInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentResourceExtension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentResourceExtension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentResourceExtension", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentPlatformInfoArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentPlatformInfoArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentPlatformInfoArray", &[]);
}
unsafe impl objc2::encode::RefEncode for ExtComponentResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExtComponentResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExtComponentResource", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentAliasResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentAliasResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentAliasResource", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentParameters", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentInstanceRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentInstanceRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentInstanceRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for RegisteredComponentRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RegisteredComponentRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RegisteredComponentRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for RegisteredComponentInstanceRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RegisteredComponentInstanceRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RegisteredComponentInstanceRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentMPWorkFunctionHeaderRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentMPWorkFunctionHeaderRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentMPWorkFunctionHeaderRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPProcessID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPProcessID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPProcessID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPTaskID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPTaskID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPTaskID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPQueueID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPQueueID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPQueueID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPSemaphoreID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPSemaphoreID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPSemaphoreID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPCriticalRegionID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPCriticalRegionID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPCriticalRegionID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPTimerID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPTimerID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPTimerID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPEventID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPEventID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPEventID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPAddressSpaceID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPAddressSpaceID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPAddressSpaceID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPNotificationID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPNotificationID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPNotificationID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPCoherenceID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPCoherenceID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPCoherenceID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPCpuID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPCpuID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPCpuID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPAreaID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPAreaID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPAreaID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPConsoleID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPConsoleID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPConsoleID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMPOpaqueID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMPOpaqueID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMPOpaqueID", &[]);
}
unsafe impl objc2::encode::RefEncode for MPTaskInfoVersion2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPTaskInfoVersion2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MPTaskInfoVersion2", &[]);
}
unsafe impl objc2::encode::RefEncode for MPTaskInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPTaskInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MPTaskInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AliasRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AliasRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AliasRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for FSAliasInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSAliasInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSAliasInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueLocaleRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueLocaleRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueLocaleRef", &[]);
}
unsafe impl objc2::encode::RefEncode for LocaleAndVariant {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LocaleAndVariant {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LocaleAndVariant", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAreaID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAreaID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAreaID", &[]);
}
unsafe impl objc2::encode::RefEncode for MachineInformationPowerPC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MachineInformationPowerPC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MachineInformationPowerPC", &[]);
}
unsafe impl objc2::encode::RefEncode for RegisterInformationPowerPC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RegisterInformationPowerPC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RegisterInformationPowerPC", &[]);
}
unsafe impl objc2::encode::RefEncode for FPUInformationPowerPC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FPUInformationPowerPC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FPUInformationPowerPC", &[]);
}
unsafe impl objc2::encode::RefEncode for Vector128 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Vector128 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Vector128", &[]);
}
unsafe impl objc2::encode::RefEncode for VectorInformationPowerPC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VectorInformationPowerPC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VectorInformationPowerPC", &[]);
}
unsafe impl objc2::encode::RefEncode for MemoryExceptionInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MemoryExceptionInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MemoryExceptionInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for ExceptionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExceptionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExceptionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ExceptionInformationPowerPC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExceptionInformationPowerPC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExceptionInformationPowerPC", &[]);
}
unsafe impl objc2::encode::RefEncode for MachineInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MachineInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MachineInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for RegisterInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RegisterInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("RegisterInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for FPUInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FPUInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FPUInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for VectorInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VectorInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VectorInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for ExceptionInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExceptionInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExceptionInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for NumFormatString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NumFormatString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NumFormatString", &[]);
}
unsafe impl objc2::encode::RefEncode for FVector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FVector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FVector", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyStateRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyStateRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyStateRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyStateEntryTerminal {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyStateEntryTerminal {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyStateEntryTerminal", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyStateEntryRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyStateEntryRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyStateEntryRange", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyboardTypeHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyboardTypeHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyboardTypeHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyboardLayout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyboardLayout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyboardLayout", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyLayoutFeatureInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyLayoutFeatureInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyLayoutFeatureInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyModifiersToTableNum {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyModifiersToTableNum {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyModifiersToTableNum", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyToCharTableIndex {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyToCharTableIndex {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyToCharTableIndex", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyStateRecordsIndex {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyStateRecordsIndex {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyStateRecordsIndex", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeyStateTerminators {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeyStateTerminators {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeyStateTerminators", &[]);
}
unsafe impl objc2::encode::RefEncode for UCKeySequenceDataIndex {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UCKeySequenceDataIndex {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UCKeySequenceDataIndex", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCollatorRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCollatorRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCollatorRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueUCTypeSelectRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueUCTypeSelectRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueUCTypeSelectRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueTextBreakLocatorRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueTextBreakLocatorRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueTextBreakLocatorRef", &[]);
}
unsafe impl objc2::encode::RefEncode for decimal {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for decimal {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("decimal", &[]);
}
unsafe impl objc2::encode::RefEncode for decimal__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for decimal__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("decimal__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for decform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for decform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("decform", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueTECObjectRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueTECObjectRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueTECObjectRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueTECSnifferObjectRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueTECSnifferObjectRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueTECSnifferObjectRef", &[]);
}
unsafe impl objc2::encode::RefEncode for TECConversionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECConversionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECConversionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueTextToUnicodeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueTextToUnicodeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueTextToUnicodeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueUnicodeToTextInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueUnicodeToTextInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueUnicodeToTextInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueUnicodeToTextRunInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueUnicodeToTextRunInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueUnicodeToTextRunInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for UnicodeMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UnicodeMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("UnicodeMapping", &[]);
}
unsafe impl objc2::encode::RefEncode for SchedulerInfoRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SchedulerInfoRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SchedulerInfoRec", &[]);
}
unsafe impl objc2::encode::RefEncode for FolderDesc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FolderDesc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FolderDesc", &[]);
}
unsafe impl objc2::encode::RefEncode for FolderRouting {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FolderRouting {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FolderRouting", &[]);
}
unsafe impl objc2::encode::RefEncode for TMTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TMTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TMTask", &[]);
}
unsafe impl objc2::encode::RefEncode for MPQueueInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPQueueInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MPQueueInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for MPSemaphoreInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSemaphoreInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MPSemaphoreInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for MPEventInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPEventInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MPEventInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for MPCriticalRegionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPCriticalRegionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MPCriticalRegionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for MPNotificationInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPNotificationInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MPNotificationInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for MPAddressSpaceInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPAddressSpaceInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MPAddressSpaceInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFContainerHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFContainerHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFContainerHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFSectionHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFSectionHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFSectionHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFLoaderInfoHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFLoaderInfoHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFLoaderInfoHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFImportedLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFImportedLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFImportedLibrary", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFImportedSymbol {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFImportedSymbol {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFImportedSymbol", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFExportedSymbolHashSlot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFExportedSymbolHashSlot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFExportedSymbolHashSlot", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFSplitHashWord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFSplitHashWord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFSplitHashWord", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFExportedSymbolKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFExportedSymbolKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFExportedSymbolKey", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFExportedSymbolKey__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFExportedSymbolKey__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFExportedSymbolKey__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFExportedSymbol {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFExportedSymbol {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFExportedSymbol", &[]);
}
unsafe impl objc2::encode::RefEncode for PEFLoaderRelocationHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PEFLoaderRelocationHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PEFLoaderRelocationHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for XLibContainerHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for XLibContainerHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("XLibContainerHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for XLibExportedSymbol {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for XLibExportedSymbol {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("XLibExportedSymbol", &[]);
}
unsafe impl objc2::encode::RefEncode for ChunkHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ChunkHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ChunkHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ContainerChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ContainerChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ContainerChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for FormatVersionChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FormatVersionChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FormatVersionChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for CommonChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CommonChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CommonChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for ExtCommonChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExtCommonChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExtCommonChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for SoundDataChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SoundDataChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SoundDataChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for Marker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Marker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Marker", &[]);
}
unsafe impl objc2::encode::RefEncode for MarkerChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MarkerChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MarkerChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for AIFFLoop {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AIFFLoop {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AIFFLoop", &[]);
}
unsafe impl objc2::encode::RefEncode for InstrumentChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for InstrumentChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("InstrumentChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIDataChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIDataChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIDataChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioRecordingChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioRecordingChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioRecordingChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for ApplicationSpecificChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ApplicationSpecificChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ApplicationSpecificChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for Comment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Comment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Comment", &[]);
}
unsafe impl objc2::encode::RefEncode for CommentsChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CommentsChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CommentsChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for TextChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TextChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TextChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for TextEncodingRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TextEncodingRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TextEncodingRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECEncodingsListRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECEncodingsListRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECEncodingsListRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECSubTextEncodingRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECSubTextEncodingRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECSubTextEncodingRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECSubTextEncodingsRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECSubTextEncodingsRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECSubTextEncodingsRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECEncodingPairRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECEncodingPairRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECEncodingPairRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECEncodingPairs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECEncodingPairs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECEncodingPairs", &[]);
}
unsafe impl objc2::encode::RefEncode for TECEncodingPairsRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECEncodingPairsRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECEncodingPairsRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECLocaleListToEncodingListRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECLocaleListToEncodingListRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECLocaleListToEncodingListRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECLocaleToEncodingsListRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECLocaleToEncodingsListRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECLocaleToEncodingsListRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECInternetNameRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECInternetNameRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECInternetNameRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECInternetNamesRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECInternetNamesRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECInternetNamesRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECBufferContextRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECBufferContextRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECBufferContextRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECPluginStateRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECPluginStateRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECPluginStateRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECConverterContextRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECConverterContextRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECConverterContextRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECSnifferContextRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECSnifferContextRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECSnifferContextRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TECPluginDispatchTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TECPluginDispatchTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TECPluginDispatchTable", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAEDataStorageType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAEDataStorageType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAEDataStorageType", &[]);
}
unsafe impl objc2::encode::RefEncode for AEDesc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AEDesc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AEDesc", &[]);
}
unsafe impl objc2::encode::RefEncode for AEKeyDesc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AEKeyDesc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AEKeyDesc", &[]);
}
unsafe impl objc2::encode::RefEncode for AEArrayData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AEArrayData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AEArrayData", &[]);
}
unsafe impl objc2::encode::RefEncode for AERemoteProcessResolverContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AERemoteProcessResolverContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AERemoteProcessResolverContext", &[]);
}
unsafe impl objc2::encode::RefEncode for AERemoteProcessResolver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AERemoteProcessResolver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AERemoteProcessResolver", &[]);
}
unsafe impl objc2::encode::RefEncode for ccntTokenRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ccntTokenRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ccntTokenRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for TextRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TextRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TextRange", &[]);
}
unsafe impl objc2::encode::RefEncode for TextRangeArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TextRangeArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TextRangeArray", &[]);
}
unsafe impl objc2::encode::RefEncode for OffsetArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OffsetArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OffsetArray", &[]);
}
unsafe impl objc2::encode::RefEncode for WritingCode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WritingCode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WritingCode", &[]);
}
unsafe impl objc2::encode::RefEncode for IntlText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IntlText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IntlText", &[]);
}
unsafe impl objc2::encode::RefEncode for TScriptingSizeResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TScriptingSizeResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TScriptingSizeResource", &[]);
}
unsafe impl objc2::encode::RefEncode for AEBuildError {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AEBuildError {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AEBuildError", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAEStreamRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAEStreamRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAEStreamRef", &[]);
}
unsafe impl objc2::encode::RefEncode for __DCSDictionary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DCSDictionary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DCSDictionary", &[]);
}
unsafe impl objc2::encode::RefEncode for __CSIdentityAuthority {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CSIdentityAuthority {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CSIdentityAuthority", &[]);
}
unsafe impl objc2::encode::RefEncode for __CSIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CSIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CSIdentity", &[]);
}
unsafe impl objc2::encode::RefEncode for __CSIdentityQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CSIdentityQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CSIdentityQuery", &[]);
}
unsafe impl objc2::encode::RefEncode for CSIdentityClientContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSIdentityClientContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSIdentityClientContext", &[]);
}
unsafe impl objc2::encode::RefEncode for CSIdentityQueryClientContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSIdentityQueryClientContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSIdentityQueryClientContext", &[]);
}
unsafe impl objc2::encode::RefEncode for IconFamilyElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IconFamilyElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IconFamilyElement", &[]);
}
unsafe impl objc2::encode::RefEncode for IconFamilyResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IconFamilyResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IconFamilyResource", &[]);
}
unsafe impl objc2::encode::RefEncode for SleepQRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SleepQRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SleepQRec", &[]);
}
unsafe impl objc2::encode::RefEncode for KCCallbackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KCCallbackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KCCallbackInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for WSClientContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WSClientContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WSClientContext", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueWSMethodInvocationRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueWSMethodInvocationRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueWSMethodInvocationRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueWSProtocolHandlerRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueWSProtocolHandlerRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueWSProtocolHandlerRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueIconRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueIconRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueIconRef", &[]);
}
unsafe impl objc2::encode::RefEncode for LSItemInfoRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LSItemInfoRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LSItemInfoRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for LSLaunchURLSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LSLaunchURLSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LSLaunchURLSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for LSLaunchFSRefSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LSLaunchFSRefSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LSLaunchFSRefSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for LSApplicationParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LSApplicationParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LSApplicationParameters", &[]);
}
unsafe impl objc2::encode::RefEncode for __MDItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __MDItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__MDItem", &[]);
}
unsafe impl objc2::encode::RefEncode for __MDQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __MDQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__MDQuery", &[]);
}
unsafe impl objc2::encode::RefEncode for MDQueryBatchingParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDQueryBatchingParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDQueryBatchingParams", &[]);
}
unsafe impl objc2::encode::RefEncode for __MDLabel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __MDLabel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__MDLabel", &[]);
}
unsafe impl objc2::encode::RefEncode for MDImporterInterfaceStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDImporterInterfaceStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDImporterInterfaceStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for MDExporterInterfaceStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDExporterInterfaceStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDExporterInterfaceStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for MDImporterURLInterfaceStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDImporterURLInterfaceStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDImporterURLInterfaceStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for MDImporterBundleWrapperURLInterfaceStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDImporterBundleWrapperURLInterfaceStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDImporterBundleWrapperURLInterfaceStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for __SKIndex {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SKIndex {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SKIndex", &[]);
}
unsafe impl objc2::encode::RefEncode for __SKIndexDocumentIterator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SKIndexDocumentIterator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SKIndexDocumentIterator", &[]);
}
unsafe impl objc2::encode::RefEncode for __SKSearch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SKSearch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SKSearch", &[]);
}
unsafe impl objc2::encode::RefEncode for __SKSearchGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SKSearchGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SKSearchGroup", &[]);
}
unsafe impl objc2::encode::RefEncode for __SKSearchResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SKSearchResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SKSearchResults", &[]);
}
unsafe impl objc2::encode::RefEncode for __SKSummary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SKSummary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SKSummary", &[]);
}
unsafe impl objc2::encode::RefEncode for __FSEventStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __FSEventStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__FSEventStream", &[]);
}
unsafe impl objc2::encode::RefEncode for FSEventStreamContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSEventStreamContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSEventStreamContext", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueLSSharedFileListRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueLSSharedFileListRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueLSSharedFileListRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueLSSharedFileListItemRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueLSSharedFileListItemRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueLSSharedFileListItemRef", &[]);
}
