#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::GameController::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::MetalKit::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type GCUIEventTypes = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCEventInteraction(pub id);
impl std::ops::Deref for GCEventInteraction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCEventInteraction {}
impl GCEventInteraction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCEventInteraction").unwrap(), alloc) })
    }
}
impl INSObject for GCEventInteraction {}
impl PNSObject for GCEventInteraction {}
impl std::convert::TryFrom<NSObject> for GCEventInteraction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCEventInteraction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCEventInteraction").unwrap()) };
        if is_kind_of {
            Ok(GCEventInteraction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCEventInteraction")
        }
    }
}
impl IGCEventInteraction for GCEventInteraction {}
pub trait IGCEventInteraction: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn handledEventTypes(&self) -> GCUIEventTypes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handledEventTypes)
    }
    unsafe fn setHandledEventTypes_(&self, handledEventTypes: GCUIEventTypes)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandledEventTypes : handledEventTypes)
    }
    unsafe fn receivesEventsInView(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, receivesEventsInView)
    }
    unsafe fn setReceivesEventsInView_(&self, receivesEventsInView: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReceivesEventsInView : receivesEventsInView)
    }
}
impl GCEventInteraction_UIInteraction for GCEventInteraction {}
pub trait GCEventInteraction_UIInteraction: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCGameControllerActivationContext(pub id);
impl std::ops::Deref for GCGameControllerActivationContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCGameControllerActivationContext {}
impl GCGameControllerActivationContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCGameControllerActivationContext").unwrap(), alloc) })
    }
}
impl INSObject for GCGameControllerActivationContext {}
impl PNSObject for GCGameControllerActivationContext {}
impl std::convert::TryFrom<NSObject> for GCGameControllerActivationContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCGameControllerActivationContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCGameControllerActivationContext").unwrap())
        };
        if is_kind_of {
            Ok(GCGameControllerActivationContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCGameControllerActivationContext")
        }
    }
}
impl IGCGameControllerActivationContext for GCGameControllerActivationContext {}
pub trait IGCGameControllerActivationContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn previousApplicationBundleID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousApplicationBundleID)
    }
}
pub trait UISceneConnectionOptions_GameController: Sized + std::ops::Deref {
    unsafe fn gameControllerActivationContext(&self) -> GCGameControllerActivationContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gameControllerActivationContext)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCVirtualControllerConfiguration(pub id);
impl std::ops::Deref for GCVirtualControllerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCVirtualControllerConfiguration {}
impl GCVirtualControllerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCVirtualControllerConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for GCVirtualControllerConfiguration {}
impl PNSObject for GCVirtualControllerConfiguration {}
impl std::convert::TryFrom<NSObject> for GCVirtualControllerConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCVirtualControllerConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCVirtualControllerConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(GCVirtualControllerConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCVirtualControllerConfiguration")
        }
    }
}
impl IGCVirtualControllerConfiguration for GCVirtualControllerConfiguration {}
pub trait IGCVirtualControllerConfiguration: Sized + std::ops::Deref {
    unsafe fn elements(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn setElements_(&self, elements: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElements : elements)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn setHidden_(&self, hidden: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidden : hidden)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCVirtualControllerElementConfiguration(pub id);
impl std::ops::Deref for GCVirtualControllerElementConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCVirtualControllerElementConfiguration {}
impl GCVirtualControllerElementConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCVirtualControllerElementConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for GCVirtualControllerElementConfiguration {}
impl PNSObject for GCVirtualControllerElementConfiguration {}
impl std::convert::TryFrom<NSObject> for GCVirtualControllerElementConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCVirtualControllerElementConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCVirtualControllerElementConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(GCVirtualControllerElementConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCVirtualControllerElementConfiguration")
        }
    }
}
impl IGCVirtualControllerElementConfiguration for GCVirtualControllerElementConfiguration {}
pub trait IGCVirtualControllerElementConfiguration: Sized + std::ops::Deref {
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn setHidden_(&self, hidden: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidden : hidden)
    }
    unsafe fn path(&self) -> UIBezierPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn setPath_(&self, path: UIBezierPath)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPath : path)
    }
    unsafe fn actsAsTouchpad(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actsAsTouchpad)
    }
    unsafe fn setActsAsTouchpad_(&self, actsAsTouchpad: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActsAsTouchpad : actsAsTouchpad)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCVirtualController(pub id);
impl std::ops::Deref for GCVirtualController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCVirtualController {}
impl GCVirtualController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCVirtualController").unwrap(), alloc) })
    }
}
impl INSObject for GCVirtualController {}
impl PNSObject for GCVirtualController {}
impl std::convert::TryFrom<NSObject> for GCVirtualController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCVirtualController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCVirtualController").unwrap()) };
        if is_kind_of {
            Ok(GCVirtualController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCVirtualController")
        }
    }
}
impl IGCVirtualController for GCVirtualController {}
pub trait IGCVirtualController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithConfiguration_(
        &self,
        configuration: GCVirtualControllerConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
    unsafe fn connectWithReplyHandler_(&self, reply: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectWithReplyHandler : reply)
    }
    unsafe fn disconnect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnect)
    }
    unsafe fn updateConfigurationForElement_configuration_(
        &self,
        element: NSString,
        config: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateConfigurationForElement : element, configuration : config)
    }
    unsafe fn setValue_forButtonElement_(&self, value: CGFloat, element: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forButtonElement : element)
    }
    unsafe fn setPosition_forDirectionPadElement_(&self, position: CGPoint, element: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position, forDirectionPadElement : element)
    }
    unsafe fn controller(&self) -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controller)
    }
    unsafe fn virtualControllerWithConfiguration_(
        configuration: GCVirtualControllerConfiguration,
    ) -> GCVirtualController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCVirtualController").unwrap(), virtualControllerWithConfiguration : configuration)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCVirtualController").unwrap(), new)
    }
}
pub type TCControlLayoutAnchor = NSInteger;
pub type TCControlLayoutAnchorCoordinateSystem = NSInteger;
pub trait PTCControlLayout: Sized + std::ops::Deref {
    unsafe fn anchor(&self) -> TCControlLayoutAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchor)
    }
    unsafe fn setAnchor_(&self, anchor: TCControlLayoutAnchor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchor : anchor)
    }
    unsafe fn anchorCoordinateSystem(&self) -> TCControlLayoutAnchorCoordinateSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorCoordinateSystem)
    }
    unsafe fn setAnchorCoordinateSystem_(
        &self,
        anchorCoordinateSystem: TCControlLayoutAnchorCoordinateSystem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorCoordinateSystem : anchorCoordinateSystem)
    }
    unsafe fn offset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn zIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zIndex)
    }
    unsafe fn setZIndex_(&self, zIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZIndex : zIndex)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn position(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
}
pub type TCColliderShape = NSInteger;
pub type TCControlLabelRole = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCControlLabel(pub id);
impl std::ops::Deref for TCControlLabel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCControlLabel {}
impl TCControlLabel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), alloc) })
    }
}
impl INSObject for TCControlLabel {}
impl PNSObject for TCControlLabel {}
impl std::convert::TryFrom<NSObject> for TCControlLabel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCControlLabel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap()) };
        if is_kind_of {
            Ok(TCControlLabel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCControlLabel")
        }
    }
}
impl ITCControlLabel for TCControlLabel {}
pub trait ITCControlLabel: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithName_role_(&self, name: NSString, role: TCControlLabelRole) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, role : role)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn role(&self) -> TCControlLabelRole
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, role)
    }
    unsafe fn buttonA() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonA)
    }
    unsafe fn buttonB() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonB)
    }
    unsafe fn buttonX() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonX)
    }
    unsafe fn buttonY() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonY)
    }
    unsafe fn buttonMenu() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonMenu)
    }
    unsafe fn buttonOptions() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonOptions)
    }
    unsafe fn buttonLeftShoulder() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonLeftShoulder)
    }
    unsafe fn buttonLeftTrigger() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonLeftTrigger)
    }
    unsafe fn buttonRightShoulder() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonRightShoulder)
    }
    unsafe fn buttonRightTrigger() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), buttonRightTrigger)
    }
    unsafe fn leftThumbstick() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), leftThumbstick)
    }
    unsafe fn leftThumbstickButton() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), leftThumbstickButton)
    }
    unsafe fn rightThumbstick() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), rightThumbstick)
    }
    unsafe fn rightThumbstickButton() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), rightThumbstickButton)
    }
    unsafe fn directionPad() -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlLabel").unwrap(), directionPad)
    }
}
pub trait PTCControl: Sized + std::ops::Deref {
    unsafe fn handleTouchBeganAtPoint_(&self, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTouchBeganAtPoint : point)
    }
    unsafe fn handleTouchMovedAtPoint_(&self, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTouchMovedAtPoint : point)
    }
    unsafe fn handleTouchEndedAtPoint_(&self, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTouchEndedAtPoint : point)
    }
    unsafe fn label(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn isPressed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPressed)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCTouchController(pub id);
impl std::ops::Deref for TCTouchController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCTouchController {}
impl TCTouchController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCTouchController").unwrap(), alloc) })
    }
}
impl INSObject for TCTouchController {}
impl PNSObject for TCTouchController {}
impl std::convert::TryFrom<NSObject> for TCTouchController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCTouchController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCTouchController").unwrap()) };
        if is_kind_of {
            Ok(TCTouchController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCTouchController")
        }
    }
}
impl ITCTouchController for TCTouchController {}
pub trait ITCTouchController: Sized + std::ops::Deref {
    unsafe fn initWithDescriptor_(&self, descriptor: TCTouchControllerDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDescriptor : descriptor)
    }
    unsafe fn automaticallyLayoutControlsForLabels_(&self, labels: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, automaticallyLayoutControlsForLabels : labels)
    }
    unsafe fn addButtonWithDescriptor_(&self, descriptor: TCButtonDescriptor) -> TCButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addButtonWithDescriptor : descriptor)
    }
    unsafe fn addSwitchWithDescriptor_(&self, descriptor: TCSwitchDescriptor) -> TCSwitch
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSwitchWithDescriptor : descriptor)
    }
    unsafe fn addThumbstickWithDescriptor_(
        &self,
        descriptor: TCThumbstickDescriptor,
    ) -> TCThumbstick
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addThumbstickWithDescriptor : descriptor)
    }
    unsafe fn addDirectionPadWithDescriptor_(
        &self,
        descriptor: TCDirectionPadDescriptor,
    ) -> TCDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addDirectionPadWithDescriptor : descriptor)
    }
    unsafe fn addThrottleWithDescriptor_(&self, descriptor: TCThrottleDescriptor) -> TCThrottle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addThrottleWithDescriptor : descriptor)
    }
    unsafe fn addTouchpadWithDescriptor_(&self, descriptor: TCTouchpadDescriptor) -> TCTouchpad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTouchpadWithDescriptor : descriptor)
    }
    unsafe fn removeAllControls(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllControls)
    }
    unsafe fn removeControl_(&self, control: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeControl : control)
    }
    unsafe fn controlAtPoint_(&self, point: CGPoint) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controlAtPoint : point)
    }
    unsafe fn handleTouchBeganAtPoint_index_(&self, point: CGPoint, index: NSInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTouchBeganAtPoint : point, index : index)
    }
    unsafe fn handleTouchMovedAtPoint_index_(&self, point: CGPoint, index: NSInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTouchMovedAtPoint : point, index : index)
    }
    unsafe fn handleTouchEndedAtPoint_index_(&self, point: CGPoint, index: NSInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTouchEndedAtPoint : point, index : index)
    }
    unsafe fn renderUsingRenderCommandEncoder_(&self, encoder: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderUsingRenderCommandEncoder : encoder)
    }
    unsafe fn connect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connect)
    }
    unsafe fn disconnect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnect)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn controls(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controls)
    }
    unsafe fn buttons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttons)
    }
    unsafe fn switches(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, switches)
    }
    unsafe fn thumbsticks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbsticks)
    }
    unsafe fn directionPads(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directionPads)
    }
    unsafe fn throttles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, throttles)
    }
    unsafe fn touchpads(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchpads)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn drawableSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawableSize)
    }
    unsafe fn setDrawableSize_(&self, drawableSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawableSize : drawableSize)
    }
    unsafe fn isConnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConnected)
    }
    unsafe fn controller(&self) -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controller)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCTouchController").unwrap(), isSupported)
    }
}
pub type TCControlContentsButtonShape = NSInteger;
pub type TCControlContentsDpadDirection = NSInteger;
pub type TCControlContentsDpadElementStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCControlContents(pub id);
impl std::ops::Deref for TCControlContents {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCControlContents {}
impl TCControlContents {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), alloc) })
    }
}
impl INSObject for TCControlContents {}
impl PNSObject for TCControlContents {}
impl std::convert::TryFrom<NSObject> for TCControlContents {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCControlContents, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCControlContents").unwrap()) };
        if is_kind_of {
            Ok(TCControlContents(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCControlContents")
        }
    }
}
impl ITCControlContents for TCControlContents {}
pub trait ITCControlContents: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn images(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, images)
    }
    unsafe fn contentsWithImages_(images: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), contentsWithImages : images)
    }
    unsafe fn buttonContentsForSystemImageNamed_size_shape_controller_(
        imageName: NSString,
        size: CGSize,
        shape: TCControlContentsButtonShape,
        controller: TCTouchController,
    ) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), buttonContentsForSystemImageNamed : imageName, size : size, shape : shape, controller : controller)
    }
    unsafe fn switchedOnContentsForSystemImageNamed_size_shape_controller_(
        imageName: NSString,
        size: CGSize,
        shape: TCControlContentsButtonShape,
        controller: TCTouchController,
    ) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), switchedOnContentsForSystemImageNamed : imageName, size : size, shape : shape, controller : controller)
    }
    unsafe fn thumbstickStickContentsOfSize_controller_(
        size: CGSize,
        controller: TCTouchController,
    ) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), thumbstickStickContentsOfSize : size, controller : controller)
    }
    unsafe fn thumbstickBackgroundContentsOfSize_controller_(
        size: CGSize,
        controller: TCTouchController,
    ) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), thumbstickBackgroundContentsOfSize : size, controller : controller)
    }
    unsafe fn throttleIndicatorContentsOfSize_controller_(
        size: CGSize,
        controller: TCTouchController,
    ) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), throttleIndicatorContentsOfSize : size, controller : controller)
    }
    unsafe fn throttleBackgroundContentsOfSize_controller_(
        size: CGSize,
        controller: TCTouchController,
    ) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), throttleBackgroundContentsOfSize : size, controller : controller)
    }
    unsafe fn directionPadContentsForLabel_size_style_direction_controller_(
        label: TCControlLabel,
        size: CGSize,
        style: TCControlContentsDpadElementStyle,
        direction: TCControlContentsDpadDirection,
        controller: TCTouchController,
    ) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlContents").unwrap(), directionPadContentsForLabel : label, size : size, style : style, direction : direction, controller : controller)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCControlImage(pub id);
impl std::ops::Deref for TCControlImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCControlImage {}
impl TCControlImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCControlImage").unwrap(), alloc) })
    }
}
impl INSObject for TCControlImage {}
impl PNSObject for TCControlImage {}
impl std::convert::TryFrom<NSObject> for TCControlImage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCControlImage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCControlImage").unwrap()) };
        if is_kind_of {
            Ok(TCControlImage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCControlImage")
        }
    }
}
impl ITCControlImage for TCControlImage {}
pub trait ITCControlImage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTexture_size_(&self, texture: *mut u64, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : texture, size : size)
    }
    unsafe fn initWithTexture_size_highlightTexture_offset_tintColor_(
        &self,
        texture: *mut u64,
        size: CGSize,
        highlightTexture: *mut u64,
        offset: CGPoint,
        tintColor: CGColorRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : texture, size : size, highlightTexture : highlightTexture, offset : offset, tintColor : tintColor)
    }
    unsafe fn initWithCGImage_size_device_(
        &self,
        cgImage: CGImageRef,
        size: CGSize,
        device: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGImage : cgImage, size : size, device : device)
    }
    unsafe fn initWithUIImage_size_device_(
        &self,
        uiImage: UIImage,
        size: CGSize,
        device: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUIImage : uiImage, size : size, device : device)
    }
    unsafe fn texture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texture)
    }
    unsafe fn setTexture_(&self, texture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTexture : texture)
    }
    unsafe fn highlightTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightTexture)
    }
    unsafe fn setHighlightTexture_(&self, highlightTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightTexture : highlightTexture)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn offset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn tintColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tintColor)
    }
    unsafe fn setTintColor_(&self, tintColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTintColor : tintColor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCButton(pub id);
impl std::ops::Deref for TCButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCButton {}
impl TCButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCButton").unwrap(), alloc) })
    }
}
impl PTCControl for TCButton {}
impl PTCControlLayout for TCButton {}
impl INSObject for TCButton {}
impl PNSObject for TCButton {}
impl std::convert::TryFrom<NSObject> for TCButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCButton, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCButton").unwrap()) };
        if is_kind_of {
            Ok(TCButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCButton")
        }
    }
}
impl ITCButton for TCButton {}
pub trait ITCButton: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn contents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCButtonDescriptor(pub id);
impl std::ops::Deref for TCButtonDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCButtonDescriptor {}
impl TCButtonDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCButtonDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for TCButtonDescriptor {}
impl PNSObject for TCButtonDescriptor {}
impl std::convert::TryFrom<NSObject> for TCButtonDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCButtonDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCButtonDescriptor").unwrap()) };
        if is_kind_of {
            Ok(TCButtonDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCButtonDescriptor")
        }
    }
}
impl ITCButtonDescriptor for TCButtonDescriptor {}
pub trait ITCButtonDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn label(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn contents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn anchor(&self) -> TCControlLayoutAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchor)
    }
    unsafe fn setAnchor_(&self, anchor: TCControlLayoutAnchor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchor : anchor)
    }
    unsafe fn anchorCoordinateSystem(&self) -> TCControlLayoutAnchorCoordinateSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorCoordinateSystem)
    }
    unsafe fn setAnchorCoordinateSystem_(
        &self,
        anchorCoordinateSystem: TCControlLayoutAnchorCoordinateSystem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorCoordinateSystem : anchorCoordinateSystem)
    }
    unsafe fn offset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn zIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zIndex)
    }
    unsafe fn setZIndex_(&self, zIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZIndex : zIndex)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn setColliderShape_(&self, colliderShape: TCColliderShape)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColliderShape : colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCSwitch(pub id);
impl std::ops::Deref for TCSwitch {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCSwitch {}
impl TCSwitch {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCSwitch").unwrap(), alloc) })
    }
}
impl PTCControl for TCSwitch {}
impl PTCControlLayout for TCSwitch {}
impl INSObject for TCSwitch {}
impl PNSObject for TCSwitch {}
impl std::convert::TryFrom<NSObject> for TCSwitch {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCSwitch, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCSwitch").unwrap()) };
        if is_kind_of {
            Ok(TCSwitch(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCSwitch")
        }
    }
}
impl ITCSwitch for TCSwitch {}
pub trait ITCSwitch: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn contents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn switchedOnContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, switchedOnContents)
    }
    unsafe fn setSwitchedOnContents_(&self, switchedOnContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSwitchedOnContents : switchedOnContents)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
    unsafe fn isSwitchedOn(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSwitchedOn)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCSwitchDescriptor(pub id);
impl std::ops::Deref for TCSwitchDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCSwitchDescriptor {}
impl TCSwitchDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCSwitchDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for TCSwitchDescriptor {}
impl PNSObject for TCSwitchDescriptor {}
impl std::convert::TryFrom<NSObject> for TCSwitchDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCSwitchDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCSwitchDescriptor").unwrap()) };
        if is_kind_of {
            Ok(TCSwitchDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCSwitchDescriptor")
        }
    }
}
impl ITCSwitchDescriptor for TCSwitchDescriptor {}
pub trait ITCSwitchDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn label(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn contents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn switchedOnContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, switchedOnContents)
    }
    unsafe fn setSwitchedOnContents_(&self, switchedOnContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSwitchedOnContents : switchedOnContents)
    }
    unsafe fn anchor(&self) -> TCControlLayoutAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchor)
    }
    unsafe fn setAnchor_(&self, anchor: TCControlLayoutAnchor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchor : anchor)
    }
    unsafe fn anchorCoordinateSystem(&self) -> TCControlLayoutAnchorCoordinateSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorCoordinateSystem)
    }
    unsafe fn setAnchorCoordinateSystem_(
        &self,
        anchorCoordinateSystem: TCControlLayoutAnchorCoordinateSystem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorCoordinateSystem : anchorCoordinateSystem)
    }
    unsafe fn offset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn zIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zIndex)
    }
    unsafe fn setZIndex_(&self, zIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZIndex : zIndex)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn setColliderShape_(&self, colliderShape: TCColliderShape)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColliderShape : colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCThumbstick(pub id);
impl std::ops::Deref for TCThumbstick {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCThumbstick {}
impl TCThumbstick {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCThumbstick").unwrap(), alloc) })
    }
}
impl PTCControl for TCThumbstick {}
impl PTCControlLayout for TCThumbstick {}
impl INSObject for TCThumbstick {}
impl PNSObject for TCThumbstick {}
impl std::convert::TryFrom<NSObject> for TCThumbstick {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCThumbstick, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCThumbstick").unwrap()) };
        if is_kind_of {
            Ok(TCThumbstick(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCThumbstick")
        }
    }
}
impl ITCThumbstick for TCThumbstick {}
pub trait ITCThumbstick: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn backgroundContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundContents)
    }
    unsafe fn setBackgroundContents_(&self, backgroundContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundContents : backgroundContents)
    }
    unsafe fn stickContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stickContents)
    }
    unsafe fn setStickContents_(&self, stickContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStickContents : stickContents)
    }
    unsafe fn hidesWhenNotPressed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hidesWhenNotPressed)
    }
    unsafe fn setHidesWhenNotPressed_(&self, hidesWhenNotPressed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidesWhenNotPressed : hidesWhenNotPressed)
    }
    unsafe fn stickSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stickSize)
    }
    unsafe fn setStickSize_(&self, stickSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStickSize : stickSize)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCThumbstickDescriptor(pub id);
impl std::ops::Deref for TCThumbstickDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCThumbstickDescriptor {}
impl TCThumbstickDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCThumbstickDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for TCThumbstickDescriptor {}
impl PNSObject for TCThumbstickDescriptor {}
impl std::convert::TryFrom<NSObject> for TCThumbstickDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCThumbstickDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCThumbstickDescriptor").unwrap()) };
        if is_kind_of {
            Ok(TCThumbstickDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCThumbstickDescriptor")
        }
    }
}
impl ITCThumbstickDescriptor for TCThumbstickDescriptor {}
pub trait ITCThumbstickDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn label(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn backgroundContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundContents)
    }
    unsafe fn setBackgroundContents_(&self, backgroundContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundContents : backgroundContents)
    }
    unsafe fn stickContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stickContents)
    }
    unsafe fn setStickContents_(&self, stickContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStickContents : stickContents)
    }
    unsafe fn hidesWhenNotPressed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hidesWhenNotPressed)
    }
    unsafe fn setHidesWhenNotPressed_(&self, hidesWhenNotPressed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidesWhenNotPressed : hidesWhenNotPressed)
    }
    unsafe fn stickSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stickSize)
    }
    unsafe fn setStickSize_(&self, stickSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStickSize : stickSize)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn anchor(&self) -> TCControlLayoutAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchor)
    }
    unsafe fn setAnchor_(&self, anchor: TCControlLayoutAnchor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchor : anchor)
    }
    unsafe fn anchorCoordinateSystem(&self) -> TCControlLayoutAnchorCoordinateSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorCoordinateSystem)
    }
    unsafe fn setAnchorCoordinateSystem_(
        &self,
        anchorCoordinateSystem: TCControlLayoutAnchorCoordinateSystem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorCoordinateSystem : anchorCoordinateSystem)
    }
    unsafe fn offset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn zIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zIndex)
    }
    unsafe fn setZIndex_(&self, zIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZIndex : zIndex)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn setColliderShape_(&self, colliderShape: TCColliderShape)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColliderShape : colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCDirectionPad(pub id);
impl std::ops::Deref for TCDirectionPad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCDirectionPad {}
impl TCDirectionPad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCDirectionPad").unwrap(), alloc) })
    }
}
impl PTCControl for TCDirectionPad {}
impl PTCControlLayout for TCDirectionPad {}
impl INSObject for TCDirectionPad {}
impl PNSObject for TCDirectionPad {}
impl std::convert::TryFrom<NSObject> for TCDirectionPad {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCDirectionPad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCDirectionPad").unwrap()) };
        if is_kind_of {
            Ok(TCDirectionPad(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCDirectionPad")
        }
    }
}
impl ITCDirectionPad for TCDirectionPad {}
pub trait ITCDirectionPad: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn compositeLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositeLabel)
    }
    unsafe fn setCompositeLabel_(&self, compositeLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompositeLabel : compositeLabel)
    }
    unsafe fn upLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upLabel)
    }
    unsafe fn setUpLabel_(&self, upLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpLabel : upLabel)
    }
    unsafe fn downLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downLabel)
    }
    unsafe fn setDownLabel_(&self, downLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownLabel : downLabel)
    }
    unsafe fn leftLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftLabel)
    }
    unsafe fn setLeftLabel_(&self, leftLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeftLabel : leftLabel)
    }
    unsafe fn rightLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightLabel)
    }
    unsafe fn setRightLabel_(&self, rightLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRightLabel : rightLabel)
    }
    unsafe fn upContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upContents)
    }
    unsafe fn setUpContents_(&self, upContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpContents : upContents)
    }
    unsafe fn downContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downContents)
    }
    unsafe fn setDownContents_(&self, downContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownContents : downContents)
    }
    unsafe fn leftContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftContents)
    }
    unsafe fn setLeftContents_(&self, leftContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeftContents : leftContents)
    }
    unsafe fn rightContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightContents)
    }
    unsafe fn setRightContents_(&self, rightContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRightContents : rightContents)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
    unsafe fn isRadial(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRadial)
    }
    unsafe fn setRadial_(&self, radial: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadial : radial)
    }
    unsafe fn isDigital(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDigital)
    }
    unsafe fn setDigital_(&self, digital: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDigital : digital)
    }
    unsafe fn inputIsMutuallyExclusive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputIsMutuallyExclusive)
    }
    unsafe fn setMutuallyExclusiveInput_(&self, mutuallyExclusiveInput: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMutuallyExclusiveInput : mutuallyExclusiveInput)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCDirectionPadDescriptor(pub id);
impl std::ops::Deref for TCDirectionPadDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCDirectionPadDescriptor {}
impl TCDirectionPadDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCDirectionPadDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for TCDirectionPadDescriptor {}
impl PNSObject for TCDirectionPadDescriptor {}
impl std::convert::TryFrom<NSObject> for TCDirectionPadDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCDirectionPadDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCDirectionPadDescriptor").unwrap()) };
        if is_kind_of {
            Ok(TCDirectionPadDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCDirectionPadDescriptor")
        }
    }
}
impl ITCDirectionPadDescriptor for TCDirectionPadDescriptor {}
pub trait ITCDirectionPadDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn compositeLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositeLabel)
    }
    unsafe fn setCompositeLabel_(&self, compositeLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompositeLabel : compositeLabel)
    }
    unsafe fn upLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upLabel)
    }
    unsafe fn setUpLabel_(&self, upLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpLabel : upLabel)
    }
    unsafe fn downLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downLabel)
    }
    unsafe fn setDownLabel_(&self, downLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownLabel : downLabel)
    }
    unsafe fn leftLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftLabel)
    }
    unsafe fn setLeftLabel_(&self, leftLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeftLabel : leftLabel)
    }
    unsafe fn rightLabel(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightLabel)
    }
    unsafe fn setRightLabel_(&self, rightLabel: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRightLabel : rightLabel)
    }
    unsafe fn upContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upContents)
    }
    unsafe fn setUpContents_(&self, upContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpContents : upContents)
    }
    unsafe fn downContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downContents)
    }
    unsafe fn setDownContents_(&self, downContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownContents : downContents)
    }
    unsafe fn leftContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftContents)
    }
    unsafe fn setLeftContents_(&self, leftContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeftContents : leftContents)
    }
    unsafe fn rightContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightContents)
    }
    unsafe fn setRightContents_(&self, rightContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRightContents : rightContents)
    }
    unsafe fn anchor(&self) -> TCControlLayoutAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchor)
    }
    unsafe fn setAnchor_(&self, anchor: TCControlLayoutAnchor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchor : anchor)
    }
    unsafe fn anchorCoordinateSystem(&self) -> TCControlLayoutAnchorCoordinateSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorCoordinateSystem)
    }
    unsafe fn setAnchorCoordinateSystem_(
        &self,
        anchorCoordinateSystem: TCControlLayoutAnchorCoordinateSystem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorCoordinateSystem : anchorCoordinateSystem)
    }
    unsafe fn offset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn zIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zIndex)
    }
    unsafe fn setZIndex_(&self, zIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZIndex : zIndex)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn setColliderShape_(&self, colliderShape: TCColliderShape)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColliderShape : colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
    unsafe fn isRadial(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRadial)
    }
    unsafe fn setRadial_(&self, radial: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadial : radial)
    }
    unsafe fn isDigital(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDigital)
    }
    unsafe fn setDigital_(&self, digital: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDigital : digital)
    }
    unsafe fn inputIsMutuallyExclusive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputIsMutuallyExclusive)
    }
    unsafe fn setMutuallyExclusiveInput_(&self, mutuallyExclusiveInput: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMutuallyExclusiveInput : mutuallyExclusiveInput)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCTouchpad(pub id);
impl std::ops::Deref for TCTouchpad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCTouchpad {}
impl TCTouchpad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCTouchpad").unwrap(), alloc) })
    }
}
impl PTCControl for TCTouchpad {}
impl PTCControlLayout for TCTouchpad {}
impl INSObject for TCTouchpad {}
impl PNSObject for TCTouchpad {}
impl std::convert::TryFrom<NSObject> for TCTouchpad {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCTouchpad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCTouchpad").unwrap()) };
        if is_kind_of {
            Ok(TCTouchpad(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCTouchpad")
        }
    }
}
impl ITCTouchpad for TCTouchpad {}
pub trait ITCTouchpad: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn contents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
    unsafe fn reportsRelativeValues(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportsRelativeValues)
    }
    unsafe fn setReportsRelativeValues_(&self, reportsRelativeValues: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportsRelativeValues : reportsRelativeValues)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCTouchpadDescriptor(pub id);
impl std::ops::Deref for TCTouchpadDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCTouchpadDescriptor {}
impl TCTouchpadDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCTouchpadDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for TCTouchpadDescriptor {}
impl PNSObject for TCTouchpadDescriptor {}
impl std::convert::TryFrom<NSObject> for TCTouchpadDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCTouchpadDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCTouchpadDescriptor").unwrap()) };
        if is_kind_of {
            Ok(TCTouchpadDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCTouchpadDescriptor")
        }
    }
}
impl ITCTouchpadDescriptor for TCTouchpadDescriptor {}
pub trait ITCTouchpadDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn label(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn contents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn anchor(&self) -> TCControlLayoutAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchor)
    }
    unsafe fn setAnchor_(&self, anchor: TCControlLayoutAnchor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchor : anchor)
    }
    unsafe fn anchorCoordinateSystem(&self) -> TCControlLayoutAnchorCoordinateSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorCoordinateSystem)
    }
    unsafe fn setAnchorCoordinateSystem_(
        &self,
        anchorCoordinateSystem: TCControlLayoutAnchorCoordinateSystem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorCoordinateSystem : anchorCoordinateSystem)
    }
    unsafe fn offset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn zIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zIndex)
    }
    unsafe fn setZIndex_(&self, zIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZIndex : zIndex)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn setColliderShape_(&self, colliderShape: TCColliderShape)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColliderShape : colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
    unsafe fn reportsRelativeValues(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportsRelativeValues)
    }
    unsafe fn setReportsRelativeValues_(&self, reportsRelativeValues: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportsRelativeValues : reportsRelativeValues)
    }
}
pub type TCThrottleOrientation = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCThrottle(pub id);
impl std::ops::Deref for TCThrottle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCThrottle {}
impl TCThrottle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCThrottle").unwrap(), alloc) })
    }
}
impl PTCControl for TCThrottle {}
impl PTCControlLayout for TCThrottle {}
impl INSObject for TCThrottle {}
impl PNSObject for TCThrottle {}
impl std::convert::TryFrom<NSObject> for TCThrottle {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCThrottle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCThrottle").unwrap()) };
        if is_kind_of {
            Ok(TCThrottle(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCThrottle")
        }
    }
}
impl ITCThrottle for TCThrottle {}
pub trait ITCThrottle: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn backgroundContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundContents)
    }
    unsafe fn setBackgroundContents_(&self, backgroundContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundContents : backgroundContents)
    }
    unsafe fn indicatorContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indicatorContents)
    }
    unsafe fn setIndicatorContents_(&self, indicatorContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndicatorContents : indicatorContents)
    }
    unsafe fn orientation(&self) -> TCThrottleOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn snapsToBaseValue(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapsToBaseValue)
    }
    unsafe fn setSnapsToBaseValue_(&self, snapsToBaseValue: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSnapsToBaseValue : snapsToBaseValue)
    }
    unsafe fn baseValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseValue)
    }
    unsafe fn setBaseValue_(&self, baseValue: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaseValue : baseValue)
    }
    unsafe fn indicatorSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indicatorSize)
    }
    unsafe fn setIndicatorSize_(&self, indicatorSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndicatorSize : indicatorSize)
    }
    unsafe fn throttleSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, throttleSize)
    }
    unsafe fn setThrottleSize_(&self, throttleSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThrottleSize : throttleSize)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCThrottleDescriptor(pub id);
impl std::ops::Deref for TCThrottleDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCThrottleDescriptor {}
impl TCThrottleDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCThrottleDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for TCThrottleDescriptor {}
impl PNSObject for TCThrottleDescriptor {}
impl std::convert::TryFrom<NSObject> for TCThrottleDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCThrottleDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCThrottleDescriptor").unwrap()) };
        if is_kind_of {
            Ok(TCThrottleDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCThrottleDescriptor")
        }
    }
}
impl ITCThrottleDescriptor for TCThrottleDescriptor {}
pub trait ITCThrottleDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn label(&self) -> TCControlLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: TCControlLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn backgroundContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundContents)
    }
    unsafe fn setBackgroundContents_(&self, backgroundContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundContents : backgroundContents)
    }
    unsafe fn indicatorContents(&self) -> TCControlContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indicatorContents)
    }
    unsafe fn setIndicatorContents_(&self, indicatorContents: TCControlContents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndicatorContents : indicatorContents)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn indicatorSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indicatorSize)
    }
    unsafe fn setIndicatorSize_(&self, indicatorSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndicatorSize : indicatorSize)
    }
    unsafe fn throttleSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, throttleSize)
    }
    unsafe fn setThrottleSize_(&self, throttleSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThrottleSize : throttleSize)
    }
    unsafe fn orientation(&self) -> TCThrottleOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn setOrientation_(&self, orientation: TCThrottleOrientation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientation : orientation)
    }
    unsafe fn snapsToBaseValue(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapsToBaseValue)
    }
    unsafe fn setSnapsToBaseValue_(&self, snapsToBaseValue: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSnapsToBaseValue : snapsToBaseValue)
    }
    unsafe fn baseValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseValue)
    }
    unsafe fn setBaseValue_(&self, baseValue: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaseValue : baseValue)
    }
    unsafe fn anchor(&self) -> TCControlLayoutAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchor)
    }
    unsafe fn setAnchor_(&self, anchor: TCControlLayoutAnchor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchor : anchor)
    }
    unsafe fn anchorCoordinateSystem(&self) -> TCControlLayoutAnchorCoordinateSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorCoordinateSystem)
    }
    unsafe fn setAnchorCoordinateSystem_(
        &self,
        anchorCoordinateSystem: TCControlLayoutAnchorCoordinateSystem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorCoordinateSystem : anchorCoordinateSystem)
    }
    unsafe fn offset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn zIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zIndex)
    }
    unsafe fn setZIndex_(&self, zIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZIndex : zIndex)
    }
    unsafe fn colliderShape(&self) -> TCColliderShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderShape)
    }
    unsafe fn setColliderShape_(&self, colliderShape: TCColliderShape)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColliderShape : colliderShape)
    }
    unsafe fn highlightDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightDuration)
    }
    unsafe fn setHighlightDuration_(&self, highlightDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightDuration : highlightDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TCTouchControllerDescriptor(pub id);
impl std::ops::Deref for TCTouchControllerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TCTouchControllerDescriptor {}
impl TCTouchControllerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TCTouchControllerDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for TCTouchControllerDescriptor {}
impl PNSObject for TCTouchControllerDescriptor {}
impl std::convert::TryFrom<NSObject> for TCTouchControllerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TCTouchControllerDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TCTouchControllerDescriptor").unwrap()) };
        if is_kind_of {
            Ok(TCTouchControllerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TCTouchControllerDescriptor")
        }
    }
}
impl ITCTouchControllerDescriptor for TCTouchControllerDescriptor {}
pub trait ITCTouchControllerDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMTKView_(&self, mtkView: MTKView) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMTKView : mtkView)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn setDevice_(&self, device: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDevice : device)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn drawableSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawableSize)
    }
    unsafe fn setDrawableSize_(&self, drawableSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawableSize : drawableSize)
    }
    unsafe fn colorPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorPixelFormat)
    }
    unsafe fn setColorPixelFormat_(&self, colorPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorPixelFormat : colorPixelFormat)
    }
    unsafe fn depthAttachmentPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthAttachmentPixelFormat)
    }
    unsafe fn setDepthAttachmentPixelFormat_(&self, depthAttachmentPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthAttachmentPixelFormat : depthAttachmentPixelFormat)
    }
    unsafe fn stencilAttachmentPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilAttachmentPixelFormat)
    }
    unsafe fn setStencilAttachmentPixelFormat_(&self, stencilAttachmentPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilAttachmentPixelFormat : stencilAttachmentPixelFormat)
    }
    unsafe fn sampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn setSampleCount_(&self, sampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleCount : sampleCount)
    }
}
unsafe extern "C" {
    pub static TCGameControllerProductCategoryTouchController: NSString;
}

unsafe impl objc2::encode::RefEncode for GCEventInteraction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCEventInteraction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCGameControllerActivationContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCGameControllerActivationContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCVirtualControllerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCVirtualControllerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCVirtualControllerElementConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCVirtualControllerElementConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCVirtualController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCVirtualController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCControlLabel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCControlLabel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCTouchController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCTouchController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCControlContents {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCControlContents {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCControlImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCControlImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCButtonDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCButtonDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCSwitch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCSwitch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCSwitchDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCSwitchDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCThumbstick {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCThumbstick {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCThumbstickDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCThumbstickDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCDirectionPad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCDirectionPad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCDirectionPadDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCDirectionPadDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCTouchpad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCTouchpad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCTouchpadDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCTouchpadDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCThrottle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCThrottle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCThrottleDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCThrottleDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TCTouchControllerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TCTouchControllerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
