#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use libc::{id_t, mach_port_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub trait NSCursor_JavaRuntimeSupport: Sized + std::ops::Deref {
    unsafe fn javaBusyButClickableCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), javaBusyButClickableCursor)
    }
    unsafe fn javaResizeNECursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), javaResizeNECursor)
    }
    unsafe fn javaResizeNWCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), javaResizeNWCursor)
    }
    unsafe fn javaResizeSECursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), javaResizeSECursor)
    }
    unsafe fn javaResizeSWCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), javaResizeSWCursor)
    }
    unsafe fn javaMoveCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), javaMoveCursor)
    }
    unsafe fn javaSetAllowsCursorSetInBackground_(allows: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), javaSetAllowsCursorSetInBackground : allows)
    }
}
pub trait NSEvent_JavaRuntimeSupport: Sized + std::ops::Deref {
    unsafe fn deadKeyCharacter(&self) -> unichar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deadKeyCharacter)
    }
    unsafe fn willBeHandledByComplexInputMethod(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willBeHandledByComplexInputMethod)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JRSDrag(pub id);
impl std::ops::Deref for JRSDrag {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JRSDrag {}
impl JRSDrag {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JRSDrag").unwrap(), alloc) })
    }
}
impl INSObject for JRSDrag {}
impl PNSObject for JRSDrag {}
impl std::convert::TryFrom<NSObject> for JRSDrag {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JRSDrag, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JRSDrag").unwrap()) };
        if is_kind_of {
            Ok(JRSDrag(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JRSDrag")
        }
    }
}
impl IJRSDrag for JRSDrag {}
pub trait IJRSDrag: Sized + std::ops::Deref {
    unsafe fn currentAllowableActions() -> NSDragOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSDrag").unwrap(), currentAllowableActions)
    }
    unsafe fn currentModifiers() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSDrag").unwrap(), currentModifiers)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JRSInputMethodController(pub id);
impl std::ops::Deref for JRSInputMethodController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JRSInputMethodController {}
impl JRSInputMethodController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JRSInputMethodController").unwrap(), alloc) })
    }
}
impl INSObject for JRSInputMethodController {}
impl PNSObject for JRSInputMethodController {}
impl std::convert::TryFrom<NSObject> for JRSInputMethodController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JRSInputMethodController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JRSInputMethodController").unwrap()) };
        if is_kind_of {
            Ok(JRSInputMethodController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JRSInputMethodController")
        }
    }
}
impl IJRSInputMethodController for JRSInputMethodController {}
pub trait IJRSInputMethodController: Sized + std::ops::Deref {
    unsafe fn availableInputMethodLocales(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableInputMethodLocales)
    }
    unsafe fn currentInputMethodName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentInputMethodName)
    }
    unsafe fn currentInputMethodLocale(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentInputMethodLocale)
    }
    unsafe fn setCurrentInputMethodForLocale_(&self, theLocale: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentInputMethodForLocale : theLocale)
    }
    unsafe fn controller() -> JRSInputMethodController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSInputMethodController").unwrap(), controller)
    }
}
pub trait NSWindow_JavaRuntimeSupport: Sized + std::ops::Deref {
    unsafe fn javaAddToOrderingGroup_(&self, ownedWindow: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, javaAddToOrderingGroup : ownedWindow)
    }
    unsafe fn javaRemoveFromOrderingGroup_(&self, ownedWindow: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, javaRemoveFromOrderingGroup : ownedWindow)
    }
}
pub trait PJRSMenuDelegate: Sized + std::ops::Deref {
    unsafe fn handleJavaMouseEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleJavaMouseEvent : event)
    }
    unsafe fn handleJavaMenuItemTargetedAtIndex_rect_(&self, menuIndex: NSUInteger, rect: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleJavaMenuItemTargetedAtIndex : menuIndex, rect : rect)
    }
}
pub trait NSMenu_JavaRuntimeSupport: Sized + std::ops::Deref {
    unsafe fn setJavaMenuDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJavaMenuDelegate : delegate)
    }
    unsafe fn isJavaMenu(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isJavaMenu)
    }
    unsafe fn javaMenuWithTitle_(title: NSString) -> NSMenu
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMenu").unwrap(), javaMenuWithTitle : title)
    }
}
pub type JRSFontRenderingStyle = u32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JRSAppKitAWT(pub id);
impl std::ops::Deref for JRSAppKitAWT {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JRSAppKitAWT {}
impl JRSAppKitAWT {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JRSAppKitAWT").unwrap(), alloc) })
    }
}
impl INSObject for JRSAppKitAWT {}
impl PNSObject for JRSAppKitAWT {}
impl std::convert::TryFrom<NSObject> for JRSAppKitAWT {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JRSAppKitAWT, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JRSAppKitAWT").unwrap()) };
        if is_kind_of {
            Ok(JRSAppKitAWT(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JRSAppKitAWT")
        }
    }
}
impl IJRSAppKitAWT for JRSAppKitAWT {}
pub trait IJRSAppKitAWT: Sized + std::ops::Deref {
    unsafe fn awtAppDelegate() -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSAppKitAWT").unwrap(), awtAppDelegate)
    }
    unsafe fn registerAWTAppWithOptions_(options: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSAppKitAWT").unwrap(), registerAWTAppWithOptions : options)
    }
    unsafe fn markAppIsDaemon() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSAppKitAWT").unwrap(), markAppIsDaemon)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JRSSymbolicator(pub id);
impl std::ops::Deref for JRSSymbolicator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JRSSymbolicator {}
impl JRSSymbolicator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JRSSymbolicator").unwrap(), alloc) })
    }
}
impl INSObject for JRSSymbolicator {}
impl PNSObject for JRSSymbolicator {}
impl std::convert::TryFrom<NSObject> for JRSSymbolicator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JRSSymbolicator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JRSSymbolicator").unwrap()) };
        if is_kind_of {
            Ok(JRSSymbolicator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JRSSymbolicator")
        }
    }
}
impl IJRSSymbolicator for JRSSymbolicator {}
pub trait IJRSSymbolicator: Sized + std::ops::Deref {
    unsafe fn addressForSymbol_(&self, symbolName: NSString) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addressForSymbol : symbolName)
    }
    unsafe fn symbolicatorForPid_(pid: pid_t) -> JRSSymbolicator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSSymbolicator").unwrap(), symbolicatorForPid : pid)
    }
}
pub type JRSUIRendererRef = CFTypeRef;
pub type JRSUIControlRef = CFTypeRef;
pub type JRSUIKey = CFIndex;
pub type JRSUIWidget = CFIndex;
pub type JRSUIState = CFIndex;
pub type JRSUISize = CFIndex;
pub type JRSUIDirection = CFIndex;
pub type JRSUIOrintation = CFIndex;
pub type JRSUIAlignmentHorizontal = CFIndex;
pub type JRSUIAlignmentVertical = CFIndex;
pub type JRSUISegmentPosition = CFIndex;
pub type JRSUIScrollBarPart = CFIndex;
pub type JRSUIVariant = CFIndex;
pub type JRSUIWindowType = CFIndex;
pub type JRSUIPresentationState = CFIndex;
pub type JRSUIUserInterfaceLayoutDirection = CFIndex;
pub type JRSUIPartHit = CFIndex;
pub trait PJRSRemoteLayer: Sized + std::ops::Deref {
    unsafe fn layerID(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerID)
    }
}
pub trait CALayer_JavaRuntimeSupport: Sized + std::ops::Deref {
    unsafe fn createRemoteLayerBoundTo_(&self, serverPort: mach_port_t) -> NSObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createRemoteLayerBoundTo : serverPort)
    }
    unsafe fn hostRemoteLayer_(&self, layerID: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hostRemoteLayer : layerID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JRSRenderServer(pub id);
impl std::ops::Deref for JRSRenderServer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JRSRenderServer {}
impl JRSRenderServer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JRSRenderServer").unwrap(), alloc) })
    }
}
impl INSObject for JRSRenderServer {}
impl PNSObject for JRSRenderServer {}
impl std::convert::TryFrom<NSObject> for JRSRenderServer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JRSRenderServer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JRSRenderServer").unwrap()) };
        if is_kind_of {
            Ok(JRSRenderServer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JRSRenderServer")
        }
    }
}
impl IJRSRenderServer for JRSRenderServer {}
pub trait IJRSRenderServer: Sized + std::ops::Deref {
    unsafe fn startRenderServer() -> mach_port_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSRenderServer").unwrap(), startRenderServer)
    }
    unsafe fn sendRenderServer_(serverPort: mach_port_t) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSRenderServer").unwrap(), sendRenderServer : serverPort)
    }
    unsafe fn recieveRenderServer_(serverName: NSString) -> mach_port_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JRSRenderServer").unwrap(), recieveRenderServer : serverName)
    }
}
unsafe extern "C" {
    pub fn JRSAccessibilityUnregisterUniqueIdForUIElement(element: id);
}
unsafe extern "C" {
    pub static JRSMenuDidReuseItemNotification: NSString;
}
unsafe extern "C" {
    pub fn JRSFontGetRenderingStyleForHints(
        fmHint: ::std::os::raw::c_int,
        aaHint: ::std::os::raw::c_int,
    ) -> JRSFontRenderingStyle;
}
unsafe extern "C" {
    pub fn JRSFontAlignStyleForFractionalMeasurement(
        style: JRSFontRenderingStyle,
    ) -> JRSFontRenderingStyle;
}
unsafe extern "C" {
    pub fn JRSFontAlignStyleForIntegerMeasurement(
        style: JRSFontRenderingStyle,
    ) -> JRSFontRenderingStyle;
}
unsafe extern "C" {
    pub fn JRSFontStyleUsesFractionalMetrics(style: JRSFontRenderingStyle) -> bool;
}
unsafe extern "C" {
    pub fn JRSFontStyleIsAntialiased(style: JRSFontRenderingStyle) -> bool;
}
unsafe extern "C" {
    pub fn JRSFontGetRenderingStyleForContext(context: CGContextRef) -> JRSFontRenderingStyle;
}
unsafe extern "C" {
    pub fn JRSFontSetRenderingStyleOnContext(context: CGContextRef, style: JRSFontRenderingStyle);
}
unsafe extern "C" {
    pub fn JRSFontCreateFallbackFontForCharacters(
        font: CTFontRef,
        unichars: *const UTF16Char,
        length: CFIndex,
    ) -> CTFontRef;
}
unsafe extern "C" {
    pub fn JRSFontGetAdvancesForGlyphsAndStyle(
        font: CTFontRef,
        tx: *const CGAffineTransform,
        style: JRSFontRenderingStyle,
        glyphs: *const CGGlyph,
        count: usize,
        advances: *mut CGSize,
    ) -> CGFloat;
}
unsafe extern "C" {
    pub fn JRSFontGetBoundingBoxesForGlyphsAndStyle(
        font: CTFontRef,
        tx: *const CGAffineTransform,
        style: JRSFontRenderingStyle,
        glyphs: *const CGGlyph,
        count: usize,
        bboxes: *mut CGRect,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn JRSUIRendererCreate() -> JRSUIRendererRef;
}
unsafe extern "C" {
    pub fn JRSUIRendererRelease(renderer: JRSUIRendererRef);
}
unsafe extern "C" {
    pub fn JRSUIControlCreate(isFlipped: Boolean) -> JRSUIControlRef;
}
unsafe extern "C" {
    pub fn JRSUIControlRelease(control: JRSUIControlRef);
}
unsafe extern "C" {
    pub fn JRSUIControlDraw(
        renderer: JRSUIRendererRef,
        control: JRSUIControlRef,
        context: CGContextRef,
        bounds: CGRect,
    );
}
unsafe extern "C" {
    pub fn JRSUIGetKey(value: JRSUIKey) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn JRSUIControlSetValueByKey(control: JRSUIControlRef, key: CFTypeRef, value: CFTypeRef);
}
unsafe extern "C" {
    pub fn JRSUIControlSetWidget(control: JRSUIControlRef, value: JRSUIWidget);
}
unsafe extern "C" {
    pub fn JRSUIControlSetState(control: JRSUIControlRef, value: JRSUIState);
}
unsafe extern "C" {
    pub fn JRSUIControlSetSize(control: JRSUIControlRef, value: JRSUISize);
}
unsafe extern "C" {
    pub fn JRSUIControlSetDirection(control: JRSUIControlRef, value: JRSUIDirection);
}
unsafe extern "C" {
    pub fn JRSUIControlSetOrientation(control: JRSUIControlRef, value: JRSUIOrintation);
}
unsafe extern "C" {
    pub fn JRSUIControlSetAlignmentVertical(
        control: JRSUIControlRef,
        value: JRSUIAlignmentHorizontal,
    );
}
unsafe extern "C" {
    pub fn JRSUIControlSetAlignmentHorizontal(
        control: JRSUIControlRef,
        value: JRSUIAlignmentVertical,
    );
}
unsafe extern "C" {
    pub fn JRSUIControlSetSegmentPosition(control: JRSUIControlRef, value: JRSUISegmentPosition);
}
unsafe extern "C" {
    pub fn JRSUIControlSetScrollBarPart(control: JRSUIControlRef, value: JRSUIScrollBarPart);
}
unsafe extern "C" {
    pub fn JRSUIControlSetVariant(control: JRSUIControlRef, value: JRSUIVariant);
}
unsafe extern "C" {
    pub fn JRSUIControlSetWindowType(control: JRSUIControlRef, value: JRSUIWindowType);
}
unsafe extern "C" {
    pub fn JRSUIControlSetPresentationState(
        control: JRSUIControlRef,
        value: JRSUIPresentationState,
    );
}
unsafe extern "C" {
    pub fn JRSUIControlSetUserInterfaceLayoutDirection(
        control: JRSUIControlRef,
        value: JRSUIUserInterfaceLayoutDirection,
    );
}
unsafe extern "C" {
    pub fn JRSUIControlSetShowArrows(control: JRSUIControlRef, value: Boolean);
}
unsafe extern "C" {
    pub fn JRSUIControlSetAnimating(control: JRSUIControlRef, value: Boolean);
}
unsafe extern "C" {
    pub fn JRSUIControlGetHitPart(
        renderer: JRSUIRendererRef,
        control: JRSUIControlRef,
        bounds: CGRect,
        point: CGPoint,
    ) -> JRSUIPartHit;
}
unsafe extern "C" {
    pub fn JRSUIControlShouldScrollToClick() -> Boolean;
}
unsafe extern "C" {
    pub fn JRSUIControlGetScrollBarPartBounds(
        control: JRSUIControlRef,
        frame: CGRect,
        part: JRSUIScrollBarPart,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn JRSUIControlGetScrollBarOffsetFor(
        control: JRSUIControlRef,
        frame: CGRect,
        offset: CGFloat,
        visibleAmount: CGFloat,
        extent: CGFloat,
    ) -> CGFloat;
}

unsafe impl objc2::encode::RefEncode for JRSDrag {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JRSDrag {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for JRSInputMethodController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JRSInputMethodController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for JRSAppKitAWT {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JRSAppKitAWT {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for JRSSymbolicator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JRSSymbolicator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for JRSRenderServer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JRSRenderServer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
