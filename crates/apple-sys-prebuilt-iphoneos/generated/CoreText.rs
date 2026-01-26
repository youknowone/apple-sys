#[allow(unused_imports)]
use crate::CFNetwork::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type UInt16 = ::std::os::raw::c_ushort;
pub type SInt16 = ::std::os::raw::c_short;
pub type UInt32 = ::std::os::raw::c_uint;
pub type Fixed = SInt32;
pub type CFIndex = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNumber {
    _unused: [u8; 0],
}
pub type CTFontSymbolicTraits = u32;
pub type CTFontStylisticClass = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTFontDescriptor {
    _unused: [u8; 0],
}
pub type CTFontDescriptorRef = *const __CTFontDescriptor;
pub type CTFontOrientation = u32;
pub type CTFontFormat = u32;
pub type CTFontPriority = u32;
pub type CTFontDescriptorMatchingState = u32;
pub type CTFontDescriptorProgressHandler = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTFont {
    _unused: [u8; 0],
}
pub type CTFontRef = *const __CTFont;
pub type CTFontOptions = CFOptionFlags;
pub type CTFontUIFontType = u32;
pub type ATSFontRef = UInt32;
pub type CTFontTableTag = FourCharCode;
pub type CTFontTableOptions = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTFontCollection {
    _unused: [u8; 0],
}
pub type CTFontCollectionRef = *const __CTFontCollection;
pub type CTMutableFontCollectionRef = *mut __CTFontCollection;
pub type CTFontCollectionSortDescriptorsCallback = ::std::option::Option<
    unsafe extern "C" fn(
        first: CTFontDescriptorRef,
        second: CTFontDescriptorRef,
        refCon: *mut ::std::os::raw::c_void,
    ) -> CFComparisonResult,
>;
pub type CTFontCollectionCopyOptions = u32;
pub type CTFontManagerError = CFIndex;
pub type CTFontManagerScope = u32;
pub type CTFontManagerAutoActivationSetting = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTFrame {
    _unused: [u8; 0],
}
pub type CTFrameRef = *const __CTFrame;
pub type CTFrameProgression = u32;
pub type CTFramePathFillRule = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTLine {
    _unused: [u8; 0],
}
pub type CTLineRef = *const __CTLine;
pub type CTLineBoundsOptions = CFOptionFlags;
pub type CTLineTruncationType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTTypesetter {
    _unused: [u8; 0],
}
pub type CTTypesetterRef = *const __CTTypesetter;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTFramesetter {
    _unused: [u8; 0],
}
pub type CTFramesetterRef = *const __CTFramesetter;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTGlyphInfo {
    _unused: [u8; 0],
}
pub type CTGlyphInfoRef = *const __CTGlyphInfo;
pub type CTCharacterCollection = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTParagraphStyle {
    _unused: [u8; 0],
}
pub type CTParagraphStyleRef = *const __CTParagraphStyle;
pub type CTTextAlignment = u8;
pub type CTLineBreakMode = u8;
pub type CTWritingDirection = i8;
pub type CTParagraphStyleSpecifier = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CTParagraphStyleSetting {
    pub spec: CTParagraphStyleSpecifier,
    pub valueSize: usize,
    pub value: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTRubyAnnotation {
    _unused: [u8; 0],
}
pub type CTRubyAnnotationRef = *const __CTRubyAnnotation;
pub type CTRubyAlignment = u8;
pub type CTRubyOverhang = u8;
pub type CTRubyPosition = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTRun {
    _unused: [u8; 0],
}
pub type CTRunRef = *const __CTRun;
pub type CTRunStatus = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTRunDelegate {
    _unused: [u8; 0],
}
pub type CTRunDelegateRef = *const __CTRunDelegate;
pub type CTRunDelegateDeallocateCallback =
    ::std::option::Option<unsafe extern "C" fn(refCon: *mut ::std::os::raw::c_void)>;
pub type CTRunDelegateGetAscentCallback =
    ::std::option::Option<unsafe extern "C" fn(refCon: *mut ::std::os::raw::c_void) -> CGFloat>;
pub type CTRunDelegateGetDescentCallback =
    ::std::option::Option<unsafe extern "C" fn(refCon: *mut ::std::os::raw::c_void) -> CGFloat>;
pub type CTRunDelegateGetWidthCallback =
    ::std::option::Option<unsafe extern "C" fn(refCon: *mut ::std::os::raw::c_void) -> CGFloat>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CTRunDelegateCallbacks {
    pub version: CFIndex,
    pub dealloc: CTRunDelegateDeallocateCallback,
    pub getAscent: CTRunDelegateGetAscentCallback,
    pub getDescent: CTRunDelegateGetDescentCallback,
    pub getWidth: CTRunDelegateGetWidthCallback,
}
pub trait PCTAdaptiveImageProviding: Sized + std::ops::Deref {
    unsafe fn imageForProposedSize_scaleFactor_imageOffset_imageSize_(
        &self,
        proposedSize: CGSize,
        scaleFactor: CGFloat,
        outImageOffset: *mut CGPoint,
        outImageSize: *mut CGSize,
    ) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageForProposedSize : proposedSize, scaleFactor : scaleFactor, imageOffset : outImageOffset, imageSize : outImageSize)
    }
}
pub type CTUnderlineStyle = i32;
pub type CTUnderlineStyleModifiers = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CTTextTab {
    _unused: [u8; 0],
}
pub type CTTextTabRef = *const __CTTextTab;
pub type SFNTLookupTableFormat = UInt16;
pub type SFNTLookupValue = UInt16;
pub type SFNTLookupOffset = UInt16;
pub type SFNTLookupKind = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SFNTLookupBinarySearchHeader {
    pub unitSize: UInt16,
    pub nUnits: UInt16,
    pub searchRange: UInt16,
    pub entrySelector: UInt16,
    pub rangeShift: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SFNTLookupArrayHeader {
    pub lookupValues: [SFNTLookupValue; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SFNTLookupTrimmedArrayHeader {
    pub firstGlyph: UInt16,
    pub count: UInt16,
    pub valueArray: [SFNTLookupValue; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SFNTLookupVectorHeader {
    pub valueSize: UInt16,
    pub firstGlyph: UInt16,
    pub count: UInt16,
    pub values: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SFNTLookupSegment {
    pub lastGlyph: UInt16,
    pub firstGlyph: UInt16,
    pub value: [UInt16; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SFNTLookupSegmentHeader {
    pub binSearch: SFNTLookupBinarySearchHeader,
    pub segments: [SFNTLookupSegment; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SFNTLookupSingle {
    pub glyph: UInt16,
    pub value: [UInt16; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SFNTLookupSingleHeader {
    pub binSearch: SFNTLookupBinarySearchHeader,
    pub entries: [SFNTLookupSingle; 1usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SFNTLookupFormatSpecificHeader {
    pub theArray: SFNTLookupArrayHeader,
    pub segment: SFNTLookupSegmentHeader,
    pub single: SFNTLookupSingleHeader,
    pub trimmedArray: SFNTLookupTrimmedArrayHeader,
    pub vector: SFNTLookupVectorHeader,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SFNTLookupTable {
    pub format: SFNTLookupTableFormat,
    pub fsHeader: SFNTLookupFormatSpecificHeader,
}
pub type SFNTLookupTablePtr = *mut SFNTLookupTable;
pub type SFNTLookupTableHandle = *mut SFNTLookupTablePtr;
pub type STClass = UInt8;
pub type STEntryIndex = UInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STHeader {
    pub filler: UInt8,
    pub nClasses: STClass,
    pub classTableOffset: UInt16,
    pub stateArrayOffset: UInt16,
    pub entryTableOffset: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STClassTable {
    pub firstGlyph: UInt16,
    pub nGlyphs: UInt16,
    pub classes: [STClass; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STEntryZero {
    pub newState: UInt16,
    pub flags: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STEntryOne {
    pub newState: UInt16,
    pub flags: UInt16,
    pub offset1: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STEntryTwo {
    pub newState: UInt16,
    pub flags: UInt16,
    pub offset1: UInt16,
    pub offset2: UInt16,
}
pub type STXClass = UInt16;
pub type STXStateIndex = UInt16;
pub type STXEntryIndex = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct STXHeader {
    pub nClasses: UInt32,
    pub classTableOffset: UInt32,
    pub stateArrayOffset: UInt32,
    pub entryTableOffset: UInt32,
}
pub type STXClassTable = SFNTLookupTable;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STXEntryZero {
    pub newState: STXStateIndex,
    pub flags: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STXEntryOne {
    pub newState: STXStateIndex,
    pub flags: UInt16,
    pub index1: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STXEntryTwo {
    pub newState: STXStateIndex,
    pub flags: UInt16,
    pub index1: UInt16,
    pub index2: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LcarCaretClassEntry {
    pub count: UInt16,
    pub partials: [UInt16; 1usize],
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct LcarCaretTable {
    pub version: Fixed,
    pub format: UInt16,
    pub lookup: SFNTLookupTable,
}
pub type LcarCaretTablePtr = *mut LcarCaretTable;
pub type JustPCActionType = UInt16;
pub type JustificationFlags = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct JustPCDecompositionAction {
    pub lowerLimit: Fixed,
    pub upperLimit: Fixed,
    pub order: UInt16,
    pub count: UInt16,
    pub glyphs: [UInt16; 1usize],
}
pub type JustPCUnconditionalAddAction = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct JustPCConditionalAddAction {
    pub substThreshold: Fixed,
    pub addGlyph: UInt16,
    pub substGlyph: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct JustPCDuctilityAction {
    pub ductilityAxis: UInt32,
    pub minimumLimit: Fixed,
    pub noStretchValue: Fixed,
    pub maximumLimit: Fixed,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JustPCGlyphRepeatAddAction {
    pub flags: UInt16,
    pub glyph: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct JustPCActionSubrecord {
    pub theClass: UInt16,
    pub theType: JustPCActionType,
    pub length: UInt32,
    pub data: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct JustPCAction {
    pub actionCount: UInt32,
    pub actions: [JustPCActionSubrecord; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct JustWidthDeltaEntry {
    pub justClass: UInt32,
    pub beforeGrowLimit: Fixed,
    pub beforeShrinkLimit: Fixed,
    pub afterGrowLimit: Fixed,
    pub afterShrinkLimit: Fixed,
    pub growFlags: JustificationFlags,
    pub shrinkFlags: JustificationFlags,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct JustWidthDeltaGroup {
    pub count: UInt32,
    pub entries: [JustWidthDeltaEntry; 1usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JustPostcompTable {
    pub lookupTable: SFNTLookupTable,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JustDirectionTable {
    pub justClass: UInt16,
    pub widthDeltaClusters: UInt16,
    pub postcomp: UInt16,
    pub lookup: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct JustTable {
    pub version: Fixed,
    pub format: UInt16,
    pub horizHeaderOffset: UInt16,
    pub vertHeaderOffset: UInt16,
}
pub type OpbdTableFormat = UInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpbdSideValues {
    pub leftSideShift: SInt16,
    pub topSideShift: SInt16,
    pub rightSideShift: SInt16,
    pub bottomSideShift: SInt16,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct OpbdTable {
    pub version: Fixed,
    pub format: OpbdTableFormat,
    pub lookupTable: SFNTLookupTable,
}
pub type MortSubtableMaskFlags = UInt32;
pub type MortLigatureActionEntry = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MortRearrangementSubtable {
    pub header: STHeader,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MortContextualSubtable {
    pub header: STHeader,
    pub substitutionTableOffset: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MortLigatureSubtable {
    pub header: STHeader,
    pub ligatureActionTableOffset: UInt16,
    pub componentTableOffset: UInt16,
    pub ligatureTableOffset: UInt16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MortSwashSubtable {
    pub lookup: SFNTLookupTable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MortInsertionSubtable {
    pub header: STHeader,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MortSpecificSubtable {
    pub rearrangement: MortRearrangementSubtable,
    pub contextual: MortContextualSubtable,
    pub ligature: MortLigatureSubtable,
    pub swash: MortSwashSubtable,
    pub insertion: MortInsertionSubtable,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct MortSubtable {
    pub length: UInt16,
    pub coverage: UInt16,
    pub flags: MortSubtableMaskFlags,
    pub u: MortSpecificSubtable,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MortFeatureEntry {
    pub featureType: UInt16,
    pub featureSelector: UInt16,
    pub enableFlags: MortSubtableMaskFlags,
    pub disableFlags: MortSubtableMaskFlags,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MortChain {
    pub defaultFlags: MortSubtableMaskFlags,
    pub length: UInt32,
    pub nFeatures: UInt16,
    pub nSubtables: UInt16,
    pub featureEntries: [MortFeatureEntry; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MortTable {
    pub version: Fixed,
    pub nChains: UInt32,
    pub chains: [MortChain; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MorxRearrangementSubtable {
    pub header: STXHeader,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MorxContextualSubtable {
    pub header: STXHeader,
    pub substitutionTableOffset: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MorxLigatureSubtable {
    pub header: STXHeader,
    pub ligatureActionTableOffset: UInt32,
    pub componentTableOffset: UInt32,
    pub ligatureTableOffset: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MorxInsertionSubtable {
    pub header: STXHeader,
    pub insertionGlyphTableOffset: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MorxSpecificSubtable {
    pub rearrangement: MorxRearrangementSubtable,
    pub contextual: MorxContextualSubtable,
    pub ligature: MorxLigatureSubtable,
    pub swash: MortSwashSubtable,
    pub insertion: MorxInsertionSubtable,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct MorxSubtable {
    pub length: UInt32,
    pub coverage: UInt32,
    pub flags: MortSubtableMaskFlags,
    pub u: MorxSpecificSubtable,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MorxChain {
    pub defaultFlags: MortSubtableMaskFlags,
    pub length: UInt32,
    pub nFeatures: UInt32,
    pub nSubtables: UInt32,
    pub featureEntries: [MortFeatureEntry; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MorxTable {
    pub version: Fixed,
    pub nChains: UInt32,
    pub chains: [MorxChain; 1usize],
}
pub type PropCharProperties = UInt16;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct PropTable {
    pub version: Fixed,
    pub format: UInt16,
    pub defaultProps: PropCharProperties,
    pub lookup: SFNTLookupTable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PropLookupSegment {
    pub lastGlyph: UInt16,
    pub firstGlyph: UInt16,
    pub value: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PropLookupSingle {
    pub glyph: UInt16,
    pub props: PropCharProperties,
}
pub type TrakValue = SInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TrakTableEntry {
    pub track: Fixed,
    pub nameTableIndex: UInt16,
    pub sizesOffset: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TrakTableData {
    pub nTracks: UInt16,
    pub nSizes: UInt16,
    pub sizeTableOffset: UInt32,
    pub trakTable: [TrakTableEntry; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TrakTable {
    pub version: Fixed,
    pub format: UInt16,
    pub horizOffset: UInt16,
    pub vertOffset: UInt16,
}
pub type KernTableFormat = UInt8;
pub type KernSubtableInfo = UInt16;
pub type KernKerningValue = SInt16;
pub type KernArrayOffset = UInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernVersion0Header {
    pub version: UInt16,
    pub nTables: UInt16,
    pub firstSubtable: [UInt16; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct KernTableHeader {
    pub version: Fixed,
    pub nTables: SInt32,
    pub firstSubtable: [UInt16; 1usize],
}
pub type KernTableHeaderPtr = *mut KernTableHeader;
pub type KernTableHeaderHandle = *mut KernTableHeaderPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernKerningPair {
    pub left: UInt16,
    pub right: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernOrderedListEntry {
    pub pair: KernKerningPair,
    pub value: KernKerningValue,
}
pub type KernOrderedListEntryPtr = *mut KernOrderedListEntry;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernOrderedListHeader {
    pub nPairs: UInt16,
    pub searchRange: UInt16,
    pub entrySelector: UInt16,
    pub rangeShift: UInt16,
    pub table: [UInt16; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernStateHeader {
    pub header: STHeader,
    pub valueTable: UInt16,
    pub firstTable: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernStateEntry {
    pub newState: UInt16,
    pub flags: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernOffsetTable {
    pub firstGlyph: UInt16,
    pub nGlyphs: UInt16,
    pub offsetTable: [KernArrayOffset; 1usize],
}
pub type KernOffsetTablePtr = *mut KernOffsetTable;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernSimpleArrayHeader {
    pub rowWidth: UInt16,
    pub leftOffsetTable: UInt16,
    pub rightOffsetTable: UInt16,
    pub theArray: KernArrayOffset,
    pub firstTable: [UInt16; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernIndexArrayHeader {
    pub glyphCount: UInt16,
    pub kernValueCount: UInt8,
    pub leftClassCount: UInt8,
    pub rightClassCount: UInt8,
    pub flags: UInt8,
    pub kernValue: [SInt16; 1usize],
    pub leftClass: [UInt8; 1usize],
    pub rightClass: [UInt8; 1usize],
    pub kernIndex: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union KernFormatSpecificHeader {
    pub orderedList: KernOrderedListHeader,
    pub stateTable: KernStateHeader,
    pub simpleArray: KernSimpleArrayHeader,
    pub indexArray: KernIndexArrayHeader,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KernVersion0SubtableHeader {
    pub version: UInt16,
    pub length: UInt16,
    pub stInfo: KernSubtableInfo,
    pub fsHeader: KernFormatSpecificHeader,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct KernSubtableHeader {
    pub length: SInt32,
    pub stInfo: KernSubtableInfo,
    pub tupleIndex: SInt16,
    pub fsHeader: KernFormatSpecificHeader,
}
pub type KernSubtableHeaderPtr = *mut KernSubtableHeader;
pub type KerxSubtableCoverage = UInt32;
pub type KerxArrayOffset = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct KerxTableHeader {
    pub version: Fixed,
    pub nTables: UInt32,
    pub firstSubtable: [UInt32; 1usize],
}
pub type KerxTableHeaderPtr = *mut KerxTableHeader;
pub type KerxTableHeaderHandle = *mut KerxTableHeaderPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KerxKerningPair {
    pub left: UInt16,
    pub right: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KerxOrderedListEntry {
    pub pair: KerxKerningPair,
    pub value: KernKerningValue,
}
pub type KerxOrderedListEntryPtr = *mut KerxOrderedListEntry;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct KerxOrderedListHeader {
    pub nPairs: UInt32,
    pub searchRange: UInt32,
    pub entrySelector: UInt32,
    pub rangeShift: UInt32,
    pub table: [UInt32; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct KerxStateHeader {
    pub header: STXHeader,
    pub valueTable: UInt32,
    pub firstTable: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KerxStateEntry {
    pub newState: UInt16,
    pub flags: UInt16,
    pub valueIndex: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct KerxControlPointHeader {
    pub header: STXHeader,
    pub flags: UInt32,
    pub firstTable: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KerxControlPointEntry {
    pub newState: UInt16,
    pub flags: UInt16,
    pub actionIndex: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KerxControlPointAction {
    pub markControlPoint: UInt16,
    pub currControlPoint: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KerxAnchorPointAction {
    pub markAnchorPoint: UInt16,
    pub currAnchorPoint: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KerxCoordinateAction {
    pub markX: UInt16,
    pub markY: UInt16,
    pub currX: UInt16,
    pub currY: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct KerxSimpleArrayHeader {
    pub rowWidth: UInt32,
    pub leftOffsetTable: UInt32,
    pub rightOffsetTable: UInt32,
    pub theArray: KerxArrayOffset,
    pub firstTable: [UInt32; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct KerxIndexArrayHeader {
    pub flags: UInt32,
    pub rowCount: UInt16,
    pub columnCount: UInt16,
    pub rowIndexTableOffset: UInt32,
    pub columnIndexTableOffset: UInt32,
    pub kerningArrayOffset: UInt32,
    pub kerningVectorOffset: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union KerxFormatSpecificHeader {
    pub orderedList: KerxOrderedListHeader,
    pub stateTable: KerxStateHeader,
    pub simpleArray: KerxSimpleArrayHeader,
    pub indexArray: KerxIndexArrayHeader,
    pub controlPoint: KerxControlPointHeader,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct KerxSubtableHeader {
    pub length: UInt32,
    pub stInfo: KerxSubtableCoverage,
    pub tupleCount: UInt32,
    pub fsHeader: KerxFormatSpecificHeader,
}
pub type KerxSubtableHeaderPtr = *mut KerxSubtableHeader;
pub type BslnBaselineClass = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BslnFormat0Part {
    pub deltas: [SInt16; 32usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BslnFormat1Part {
    pub deltas: [SInt16; 32usize],
    pub mappingData: SFNTLookupTable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BslnFormat2Part {
    pub stdGlyph: UInt16,
    pub ctlPoints: [SInt16; 32usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BslnFormat3Part {
    pub stdGlyph: UInt16,
    pub ctlPoints: [SInt16; 32usize],
    pub mappingData: SFNTLookupTable,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union BslnFormatUnion {
    pub fmt0Part: BslnFormat0Part,
    pub fmt1Part: BslnFormat1Part,
    pub fmt2Part: BslnFormat2Part,
    pub fmt3Part: BslnFormat3Part,
}
pub type BslnTableFormat = UInt16;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct BslnTable {
    pub version: Fixed,
    pub format: BslnTableFormat,
    pub defaultBaseline: UInt16,
    pub parts: BslnFormatUnion,
}
pub type BslnTablePtr = *mut BslnTable;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct ALMXHeader {
    pub Version: Fixed,
    pub Flags: UInt16,
    pub NMasters: UInt16,
    pub FirstGlyph: UInt16,
    pub LastGlyph: UInt16,
    pub lookup: SFNTLookupTable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALMXGlyphEntry {
    pub GlyphIndexOffset: SInt16,
    pub HorizontalAdvance: SInt16,
    pub XOffsetToHOrigin: SInt16,
    pub VerticalAdvance: SInt16,
    pub YOffsetToVOrigin: SInt16,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct ROTAHeader {
    pub Version: Fixed,
    pub Flags: UInt16,
    pub NMasters: UInt16,
    pub FirstGlyph: UInt16,
    pub LastGlyph: UInt16,
    pub lookup: SFNTLookupTable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ROTAGlyphEntry {
    pub GlyphIndexOffset: SInt16,
    pub HBaselineOffset: SInt16,
    pub VBaselineOffset: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AnchorPoint {
    pub x: SInt16,
    pub y: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AnchorPointTable {
    pub nPoints: UInt32,
    pub points: [AnchorPoint; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AnkrTable {
    pub version: UInt16,
    pub flags: UInt16,
    pub lookupTableOffset: UInt32,
    pub anchorPointTableOffset: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LtagStringRange {
    pub offset: UInt16,
    pub length: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LtagTable {
    pub version: UInt32,
    pub flags: UInt32,
    pub numTags: UInt32,
    pub tagRange: [LtagStringRange; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntDirectoryEntry {
    pub tableTag: FourCharCode,
    pub checkSum: UInt32,
    pub offset: UInt32,
    pub length: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntDirectory {
    pub format: FourCharCode,
    pub numOffsets: UInt16,
    pub searchRange: UInt16,
    pub entrySelector: UInt16,
    pub rangeShift: UInt16,
    pub table: [sfntDirectoryEntry; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfntCMapSubHeader {
    pub format: UInt16,
    pub length: UInt16,
    pub languageID: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntCMapExtendedSubHeader {
    pub format: UInt16,
    pub reserved: UInt16,
    pub length: UInt32,
    pub language: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntCMapEncoding {
    pub platformID: UInt16,
    pub scriptID: UInt16,
    pub offset: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfntCMapHeader {
    pub version: UInt16,
    pub numTables: UInt16,
    pub encoding: [sfntCMapEncoding; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfntNameRecord {
    pub platformID: UInt16,
    pub scriptID: UInt16,
    pub languageID: UInt16,
    pub nameID: UInt16,
    pub length: UInt16,
    pub offset: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfntNameHeader {
    pub format: UInt16,
    pub count: UInt16,
    pub stringOffset: UInt16,
    pub rec: [sfntNameRecord; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntVariationAxis {
    pub axisTag: FourCharCode,
    pub minValue: Fixed,
    pub defaultValue: Fixed,
    pub maxValue: Fixed,
    pub flags: SInt16,
    pub nameID: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntInstance {
    pub nameID: SInt16,
    pub flags: SInt16,
    pub coord: [Fixed; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntVariationHeader {
    pub version: Fixed,
    pub offsetToData: UInt16,
    pub countSizePairs: UInt16,
    pub axisCount: UInt16,
    pub axisSize: UInt16,
    pub instanceCount: UInt16,
    pub instanceSize: UInt16,
    pub axis: [sfntVariationAxis; 1usize],
    pub instance: [sfntInstance; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntFontDescriptor {
    pub name: FourCharCode,
    pub value: Fixed,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntDescriptorHeader {
    pub version: Fixed,
    pub descriptorCount: SInt32,
    pub descriptor: [sfntFontDescriptor; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntFeatureName {
    pub featureType: UInt16,
    pub settingCount: UInt16,
    pub offsetToSettings: SInt32,
    pub featureFlags: UInt16,
    pub nameID: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfntFontFeatureSetting {
    pub setting: UInt16,
    pub nameID: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfntFontRunFeature {
    pub featureType: UInt16,
    pub setting: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct sfntFeatureHeader {
    pub version: SInt32,
    pub featureNameCount: UInt16,
    pub featureSetCount: UInt16,
    pub reserved: SInt32,
    pub names: [sfntFeatureName; 1usize],
    pub settings: [sfntFontFeatureSetting; 1usize],
    pub runs: [sfntFontRunFeature; 1usize],
}
pub type FontNameCode = UInt32;
pub type FontPlatformCode = UInt32;
pub type FontScriptCode = UInt32;
pub type FontLanguageCode = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FontVariation {
    pub name: FourCharCode,
    pub value: Fixed,
}
unsafe extern "C" {
    pub static kCTFontSymbolicTrait: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontWeightTrait: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontWidthTrait: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontSlantTrait: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCTFontURLAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontNameAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDisplayNameAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFamilyNameAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontStyleNameAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontTraitsAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVariationAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVariationAxesAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontSizeAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontMatrixAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontCascadeListAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontCharacterSetAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontLanguagesAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontBaselineAdjustAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontMacintoshEncodingsAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeaturesAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureSettingsAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFixedAdvanceAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontOrientationAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFormatAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontRegistrationScopeAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontPriorityAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontEnabledAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDownloadableAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDownloadedAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontOpticalSizeAttribute: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateWithNameAndSize(
        name: CFStringRef,
        size: CGFloat,
    ) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef)
        -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateCopyWithAttributes(
        original: CTFontDescriptorRef,
        attributes: CFDictionaryRef,
    ) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateCopyWithFamily(
        original: CTFontDescriptorRef,
        family: CFStringRef,
    ) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateCopyWithSymbolicTraits(
        original: CTFontDescriptorRef,
        symTraitValue: CTFontSymbolicTraits,
        symTraitMask: CTFontSymbolicTraits,
    ) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateCopyWithVariation(
        original: CTFontDescriptorRef,
        variationIdentifier: CFNumberRef,
        variationValue: CGFloat,
    ) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateCopyWithFeature(
        original: CTFontDescriptorRef,
        featureTypeIdentifier: CFNumberRef,
        featureSelectorIdentifier: CFNumberRef,
    ) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateMatchingFontDescriptors(
        descriptor: CTFontDescriptorRef,
        mandatoryAttributes: CFSetRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCreateMatchingFontDescriptor(
        descriptor: CTFontDescriptorRef,
        mandatoryAttributes: CFSetRef,
    ) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptorMatchingSourceDescriptor: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptorMatchingDescriptors: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptorMatchingResult: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptorMatchingPercentage: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptorMatchingCurrentAssetSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptorMatchingTotalDownloadedSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptorMatchingTotalAssetSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptorMatchingError: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorMatchFontDescriptorsWithProgressHandler(
        descriptors: CFArrayRef,
        mandatoryAttributes: CFSetRef,
        progressBlock: CTFontDescriptorProgressHandler,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCopyAttributes(descriptor: CTFontDescriptorRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCopyAttribute(
        descriptor: CTFontDescriptorRef,
        attribute: CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CTFontDescriptorCopyLocalizedAttribute(
        descriptor: CTFontDescriptorRef,
        attribute: CFStringRef,
        language: *mut CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CTFontGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCTFontCopyrightNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFamilyNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontSubFamilyNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontStyleNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontUniqueNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFullNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVersionNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontPostScriptNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontTrademarkNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontManufacturerNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDesignerNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDescriptionNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVendorURLNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontDesignerURLNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontLicenseNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontLicenseURLNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontSampleTextNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontPostScriptCIDNameKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCreateWithName(
        name: CFStringRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateWithFontDescriptor(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateWithNameAndOptions(
        name: CFStringRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        options: CTFontOptions,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateWithFontDescriptorAndOptions(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        options: CTFontOptions,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateUIFontForLanguage(
        uiType: CTFontUIFontType,
        size: CGFloat,
        language: CFStringRef,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateCopyWithAttributes(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateCopyWithSymbolicTraits(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        symTraitValue: CTFontSymbolicTraits,
        symTraitMask: CTFontSymbolicTraits,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateCopyWithFamily(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        family: CFStringRef,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateForString(
        currentFont: CTFontRef,
        string: CFStringRef,
        range: CFRange,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateForStringWithLanguage(
        currentFont: CTFontRef,
        string: CFStringRef,
        range: CFRange,
        language: CFStringRef,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCopyFontDescriptor(font: CTFontRef) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontCopyAttribute(font: CTFontRef, attribute: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CTFontGetSize(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetMatrix(font: CTFontRef) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CTFontGetSymbolicTraits(font: CTFontRef) -> CTFontSymbolicTraits;
}
unsafe extern "C" {
    pub fn CTFontCopyTraits(font: CTFontRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CTFontCopyDefaultCascadeListForLanguages(
        font: CTFontRef,
        languagePrefList: CFArrayRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontCopyPostScriptName(font: CTFontRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCopyFamilyName(font: CTFontRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCopyFullName(font: CTFontRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCopyDisplayName(font: CTFontRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCopyName(font: CTFontRef, nameKey: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCopyLocalizedName(
        font: CTFontRef,
        nameKey: CFStringRef,
        actualLanguage: *mut CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCopyCharacterSet(font: CTFontRef) -> CFCharacterSetRef;
}
unsafe extern "C" {
    pub fn CTFontGetStringEncoding(font: CTFontRef) -> CFStringEncoding;
}
unsafe extern "C" {
    pub fn CTFontCopySupportedLanguages(font: CTFontRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontGetGlyphsForCharacters(
        font: CTFontRef,
        characters: *const UniChar,
        glyphs: *mut CGGlyph,
        count: CFIndex,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetUnitsPerEm(font: CTFontRef) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn CTFontGetGlyphCount(font: CTFontRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CTFontGetBoundingBox(font: CTFontRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CTFontGetUnderlinePosition(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetUnderlineThickness(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetSlantAngle(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetCapHeight(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTFontGetGlyphWithName(font: CTFontRef, glyphName: CFStringRef) -> CGGlyph;
}
unsafe extern "C" {
    pub fn CTFontCopyNameForGlyph(font: CTFontRef, glyph: CGGlyph) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontGetBoundingRectsForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        boundingRects: *mut CGRect,
        count: CFIndex,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CTFontGetOpticalBoundsForGlyphs(
        font: CTFontRef,
        glyphs: *const CGGlyph,
        boundingRects: *mut CGRect,
        count: CFIndex,
        options: CFOptionFlags,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CTFontGetAdvancesForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        advances: *mut CGSize,
        count: CFIndex,
    ) -> f64;
}
unsafe extern "C" {
    pub fn CTFontGetVerticalTranslationsForGlyphs(
        font: CTFontRef,
        glyphs: *const CGGlyph,
        translations: *mut CGSize,
        count: CFIndex,
    );
}
unsafe extern "C" {
    pub fn CTFontCreatePathForGlyph(
        font: CTFontRef,
        glyph: CGGlyph,
        matrix: *const CGAffineTransform,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub static kCTFontVariationAxisIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVariationAxisMinimumValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVariationAxisMaximumValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVariationAxisDefaultValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVariationAxisNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontVariationAxisHiddenKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCopyVariationAxes(font: CTFontRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontCopyVariation(font: CTFontRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub static kCTFontOpenTypeFeatureTag: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontOpenTypeFeatureValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureTypeIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureTypeNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureTypeExclusiveKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureTypeSelectorsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureSelectorIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureSelectorNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureSelectorDefaultKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureSelectorSettingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureSampleTextKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontFeatureTooltipTextKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCopyFeatures(font: CTFontRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontCopyFeatureSettings(font: CTFontRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontCopyGraphicsFont(
        font: CTFontRef,
        attributes: *mut CTFontDescriptorRef,
    ) -> CGFontRef;
}
unsafe extern "C" {
    pub fn CTFontCreateWithGraphicsFont(
        graphicsFont: CGFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn CTFontCopyAvailableTables(font: CTFontRef, options: CTFontTableOptions) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontHasTable(font: CTFontRef, tag: CTFontTableTag) -> bool;
}
unsafe extern "C" {
    pub fn CTFontCopyTable(
        font: CTFontRef,
        table: CTFontTableTag,
        options: CTFontTableOptions,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CTFontDrawGlyphs(
        font: CTFontRef,
        glyphs: *const CGGlyph,
        positions: *const CGPoint,
        count: usize,
        context: CGContextRef,
    );
}
unsafe extern "C" {
    pub fn CTFontGetLigatureCaretPositions(
        font: CTFontRef,
        glyph: CGGlyph,
        positions: *mut CGFloat,
        maxPositions: CFIndex,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub static kCTBaselineClassRoman: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineClassIdeographicCentered: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineClassIdeographicLow: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineClassIdeographicHigh: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineClassHanging: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineClassMath: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineReferenceFont: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineOriginalFont: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontGetTypographicBoundsForAdaptiveImageProvider(
        font: CTFontRef,
        provider: *mut u64,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CTFontDrawImageFromAdaptiveImageProviderAtPoint(
        font: CTFontRef,
        provider: *mut u64,
        point: CGPoint,
        context: CGContextRef,
    );
}
unsafe extern "C" {
    pub fn CTFontCollectionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCTFontCollectionRemoveDuplicatesOption: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontCollectionCreateFromAvailableFonts(
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}
unsafe extern "C" {
    pub fn CTFontCollectionCreateWithFontDescriptors(
        queryDescriptors: CFArrayRef,
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}
unsafe extern "C" {
    pub fn CTFontCollectionCreateCopyWithFontDescriptors(
        original: CTFontCollectionRef,
        queryDescriptors: CFArrayRef,
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;
}
unsafe extern "C" {
    pub fn CTFontCollectionCreateMatchingFontDescriptors(
        collection: CTFontCollectionRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback(
        collection: CTFontCollectionRef,
        sortCallback: CTFontCollectionSortDescriptorsCallback,
        refCon: *mut ::std::os::raw::c_void,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontCollectionCreateMatchingFontDescriptorsWithOptions(
        collection: CTFontCollectionRef,
        options: CFDictionaryRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontCollectionCopyFontAttribute(
        collection: CTFontCollectionRef,
        attributeName: CFStringRef,
        options: CTFontCollectionCopyOptions,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontCollectionCopyFontAttributes(
        collection: CTFontCollectionRef,
        attributeNames: CFSetRef,
        options: CTFontCollectionCopyOptions,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub static kCTFontManagerErrorDomain: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontManagerErrorFontURLsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontManagerErrorFontDescriptorsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFontManagerErrorFontAssetNameKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontManagerCopyAvailablePostScriptNames() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontManagerCopyAvailableFontFamilyNames() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontManagerCompareFontFamilyNames(
        family1: *const ::std::os::raw::c_void,
        family2: *const ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    ) -> CFComparisonResult;
}
unsafe extern "C" {
    pub fn CTFontManagerCreateFontDescriptorsFromURL(fileURL: CFURLRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontManagerCreateFontDescriptorFromData(data: CFDataRef) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn CTFontManagerCreateFontDescriptorsFromData(data: CFDataRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub static kCTFontRegistrationUserInfoAttribute: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFontManagerRegisterFontsForURL(
        fontURL: CFURLRef,
        scope: CTFontManagerScope,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CTFontManagerUnregisterFontsForURL(
        fontURL: CFURLRef,
        scope: CTFontManagerScope,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CTFontManagerRegisterGraphicsFont(font: CGFontRef, error: *mut CFErrorRef) -> bool;
}
unsafe extern "C" {
    pub fn CTFontManagerUnregisterGraphicsFont(font: CGFontRef, error: *mut CFErrorRef) -> bool;
}
unsafe extern "C" {
    pub fn CTFontManagerRegisterFontsForURLs(
        fontURLs: CFArrayRef,
        scope: CTFontManagerScope,
        errors: *mut CFArrayRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CTFontManagerUnregisterFontsForURLs(
        fontURLs: CFArrayRef,
        scope: CTFontManagerScope,
        errors: *mut CFArrayRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CTFontManagerRegisterFontURLs(
        fontURLs: CFArrayRef,
        scope: CTFontManagerScope,
        enabled: bool,
        registrationHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CTFontManagerUnregisterFontURLs(
        fontURLs: CFArrayRef,
        scope: CTFontManagerScope,
        registrationHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CTFontManagerRegisterFontDescriptors(
        fontDescriptors: CFArrayRef,
        scope: CTFontManagerScope,
        enabled: bool,
        registrationHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CTFontManagerUnregisterFontDescriptors(
        fontDescriptors: CFArrayRef,
        scope: CTFontManagerScope,
        registrationHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CTFontManagerRegisterFontsWithAssetNames(
        fontAssetNames: CFArrayRef,
        bundle: CFBundleRef,
        scope: CTFontManagerScope,
        enabled: bool,
        registrationHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CTFontManagerCopyRegisteredFontDescriptors(
        scope: CTFontManagerScope,
        enabled: bool,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFontManagerRequestFonts(
        fontDescriptors: CFArrayRef,
        completionHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub static kCTFontManagerRegisteredFontsChangedNotification: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFrameGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCTFrameProgressionAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFramePathFillRuleAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFramePathWidthAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFrameClippingPathsAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTFramePathClippingPathAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub fn CTFrameGetStringRange(frame: CTFrameRef) -> CFRange;
}
unsafe extern "C" {
    pub fn CTFrameGetVisibleStringRange(frame: CTFrameRef) -> CFRange;
}
unsafe extern "C" {
    pub fn CTFrameGetPath(frame: CTFrameRef) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CTFrameGetFrameAttributes(frame: CTFrameRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CTFrameGetLines(frame: CTFrameRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTFrameGetLineOrigins(frame: CTFrameRef, range: CFRange, origins: *mut CGPoint);
}
unsafe extern "C" {
    pub fn CTFrameDraw(frame: CTFrameRef, context: CGContextRef);
}
unsafe extern "C" {
    pub fn CTLineGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CTLineCreateWithAttributedString(attrString: CFAttributedStringRef) -> CTLineRef;
}
unsafe extern "C" {
    pub fn CTLineCreateTruncatedLine(
        line: CTLineRef,
        width: f64,
        truncationType: CTLineTruncationType,
        truncationToken: CTLineRef,
    ) -> CTLineRef;
}
unsafe extern "C" {
    pub fn CTLineCreateJustifiedLine(
        line: CTLineRef,
        justificationFactor: CGFloat,
        justificationWidth: f64,
    ) -> CTLineRef;
}
unsafe extern "C" {
    pub fn CTLineGetGlyphCount(line: CTLineRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CTLineGetStringRange(line: CTLineRef) -> CFRange;
}
unsafe extern "C" {
    pub fn CTLineGetPenOffsetForFlush(
        line: CTLineRef,
        flushFactor: CGFloat,
        flushWidth: f64,
    ) -> f64;
}
unsafe extern "C" {
    pub fn CTLineDraw(line: CTLineRef, context: CGContextRef);
}
unsafe extern "C" {
    pub fn CTLineGetTypographicBounds(
        line: CTLineRef,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> f64;
}
unsafe extern "C" {
    pub fn CTLineGetBoundsWithOptions(line: CTLineRef, options: CTLineBoundsOptions) -> CGRect;
}
unsafe extern "C" {
    pub fn CTLineGetTrailingWhitespaceWidth(line: CTLineRef) -> f64;
}
unsafe extern "C" {
    pub fn CTLineGetImageBounds(line: CTLineRef, context: CGContextRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CTLineGetStringIndexForPosition(line: CTLineRef, position: CGPoint) -> CFIndex;
}
unsafe extern "C" {
    pub fn CTLineGetOffsetForStringIndex(
        line: CTLineRef,
        charIndex: CFIndex,
        secondaryOffset: *mut CGFloat,
    ) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTLineEnumerateCaretOffsets(line: CTLineRef, block: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CTTypesetterGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCTTypesetterOptionAllowUnboundedLayout: CFStringRef;
}
unsafe extern "C" {
    pub static kCTTypesetterOptionDisableBidiProcessing: CFStringRef;
}
unsafe extern "C" {
    pub static kCTTypesetterOptionForcedEmbeddingLevel: CFStringRef;
}
unsafe extern "C" {
    pub fn CTTypesetterCreateWithAttributedString(string: CFAttributedStringRef)
        -> CTTypesetterRef;
}
unsafe extern "C" {
    pub fn CTTypesetterCreateWithAttributedStringAndOptions(
        string: CFAttributedStringRef,
        options: CFDictionaryRef,
    ) -> CTTypesetterRef;
}
unsafe extern "C" {
    pub fn CTTypesetterCreateLineWithOffset(
        typesetter: CTTypesetterRef,
        stringRange: CFRange,
        offset: f64,
    ) -> CTLineRef;
}
unsafe extern "C" {
    pub fn CTTypesetterCreateLine(typesetter: CTTypesetterRef, stringRange: CFRange) -> CTLineRef;
}
unsafe extern "C" {
    pub fn CTTypesetterSuggestLineBreakWithOffset(
        typesetter: CTTypesetterRef,
        startIndex: CFIndex,
        width: f64,
        offset: f64,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CTTypesetterSuggestLineBreak(
        typesetter: CTTypesetterRef,
        startIndex: CFIndex,
        width: f64,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CTTypesetterSuggestClusterBreakWithOffset(
        typesetter: CTTypesetterRef,
        startIndex: CFIndex,
        width: f64,
        offset: f64,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CTTypesetterSuggestClusterBreak(
        typesetter: CTTypesetterRef,
        startIndex: CFIndex,
        width: f64,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CTFramesetterGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CTFramesetterCreateWithTypesetter(typesetter: CTTypesetterRef) -> CTFramesetterRef;
}
unsafe extern "C" {
    pub fn CTFramesetterCreateWithAttributedString(
        attrString: CFAttributedStringRef,
    ) -> CTFramesetterRef;
}
unsafe extern "C" {
    pub fn CTFramesetterCreateFrame(
        framesetter: CTFramesetterRef,
        stringRange: CFRange,
        path: CGPathRef,
        frameAttributes: CFDictionaryRef,
    ) -> CTFrameRef;
}
unsafe extern "C" {
    pub fn CTFramesetterGetTypesetter(framesetter: CTFramesetterRef) -> CTTypesetterRef;
}
unsafe extern "C" {
    pub fn CTFramesetterSuggestFrameSizeWithConstraints(
        framesetter: CTFramesetterRef,
        stringRange: CFRange,
        frameAttributes: CFDictionaryRef,
        constraints: CGSize,
        fitRange: *mut CFRange,
    ) -> CGSize;
}
unsafe extern "C" {
    pub fn CTGlyphInfoGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CTGlyphInfoCreateWithGlyphName(
        glyphName: CFStringRef,
        font: CTFontRef,
        baseString: CFStringRef,
    ) -> CTGlyphInfoRef;
}
unsafe extern "C" {
    pub fn CTGlyphInfoCreateWithGlyph(
        glyph: CGGlyph,
        font: CTFontRef,
        baseString: CFStringRef,
    ) -> CTGlyphInfoRef;
}
unsafe extern "C" {
    pub fn CTGlyphInfoCreateWithCharacterIdentifier(
        cid: CGFontIndex,
        collection: CTCharacterCollection,
        baseString: CFStringRef,
    ) -> CTGlyphInfoRef;
}
unsafe extern "C" {
    pub fn CTGlyphInfoGetGlyphName(glyphInfo: CTGlyphInfoRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTGlyphInfoGetGlyph(glyphInfo: CTGlyphInfoRef) -> CGGlyph;
}
unsafe extern "C" {
    pub fn CTGlyphInfoGetCharacterIdentifier(glyphInfo: CTGlyphInfoRef) -> CGFontIndex;
}
unsafe extern "C" {
    pub fn CTGlyphInfoGetCharacterCollection(glyphInfo: CTGlyphInfoRef) -> CTCharacterCollection;
}
unsafe extern "C" {
    pub fn CTParagraphStyleGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CTParagraphStyleCreate(
        settings: *const CTParagraphStyleSetting,
        settingCount: usize,
    ) -> CTParagraphStyleRef;
}
unsafe extern "C" {
    pub fn CTParagraphStyleCreateCopy(paragraphStyle: CTParagraphStyleRef) -> CTParagraphStyleRef;
}
unsafe extern "C" {
    pub fn CTParagraphStyleGetValueForSpecifier(
        paragraphStyle: CTParagraphStyleRef,
        spec: CTParagraphStyleSpecifier,
        valueBufferSize: usize,
        valueBuffer: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CTRubyAnnotationGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CTRubyAnnotationCreate(
        alignment: CTRubyAlignment,
        overhang: CTRubyOverhang,
        sizeFactor: CGFloat,
        text: *mut CFStringRef,
    ) -> CTRubyAnnotationRef;
}
unsafe extern "C" {
    pub static kCTRubyAnnotationSizeFactorAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTRubyAnnotationScaleToFitAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub fn CTRubyAnnotationCreateWithAttributes(
        alignment: CTRubyAlignment,
        overhang: CTRubyOverhang,
        position: CTRubyPosition,
        string: CFStringRef,
        attributes: CFDictionaryRef,
    ) -> CTRubyAnnotationRef;
}
unsafe extern "C" {
    pub fn CTRubyAnnotationCreateCopy(rubyAnnotation: CTRubyAnnotationRef) -> CTRubyAnnotationRef;
}
unsafe extern "C" {
    pub fn CTRubyAnnotationGetAlignment(rubyAnnotation: CTRubyAnnotationRef) -> CTRubyAlignment;
}
unsafe extern "C" {
    pub fn CTRubyAnnotationGetOverhang(rubyAnnotation: CTRubyAnnotationRef) -> CTRubyOverhang;
}
unsafe extern "C" {
    pub fn CTRubyAnnotationGetSizeFactor(rubyAnnotation: CTRubyAnnotationRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CTRubyAnnotationGetTextForPosition(
        rubyAnnotation: CTRubyAnnotationRef,
        position: CTRubyPosition,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CTRunGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CTRunGetGlyphCount(run: CTRunRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CTRunGetAttributes(run: CTRunRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CTRunGetStatus(run: CTRunRef) -> CTRunStatus;
}
unsafe extern "C" {
    pub fn CTRunGetGlyphsPtr(run: CTRunRef) -> *const CGGlyph;
}
unsafe extern "C" {
    pub fn CTRunGetGlyphs(run: CTRunRef, range: CFRange, buffer: *mut CGGlyph);
}
unsafe extern "C" {
    pub fn CTRunGetPositionsPtr(run: CTRunRef) -> *const CGPoint;
}
unsafe extern "C" {
    pub fn CTRunGetPositions(run: CTRunRef, range: CFRange, buffer: *mut CGPoint);
}
unsafe extern "C" {
    pub fn CTRunGetAdvancesPtr(run: CTRunRef) -> *const CGSize;
}
unsafe extern "C" {
    pub fn CTRunGetAdvances(run: CTRunRef, range: CFRange, buffer: *mut CGSize);
}
unsafe extern "C" {
    pub fn CTRunGetStringIndicesPtr(run: CTRunRef) -> *const CFIndex;
}
unsafe extern "C" {
    pub fn CTRunGetStringIndices(run: CTRunRef, range: CFRange, buffer: *mut CFIndex);
}
unsafe extern "C" {
    pub fn CTRunGetStringRange(run: CTRunRef) -> CFRange;
}
unsafe extern "C" {
    pub fn CTRunGetTypographicBounds(
        run: CTRunRef,
        range: CFRange,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> f64;
}
unsafe extern "C" {
    pub fn CTRunGetImageBounds(run: CTRunRef, context: CGContextRef, range: CFRange) -> CGRect;
}
unsafe extern "C" {
    pub fn CTRunGetTextMatrix(run: CTRunRef) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CTRunGetBaseAdvancesAndOrigins(
        runRef: CTRunRef,
        range: CFRange,
        advancesBuffer: *mut CGSize,
        originsBuffer: *mut CGPoint,
    );
}
unsafe extern "C" {
    pub fn CTRunDraw(run: CTRunRef, context: CGContextRef, range: CFRange);
}
unsafe extern "C" {
    pub fn CTRunDelegateGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CTRunDelegateCreate(
        callbacks: *const CTRunDelegateCallbacks,
        refCon: *mut ::std::os::raw::c_void,
    ) -> CTRunDelegateRef;
}
unsafe extern "C" {
    pub fn CTRunDelegateGetRefCon(runDelegate: CTRunDelegateRef) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static kCTFontAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTForegroundColorFromContextAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTKernAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTTrackingAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTLigatureAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTForegroundColorAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBackgroundColorAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTParagraphStyleAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTStrokeWidthAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTStrokeColorAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTUnderlineStyleAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTSuperscriptAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTUnderlineColorAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTVerticalFormsAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTHorizontalInVerticalFormsAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTGlyphInfoAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTCharacterShapeAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTLanguageAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTRunDelegateAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineClassAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineInfoAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineReferenceInfoAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTBaselineOffsetAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTWritingDirectionAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTRubyAnnotationAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCTAdaptiveImageProviderAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub fn CTTextTabGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCTTabColumnTerminatorsAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub fn CTTextTabCreate(
        alignment: CTTextAlignment,
        location: f64,
        options: CFDictionaryRef,
    ) -> CTTextTabRef;
}
unsafe extern "C" {
    pub fn CTTextTabGetAlignment(tab: CTTextTabRef) -> CTTextAlignment;
}
unsafe extern "C" {
    pub fn CTTextTabGetLocation(tab: CTTextTabRef) -> f64;
}
unsafe extern "C" {
    pub fn CTTextTabGetOptions(tab: CTTextTabRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CTGetCoreTextVersion() -> u32;
}

unsafe impl objc2::encode::RefEncode for __CFNumber {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNumber {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNumber", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTFontDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTFontDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTFontDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTFont {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTFont {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTFont", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTFontCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTFontCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTFontCollection", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTFrame {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTFrame {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTFrame", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTLine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTLine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTLine", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTTypesetter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTTypesetter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTTypesetter", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTFramesetter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTFramesetter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTFramesetter", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTGlyphInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTGlyphInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTGlyphInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTParagraphStyle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTParagraphStyle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTParagraphStyle", &[]);
}
unsafe impl objc2::encode::RefEncode for CTParagraphStyleSetting {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTParagraphStyleSetting {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CTParagraphStyleSetting", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTRubyAnnotation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTRubyAnnotation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTRubyAnnotation", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTRun {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTRun {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTRun", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTRunDelegate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTRunDelegate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTRunDelegate", &[]);
}
unsafe impl objc2::encode::RefEncode for CTRunDelegateCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTRunDelegateCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CTRunDelegateCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for __CTTextTab {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CTTextTab {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CTTextTab", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupBinarySearchHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupBinarySearchHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupBinarySearchHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupArrayHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupArrayHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupArrayHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupTrimmedArrayHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupTrimmedArrayHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupTrimmedArrayHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupVectorHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupVectorHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupVectorHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupSegment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupSegment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupSegment", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupSegmentHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupSegmentHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupSegmentHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupSingle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupSingle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupSingle", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupSingleHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupSingleHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupSingleHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupFormatSpecificHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupFormatSpecificHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupFormatSpecificHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for SFNTLookupTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFNTLookupTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SFNTLookupTable", &[]);
}
unsafe impl objc2::encode::RefEncode for STHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for STClassTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STClassTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STClassTable", &[]);
}
unsafe impl objc2::encode::RefEncode for STEntryZero {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STEntryZero {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STEntryZero", &[]);
}
unsafe impl objc2::encode::RefEncode for STEntryOne {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STEntryOne {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STEntryOne", &[]);
}
unsafe impl objc2::encode::RefEncode for STEntryTwo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STEntryTwo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STEntryTwo", &[]);
}
unsafe impl objc2::encode::RefEncode for STXHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STXHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STXHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for STXEntryZero {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STXEntryZero {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STXEntryZero", &[]);
}
unsafe impl objc2::encode::RefEncode for STXEntryOne {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STXEntryOne {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STXEntryOne", &[]);
}
unsafe impl objc2::encode::RefEncode for STXEntryTwo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STXEntryTwo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STXEntryTwo", &[]);
}
unsafe impl objc2::encode::RefEncode for LcarCaretClassEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LcarCaretClassEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LcarCaretClassEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for LcarCaretTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LcarCaretTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LcarCaretTable", &[]);
}
unsafe impl objc2::encode::RefEncode for JustPCDecompositionAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustPCDecompositionAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustPCDecompositionAction", &[]);
}
unsafe impl objc2::encode::RefEncode for JustPCConditionalAddAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustPCConditionalAddAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustPCConditionalAddAction", &[]);
}
unsafe impl objc2::encode::RefEncode for JustPCDuctilityAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustPCDuctilityAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustPCDuctilityAction", &[]);
}
unsafe impl objc2::encode::RefEncode for JustPCGlyphRepeatAddAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustPCGlyphRepeatAddAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustPCGlyphRepeatAddAction", &[]);
}
unsafe impl objc2::encode::RefEncode for JustPCActionSubrecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustPCActionSubrecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustPCActionSubrecord", &[]);
}
unsafe impl objc2::encode::RefEncode for JustPCAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustPCAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustPCAction", &[]);
}
unsafe impl objc2::encode::RefEncode for JustWidthDeltaEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustWidthDeltaEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustWidthDeltaEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for JustWidthDeltaGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustWidthDeltaGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustWidthDeltaGroup", &[]);
}
unsafe impl objc2::encode::RefEncode for JustPostcompTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustPostcompTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustPostcompTable", &[]);
}
unsafe impl objc2::encode::RefEncode for JustDirectionTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustDirectionTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustDirectionTable", &[]);
}
unsafe impl objc2::encode::RefEncode for JustTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JustTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JustTable", &[]);
}
unsafe impl objc2::encode::RefEncode for OpbdSideValues {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpbdSideValues {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpbdSideValues", &[]);
}
unsafe impl objc2::encode::RefEncode for OpbdTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpbdTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpbdTable", &[]);
}
unsafe impl objc2::encode::RefEncode for MortRearrangementSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortRearrangementSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortRearrangementSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MortContextualSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortContextualSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortContextualSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MortLigatureSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortLigatureSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortLigatureSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MortSwashSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortSwashSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortSwashSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MortInsertionSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortInsertionSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortInsertionSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MortSpecificSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortSpecificSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortSpecificSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MortSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MortFeatureEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortFeatureEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortFeatureEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for MortChain {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortChain {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortChain", &[]);
}
unsafe impl objc2::encode::RefEncode for MortTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MortTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MortTable", &[]);
}
unsafe impl objc2::encode::RefEncode for MorxRearrangementSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MorxRearrangementSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MorxRearrangementSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MorxContextualSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MorxContextualSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MorxContextualSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MorxLigatureSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MorxLigatureSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MorxLigatureSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MorxInsertionSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MorxInsertionSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MorxInsertionSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MorxSpecificSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MorxSpecificSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MorxSpecificSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MorxSubtable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MorxSubtable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MorxSubtable", &[]);
}
unsafe impl objc2::encode::RefEncode for MorxChain {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MorxChain {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MorxChain", &[]);
}
unsafe impl objc2::encode::RefEncode for MorxTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MorxTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MorxTable", &[]);
}
unsafe impl objc2::encode::RefEncode for PropTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PropTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PropTable", &[]);
}
unsafe impl objc2::encode::RefEncode for PropLookupSegment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PropLookupSegment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PropLookupSegment", &[]);
}
unsafe impl objc2::encode::RefEncode for PropLookupSingle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PropLookupSingle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PropLookupSingle", &[]);
}
unsafe impl objc2::encode::RefEncode for TrakTableEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TrakTableEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TrakTableEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for TrakTableData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TrakTableData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TrakTableData", &[]);
}
unsafe impl objc2::encode::RefEncode for TrakTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TrakTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TrakTable", &[]);
}
unsafe impl objc2::encode::RefEncode for KernVersion0Header {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernVersion0Header {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernVersion0Header", &[]);
}
unsafe impl objc2::encode::RefEncode for KernTableHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernTableHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernTableHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KernKerningPair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernKerningPair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernKerningPair", &[]);
}
unsafe impl objc2::encode::RefEncode for KernOrderedListEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernOrderedListEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernOrderedListEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for KernOrderedListHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernOrderedListHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernOrderedListHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KernStateHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernStateHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernStateHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KernStateEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernStateEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernStateEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for KernOffsetTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernOffsetTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernOffsetTable", &[]);
}
unsafe impl objc2::encode::RefEncode for KernSimpleArrayHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernSimpleArrayHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernSimpleArrayHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KernIndexArrayHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernIndexArrayHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernIndexArrayHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KernFormatSpecificHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernFormatSpecificHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernFormatSpecificHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KernVersion0SubtableHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernVersion0SubtableHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernVersion0SubtableHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KernSubtableHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KernSubtableHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KernSubtableHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxTableHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxTableHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxTableHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxKerningPair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxKerningPair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxKerningPair", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxOrderedListEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxOrderedListEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxOrderedListEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxOrderedListHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxOrderedListHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxOrderedListHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxStateHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxStateHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxStateHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxStateEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxStateEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxStateEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxControlPointHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxControlPointHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxControlPointHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxControlPointEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxControlPointEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxControlPointEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxControlPointAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxControlPointAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxControlPointAction", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxAnchorPointAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxAnchorPointAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxAnchorPointAction", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxCoordinateAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxCoordinateAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxCoordinateAction", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxSimpleArrayHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxSimpleArrayHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxSimpleArrayHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxIndexArrayHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxIndexArrayHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxIndexArrayHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxFormatSpecificHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxFormatSpecificHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxFormatSpecificHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for KerxSubtableHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for KerxSubtableHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("KerxSubtableHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for BslnFormat0Part {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BslnFormat0Part {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BslnFormat0Part", &[]);
}
unsafe impl objc2::encode::RefEncode for BslnFormat1Part {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BslnFormat1Part {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BslnFormat1Part", &[]);
}
unsafe impl objc2::encode::RefEncode for BslnFormat2Part {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BslnFormat2Part {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BslnFormat2Part", &[]);
}
unsafe impl objc2::encode::RefEncode for BslnFormat3Part {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BslnFormat3Part {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BslnFormat3Part", &[]);
}
unsafe impl objc2::encode::RefEncode for BslnFormatUnion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BslnFormatUnion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BslnFormatUnion", &[]);
}
unsafe impl objc2::encode::RefEncode for BslnTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BslnTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BslnTable", &[]);
}
unsafe impl objc2::encode::RefEncode for ALMXHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALMXHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ALMXHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ALMXGlyphEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALMXGlyphEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ALMXGlyphEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for ROTAHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ROTAHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ROTAHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ROTAGlyphEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ROTAGlyphEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ROTAGlyphEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for AnchorPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AnchorPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AnchorPoint", &[]);
}
unsafe impl objc2::encode::RefEncode for AnchorPointTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AnchorPointTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AnchorPointTable", &[]);
}
unsafe impl objc2::encode::RefEncode for AnkrTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AnkrTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AnkrTable", &[]);
}
unsafe impl objc2::encode::RefEncode for LtagStringRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LtagStringRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LtagStringRange", &[]);
}
unsafe impl objc2::encode::RefEncode for LtagTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LtagTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LtagTable", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntDirectoryEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntDirectoryEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntDirectoryEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntDirectory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntDirectory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntDirectory", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntCMapSubHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntCMapSubHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntCMapSubHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntCMapExtendedSubHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntCMapExtendedSubHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntCMapExtendedSubHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntCMapEncoding {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntCMapEncoding {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntCMapEncoding", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntCMapHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntCMapHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntCMapHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntNameRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntNameRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntNameRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntNameHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntNameHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntNameHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntVariationAxis {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntVariationAxis {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntVariationAxis", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntInstance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntInstance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntInstance", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntVariationHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntVariationHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntVariationHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntFontDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntFontDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntFontDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntDescriptorHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntDescriptorHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntDescriptorHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntFeatureName {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntFeatureName {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntFeatureName", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntFontFeatureSetting {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntFontFeatureSetting {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntFontFeatureSetting", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntFontRunFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntFontRunFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntFontRunFeature", &[]);
}
unsafe impl objc2::encode::RefEncode for sfntFeatureHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sfntFeatureHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sfntFeatureHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for FontVariation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FontVariation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FontVariation", &[]);
}
