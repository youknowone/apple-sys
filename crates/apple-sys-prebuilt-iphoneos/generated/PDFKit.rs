#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub trait PNSMenuDelegate: Sized + std::ops::Deref {
    unsafe fn menuNeedsUpdate_(&self, menu: NSMenu)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, menuNeedsUpdate : menu)
    }
    unsafe fn numberOfItemsInMenu_(&self, menu: NSMenu) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, numberOfItemsInMenu : menu)
    }
    unsafe fn menu_updateItem_atIndex_shouldCancel_(
        &self,
        menu: NSMenu,
        item: NSMenuItem,
        index: NSInteger,
        shouldCancel: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, menu : menu, updateItem : item, atIndex : index, shouldCancel : shouldCancel)
    }
    unsafe fn menuWillOpen_(&self, menu: NSMenu)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, menuWillOpen : menu)
    }
    unsafe fn menuDidClose_(&self, menu: NSMenu)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, menuDidClose : menu)
    }
    unsafe fn menu_willHighlightItem_(&self, menu: NSMenu, item: NSMenuItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, menu : menu, willHighlightItem : item)
    }
    unsafe fn confinementRectForMenu_onScreen_(&self, menu: NSMenu, screen: NSScreen) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, confinementRectForMenu : menu, onScreen : screen)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAction(pub id);
impl std::ops::Deref for PDFAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAction {}
impl PDFAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAction").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAction {}
impl INSObject for PDFAction {}
impl PNSObject for PDFAction {}
impl std::convert::TryFrom<NSObject> for PDFAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFAction, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAction").unwrap()) };
        if is_kind_of {
            Ok(PDFAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFAction")
        }
    }
}
impl IPDFAction for PDFAction {}
pub trait IPDFAction: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFActionGoTo(pub id);
impl std::ops::Deref for PDFActionGoTo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFActionGoTo {}
impl PDFActionGoTo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFActionGoTo").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFActionGoTo {}
impl IPDFAction for PDFActionGoTo {}
impl From<PDFActionGoTo> for PDFAction {
    fn from(child: PDFActionGoTo) -> PDFAction {
        PDFAction(child.0)
    }
}
impl std::convert::TryFrom<PDFAction> for PDFActionGoTo {
    type Error = &'static str;
    fn try_from(parent: PDFAction) -> Result<PDFActionGoTo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFActionGoTo").unwrap()) };
        if is_kind_of {
            Ok(PDFActionGoTo(parent.0))
        } else {
            Err("This PDFAction cannot be downcasted to PDFActionGoTo")
        }
    }
}
impl INSObject for PDFActionGoTo {}
impl PNSObject for PDFActionGoTo {}
impl IPDFActionGoTo for PDFActionGoTo {}
pub trait IPDFActionGoTo: Sized + std::ops::Deref {
    unsafe fn initWithDestination_(&self, destination: PDFDestination) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDestination : destination)
    }
    unsafe fn destination(&self) -> PDFDestination
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn setDestination_(&self, destination: PDFDestination)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestination : destination)
    }
}
pub type PDFActionNamedName = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFActionNamed(pub id);
impl std::ops::Deref for PDFActionNamed {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFActionNamed {}
impl PDFActionNamed {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFActionNamed").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFActionNamed {}
impl IPDFAction for PDFActionNamed {}
impl std::convert::TryFrom<PDFAction> for PDFActionNamed {
    type Error = &'static str;
    fn try_from(parent: PDFAction) -> Result<PDFActionNamed, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFActionNamed").unwrap()) };
        if is_kind_of {
            Ok(PDFActionNamed(parent.0))
        } else {
            Err("This PDFAction cannot be downcasted to PDFActionNamed")
        }
    }
}
impl INSObject for PDFActionNamed {}
impl PNSObject for PDFActionNamed {}
impl IPDFActionNamed for PDFActionNamed {}
pub trait IPDFActionNamed: Sized + std::ops::Deref {
    unsafe fn initWithName_(&self, name: PDFActionNamedName) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name)
    }
    unsafe fn name(&self) -> PDFActionNamedName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: PDFActionNamedName)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFActionResetForm(pub id);
impl std::ops::Deref for PDFActionResetForm {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFActionResetForm {}
impl PDFActionResetForm {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFActionResetForm").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFActionResetForm {}
impl IPDFAction for PDFActionResetForm {}
impl std::convert::TryFrom<PDFAction> for PDFActionResetForm {
    type Error = &'static str;
    fn try_from(parent: PDFAction) -> Result<PDFActionResetForm, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFActionResetForm").unwrap()) };
        if is_kind_of {
            Ok(PDFActionResetForm(parent.0))
        } else {
            Err("This PDFAction cannot be downcasted to PDFActionResetForm")
        }
    }
}
impl INSObject for PDFActionResetForm {}
impl PNSObject for PDFActionResetForm {}
impl IPDFActionResetForm for PDFActionResetForm {}
pub trait IPDFActionResetForm: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fields(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fields)
    }
    unsafe fn setFields_(&self, fields: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFields : fields)
    }
    unsafe fn fieldsIncludedAreCleared(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldsIncludedAreCleared)
    }
    unsafe fn setFieldsIncludedAreCleared_(&self, fieldsIncludedAreCleared: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldsIncludedAreCleared : fieldsIncludedAreCleared)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFActionRemoteGoTo(pub id);
impl std::ops::Deref for PDFActionRemoteGoTo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFActionRemoteGoTo {}
impl PDFActionRemoteGoTo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFActionRemoteGoTo").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFActionRemoteGoTo {}
impl IPDFAction for PDFActionRemoteGoTo {}
impl std::convert::TryFrom<PDFAction> for PDFActionRemoteGoTo {
    type Error = &'static str;
    fn try_from(parent: PDFAction) -> Result<PDFActionRemoteGoTo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFActionRemoteGoTo").unwrap()) };
        if is_kind_of {
            Ok(PDFActionRemoteGoTo(parent.0))
        } else {
            Err("This PDFAction cannot be downcasted to PDFActionRemoteGoTo")
        }
    }
}
impl INSObject for PDFActionRemoteGoTo {}
impl PNSObject for PDFActionRemoteGoTo {}
impl IPDFActionRemoteGoTo for PDFActionRemoteGoTo {}
pub trait IPDFActionRemoteGoTo: Sized + std::ops::Deref {
    unsafe fn initWithPageIndex_atPoint_fileURL_(
        &self,
        pageIndex: NSUInteger,
        point: NSPoint,
        url: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPageIndex : pageIndex, atPoint : point, fileURL : url)
    }
    unsafe fn pageIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageIndex)
    }
    unsafe fn setPageIndex_(&self, pageIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageIndex : pageIndex)
    }
    unsafe fn point(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point)
    }
    unsafe fn setPoint_(&self, point: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint : point)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFActionURL(pub id);
impl std::ops::Deref for PDFActionURL {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFActionURL {}
impl PDFActionURL {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFActionURL").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFActionURL {}
impl IPDFAction for PDFActionURL {}
impl std::convert::TryFrom<PDFAction> for PDFActionURL {
    type Error = &'static str;
    fn try_from(parent: PDFAction) -> Result<PDFActionURL, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFActionURL").unwrap()) };
        if is_kind_of {
            Ok(PDFActionURL(parent.0))
        } else {
            Err("This PDFAction cannot be downcasted to PDFActionURL")
        }
    }
}
impl INSObject for PDFActionURL {}
impl PNSObject for PDFActionURL {}
impl IPDFActionURL for PDFActionURL {}
pub trait IPDFActionURL: Sized + std::ops::Deref {
    unsafe fn initWithURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
}
pub type PDFAreaOfInterest = NSInteger;
pub type PDFDisplayBox = NSInteger;
pub type PDFPageImageInitializationOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFPage(pub id);
impl std::ops::Deref for PDFPage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFPage {}
impl PDFPage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFPage").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFPage {}
impl INSObject for PDFPage {}
impl PNSObject for PDFPage {}
impl std::convert::TryFrom<NSObject> for PDFPage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFPage, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFPage").unwrap()) };
        if is_kind_of {
            Ok(PDFPage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFPage")
        }
    }
}
impl IPDFPage for PDFPage {}
pub trait IPDFPage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithImage_options_(&self, image: NSImage, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, options : options)
    }
    unsafe fn initWithImage_(&self, image: NSImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image)
    }
    unsafe fn boundsForBox_(&self, box_: PDFDisplayBox) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundsForBox : box_)
    }
    unsafe fn setBounds_forBox_(&self, bounds: NSRect, box_: PDFDisplayBox)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBounds : bounds, forBox : box_)
    }
    unsafe fn addAnnotation_(&self, annotation: PDFAnnotation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAnnotation : annotation)
    }
    unsafe fn removeAnnotation_(&self, annotation: PDFAnnotation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAnnotation : annotation)
    }
    unsafe fn annotationAtPoint_(&self, point: NSPoint) -> PDFAnnotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, annotationAtPoint : point)
    }
    unsafe fn transformForBox_(&self, box_: PDFDisplayBox) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transformForBox : box_)
    }
    unsafe fn drawWithBox_toContext_(&self, box_: PDFDisplayBox, context: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithBox : box_, toContext : context)
    }
    unsafe fn transformContext_forBox_(&self, context: CGContextRef, box_: PDFDisplayBox)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transformContext : context, forBox : box_)
    }
    unsafe fn thumbnailOfSize_forBox_(&self, size: NSSize, box_: PDFDisplayBox) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, thumbnailOfSize : size, forBox : box_)
    }
    unsafe fn characterBoundsAtIndex_(&self, index: NSInteger) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, characterBoundsAtIndex : index)
    }
    unsafe fn characterIndexAtPoint_(&self, point: NSPoint) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, characterIndexAtPoint : point)
    }
    unsafe fn selectionForRect_(&self, rect: NSRect) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionForRect : rect)
    }
    unsafe fn selectionForWordAtPoint_(&self, point: NSPoint) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionForWordAtPoint : point)
    }
    unsafe fn selectionForLineAtPoint_(&self, point: NSPoint) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionForLineAtPoint : point)
    }
    unsafe fn selectionFromPoint_toPoint_(
        &self,
        startPoint: NSPoint,
        endPoint: NSPoint,
    ) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionFromPoint : startPoint, toPoint : endPoint)
    }
    unsafe fn selectionForRange_(&self, range: NSRange) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionForRange : range)
    }
    unsafe fn document(&self) -> PDFDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, document)
    }
    unsafe fn pageRef(&self) -> CGPDFPageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageRef)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn rotation(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn annotations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, annotations)
    }
    unsafe fn displaysAnnotations(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysAnnotations)
    }
    unsafe fn setDisplaysAnnotations_(&self, displaysAnnotations: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysAnnotations : displaysAnnotations)
    }
    unsafe fn numberOfCharacters(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfCharacters)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn attributedString(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedString)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
}
impl PDFPage_PDFPageDeprecated for PDFPage {}
pub trait PDFPage_PDFPageDeprecated: Sized + std::ops::Deref {
    unsafe fn drawWithBox_(&self, box_: PDFDisplayBox)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithBox : box_)
    }
    unsafe fn transformContextForBox_(&self, box_: PDFDisplayBox)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transformContextForBox : box_)
    }
}
pub type PDFAnnotationSubtype = NSString;
pub type PDFAnnotationKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotation(pub id);
impl std::ops::Deref for PDFAnnotation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotation {}
impl PDFAnnotation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotation").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotation {}
impl PNSCoding for PDFAnnotation {}
impl INSObject for PDFAnnotation {}
impl PNSObject for PDFAnnotation {}
impl std::convert::TryFrom<NSObject> for PDFAnnotation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFAnnotation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotation").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFAnnotation")
        }
    }
}
impl IPDFAnnotation for PDFAnnotation {}
pub trait IPDFAnnotation: Sized + std::ops::Deref {
    unsafe fn initWithBounds_forType_withProperties_(
        &self,
        bounds: NSRect,
        annotationType: NSString,
        properties: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBounds : bounds, forType : annotationType, withProperties : properties)
    }
    unsafe fn drawWithBox_inContext_(&self, box_: PDFDisplayBox, context: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithBox : box_, inContext : context)
    }
    unsafe fn setValue_forAnnotationKey_(&self, value: id, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forAnnotationKey : key)
    }
    unsafe fn setBoolean_forAnnotationKey_(&self, value: BOOL, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoolean : value, forAnnotationKey : key)
    }
    unsafe fn setRect_forAnnotationKey_(&self, value: NSRect, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRect : value, forAnnotationKey : key)
    }
    unsafe fn valueForAnnotationKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForAnnotationKey : key)
    }
    unsafe fn removeValueForAnnotationKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeValueForAnnotationKey : key)
    }
    unsafe fn page(&self) -> PDFPage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, page)
    }
    unsafe fn setPage_(&self, page: PDFPage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPage : page)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn bounds(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn setBounds_(&self, bounds: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBounds : bounds)
    }
    unsafe fn shouldDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldDisplay)
    }
    unsafe fn setShouldDisplay_(&self, shouldDisplay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldDisplay : shouldDisplay)
    }
    unsafe fn shouldPrint(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldPrint)
    }
    unsafe fn setShouldPrint_(&self, shouldPrint: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldPrint : shouldPrint)
    }
    unsafe fn modificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modificationDate)
    }
    unsafe fn setModificationDate_(&self, modificationDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModificationDate : modificationDate)
    }
    unsafe fn userName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userName)
    }
    unsafe fn setUserName_(&self, userName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserName : userName)
    }
    unsafe fn popup(&self) -> PDFAnnotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popup)
    }
    unsafe fn setPopup_(&self, popup: PDFAnnotation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPopup : popup)
    }
    unsafe fn border(&self) -> PDFBorder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, border)
    }
    unsafe fn setBorder_(&self, border: PDFBorder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorder : border)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn contents(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn action(&self) -> PDFAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn setAction_(&self, action: PDFAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAction : action)
    }
    unsafe fn hasAppearanceStream(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAppearanceStream)
    }
    unsafe fn isHighlighted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHighlighted)
    }
    unsafe fn setHighlighted_(&self, highlighted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlighted : highlighted)
    }
    unsafe fn annotationKeyValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, annotationKeyValues)
    }
}
impl PDFAnnotation_PDFAnnotationDeprecated for PDFAnnotation {}
pub trait PDFAnnotation_PDFAnnotationDeprecated: Sized + std::ops::Deref {
    unsafe fn initWithDictionary_forPage_(
        &self,
        dictionary: NSDictionary,
        page: PDFPage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : dictionary, forPage : page)
    }
    unsafe fn initWithBounds_(&self, bounds: NSRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBounds : bounds)
    }
    unsafe fn removeAllAppearanceStreams(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllAppearanceStreams)
    }
    unsafe fn drawWithBox_(&self, box_: PDFDisplayBox)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithBox : box_)
    }
    unsafe fn toolTip(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toolTip)
    }
    unsafe fn mouseUpAction(&self) -> PDFAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mouseUpAction)
    }
    unsafe fn setMouseUpAction_(&self, mouseUpAction: PDFAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMouseUpAction : mouseUpAction)
    }
}
pub type PDFLineStyle = NSInteger;
pub type PDFTextAnnotationIconType = NSInteger;
pub type PDFMarkupType = NSInteger;
pub type PDFWidgetControlType = NSInteger;
pub type PDFWidgetCellState = NSInteger;
pub type PDFAnnotationWidgetSubtype = NSString;
pub type PDFAnnotationLineEndingStyle = NSString;
pub type PDFAnnotationTextIconType = NSString;
pub type PDFAnnotationHighlightingMode = NSString;
impl PDFAnnotation_PDFAnnotationUtilities for PDFAnnotation {}
pub trait PDFAnnotation_PDFAnnotationUtilities: Sized + std::ops::Deref {
    unsafe fn addBezierPath_(&self, path: NSBezierPath)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addBezierPath : path)
    }
    unsafe fn removeBezierPath_(&self, path: NSBezierPath)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeBezierPath : path)
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
    unsafe fn fontColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontColor)
    }
    unsafe fn setFontColor_(&self, fontColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontColor : fontColor)
    }
    unsafe fn interiorColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interiorColor)
    }
    unsafe fn setInteriorColor_(&self, interiorColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteriorColor : interiorColor)
    }
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
    unsafe fn startPoint(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startPoint)
    }
    unsafe fn setStartPoint_(&self, startPoint: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartPoint : startPoint)
    }
    unsafe fn endPoint(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endPoint)
    }
    unsafe fn setEndPoint_(&self, endPoint: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndPoint : endPoint)
    }
    unsafe fn startLineStyle(&self) -> PDFLineStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startLineStyle)
    }
    unsafe fn setStartLineStyle_(&self, startLineStyle: PDFLineStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartLineStyle : startLineStyle)
    }
    unsafe fn endLineStyle(&self) -> PDFLineStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endLineStyle)
    }
    unsafe fn setEndLineStyle_(&self, endLineStyle: PDFLineStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndLineStyle : endLineStyle)
    }
    unsafe fn iconType(&self) -> PDFTextAnnotationIconType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconType)
    }
    unsafe fn setIconType_(&self, iconType: PDFTextAnnotationIconType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIconType : iconType)
    }
    unsafe fn quadrilateralPoints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quadrilateralPoints)
    }
    unsafe fn setQuadrilateralPoints_(&self, quadrilateralPoints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuadrilateralPoints : quadrilateralPoints)
    }
    unsafe fn markupType(&self) -> PDFMarkupType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, markupType)
    }
    unsafe fn setMarkupType_(&self, markupType: PDFMarkupType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarkupType : markupType)
    }
    unsafe fn widgetFieldType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widgetFieldType)
    }
    unsafe fn setWidgetFieldType_(&self, widgetFieldType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidgetFieldType : widgetFieldType)
    }
    unsafe fn widgetControlType(&self) -> PDFWidgetControlType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widgetControlType)
    }
    unsafe fn setWidgetControlType_(&self, widgetControlType: PDFWidgetControlType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidgetControlType : widgetControlType)
    }
    unsafe fn isMultiline(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMultiline)
    }
    unsafe fn setMultiline_(&self, multiline: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMultiline : multiline)
    }
    unsafe fn isActivatableTextField(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActivatableTextField)
    }
    unsafe fn isPasswordField(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPasswordField)
    }
    unsafe fn hasComb(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasComb)
    }
    unsafe fn setComb_(&self, comb: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComb : comb)
    }
    unsafe fn maximumLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumLength)
    }
    unsafe fn setMaximumLength_(&self, maximumLength: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumLength : maximumLength)
    }
    unsafe fn widgetStringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widgetStringValue)
    }
    unsafe fn setWidgetStringValue_(&self, widgetStringValue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidgetStringValue : widgetStringValue)
    }
    unsafe fn widgetDefaultStringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widgetDefaultStringValue)
    }
    unsafe fn setWidgetDefaultStringValue_(&self, widgetDefaultStringValue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidgetDefaultStringValue : widgetDefaultStringValue)
    }
    unsafe fn allowsToggleToOff(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsToggleToOff)
    }
    unsafe fn setAllowsToggleToOff_(&self, allowsToggleToOff: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsToggleToOff : allowsToggleToOff)
    }
    unsafe fn radiosInUnison(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiosInUnison)
    }
    unsafe fn setRadiosInUnison_(&self, radiosInUnison: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiosInUnison : radiosInUnison)
    }
    unsafe fn isReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadOnly)
    }
    unsafe fn setReadOnly_(&self, readOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadOnly : readOnly)
    }
    unsafe fn isListChoice(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isListChoice)
    }
    unsafe fn setListChoice_(&self, listChoice: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListChoice : listChoice)
    }
    unsafe fn choices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, choices)
    }
    unsafe fn setChoices_(&self, choices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChoices : choices)
    }
    unsafe fn values(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, values)
    }
    unsafe fn setValues_(&self, values: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValues : values)
    }
    unsafe fn buttonWidgetState(&self) -> PDFWidgetCellState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonWidgetState)
    }
    unsafe fn setButtonWidgetState_(&self, buttonWidgetState: PDFWidgetCellState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setButtonWidgetState : buttonWidgetState)
    }
    unsafe fn buttonWidgetStateString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonWidgetStateString)
    }
    unsafe fn setButtonWidgetStateString_(&self, buttonWidgetStateString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setButtonWidgetStateString : buttonWidgetStateString)
    }
    unsafe fn isOpen(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpen)
    }
    unsafe fn setOpen_(&self, open: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpen : open)
    }
    unsafe fn paths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paths)
    }
    unsafe fn destination(&self) -> PDFDestination
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn setDestination_(&self, destination: PDFDestination)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestination : destination)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
    unsafe fn fieldName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldName)
    }
    unsafe fn setFieldName_(&self, fieldName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldName : fieldName)
    }
    unsafe fn caption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, caption)
    }
    unsafe fn setCaption_(&self, caption: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaption : caption)
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
    unsafe fn stampName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stampName)
    }
    unsafe fn setStampName_(&self, stampName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStampName : stampName)
    }
    unsafe fn lineStyleFromName_(name: NSString) -> PDFLineStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotation").unwrap(), lineStyleFromName : name)
    }
    unsafe fn nameForLineStyle_(style: PDFLineStyle) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotation").unwrap(), nameForLineStyle : style)
    }
}
pub type PDFAppearanceCharacteristicsKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAppearanceCharacteristics(pub id);
impl std::ops::Deref for PDFAppearanceCharacteristics {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAppearanceCharacteristics {}
impl PDFAppearanceCharacteristics {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAppearanceCharacteristics").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAppearanceCharacteristics {}
impl INSObject for PDFAppearanceCharacteristics {}
impl PNSObject for PDFAppearanceCharacteristics {}
impl std::convert::TryFrom<NSObject> for PDFAppearanceCharacteristics {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFAppearanceCharacteristics, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAppearanceCharacteristics").unwrap()) };
        if is_kind_of {
            Ok(PDFAppearanceCharacteristics(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFAppearanceCharacteristics")
        }
    }
}
impl IPDFAppearanceCharacteristics for PDFAppearanceCharacteristics {}
pub trait IPDFAppearanceCharacteristics: Sized + std::ops::Deref {
    unsafe fn controlType(&self) -> PDFWidgetControlType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlType)
    }
    unsafe fn setControlType_(&self, controlType: PDFWidgetControlType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlType : controlType)
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
    unsafe fn borderColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderColor)
    }
    unsafe fn setBorderColor_(&self, borderColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderColor : borderColor)
    }
    unsafe fn rotation(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn caption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, caption)
    }
    unsafe fn setCaption_(&self, caption: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaption : caption)
    }
    unsafe fn rolloverCaption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rolloverCaption)
    }
    unsafe fn setRolloverCaption_(&self, rolloverCaption: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRolloverCaption : rolloverCaption)
    }
    unsafe fn downCaption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downCaption)
    }
    unsafe fn setDownCaption_(&self, downCaption: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownCaption : downCaption)
    }
    unsafe fn appearanceCharacteristicsKeyValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appearanceCharacteristicsKeyValues)
    }
}
pub type PDFBorderStyle = NSInteger;
pub type PDFBorderKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFBorder(pub id);
impl std::ops::Deref for PDFBorder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFBorder {}
impl PDFBorder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFBorder").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFBorder {}
impl PNSCoding for PDFBorder {}
impl INSObject for PDFBorder {}
impl PNSObject for PDFBorder {}
impl std::convert::TryFrom<NSObject> for PDFBorder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFBorder, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFBorder").unwrap()) };
        if is_kind_of {
            Ok(PDFBorder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFBorder")
        }
    }
}
impl IPDFBorder for PDFBorder {}
pub trait IPDFBorder: Sized + std::ops::Deref {
    unsafe fn drawInRect_(&self, rect: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawInRect : rect)
    }
    unsafe fn style(&self) -> PDFBorderStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn setStyle_(&self, style: PDFBorderStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStyle : style)
    }
    unsafe fn lineWidth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineWidth)
    }
    unsafe fn setLineWidth_(&self, lineWidth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineWidth : lineWidth)
    }
    unsafe fn dashPattern(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dashPattern)
    }
    unsafe fn setDashPattern_(&self, dashPattern: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDashPattern : dashPattern)
    }
    unsafe fn borderKeyValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderKeyValues)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFDestination(pub id);
impl std::ops::Deref for PDFDestination {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFDestination {}
impl PDFDestination {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFDestination").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFDestination {}
impl INSObject for PDFDestination {}
impl PNSObject for PDFDestination {}
impl std::convert::TryFrom<NSObject> for PDFDestination {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFDestination, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFDestination").unwrap()) };
        if is_kind_of {
            Ok(PDFDestination(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFDestination")
        }
    }
}
impl IPDFDestination for PDFDestination {}
pub trait IPDFDestination: Sized + std::ops::Deref {
    unsafe fn initWithPage_atPoint_(&self, page: PDFPage, point: NSPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPage : page, atPoint : point)
    }
    unsafe fn compare_(&self, destination: PDFDestination) -> NSComparisonResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compare : destination)
    }
    unsafe fn page(&self) -> PDFPage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, page)
    }
    unsafe fn point(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point)
    }
    unsafe fn zoom(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoom)
    }
    unsafe fn setZoom_(&self, zoom: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoom : zoom)
    }
}
pub type PDFPrintScalingMode = NSInteger;
pub type PDFDocumentPermissions = NSInteger;
pub type PDFDocumentAttribute = NSString;
pub type PDFDocumentWriteOption = NSString;
pub type PDFAccessPermissions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFDocument(pub id);
impl std::ops::Deref for PDFDocument {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFDocument {}
impl PDFDocument {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFDocument").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFDocument {}
impl INSObject for PDFDocument {}
impl PNSObject for PDFDocument {}
impl std::convert::TryFrom<NSObject> for PDFDocument {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFDocument, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFDocument").unwrap()) };
        if is_kind_of {
            Ok(PDFDocument(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFDocument")
        }
    }
}
impl IPDFDocument for PDFDocument {}
pub trait IPDFDocument: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn unlockWithPassword_(&self, password: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unlockWithPassword : password)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
    unsafe fn dataRepresentationWithOptions_(&self, options: NSDictionary) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataRepresentationWithOptions : options)
    }
    unsafe fn writeToFile_(&self, path: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToFile : path)
    }
    unsafe fn writeToFile_withOptions_(&self, path: NSString, options: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToFile : path, withOptions : options)
    }
    unsafe fn writeToURL_(&self, url: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : url)
    }
    unsafe fn writeToURL_withOptions_(&self, url: NSURL, options: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : url, withOptions : options)
    }
    unsafe fn outlineItemForSelection_(&self, selection: PDFSelection) -> PDFOutline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, outlineItemForSelection : selection)
    }
    unsafe fn pageAtIndex_(&self, index: NSUInteger) -> PDFPage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pageAtIndex : index)
    }
    unsafe fn indexForPage_(&self, page: PDFPage) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexForPage : page)
    }
    unsafe fn insertPage_atIndex_(&self, page: PDFPage, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertPage : page, atIndex : index)
    }
    unsafe fn removePageAtIndex_(&self, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePageAtIndex : index)
    }
    unsafe fn exchangePageAtIndex_withPageAtIndex_(&self, indexA: NSUInteger, indexB: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exchangePageAtIndex : indexA, withPageAtIndex : indexB)
    }
    unsafe fn findString_withOptions_(
        &self,
        string: NSString,
        options: NSStringCompareOptions,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findString : string, withOptions : options)
    }
    unsafe fn beginFindString_withOptions_(&self, string: NSString, options: NSStringCompareOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginFindString : string, withOptions : options)
    }
    unsafe fn beginFindStrings_withOptions_(
        &self,
        strings: NSArray,
        options: NSStringCompareOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginFindStrings : strings, withOptions : options)
    }
    unsafe fn findString_fromSelection_withOptions_(
        &self,
        string: NSString,
        selection: PDFSelection,
        options: NSStringCompareOptions,
    ) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findString : string, fromSelection : selection, withOptions : options)
    }
    unsafe fn cancelFindString(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelFindString)
    }
    unsafe fn printOperationForPrintInfo_scalingMode_autoRotate_(
        &self,
        printInfo: NSPrintInfo,
        scaleMode: PDFPrintScalingMode,
        doRotate: BOOL,
    ) -> NSPrintOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, printOperationForPrintInfo : printInfo, scalingMode : scaleMode, autoRotate : doRotate)
    }
    unsafe fn selectionFromPage_atPoint_toPage_atPoint_(
        &self,
        startPage: PDFPage,
        startPoint: NSPoint,
        endPage: PDFPage,
        endPoint: NSPoint,
    ) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionFromPage : startPage, atPoint : startPoint, toPage : endPage, atPoint : endPoint)
    }
    unsafe fn selectionFromPage_atPoint_toPage_atPoint_withGranularity_(
        &self,
        startPage: PDFPage,
        startPoint: NSPoint,
        endPage: PDFPage,
        endPoint: NSPoint,
        granularity: PDFSelectionGranularity,
    ) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionFromPage : startPage, atPoint : startPoint, toPage : endPage, atPoint : endPoint, withGranularity : granularity)
    }
    unsafe fn selectionFromPage_atCharacterIndex_toPage_atCharacterIndex_(
        &self,
        startPage: PDFPage,
        startCharacter: NSUInteger,
        endPage: PDFPage,
        endCharacter: NSUInteger,
    ) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionFromPage : startPage, atCharacterIndex : startCharacter, toPage : endPage, atCharacterIndex : endCharacter)
    }
    unsafe fn documentURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentURL)
    }
    unsafe fn documentRef(&self) -> CGPDFDocumentRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentRef)
    }
    unsafe fn documentAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentAttributes)
    }
    unsafe fn setDocumentAttributes_(&self, documentAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentAttributes : documentAttributes)
    }
    unsafe fn majorVersion(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, majorVersion)
    }
    unsafe fn minorVersion(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minorVersion)
    }
    unsafe fn isEncrypted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEncrypted)
    }
    unsafe fn isLocked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocked)
    }
    unsafe fn allowsPrinting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsPrinting)
    }
    unsafe fn allowsCopying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCopying)
    }
    unsafe fn allowsDocumentChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsDocumentChanges)
    }
    unsafe fn allowsDocumentAssembly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsDocumentAssembly)
    }
    unsafe fn allowsContentAccessibility(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsContentAccessibility)
    }
    unsafe fn allowsCommenting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCommenting)
    }
    unsafe fn allowsFormFieldEntry(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsFormFieldEntry)
    }
    unsafe fn accessPermissions(&self) -> PDFAccessPermissions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessPermissions)
    }
    unsafe fn permissionsStatus(&self) -> PDFDocumentPermissions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, permissionsStatus)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
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
    unsafe fn outlineRoot(&self) -> PDFOutline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outlineRoot)
    }
    unsafe fn setOutlineRoot_(&self, outlineRoot: PDFOutline)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutlineRoot : outlineRoot)
    }
    unsafe fn pageCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageCount)
    }
    unsafe fn pageClass(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageClass)
    }
    unsafe fn isFinding(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFinding)
    }
    unsafe fn selectionForEntireDocument(&self) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionForEntireDocument)
    }
}
pub trait PPDFDocumentDelegate: Sized + std::ops::Deref {
    unsafe fn documentDidUnlock_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, documentDidUnlock : notification)
    }
    unsafe fn documentDidBeginDocumentFind_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, documentDidBeginDocumentFind : notification)
    }
    unsafe fn documentDidEndDocumentFind_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, documentDidEndDocumentFind : notification)
    }
    unsafe fn documentDidBeginPageFind_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, documentDidBeginPageFind : notification)
    }
    unsafe fn documentDidEndPageFind_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, documentDidEndPageFind : notification)
    }
    unsafe fn documentDidFindMatch_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, documentDidFindMatch : notification)
    }
    unsafe fn didMatchString_(&self, instance: PDFSelection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didMatchString : instance)
    }
    unsafe fn classForPage(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classForPage)
    }
    unsafe fn classForAnnotationType_(&self, annotationType: NSString) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classForAnnotationType : annotationType)
    }
    unsafe fn classForAnnotationClass_(&self, annotationClass: Class) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classForAnnotationClass : annotationClass)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFOutline(pub id);
impl std::ops::Deref for PDFOutline {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFOutline {}
impl PDFOutline {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFOutline").unwrap(), alloc) })
    }
}
impl INSObject for PDFOutline {}
impl PNSObject for PDFOutline {}
impl std::convert::TryFrom<NSObject> for PDFOutline {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFOutline, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFOutline").unwrap()) };
        if is_kind_of {
            Ok(PDFOutline(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFOutline")
        }
    }
}
impl IPDFOutline for PDFOutline {}
pub trait IPDFOutline: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn childAtIndex_(&self, index: NSUInteger) -> PDFOutline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, childAtIndex : index)
    }
    unsafe fn insertChild_atIndex_(&self, child: PDFOutline, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertChild : child, atIndex : index)
    }
    unsafe fn removeFromParent(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeFromParent)
    }
    unsafe fn document(&self) -> PDFDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, document)
    }
    unsafe fn parent(&self) -> PDFOutline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parent)
    }
    unsafe fn numberOfChildren(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfChildren)
    }
    unsafe fn index(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, index)
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
    unsafe fn isOpen(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpen)
    }
    unsafe fn setIsOpen_(&self, isOpen: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsOpen : isOpen)
    }
    unsafe fn destination(&self) -> PDFDestination
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn setDestination_(&self, destination: PDFDestination)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestination : destination)
    }
    unsafe fn action(&self) -> PDFAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn setAction_(&self, action: PDFAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAction : action)
    }
}
pub type PDFSelectionGranularity = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFSelection(pub id);
impl std::ops::Deref for PDFSelection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFSelection {}
impl PDFSelection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFSelection").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFSelection {}
impl INSObject for PDFSelection {}
impl PNSObject for PDFSelection {}
impl std::convert::TryFrom<NSObject> for PDFSelection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PDFSelection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFSelection").unwrap()) };
        if is_kind_of {
            Ok(PDFSelection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PDFSelection")
        }
    }
}
impl IPDFSelection for PDFSelection {}
pub trait IPDFSelection: Sized + std::ops::Deref {
    unsafe fn initWithDocument_(&self, document: PDFDocument) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDocument : document)
    }
    unsafe fn boundsForPage_(&self, page: PDFPage) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundsForPage : page)
    }
    unsafe fn numberOfTextRangesOnPage_(&self, page: PDFPage) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, numberOfTextRangesOnPage : page)
    }
    unsafe fn rangeAtIndex_onPage_(&self, index: NSUInteger, page: PDFPage) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rangeAtIndex : index, onPage : page)
    }
    unsafe fn selectionsByLine(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionsByLine)
    }
    unsafe fn addSelection_(&self, selection: PDFSelection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSelection : selection)
    }
    unsafe fn addSelections_(&self, selections: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSelections : selections)
    }
    unsafe fn extendSelectionAtEnd_(&self, succeed: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendSelectionAtEnd : succeed)
    }
    unsafe fn extendSelectionAtStart_(&self, precede: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendSelectionAtStart : precede)
    }
    unsafe fn extendSelectionForLineBoundaries(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extendSelectionForLineBoundaries)
    }
    unsafe fn drawForPage_active_(&self, page: PDFPage, active: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawForPage : page, active : active)
    }
    unsafe fn drawForPage_withBox_active_(&self, page: PDFPage, box_: PDFDisplayBox, active: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawForPage : page, withBox : box_, active : active)
    }
    unsafe fn pages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pages)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn attributedString(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedString)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFThumbnailView(pub id);
impl std::ops::Deref for PDFThumbnailView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFThumbnailView {}
impl PDFThumbnailView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFThumbnailView").unwrap(), alloc) })
    }
}
impl PNSCoding for PDFThumbnailView {}
impl INSView for PDFThumbnailView {}
impl PNSAnimatablePropertyContainer for PDFThumbnailView {}
impl PNSUserInterfaceItemIdentification for PDFThumbnailView {}
impl PNSDraggingDestination for PDFThumbnailView {}
impl PNSAppearanceCustomization for PDFThumbnailView {}
impl PNSAccessibilityElement for PDFThumbnailView {}
impl PNSAccessibility for PDFThumbnailView {}
impl std::convert::TryFrom<NSView> for PDFThumbnailView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<PDFThumbnailView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFThumbnailView").unwrap()) };
        if is_kind_of {
            Ok(PDFThumbnailView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to PDFThumbnailView")
        }
    }
}
impl INSResponder for PDFThumbnailView {}
impl INSObject for PDFThumbnailView {}
impl PNSObject for PDFThumbnailView {}
impl IPDFThumbnailView for PDFThumbnailView {}
pub trait IPDFThumbnailView: Sized + std::ops::Deref {
    unsafe fn PDFView(&self) -> PDFView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PDFView)
    }
    unsafe fn setPDFView_(&self, PDFView_: PDFView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPDFView : PDFView)
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
    unsafe fn selectedPages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedPages)
    }
    unsafe fn thumbnailSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnailSize)
    }
    unsafe fn setThumbnailSize_(&self, thumbnailSize: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThumbnailSize : thumbnailSize)
    }
    unsafe fn maximumNumberOfColumns(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumNumberOfColumns)
    }
    unsafe fn setMaximumNumberOfColumns_(&self, maximumNumberOfColumns: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumNumberOfColumns : maximumNumberOfColumns)
    }
    unsafe fn labelFont(&self) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, labelFont)
    }
    unsafe fn setLabelFont_(&self, labelFont: NSFont)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabelFont : labelFont)
    }
    unsafe fn allowsDragging(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsDragging)
    }
    unsafe fn setAllowsDragging_(&self, allowsDragging: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsDragging : allowsDragging)
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
}
pub type PDFDisplayMode = NSInteger;
pub type PDFDisplayDirection = NSInteger;
pub type PDFInterpolationQuality = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFView(pub id);
impl std::ops::Deref for PDFView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFView {}
impl PDFView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFView").unwrap(), alloc) })
    }
}
impl PNSAnimationDelegate for PDFView {}
impl PNSMenuDelegate for PDFView {}
impl INSView for PDFView {}
impl PNSAnimatablePropertyContainer for PDFView {}
impl PNSUserInterfaceItemIdentification for PDFView {}
impl PNSDraggingDestination for PDFView {}
impl PNSAppearanceCustomization for PDFView {}
impl PNSAccessibilityElement for PDFView {}
impl PNSAccessibility for PDFView {}
impl std::convert::TryFrom<NSView> for PDFView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<PDFView, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFView").unwrap()) };
        if is_kind_of {
            Ok(PDFView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to PDFView")
        }
    }
}
impl INSResponder for PDFView {}
impl PNSCoding for PDFView {}
impl INSObject for PDFView {}
impl PNSObject for PDFView {}
impl IPDFView for PDFView {}
pub trait IPDFView: Sized + std::ops::Deref {
    unsafe fn goToFirstPage_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToFirstPage : sender)
    }
    unsafe fn goToLastPage_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToLastPage : sender)
    }
    unsafe fn goToNextPage_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToNextPage : sender)
    }
    unsafe fn goToPreviousPage_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToPreviousPage : sender)
    }
    unsafe fn goBack_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goBack : sender)
    }
    unsafe fn goForward_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goForward : sender)
    }
    unsafe fn goToPage_(&self, page: PDFPage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToPage : page)
    }
    unsafe fn goToDestination_(&self, destination: PDFDestination)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToDestination : destination)
    }
    unsafe fn goToSelection_(&self, selection: PDFSelection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToSelection : selection)
    }
    unsafe fn goToRect_onPage_(&self, rect: NSRect, page: PDFPage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToRect : rect, onPage : page)
    }
    unsafe fn zoomIn_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, zoomIn : sender)
    }
    unsafe fn zoomOut_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, zoomOut : sender)
    }
    unsafe fn areaOfInterestForMouse_(&self, event: NSEvent) -> PDFAreaOfInterest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, areaOfInterestForMouse : event)
    }
    unsafe fn areaOfInterestForPoint_(&self, cursorLocation: NSPoint) -> PDFAreaOfInterest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, areaOfInterestForPoint : cursorLocation)
    }
    unsafe fn setCursorForAreaOfInterest_(&self, area: PDFAreaOfInterest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCursorForAreaOfInterest : area)
    }
    unsafe fn performAction_(&self, action: PDFAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performAction : action)
    }
    unsafe fn setCurrentSelection_animate_(&self, selection: PDFSelection, animate: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentSelection : selection, animate : animate)
    }
    unsafe fn clearSelection(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearSelection)
    }
    unsafe fn selectAll_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectAll : sender)
    }
    unsafe fn scrollSelectionToVisible_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollSelectionToVisible : sender)
    }
    unsafe fn drawPage_toContext_(&self, page: PDFPage, context: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPage : page, toContext : context)
    }
    unsafe fn drawPagePost_toContext_(&self, page: PDFPage, context: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPagePost : page, toContext : context)
    }
    unsafe fn copy_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copy : sender)
    }
    unsafe fn printWithInfo_autoRotate_(&self, printInfo: NSPrintInfo, doRotate: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, printWithInfo : printInfo, autoRotate : doRotate)
    }
    unsafe fn printWithInfo_autoRotate_pageScaling_(
        &self,
        printInfo: NSPrintInfo,
        doRotate: BOOL,
        scale: PDFPrintScalingMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, printWithInfo : printInfo, autoRotate : doRotate, pageScaling : scale)
    }
    unsafe fn pageForPoint_nearest_(&self, point: NSPoint, nearest: BOOL) -> PDFPage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pageForPoint : point, nearest : nearest)
    }
    unsafe fn convertPoint_toPage_(&self, point: NSPoint, page: PDFPage) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, toPage : page)
    }
    unsafe fn convertRect_toPage_(&self, rect: NSRect, page: PDFPage) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRect : rect, toPage : page)
    }
    unsafe fn convertPoint_fromPage_(&self, point: NSPoint, page: PDFPage) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, fromPage : page)
    }
    unsafe fn convertRect_fromPage_(&self, rect: NSRect, page: PDFPage) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRect : rect, fromPage : page)
    }
    unsafe fn layoutDocumentView(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutDocumentView)
    }
    unsafe fn annotationsChangedOnPage_(&self, page: PDFPage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, annotationsChangedOnPage : page)
    }
    unsafe fn rowSizeForPage_(&self, page: PDFPage) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rowSizeForPage : page)
    }
    unsafe fn document(&self) -> PDFDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, document)
    }
    unsafe fn setDocument_(&self, document: PDFDocument)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocument : document)
    }
    unsafe fn canGoToFirstPage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoToFirstPage)
    }
    unsafe fn canGoToLastPage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoToLastPage)
    }
    unsafe fn canGoToNextPage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoToNextPage)
    }
    unsafe fn canGoToPreviousPage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoToPreviousPage)
    }
    unsafe fn canGoBack(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoBack)
    }
    unsafe fn canGoForward(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoForward)
    }
    unsafe fn currentPage(&self) -> PDFPage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPage)
    }
    unsafe fn currentDestination(&self) -> PDFDestination
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentDestination)
    }
    unsafe fn displayMode(&self) -> PDFDisplayMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayMode)
    }
    unsafe fn setDisplayMode_(&self, displayMode: PDFDisplayMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayMode : displayMode)
    }
    unsafe fn displayDirection(&self) -> PDFDisplayDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayDirection)
    }
    unsafe fn setDisplayDirection_(&self, displayDirection: PDFDisplayDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayDirection : displayDirection)
    }
    unsafe fn displaysPageBreaks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysPageBreaks)
    }
    unsafe fn setDisplaysPageBreaks_(&self, displaysPageBreaks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysPageBreaks : displaysPageBreaks)
    }
    unsafe fn pageBreakMargins(&self) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageBreakMargins)
    }
    unsafe fn setPageBreakMargins_(&self, pageBreakMargins: NSEdgeInsets)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageBreakMargins : pageBreakMargins)
    }
    unsafe fn displayBox(&self) -> PDFDisplayBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayBox)
    }
    unsafe fn setDisplayBox_(&self, displayBox: PDFDisplayBox)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayBox : displayBox)
    }
    unsafe fn displaysAsBook(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysAsBook)
    }
    unsafe fn setDisplaysAsBook_(&self, displaysAsBook: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysAsBook : displaysAsBook)
    }
    unsafe fn displaysRTL(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysRTL)
    }
    unsafe fn setDisplaysRTL_(&self, displaysRTL: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysRTL : displaysRTL)
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
    unsafe fn interpolationQuality(&self) -> PDFInterpolationQuality
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interpolationQuality)
    }
    unsafe fn setInterpolationQuality_(&self, interpolationQuality: PDFInterpolationQuality)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterpolationQuality : interpolationQuality)
    }
    unsafe fn pageShadowsEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageShadowsEnabled)
    }
    unsafe fn enablePageShadows_(&self, pageShadowsEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enablePageShadows : pageShadowsEnabled)
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
    unsafe fn pageOverlayViewProvider(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageOverlayViewProvider)
    }
    unsafe fn setPageOverlayViewProvider_(&self, pageOverlayViewProvider: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageOverlayViewProvider : pageOverlayViewProvider)
    }
    unsafe fn scaleFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn setScaleFactor_(&self, scaleFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleFactor : scaleFactor)
    }
    unsafe fn minScaleFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minScaleFactor)
    }
    unsafe fn setMinScaleFactor_(&self, minScaleFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinScaleFactor : minScaleFactor)
    }
    unsafe fn maxScaleFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxScaleFactor)
    }
    unsafe fn setMaxScaleFactor_(&self, maxScaleFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxScaleFactor : maxScaleFactor)
    }
    unsafe fn autoScales(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoScales)
    }
    unsafe fn setAutoScales_(&self, autoScales: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoScales : autoScales)
    }
    unsafe fn scaleFactorForSizeToFit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactorForSizeToFit)
    }
    unsafe fn canZoomIn(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canZoomIn)
    }
    unsafe fn canZoomOut(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canZoomOut)
    }
    unsafe fn currentSelection(&self) -> PDFSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentSelection)
    }
    unsafe fn setCurrentSelection_(&self, currentSelection: PDFSelection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentSelection : currentSelection)
    }
    unsafe fn highlightedSelections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightedSelections)
    }
    unsafe fn setHighlightedSelections_(&self, highlightedSelections: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightedSelections : highlightedSelections)
    }
    unsafe fn documentView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentView)
    }
    unsafe fn acceptsDraggedFiles(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptsDraggedFiles)
    }
    unsafe fn setAcceptsDraggedFiles_(&self, acceptsDraggedFiles: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcceptsDraggedFiles : acceptsDraggedFiles)
    }
    unsafe fn visiblePages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visiblePages)
    }
    unsafe fn enableDataDetectors(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableDataDetectors)
    }
    unsafe fn setEnableDataDetectors_(&self, enableDataDetectors: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableDataDetectors : enableDataDetectors)
    }
    unsafe fn isInMarkupMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInMarkupMode)
    }
    unsafe fn setInMarkupMode_(&self, inMarkupMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInMarkupMode : inMarkupMode)
    }
}
pub trait PPDFViewDelegate: Sized + std::ops::Deref {
    unsafe fn PDFViewWillClickOnLink_withURL_(&self, sender: PDFView, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PDFViewWillClickOnLink : sender, withURL : url)
    }
    unsafe fn PDFViewWillChangeScaleFactor_toScale_(
        &self,
        sender: PDFView,
        scaler: CGFloat,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PDFViewWillChangeScaleFactor : sender, toScale : scaler)
    }
    unsafe fn PDFViewPrintJobTitle_(&self, sender: PDFView) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PDFViewPrintJobTitle : sender)
    }
    unsafe fn PDFViewPerformPrint_(&self, sender: PDFView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PDFViewPerformPrint : sender)
    }
    unsafe fn PDFViewPerformFind_(&self, sender: PDFView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PDFViewPerformFind : sender)
    }
    unsafe fn PDFViewPerformGoToPage_(&self, sender: PDFView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PDFViewPerformGoToPage : sender)
    }
    unsafe fn PDFViewOpenPDF_forRemoteGoToAction_(
        &self,
        sender: PDFView,
        action: PDFActionRemoteGoTo,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PDFViewOpenPDF : sender, forRemoteGoToAction : action)
    }
}
impl PDFView_PDFViewDeprecated for PDFView {}
pub trait PDFView_PDFViewDeprecated: Sized + std::ops::Deref {
    unsafe fn takePasswordFrom_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, takePasswordFrom : sender)
    }
    unsafe fn drawPage_(&self, page: PDFPage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPage : page)
    }
    unsafe fn drawPagePost_(&self, page: PDFPage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPagePost : page)
    }
    unsafe fn takeBackgroundColorFrom_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, takeBackgroundColorFrom : sender)
    }
    unsafe fn shouldAntiAlias(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldAntiAlias)
    }
    unsafe fn setShouldAntiAlias_(&self, shouldAntiAlias: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldAntiAlias : shouldAntiAlias)
    }
    unsafe fn greekingThreshold(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, greekingThreshold)
    }
    unsafe fn setGreekingThreshold_(&self, greekingThreshold: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGreekingThreshold : greekingThreshold)
    }
    unsafe fn allowsDragging(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsDragging)
    }
    unsafe fn setAllowsDragging_(&self, allowsDragging: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsDragging : allowsDragging)
    }
}
pub trait PPDFPageOverlayViewProvider: Sized + std::ops::Deref {
    unsafe fn pdfView_overlayViewForPage_(&self, view: PDFView, page: PDFPage) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pdfView : view, overlayViewForPage : page)
    }
    unsafe fn pdfView_willDisplayOverlayView_forPage_(
        &self,
        pdfView: PDFView,
        overlayView: NSView,
        page: PDFPage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pdfView : pdfView, willDisplayOverlayView : overlayView, forPage : page)
    }
    unsafe fn pdfView_willEndDisplayingOverlayView_forPage_(
        &self,
        pdfView: PDFView,
        overlayView: NSView,
        page: PDFPage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pdfView : pdfView, willEndDisplayingOverlayView : overlayView, forPage : page)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationButtonWidget(pub id);
impl std::ops::Deref for PDFAnnotationButtonWidget {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationButtonWidget {}
impl PDFAnnotationButtonWidget {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationButtonWidget").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationButtonWidget {}
impl IPDFAnnotation for PDFAnnotationButtonWidget {}
impl PNSCoding for PDFAnnotationButtonWidget {}
impl From<PDFAnnotationButtonWidget> for PDFAnnotation {
    fn from(child: PDFAnnotationButtonWidget) -> PDFAnnotation {
        PDFAnnotation(child.0)
    }
}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationButtonWidget {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationButtonWidget, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationButtonWidget").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationButtonWidget(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationButtonWidget")
        }
    }
}
impl INSObject for PDFAnnotationButtonWidget {}
impl PNSObject for PDFAnnotationButtonWidget {}
impl IPDFAnnotationButtonWidget for PDFAnnotationButtonWidget {}
pub trait IPDFAnnotationButtonWidget: Sized + std::ops::Deref {
    unsafe fn controlType(&self) -> PDFWidgetControlType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlType)
    }
    unsafe fn setControlType_(&self, type_: PDFWidgetControlType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlType : type_)
    }
    unsafe fn state(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn setState_(&self, value: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setState : value)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : color)
    }
    unsafe fn allowsToggleToOff(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsToggleToOff)
    }
    unsafe fn setAllowsToggleToOff_(&self, allowOff: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsToggleToOff : allowOff)
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
    unsafe fn fontColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontColor)
    }
    unsafe fn setFontColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontColor : color)
    }
    unsafe fn caption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, caption)
    }
    unsafe fn setCaption_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaption : name)
    }
    unsafe fn fieldName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldName)
    }
    unsafe fn setFieldName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldName : name)
    }
    unsafe fn onStateValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, onStateValue)
    }
    unsafe fn setOnStateValue_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnStateValue : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationChoiceWidget(pub id);
impl std::ops::Deref for PDFAnnotationChoiceWidget {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationChoiceWidget {}
impl PDFAnnotationChoiceWidget {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationChoiceWidget").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationChoiceWidget {}
impl IPDFAnnotation for PDFAnnotationChoiceWidget {}
impl PNSCoding for PDFAnnotationChoiceWidget {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationChoiceWidget {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationChoiceWidget, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationChoiceWidget").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationChoiceWidget(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationChoiceWidget")
        }
    }
}
impl INSObject for PDFAnnotationChoiceWidget {}
impl PNSObject for PDFAnnotationChoiceWidget {}
impl IPDFAnnotationChoiceWidget for PDFAnnotationChoiceWidget {}
pub trait IPDFAnnotationChoiceWidget: Sized + std::ops::Deref {
    unsafe fn stringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringValue)
    }
    unsafe fn setStringValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStringValue : value)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : color)
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
    unsafe fn fontColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontColor)
    }
    unsafe fn setFontColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontColor : color)
    }
    unsafe fn fieldName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldName)
    }
    unsafe fn setFieldName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldName : name)
    }
    unsafe fn isListChoice(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isListChoice)
    }
    unsafe fn setIsListChoice_(&self, isList: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsListChoice : isList)
    }
    unsafe fn choices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, choices)
    }
    unsafe fn setChoices_(&self, options: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChoices : options)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationCircle(pub id);
impl std::ops::Deref for PDFAnnotationCircle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationCircle {}
impl PDFAnnotationCircle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationCircle").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationCircle {}
impl IPDFAnnotation for PDFAnnotationCircle {}
impl PNSCoding for PDFAnnotationCircle {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationCircle {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationCircle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationCircle").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationCircle(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationCircle")
        }
    }
}
impl INSObject for PDFAnnotationCircle {}
impl PNSObject for PDFAnnotationCircle {}
impl IPDFAnnotationCircle for PDFAnnotationCircle {}
pub trait IPDFAnnotationCircle: Sized + std::ops::Deref {
    unsafe fn interiorColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interiorColor)
    }
    unsafe fn setInteriorColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteriorColor : color)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationFreeText(pub id);
impl std::ops::Deref for PDFAnnotationFreeText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationFreeText {}
impl PDFAnnotationFreeText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationFreeText").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationFreeText {}
impl PNSCoding for PDFAnnotationFreeText {}
impl IPDFAnnotation for PDFAnnotationFreeText {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationFreeText {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationFreeText, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationFreeText").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationFreeText(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationFreeText")
        }
    }
}
impl INSObject for PDFAnnotationFreeText {}
impl PNSObject for PDFAnnotationFreeText {}
impl IPDFAnnotationFreeText for PDFAnnotationFreeText {}
pub trait IPDFAnnotationFreeText: Sized + std::ops::Deref {
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
    unsafe fn fontColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontColor)
    }
    unsafe fn setFontColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontColor : color)
    }
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationInk(pub id);
impl std::ops::Deref for PDFAnnotationInk {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationInk {}
impl PDFAnnotationInk {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationInk").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationInk {}
impl PNSCoding for PDFAnnotationInk {}
impl IPDFAnnotation for PDFAnnotationInk {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationInk {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationInk, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationInk").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationInk(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationInk")
        }
    }
}
impl INSObject for PDFAnnotationInk {}
impl PNSObject for PDFAnnotationInk {}
impl IPDFAnnotationInk for PDFAnnotationInk {}
pub trait IPDFAnnotationInk: Sized + std::ops::Deref {
    unsafe fn paths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paths)
    }
    unsafe fn addBezierPath_(&self, path: NSBezierPath)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addBezierPath : path)
    }
    unsafe fn removeBezierPath_(&self, path: NSBezierPath)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeBezierPath : path)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationLine(pub id);
impl std::ops::Deref for PDFAnnotationLine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationLine {}
impl PDFAnnotationLine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationLine").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationLine {}
impl PNSCoding for PDFAnnotationLine {}
impl IPDFAnnotation for PDFAnnotationLine {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationLine {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationLine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationLine").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationLine(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationLine")
        }
    }
}
impl INSObject for PDFAnnotationLine {}
impl PNSObject for PDFAnnotationLine {}
impl IPDFAnnotationLine for PDFAnnotationLine {}
pub trait IPDFAnnotationLine: Sized + std::ops::Deref {
    unsafe fn startPoint(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startPoint)
    }
    unsafe fn setStartPoint_(&self, point: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartPoint : point)
    }
    unsafe fn endPoint(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endPoint)
    }
    unsafe fn setEndPoint_(&self, point: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndPoint : point)
    }
    unsafe fn startLineStyle(&self) -> PDFLineStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startLineStyle)
    }
    unsafe fn setStartLineStyle_(&self, style: PDFLineStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartLineStyle : style)
    }
    unsafe fn endLineStyle(&self) -> PDFLineStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endLineStyle)
    }
    unsafe fn setEndLineStyle_(&self, style: PDFLineStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndLineStyle : style)
    }
    unsafe fn interiorColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interiorColor)
    }
    unsafe fn setInteriorColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteriorColor : color)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationLink(pub id);
impl std::ops::Deref for PDFAnnotationLink {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationLink {}
impl PDFAnnotationLink {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationLink").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationLink {}
impl IPDFAnnotation for PDFAnnotationLink {}
impl PNSCoding for PDFAnnotationLink {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationLink {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationLink, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationLink").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationLink(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationLink")
        }
    }
}
impl INSObject for PDFAnnotationLink {}
impl PNSObject for PDFAnnotationLink {}
impl IPDFAnnotationLink for PDFAnnotationLink {}
pub trait IPDFAnnotationLink: Sized + std::ops::Deref {
    unsafe fn destination(&self) -> PDFDestination
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn setDestination_(&self, destination: PDFDestination)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestination : destination)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : url)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationMarkup(pub id);
impl std::ops::Deref for PDFAnnotationMarkup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationMarkup {}
impl PDFAnnotationMarkup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationMarkup").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationMarkup {}
impl PNSCoding for PDFAnnotationMarkup {}
impl IPDFAnnotation for PDFAnnotationMarkup {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationMarkup {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationMarkup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationMarkup").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationMarkup(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationMarkup")
        }
    }
}
impl INSObject for PDFAnnotationMarkup {}
impl PNSObject for PDFAnnotationMarkup {}
impl IPDFAnnotationMarkup for PDFAnnotationMarkup {}
pub trait IPDFAnnotationMarkup: Sized + std::ops::Deref {
    unsafe fn quadrilateralPoints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quadrilateralPoints)
    }
    unsafe fn setQuadrilateralPoints_(&self, points: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuadrilateralPoints : points)
    }
    unsafe fn markupType(&self) -> PDFMarkupType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, markupType)
    }
    unsafe fn setMarkupType_(&self, type_: PDFMarkupType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarkupType : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationPopup(pub id);
impl std::ops::Deref for PDFAnnotationPopup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationPopup {}
impl PDFAnnotationPopup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationPopup").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationPopup {}
impl PNSCoding for PDFAnnotationPopup {}
impl IPDFAnnotation for PDFAnnotationPopup {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationPopup {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationPopup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationPopup").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationPopup(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationPopup")
        }
    }
}
impl INSObject for PDFAnnotationPopup {}
impl PNSObject for PDFAnnotationPopup {}
impl IPDFAnnotationPopup for PDFAnnotationPopup {}
pub trait IPDFAnnotationPopup: Sized + std::ops::Deref {
    unsafe fn isOpen(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpen)
    }
    unsafe fn setIsOpen_(&self, isOpen: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsOpen : isOpen)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationSquare(pub id);
impl std::ops::Deref for PDFAnnotationSquare {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationSquare {}
impl PDFAnnotationSquare {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationSquare").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationSquare {}
impl PNSCoding for PDFAnnotationSquare {}
impl IPDFAnnotation for PDFAnnotationSquare {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationSquare {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationSquare, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationSquare").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationSquare(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationSquare")
        }
    }
}
impl INSObject for PDFAnnotationSquare {}
impl PNSObject for PDFAnnotationSquare {}
impl IPDFAnnotationSquare for PDFAnnotationSquare {}
pub trait IPDFAnnotationSquare: Sized + std::ops::Deref {
    unsafe fn interiorColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interiorColor)
    }
    unsafe fn setInteriorColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteriorColor : color)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationStamp(pub id);
impl std::ops::Deref for PDFAnnotationStamp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationStamp {}
impl PDFAnnotationStamp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationStamp").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationStamp {}
impl IPDFAnnotation for PDFAnnotationStamp {}
impl PNSCoding for PDFAnnotationStamp {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationStamp {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationStamp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationStamp").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationStamp(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationStamp")
        }
    }
}
impl INSObject for PDFAnnotationStamp {}
impl PNSObject for PDFAnnotationStamp {}
impl IPDFAnnotationStamp for PDFAnnotationStamp {}
pub trait IPDFAnnotationStamp: Sized + std::ops::Deref {
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationText(pub id);
impl std::ops::Deref for PDFAnnotationText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationText {}
impl PDFAnnotationText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationText").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationText {}
impl PNSCoding for PDFAnnotationText {}
impl IPDFAnnotation for PDFAnnotationText {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationText {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationText, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationText").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationText(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationText")
        }
    }
}
impl INSObject for PDFAnnotationText {}
impl PNSObject for PDFAnnotationText {}
impl IPDFAnnotationText for PDFAnnotationText {}
pub trait IPDFAnnotationText: Sized + std::ops::Deref {
    unsafe fn iconType(&self) -> PDFTextAnnotationIconType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconType)
    }
    unsafe fn setIconType_(&self, type_: PDFTextAnnotationIconType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIconType : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PDFAnnotationTextWidget(pub id);
impl std::ops::Deref for PDFAnnotationTextWidget {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PDFAnnotationTextWidget {}
impl PDFAnnotationTextWidget {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PDFAnnotationTextWidget").unwrap(), alloc) })
    }
}
impl PNSCopying for PDFAnnotationTextWidget {}
impl IPDFAnnotation for PDFAnnotationTextWidget {}
impl PNSCoding for PDFAnnotationTextWidget {}
impl std::convert::TryFrom<PDFAnnotation> for PDFAnnotationTextWidget {
    type Error = &'static str;
    fn try_from(parent: PDFAnnotation) -> Result<PDFAnnotationTextWidget, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PDFAnnotationTextWidget").unwrap()) };
        if is_kind_of {
            Ok(PDFAnnotationTextWidget(parent.0))
        } else {
            Err("This PDFAnnotation cannot be downcasted to PDFAnnotationTextWidget")
        }
    }
}
impl INSObject for PDFAnnotationTextWidget {}
impl PNSObject for PDFAnnotationTextWidget {}
impl IPDFAnnotationTextWidget for PDFAnnotationTextWidget {}
pub trait IPDFAnnotationTextWidget: Sized + std::ops::Deref {
    unsafe fn stringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringValue)
    }
    unsafe fn setStringValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStringValue : value)
    }
    unsafe fn attributedStringValue(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedStringValue)
    }
    unsafe fn setAttributedStringValue_(&self, value: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedStringValue : value)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : color)
    }
    unsafe fn rotation(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
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
    unsafe fn fontColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontColor)
    }
    unsafe fn setFontColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontColor : color)
    }
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
    unsafe fn maximumLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumLength)
    }
    unsafe fn setMaximumLength_(&self, maxLen: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumLength : maxLen)
    }
    unsafe fn fieldName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldName)
    }
    unsafe fn setFieldName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldName : name)
    }
    unsafe fn isMultiline(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMultiline)
    }
    unsafe fn setIsMultiline_(&self, multiline: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsMultiline : multiline)
    }
}
unsafe extern "C" {
    pub static PDFPageImageInitializationOptionMediaBox: PDFPageImageInitializationOption;
}
unsafe extern "C" {
    pub static PDFPageImageInitializationOptionRotation: PDFPageImageInitializationOption;
}
unsafe extern "C" {
    pub static PDFPageImageInitializationOptionUpscaleIfSmaller: PDFPageImageInitializationOption;
}
unsafe extern "C" {
    pub static PDFPageImageInitializationOptionCompressionQuality: PDFPageImageInitializationOption;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyAppearanceDictionary: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyAppearanceState: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyBorder: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyColor: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyContents: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyFlags: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyDate: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyName: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyPage: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyRect: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeySubtype: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyAction: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyAdditionalActions: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyBorderStyle: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyDefaultAppearance: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyDestination: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyHighlightingMode: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyInklist: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyInteriorColor: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyLinePoints: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyLineEndingStyles: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyIconName: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyOpen: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyParent: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyPopup: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyQuadding: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyQuadPoints: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyTextLabel: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetDownCaption: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetBorderColor: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetBackgroundColor: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetCaption: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetDefaultValue: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetFieldFlags: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetFieldType: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetAppearanceDictionary: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetMaxLen: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetOptions: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetRotation: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetRolloverCaption: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetTextLabelUI: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationKeyWidgetValue: PDFAnnotationKey;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeText: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeLink: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeFreeText: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeLine: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeSquare: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeCircle: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeHighlight: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeUnderline: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeStrikeOut: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeInk: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeStamp: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypePopup: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationSubtypeWidget: PDFAnnotationSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationWidgetSubtypeButton: PDFAnnotationWidgetSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationWidgetSubtypeChoice: PDFAnnotationWidgetSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationWidgetSubtypeSignature: PDFAnnotationWidgetSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationWidgetSubtypeText: PDFAnnotationWidgetSubtype;
}
unsafe extern "C" {
    pub static mut PDFAnnotationLineEndingStyleNone: PDFAnnotationLineEndingStyle;
}
unsafe extern "C" {
    pub static mut PDFAnnotationLineEndingStyleSquare: PDFAnnotationLineEndingStyle;
}
unsafe extern "C" {
    pub static mut PDFAnnotationLineEndingStyleCircle: PDFAnnotationLineEndingStyle;
}
unsafe extern "C" {
    pub static mut PDFAnnotationLineEndingStyleDiamond: PDFAnnotationLineEndingStyle;
}
unsafe extern "C" {
    pub static mut PDFAnnotationLineEndingStyleOpenArrow: PDFAnnotationLineEndingStyle;
}
unsafe extern "C" {
    pub static mut PDFAnnotationLineEndingStyleClosedArrow: PDFAnnotationLineEndingStyle;
}
unsafe extern "C" {
    pub static mut PDFAnnotationTextIconTypeComment: PDFAnnotationTextIconType;
}
unsafe extern "C" {
    pub static mut PDFAnnotationTextIconTypeKey: PDFAnnotationTextIconType;
}
unsafe extern "C" {
    pub static mut PDFAnnotationTextIconTypeNote: PDFAnnotationTextIconType;
}
unsafe extern "C" {
    pub static mut PDFAnnotationTextIconTypeHelp: PDFAnnotationTextIconType;
}
unsafe extern "C" {
    pub static mut PDFAnnotationTextIconTypeNewParagraph: PDFAnnotationTextIconType;
}
unsafe extern "C" {
    pub static mut PDFAnnotationTextIconTypeParagraph: PDFAnnotationTextIconType;
}
unsafe extern "C" {
    pub static mut PDFAnnotationTextIconTypeInsert: PDFAnnotationTextIconType;
}
unsafe extern "C" {
    pub static mut PDFAnnotationHighlightingModeNone: PDFAnnotationHighlightingMode;
}
unsafe extern "C" {
    pub static mut PDFAnnotationHighlightingModeInvert: PDFAnnotationHighlightingMode;
}
unsafe extern "C" {
    pub static mut PDFAnnotationHighlightingModeOutline: PDFAnnotationHighlightingMode;
}
unsafe extern "C" {
    pub static mut PDFAnnotationHighlightingModePush: PDFAnnotationHighlightingMode;
}
unsafe extern "C" {
    pub static mut PDFAppearanceCharacteristicsKeyBackgroundColor: PDFAppearanceCharacteristicsKey;
}
unsafe extern "C" {
    pub static mut PDFAppearanceCharacteristicsKeyBorderColor: PDFAppearanceCharacteristicsKey;
}
unsafe extern "C" {
    pub static mut PDFAppearanceCharacteristicsKeyRotation: PDFAppearanceCharacteristicsKey;
}
unsafe extern "C" {
    pub static mut PDFAppearanceCharacteristicsKeyCaption: PDFAppearanceCharacteristicsKey;
}
unsafe extern "C" {
    pub static mut PDFAppearanceCharacteristicsKeyRolloverCaption: PDFAppearanceCharacteristicsKey;
}
unsafe extern "C" {
    pub static mut PDFAppearanceCharacteristicsKeyDownCaption: PDFAppearanceCharacteristicsKey;
}
unsafe extern "C" {
    pub static mut PDFBorderKeyLineWidth: PDFBorderKey;
}
unsafe extern "C" {
    pub static mut PDFBorderKeyStyle: PDFBorderKey;
}
unsafe extern "C" {
    pub static mut PDFBorderKeyDashPattern: PDFBorderKey;
}
unsafe extern "C" {
    pub static kPDFDestinationUnspecifiedValue: CGFloat;
}
unsafe extern "C" {
    pub static PDFDocumentDidUnlockNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidBeginFindNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidEndFindNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidBeginPageFindNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidEndPageFindNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidFindMatchNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidBeginWriteNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidEndWriteNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidBeginPageWriteNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentDidEndPageWriteNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFDocumentFoundSelectionKey: NSString;
}
unsafe extern "C" {
    pub static PDFDocumentPageIndexKey: NSString;
}
unsafe extern "C" {
    pub static PDFDocumentTitleAttribute: PDFDocumentAttribute;
}
unsafe extern "C" {
    pub static PDFDocumentAuthorAttribute: PDFDocumentAttribute;
}
unsafe extern "C" {
    pub static PDFDocumentSubjectAttribute: PDFDocumentAttribute;
}
unsafe extern "C" {
    pub static PDFDocumentCreatorAttribute: PDFDocumentAttribute;
}
unsafe extern "C" {
    pub static PDFDocumentProducerAttribute: PDFDocumentAttribute;
}
unsafe extern "C" {
    pub static PDFDocumentCreationDateAttribute: PDFDocumentAttribute;
}
unsafe extern "C" {
    pub static PDFDocumentModificationDateAttribute: PDFDocumentAttribute;
}
unsafe extern "C" {
    pub static PDFDocumentKeywordsAttribute: PDFDocumentAttribute;
}
unsafe extern "C" {
    pub static PDFDocumentOwnerPasswordOption: PDFDocumentWriteOption;
}
unsafe extern "C" {
    pub static PDFDocumentUserPasswordOption: PDFDocumentWriteOption;
}
unsafe extern "C" {
    pub static PDFDocumentAccessPermissionsOption: PDFDocumentWriteOption;
}
unsafe extern "C" {
    pub static PDFDocumentBurnInAnnotationsOption: PDFDocumentWriteOption;
}
unsafe extern "C" {
    pub static PDFDocumentSaveTextFromOCROption: PDFDocumentWriteOption;
}
unsafe extern "C" {
    pub static PDFDocumentSaveImagesAsJPEGOption: PDFDocumentWriteOption;
}
unsafe extern "C" {
    pub static PDFDocumentOptimizeImagesForScreenOption: PDFDocumentWriteOption;
}
unsafe extern "C" {
    pub static PDFThumbnailViewDocumentEditedNotification: NSString;
}
unsafe extern "C" {
    pub static PDFViewDocumentChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewChangedHistoryNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewPageChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewScaleChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewAnnotationHitNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewCopyPermissionNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewPrintPermissionNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewAnnotationWillHitNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewSelectionChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewDisplayModeChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewDisplayBoxChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static PDFViewVisiblePagesChangedNotification: NSNotificationName;
}

unsafe impl objc2::encode::RefEncode for PDFAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFActionGoTo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFActionGoTo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFActionNamed {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFActionNamed {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFActionResetForm {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFActionResetForm {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFActionRemoteGoTo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFActionRemoteGoTo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFActionURL {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFActionURL {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFPage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFPage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAppearanceCharacteristics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAppearanceCharacteristics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFBorder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFBorder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFDestination {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFDestination {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFDocument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFDocument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFOutline {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFOutline {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFSelection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFSelection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFThumbnailView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFThumbnailView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationButtonWidget {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationButtonWidget {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationChoiceWidget {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationChoiceWidget {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationCircle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationCircle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationFreeText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationFreeText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationInk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationInk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationLine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationLine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationLink {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationLink {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationMarkup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationMarkup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationPopup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationPopup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationSquare {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationSquare {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationStamp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationStamp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PDFAnnotationTextWidget {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PDFAnnotationTextWidget {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
