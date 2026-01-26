#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub trait PQLPreviewItem: Sized + std::ops::Deref {
    unsafe fn previewItemURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previewItemURL)
    }
    unsafe fn previewItemTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previewItemTitle)
    }
    unsafe fn previewItemDisplayState(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previewItemDisplayState)
    }
}
pub trait NSURL_QLPreviewConvenienceAdditions: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLPreviewPanel(pub id);
impl std::ops::Deref for QLPreviewPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLPreviewPanel {}
impl QLPreviewPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLPreviewPanel").unwrap(), alloc) })
    }
}
impl INSPanel for QLPreviewPanel {}
impl std::convert::TryFrom<NSPanel> for QLPreviewPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<QLPreviewPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLPreviewPanel").unwrap()) };
        if is_kind_of {
            Ok(QLPreviewPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to QLPreviewPanel")
        }
    }
}
impl INSWindow for QLPreviewPanel {}
impl PNSAnimatablePropertyContainer for QLPreviewPanel {}
impl PNSMenuItemValidation for QLPreviewPanel {}
impl PNSUserInterfaceValidations for QLPreviewPanel {}
impl PNSUserInterfaceItemIdentification for QLPreviewPanel {}
impl PNSAppearanceCustomization for QLPreviewPanel {}
impl PNSAccessibilityElement for QLPreviewPanel {}
impl PNSAccessibility for QLPreviewPanel {}
impl INSResponder for QLPreviewPanel {}
impl PNSCoding for QLPreviewPanel {}
impl INSObject for QLPreviewPanel {}
impl PNSObject for QLPreviewPanel {}
impl IQLPreviewPanel for QLPreviewPanel {}
pub trait IQLPreviewPanel: Sized + std::ops::Deref {
    unsafe fn updateController(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateController)
    }
    unsafe fn reloadData(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadData)
    }
    unsafe fn refreshCurrentPreviewItem(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refreshCurrentPreviewItem)
    }
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
    unsafe fn currentController(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentController)
    }
    unsafe fn dataSource(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSource)
    }
    unsafe fn setDataSource_(&self, dataSource: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataSource : dataSource)
    }
    unsafe fn currentPreviewItemIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPreviewItemIndex)
    }
    unsafe fn setCurrentPreviewItemIndex_(&self, currentPreviewItemIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentPreviewItemIndex : currentPreviewItemIndex)
    }
    unsafe fn currentPreviewItem(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPreviewItem)
    }
    unsafe fn displayState(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayState)
    }
    unsafe fn setDisplayState_(&self, displayState: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayState : displayState)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn isInFullScreenMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInFullScreenMode)
    }
    unsafe fn sharedPreviewPanel() -> QLPreviewPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QLPreviewPanel").unwrap(), sharedPreviewPanel)
    }
    unsafe fn sharedPreviewPanelExists() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QLPreviewPanel").unwrap(), sharedPreviewPanelExists)
    }
}
pub trait NSObject_QLPreviewPanelController: Sized + std::ops::Deref {
    unsafe fn acceptsPreviewPanelControl_(&self, panel: QLPreviewPanel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acceptsPreviewPanelControl : panel)
    }
    unsafe fn beginPreviewPanelControl_(&self, panel: QLPreviewPanel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginPreviewPanelControl : panel)
    }
    unsafe fn endPreviewPanelControl_(&self, panel: QLPreviewPanel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endPreviewPanelControl : panel)
    }
}
pub trait PQLPreviewPanelDataSource: Sized + std::ops::Deref {
    unsafe fn numberOfPreviewItemsInPreviewPanel_(&self, panel: QLPreviewPanel) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, numberOfPreviewItemsInPreviewPanel : panel)
    }
    unsafe fn previewPanel_previewItemAtIndex_(
        &self,
        panel: QLPreviewPanel,
        index: NSInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, previewPanel : panel, previewItemAtIndex : index)
    }
}
pub trait PQLPreviewPanelDelegate: Sized + std::ops::Deref {
    unsafe fn previewPanel_handleEvent_(&self, panel: QLPreviewPanel, event: NSEvent) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, previewPanel : panel, handleEvent : event)
    }
    unsafe fn previewPanel_sourceFrameOnScreenForPreviewItem_(
        &self,
        panel: QLPreviewPanel,
        item: *mut u64,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, previewPanel : panel, sourceFrameOnScreenForPreviewItem : item)
    }
    unsafe fn previewPanel_transitionImageForPreviewItem_contentRect_(
        &self,
        panel: QLPreviewPanel,
        item: *mut u64,
        contentRect: *mut NSRect,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, previewPanel : panel, transitionImageForPreviewItem : item, contentRect : contentRect)
    }
}
pub type QLPreviewViewStyle = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLPreviewView(pub id);
impl std::ops::Deref for QLPreviewView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLPreviewView {}
impl QLPreviewView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLPreviewView").unwrap(), alloc) })
    }
}
impl INSView for QLPreviewView {}
impl PNSAnimatablePropertyContainer for QLPreviewView {}
impl PNSUserInterfaceItemIdentification for QLPreviewView {}
impl PNSDraggingDestination for QLPreviewView {}
impl PNSAppearanceCustomization for QLPreviewView {}
impl PNSAccessibilityElement for QLPreviewView {}
impl PNSAccessibility for QLPreviewView {}
impl std::convert::TryFrom<NSView> for QLPreviewView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<QLPreviewView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLPreviewView").unwrap()) };
        if is_kind_of {
            Ok(QLPreviewView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to QLPreviewView")
        }
    }
}
impl INSResponder for QLPreviewView {}
impl PNSCoding for QLPreviewView {}
impl INSObject for QLPreviewView {}
impl PNSObject for QLPreviewView {}
impl IQLPreviewView for QLPreviewView {}
pub trait IQLPreviewView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_style_(&self, frame: NSRect, style: QLPreviewViewStyle) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame, style : style)
    }
    unsafe fn initWithFrame_(&self, frame: NSRect) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame)
    }
    unsafe fn refreshPreviewItem(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refreshPreviewItem)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn previewItem(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previewItem)
    }
    unsafe fn setPreviewItem_(&self, previewItem: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviewItem : previewItem)
    }
    unsafe fn displayState(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayState)
    }
    unsafe fn setDisplayState_(&self, displayState: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayState : displayState)
    }
    unsafe fn shouldCloseWithWindow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCloseWithWindow)
    }
    unsafe fn setShouldCloseWithWindow_(&self, shouldCloseWithWindow: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldCloseWithWindow : shouldCloseWithWindow)
    }
    unsafe fn autostarts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autostarts)
    }
    unsafe fn setAutostarts_(&self, autostarts: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutostarts : autostarts)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLPreviewReplyAttachment(pub id);
impl std::ops::Deref for QLPreviewReplyAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLPreviewReplyAttachment {}
impl QLPreviewReplyAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLPreviewReplyAttachment").unwrap(), alloc) })
    }
}
impl INSObject for QLPreviewReplyAttachment {}
impl PNSObject for QLPreviewReplyAttachment {}
impl std::convert::TryFrom<NSObject> for QLPreviewReplyAttachment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLPreviewReplyAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLPreviewReplyAttachment").unwrap()) };
        if is_kind_of {
            Ok(QLPreviewReplyAttachment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLPreviewReplyAttachment")
        }
    }
}
impl IQLPreviewReplyAttachment for QLPreviewReplyAttachment {}
pub trait IQLPreviewReplyAttachment: Sized + std::ops::Deref {
    unsafe fn initWithData_contentType_(&self, data: NSData, contentType: UTType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, contentType : contentType)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLPreviewReply(pub id);
impl std::ops::Deref for QLPreviewReply {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLPreviewReply {}
impl QLPreviewReply {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLPreviewReply").unwrap(), alloc) })
    }
}
impl INSObject for QLPreviewReply {}
impl PNSObject for QLPreviewReply {}
impl std::convert::TryFrom<NSObject> for QLPreviewReply {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLPreviewReply, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLPreviewReply").unwrap()) };
        if is_kind_of {
            Ok(QLPreviewReply(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLPreviewReply")
        }
    }
}
impl IQLPreviewReply for QLPreviewReply {}
pub trait IQLPreviewReply: Sized + std::ops::Deref {
    unsafe fn initWithContextSize_isBitmap_drawingBlock_(
        &self,
        contextSize: CGSize,
        isBitmap: BOOL,
        drawingBlock: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContextSize : contextSize, isBitmap : isBitmap, drawingBlock : drawingBlock)
    }
    unsafe fn initWithFileURL_(&self, fileURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileURL : fileURL)
    }
    unsafe fn initWithDataOfContentType_contentSize_dataCreationBlock_(
        &self,
        contentType: UTType,
        contentSize: CGSize,
        dataCreationBlock: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDataOfContentType : contentType, contentSize : contentSize, dataCreationBlock : dataCreationBlock)
    }
    unsafe fn stringEncoding(&self) -> NSStringEncoding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringEncoding)
    }
    unsafe fn setStringEncoding_(&self, stringEncoding: NSStringEncoding)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStringEncoding : stringEncoding)
    }
    unsafe fn attachments(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachments)
    }
    unsafe fn setAttachments_(&self, attachments: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachments : attachments)
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
}
impl QLPreviewReply_UI for QLPreviewReply {}
pub trait QLPreviewReply_UI: Sized + std::ops::Deref {
    unsafe fn initForPDFWithPageSize_documentCreationBlock_(
        &self,
        defaultPageSize: CGSize,
        documentCreationBlock: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForPDFWithPageSize : defaultPageSize, documentCreationBlock : documentCreationBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLPreviewProvider(pub id);
impl std::ops::Deref for QLPreviewProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLPreviewProvider {}
impl QLPreviewProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLPreviewProvider").unwrap(), alloc) })
    }
}
impl PNSExtensionRequestHandling for QLPreviewProvider {}
impl INSObject for QLPreviewProvider {}
impl PNSObject for QLPreviewProvider {}
impl std::convert::TryFrom<NSObject> for QLPreviewProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLPreviewProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLPreviewProvider").unwrap()) };
        if is_kind_of {
            Ok(QLPreviewProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLPreviewProvider")
        }
    }
}
impl IQLPreviewProvider for QLPreviewProvider {}
pub trait IQLPreviewProvider: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLFilePreviewRequest(pub id);
impl std::ops::Deref for QLFilePreviewRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLFilePreviewRequest {}
impl QLFilePreviewRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLFilePreviewRequest").unwrap(), alloc) })
    }
}
impl INSObject for QLFilePreviewRequest {}
impl PNSObject for QLFilePreviewRequest {}
impl std::convert::TryFrom<NSObject> for QLFilePreviewRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLFilePreviewRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLFilePreviewRequest").unwrap()) };
        if is_kind_of {
            Ok(QLFilePreviewRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLFilePreviewRequest")
        }
    }
}
impl IQLFilePreviewRequest for QLFilePreviewRequest {}
pub trait IQLFilePreviewRequest: Sized + std::ops::Deref {
    unsafe fn fileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileURL)
    }
}

unsafe impl objc2::encode::RefEncode for QLPreviewPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLPreviewPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLPreviewView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLPreviewView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLPreviewReplyAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLPreviewReplyAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLPreviewReply {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLPreviewReply {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLPreviewProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLPreviewProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLFilePreviewRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLFilePreviewRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
