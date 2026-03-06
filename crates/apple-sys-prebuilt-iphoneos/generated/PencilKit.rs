#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKTool(pub id);
impl std::ops::Deref for PKTool {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKTool {}
impl PKTool {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKTool").unwrap(), alloc) })
    }
}
impl PNSCopying for PKTool {}
impl INSObject for PKTool {}
impl PNSObject for PKTool {}
impl std::convert::TryFrom<NSObject> for PKTool {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKTool, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKTool").unwrap()) };
        if is_kind_of {
            Ok(PKTool(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKTool")
        }
    }
}
impl IPKTool for PKTool {}
pub trait IPKTool: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKLassoTool(pub id);
impl std::ops::Deref for PKLassoTool {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKLassoTool {}
impl PKLassoTool {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKLassoTool").unwrap(), alloc) })
    }
}
impl IPKTool for PKLassoTool {}
impl PNSCopying for PKLassoTool {}
impl From<PKLassoTool> for PKTool {
    fn from(child: PKLassoTool) -> PKTool {
        PKTool(child.0)
    }
}
impl std::convert::TryFrom<PKTool> for PKLassoTool {
    type Error = &'static str;
    fn try_from(parent: PKTool) -> Result<PKLassoTool, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKLassoTool").unwrap()) };
        if is_kind_of {
            Ok(PKLassoTool(parent.0))
        } else {
            Err("This PKTool cannot be downcasted to PKLassoTool")
        }
    }
}
impl INSObject for PKLassoTool {}
impl PNSObject for PKLassoTool {}
impl IPKLassoTool for PKLassoTool {}
pub trait IPKLassoTool: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub type PKEraserType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKEraserTool(pub id);
impl std::ops::Deref for PKEraserTool {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKEraserTool {}
impl PKEraserTool {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKEraserTool").unwrap(), alloc) })
    }
}
impl IPKTool for PKEraserTool {}
impl PNSCopying for PKEraserTool {}
impl std::convert::TryFrom<PKTool> for PKEraserTool {
    type Error = &'static str;
    fn try_from(parent: PKTool) -> Result<PKEraserTool, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKEraserTool").unwrap()) };
        if is_kind_of {
            Ok(PKEraserTool(parent.0))
        } else {
            Err("This PKTool cannot be downcasted to PKEraserTool")
        }
    }
}
impl INSObject for PKEraserTool {}
impl PNSObject for PKEraserTool {}
impl IPKEraserTool for PKEraserTool {}
pub trait IPKEraserTool: Sized + std::ops::Deref {
    unsafe fn initWithEraserType_(&self, eraserType: PKEraserType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEraserType : eraserType)
    }
    unsafe fn initWithEraserType_width_(
        &self,
        eraserType: PKEraserType,
        width: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEraserType : eraserType, width : width)
    }
    unsafe fn eraserType(&self) -> PKEraserType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eraserType)
    }
    unsafe fn width(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn defaultWidthForEraserType_(eraserType: PKEraserType) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKEraserTool").unwrap(), defaultWidthForEraserType : eraserType)
    }
    unsafe fn minimumWidthForEraserType_(eraserType: PKEraserType) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKEraserTool").unwrap(), minimumWidthForEraserType : eraserType)
    }
    unsafe fn maximumWidthForEraserType_(eraserType: PKEraserType) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKEraserTool").unwrap(), maximumWidthForEraserType : eraserType)
    }
}
pub type PKInkType = NSString;
pub type PKContentVersion = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKInkingTool(pub id);
impl std::ops::Deref for PKInkingTool {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKInkingTool {}
impl PKInkingTool {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKInkingTool").unwrap(), alloc) })
    }
}
impl IPKTool for PKInkingTool {}
impl PNSCopying for PKInkingTool {}
impl std::convert::TryFrom<PKTool> for PKInkingTool {
    type Error = &'static str;
    fn try_from(parent: PKTool) -> Result<PKInkingTool, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKInkingTool").unwrap()) };
        if is_kind_of {
            Ok(PKInkingTool(parent.0))
        } else {
            Err("This PKTool cannot be downcasted to PKInkingTool")
        }
    }
}
impl INSObject for PKInkingTool {}
impl PNSObject for PKInkingTool {}
impl IPKInkingTool for PKInkingTool {}
pub trait IPKInkingTool: Sized + std::ops::Deref {
    unsafe fn initWithInkType_color_width_(
        &self,
        type_: NSString,
        color: NSColor,
        width: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : type_, color : color, width : width)
    }
    unsafe fn initWithInkType_color_width_azimuth_(
        &self,
        type_: NSString,
        color: NSColor,
        width: CGFloat,
        angle: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : type_, color : color, width : width, azimuth : angle)
    }
    unsafe fn initWithInkType_color_(&self, type_: NSString, color: NSColor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : type_, color : color)
    }
    unsafe fn initWithInk_width_(&self, ink: PKInk, width: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInk : ink, width : width)
    }
    unsafe fn inkType(&self) -> PKInkType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inkType)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn width(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn azimuth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, azimuth)
    }
    unsafe fn ink(&self) -> PKInk
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ink)
    }
    unsafe fn requiredContentVersion(&self) -> PKContentVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredContentVersion)
    }
    unsafe fn defaultWidthForInkType_(inkType: NSString) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKInkingTool").unwrap(), defaultWidthForInkType : inkType)
    }
    unsafe fn minimumWidthForInkType_(inkType: NSString) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKInkingTool").unwrap(), minimumWidthForInkType : inkType)
    }
    unsafe fn maximumWidthForInkType_(inkType: NSString) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKInkingTool").unwrap(), maximumWidthForInkType : inkType)
    }
    unsafe fn invertColor_(color: CGColorRef) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKInkingTool").unwrap(), invertColor : color)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKToolPickerItem(pub id);
impl std::ops::Deref for PKToolPickerItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKToolPickerItem {}
impl PKToolPickerItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKToolPickerItem").unwrap(), alloc) })
    }
}
impl PNSCopying for PKToolPickerItem {}
impl INSObject for PKToolPickerItem {}
impl PNSObject for PKToolPickerItem {}
impl std::convert::TryFrom<NSObject> for PKToolPickerItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKToolPickerItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKToolPickerItem").unwrap()) };
        if is_kind_of {
            Ok(PKToolPickerItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKToolPickerItem")
        }
    }
}
impl IPKToolPickerItem for PKToolPickerItem {}
pub trait IPKToolPickerItem: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn tool(&self) -> PKTool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tool)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKToolPickerEraserItem(pub id);
impl std::ops::Deref for PKToolPickerEraserItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKToolPickerEraserItem {}
impl PKToolPickerEraserItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKToolPickerEraserItem").unwrap(), alloc) })
    }
}
impl IPKToolPickerItem for PKToolPickerEraserItem {}
impl PNSCopying for PKToolPickerEraserItem {}
impl From<PKToolPickerEraserItem> for PKToolPickerItem {
    fn from(child: PKToolPickerEraserItem) -> PKToolPickerItem {
        PKToolPickerItem(child.0)
    }
}
impl std::convert::TryFrom<PKToolPickerItem> for PKToolPickerEraserItem {
    type Error = &'static str;
    fn try_from(parent: PKToolPickerItem) -> Result<PKToolPickerEraserItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKToolPickerEraserItem").unwrap()) };
        if is_kind_of {
            Ok(PKToolPickerEraserItem(parent.0))
        } else {
            Err("This PKToolPickerItem cannot be downcasted to PKToolPickerEraserItem")
        }
    }
}
impl INSObject for PKToolPickerEraserItem {}
impl PNSObject for PKToolPickerEraserItem {}
impl IPKToolPickerEraserItem for PKToolPickerEraserItem {}
pub trait IPKToolPickerEraserItem: Sized + std::ops::Deref {
    unsafe fn initWithEraserType_(&self, eraserType: PKEraserType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEraserType : eraserType)
    }
    unsafe fn initWithEraserType_width_(
        &self,
        eraserType: PKEraserType,
        width: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEraserType : eraserType, width : width)
    }
    unsafe fn eraserTool(&self) -> PKEraserTool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eraserTool)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKToolPickerInkingItem(pub id);
impl std::ops::Deref for PKToolPickerInkingItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKToolPickerInkingItem {}
impl PKToolPickerInkingItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKToolPickerInkingItem").unwrap(), alloc) })
    }
}
impl IPKToolPickerItem for PKToolPickerInkingItem {}
impl PNSCopying for PKToolPickerInkingItem {}
impl std::convert::TryFrom<PKToolPickerItem> for PKToolPickerInkingItem {
    type Error = &'static str;
    fn try_from(parent: PKToolPickerItem) -> Result<PKToolPickerInkingItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKToolPickerInkingItem").unwrap()) };
        if is_kind_of {
            Ok(PKToolPickerInkingItem(parent.0))
        } else {
            Err("This PKToolPickerItem cannot be downcasted to PKToolPickerInkingItem")
        }
    }
}
impl INSObject for PKToolPickerInkingItem {}
impl PNSObject for PKToolPickerInkingItem {}
impl IPKToolPickerInkingItem for PKToolPickerInkingItem {}
pub trait IPKToolPickerInkingItem: Sized + std::ops::Deref {
    unsafe fn initWithInkType_(&self, inkType: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : inkType)
    }
    unsafe fn initWithInkType_color_(&self, inkType: NSString, color: NSColor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : inkType, color : color)
    }
    unsafe fn initWithInkType_width_(&self, inkType: NSString, width: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : inkType, width : width)
    }
    unsafe fn initWithInkType_color_width_(
        &self,
        inkType: NSString,
        color: NSColor,
        width: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : inkType, color : color, width : width)
    }
    unsafe fn initWithInkType_color_width_identifier_(
        &self,
        inkType: NSString,
        color: NSColor,
        width: CGFloat,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : inkType, color : color, width : width, identifier : identifier)
    }
    unsafe fn initWithInkType_color_width_azimuth_identifier_(
        &self,
        inkType: NSString,
        color: NSColor,
        width: CGFloat,
        azimuth: CGFloat,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : inkType, color : color, width : width, azimuth : azimuth, identifier : identifier)
    }
    unsafe fn inkingTool(&self) -> PKInkingTool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inkingTool)
    }
    unsafe fn allowsColorSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsColorSelection)
    }
    unsafe fn setAllowsColorSelection_(&self, allowsColorSelection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsColorSelection : allowsColorSelection)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKDrawing(pub id);
impl std::ops::Deref for PKDrawing {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKDrawing {}
impl PKDrawing {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKDrawing").unwrap(), alloc) })
    }
}
impl PNSCopying for PKDrawing {}
impl PNSSecureCoding for PKDrawing {}
impl INSObject for PKDrawing {}
impl PNSObject for PKDrawing {}
impl std::convert::TryFrom<NSObject> for PKDrawing {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKDrawing, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKDrawing").unwrap()) };
        if is_kind_of {
            Ok(PKDrawing(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKDrawing")
        }
    }
}
impl IPKDrawing for PKDrawing {}
pub trait IPKDrawing: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithStrokes_(&self, strokes: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStrokes : strokes)
    }
    unsafe fn initWithData_error_(&self, data: NSData, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, error : error)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
    unsafe fn imageFromRect_scale_(&self, rect: CGRect, scale: CGFloat) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageFromRect : rect, scale : scale)
    }
    unsafe fn drawingByApplyingTransform_(&self, transform: CGAffineTransform) -> PKDrawing
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawingByApplyingTransform : transform)
    }
    unsafe fn drawingByAppendingDrawing_(&self, drawing: PKDrawing) -> PKDrawing
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawingByAppendingDrawing : drawing)
    }
    unsafe fn drawingByAppendingStrokes_(&self, strokes: NSArray) -> PKDrawing
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawingByAppendingStrokes : strokes)
    }
    unsafe fn strokes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strokes)
    }
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn requiredContentVersion(&self) -> PKContentVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredContentVersion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKStroke(pub id);
impl std::ops::Deref for PKStroke {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKStroke {}
impl PKStroke {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKStroke").unwrap(), alloc) })
    }
}
impl PNSCopying for PKStroke {}
impl INSObject for PKStroke {}
impl PNSObject for PKStroke {}
impl std::convert::TryFrom<NSObject> for PKStroke {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKStroke, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKStroke").unwrap()) };
        if is_kind_of {
            Ok(PKStroke(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKStroke")
        }
    }
}
impl IPKStroke for PKStroke {}
pub trait IPKStroke: Sized + std::ops::Deref {
    unsafe fn initWithInk_strokePath_transform_mask_(
        &self,
        ink: PKInk,
        strokePath: PKStrokePath,
        transform: CGAffineTransform,
        mask: NSBezierPath,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInk : ink, strokePath : strokePath, transform : transform, mask : mask)
    }
    unsafe fn initWithInk_strokePath_transform_mask_randomSeed_(
        &self,
        ink: PKInk,
        strokePath: PKStrokePath,
        transform: CGAffineTransform,
        mask: NSBezierPath,
        randomSeed: u32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInk : ink, strokePath : strokePath, transform : transform, mask : mask, randomSeed : randomSeed)
    }
    unsafe fn ink(&self) -> PKInk
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ink)
    }
    unsafe fn transform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn path(&self) -> PKStrokePath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn mask(&self) -> NSBezierPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mask)
    }
    unsafe fn renderBounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderBounds)
    }
    unsafe fn maskedPathRanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskedPathRanges)
    }
    unsafe fn randomSeed(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, randomSeed)
    }
    unsafe fn requiredContentVersion(&self) -> PKContentVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredContentVersion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKFloatRange(pub id);
impl std::ops::Deref for PKFloatRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKFloatRange {}
impl PKFloatRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKFloatRange").unwrap(), alloc) })
    }
}
impl PNSCopying for PKFloatRange {}
impl INSObject for PKFloatRange {}
impl PNSObject for PKFloatRange {}
impl std::convert::TryFrom<NSObject> for PKFloatRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKFloatRange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKFloatRange").unwrap()) };
        if is_kind_of {
            Ok(PKFloatRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKFloatRange")
        }
    }
}
impl IPKFloatRange for PKFloatRange {}
pub trait IPKFloatRange: Sized + std::ops::Deref {
    unsafe fn initWithLowerBound_upperBound_(
        &self,
        lowerBound: CGFloat,
        upperBound: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLowerBound : lowerBound, upperBound : upperBound)
    }
    unsafe fn lowerBound(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowerBound)
    }
    unsafe fn upperBound(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperBound)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKStrokePath(pub id);
impl std::ops::Deref for PKStrokePath {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKStrokePath {}
impl PKStrokePath {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKStrokePath").unwrap(), alloc) })
    }
}
impl PNSCopying for PKStrokePath {}
impl INSObject for PKStrokePath {}
impl PNSObject for PKStrokePath {}
impl std::convert::TryFrom<NSObject> for PKStrokePath {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKStrokePath, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKStrokePath").unwrap()) };
        if is_kind_of {
            Ok(PKStrokePath(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKStrokePath")
        }
    }
}
impl IPKStrokePath for PKStrokePath {}
pub trait IPKStrokePath: Sized + std::ops::Deref {
    unsafe fn initWithControlPoints_creationDate_(
        &self,
        controlPoints: NSArray,
        creationDate: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithControlPoints : controlPoints, creationDate : creationDate)
    }
    unsafe fn pointAtIndex_(&self, i: NSUInteger) -> PKStrokePoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pointAtIndex : i)
    }
    unsafe fn objectAtIndexedSubscript_(&self, i: NSUInteger) -> PKStrokePoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : i)
    }
    unsafe fn interpolatedLocationAt_(&self, parametricValue: CGFloat) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, interpolatedLocationAt : parametricValue)
    }
    unsafe fn interpolatedPointAt_(&self, parametricValue: CGFloat) -> PKStrokePoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, interpolatedPointAt : parametricValue)
    }
    unsafe fn enumerateInterpolatedPointsInRange_strideByDistance_usingBlock_(
        &self,
        range: PKFloatRange,
        distanceStep: CGFloat,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateInterpolatedPointsInRange : range, strideByDistance : distanceStep, usingBlock : block)
    }
    unsafe fn enumerateInterpolatedPointsInRange_strideByTime_usingBlock_(
        &self,
        range: PKFloatRange,
        timeStep: NSTimeInterval,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateInterpolatedPointsInRange : range, strideByTime : timeStep, usingBlock : block)
    }
    unsafe fn enumerateInterpolatedPointsInRange_strideByParametricStep_usingBlock_(
        &self,
        range: PKFloatRange,
        parametricStep: CGFloat,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateInterpolatedPointsInRange : range, strideByParametricStep : parametricStep, usingBlock : block)
    }
    unsafe fn parametricValue_offsetByDistance_(
        &self,
        parametricValue: CGFloat,
        distanceStep: CGFloat,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parametricValue : parametricValue, offsetByDistance : distanceStep)
    }
    unsafe fn parametricValue_offsetByTime_(
        &self,
        parametricValue: CGFloat,
        timeStep: NSTimeInterval,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parametricValue : parametricValue, offsetByTime : timeStep)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKStrokePoint(pub id);
impl std::ops::Deref for PKStrokePoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKStrokePoint {}
impl PKStrokePoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKStrokePoint").unwrap(), alloc) })
    }
}
impl PNSCopying for PKStrokePoint {}
impl INSObject for PKStrokePoint {}
impl PNSObject for PKStrokePoint {}
impl std::convert::TryFrom<NSObject> for PKStrokePoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKStrokePoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKStrokePoint").unwrap()) };
        if is_kind_of {
            Ok(PKStrokePoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKStrokePoint")
        }
    }
}
impl IPKStrokePoint for PKStrokePoint {}
pub trait IPKStrokePoint: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLocation_timeOffset_size_opacity_force_azimuth_altitude_(
        &self,
        location: CGPoint,
        timeOffset: NSTimeInterval,
        size: CGSize,
        opacity: CGFloat,
        force: CGFloat,
        azimuth: CGFloat,
        altitude: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location, timeOffset : timeOffset, size : size, opacity : opacity, force : force, azimuth : azimuth, altitude : altitude)
    }
    unsafe fn initWithLocation_timeOffset_size_opacity_force_azimuth_altitude_secondaryScale_(
        &self,
        location: CGPoint,
        timeOffset: NSTimeInterval,
        size: CGSize,
        opacity: CGFloat,
        force: CGFloat,
        azimuth: CGFloat,
        altitude: CGFloat,
        secondaryScale: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location, timeOffset : timeOffset, size : size, opacity : opacity, force : force, azimuth : azimuth, altitude : altitude, secondaryScale : secondaryScale)
    }
    unsafe fn initWithLocation_timeOffset_size_opacity_force_azimuth_altitude_secondaryScale_threshold_(
        &self,
        location: CGPoint,
        timeOffset: NSTimeInterval,
        size: CGSize,
        opacity: CGFloat,
        force: CGFloat,
        azimuth: CGFloat,
        altitude: CGFloat,
        secondaryScale: CGFloat,
        threshold: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location, timeOffset : timeOffset, size : size, opacity : opacity, force : force, azimuth : azimuth, altitude : altitude, secondaryScale : secondaryScale, threshold : threshold)
    }
    unsafe fn location(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn timeOffset(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeOffset)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn opacity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opacity)
    }
    unsafe fn azimuth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, azimuth)
    }
    unsafe fn force(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, force)
    }
    unsafe fn altitude(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altitude)
    }
    unsafe fn secondaryScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secondaryScale)
    }
    unsafe fn threshold(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threshold)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKInk(pub id);
impl std::ops::Deref for PKInk {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKInk {}
impl PKInk {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKInk").unwrap(), alloc) })
    }
}
impl PNSCopying for PKInk {}
impl INSObject for PKInk {}
impl PNSObject for PKInk {}
impl std::convert::TryFrom<NSObject> for PKInk {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKInk, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKInk").unwrap()) };
        if is_kind_of {
            Ok(PKInk(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKInk")
        }
    }
}
impl IPKInk for PKInk {}
pub trait IPKInk: Sized + std::ops::Deref {
    unsafe fn initWithInkType_color_(&self, type_: NSString, color: NSColor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInkType : type_, color : color)
    }
    unsafe fn inkType(&self) -> PKInkType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inkType)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn requiredContentVersion(&self) -> PKContentVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredContentVersion)
    }
}
unsafe extern "C" {
    pub static PKInkTypePen: PKInkType;
}
unsafe extern "C" {
    pub static PKInkTypePencil: PKInkType;
}
unsafe extern "C" {
    pub static PKInkTypeMarker: PKInkType;
}
unsafe extern "C" {
    pub static PKInkTypeMonoline: PKInkType;
}
unsafe extern "C" {
    pub static PKInkTypeFountainPen: PKInkType;
}
unsafe extern "C" {
    pub static PKInkTypeWatercolor: PKInkType;
}
unsafe extern "C" {
    pub static PKInkTypeReed: PKInkType;
}
unsafe extern "C" {
    pub static PKInkTypeCrayon: PKInkType;
}
unsafe extern "C" {
    pub static PKAppleDrawingTypeIdentifier: CFStringRef;
}

unsafe impl objc2::encode::RefEncode for PKTool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKTool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKLassoTool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKLassoTool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKEraserTool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKEraserTool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKInkingTool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKInkingTool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKToolPickerItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKToolPickerItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKToolPickerEraserItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKToolPickerEraserItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKToolPickerInkingItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKToolPickerInkingItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKDrawing {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKDrawing {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKStroke {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKStroke {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKFloatRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKFloatRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKStrokePath {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKStrokePath {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKStrokePoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKStrokePoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKInk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKInk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
