#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::ApplicationServices::*;
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
use crate::IOKit::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type PRefCon = *mut ::std::os::raw::c_void;
pub type StyleField = Style;
pub type CMDisplayIDType = UInt32;
pub type HIPoint = CGPoint;
pub type HISize = CGSize;
pub type HIRect = CGRect;
pub type HICoordinateSpace = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventRef {
    _unused: [u8; 0],
}
pub type EventRef = *mut OpaqueEventRef;
pub type EventPriority = SInt16;
pub type EventTime = f64;
pub type EventTimeout = EventTime;
pub type EventTimerInterval = EventTime;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct EventTypeSpec {
    pub eventClass: OSType,
    pub eventKind: UInt32,
}
pub type EventParamName = OSType;
pub type EventParamType = OSType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventLoopRef {
    _unused: [u8; 0],
}
pub type EventLoopRef = *mut OpaqueEventLoopRef;
pub type EventAttributes = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventQueueRef {
    _unused: [u8; 0],
}
pub type EventQueueRef = *mut OpaqueEventQueueRef;
pub type EventComparatorProcPtr = ::std::option::Option<
    unsafe extern "C" fn(inEvent: EventRef, inCompareData: *mut ::std::os::raw::c_void) -> Boolean,
>;
pub type EventComparatorUPP = EventComparatorProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __EventLoopTimer {
    _unused: [u8; 0],
}
pub type EventLoopTimerRef = *mut __EventLoopTimer;
pub type EventLoopTimerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(inTimer: EventLoopTimerRef, inUserData: *mut ::std::os::raw::c_void),
>;
pub type EventLoopIdleTimerMessage = UInt16;
pub type EventLoopIdleTimerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inTimer: EventLoopTimerRef,
        inState: EventLoopIdleTimerMessage,
        inUserData: *mut ::std::os::raw::c_void,
    ),
>;
pub type EventLoopTimerUPP = EventLoopTimerProcPtr;
pub type EventLoopIdleTimerUPP = EventLoopIdleTimerProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventHandlerRef {
    _unused: [u8; 0],
}
pub type EventHandlerRef = *mut OpaqueEventHandlerRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventHandlerCallRef {
    _unused: [u8; 0],
}
pub type EventHandlerCallRef = *mut OpaqueEventHandlerCallRef;
pub type EventHandlerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inHandlerCallRef: EventHandlerCallRef,
        inEvent: EventRef,
        inUserData: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type EventHandlerUPP = EventHandlerProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventTargetRef {
    _unused: [u8; 0],
}
pub type EventTargetRef = *mut OpaqueEventTargetRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueHIObjectClassRef {
    _unused: [u8; 0],
}
pub type HIObjectClassRef = *mut OpaqueHIObjectClassRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueHIObjectRef {
    _unused: [u8; 0],
}
pub type HIObjectRef = *mut OpaqueHIObjectRef;
pub type HIDelegatePosition = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueControlRef {
    _unused: [u8; 0],
}
pub type ControlRef = *mut OpaqueControlRef;
pub type ControlHandle = ControlRef;
pub type HIViewRef = ControlRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueHIArchiveRef {
    _unused: [u8; 0],
}
pub type HIArchiveRef = *mut OpaqueHIArchiveRef;
pub type AppearancePartCode = SInt16;
pub type AppearanceRegionCode = UInt16;
pub type ThemeBrush = SInt16;
pub type ThemeTextColor = SInt16;
pub type ThemeDrawState = UInt32;
pub type ThemeCursor = UInt32;
pub type ThemeMenuBarState = UInt16;
pub type ThemeMenuState = UInt16;
pub type ThemeMenuType = UInt16;
pub type ThemeMenuItemType = UInt16;
pub type ThemeBackgroundKind = UInt32;
pub type ThemeCheckBoxStyle = UInt16;
pub type ThemeScrollBarArrowStyle = UInt16;
pub type ThemeScrollBarThumbStyle = UInt16;
pub type ThemeFontID = UInt16;
pub type ThemeTabStyle = UInt16;
pub type ThemeTabDirection = UInt16;
pub type ThemeTrackKind = UInt16;
pub type ThemeTrackEnableState = UInt8;
pub type ThemeTrackPressState = UInt8;
pub type ThemeThumbDirection = UInt8;
pub type ThemeTrackAttributes = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ScrollBarTrackInfo {
    pub viewsize: SInt32,
    pub pressState: ThemeTrackPressState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SliderTrackInfo {
    pub thumbDir: ThemeThumbDirection,
    pub pressState: ThemeTrackPressState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProgressTrackInfo {
    pub phase: UInt8,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct ThemeTrackDrawInfo {
    pub __bindgen_anon_1: ThemeTrackDrawInfo__bindgen_ty_1,
    pub kind: ThemeTrackKind,
    pub bounds: Rect,
    pub min: SInt32,
    pub max: SInt32,
    pub value: SInt32,
    pub reserved: UInt32,
    pub attributes: ThemeTrackAttributes,
    pub enableState: ThemeTrackEnableState,
    pub filler1: UInt8,
    pub trackInfo: ThemeTrackDrawInfo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ThemeTrackDrawInfo__bindgen_ty_1 {
    pub scrollbar: ScrollBarTrackInfo,
    pub slider: SliderTrackInfo,
    pub progress: ProgressTrackInfo,
}
pub type ThemeWindowAttributes = UInt32;
pub type ThemeWindowType = UInt16;
pub type ThemeTitleBarWidget = UInt16;
pub type ThemeArrowOrientation = UInt16;
pub type ThemePopupArrowSize = UInt16;
pub type ThemeGrowDirection = UInt16;
pub type ThemeButtonKind = UInt16;
pub type ThemeButtonValue = UInt16;
pub type ThemeButtonAdornment = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ThemeButtonDrawInfo {
    pub state: ThemeDrawState,
    pub value: ThemeButtonValue,
    pub adornment: ThemeButtonAdornment,
}
pub type ThemeButtonDrawInfoPtr = *mut ThemeButtonDrawInfo;
pub type ThemeDragSoundKind = OSType;
pub type ThemeSoundKind = OSType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ThemeWindowMetrics {
    pub metricSize: UInt16,
    pub titleHeight: SInt16,
    pub titleWidth: SInt16,
    pub popupTabOffset: SInt16,
    pub popupTabWidth: SInt16,
    pub popupTabPosition: UInt16,
}
pub type ThemeWindowMetricsPtr = *mut ThemeWindowMetrics;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueThemeDrawingState {
    _unused: [u8; 0],
}
pub type ThemeDrawingState = *mut OpaqueThemeDrawingState;
pub type ThemeTabTitleDrawProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        bounds: *const Rect,
        style: ThemeTabStyle,
        direction: ThemeTabDirection,
        depth: SInt16,
        isColorDev: Boolean,
        userData: URefCon,
    ),
>;
pub type ThemeEraseProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        bounds: *const Rect,
        eraseData: URefCon,
        depth: SInt16,
        isColorDev: Boolean,
    ),
>;
pub type ThemeButtonDrawProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        bounds: *const Rect,
        kind: ThemeButtonKind,
        info: *const ThemeButtonDrawInfo,
        userData: URefCon,
        depth: SInt16,
        isColorDev: Boolean,
    ),
>;
pub type WindowTitleDrawingProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        bounds: *const Rect,
        depth: SInt16,
        colorDevice: Boolean,
        userData: URefCon,
    ),
>;
pub type ThemeIteratorProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inFileName: ConstStr255Param,
        resID: SInt16,
        inThemeSettings: Collection,
        inUserData: PRefCon,
    ) -> Boolean,
>;
pub type ThemeTabTitleDrawUPP = ThemeTabTitleDrawProcPtr;
pub type ThemeEraseUPP = ThemeEraseProcPtr;
pub type ThemeButtonDrawUPP = ThemeButtonDrawProcPtr;
pub type WindowTitleDrawingUPP = WindowTitleDrawingProcPtr;
pub type ThemeIteratorUPP = ThemeIteratorProcPtr;
pub type MenuTitleDrawingProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inBounds: *const Rect,
        inDepth: SInt16,
        inIsColorDevice: Boolean,
        inUserData: SRefCon,
    ),
>;
pub type MenuItemDrawingProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inBounds: *const Rect,
        inDepth: SInt16,
        inIsColorDevice: Boolean,
        inUserData: SRefCon,
    ),
>;
pub type MenuTitleDrawingUPP = MenuTitleDrawingProcPtr;
pub type MenuItemDrawingUPP = MenuItemDrawingProcPtr;
pub type EventKind = UInt16;
pub type EventMask = UInt16;
pub type EventModifiers = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct EventRecord {
    pub what: EventKind,
    pub message: ::std::os::raw::c_ulong,
    pub when: UInt32,
    pub where_: Point,
    pub modifiers: EventModifiers,
}
pub type FKEYProcPtr = ::std::option::Option<unsafe extern "C" fn()>;
pub type FKEYUPP = FKEYProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct EvQEl {
    pub qLink: QElemPtr,
    pub qType: SInt16,
    pub evtQWhat: EventKind,
    pub evtQMessage: ::std::os::raw::c_ulong,
    pub evtQWhen: UInt32,
    pub evtQWhere: Point,
    pub evtQModifiers: EventModifiers,
}
pub type EvQElPtr = *mut EvQEl;
pub type GetNextEventFilterProcPtr =
    ::std::option::Option<unsafe extern "C" fn(theEvent: *mut EventRecord, result: *mut Boolean)>;
pub type GetNextEventFilterUPP = GetNextEventFilterProcPtr;
pub type GNEFilterUPP = GetNextEventFilterUPP;
pub type MenuAttributes = UInt32;
pub type MenuItemAttributes = UInt32;
pub type MenuTrackingMode = UInt32;
pub type MenuEventOptions = UInt32;
pub type MenuID = SInt16;
pub type MenuItemIndex = UInt16;
pub type MenuCommand = OSType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMenuRef {
    _unused: [u8; 0],
}
pub type MenuRef = *mut OpaqueMenuRef;
pub type MenuHandle = MenuRef;
pub type MenuBarHandle = Handle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MenuBarHeader {
    pub lastMenu: UInt16,
    pub lastRight: SInt16,
    pub mbResID: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HMenuBarHeader {
    pub lastHMenu: UInt16,
    pub menuTitleBits: PixMapHandle,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MenuBarMenu {
    pub menu: MenuRef,
    pub menuLeft: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HMenuBarMenu {
    pub menu: MenuRef,
    pub reserved: SInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MenuTrackingData {
    pub menu: MenuRef,
    pub itemSelected: MenuItemIndex,
    pub itemUnderMouse: MenuItemIndex,
    pub itemRect: Rect,
    pub virtualMenuTop: SInt32,
    pub virtualMenuBottom: SInt32,
}
pub type MenuTrackingDataPtr = *mut MenuTrackingData;
pub type MenuItemDataFlags = UInt64;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MenuItemDataRec {
    pub whichData: MenuItemDataFlags,
    pub text: StringPtr,
    pub mark: UniChar,
    pub cmdKey: UniChar,
    pub cmdKeyGlyph: UInt32,
    pub cmdKeyModifiers: UInt32,
    pub style: Style,
    pub enabled: Boolean,
    pub iconEnabled: Boolean,
    pub filler1: UInt8,
    pub iconID: SInt32,
    pub iconType: UInt32,
    pub iconHandle: Handle,
    pub cmdID: MenuCommand,
    pub encoding: TextEncoding,
    pub submenuID: MenuID,
    pub submenuHandle: MenuRef,
    pub fontID: SInt32,
    pub refcon: URefCon,
    pub attr: OptionBits,
    pub cfText: CFStringRef,
    pub properties: Collection,
    pub indent: UInt32,
    pub cmdVirtualKey: UInt16,
    pub attributedText: CFAttributedStringRef,
    pub font: CTFontRef,
}
pub type MenuItemDataPtr = *mut MenuItemDataRec;
pub type MenuItemID = UInt32;
pub type MenuDefType = UInt32;
pub type MenuDefUPP = *mut ::std::os::raw::c_void;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct MenuDefSpec {
    pub __bindgen_anon_1: MenuDefSpec__bindgen_ty_1,
    pub defType: MenuDefType,
    pub u: MenuDefSpec__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union MenuDefSpec__bindgen_ty_1 {
    pub __bindgen_anon_1: MenuDefSpec__bindgen_ty_1__bindgen_ty_1,
    pub defProc: MenuDefUPP,
    pub view: MenuDefSpec__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MenuDefSpec__bindgen_ty_1__bindgen_ty_1 {
    pub classID: CFStringRef,
    pub initEvent: EventRef,
}
pub type MenuDefSpecPtr = *mut MenuDefSpec;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ContextualMenuInterfaceStruct {
    pub _reserved: *mut ::std::os::raw::c_void,
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            thisPointer: *mut ::std::os::raw::c_void,
            iid: CFUUIDBytes,
            ppv: *mut *mut ::std::os::raw::c_void,
        ) -> SInt32,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(thisPointer: *mut ::std::os::raw::c_void) -> UInt32,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(thisPointer: *mut ::std::os::raw::c_void) -> UInt32,
    >,
    pub ExamineContext: ::std::option::Option<
        unsafe extern "C" fn(
            thisInstance: *mut ::std::os::raw::c_void,
            inContext: *const AEDesc,
            outCommandPairs: *mut AEDescList,
        ) -> OSStatus,
    >,
    pub HandleSelection: ::std::option::Option<
        unsafe extern "C" fn(
            thisInstance: *mut ::std::os::raw::c_void,
            inContext: *mut AEDesc,
            inCommandID: SInt32,
        ) -> OSStatus,
    >,
    pub PostMenuCleanup:
        ::std::option::Option<unsafe extern "C" fn(thisInstance: *mut ::std::os::raw::c_void)>,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MDEFHiliteItemData {
    pub previousItem: MenuItemIndex,
    pub newItem: MenuItemIndex,
    pub context: *mut ::std::os::raw::c_void,
}
pub type MDEFHiliteItemDataPtr = *mut MDEFHiliteItemData;
pub type HiliteMenuItemData = MDEFHiliteItemData;
pub type HiliteMenuItemDataPtr = MDEFHiliteItemDataPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MDEFDrawData {
    pub trackingData: MenuTrackingData,
    pub context: *mut ::std::os::raw::c_void,
}
pub type MDEFDrawDataPtr = *mut MDEFDrawData;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MDEFFindItemData {
    pub trackingData: MenuTrackingData,
    pub context: *mut ::std::os::raw::c_void,
}
pub type MDEFFindItemDataPtr = *mut MDEFFindItemData;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct MDEFDrawItemsData {
    pub firstItem: MenuItemIndex,
    pub lastItem: MenuItemIndex,
    pub trackingData: *mut MenuTrackingData,
    pub context: *mut ::std::os::raw::c_void,
}
pub type MDEFDrawItemsDataPtr = *mut MDEFDrawItemsData;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MCEntry {
    pub mctID: MenuID,
    pub mctItem: ::std::os::raw::c_short,
    pub mctRGB1: RGBColor,
    pub mctRGB2: RGBColor,
    pub mctRGB3: RGBColor,
    pub mctRGB4: RGBColor,
    pub mctReserved: ::std::os::raw::c_short,
}
pub type MCEntryPtr = *mut MCEntry;
pub type MCTable = [MCEntry; 1usize];
pub type MCTablePtr = *mut MCEntry;
pub type MCTableHandle = *mut MCTablePtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MenuCRsrc {
    pub numEntries: ::std::os::raw::c_short,
    pub mcEntryRecs: MCTable,
}
pub type MenuCRsrcPtr = *mut MenuCRsrc;
pub type MenuCRsrcHandle = *mut MenuCRsrcPtr;
pub type MenuBarDefProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        selector: ::std::os::raw::c_short,
        message: ::std::os::raw::c_short,
        parameter1: ::std::os::raw::c_short,
        parameter2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long,
>;
pub type MenuHookProcPtr = ::std::option::Option<unsafe extern "C" fn()>;
pub type MBarHookProcPtr =
    ::std::option::Option<unsafe extern "C" fn(menuRect: *mut Rect) -> ::std::os::raw::c_short>;
pub type MenuBarDefUPP = MenuBarDefProcPtr;
pub type MenuHookUPP = MenuHookProcPtr;
pub type MBarHookUPP = MBarHookProcPtr;
pub type HIToolbarRef = HIObjectRef;
pub type HIToolbarItemRef = HIObjectRef;
pub type HIToolbarDisplayMode = UInt32;
pub type HIToolbarDisplaySize = UInt32;
pub type TEPtr = *mut TERec;
pub type TEHandle = *mut TEPtr;
pub type HighHookProcPtr = ::std::option::Option<unsafe extern "C" fn(r: *const Rect, pTE: TEPtr)>;
pub type EOLHookProcPtr = ::std::option::Option<
    unsafe extern "C" fn(theChar: ::std::os::raw::c_char, pTE: TEPtr, hTE: TEHandle) -> Boolean,
>;
pub type CaretHookProcPtr = ::std::option::Option<unsafe extern "C" fn(r: *const Rect, pTE: TEPtr)>;
pub type WidthHookProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        textLen: ::std::os::raw::c_ushort,
        textOffset: ::std::os::raw::c_ushort,
        textBufferPtr: *mut ::std::os::raw::c_void,
        pTE: TEPtr,
        hTE: TEHandle,
    ) -> ::std::os::raw::c_ushort,
>;
pub type TextWidthHookProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        textLen: ::std::os::raw::c_ushort,
        textOffset: ::std::os::raw::c_ushort,
        textBufferPtr: *mut ::std::os::raw::c_void,
        pTE: TEPtr,
        hTE: TEHandle,
    ) -> ::std::os::raw::c_ushort,
>;
pub type NWidthHookProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        styleRunLen: ::std::os::raw::c_ushort,
        styleRunOffset: ::std::os::raw::c_ushort,
        slop: ::std::os::raw::c_short,
        direction: ::std::os::raw::c_short,
        textBufferPtr: *mut ::std::os::raw::c_void,
        lineStart: *mut ::std::os::raw::c_short,
        pTE: TEPtr,
        hTE: TEHandle,
    ) -> ::std::os::raw::c_ushort,
>;
pub type DrawHookProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        textOffset: ::std::os::raw::c_ushort,
        drawLen: ::std::os::raw::c_ushort,
        textBufferPtr: *mut ::std::os::raw::c_void,
        pTE: TEPtr,
        hTE: TEHandle,
    ),
>;
pub type HitTestHookProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        styleRunLen: ::std::os::raw::c_ushort,
        styleRunOffset: ::std::os::raw::c_ushort,
        slop: ::std::os::raw::c_ushort,
        textBufferPtr: *mut ::std::os::raw::c_void,
        pTE: TEPtr,
        hTE: TEHandle,
        pixelWidth: *mut ::std::os::raw::c_ushort,
        charOffset: *mut ::std::os::raw::c_ushort,
        pixelInChar: *mut Boolean,
    ) -> Boolean,
>;
pub type TEFindWordProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        currentPos: ::std::os::raw::c_ushort,
        caller: ::std::os::raw::c_short,
        pTE: TEPtr,
        hTE: TEHandle,
        wordStart: *mut ::std::os::raw::c_ushort,
        wordEnd: *mut ::std::os::raw::c_ushort,
    ),
>;
pub type TERecalcProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        pTE: TEPtr,
        changeLength: ::std::os::raw::c_ushort,
        lineStart: *mut ::std::os::raw::c_ushort,
        firstChar: *mut ::std::os::raw::c_ushort,
        lastChar: *mut ::std::os::raw::c_ushort,
    ),
>;
pub type TEDoTextProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        pTE: TEPtr,
        firstChar: ::std::os::raw::c_ushort,
        lastChar: ::std::os::raw::c_ushort,
        selector: ::std::os::raw::c_short,
        currentGrafPort: *mut GrafPtr,
        charPosition: *mut ::std::os::raw::c_short,
    ),
>;
pub type TEClickLoopProcPtr = ::std::option::Option<unsafe extern "C" fn(pTE: TEPtr) -> Boolean>;
pub type WordBreakProcPtr = ::std::option::Option<
    unsafe extern "C" fn(text: Ptr, charPos: ::std::os::raw::c_short) -> Boolean,
>;
pub type HighHookUPP = HighHookProcPtr;
pub type EOLHookUPP = EOLHookProcPtr;
pub type CaretHookUPP = CaretHookProcPtr;
pub type WidthHookUPP = WidthHookProcPtr;
pub type TextWidthHookUPP = TextWidthHookProcPtr;
pub type NWidthHookUPP = NWidthHookProcPtr;
pub type DrawHookUPP = DrawHookProcPtr;
pub type HitTestHookUPP = HitTestHookProcPtr;
pub type TEFindWordUPP = TEFindWordProcPtr;
pub type TERecalcUPP = TERecalcProcPtr;
pub type TEDoTextUPP = TEDoTextProcPtr;
pub type TEClickLoopUPP = TEClickLoopProcPtr;
pub type WordBreakUPP = WordBreakProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TERec {
    pub destRect: Rect,
    pub viewRect: Rect,
    pub selRect: Rect,
    pub lineHeight: ::std::os::raw::c_short,
    pub fontAscent: ::std::os::raw::c_short,
    pub selPoint: Point,
    pub selStart: ::std::os::raw::c_short,
    pub selEnd: ::std::os::raw::c_short,
    pub active: ::std::os::raw::c_short,
    pub wordBreak: WordBreakUPP,
    pub clickLoop: TEClickLoopUPP,
    pub clickTime: ::std::os::raw::c_long,
    pub clickLoc: ::std::os::raw::c_short,
    pub caretTime: ::std::os::raw::c_long,
    pub caretState: ::std::os::raw::c_short,
    pub just: ::std::os::raw::c_short,
    pub teLength: ::std::os::raw::c_short,
    pub hText: Handle,
    pub hDispatchRec: ::std::os::raw::c_long,
    pub clikStuff: ::std::os::raw::c_short,
    pub crOnly: ::std::os::raw::c_short,
    pub txFont: ::std::os::raw::c_short,
    pub txFace: StyleField,
    pub txMode: ::std::os::raw::c_short,
    pub txSize: ::std::os::raw::c_short,
    pub inPort: GrafPtr,
    pub highHook: HighHookUPP,
    pub caretHook: CaretHookUPP,
    pub nLines: ::std::os::raw::c_short,
    pub lineStarts: [::std::os::raw::c_short; 16001usize],
}
pub type CharsPtr = *mut ::std::os::raw::c_char;
pub type CharsHandle = *mut CharsPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleRun {
    pub startChar: ::std::os::raw::c_short,
    pub styleIndex: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STElement {
    pub stCount: ::std::os::raw::c_short,
    pub stHeight: ::std::os::raw::c_short,
    pub stAscent: ::std::os::raw::c_short,
    pub stFont: ::std::os::raw::c_short,
    pub stFace: StyleField,
    pub stSize: ::std::os::raw::c_short,
    pub stColor: RGBColor,
}
pub type STPtr = *mut STElement;
pub type STHandle = *mut STPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LHElement {
    pub lhHeight: ::std::os::raw::c_short,
    pub lhAscent: ::std::os::raw::c_short,
}
pub type LHPtr = *mut LHElement;
pub type LHHandle = *mut LHPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ScrpSTElement {
    pub scrpStartChar: SInt32,
    pub scrpHeight: SInt16,
    pub scrpAscent: SInt16,
    pub scrpFont: SInt16,
    pub scrpFace: StyleField,
    pub scrpSize: SInt16,
    pub scrpColor: RGBColor,
}
pub type ScrpSTTable = [ScrpSTElement; 1601usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StScrpRec {
    pub scrpNStyles: ::std::os::raw::c_short,
    pub scrpStyleTab: ScrpSTTable,
}
pub type StScrpPtr = *mut StScrpRec;
pub type StScrpHandle = *mut StScrpPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct NullStRec {
    pub teReserved: ::std::os::raw::c_long,
    pub nullScrap: StScrpHandle,
}
pub type NullStPtr = *mut NullStRec;
pub type NullStHandle = *mut NullStPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TEStyleRec {
    pub nRuns: ::std::os::raw::c_short,
    pub nStyles: ::std::os::raw::c_short,
    pub styleTab: STHandle,
    pub lhTab: LHHandle,
    pub teRefCon: ::std::os::raw::c_long,
    pub nullStyle: NullStHandle,
    pub runs: [StyleRun; 8001usize],
}
pub type TEStylePtr = *mut TEStyleRec;
pub type TEStyleHandle = *mut TEStylePtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TextStyle {
    pub tsFont: ::std::os::raw::c_short,
    pub tsFace: StyleField,
    pub tsSize: ::std::os::raw::c_short,
    pub tsColor: RGBColor,
}
pub type TextStylePtr = *mut TextStyle;
pub type TextStyleHandle = *mut TextStylePtr;
pub type TEIntHook = ::std::os::raw::c_short;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueDragRef {
    _unused: [u8; 0],
}
pub type DragRef = *mut OpaqueDragRef;
pub type DragItemRef = URefCon;
pub type FlavorType = OSType;
pub type DragAttributes = OptionBits;
pub type DragBehaviors = OptionBits;
pub type DragImageFlags = OptionBits;
pub type FlavorFlags = UInt32;
pub type DragActions = OptionBits;
pub type DragInputProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        mouse: *mut Point,
        modifiers: *mut SInt16,
        dragInputRefCon: *mut ::std::os::raw::c_void,
        theDrag: DragRef,
    ) -> OSErr,
>;
pub type DragInputUPP = DragInputProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HFSFlavor {
    pub fileType: OSType,
    pub fileCreator: OSType,
    pub fdFlags: UInt16,
    pub fileSpec: FSSpec,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct PromiseHFSFlavor {
    pub fileType: OSType,
    pub fileCreator: OSType,
    pub fdFlags: UInt16,
    pub promisedFlavor: FlavorType,
}
pub type DragTrackingMessage = SInt16;
pub type DragRegionMessage = SInt16;
pub type ZoomAcceleration = SInt16;
pub type StandardDropLocation = OSType;
pub type DragSendDataProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theType: FlavorType,
        dragSendRefCon: *mut ::std::os::raw::c_void,
        theItemRef: DragItemRef,
        theDrag: DragRef,
    ) -> OSErr,
>;
pub type DragTrackingHandlerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        message: DragTrackingMessage,
        theWindow: WindowRef,
        handlerRefCon: *mut ::std::os::raw::c_void,
        theDrag: DragRef,
    ) -> OSErr,
>;
pub type DragReceiveHandlerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theWindow: WindowRef,
        handlerRefCon: *mut ::std::os::raw::c_void,
        theDrag: DragRef,
    ) -> OSErr,
>;
pub type DragDrawingProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        message: DragRegionMessage,
        showRegion: RgnHandle,
        showOrigin: Point,
        hideRegion: RgnHandle,
        hideOrigin: Point,
        dragDrawingRefCon: *mut ::std::os::raw::c_void,
        theDrag: DragRef,
    ) -> OSErr,
>;
pub type DragSendDataUPP = DragSendDataProcPtr;
pub type DragTrackingHandlerUPP = DragTrackingHandlerProcPtr;
pub type DragReceiveHandlerUPP = DragReceiveHandlerProcPtr;
pub type DragDrawingUPP = DragDrawingProcPtr;
pub type DragReference = DragRef;
pub type ItemReference = DragItemRef;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ControlTemplate {
    pub controlRect: Rect,
    pub controlValue: SInt16,
    pub controlVisible: Boolean,
    pub fill: UInt8,
    pub controlMaximum: SInt16,
    pub controlMinimum: SInt16,
    pub controlDefProcID: SInt16,
    pub controlReference: SInt32,
    pub controlTitle: Str255,
}
pub type ControlTemplatePtr = *mut ControlTemplate;
pub type ControlTemplateHandle = *mut ControlTemplatePtr;
pub type ControlPartCode = SInt16;
pub type ControlActionProcPtr =
    ::std::option::Option<unsafe extern "C" fn(theControl: ControlRef, partCode: ControlPartCode)>;
pub type ControlActionUPP = ControlActionProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CtlCTab {
    pub ccSeed: SInt32,
    pub ccRider: SInt16,
    pub ctSize: SInt16,
    pub ctTable: [ColorSpec; 4usize],
}
pub type CCTabPtr = *mut CtlCTab;
pub type CCTabHandle = *mut CCTabPtr;
pub type ControlVariant = SInt16;
pub type ControlFocusPart = ControlPartCode;
pub type ControlContentType = SInt16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ControlImageContentInfo {
    pub __bindgen_anon_1: ControlImageContentInfo__bindgen_ty_1,
    pub contentType: ControlContentType,
    pub u: ControlImageContentInfo__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union ControlImageContentInfo__bindgen_ty_1 {
    pub resID: SInt16,
    pub iconRef: IconRef,
    pub imageRef: CGImageRef,
}
pub type ControlImageContentInfoPtr = *mut ControlImageContentInfo;
pub type ControlButtonContentInfo = ControlImageContentInfo;
pub type ControlButtonContentInfoPtr = *mut ControlButtonContentInfo;
pub type ControlKeyScriptBehavior = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControlFontStyleRec {
    pub flags: SInt16,
    pub font: SInt16,
    pub size: SInt16,
    pub style: SInt16,
    pub mode: SInt16,
    pub just: SInt16,
    pub foreColor: RGBColor,
    pub backColor: RGBColor,
}
pub type ControlFontStylePtr = *mut ControlFontStyleRec;
pub type ClickActivationResult = UInt32;
pub type ControlSize = UInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndicatorDragConstraint {
    pub limitRect: Rect,
    pub slopRect: Rect,
    pub axis: DragConstraint,
}
pub type IndicatorDragConstraintPtr = *mut IndicatorDragConstraint;
pub type ControlKeyFilterResult = SInt16;
pub type ControlKeyFilterProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theControl: ControlRef,
        keyCode: *mut SInt16,
        charCode: *mut SInt16,
        modifiers: *mut EventModifiers,
    ) -> ControlKeyFilterResult,
>;
pub type ControlKeyFilterUPP = ControlKeyFilterProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ControlID {
    pub signature: OSType,
    pub id: SInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ControlKind {
    pub signature: OSType,
    pub kind: OSType,
}
pub type HIWindowRef = WindowRef;
pub type PropertyCreator = OSType;
pub type PropertyTag = OSType;
pub type WindowClass = UInt32;
pub type WindowAttributes = OptionBits;
pub type WindowPositionMethod = UInt32;
pub type WindowRegionCode = UInt16;
pub type WindowPartCode = SInt16;
pub type WindowDefPartCode = SInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WStateData {
    pub userState: Rect,
    pub stdState: Rect,
}
pub type WStateDataPtr = *mut WStateData;
pub type WStateDataHandle = *mut WStateDataPtr;
pub type WindowDefUPP = *mut ::std::os::raw::c_void;
pub type WindowDefType = UInt32;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct WindowDefSpec {
    pub __bindgen_anon_1: WindowDefSpec__bindgen_ty_1,
    pub defType: WindowDefType,
    pub u: WindowDefSpec__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union WindowDefSpec__bindgen_ty_1 {
    pub defProc: WindowDefUPP,
    pub classRef: *mut ::std::os::raw::c_void,
    pub procID: ::std::os::raw::c_short,
    pub rootView: *mut ::std::os::raw::c_void,
}
pub type WindowDefSpecPtr = *mut WindowDefSpec;
pub type HIWindowBackingLocation = UInt32;
pub type HIWindowSharingType = UInt32;
pub type WindowModality = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueWindowGroupRef {
    _unused: [u8; 0],
}
pub type WindowGroupRef = *mut OpaqueWindowGroupRef;
pub type WindowGroupAttributes = UInt32;
pub type WindowActivationScope = UInt32;
pub type WindowGroupContentOptions = UInt32;
pub type HIWindowDepth = UInt32;
pub type ScrollWindowOptions = UInt32;
pub type WindowTransitionEffect = UInt32;
pub type WindowTransitionAction = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TransitionWindowOptions {
    pub version: UInt32,
    pub duration: EventTime,
    pub window: WindowRef,
    pub userData: *mut ::std::os::raw::c_void,
}
pub type WindowConstrainOptions = UInt32;
pub type WindowLatentVisibility = UInt32;
pub type HIWindowAvailability = OptionBits;
pub type WindowDrawerState = UInt32;
pub type HIWindowScaleMode = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIContentBorderMetrics {
    pub top: CGFloat,
    pub left: CGFloat,
    pub bottom: CGFloat,
    pub right: CGFloat,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SetupWindowProxyDragImageRec {
    pub imageGWorld: GWorldPtr,
    pub imageRgn: RgnHandle,
    pub outlineRgn: RgnHandle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MeasureWindowTitleRec {
    pub fullTitleWidth: SInt16,
    pub titleTextWidth: SInt16,
    pub isUnicodeTitle: Boolean,
    pub unused: Boolean,
}
pub type MeasureWindowTitleRecPtr = *mut MeasureWindowTitleRec;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct GetGrowImageRegionRec {
    pub growRect: Rect,
    pub growImageRegion: RgnHandle,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct GetWindowRegionRec {
    pub winRgn: RgnHandle,
    pub regionCode: WindowRegionCode,
}
pub type GetWindowRegionPtr = *mut GetWindowRegionRec;
pub type GetWindowRegionRecPtr = *mut GetWindowRegionRec;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct WinCTab {
    pub wCSeed: ::std::os::raw::c_long,
    pub wCReserved: ::std::os::raw::c_short,
    pub ctSize: ::std::os::raw::c_short,
    pub ctTable: [ColorSpec; 5usize],
}
pub type WCTabPtr = *mut WinCTab;
pub type WCTabHandle = *mut WCTabPtr;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct BasicWindowDescription {
    pub __bindgen_anon_1: BasicWindowDescription__bindgen_ty_1,
    pub descriptionSize: UInt32,
    pub windowContentRect: Rect,
    pub windowZoomRect: Rect,
    pub windowRefCon: URefCon,
    pub windowStateFlags: UInt32,
    pub windowPositionMethod: WindowPositionMethod,
    pub windowDefinitionVersion: UInt32,
    pub windowDefinition: BasicWindowDescription__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union BasicWindowDescription__bindgen_ty_1 {
    pub __bindgen_anon_1: BasicWindowDescription__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: BasicWindowDescription__bindgen_ty_1__bindgen_ty_2,
    pub versionOne: BasicWindowDescription__bindgen_ty_1__bindgen_ty_1,
    pub versionTwo: BasicWindowDescription__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BasicWindowDescription__bindgen_ty_1__bindgen_ty_1 {
    pub windowDefProc: SInt16,
    pub windowHasCloseBox: Boolean,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct BasicWindowDescription__bindgen_ty_1__bindgen_ty_2 {
    pub windowClass: WindowClass,
    pub windowAttributes: WindowAttributes,
}
pub type DeskHookProcPtr =
    ::std::option::Option<unsafe extern "C" fn(mouseClick: Boolean, theEvent: *mut EventRecord)>;
pub type WindowPaintProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        device: GDHandle,
        qdContext: GrafPtr,
        window: WindowRef,
        inClientPaintRgn: RgnHandle,
        outSystemPaintRgn: RgnHandle,
        refCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type DeskHookUPP = DeskHookProcPtr;
pub type WindowPaintUPP = WindowPaintProcPtr;
pub type WindowPaintProcOptions = OptionBits;
pub type MouseTrackingResult = UInt16;
pub type EventMouseButton = UInt16;
pub type EventMouseWheelAxis = UInt16;
pub type TSMDocAccessAttributes = UInt32;
pub type HIModalClickResult = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HICommand {
    pub __bindgen_anon_1: HICommand__bindgen_ty_1,
    pub attributes: UInt32,
    pub commandID: UInt32,
    pub menu: HICommand__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HICommand__bindgen_ty_1 {
    pub menuRef: MenuRef,
    pub menuItemIndex: MenuItemIndex,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct HICommandExtended {
    pub __bindgen_anon_1: HICommandExtended__bindgen_ty_1,
    pub attributes: UInt32,
    pub commandID: UInt32,
    pub source: HICommandExtended__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union HICommandExtended__bindgen_ty_1 {
    pub __bindgen_anon_1: HICommandExtended__bindgen_ty_1__bindgen_ty_1,
    pub control: ControlRef,
    pub window: WindowRef,
    pub menu: HICommandExtended__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HICommandExtended__bindgen_ty_1__bindgen_ty_1 {
    pub menuRef: MenuRef,
    pub menuItemIndex: MenuItemIndex,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TabletPointRec {
    pub absX: SInt32,
    pub absY: SInt32,
    pub absZ: SInt32,
    pub buttons: UInt16,
    pub pressure: UInt16,
    pub tiltX: SInt16,
    pub tiltY: SInt16,
    pub rotation: UInt16,
    pub tangentialPressure: SInt16,
    pub deviceID: UInt16,
    pub vendor1: SInt16,
    pub vendor2: SInt16,
    pub vendor3: SInt16,
}
pub type TabletPointerRec = TabletPointRec;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TabletProximityRec {
    pub vendorID: UInt16,
    pub tabletID: UInt16,
    pub pointerID: UInt16,
    pub deviceID: UInt16,
    pub systemTabletID: UInt16,
    pub vendorPointerType: UInt16,
    pub pointerSerialNumber: UInt32,
    pub uniqueID: UInt64,
    pub capabilityMask: UInt32,
    pub pointerType: UInt8,
    pub enterProximity: UInt8,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct EventHotKeyID {
    pub signature: OSType,
    pub id: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventHotKeyRef {
    _unused: [u8; 0],
}
pub type EventHotKeyRef = *mut OpaqueEventHotKeyRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueToolboxObjectClassRef {
    _unused: [u8; 0],
}
pub type ToolboxObjectClassRef = *mut OpaqueToolboxObjectClassRef;
pub type EventClassID = UInt32;
pub type EventClass = UInt32;
pub type EventType = UInt32;
pub type HIThemeOrientation = UInt32;
pub type HIThemeSplitterAdornment = UInt32;
pub type HIThemeGrowBoxKind = UInt32;
pub type HIThemeGrowBoxSize = UInt32;
pub type HIThemeGroupBoxKind = UInt32;
pub type HIThemeHeaderKind = UInt32;
pub type HIThemeFrameKind = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIScrollBarTrackInfo {
    pub version: UInt32,
    pub enableState: ThemeTrackEnableState,
    pub pressState: ThemeTrackPressState,
    pub viewsize: CGFloat,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct HIThemeTrackDrawInfo {
    pub __bindgen_anon_1: HIThemeTrackDrawInfo__bindgen_ty_1,
    pub version: UInt32,
    pub kind: ThemeTrackKind,
    pub bounds: HIRect,
    pub min: SInt32,
    pub max: SInt32,
    pub value: SInt32,
    pub reserved: UInt32,
    pub attributes: ThemeTrackAttributes,
    pub enableState: ThemeTrackEnableState,
    pub filler1: UInt8,
    pub trackInfo: HIThemeTrackDrawInfo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union HIThemeTrackDrawInfo__bindgen_ty_1 {
    pub scrollbar: ScrollBarTrackInfo,
    pub slider: SliderTrackInfo,
    pub progress: ProgressTrackInfo,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeAnimationTimeInfo {
    pub start: CFAbsoluteTime,
    pub current: CFAbsoluteTime,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeAnimationFrameInfo {
    pub index: UInt32,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct HIThemeButtonDrawInfo {
    pub __bindgen_anon_1: HIThemeButtonDrawInfo__bindgen_ty_1,
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub kind: ThemeButtonKind,
    pub value: ThemeButtonValue,
    pub adornment: ThemeButtonAdornment,
    pub animation: HIThemeButtonDrawInfo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union HIThemeButtonDrawInfo__bindgen_ty_1 {
    pub time: HIThemeAnimationTimeInfo,
    pub frame: HIThemeAnimationFrameInfo,
}
pub type HIThemeButtonDrawInfoPtr = *mut HIThemeButtonDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeSplitterDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub adornment: HIThemeSplitterAdornment,
}
pub type HIThemeSplitterDrawInfoPtr = *mut HIThemeSplitterDrawInfo;
pub type HIThemeTabAdornment = UInt32;
pub type HIThemeTabSize = UInt32;
pub type HIThemeTabPosition = UInt32;
pub type HIThemeTabKind = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeTabDrawInfo {
    pub version: UInt32,
    pub style: ThemeTabStyle,
    pub direction: ThemeTabDirection,
    pub size: HIThemeTabSize,
    pub adornment: HIThemeTabAdornment,
    pub kind: HIThemeTabKind,
    pub position: HIThemeTabPosition,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeTabDrawInfoVersionZero {
    pub version: UInt32,
    pub style: ThemeTabStyle,
    pub direction: ThemeTabDirection,
    pub size: HIThemeTabSize,
    pub adornment: HIThemeTabAdornment,
}
pub type HIThemeTabPaneAdornment = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeTabPaneDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub direction: ThemeTabDirection,
    pub size: HIThemeTabSize,
    pub kind: HIThemeTabKind,
    pub adornment: HIThemeTabPaneAdornment,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeTabPaneDrawInfoVersionZero {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub direction: ThemeTabDirection,
    pub size: HIThemeTabSize,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeMenuDrawInfo {
    pub version: UInt32,
    pub menuType: ThemeMenuType,
    pub reserved1: ::std::os::raw::c_ulong,
    pub reserved2: CGFloat,
    pub menuDirection: UInt32,
    pub reserved3: CGFloat,
    pub reserved4: CGFloat,
}
pub type HIThemeMenuDrawInfoPtr = *mut HIThemeMenuDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeMenuDrawInfoVersionZero {
    pub version: UInt32,
    pub menuType: ThemeMenuType,
}
pub type HIThemeMenuDrawInfoVersionZeroPtr = *mut HIThemeMenuDrawInfoVersionZero;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeMenuItemDrawInfo {
    pub version: UInt32,
    pub itemType: ThemeMenuItemType,
    pub state: ThemeMenuState,
}
pub type HIThemeMenuItemDrawInfoPtr = *mut HIThemeMenuItemDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeFrameDrawInfo {
    pub version: UInt32,
    pub kind: HIThemeFrameKind,
    pub state: ThemeDrawState,
    pub isFocused: Boolean,
}
pub type HIThemeFrameDrawInfoPtr = *mut HIThemeFrameDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeGroupBoxDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub kind: HIThemeGroupBoxKind,
}
pub type HIThemeGroupBoxDrawInfoPtr = *mut HIThemeGroupBoxDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeGrabberDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
}
pub type HIThemeGrabberDrawInfoPtr = *mut HIThemeGrabberDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemePlacardDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
}
pub type HIThemePlacardDrawInfoPtr = *mut HIThemePlacardDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeHeaderDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub kind: HIThemeHeaderKind,
}
pub type HIThemeHeaderDrawInfoPtr = *mut HIThemeHeaderDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeMenuBarDrawInfo {
    pub version: UInt32,
    pub state: ThemeMenuBarState,
    pub attributes: OptionBits,
}
pub type HIThemeMenuBarDrawInfoPtr = *mut HIThemeMenuBarDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeMenuTitleDrawInfo {
    pub version: UInt32,
    pub state: ThemeMenuState,
    pub attributes: OptionBits,
    pub condensedTitleExtra: CGFloat,
}
pub type HIThemeMenuTitleDrawInfoPtr = *mut HIThemeMenuTitleDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeTickMarkDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
}
pub type HIThemeTickMarkDrawInfoPtr = *mut HIThemeTickMarkDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeWindowDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub windowType: ThemeWindowType,
    pub attributes: ThemeWindowAttributes,
    pub titleHeight: CGFloat,
    pub titleWidth: CGFloat,
}
pub type HIThemeWindowDrawInfoPtr = *mut HIThemeWindowDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeWindowWidgetDrawInfo {
    pub version: UInt32,
    pub widgetState: ThemeDrawState,
    pub widgetType: ThemeTitleBarWidget,
    pub windowState: ThemeDrawState,
    pub windowType: ThemeWindowType,
    pub attributes: ThemeWindowAttributes,
    pub titleHeight: CGFloat,
    pub titleWidth: CGFloat,
}
pub type HIThemeWindowWidgetDrawInfoPtr = *mut HIThemeWindowWidgetDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeSeparatorDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
}
pub type HIThemeSeparatorDrawInfoPtr = *mut HIThemeSeparatorDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeScrollBarDelimitersDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub windowType: ThemeWindowType,
    pub attributes: ThemeWindowAttributes,
}
pub type HIThemeScrollBarDelimitersDrawInfoPtr = *mut HIThemeScrollBarDelimitersDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeChasingArrowsDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub index: UInt32,
}
pub type HIThemeChasingArrowsDrawInfoPtr = *mut HIThemeChasingArrowsDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemePopupArrowDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub orientation: ThemeArrowOrientation,
    pub size: ThemePopupArrowSize,
}
pub type HIThemePopupArrowDrawInfoPtr = *mut HIThemePopupArrowDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeGrowBoxDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub kind: HIThemeGrowBoxKind,
    pub direction: ThemeGrowDirection,
    pub size: HIThemeGrowBoxSize,
}
pub type HIThemeGrowBoxDrawInfoPtr = *mut HIThemeGrowBoxDrawInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeBackgroundDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub kind: ThemeBackgroundKind,
}
pub type HIThemeBackgroundDrawInfoPtr = *mut HIThemeBackgroundDrawInfo;
pub type HIThemeSegmentPosition = UInt32;
pub type HIThemeSegmentKind = UInt32;
pub type HIThemeSegmentSize = UInt32;
pub type HIThemeSegmentAdornment = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeSegmentDrawInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub value: ThemeButtonValue,
    pub size: HIThemeSegmentSize,
    pub kind: HIThemeSegmentKind,
    pub position: HIThemeSegmentPosition,
    pub adornment: HIThemeSegmentAdornment,
}
pub type HIThemeSegmentDrawInfoPtr = *mut HIThemeSegmentDrawInfo;
pub type HIThemeTextTruncation = UInt32;
pub type HIThemeTextHorizontalFlush = UInt32;
pub type HIThemeTextVerticalFlush = UInt32;
pub type HIThemeTextBoxOptions = OptionBits;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIThemeTextInfo {
    pub version: UInt32,
    pub state: ThemeDrawState,
    pub fontID: ThemeFontID,
    pub horizontalFlushness: HIThemeTextHorizontalFlush,
    pub verticalFlushness: HIThemeTextVerticalFlush,
    pub options: HIThemeTextBoxOptions,
    pub truncationPosition: HIThemeTextTruncation,
    pub truncationMaxLines: UInt32,
    pub truncationHappened: Boolean,
    pub filler1: UInt8,
    pub font: CTFontRef,
}
pub type HIThemeFocusRing = UInt32;
pub type ThemeMetric = UInt32;
pub type HIViewID = ControlID;
pub type HIViewZOrderOp = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIViewFrameMetrics {
    pub top: CGFloat,
    pub left: CGFloat,
    pub bottom: CGFloat,
    pub right: CGFloat,
}
pub type HIViewFeatures = UInt64;
pub type HIViewPartCode = ControlPartCode;
pub type HIViewImageContentType = ControlContentType;
pub type HIViewImageContentInfo = ControlImageContentInfo;
pub type HIViewContentType = SInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HITypeAndCreator {
    pub type_: OSType,
    pub creator: OSType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HIViewContentInfo {
    pub __bindgen_anon_1: HIViewContentInfo__bindgen_ty_1,
    pub contentType: HIViewContentType,
    pub u: HIViewContentInfo__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union HIViewContentInfo__bindgen_ty_1 {
    pub iconRef: IconRef,
    pub iconTypeAndCreator: HITypeAndCreator,
    pub imageRef: CGImageRef,
    pub imageResource: CFStringRef,
    pub imageFile: CFURLRef,
    pub nsImage: NSImage,
}
pub type HIViewContentInfoPtr = *mut HIViewContentInfo;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIViewKind {
    pub signature: OSType,
    pub kind: OSType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueHIViewTrackingAreaRef {
    _unused: [u8; 0],
}
pub type HIViewTrackingAreaRef = *mut OpaqueHIViewTrackingAreaRef;
pub type HIViewTrackingAreaID = UInt64;
pub type HIBindingKind = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HISideBinding {
    pub toView: HIViewRef,
    pub kind: HIBindingKind,
    pub offset: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HIBinding {
    pub top: HISideBinding,
    pub left: HISideBinding,
    pub bottom: HISideBinding,
    pub right: HISideBinding,
}
pub type HIScaleKind = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIAxisScale {
    pub toView: HIViewRef,
    pub kind: HIScaleKind,
    pub ratio: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HIScaling {
    pub x: HIAxisScale,
    pub y: HIAxisScale,
}
pub type HIPositionKind = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HIAxisPosition {
    pub toView: HIViewRef,
    pub kind: HIPositionKind,
    pub offset: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HIPositioning {
    pub x: HIAxisPosition,
    pub y: HIAxisPosition,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct HILayoutInfo {
    pub version: UInt32,
    pub binding: HIBinding,
    pub scale: HIScaling,
    pub position: HIPositioning,
}
pub type NMRecPtr = *mut NMRec;
pub type NMProcPtr = ::std::option::Option<unsafe extern "C" fn(nmReqPtr: NMRecPtr)>;
pub type NMUPP = NMProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct NMRec {
    pub qLink: QElemPtr,
    pub qType: ::std::os::raw::c_short,
    pub nmFlags: ::std::os::raw::c_short,
    pub nmPrivate: SRefCon,
    pub nmReserved: ::std::os::raw::c_short,
    pub nmMark: ::std::os::raw::c_short,
    pub nmIcon: Handle,
    pub nmSound: Handle,
    pub nmStr: StringPtr,
    pub nmResp: NMUPP,
    pub nmRefCon: SRefCon,
}
pub type DITLMethod = SInt16;
pub type StageList = SInt16;
pub type DialogRef = DialogPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct DialogTemplate {
    pub boundsRect: Rect,
    pub procID: SInt16,
    pub visible: Boolean,
    pub filler1: Boolean,
    pub goAwayFlag: Boolean,
    pub filler2: Boolean,
    pub refCon: SInt32,
    pub itemsID: SInt16,
    pub title: Str255,
}
pub type DialogTPtr = *mut DialogTemplate;
pub type DialogTHndl = *mut DialogTPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlertTemplate {
    pub boundsRect: Rect,
    pub itemsID: SInt16,
    pub stages: StageList,
}
pub type AlertTPtr = *mut AlertTemplate;
pub type AlertTHndl = *mut AlertTPtr;
pub type DialogItemIndexZeroBased = SInt16;
pub type DialogItemIndex = SInt16;
pub type DialogItemType = SInt16;
pub type SoundProcPtr = ::std::option::Option<unsafe extern "C" fn(soundNumber: SInt16)>;
pub type ModalFilterProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theDialog: DialogRef,
        theEvent: *mut EventRecord,
        itemHit: *mut DialogItemIndex,
    ) -> Boolean,
>;
pub type ModalFilterYDProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theDialog: DialogRef,
        theEvent: *mut EventRecord,
        itemHit: *mut ::std::os::raw::c_short,
        yourDataPtr: *mut ::std::os::raw::c_void,
    ) -> Boolean,
>;
pub type UserItemProcPtr =
    ::std::option::Option<unsafe extern "C" fn(theDialog: DialogRef, itemNo: DialogItemIndex)>;
pub type SoundUPP = SoundProcPtr;
pub type ModalFilterUPP = ModalFilterProcPtr;
pub type ModalFilterYDUPP = ModalFilterYDProcPtr;
pub type UserItemUPP = UserItemProcPtr;
pub type AlertType = SInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AlertStdAlertParamRec {
    pub movable: Boolean,
    pub helpButton: Boolean,
    pub filterProc: ModalFilterUPP,
    pub defaultText: ConstStringPtr,
    pub cancelText: ConstStringPtr,
    pub otherText: ConstStringPtr,
    pub defaultButton: SInt16,
    pub cancelButton: SInt16,
    pub position: UInt16,
}
pub type AlertStdAlertParamPtr = *mut AlertStdAlertParamRec;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AlertStdCFStringAlertParamRec {
    pub version: UInt32,
    pub movable: Boolean,
    pub helpButton: Boolean,
    pub defaultText: CFStringRef,
    pub cancelText: CFStringRef,
    pub otherText: CFStringRef,
    pub defaultButton: SInt16,
    pub cancelButton: SInt16,
    pub position: UInt16,
    pub flags: OptionBits,
    pub icon: IconRef,
}
pub type AlertStdCFStringAlertParamPtr = *mut AlertStdCFStringAlertParamRec;
pub type AEIdleProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theEvent: *mut EventRecord,
        sleepTime: *mut SInt32,
        mouseRgn: *mut RgnHandle,
    ) -> Boolean,
>;
pub type AEFilterProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theEvent: *mut EventRecord,
        returnID: SInt32,
        transactionID: AETransactionID,
        sender: *const AEAddressDesc,
    ) -> Boolean,
>;
pub type AEIdleUPP = AEIdleProcPtr;
pub type AEFilterUPP = AEFilterProcPtr;
pub type AEInteractAllowed = SInt8;
pub type TSMDocumentInterfaceType = OSType;
pub type TextServiceClass = OSType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueTSMDocumentID {
    _unused: [u8; 0],
}
pub type TSMDocumentID = *mut OpaqueTSMDocumentID;
pub type TSMDocumentPropertyTag = OSType;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TextServiceInfo {
    pub fComponent: Component,
    pub fItemName: Str255,
}
pub type TextServiceInfoPtr = *mut TextServiceInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TextServiceList {
    pub fTextServiceCount: ::std::os::raw::c_short,
    pub fServices: [TextServiceInfo; 1usize],
}
pub type TextServiceListPtr = *mut TextServiceList;
pub type TextServiceListHandle = *mut TextServiceListPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScriptLanguageRecord {
    pub fScript: ScriptCode,
    pub fLanguage: LangCode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScriptLanguageSupport {
    pub fScriptLanguageCount: SInt16,
    pub fScriptLanguageArray: [ScriptLanguageRecord; 1usize],
}
pub type ScriptLanguageSupportPtr = *mut ScriptLanguageSupport;
pub type ScriptLanguageSupportHandle = *mut ScriptLanguageSupportPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TSMGlyphInfo {
    pub range: CFRange,
    pub fontRef: ATSFontRef,
    pub collection: UInt16,
    pub glyphID: UInt16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TSMGlyphInfoArray {
    pub numGlyphInfo: ItemCount,
    pub glyphInfo: [TSMGlyphInfo; 1usize],
}
pub type TextServicePropertyTag = OSType;
pub type TextServicePropertyValue = *mut ::std::os::raw::c_void;
pub type ScrapFlavorType = FourCharCode;
pub type ScrapFlavorFlags = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ScrapFlavorInfo {
    pub flavorType: ScrapFlavorType,
    pub flavorFlags: ScrapFlavorFlags,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueScrapRef {
    _unused: [u8; 0],
}
pub type ScrapRef = *mut OpaqueScrapRef;
pub type ScrapPromiseKeeperProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        scrap: ScrapRef,
        flavorType: ScrapFlavorType,
        userData: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type ScrapPromiseKeeperUPP = ScrapPromiseKeeperProcPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueTXNObject {
    _unused: [u8; 0],
}
pub type TXNObject = *mut OpaqueTXNObject;
pub type TXNVersionValue = UInt32;
pub type TXNFrameID = UInt32;
pub type TXNFeatureBits = OptionBits;
pub type TXNFrameOptions = OptionBits;
pub type TXNFileType = OSType;
pub type TXNPermanentTextEncodingType = UInt32;
pub type TXNDataType = OSType;
pub type TXNTabType = SInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TXNTab {
    pub value: SInt16,
    pub tabType: TXNTabType,
    pub filler: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TXNMargins {
    pub topMargin: SInt16,
    pub leftMargin: SInt16,
    pub bottomMargin: SInt16,
    pub rightMargin: SInt16,
}
pub type TXNControlTag = FourCharCode;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union TXNControlData {
    pub uValue: ::std::os::raw::c_ulong,
    pub sValue: ::std::os::raw::c_long,
    pub tabValue: TXNTab,
    pub marginsPtr: *mut TXNMargins,
}
pub type TXNAutoScrollBehavior = UInt32;
pub type TXNOffset = UInt32;
pub type TXNHyperLinkState = UInt32;
pub type TXNTypeRunAttributes = FourCharCode;
pub type TXNTypeRunAttributeSizes = ByteCount;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TXNATSUIFeatures {
    pub featureCount: ItemCount,
    pub featureTypes: *mut ATSUFontFeatureType,
    pub featureSelectors: *mut ATSUFontFeatureSelector,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TXNATSUIVariations {
    pub variationCount: ItemCount,
    pub variationAxis: *mut ATSUFontVariationAxis,
    pub variationValues: *mut ATSUFontVariationValue,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union TXNAttributeData {
    pub dataPtr: *mut ::std::os::raw::c_void,
    pub dataValue: UInt32,
    pub atsuFeatures: *mut TXNATSUIFeatures,
    pub atsuVariations: *mut TXNATSUIVariations,
    pub urlReference: CFURLRef,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct TXNTypeAttributes {
    pub tag: TXNTypeRunAttributes,
    pub size: ByteCount,
    pub data: TXNAttributeData,
}
pub type TXNContinuousFlags = OptionBits;
pub type TXNMatchOptions = OptionBits;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TXNMatchTextRecord {
    pub iTextPtr: *const ::std::os::raw::c_void,
    pub iTextToMatchLength: ::std::os::raw::c_long,
    pub iTextEncoding: TextEncoding,
}
pub type TXNBackgroundType = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TXNBackgroundData {
    pub color: RGBColor,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TXNBackground {
    pub bgType: TXNBackgroundType,
    pub bg: TXNBackgroundData,
}
pub type TXNScrollUnit = UInt32;
pub type TXNScrollBarOrientation = UInt32;
pub type TXNScrollBarState = Boolean;
pub type TXNDrawItems = OptionBits;
pub type TXNRectKey = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TXNCarbonEventInfo {
    pub useCarbonEvents: Boolean,
    pub filler: UInt8,
    pub flags: UInt16,
    pub fDictionary: CFDictionaryRef,
}
pub type TXNFindProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        matchData: *const TXNMatchTextRecord,
        iDataType: TXNDataType,
        iMatchOptions: TXNMatchOptions,
        iSearchTextPtr: *const ::std::os::raw::c_void,
        encoding: TextEncoding,
        absStartOffset: TXNOffset,
        searchTextLength: ByteCount,
        oStartMatch: *mut TXNOffset,
        oEndMatch: *mut TXNOffset,
        ofound: *mut Boolean,
        refCon: URefCon,
    ) -> OSStatus,
>;
pub type TXNActionNameMapperProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        actionName: CFStringRef,
        commandID: UInt32,
        inUserData: *mut ::std::os::raw::c_void,
    ) -> CFStringRef,
>;
pub type TXNContextualMenuSetupProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        iContextualMenu: MenuRef,
        object: TXNObject,
        inUserData: *mut ::std::os::raw::c_void,
    ),
>;
pub type TXNScrollInfoProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        iValue: SInt32,
        iMaximumValue: SInt32,
        iScrollBarOrientation: TXNScrollBarOrientation,
        iRefCon: SRefCon,
    ),
>;
pub type TXNFindUPP = TXNFindProcPtr;
pub type TXNActionNameMapperUPP = TXNActionNameMapperProcPtr;
pub type TXNContextualMenuSetupUPP = TXNContextualMenuSetupProcPtr;
pub type TXNScrollInfoUPP = TXNScrollInfoProcPtr;
pub type TXNCommandEventSupportOptions = OptionBits;
pub type TXTNTag = FourCharCode;
pub type TXNErrors = OSStatus;
pub type TXNObjectRefcon = *mut ::std::os::raw::c_void;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TXNLongRect {
    pub top: SInt32,
    pub left: SInt32,
    pub bottom: SInt32,
    pub right: SInt32,
}
pub type TXNFrameType = UInt32;
pub type TXNActionKey = UInt32;
pub type TXNActionKeyMapperProcPtr = ::std::option::Option<
    unsafe extern "C" fn(actionKey: TXNActionKey, commandID: UInt32) -> CFStringRef,
>;
pub type TXNActionKeyMapperUPP = TXNActionKeyMapperProcPtr;
pub type TXNCountOptions = OptionBits;
pub type HMContentRequest = SInt16;
pub type HMContentType = UInt32;
pub type HMTagDisplaySide = SInt16;
pub type HMContentProvidedType = SInt16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HMStringResType {
    pub hmmResID: ::std::os::raw::c_short,
    pub hmmIndex: ::std::os::raw::c_short,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct HMHelpContent {
    pub __bindgen_anon_1: HMHelpContent__bindgen_ty_1,
    pub contentType: HMContentType,
    pub u: HMHelpContent__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union HMHelpContent__bindgen_ty_1 {
    pub tagCFString: CFStringRef,
    pub tagString: Str255,
    pub tagStringRes: HMStringResType,
    pub tagTEHandle: TEHandle,
    pub tagTextRes: SInt16,
    pub tagStrRes: SInt16,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct HMHelpContentRec {
    pub version: SInt32,
    pub absHotRect: Rect,
    pub tagSide: HMTagDisplaySide,
    pub content: [HMHelpContent; 2usize],
}
pub type HMHelpContentPtr = *mut HMHelpContentRec;
pub type HMControlContentProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inControl: ControlRef,
        inGlobalMouse: Point,
        inRequest: HMContentRequest,
        outContentProvided: *mut HMContentProvidedType,
        ioHelpContent: *mut HMHelpContentRec,
    ) -> OSStatus,
>;
pub type HMWindowContentProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inWindow: WindowRef,
        inGlobalMouse: Point,
        inRequest: HMContentRequest,
        outContentProvided: *mut HMContentProvidedType,
        ioHelpContent: *mut HMHelpContentRec,
    ) -> OSStatus,
>;
pub type HMMenuTitleContentProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inMenu: MenuRef,
        inRequest: HMContentRequest,
        outContentProvided: *mut HMContentProvidedType,
        ioHelpContent: *mut HMHelpContentRec,
    ) -> OSStatus,
>;
pub type HMMenuItemContentProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inTrackingData: *const MenuTrackingData,
        inRequest: HMContentRequest,
        outContentProvided: *mut HMContentProvidedType,
        ioHelpContent: *mut HMHelpContentRec,
    ) -> OSStatus,
>;
pub type HMControlContentUPP = HMControlContentProcPtr;
pub type HMWindowContentUPP = HMWindowContentProcPtr;
pub type HMMenuTitleContentUPP = HMMenuTitleContentProcPtr;
pub type HMMenuItemContentUPP = HMMenuItemContentProcPtr;
pub type ControlPushButtonIconAlignment = UInt16;
pub type ControlBevelThickness = UInt16;
pub type ControlBevelButtonBehavior = UInt16;
pub type ControlBevelButtonMenuBehavior = UInt16;
pub type ControlBevelButtonMenuPlacement = UInt16;
pub type ControlButtonGraphicAlignment = SInt16;
pub type ControlButtonTextAlignment = SInt16;
pub type ControlButtonTextPlacement = SInt16;
pub type ControlRoundButtonSize = SInt16;
pub type ControlClockType = UInt16;
pub type ControlClockFlags = UInt32;
pub type ControlUserPaneDrawProcPtr =
    ::std::option::Option<unsafe extern "C" fn(control: ControlRef, part: ControlPartCode)>;
pub type ControlUserPaneHitTestProcPtr = ::std::option::Option<
    unsafe extern "C" fn(control: ControlRef, where_: Point) -> ControlPartCode,
>;
pub type ControlUserPaneTrackingProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        control: ControlRef,
        startPt: Point,
        actionProc: ControlActionUPP,
    ) -> ControlPartCode,
>;
pub type ControlUserPaneIdleProcPtr =
    ::std::option::Option<unsafe extern "C" fn(control: ControlRef)>;
pub type ControlUserPaneKeyDownProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        control: ControlRef,
        keyCode: SInt16,
        charCode: SInt16,
        modifiers: SInt16,
    ) -> ControlPartCode,
>;
pub type ControlUserPaneActivateProcPtr =
    ::std::option::Option<unsafe extern "C" fn(control: ControlRef, activating: Boolean)>;
pub type ControlUserPaneFocusProcPtr = ::std::option::Option<
    unsafe extern "C" fn(control: ControlRef, action: ControlFocusPart) -> ControlPartCode,
>;
pub type ControlUserPaneDrawUPP = ControlUserPaneDrawProcPtr;
pub type ControlUserPaneHitTestUPP = ControlUserPaneHitTestProcPtr;
pub type ControlUserPaneTrackingUPP = ControlUserPaneTrackingProcPtr;
pub type ControlUserPaneIdleUPP = ControlUserPaneIdleProcPtr;
pub type ControlUserPaneKeyDownUPP = ControlUserPaneKeyDownProcPtr;
pub type ControlUserPaneActivateUPP = ControlUserPaneActivateProcPtr;
pub type ControlUserPaneFocusUPP = ControlUserPaneFocusProcPtr;
pub type DataBrowserViewStyle = OSType;
pub type DataBrowserSelectionFlags = UInt32;
pub type DataBrowserSortOrder = UInt16;
pub type DataBrowserItemID = ::std::os::raw::c_ulong;
pub type DataBrowserItemState = UInt32;
pub type DataBrowserRevealOptions = UInt8;
pub type DataBrowserSetOption = UInt32;
pub type DataBrowserSelectionAnchorDirection = UInt32;
pub type DataBrowserEditCommand = UInt32;
pub type DataBrowserItemNotification = UInt32;
pub type DataBrowserPropertyID = ::std::os::raw::c_ulong;
pub type DataBrowserPropertyType = OSType;
pub type DataBrowserPropertyPart = OSType;
pub type DataBrowserPropertyFlags = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct DataBrowserPropertyDesc {
    pub propertyID: DataBrowserPropertyID,
    pub propertyType: DataBrowserPropertyType,
    pub propertyFlags: DataBrowserPropertyFlags,
}
pub type DataBrowserItemProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        item: DataBrowserItemID,
        state: DataBrowserItemState,
        clientData: *mut ::std::os::raw::c_void,
    ),
>;
pub type DataBrowserItemUPP = DataBrowserItemProcPtr;
pub type DataBrowserMetric = UInt32;
pub type DataBrowserItemDataRef = *mut ::std::os::raw::c_void;
pub type DataBrowserItemDataProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        item: DataBrowserItemID,
        property: DataBrowserPropertyID,
        itemData: DataBrowserItemDataRef,
        setValue: Boolean,
    ) -> OSStatus,
>;
pub type DataBrowserItemDataUPP = DataBrowserItemDataProcPtr;
pub type DataBrowserItemCompareProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        itemOne: DataBrowserItemID,
        itemTwo: DataBrowserItemID,
        sortProperty: DataBrowserPropertyID,
    ) -> Boolean,
>;
pub type DataBrowserItemCompareUPP = DataBrowserItemCompareProcPtr;
pub type DataBrowserItemNotificationWithItemProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        item: DataBrowserItemID,
        message: DataBrowserItemNotification,
        itemData: DataBrowserItemDataRef,
    ),
>;
pub type DataBrowserItemNotificationProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        item: DataBrowserItemID,
        message: DataBrowserItemNotification,
    ),
>;
pub type DataBrowserItemNotificationWithItemUPP = DataBrowserItemNotificationWithItemProcPtr;
pub type DataBrowserItemNotificationUPP = DataBrowserItemNotificationProcPtr;
pub type DataBrowserAddDragItemProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        theDrag: DragReference,
        item: DataBrowserItemID,
        itemRef: *mut ItemReference,
    ) -> Boolean,
>;
pub type DataBrowserAcceptDragProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        theDrag: DragReference,
        item: DataBrowserItemID,
    ) -> Boolean,
>;
pub type DataBrowserReceiveDragProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        theDrag: DragReference,
        item: DataBrowserItemID,
    ) -> Boolean,
>;
pub type DataBrowserPostProcessDragProcPtr = ::std::option::Option<
    unsafe extern "C" fn(browser: ControlRef, theDrag: DragReference, trackDragResult: OSStatus),
>;
pub type DataBrowserAddDragItemUPP = DataBrowserAddDragItemProcPtr;
pub type DataBrowserAcceptDragUPP = DataBrowserAcceptDragProcPtr;
pub type DataBrowserReceiveDragUPP = DataBrowserReceiveDragProcPtr;
pub type DataBrowserPostProcessDragUPP = DataBrowserPostProcessDragProcPtr;
pub type DataBrowserGetContextualMenuProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        menu: *mut MenuRef,
        helpType: *mut UInt32,
        helpItemString: *mut CFStringRef,
        selection: *mut AEDesc,
    ),
>;
pub type DataBrowserSelectContextualMenuProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        menu: MenuRef,
        selectionType: UInt32,
        menuID: SInt16,
        menuItem: MenuItemIndex,
    ),
>;
pub type DataBrowserGetContextualMenuUPP = DataBrowserGetContextualMenuProcPtr;
pub type DataBrowserSelectContextualMenuUPP = DataBrowserSelectContextualMenuProcPtr;
pub type DataBrowserItemHelpContentProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        item: DataBrowserItemID,
        property: DataBrowserPropertyID,
        inRequest: HMContentRequest,
        outContentProvided: *mut HMContentProvidedType,
        ioHelpContent: *mut HMHelpContentRec,
    ),
>;
pub type DataBrowserItemHelpContentUPP = DataBrowserItemHelpContentProcPtr;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct DataBrowserCallbacks {
    pub __bindgen_anon_1: DataBrowserCallbacks__bindgen_ty_1,
    pub version: UInt32,
    pub u: DataBrowserCallbacks__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DataBrowserCallbacks__bindgen_ty_1 {
    pub v1: DataBrowserCallbacks__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct DataBrowserCallbacks__bindgen_ty_1__bindgen_ty_1 {
    pub itemDataCallback: DataBrowserItemDataUPP,
    pub itemCompareCallback: DataBrowserItemCompareUPP,
    pub itemNotificationCallback: DataBrowserItemNotificationUPP,
    pub addDragItemCallback: DataBrowserAddDragItemUPP,
    pub acceptDragCallback: DataBrowserAcceptDragUPP,
    pub receiveDragCallback: DataBrowserReceiveDragUPP,
    pub postProcessDragCallback: DataBrowserPostProcessDragUPP,
    pub itemHelpContentCallback: DataBrowserItemHelpContentUPP,
    pub getContextualMenuCallback: DataBrowserGetContextualMenuUPP,
    pub selectContextualMenuCallback: DataBrowserSelectContextualMenuUPP,
}
pub type DataBrowserDragFlags = UInt32;
pub type DataBrowserTrackingResult = SInt16;
pub type DataBrowserDrawItemProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        item: DataBrowserItemID,
        property: DataBrowserPropertyID,
        itemState: DataBrowserItemState,
        theRect: *const Rect,
        gdDepth: SInt16,
        colorDevice: Boolean,
    ),
>;
pub type DataBrowserEditItemProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        item: DataBrowserItemID,
        property: DataBrowserPropertyID,
        theString: CFStringRef,
        maxEditTextRect: *mut Rect,
        shrinkToFit: *mut Boolean,
    ) -> Boolean,
>;
pub type DataBrowserHitTestProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        itemID: DataBrowserItemID,
        property: DataBrowserPropertyID,
        theRect: *const Rect,
        mouseRect: *const Rect,
    ) -> Boolean,
>;
pub type DataBrowserTrackingProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        itemID: DataBrowserItemID,
        property: DataBrowserPropertyID,
        theRect: *const Rect,
        startPt: Point,
        modifiers: EventModifiers,
    ) -> DataBrowserTrackingResult,
>;
pub type DataBrowserItemDragRgnProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        itemID: DataBrowserItemID,
        property: DataBrowserPropertyID,
        theRect: *const Rect,
        dragRgn: RgnHandle,
    ),
>;
pub type DataBrowserItemAcceptDragProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        itemID: DataBrowserItemID,
        property: DataBrowserPropertyID,
        theRect: *const Rect,
        theDrag: DragReference,
    ) -> DataBrowserDragFlags,
>;
pub type DataBrowserItemReceiveDragProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        browser: ControlRef,
        itemID: DataBrowserItemID,
        property: DataBrowserPropertyID,
        dragFlags: DataBrowserDragFlags,
        theDrag: DragReference,
    ) -> Boolean,
>;
pub type DataBrowserDrawItemUPP = DataBrowserDrawItemProcPtr;
pub type DataBrowserEditItemUPP = DataBrowserEditItemProcPtr;
pub type DataBrowserHitTestUPP = DataBrowserHitTestProcPtr;
pub type DataBrowserTrackingUPP = DataBrowserTrackingProcPtr;
pub type DataBrowserItemDragRgnUPP = DataBrowserItemDragRgnProcPtr;
pub type DataBrowserItemAcceptDragUPP = DataBrowserItemAcceptDragProcPtr;
pub type DataBrowserItemReceiveDragUPP = DataBrowserItemReceiveDragProcPtr;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct DataBrowserCustomCallbacks {
    pub __bindgen_anon_1: DataBrowserCustomCallbacks__bindgen_ty_1,
    pub version: UInt32,
    pub u: DataBrowserCustomCallbacks__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DataBrowserCustomCallbacks__bindgen_ty_1 {
    pub v1: DataBrowserCustomCallbacks__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct DataBrowserCustomCallbacks__bindgen_ty_1__bindgen_ty_1 {
    pub drawItemCallback: DataBrowserDrawItemUPP,
    pub editTextCallback: DataBrowserEditItemUPP,
    pub hitTestCallback: DataBrowserHitTestUPP,
    pub trackingCallback: DataBrowserTrackingUPP,
    pub dragRegionCallback: DataBrowserItemDragRgnUPP,
    pub acceptDragCallback: DataBrowserItemAcceptDragUPP,
    pub receiveDragCallback: DataBrowserItemReceiveDragUPP,
}
pub type DataBrowserTableViewHiliteStyle = UInt32;
pub type DataBrowserTableViewPropertyFlags = UInt32;
pub type DataBrowserTableViewRowIndex = ::std::os::raw::c_ulong;
pub type DataBrowserTableViewColumnIndex = ::std::os::raw::c_ulong;
pub type DataBrowserTableViewColumnID = DataBrowserPropertyID;
pub type DataBrowserTableViewColumnDesc = DataBrowserPropertyDesc;
pub type DataBrowserListViewPropertyFlags = DataBrowserPropertyFlags;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct DataBrowserListViewHeaderDesc {
    pub version: UInt32,
    pub minimumWidth: UInt16,
    pub maximumWidth: UInt16,
    pub titleOffset: SInt16,
    pub titleString: CFStringRef,
    pub initialOrder: DataBrowserSortOrder,
    pub btnFontStyle: ControlFontStyleRec,
    pub btnContentInfo: ControlButtonContentInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataBrowserListViewColumnDesc {
    pub propertyDesc: DataBrowserTableViewColumnDesc,
    pub headerBtnDesc: DataBrowserListViewHeaderDesc,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct DataBrowserAccessibilityItemInfoV0 {
    pub container: DataBrowserItemID,
    pub item: DataBrowserItemID,
    pub columnProperty: DataBrowserPropertyID,
    pub propertyPart: DataBrowserPropertyPart,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct DataBrowserAccessibilityItemInfoV1 {
    pub container: DataBrowserItemID,
    pub item: DataBrowserItemID,
    pub columnProperty: DataBrowserPropertyID,
    pub propertyPart: DataBrowserPropertyPart,
    pub rowIndex: DataBrowserTableViewRowIndex,
    pub columnIndex: DataBrowserTableViewColumnIndex,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct DataBrowserAccessibilityItemInfo {
    pub __bindgen_anon_1: DataBrowserAccessibilityItemInfo__bindgen_ty_1,
    pub version: UInt32,
    pub u: DataBrowserAccessibilityItemInfo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DataBrowserAccessibilityItemInfo__bindgen_ty_1 {
    pub v0: DataBrowserAccessibilityItemInfoV0,
    pub v1: DataBrowserAccessibilityItemInfoV1,
}
pub type ControlDisclosureTriangleOrientation = UInt16;
pub type HIImageViewAutoTransformOptions = UInt32;
pub type ControlPopupArrowOrientation = UInt16;
pub type ControlPopupArrowSize = UInt16;
pub type HIScrollViewAction = UInt32;
pub type HISegmentBehavior = UInt32;
pub type ControlSliderOrientation = UInt16;
pub type ControlTabDirection = UInt16;
pub type ControlTabSize = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ControlTabEntry {
    pub icon: *mut ControlButtonContentInfo,
    pub name: CFStringRef,
    pub enabled: Boolean,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControlTabInfoRec {
    pub version: SInt16,
    pub iconSuiteID: SInt16,
    pub name: Str255,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ControlTabInfoRecV1 {
    pub version: SInt16,
    pub iconSuiteID: SInt16,
    pub name: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControlEditTextSelectionRec {
    pub selStart: SInt16,
    pub selEnd: SInt16,
}
pub type ControlEditTextSelectionPtr = *mut ControlEditTextSelectionRec;
pub type ControlEditTextValidationProcPtr =
    ::std::option::Option<unsafe extern "C" fn(control: ControlRef)>;
pub type EditUnicodePostUpdateProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        uniText: UniCharArrayHandle,
        uniTextLength: UniCharCount,
        iStartOffset: UniCharArrayOffset,
        iEndOffset: UniCharArrayOffset,
        refcon: *mut ::std::os::raw::c_void,
    ) -> Boolean,
>;
pub type ControlEditTextValidationUPP = ControlEditTextValidationProcPtr;
pub type EditUnicodePostUpdateUPP = EditUnicodePostUpdateProcPtr;
pub type Cell = Point;
pub type ListBounds = Rect;
pub type DataPtr = *mut ::std::os::raw::c_char;
pub type DataHandle = *mut DataPtr;
pub type ListSearchProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        aPtr: Ptr,
        bPtr: Ptr,
        aLen: ::std::os::raw::c_short,
        bLen: ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_short,
>;
pub type ListClickLoopProcPtr = ::std::option::Option<unsafe extern "C" fn() -> Boolean>;
pub type ListSearchUPP = ListSearchProcPtr;
pub type ListClickLoopUPP = ListClickLoopProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ListRec {
    pub rView: Rect,
    pub port: GrafPtr,
    pub indent: Point,
    pub cellSize: Point,
    pub visible: ListBounds,
    pub vScroll: ControlRef,
    pub hScroll: ControlRef,
    pub selFlags: SInt8,
    pub lActive: Boolean,
    pub lReserved: SInt8,
    pub listFlags: SInt8,
    pub clikTime: ::std::os::raw::c_long,
    pub clikLoc: Point,
    pub mouseLoc: Point,
    pub lClickLoop: ListClickLoopUPP,
    pub lastClick: Cell,
    pub refCon: ::std::os::raw::c_long,
    pub listDefProc: Handle,
    pub userHandle: Handle,
    pub dataBounds: ListBounds,
    pub cells: DataHandle,
    pub maxIndex: ::std::os::raw::c_short,
    pub cellArray: [::std::os::raw::c_short; 1usize],
}
pub type ListPtr = *mut ListRec;
pub type ListHandle = *mut ListPtr;
pub type ListRef = ListHandle;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct StandardIconListCellDataRec {
    pub iconHandle: Handle,
    pub font: ::std::os::raw::c_short,
    pub face: ::std::os::raw::c_short,
    pub size: ::std::os::raw::c_short,
    pub name: Str255,
}
pub type StandardIconListCellDataPtr = *mut StandardIconListCellDataRec;
pub type ListDefProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        lMessage: ::std::os::raw::c_short,
        lSelect: Boolean,
        lRect: *mut Rect,
        lCell: Cell,
        lDataOffset: ::std::os::raw::c_short,
        lDataLen: ::std::os::raw::c_short,
        lHandle: ListHandle,
    ),
>;
pub type ListDefUPP = ListDefProcPtr;
pub type ListDefType = UInt32;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct ListDefSpec {
    pub __bindgen_anon_1: ListDefSpec__bindgen_ty_1,
    pub defType: ListDefType,
    pub u: ListDefSpec__bindgen_ty_1,
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub union ListDefSpec__bindgen_ty_1 {
    pub userProc: ListDefUPP,
}
pub type ListDefSpecPtr = *mut ListDefSpec;
pub type FileType = OSType;
pub type ScrapType = ResType;
pub type TranslationAttributes = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FileTypeSpec {
    pub format: FileType,
    pub hint: ::std::os::raw::c_long,
    pub flags: TranslationAttributes,
    pub catInfoType: OSType,
    pub catInfoCreator: OSType,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FileTranslationList {
    pub modDate: ::std::os::raw::c_ulong,
    pub groupCount: ::std::os::raw::c_ulong,
}
pub type FileTranslationListPtr = *mut FileTranslationList;
pub type FileTranslationListHandle = *mut FileTranslationListPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ScrapTypeSpec {
    pub format: ScrapType,
    pub hint: ::std::os::raw::c_long,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ScrapTranslationList {
    pub modDate: ::std::os::raw::c_ulong,
    pub groupCount: ::std::os::raw::c_ulong,
}
pub type ScrapTranslationListPtr = *mut ScrapTranslationList;
pub type ScrapTranslationListHandle = *mut ScrapTranslationListPtr;
pub type TranslationRefNum = ::std::os::raw::c_long;
pub type DocOpenMethod = ::std::os::raw::c_short;
pub type TypesBlockPtr = *mut OSType;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FileTranslationSpec {
    pub componentSignature: OSType,
    pub translationSystemInfo: *const ::std::os::raw::c_void,
    pub src: FileTypeSpec,
    pub dst: FileTypeSpec,
}
pub type FileTranslationSpecArrayPtr = *mut FileTranslationSpec;
pub type FileTranslationSpecArrayHandle = *mut FileTranslationSpecArrayPtr;
pub type GetScrapDataProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        requestedFormat: ScrapType,
        dataH: Handle,
        srcDataGetterRefCon: *mut ::std::os::raw::c_void,
    ) -> OSErr,
>;
pub type GetScrapDataUPP = GetScrapDataProcPtr;
pub type GetScrapData = GetScrapDataUPP;
pub type TSCode = SInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TypeSelectRecord {
    pub tsrLastKeyTime: UInt32,
    pub tsrScript: ScriptCode,
    pub tsrKeyStrokes: Str63,
}
pub type SystemUIMode = UInt32;
pub type SystemUIOptions = OptionBits;
pub type PhysicalKeyboardLayoutType = OSType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueKeyboardLayoutRef {
    _unused: [u8; 0],
}
pub type KeyboardLayoutRef = *mut OpaqueKeyboardLayoutRef;
pub type KeyboardLayoutPropertyTag = UInt32;
pub type KeyboardLayoutKind = SInt32;
pub type KeyboardLayoutIdentifier = SInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueIBNibRef {
    _unused: [u8; 0],
}
pub type IBNibRef = *mut OpaqueIBNibRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TISInputSource {
    _unused: [u8; 0],
}
pub type TISInputSourceRef = *mut __TISInputSource;
pub type IMKLocationToOffsetMappingMode = NSInteger;
pub trait PIMKTextInput: Sized + std::ops::Deref {
    unsafe fn insertText_replacementRange_(&self, string: id, replacementRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertText : string, replacementRange : replacementRange)
    }
    unsafe fn setMarkedText_selectionRange_replacementRange_(
        &self,
        string: id,
        selectionRange: NSRange,
        replacementRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarkedText : string, selectionRange : selectionRange, replacementRange : replacementRange)
    }
    unsafe fn selectedRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedRange)
    }
    unsafe fn markedRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, markedRange)
    }
    unsafe fn attributedSubstringFromRange_(&self, range: NSRange) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributedSubstringFromRange : range)
    }
    unsafe fn length(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn characterIndexForPoint_tracking_inMarkedRange_(
        &self,
        point: NSPoint,
        mappingMode: IMKLocationToOffsetMappingMode,
        inMarkedRange: *mut BOOL,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, characterIndexForPoint : point, tracking : mappingMode, inMarkedRange : inMarkedRange)
    }
    unsafe fn attributesForCharacterIndex_lineHeightRectangle_(
        &self,
        index: NSUInteger,
        lineRect: *mut NSRect,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributesForCharacterIndex : index, lineHeightRectangle : lineRect)
    }
    unsafe fn validAttributesForMarkedText(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, validAttributesForMarkedText)
    }
    unsafe fn overrideKeyboardWithKeyboardNamed_(&self, keyboardUniqueName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, overrideKeyboardWithKeyboardNamed : keyboardUniqueName)
    }
    unsafe fn selectInputMode_(&self, modeIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectInputMode : modeIdentifier)
    }
    unsafe fn supportsUnicode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsUnicode)
    }
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn windowLevel(&self) -> CGWindowLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowLevel)
    }
    unsafe fn supportsProperty_(&self, property: TSMDocumentPropertyTag) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsProperty : property)
    }
    unsafe fn uniqueClientIdentifierString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueClientIdentifierString)
    }
    unsafe fn stringFromRange_actualRange_(
        &self,
        range: NSRange,
        actualRange: NSRangePointer,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringFromRange : range, actualRange : actualRange)
    }
    unsafe fn firstRectForCharacterRange_actualRange_(
        &self,
        aRange: NSRange,
        actualRange: NSRangePointer,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, firstRectForCharacterRange : aRange, actualRange : actualRange)
    }
}
pub trait PIMKUnicodeTextInput: Sized + std::ops::Deref {
    unsafe fn insertText_(&self, string: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertText : string)
    }
}
pub type TSMTEPreUpdateProcPtr =
    ::std::option::Option<unsafe extern "C" fn(textH: TEHandle, refCon: ::std::os::raw::c_long)>;
pub type TSMTEPostUpdateProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        textH: TEHandle,
        fixLen: ::std::os::raw::c_long,
        inputAreaStart: ::std::os::raw::c_long,
        inputAreaEnd: ::std::os::raw::c_long,
        pinStart: ::std::os::raw::c_long,
        pinEnd: ::std::os::raw::c_long,
        refCon: ::std::os::raw::c_long,
    ),
>;
pub type TSMTEPreUpdateUPP = TSMTEPreUpdateProcPtr;
pub type TSMTEPostUpdateUPP = TSMTEPostUpdateProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct TSMTERec {
    pub textH: TEHandle,
    pub preUpdateProc: TSMTEPreUpdateUPP,
    pub postUpdateProc: TSMTEPostUpdateUPP,
    pub updateFlag: ::std::os::raw::c_long,
    pub refCon: ::std::os::raw::c_long,
}
pub type TSMTERecPtr = *mut TSMTERec;
pub type TSMTERecHandle = *mut TSMTERecPtr;
pub type OSAError = ComponentResult;
pub type OSAID = UInt32;
pub type OSACreateAppleEventProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theAEEventClass: AEEventClass,
        theAEEventID: AEEventID,
        target: *const AEAddressDesc,
        returnID: ::std::os::raw::c_short,
        transactionID: SInt32,
        result: *mut AppleEvent,
        refCon: SRefCon,
    ) -> OSErr,
>;
pub type OSASendProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        theAppleEvent: *const AppleEvent,
        reply: *mut AppleEvent,
        sendMode: AESendMode,
        sendPriority: AESendPriority,
        timeOutInTicks: SInt32,
        idleProc: AEIdleUPP,
        filterProc: AEFilterUPP,
        refCon: SRefCon,
    ) -> OSErr,
>;
pub type OSACreateAppleEventUPP = OSACreateAppleEventProcPtr;
pub type OSASendUPP = OSASendProcPtr;
pub type OSAActiveProcPtr = ::std::option::Option<unsafe extern "C" fn(refCon: SRefCon) -> OSErr>;
pub type OSAActiveUPP = OSAActiveProcPtr;
pub type ScriptingComponentSelector = OSType;
pub type GenericID = OSAID;
pub type DialogPlacementSpec = SInt16;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct NPMColor {
    pub profile: CMProfileRef,
    pub color: CMColor,
}
pub type NPMColorPtr = *mut NPMColor;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePicker {
    _unused: [u8; 0],
}
pub type Picker = *mut OpaquePicker;
pub type picker = Picker;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PickerMenuItemInfo {
    pub editMenuID: SInt16,
    pub cutItem: SInt16,
    pub copyItem: SInt16,
    pub pasteItem: SInt16,
    pub clearItem: SInt16,
    pub undoItem: SInt16,
}
pub type NColorChangedProcPtr =
    ::std::option::Option<unsafe extern "C" fn(userData: SRefCon, newColor: *mut NPMColor)>;
pub type NColorChangedUPP = NColorChangedProcPtr;
pub type ColorChangedUPP = *mut ::std::os::raw::c_void;
pub type UserEventUPP = *mut ::std::os::raw::c_void;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct NColorPickerInfo {
    pub theColor: NPMColor,
    pub dstProfile: CMProfileRef,
    pub flags: UInt32,
    pub placeWhere: DialogPlacementSpec,
    pub dialogOrigin: Point,
    pub pickerType: OSType,
    pub colorProc: NColorChangedUPP,
    pub colorProcData: URefCon,
    pub prompt: Str255,
    pub mInfo: PickerMenuItemInfo,
    pub newColorChosen: Boolean,
    pub reserved: UInt8,
}
pub type CalibrateEventProcPtr =
    ::std::option::Option<unsafe extern "C" fn(event: *mut EventRecord)>;
pub type CalibrateEventUPP = CalibrateEventProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CalibratorInfo {
    pub dataSize: UInt32,
    pub displayID: CMDisplayIDType,
    pub profileLocationSize: UInt32,
    pub profileLocationPtr: *mut CMProfileLocation,
    pub eventProc: CalibrateEventUPP,
    pub isGood: Boolean,
}
pub type CanCalibrateProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        displayID: CMDisplayIDType,
        errMessage: *mut ::std::os::raw::c_uchar,
    ) -> Boolean,
>;
pub type CalibrateProcPtr =
    ::std::option::Option<unsafe extern "C" fn(theInfo: *mut CalibratorInfo) -> OSErr>;
pub type CanCalibrateUPP = CanCalibrateProcPtr;
pub type CalibrateUPP = CalibrateProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FontSelectionQDStyle {
    pub version: UInt32,
    pub instance: FMFontFamilyInstance,
    pub size: FMFontSize,
    pub hasColor: Boolean,
    pub reserved: UInt8,
    pub color: RGBColor,
}
pub type FontSelectionQDStylePtr = *mut FontSelectionQDStyle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueFCFontDescriptorRef {
    _unused: [u8; 0],
}
pub type FCFontDescriptorRef = *mut OpaqueFCFontDescriptorRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueSRSpeechObject {
    _unused: [u8; 0],
}
pub type SRSpeechObject = *mut OpaqueSRSpeechObject;
pub type SRRecognitionSystem = SRSpeechObject;
pub type SRRecognizer = SRSpeechObject;
pub type SRSpeechSource = SRSpeechObject;
pub type SRRecognitionResult = SRSpeechSource;
pub type SRLanguageObject = SRSpeechObject;
pub type SRLanguageModel = SRLanguageObject;
pub type SRPath = SRLanguageObject;
pub type SRPhrase = SRLanguageObject;
pub type SRWord = SRLanguageObject;
pub type SRSpeedSetting = UInt16;
pub type SRRejectionLevel = UInt16;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SRCallBackStruct {
    pub what: UInt32,
    pub message: ::std::os::raw::c_long,
    pub instance: SRRecognizer,
    pub status: OSErr,
    pub flags: SInt16,
    pub refCon: SRefCon,
}
pub type SRCallBackProcPtr =
    ::std::option::Option<unsafe extern "C" fn(param: *mut SRCallBackStruct)>;
pub type SRCallBackUPP = SRCallBackProcPtr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct SRCallBackParam {
    pub callBack: SRCallBackUPP,
    pub refCon: SRefCon,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueURLReference {
    _unused: [u8; 0],
}
pub type URLReference = *mut OpaqueURLReference;
pub type URLOpenFlags = UInt32;
pub type URLState = UInt32;
pub type URLEvent = UInt32;
pub type URLEventMask = ::std::os::raw::c_ulong;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct URLCallbackInfo {
    pub version: UInt32,
    pub urlRef: URLReference,
    pub property: *const ::std::os::raw::c_char,
    pub currentSize: UInt32,
    pub systemEvent: *mut EventRecord,
}
pub type URLNotifyProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        userContext: *mut ::std::os::raw::c_void,
        event: URLEvent,
        callbackInfo: *mut URLCallbackInfo,
    ) -> OSStatus,
>;
pub type URLSystemEventProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        userContext: *mut ::std::os::raw::c_void,
        event: *mut EventRecord,
    ) -> OSStatus,
>;
pub type URLNotifyUPP = URLNotifyProcPtr;
pub type URLSystemEventUPP = URLSystemEventProcPtr;
pub type AHTOCType = SInt16;
unsafe extern "C" {
    pub fn HIPointConvert(
        ioPoint: *mut HIPoint,
        inSourceSpace: HICoordinateSpace,
        inSourceObject: *mut ::std::os::raw::c_void,
        inDestinationSpace: HICoordinateSpace,
        inDestinationObject: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn HIRectConvert(
        ioRect: *mut HIRect,
        inSourceSpace: HICoordinateSpace,
        inSourceObject: *mut ::std::os::raw::c_void,
        inDestinationSpace: HICoordinateSpace,
        inDestinationObject: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn HISizeConvert(
        ioSize: *mut HISize,
        inSourceSpace: HICoordinateSpace,
        inSourceObject: *mut ::std::os::raw::c_void,
        inDestinationSpace: HICoordinateSpace,
        inDestinationObject: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn GetCurrentEventLoop() -> EventLoopRef;
}
unsafe extern "C" {
    pub fn GetMainEventLoop() -> EventLoopRef;
}
unsafe extern "C" {
    pub fn RunCurrentEventLoop(inTimeout: EventTimeout) -> OSStatus;
}
unsafe extern "C" {
    pub fn QuitEventLoop(inEventLoop: EventLoopRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetCFRunLoopFromEventLoop(inEventLoop: EventLoopRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn ReceiveNextEvent(
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
        inTimeout: EventTimeout,
        inPullEvent: Boolean,
        outEvent: *mut EventRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateEvent(
        inAllocator: CFAllocatorRef,
        inClassID: OSType,
        inKind: UInt32,
        inWhen: EventTime,
        inAttributes: EventAttributes,
        outEvent: *mut EventRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CopyEvent(inOther: EventRef) -> EventRef;
}
unsafe extern "C" {
    pub fn CopyEventAs(
        inAllocator: CFAllocatorRef,
        inOther: EventRef,
        inEventClass: OSType,
        inEventKind: UInt32,
    ) -> EventRef;
}
unsafe extern "C" {
    pub fn RetainEvent(inEvent: EventRef) -> EventRef;
}
unsafe extern "C" {
    pub fn GetEventRetainCount(inEvent: EventRef) -> ItemCount;
}
unsafe extern "C" {
    pub fn ReleaseEvent(inEvent: EventRef);
}
unsafe extern "C" {
    pub fn SetEventParameter(
        inEvent: EventRef,
        inName: EventParamName,
        inType: EventParamType,
        inSize: ByteCount,
        inDataPtr: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetEventParameter(
        inEvent: EventRef,
        inName: EventParamName,
        inDesiredType: EventParamType,
        outActualType: *mut EventParamType,
        inBufferSize: ByteCount,
        outActualSize: *mut ByteCount,
        outData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn RemoveEventParameter(inEvent: EventRef, inName: EventParamName) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetEventClass(inEvent: EventRef) -> OSType;
}
unsafe extern "C" {
    pub fn GetEventKind(inEvent: EventRef) -> UInt32;
}
unsafe extern "C" {
    pub fn GetEventTime(inEvent: EventRef) -> EventTime;
}
unsafe extern "C" {
    pub fn SetEventTime(inEvent: EventRef, inTime: EventTime) -> OSStatus;
}
unsafe extern "C" {
    pub fn CreateEventWithCGEvent(
        inAllocator: CFAllocatorRef,
        inEvent: CGEventRef,
        inAttributes: EventAttributes,
        outEvent: *mut EventRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CopyEventCGEvent(inEvent: EventRef) -> CGEventRef;
}
unsafe extern "C" {
    pub fn GetCurrentEventQueue() -> EventQueueRef;
}
unsafe extern "C" {
    pub fn GetMainEventQueue() -> EventQueueRef;
}
unsafe extern "C" {
    pub fn PostEventToQueue(
        inQueue: EventQueueRef,
        inEvent: EventRef,
        inPriority: EventPriority,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FlushEventsMatchingListFromQueue(
        inQueue: EventQueueRef,
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FlushSpecificEventsFromQueue(
        inQueue: EventQueueRef,
        inComparator: EventComparatorUPP,
        inCompareData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FlushEventQueue(inQueue: EventQueueRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn FindSpecificEventInQueue(
        inQueue: EventQueueRef,
        inComparator: EventComparatorUPP,
        inCompareData: *mut ::std::os::raw::c_void,
    ) -> EventRef;
}
unsafe extern "C" {
    pub fn GetNumEventsInQueue(inQueue: EventQueueRef) -> ItemCount;
}
unsafe extern "C" {
    pub fn RemoveEventFromQueue(inQueue: EventQueueRef, inEvent: EventRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn IsEventInQueue(inQueue: EventQueueRef, inEvent: EventRef) -> Boolean;
}
unsafe extern "C" {
    pub fn AcquireFirstMatchingEventInQueue(
        inQueue: EventQueueRef,
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
        inOptions: OptionBits,
    ) -> EventRef;
}
unsafe extern "C" {
    pub fn GetCurrentEvent() -> EventRef;
}
unsafe extern "C" {
    pub fn GetCurrentEventButtonState() -> UInt32;
}
unsafe extern "C" {
    pub fn GetCurrentEventKeyModifiers() -> UInt32;
}
unsafe extern "C" {
    pub fn HIGetMousePosition(
        inSpace: HICoordinateSpace,
        inObject: *mut ::std::os::raw::c_void,
        outPoint: *mut HIPoint,
    ) -> *mut HIPoint;
}
unsafe extern "C" {
    pub fn GetCurrentButtonState() -> UInt32;
}
unsafe extern "C" {
    pub fn GetCurrentKeyModifiers() -> UInt32;
}
unsafe extern "C" {
    pub fn GetCurrentEventTime() -> EventTime;
}
unsafe extern "C" {
    pub fn InstallEventLoopTimer(
        inEventLoop: EventLoopRef,
        inFireDelay: EventTimerInterval,
        inInterval: EventTimerInterval,
        inTimerProc: EventLoopTimerUPP,
        inTimerData: *mut ::std::os::raw::c_void,
        outTimer: *mut EventLoopTimerRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn RemoveEventLoopTimer(inTimer: EventLoopTimerRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SetEventLoopTimerNextFireTime(
        inTimer: EventLoopTimerRef,
        inNextFire: EventTimerInterval,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn InstallEventHandler(
        inTarget: EventTargetRef,
        inHandler: EventHandlerUPP,
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
        inUserData: *mut ::std::os::raw::c_void,
        outRef: *mut EventHandlerRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn RemoveEventHandler(inHandlerRef: EventHandlerRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AddEventTypesToHandler(
        inHandlerRef: EventHandlerRef,
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn RemoveEventTypesFromHandler(
        inHandlerRef: EventHandlerRef,
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CallNextEventHandler(inCallRef: EventHandlerCallRef, inEvent: EventRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SendEventToEventTarget(inEvent: EventRef, inTarget: EventTargetRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SendEventToEventTargetWithOptions(
        inEvent: EventRef,
        inTarget: EventTargetRef,
        inOptions: OptionBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn EnableSecureEventInput() -> OSStatus;
}
unsafe extern "C" {
    pub fn DisableSecureEventInput() -> OSStatus;
}
unsafe extern "C" {
    pub fn IsSecureEventInputEnabled() -> Boolean;
}
unsafe extern "C" {
    pub static kHIObjectInitParamUserName: CFStringRef;
}
unsafe extern "C" {
    pub static kHIObjectInitParamDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kHIObjectInitParamEventName: CFStringRef;
}
unsafe extern "C" {
    pub static kHIObjectInitParamEventType: CFStringRef;
}
unsafe extern "C" {
    pub fn HIObjectRegisterSubclass(
        inClassID: CFStringRef,
        inBaseClassID: CFStringRef,
        inOptions: OptionBits,
        inConstructProc: EventHandlerUPP,
        inNumEvents: ItemCount,
        inEventList: *const EventTypeSpec,
        inConstructData: *mut ::std::os::raw::c_void,
        outClassRef: *mut HIObjectClassRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIObjectUnregisterClass(inClassRef: HIObjectClassRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIObjectCreate(
        inClassID: CFStringRef,
        inConstructData: EventRef,
        outObject: *mut HIObjectRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIObjectGetEventTarget(inObject: HIObjectRef) -> EventTargetRef;
}
unsafe extern "C" {
    pub fn HIObjectPrintDebugInfo(inObject: HIObjectRef);
}
unsafe extern "C" {
    pub fn HIObjectCopyClassID(inObject: HIObjectRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn HIObjectIsOfClass(inObject: HIObjectRef, inObjectClassID: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn HIObjectDynamicCast(
        inObject: HIObjectRef,
        inClassID: CFStringRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn HIObjectCreateFromBundle(inBundle: CFBundleRef, outObject: *mut HIObjectRef)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn HIObjectFromEventTarget(inTarget: EventTargetRef) -> HIObjectRef;
}
unsafe extern "C" {
    pub fn HIObjectIsArchivingIgnored(inObject: HIObjectRef) -> Boolean;
}
unsafe extern "C" {
    pub static kHIObjectCustomDataParameterNamesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kHIObjectCustomDataParameterTypesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kHIObjectCustomDataParameterValuesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kHIObjectCustomDataClassIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kHIObjectCustomDataSuperClassIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kHIObjectCustomDataDelegateGroupParametersKey: CFStringRef;
}
unsafe extern "C" {
    pub static kHIDelegateBeforeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kHIDelegateAfterKey: CFStringRef;
}
unsafe extern "C" {
    pub fn HIObjectAddDelegate(
        inObject: HIObjectRef,
        inDelegate: HIObjectRef,
        inPosition: HIDelegatePosition,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIObjectRemoveDelegate(
        inObject: HIObjectRef,
        inDelegate: HIObjectRef,
        inPosition: HIDelegatePosition,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIObjectCopyDelegates(
        inObject: HIObjectRef,
        outDelegates: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIObjectGetEventHandlerObject(inRef: EventHandlerCallRef) -> HIObjectRef;
}
unsafe extern "C" {
    pub fn Button() -> Boolean;
}
unsafe extern "C" {
    pub fn GetKeys(theKeys: *mut BigEndianUInt32);
}
unsafe extern "C" {
    pub fn FlushEvents(whichMask: EventMask, stopMask: EventMask);
}
unsafe extern "C" {
    pub fn IsCmdChar(event: *const EventRecord, test: ::std::os::raw::c_short) -> Boolean;
}
unsafe extern "C" {
    pub fn LMGetKeyThresh() -> SInt16;
}
unsafe extern "C" {
    pub fn LMGetKeyRepThresh() -> SInt16;
}
unsafe extern "C" {
    pub fn LMGetKbdLast() -> UInt8;
}
unsafe extern "C" {
    pub fn LMGetKbdType() -> UInt8;
}
unsafe extern "C" {
    pub fn GetMenuTrackingData(theMenu: MenuRef, outData: *mut MenuTrackingData) -> OSStatus;
}
unsafe extern "C" {
    pub fn CheckEventQueueForUserCancel() -> Boolean;
}
unsafe extern "C" {
    pub fn IsUserCancelEventRef(event: EventRef) -> Boolean;
}
unsafe extern "C" {
    pub fn HIMouseTrackingGetParameters(
        inSelector: OSType,
        outTime: *mut EventTime,
        outDistance: *mut HISize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetApplicationEventTarget() -> EventTargetRef;
}
unsafe extern "C" {
    pub fn GetEventDispatcherTarget() -> EventTargetRef;
}
unsafe extern "C" {
    pub fn GetEventMonitorTarget() -> EventTargetRef;
}
unsafe extern "C" {
    pub fn ProcessHICommand(inCommand: *const HICommand) -> OSStatus;
}
unsafe extern "C" {
    pub fn RegisterEventHotKey(
        inHotKeyCode: UInt32,
        inHotKeyModifiers: UInt32,
        inHotKeyID: EventHotKeyID,
        inTarget: EventTargetRef,
        inOptions: OptionBits,
        outRef: *mut EventHotKeyRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn UnregisterEventHotKey(inHotKey: EventHotKeyRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CopySymbolicHotKeys(outHotKeyArray: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn PushSymbolicHotKeyMode(inOptions: OptionBits) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn PopSymbolicHotKeyMode(inToken: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn GetSymbolicHotKeyMode() -> OptionBits;
}
unsafe extern "C" {
    pub fn HIThemeDrawButton(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeButtonDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
        outLabelRect: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetButtonShape(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeButtonDrawInfo,
        outShape: *mut HIShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetButtonContentBounds(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeButtonDrawInfo,
        outBounds: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetButtonBackgroundBounds(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeButtonDrawInfo,
        outBounds: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawChasingArrows(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeChasingArrowsDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawPopupArrow(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemePopupArrowDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawMenuBarBackground(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeMenuBarDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawMenuTitle(
        inMenuBarRect: *const HIRect,
        inTitleRect: *const HIRect,
        inDrawInfo: *const HIThemeMenuTitleDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
        outLabelRect: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawMenuBackground(
        inMenuRect: *const HIRect,
        inMenuDrawInfo: *const HIThemeMenuDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawMenuItem(
        inMenuRect: *const HIRect,
        inItemRect: *const HIRect,
        inItemDrawInfo: *const HIThemeMenuItemDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
        outContentRect: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawMenuSeparator(
        inMenuRect: *const HIRect,
        inItemRect: *const HIRect,
        inItemDrawInfo: *const HIThemeMenuItemDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetMenuBackgroundShape(
        inMenuRect: *const HIRect,
        inMenuDrawInfo: *const HIThemeMenuDrawInfo,
        outShape: *mut HIShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawSegment(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeSegmentDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawTabPane(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeTabPaneDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawTab(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeTabDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
        outLabelRect: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTabPaneDrawShape(
        inRect: *const HIRect,
        inDirection: ThemeTabDirection,
        inTabSize: HIThemeTabSize,
        outShape: *mut HIShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTabPaneContentShape(
        inRect: *const HIRect,
        inDirection: ThemeTabDirection,
        inTabSize: HIThemeTabSize,
        outShape: *mut HIShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTabDrawShape(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeTabDrawInfo,
        outShape: *mut HIShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTabShape(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeTabDrawInfo,
        outShape: *mut HIShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTextDimensions(
        inString: CFTypeRef,
        inWidth: CGFloat,
        inTextInfo: *mut HIThemeTextInfo,
        outWidth: *mut CGFloat,
        outHeight: *mut CGFloat,
        outBaseline: *mut CGFloat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawTextBox(
        inString: CFTypeRef,
        inBounds: *const HIRect,
        inTextInfo: *mut HIThemeTextInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetUIFontType(inFontID: ThemeFontID) -> CTFontUIFontType;
}
unsafe extern "C" {
    pub fn HIThemeDrawTrack(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        inGhostRect: *const HIRect,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawTrackTickMarks(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        inNumTicks: ItemCount,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawTickMark(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeTickMarkDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTrackThumbShape(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        outThumbShape: *mut HIShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeHitTestTrack(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        inMousePoint: *const HIPoint,
        outPartHit: *mut ControlPartCode,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn HIThemeGetTrackBounds(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        outBounds: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTrackPartBounds(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        inPartCode: ControlPartCode,
        outPartBounds: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTrackParts(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        outNumberOfParts: *mut UInt32,
        inMaxParts: UInt32,
        ioPartsBuffer: *mut ControlPartCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTrackDragRect(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        outDragRect: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTrackThumbPositionFromOffset(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        inThumbOffset: *const HIPoint,
        outRelativePosition: *mut CGFloat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTrackThumbPositionFromBounds(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        inThumbBounds: *const HIRect,
        outRelativePosition: *mut CGFloat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTrackLiveValue(
        inDrawInfo: *const HIThemeTrackDrawInfo,
        inRelativePosition: CGFloat,
        outValue: *mut SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetScrollBarTrackRect(
        inBounds: *const HIRect,
        inTrackInfo: *const HIScrollBarTrackInfo,
        inIsHoriz: Boolean,
        outTrackBounds: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeHitTestScrollBarArrows(
        inScrollBarBounds: *const HIRect,
        inTrackInfo: *const HIScrollBarTrackInfo,
        inIsHoriz: Boolean,
        inPtHit: *const HIPoint,
        outTrackBounds: *mut HIRect,
        outPartCode: *mut ControlPartCode,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn HIThemeDrawScrollBarDelimiters(
        inContRect: *const HIRect,
        inDrawInfo: *const HIThemeScrollBarDelimitersDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawWindowFrame(
        inContRect: *const HIRect,
        inDrawInfo: *const HIThemeWindowDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
        outTitleRect: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawTitleBarWidget(
        inContRect: *const HIRect,
        inDrawInfo: *const HIThemeWindowWidgetDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawGrowBox(
        inOrigin: *const HIPoint,
        inDrawInfo: *const HIThemeGrowBoxDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetGrowBoxBounds(
        inOrigin: *const HIPoint,
        inDrawInfo: *const HIThemeGrowBoxDrawInfo,
        outBounds: *mut HIRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetWindowShape(
        inContRect: *const HIRect,
        inDrawInfo: *const HIThemeWindowDrawInfo,
        inWinRegion: WindowRegionCode,
        outShape: *mut HIShapeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetWindowRegionHit(
        inContRect: *const HIRect,
        inDrawInfo: *const HIThemeWindowDrawInfo,
        inPoint: *const HIPoint,
        outRegionHit: *mut WindowRegionCode,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn HIThemeDrawFrame(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeFrameDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawGroupBox(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeGroupBoxDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawGenericWell(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeButtonDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawPaneSplitter(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeSplitterDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawGrabber(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeGrabberDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawPlacard(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemePlacardDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawHeader(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeHeaderDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawFocusRect(
        inRect: *const HIRect,
        inHasFocus: Boolean,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeBeginFocus(
        inContext: CGContextRef,
        inRing: HIThemeFocusRing,
        inReserved: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeEndFocus(inContext: CGContextRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawSeparator(
        inRect: *const HIRect,
        inDrawInfo: *const HIThemeSeparatorDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeSetFill(
        inBrush: ThemeBrush,
        inInfo: *mut ::std::os::raw::c_void,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeSetStroke(
        inBrush: ThemeBrush,
        inInfo: *mut ::std::os::raw::c_void,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeSetTextFill(
        inColor: ThemeTextColor,
        inInfo: *mut ::std::os::raw::c_void,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeApplyBackground(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeBackgroundDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeDrawBackground(
        inBounds: *const HIRect,
        inDrawInfo: *const HIThemeBackgroundDrawInfo,
        inContext: CGContextRef,
        inOrientation: HIThemeOrientation,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeBrushCreateCGColor(inBrush: ThemeBrush, outColor: *mut CGColorRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIThemeGetTextColorForThemeBrush(
        inBrush: ThemeBrush,
        inWindowIsActive: Boolean,
        outColor: *mut ThemeTextColor,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetThemeMenuSeparatorHeight(outHeight: *mut SInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetThemeMenuItemExtra(
        inItemType: ThemeMenuItemType,
        outHeight: *mut SInt16,
        outWidth: *mut SInt16,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetThemeMenuTitleExtra(outWidth: *mut SInt16, inIsSquished: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetThemeMetric(inMetric: ThemeMetric, outMetric: *mut SInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn CopyThemeIdentifier(outIdentifier: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AESend(
        theAppleEvent: *const AppleEvent,
        reply: *mut AppleEvent,
        sendMode: AESendMode,
        sendPriority: AESendPriority,
        timeOutInTicks: SInt32,
        idleProc: AEIdleUPP,
        filterProc: AEFilterUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEProcessAppleEvent(theEventRecord: *const EventRecord) -> OSErr;
}
unsafe extern "C" {
    pub fn AEProcessEvent(inEvent: EventRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AEResetTimer(reply: *const AppleEvent) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetInteractionAllowed(level: *mut AEInteractAllowed) -> OSErr;
}
unsafe extern "C" {
    pub fn AESetInteractionAllowed(level: AEInteractAllowed) -> OSErr;
}
unsafe extern "C" {
    pub fn AEInteractWithUser(
        timeOutInTicks: SInt32,
        nmReqPtr: NMRecPtr,
        idleProc: AEIdleUPP,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AESuspendTheCurrentEvent(theAppleEvent: *const AppleEvent) -> OSErr;
}
unsafe extern "C" {
    pub fn AEResumeTheCurrentEvent(
        theAppleEvent: *const AppleEvent,
        reply: *const AppleEvent,
        dispatcher: AEEventHandlerUPP,
        handlerRefcon: SRefCon,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn AEGetTheCurrentEvent(theAppleEvent: *mut AppleEvent) -> OSErr;
}
unsafe extern "C" {
    pub fn AESetTheCurrentEvent(theAppleEvent: *const AppleEvent) -> OSErr;
}
unsafe extern "C" {
    pub fn TSMSetDocumentProperty(
        docID: TSMDocumentID,
        propertyTag: TSMDocumentPropertyTag,
        propertySize: UInt32,
        propertyData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TSMGetDocumentProperty(
        docID: TSMDocumentID,
        propertyTag: TSMDocumentPropertyTag,
        bufferSize: UInt32,
        actualSize: *mut UInt32,
        propertyBuffer: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TSMRemoveDocumentProperty(
        docID: TSMDocumentID,
        propertyTag: TSMDocumentPropertyTag,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn TSMGetActiveDocument() -> TSMDocumentID;
}
unsafe extern "C" {
    pub static kHIViewMenuContentID: HIViewID;
}
unsafe extern "C" {
    pub static kHIViewWindowContentID: HIViewID;
}
unsafe extern "C" {
    pub static kHIViewWindowGrowBoxID: HIViewID;
}
unsafe extern "C" {
    pub static kHIToolboxVersionNumber: f32;
}
unsafe extern "C" {
    pub fn SetSystemUIMode(inMode: SystemUIMode, inOptions: SystemUIOptions) -> OSStatus;
}
unsafe extern "C" {
    pub fn GetSystemUIMode(outMode: *mut SystemUIMode, outOptions: *mut SystemUIOptions);
}
unsafe extern "C" {
    pub fn GetApplicationTextEncoding() -> TextEncoding;
}
unsafe extern "C" {
    pub fn HISearchWindowShow(inSearchString: CFStringRef, inFlags: OptionBits) -> OSStatus;
}
unsafe extern "C" {
    pub fn HIDictionaryWindowShow(
        dictionary: DCSDictionaryRef,
        textString: CFTypeRef,
        selectionRange: CFRange,
        textFont: CTFontRef,
        textOrigin: CGPoint,
        verticalText: Boolean,
        viewTransform: *const CGAffineTransform,
    );
}
unsafe extern "C" {
    pub fn KBGetLayoutType(iKeyboardType: SInt16) -> PhysicalKeyboardLayoutType;
}
unsafe extern "C" {
    pub fn TISInputSourceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceCategory: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceType: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceIsASCIICapable: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceIsEnableCapable: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceIsSelectCapable: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceIsEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceIsSelected: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceID: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyBundleID: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputModeID: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyLocalizedName: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyInputSourceLanguages: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyUnicodeKeyLayoutData: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyIconRef: CFStringRef;
}
unsafe extern "C" {
    pub static kTISPropertyIconImageURL: CFStringRef;
}
unsafe extern "C" {
    pub static kTISCategoryKeyboardInputSource: CFStringRef;
}
unsafe extern "C" {
    pub static kTISCategoryPaletteInputSource: CFStringRef;
}
unsafe extern "C" {
    pub static kTISCategoryInkInputSource: CFStringRef;
}
unsafe extern "C" {
    pub static kTISTypeKeyboardLayout: CFStringRef;
}
unsafe extern "C" {
    pub static kTISTypeKeyboardInputMethodWithoutModes: CFStringRef;
}
unsafe extern "C" {
    pub static kTISTypeKeyboardInputMethodModeEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kTISTypeKeyboardInputMode: CFStringRef;
}
unsafe extern "C" {
    pub static kTISTypeCharacterPalette: CFStringRef;
}
unsafe extern "C" {
    pub static kTISTypeKeyboardViewer: CFStringRef;
}
unsafe extern "C" {
    pub static kTISTypeInk: CFStringRef;
}
unsafe extern "C" {
    pub fn TISGetInputSourceProperty(
        inputSource: TISInputSourceRef,
        propertyKey: CFStringRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn TISCreateInputSourceList(
        properties: CFDictionaryRef,
        includeAllInstalled: Boolean,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
}
unsafe extern "C" {
    pub fn TISCopyCurrentKeyboardLayoutInputSource() -> TISInputSourceRef;
}
unsafe extern "C" {
    pub fn TISCopyCurrentASCIICapableKeyboardInputSource() -> TISInputSourceRef;
}
unsafe extern "C" {
    pub fn TISCopyCurrentASCIICapableKeyboardLayoutInputSource() -> TISInputSourceRef;
}
unsafe extern "C" {
    pub fn TISCopyInputSourceForLanguage(language: CFStringRef) -> TISInputSourceRef;
}
unsafe extern "C" {
    pub fn TISCreateASCIICapableInputSourceList() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn TISSelectInputSource(inputSource: TISInputSourceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TISDeselectInputSource(inputSource: TISInputSourceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TISEnableInputSource(inputSource: TISInputSourceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TISDisableInputSource(inputSource: TISInputSourceRef) -> OSStatus;
}
unsafe extern "C" {
    pub static kTISNotifySelectedKeyboardInputSourceChanged: CFStringRef;
}
unsafe extern "C" {
    pub static kTISNotifyEnabledKeyboardInputSourcesChanged: CFStringRef;
}
unsafe extern "C" {
    pub fn TISSetInputMethodKeyboardLayoutOverride(keyboardLayout: TISInputSourceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn TISCopyInputMethodKeyboardLayoutOverride() -> TISInputSourceRef;
}
unsafe extern "C" {
    pub fn TISRegisterInputSource(location: CFURLRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DebugPrintMainEventQueue();
}
unsafe extern "C" {
    pub fn DebugPrintEvent(inEvent: EventRef);
}
unsafe extern "C" {
    pub static mut IMKTextOrientationName: NSString;
}
unsafe extern "C" {
    pub fn OSALoad(
        scriptingComponent: ComponentInstance,
        scriptData: *const AEDesc,
        modeFlags: SInt32,
        resultingScriptID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAStore(
        scriptingComponent: ComponentInstance,
        scriptID: OSAID,
        desiredType: DescType,
        modeFlags: SInt32,
        resultingScriptData: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAExecute(
        scriptingComponent: ComponentInstance,
        compiledScriptID: OSAID,
        contextID: OSAID,
        modeFlags: SInt32,
        resultingScriptValueID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSADisplay(
        scriptingComponent: ComponentInstance,
        scriptValueID: OSAID,
        desiredType: DescType,
        modeFlags: SInt32,
        resultingText: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACopyDisplayString(
        scriptingComponent: ComponentInstance,
        scriptID: OSAID,
        modeFlags: SInt32,
        result: *mut CFAttributedStringRef,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAScriptError(
        scriptingComponent: ComponentInstance,
        selector: OSType,
        desiredType: DescType,
        resultingErrorDescription: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSADispose(scriptingComponent: ComponentInstance, scriptID: OSAID) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetScriptInfo(
        scriptingComponent: ComponentInstance,
        scriptID: OSAID,
        selector: OSType,
        value: ::std::os::raw::c_long,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetScriptInfo(
        scriptingComponent: ComponentInstance,
        scriptID: OSAID,
        selector: OSType,
        result: *mut ::std::os::raw::c_long,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetActiveProc(
        scriptingComponent: ComponentInstance,
        activeProc: OSAActiveUPP,
        refCon: SRefCon,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetActiveProc(
        scriptingComponent: ComponentInstance,
        activeProc: *mut OSAActiveUPP,
        refCon: *mut SRefCon,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAScriptingComponentName(
        scriptingComponent: ComponentInstance,
        resultingScriptingComponentName: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACompile(
        scriptingComponent: ComponentInstance,
        sourceData: *const AEDesc,
        modeFlags: SInt32,
        previousAndResultingScriptID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACopyID(
        scriptingComponent: ComponentInstance,
        fromID: OSAID,
        toID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACopyScript(
        scriptingComponent: ComponentInstance,
        fromID: OSAID,
        toID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetSource(
        scriptingComponent: ComponentInstance,
        scriptID: OSAID,
        desiredType: DescType,
        resultingSourceData: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACopySourceString(
        scriptingComponent: ComponentInstance,
        scriptID: OSAID,
        modeFlags: SInt32,
        result: *mut CFAttributedStringRef,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACoerceFromDesc(
        scriptingComponent: ComponentInstance,
        scriptData: *const AEDesc,
        modeFlags: SInt32,
        resultingScriptID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACoerceToDesc(
        scriptingComponent: ComponentInstance,
        scriptID: OSAID,
        desiredType: DescType,
        modeFlags: SInt32,
        result: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetSendProc(
        scriptingComponent: ComponentInstance,
        sendProc: OSASendUPP,
        refCon: SRefCon,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetSendProc(
        scriptingComponent: ComponentInstance,
        sendProc: *mut OSASendUPP,
        refCon: *mut SRefCon,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetCreateProc(
        scriptingComponent: ComponentInstance,
        createProc: OSACreateAppleEventUPP,
        refCon: SRefCon,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetCreateProc(
        scriptingComponent: ComponentInstance,
        createProc: *mut OSACreateAppleEventUPP,
        refCon: *mut SRefCon,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetDefaultTarget(
        scriptingComponent: ComponentInstance,
        target: *const AEAddressDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAStartRecording(
        scriptingComponent: ComponentInstance,
        compiledScriptToModifyID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAStopRecording(
        scriptingComponent: ComponentInstance,
        compiledScriptID: OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSALoadExecute(
        scriptingComponent: ComponentInstance,
        scriptData: *const AEDesc,
        contextID: OSAID,
        modeFlags: SInt32,
        resultingScriptValueID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACompileExecute(
        scriptingComponent: ComponentInstance,
        sourceData: *const AEDesc,
        contextID: OSAID,
        modeFlags: SInt32,
        resultingScriptValueID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSADoScript(
        scriptingComponent: ComponentInstance,
        sourceData: *const AEDesc,
        contextID: OSAID,
        desiredType: DescType,
        modeFlags: SInt32,
        resultingText: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetCurrentDialect(
        scriptingComponent: ComponentInstance,
        dialectCode: ::std::os::raw::c_short,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetCurrentDialect(
        scriptingComponent: ComponentInstance,
        resultingDialectCode: *mut ::std::os::raw::c_short,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAAvailableDialects(
        scriptingComponent: ComponentInstance,
        resultingDialectInfoList: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetDialectInfo(
        scriptingComponent: ComponentInstance,
        dialectCode: ::std::os::raw::c_short,
        selector: OSType,
        resultingDialectInfo: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAAvailableDialectCodeList(
        scriptingComponent: ComponentInstance,
        resultingDialectCodeList: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetResumeDispatchProc(
        scriptingComponent: ComponentInstance,
        resumeDispatchProc: AEEventHandlerUPP,
        refCon: SRefCon,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetResumeDispatchProc(
        scriptingComponent: ComponentInstance,
        resumeDispatchProc: *mut AEEventHandlerUPP,
        refCon: *mut SRefCon,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAExecuteEvent(
        scriptingComponent: ComponentInstance,
        theAppleEvent: *const AppleEvent,
        contextID: OSAID,
        modeFlags: SInt32,
        resultingScriptValueID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSADoEvent(
        scriptingComponent: ComponentInstance,
        theAppleEvent: *const AppleEvent,
        contextID: OSAID,
        modeFlags: SInt32,
        reply: *mut AppleEvent,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAMakeContext(
        scriptingComponent: ComponentInstance,
        contextName: *const AEDesc,
        parentContext: OSAID,
        resultingContextID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetScriptDataFromURL(
        scriptURL: CFURLRef,
        storable: *mut Boolean,
        modeFlags: SInt32,
        resultingScriptData: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSALoadScriptData(
        scriptingComponent: ComponentInstance,
        scriptData: *const AEDesc,
        fromURL: CFURLRef,
        modeFlags: SInt32,
        resultingScriptID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSALoadFile(
        scriptingComponent: ComponentInstance,
        scriptFile: *const FSRef,
        storable: *mut Boolean,
        modeFlags: SInt32,
        resultingScriptID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAStoreFile(
        scriptingComponent: ComponentInstance,
        scriptID: OSAID,
        desiredType: DescType,
        modeFlags: SInt32,
        scriptFile: *const FSRef,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSALoadExecuteFile(
        scriptingComponent: ComponentInstance,
        scriptFile: *const FSRef,
        contextID: OSAID,
        modeFlags: SInt32,
        resultingScriptValueID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSADoScriptFile(
        scriptingComponent: ComponentInstance,
        scriptFile: *const FSRef,
        contextID: OSAID,
        desiredType: DescType,
        modeFlags: SInt32,
        resultingText: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetStorageType(scriptData: AEDataStorage, dscType: *mut DescType) -> OSErr;
}
unsafe extern "C" {
    pub fn OSAAddStorageType(scriptData: AEDataStorage, dscType: DescType) -> OSErr;
}
unsafe extern "C" {
    pub fn OSARemoveStorageType(scriptData: AEDataStorage) -> OSErr;
}
unsafe extern "C" {
    pub fn OSAGetDefaultScriptingComponent(
        genericScriptingComponent: ComponentInstance,
        scriptingSubType: *mut ScriptingComponentSelector,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetDefaultScriptingComponent(
        genericScriptingComponent: ComponentInstance,
        scriptingSubType: ScriptingComponentSelector,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetScriptingComponent(
        genericScriptingComponent: ComponentInstance,
        scriptingSubType: ScriptingComponentSelector,
        scriptingInstance: *mut ComponentInstance,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetScriptingComponentFromStored(
        genericScriptingComponent: ComponentInstance,
        scriptData: *const AEDesc,
        scriptingSubType: *mut ScriptingComponentSelector,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGenericToRealID(
        genericScriptingComponent: ComponentInstance,
        theScriptID: *mut OSAID,
        theExactComponent: *mut ComponentInstance,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSARealToGenericID(
        genericScriptingComponent: ComponentInstance,
        theScriptID: *mut OSAID,
        theExactComponent: ComponentInstance,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn ASInit(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        minStackSize: UInt32,
        preferredStackSize: UInt32,
        maxStackSize: UInt32,
        minHeapSize: UInt32,
        preferredHeapSize: UInt32,
        maxHeapSize: UInt32,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn ASSetSourceStyles(
        scriptingComponent: ComponentInstance,
        sourceStyles: STHandle,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn ASGetSourceStyles(
        scriptingComponent: ComponentInstance,
        resultingSourceStyles: *mut STHandle,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn ASCopySourceAttributes(
        scriptingComponent: ComponentInstance,
        resultingSourceAttributes: *mut CFArrayRef,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn ASSetSourceAttributes(
        scriptingComponent: ComponentInstance,
        sourceAttributes: CFArrayRef,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn ASGetSourceStyleNames(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        resultingSourceStyleNamesList: *mut AEDescList,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetProperty(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        contextID: OSAID,
        variableName: *const AEDesc,
        scriptValueID: OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetProperty(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        contextID: OSAID,
        variableName: *const AEDesc,
        resultingScriptValueID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetPropertyNames(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        contextID: OSAID,
        resultingPropertyNames: *mut AEDescList,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSASetHandler(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        contextID: OSAID,
        handlerName: *const AEDesc,
        compiledScriptID: OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetHandler(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        contextID: OSAID,
        handlerName: *const AEDesc,
        resultingCompiledScriptID: *mut OSAID,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetHandlerNames(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        contextID: OSAID,
        resultingHandlerNames: *mut AEDescList,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSAGetSysTerminology(
        scriptingComponent: ComponentInstance,
        modeFlags: SInt32,
        terminologyID: ::std::os::raw::c_short,
        terminologyList: *mut AEDesc,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACopyScriptingDefinition(
        ref_: *const FSRef,
        modeFlags: SInt32,
        sdef: *mut CFDataRef,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn OSACopyScriptingDefinitionFromURL(
        url: CFURLRef,
        modeFlags: SInt32,
        sdef: *mut CFDataRef,
    ) -> OSAError;
}
unsafe extern "C" {
    pub fn NewNColorChangedUPP(userRoutine: NColorChangedProcPtr) -> NColorChangedUPP;
}
unsafe extern "C" {
    pub fn DisposeNColorChangedUPP(userUPP: NColorChangedUPP);
}
unsafe extern "C" {
    pub fn InvokeNColorChangedUPP(
        userData: SRefCon,
        newColor: *mut NPMColor,
        userUPP: NColorChangedUPP,
    );
}
unsafe extern "C" {
    pub fn GetColor(
        where_: Point,
        prompt: ConstStr255Param,
        inColor: *const RGBColor,
        outColor: *mut RGBColor,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn NPickColor(theColorInfo: *mut NColorPickerInfo) -> OSErr;
}
unsafe extern "C" {
    pub fn NewCalibrateEventUPP(userRoutine: CalibrateEventProcPtr) -> CalibrateEventUPP;
}
unsafe extern "C" {
    pub fn NewCanCalibrateUPP(userRoutine: CanCalibrateProcPtr) -> CanCalibrateUPP;
}
unsafe extern "C" {
    pub fn NewCalibrateUPP(userRoutine: CalibrateProcPtr) -> CalibrateUPP;
}
unsafe extern "C" {
    pub fn DisposeCalibrateEventUPP(userUPP: CalibrateEventUPP);
}
unsafe extern "C" {
    pub fn DisposeCanCalibrateUPP(userUPP: CanCalibrateUPP);
}
unsafe extern "C" {
    pub fn DisposeCalibrateUPP(userUPP: CalibrateUPP);
}
unsafe extern "C" {
    pub fn InvokeCalibrateEventUPP(event: *mut EventRecord, userUPP: CalibrateEventUPP);
}
unsafe extern "C" {
    pub fn InvokeCanCalibrateUPP(
        displayID: CMDisplayIDType,
        errMessage: *mut ::std::os::raw::c_uchar,
        userUPP: CanCalibrateUPP,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn InvokeCalibrateUPP(theInfo: *mut CalibratorInfo, userUPP: CalibrateUPP) -> OSErr;
}
unsafe extern "C" {
    pub fn CMCalibrateDisplay(theInfo: *mut CalibratorInfo) -> OSErr;
}
unsafe extern "C" {
    pub static kFontPanelATSUFontIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelVariationAxesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelVariationValuesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelFeatureTypesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelFeatureSelectorsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelAttributesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelAttributeTagsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelAttributeSizesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelAttributeValuesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelMouseTrackingState: CFStringRef;
}
unsafe extern "C" {
    pub static kFontPanelBackgroundColorAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub fn FPIsFontPanelVisible() -> Boolean;
}
unsafe extern "C" {
    pub fn FPShowHideFontPanel() -> OSStatus;
}
unsafe extern "C" {
    pub fn SetFontInfoForSelection(
        iStyleType: OSType,
        iNumStyles: UInt32,
        iStyles: *mut ::std::os::raw::c_void,
        iFPEventTarget: EventTargetRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FCCopyCollectionNames() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn FCCopyFontDescriptorsInCollection(iCollection: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn FCAddCollection(iCollection: CFStringRef, iCollectionOptions: OptionBits) -> OSStatus;
}
unsafe extern "C" {
    pub fn FCRemoveCollection(iCollection: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn FCAddFontDescriptorToCollection(
        iDescriptor: FCFontDescriptorRef,
        iCollection: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn FCRemoveFontDescriptorFromCollection(
        iDescriptor: FCFontDescriptorRef,
        iCollection: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kFCFontFamilyAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kFCFontNameAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kFCFontFaceAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kFCFontSizeAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kFCFontVisibleNameAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kFCFontCGColorAttribute: CFStringRef;
}
unsafe extern "C" {
    pub fn FCFontDescriptorCreateWithFontAttributes(
        iAttributes: CFDictionaryRef,
    ) -> FCFontDescriptorRef;
}
unsafe extern "C" {
    pub fn FCFontDescriptorCreateWithName(
        iFontName: CFStringRef,
        iSize: CGFloat,
    ) -> FCFontDescriptorRef;
}
unsafe extern "C" {
    pub fn NewSRCallBackUPP(userRoutine: SRCallBackProcPtr) -> SRCallBackUPP;
}
unsafe extern "C" {
    pub fn DisposeSRCallBackUPP(userUPP: SRCallBackUPP);
}
unsafe extern "C" {
    pub fn InvokeSRCallBackUPP(param: *mut SRCallBackStruct, userUPP: SRCallBackUPP);
}
unsafe extern "C" {
    pub fn SROpenRecognitionSystem(system: *mut SRRecognitionSystem, systemID: OSType) -> OSErr;
}
unsafe extern "C" {
    pub fn SRCloseRecognitionSystem(system: SRRecognitionSystem) -> OSErr;
}
unsafe extern "C" {
    pub fn SRSetProperty(
        srObject: SRSpeechObject,
        selector: OSType,
        property: *const ::std::os::raw::c_void,
        propertyLen: Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRGetProperty(
        srObject: SRSpeechObject,
        selector: OSType,
        property: *mut ::std::os::raw::c_void,
        propertyLen: *mut Size,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRReleaseObject(srObject: SRSpeechObject) -> OSErr;
}
unsafe extern "C" {
    pub fn SRGetReference(srObject: SRSpeechObject, newObjectRef: *mut SRSpeechObject) -> OSErr;
}
unsafe extern "C" {
    pub fn SRNewRecognizer(
        system: SRRecognitionSystem,
        recognizer: *mut SRRecognizer,
        sourceID: OSType,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRStartListening(recognizer: SRRecognizer) -> OSErr;
}
unsafe extern "C" {
    pub fn SRStopListening(recognizer: SRRecognizer) -> OSErr;
}
unsafe extern "C" {
    pub fn SRSetLanguageModel(recognizer: SRRecognizer, languageModel: SRLanguageModel) -> OSErr;
}
unsafe extern "C" {
    pub fn SRGetLanguageModel(
        recognizer: SRRecognizer,
        languageModel: *mut SRLanguageModel,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRContinueRecognition(recognizer: SRRecognizer) -> OSErr;
}
unsafe extern "C" {
    pub fn SRCancelRecognition(recognizer: SRRecognizer) -> OSErr;
}
unsafe extern "C" {
    pub fn SRIdle() -> OSErr;
}
unsafe extern "C" {
    pub fn SRNewLanguageModel(
        system: SRRecognitionSystem,
        model: *mut SRLanguageModel,
        name: *const ::std::os::raw::c_void,
        nameLength: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRNewPath(system: SRRecognitionSystem, path: *mut SRPath) -> OSErr;
}
unsafe extern "C" {
    pub fn SRNewPhrase(
        system: SRRecognitionSystem,
        phrase: *mut SRPhrase,
        text: *const ::std::os::raw::c_void,
        textLength: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRNewWord(
        system: SRRecognitionSystem,
        word: *mut SRWord,
        text: *const ::std::os::raw::c_void,
        textLength: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRPutLanguageObjectIntoHandle(
        languageObject: SRLanguageObject,
        lobjHandle: Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRPutLanguageObjectIntoDataFile(
        languageObject: SRLanguageObject,
        fRefNum: ::std::os::raw::c_short,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRNewLanguageObjectFromHandle(
        system: SRRecognitionSystem,
        languageObject: *mut SRLanguageObject,
        lObjHandle: Handle,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRNewLanguageObjectFromDataFile(
        system: SRRecognitionSystem,
        languageObject: *mut SRLanguageObject,
        fRefNum: ::std::os::raw::c_short,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SREmptyLanguageObject(languageObject: SRLanguageObject) -> OSErr;
}
unsafe extern "C" {
    pub fn SRChangeLanguageObject(
        languageObject: SRLanguageObject,
        text: *const ::std::os::raw::c_void,
        textLength: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRAddLanguageObject(base: SRLanguageObject, addon: SRLanguageObject) -> OSErr;
}
unsafe extern "C" {
    pub fn SRAddText(
        base: SRLanguageObject,
        text: *const ::std::os::raw::c_void,
        textLength: SInt32,
        refCon: SRefCon,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRRemoveLanguageObject(base: SRLanguageObject, toRemove: SRLanguageObject) -> OSErr;
}
unsafe extern "C" {
    pub fn SRCountItems(container: SRSpeechObject, count: *mut ::std::os::raw::c_long) -> OSErr;
}
unsafe extern "C" {
    pub fn SRGetIndexedItem(
        container: SRSpeechObject,
        item: *mut SRSpeechObject,
        index: ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRSetIndexedItem(
        container: SRSpeechObject,
        item: SRSpeechObject,
        index: ::std::os::raw::c_long,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRRemoveIndexedItem(container: SRSpeechObject, index: ::std::os::raw::c_long) -> OSErr;
}
unsafe extern "C" {
    pub fn SRDrawText(
        recognizer: SRRecognizer,
        dispText: *const ::std::os::raw::c_void,
        dispLength: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRDrawRecognizedText(
        recognizer: SRRecognizer,
        dispText: *const ::std::os::raw::c_void,
        dispLength: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRSpeakText(
        recognizer: SRRecognizer,
        speakText: *const ::std::os::raw::c_void,
        speakLength: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRSpeakAndDrawText(
        recognizer: SRRecognizer,
        text: *const ::std::os::raw::c_void,
        textLength: SInt32,
    ) -> OSErr;
}
unsafe extern "C" {
    pub fn SRStopSpeech(recognizer: SRRecognizer) -> OSErr;
}
unsafe extern "C" {
    pub fn SRSpeechBusy(recognizer: SRRecognizer) -> Boolean;
}
unsafe extern "C" {
    pub fn SRProcessBegin(recognizer: SRRecognizer, failed: Boolean) -> OSErr;
}
unsafe extern "C" {
    pub fn SRProcessEnd(recognizer: SRRecognizer, failed: Boolean) -> OSErr;
}
unsafe extern "C" {
    pub fn KCAddInternetPassword(
        serverName: StringPtr,
        securityDomain: StringPtr,
        accountName: StringPtr,
        port: UInt16,
        protocol: OSType,
        authType: OSType,
        passwordLength: UInt32,
        passwordData: *const ::std::os::raw::c_void,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCAddInternetPasswordWithPath(
        serverName: StringPtr,
        securityDomain: StringPtr,
        accountName: StringPtr,
        path: StringPtr,
        port: UInt16,
        protocol: OSType,
        authType: OSType,
        passwordLength: UInt32,
        passwordData: *const ::std::os::raw::c_void,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCAddGenericPassword(
        serviceName: StringPtr,
        accountName: StringPtr,
        passwordLength: UInt32,
        passwordData: *const ::std::os::raw::c_void,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCAddItem(item: KCItemRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCUnlock(keychain: KCRef, password: StringPtr) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCCreateKeychain(password: StringPtr, keychain: *mut KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn KCChangeSettings(keychain: KCRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn kcunlock(keychain: KCRef, password: *const ::std::os::raw::c_char) -> OSStatus;
}
unsafe extern "C" {
    pub fn kccreatekeychain(
        password: *const ::std::os::raw::c_char,
        keychain: *mut KCRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn kcaddinternetpassword(
        serverName: *const ::std::os::raw::c_char,
        securityDomain: *const ::std::os::raw::c_char,
        accountName: *const ::std::os::raw::c_char,
        port: UInt16,
        protocol: OSType,
        authType: OSType,
        passwordLength: UInt32,
        passwordData: *const ::std::os::raw::c_void,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn kcaddinternetpasswordwithpath(
        serverName: *const ::std::os::raw::c_char,
        securityDomain: *const ::std::os::raw::c_char,
        accountName: *const ::std::os::raw::c_char,
        path: *const ::std::os::raw::c_char,
        port: UInt16,
        protocol: OSType,
        authType: OSType,
        passwordLength: UInt32,
        passwordData: *const ::std::os::raw::c_void,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn kcaddgenericpassword(
        serviceName: *const ::std::os::raw::c_char,
        accountName: *const ::std::os::raw::c_char,
        passwordLength: UInt32,
        passwordData: *const ::std::os::raw::c_void,
        item: *mut KCItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewURLNotifyUPP(userRoutine: URLNotifyProcPtr) -> URLNotifyUPP;
}
unsafe extern "C" {
    pub fn NewURLSystemEventUPP(userRoutine: URLSystemEventProcPtr) -> URLSystemEventUPP;
}
unsafe extern "C" {
    pub fn DisposeURLNotifyUPP(userUPP: URLNotifyUPP);
}
unsafe extern "C" {
    pub fn DisposeURLSystemEventUPP(userUPP: URLSystemEventUPP);
}
unsafe extern "C" {
    pub fn InvokeURLNotifyUPP(
        userContext: *mut ::std::os::raw::c_void,
        event: URLEvent,
        callbackInfo: *mut URLCallbackInfo,
        userUPP: URLNotifyUPP,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn InvokeURLSystemEventUPP(
        userContext: *mut ::std::os::raw::c_void,
        event: *mut EventRecord,
        userUPP: URLSystemEventUPP,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecChooseIdentity(
        displayInfo: CFStringRef,
        identities: CFArrayRef,
        identityRef: *mut SecIdentityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecChooseIdentityAsSheet(
        parentWindow: WindowRef,
        inTarget: EventTargetRef,
        displayInfo: CFStringRef,
        identities: CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecDisplayCertificate(
        certificate: SecCertificateRef,
        keychainList: CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecDisplayCertificateGroup(
        certificates: *const CSSM_CERTGROUP,
        keychainList: CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecEditTrust(displayInfo: CFStringRef, trust: SecTrustRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecEditTrustAsSheet(
        parentWindow: WindowRef,
        inTarget: EventTargetRef,
        displayInfo: CFStringRef,
        trust: SecTrustRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AHSearch(bookname: CFStringRef, query: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AHGotoMainTOC(toctype: AHTOCType) -> OSStatus;
}
unsafe extern "C" {
    pub fn AHGotoPage(bookname: CFStringRef, path: CFStringRef, anchor: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AHLookupAnchor(bookname: CFStringRef, anchor: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AHRegisterHelpBook(appBundleRef: *const FSRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AHRegisterHelpBookWithURL(applicationURL: CFURLRef) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for OpaqueEventRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueEventRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueEventRef", &[]);
}
unsafe impl objc2::encode::RefEncode for EventTypeSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EventTypeSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("EventTypeSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueEventLoopRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueEventLoopRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueEventLoopRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueEventQueueRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueEventQueueRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueEventQueueRef", &[]);
}
unsafe impl objc2::encode::RefEncode for __EventLoopTimer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __EventLoopTimer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__EventLoopTimer", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueEventHandlerRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueEventHandlerRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueEventHandlerRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueEventHandlerCallRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueEventHandlerCallRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueEventHandlerCallRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueEventTargetRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueEventTargetRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueEventTargetRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueHIObjectClassRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueHIObjectClassRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueHIObjectClassRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueHIObjectRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueHIObjectRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueHIObjectRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueControlRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueControlRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueControlRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueHIArchiveRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueHIArchiveRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueHIArchiveRef", &[]);
}
unsafe impl objc2::encode::RefEncode for ScrollBarTrackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScrollBarTrackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScrollBarTrackInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for SliderTrackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SliderTrackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SliderTrackInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ProgressTrackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ProgressTrackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ProgressTrackInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ThemeTrackDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ThemeTrackDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ThemeTrackDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ThemeTrackDrawInfo__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ThemeTrackDrawInfo__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ThemeTrackDrawInfo__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for ThemeButtonDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ThemeButtonDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ThemeButtonDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ThemeWindowMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ThemeWindowMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ThemeWindowMetrics", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueThemeDrawingState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueThemeDrawingState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueThemeDrawingState", &[]);
}
unsafe impl objc2::encode::RefEncode for EventRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EventRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("EventRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for EvQEl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EvQEl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("EvQEl", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMenuRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMenuRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMenuRef", &[]);
}
unsafe impl objc2::encode::RefEncode for MenuBarHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MenuBarHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MenuBarHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for HMenuBarHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMenuBarHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HMenuBarHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for MenuBarMenu {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MenuBarMenu {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MenuBarMenu", &[]);
}
unsafe impl objc2::encode::RefEncode for HMenuBarMenu {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMenuBarMenu {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HMenuBarMenu", &[]);
}
unsafe impl objc2::encode::RefEncode for MenuTrackingData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MenuTrackingData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MenuTrackingData", &[]);
}
unsafe impl objc2::encode::RefEncode for MenuItemDataRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MenuItemDataRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MenuItemDataRec", &[]);
}
unsafe impl objc2::encode::RefEncode for MenuDefSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MenuDefSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MenuDefSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for MenuDefSpec__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MenuDefSpec__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MenuDefSpec__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MenuDefSpec__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MenuDefSpec__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MenuDefSpec__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for ContextualMenuInterfaceStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ContextualMenuInterfaceStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ContextualMenuInterfaceStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for MDEFHiliteItemData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDEFHiliteItemData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDEFHiliteItemData", &[]);
}
unsafe impl objc2::encode::RefEncode for MDEFDrawData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDEFDrawData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDEFDrawData", &[]);
}
unsafe impl objc2::encode::RefEncode for MDEFFindItemData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDEFFindItemData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDEFFindItemData", &[]);
}
unsafe impl objc2::encode::RefEncode for MDEFDrawItemsData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDEFDrawItemsData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDEFDrawItemsData", &[]);
}
unsafe impl objc2::encode::RefEncode for MCEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MCEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MCEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for MenuCRsrc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MenuCRsrc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MenuCRsrc", &[]);
}
unsafe impl objc2::encode::RefEncode for TERec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TERec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TERec", &[]);
}
unsafe impl objc2::encode::RefEncode for StyleRun {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for StyleRun {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("StyleRun", &[]);
}
unsafe impl objc2::encode::RefEncode for STElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("STElement", &[]);
}
unsafe impl objc2::encode::RefEncode for LHElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LHElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("LHElement", &[]);
}
unsafe impl objc2::encode::RefEncode for ScrpSTElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScrpSTElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScrpSTElement", &[]);
}
unsafe impl objc2::encode::RefEncode for StScrpRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for StScrpRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("StScrpRec", &[]);
}
unsafe impl objc2::encode::RefEncode for NullStRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NullStRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NullStRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TEStyleRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TEStyleRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TEStyleRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TextStyle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TextStyle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TextStyle", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueDragRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueDragRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueDragRef", &[]);
}
unsafe impl objc2::encode::RefEncode for HFSFlavor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HFSFlavor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HFSFlavor", &[]);
}
unsafe impl objc2::encode::RefEncode for PromiseHFSFlavor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PromiseHFSFlavor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PromiseHFSFlavor", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlTemplate", &[]);
}
unsafe impl objc2::encode::RefEncode for CtlCTab {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CtlCTab {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CtlCTab", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlImageContentInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlImageContentInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlImageContentInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlImageContentInfo__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlImageContentInfo__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlImageContentInfo__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlFontStyleRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlFontStyleRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlFontStyleRec", &[]);
}
unsafe impl objc2::encode::RefEncode for IndicatorDragConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IndicatorDragConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IndicatorDragConstraint", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlID", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlKind {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlKind {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlKind", &[]);
}
unsafe impl objc2::encode::RefEncode for WStateData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WStateData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WStateData", &[]);
}
unsafe impl objc2::encode::RefEncode for WindowDefSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WindowDefSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WindowDefSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for WindowDefSpec__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WindowDefSpec__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WindowDefSpec__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueWindowGroupRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueWindowGroupRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueWindowGroupRef", &[]);
}
unsafe impl objc2::encode::RefEncode for TransitionWindowOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TransitionWindowOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TransitionWindowOptions", &[]);
}
unsafe impl objc2::encode::RefEncode for HIContentBorderMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIContentBorderMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIContentBorderMetrics", &[]);
}
unsafe impl objc2::encode::RefEncode for SetupWindowProxyDragImageRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SetupWindowProxyDragImageRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SetupWindowProxyDragImageRec", &[]);
}
unsafe impl objc2::encode::RefEncode for MeasureWindowTitleRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MeasureWindowTitleRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MeasureWindowTitleRec", &[]);
}
unsafe impl objc2::encode::RefEncode for GetGrowImageRegionRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GetGrowImageRegionRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GetGrowImageRegionRec", &[]);
}
unsafe impl objc2::encode::RefEncode for GetWindowRegionRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GetWindowRegionRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GetWindowRegionRec", &[]);
}
unsafe impl objc2::encode::RefEncode for WinCTab {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WinCTab {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("WinCTab", &[]);
}
unsafe impl objc2::encode::RefEncode for BasicWindowDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BasicWindowDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BasicWindowDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for BasicWindowDescription__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BasicWindowDescription__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BasicWindowDescription__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for BasicWindowDescription__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BasicWindowDescription__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BasicWindowDescription__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for BasicWindowDescription__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BasicWindowDescription__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BasicWindowDescription__bindgen_ty_1__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for HICommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HICommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HICommand", &[]);
}
unsafe impl objc2::encode::RefEncode for HICommand__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HICommand__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HICommand__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for HICommandExtended {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HICommandExtended {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HICommandExtended", &[]);
}
unsafe impl objc2::encode::RefEncode for HICommandExtended__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HICommandExtended__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HICommandExtended__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for HICommandExtended__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HICommandExtended__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HICommandExtended__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for TabletPointRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TabletPointRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TabletPointRec", &[]);
}
unsafe impl objc2::encode::RefEncode for TabletProximityRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TabletProximityRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TabletProximityRec", &[]);
}
unsafe impl objc2::encode::RefEncode for EventHotKeyID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EventHotKeyID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("EventHotKeyID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueEventHotKeyRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueEventHotKeyRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueEventHotKeyRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueToolboxObjectClassRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueToolboxObjectClassRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueToolboxObjectClassRef", &[]);
}
unsafe impl objc2::encode::RefEncode for HIScrollBarTrackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIScrollBarTrackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIScrollBarTrackInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeTrackDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeTrackDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeTrackDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeTrackDrawInfo__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeTrackDrawInfo__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeTrackDrawInfo__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeAnimationTimeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeAnimationTimeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeAnimationTimeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeAnimationFrameInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeAnimationFrameInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeAnimationFrameInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeButtonDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeButtonDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeButtonDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeButtonDrawInfo__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeButtonDrawInfo__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeButtonDrawInfo__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeSplitterDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeSplitterDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeSplitterDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeTabDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeTabDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeTabDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeTabDrawInfoVersionZero {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeTabDrawInfoVersionZero {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeTabDrawInfoVersionZero", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeTabPaneDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeTabPaneDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeTabPaneDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeTabPaneDrawInfoVersionZero {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeTabPaneDrawInfoVersionZero {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeTabPaneDrawInfoVersionZero", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeMenuDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeMenuDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeMenuDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeMenuDrawInfoVersionZero {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeMenuDrawInfoVersionZero {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeMenuDrawInfoVersionZero", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeMenuItemDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeMenuItemDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeMenuItemDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeFrameDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeFrameDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeFrameDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeGroupBoxDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeGroupBoxDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeGroupBoxDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeGrabberDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeGrabberDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeGrabberDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemePlacardDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemePlacardDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemePlacardDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeHeaderDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeHeaderDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeHeaderDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeMenuBarDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeMenuBarDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeMenuBarDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeMenuTitleDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeMenuTitleDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeMenuTitleDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeTickMarkDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeTickMarkDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeTickMarkDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeWindowDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeWindowDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeWindowDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeWindowWidgetDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeWindowWidgetDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeWindowWidgetDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeSeparatorDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeSeparatorDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeSeparatorDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeScrollBarDelimitersDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeScrollBarDelimitersDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeScrollBarDelimitersDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeChasingArrowsDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeChasingArrowsDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeChasingArrowsDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemePopupArrowDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemePopupArrowDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemePopupArrowDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeGrowBoxDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeGrowBoxDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeGrowBoxDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeBackgroundDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeBackgroundDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeBackgroundDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeSegmentDrawInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeSegmentDrawInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeSegmentDrawInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIThemeTextInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIThemeTextInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIThemeTextInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIViewFrameMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIViewFrameMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIViewFrameMetrics", &[]);
}
unsafe impl objc2::encode::RefEncode for HITypeAndCreator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HITypeAndCreator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HITypeAndCreator", &[]);
}
unsafe impl objc2::encode::RefEncode for HIViewContentInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIViewContentInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIViewContentInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for HIViewContentInfo__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIViewContentInfo__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIViewContentInfo__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for HIViewKind {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIViewKind {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIViewKind", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueHIViewTrackingAreaRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueHIViewTrackingAreaRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueHIViewTrackingAreaRef", &[]);
}
unsafe impl objc2::encode::RefEncode for HISideBinding {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HISideBinding {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HISideBinding", &[]);
}
unsafe impl objc2::encode::RefEncode for HIBinding {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIBinding {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIBinding", &[]);
}
unsafe impl objc2::encode::RefEncode for HIAxisScale {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIAxisScale {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIAxisScale", &[]);
}
unsafe impl objc2::encode::RefEncode for HIScaling {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIScaling {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIScaling", &[]);
}
unsafe impl objc2::encode::RefEncode for HIAxisPosition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIAxisPosition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIAxisPosition", &[]);
}
unsafe impl objc2::encode::RefEncode for HIPositioning {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HIPositioning {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HIPositioning", &[]);
}
unsafe impl objc2::encode::RefEncode for HILayoutInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HILayoutInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HILayoutInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for NMRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NMRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NMRec", &[]);
}
unsafe impl objc2::encode::RefEncode for DialogTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DialogTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DialogTemplate", &[]);
}
unsafe impl objc2::encode::RefEncode for AlertTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AlertTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AlertTemplate", &[]);
}
unsafe impl objc2::encode::RefEncode for AlertStdAlertParamRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AlertStdAlertParamRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AlertStdAlertParamRec", &[]);
}
unsafe impl objc2::encode::RefEncode for AlertStdCFStringAlertParamRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AlertStdCFStringAlertParamRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AlertStdCFStringAlertParamRec", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueTSMDocumentID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueTSMDocumentID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueTSMDocumentID", &[]);
}
unsafe impl objc2::encode::RefEncode for TextServiceInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TextServiceInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TextServiceInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for TextServiceList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TextServiceList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TextServiceList", &[]);
}
unsafe impl objc2::encode::RefEncode for ScriptLanguageRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScriptLanguageRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScriptLanguageRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for ScriptLanguageSupport {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScriptLanguageSupport {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScriptLanguageSupport", &[]);
}
unsafe impl objc2::encode::RefEncode for TSMGlyphInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TSMGlyphInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TSMGlyphInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for TSMGlyphInfoArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TSMGlyphInfoArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TSMGlyphInfoArray", &[]);
}
unsafe impl objc2::encode::RefEncode for ScrapFlavorInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScrapFlavorInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScrapFlavorInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueScrapRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueScrapRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueScrapRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueTXNObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueTXNObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueTXNObject", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNTab {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNTab {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNTab", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNMargins {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNMargins {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNMargins", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNControlData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNControlData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNControlData", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNATSUIFeatures {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNATSUIFeatures {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNATSUIFeatures", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNATSUIVariations {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNATSUIVariations {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNATSUIVariations", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNAttributeData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNAttributeData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNAttributeData", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNTypeAttributes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNTypeAttributes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNTypeAttributes", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNMatchTextRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNMatchTextRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNMatchTextRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNBackgroundData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNBackgroundData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNBackgroundData", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNBackground {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNBackground {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNBackground", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNCarbonEventInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNCarbonEventInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNCarbonEventInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for TXNLongRect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TXNLongRect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TXNLongRect", &[]);
}
unsafe impl objc2::encode::RefEncode for HMStringResType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMStringResType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HMStringResType", &[]);
}
unsafe impl objc2::encode::RefEncode for HMHelpContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMHelpContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HMHelpContent", &[]);
}
unsafe impl objc2::encode::RefEncode for HMHelpContent__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMHelpContent__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HMHelpContent__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for HMHelpContentRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMHelpContentRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HMHelpContentRec", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserPropertyDesc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserPropertyDesc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserPropertyDesc", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserCallbacks__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserCallbacks__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserCallbacks__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserCallbacks__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserCallbacks__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserCallbacks__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserCustomCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserCustomCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserCustomCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserCustomCallbacks__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserCustomCallbacks__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserCustomCallbacks__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserCustomCallbacks__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserCustomCallbacks__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserCustomCallbacks__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserListViewHeaderDesc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserListViewHeaderDesc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserListViewHeaderDesc", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserListViewColumnDesc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserListViewColumnDesc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserListViewColumnDesc", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserAccessibilityItemInfoV0 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserAccessibilityItemInfoV0 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserAccessibilityItemInfoV0", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserAccessibilityItemInfoV1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserAccessibilityItemInfoV1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserAccessibilityItemInfoV1", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserAccessibilityItemInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserAccessibilityItemInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserAccessibilityItemInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for DataBrowserAccessibilityItemInfo__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DataBrowserAccessibilityItemInfo__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DataBrowserAccessibilityItemInfo__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlTabEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlTabEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlTabEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlTabInfoRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlTabInfoRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlTabInfoRec", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlTabInfoRecV1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlTabInfoRecV1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlTabInfoRecV1", &[]);
}
unsafe impl objc2::encode::RefEncode for ControlEditTextSelectionRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ControlEditTextSelectionRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ControlEditTextSelectionRec", &[]);
}
unsafe impl objc2::encode::RefEncode for ListRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ListRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ListRec", &[]);
}
unsafe impl objc2::encode::RefEncode for StandardIconListCellDataRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for StandardIconListCellDataRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("StandardIconListCellDataRec", &[]);
}
unsafe impl objc2::encode::RefEncode for ListDefSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ListDefSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ListDefSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for ListDefSpec__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ListDefSpec__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ListDefSpec__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for FileTypeSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FileTypeSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FileTypeSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for FileTranslationList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FileTranslationList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FileTranslationList", &[]);
}
unsafe impl objc2::encode::RefEncode for ScrapTypeSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScrapTypeSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScrapTypeSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for ScrapTranslationList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScrapTranslationList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScrapTranslationList", &[]);
}
unsafe impl objc2::encode::RefEncode for FileTranslationSpec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FileTranslationSpec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FileTranslationSpec", &[]);
}
unsafe impl objc2::encode::RefEncode for TypeSelectRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TypeSelectRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TypeSelectRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueKeyboardLayoutRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueKeyboardLayoutRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueKeyboardLayoutRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueIBNibRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueIBNibRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueIBNibRef", &[]);
}
unsafe impl objc2::encode::RefEncode for __TISInputSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __TISInputSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__TISInputSource", &[]);
}
unsafe impl objc2::encode::RefEncode for TSMTERec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TSMTERec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TSMTERec", &[]);
}
unsafe impl objc2::encode::RefEncode for NPMColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NPMColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NPMColor", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePicker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePicker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePicker", &[]);
}
unsafe impl objc2::encode::RefEncode for PickerMenuItemInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PickerMenuItemInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PickerMenuItemInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for NColorPickerInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NColorPickerInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NColorPickerInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CalibratorInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalibratorInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CalibratorInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for FontSelectionQDStyle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FontSelectionQDStyle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FontSelectionQDStyle", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueFCFontDescriptorRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueFCFontDescriptorRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueFCFontDescriptorRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueSRSpeechObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueSRSpeechObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueSRSpeechObject", &[]);
}
unsafe impl objc2::encode::RefEncode for SRCallBackStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRCallBackStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SRCallBackStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for SRCallBackParam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRCallBackParam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SRCallBackParam", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueURLReference {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueURLReference {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueURLReference", &[]);
}
unsafe impl objc2::encode::RefEncode for URLCallbackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for URLCallbackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("URLCallbackInfo", &[]);
}
