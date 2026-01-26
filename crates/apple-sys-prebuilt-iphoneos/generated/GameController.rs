#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreHaptics::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDDevice {
    _unused: [u8; 0],
}
pub type IOHIDDeviceRef = *mut __IOHIDDevice;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GCPoint2 {
    pub x: f32,
    pub y: f32,
}
pub trait NSValue_GCTypes: Sized + std::ops::Deref {
    unsafe fn GCPoint2Value(&self) -> GCPoint2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GCPoint2Value)
    }
    unsafe fn valueWithGCPoint2_(point: GCPoint2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSValue").unwrap(), valueWithGCPoint2 : point)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCColor(pub id);
impl std::ops::Deref for GCColor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCColor {}
impl GCColor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCColor").unwrap(), alloc) })
    }
}
impl PNSCopying for GCColor {}
impl PNSSecureCoding for GCColor {}
impl INSObject for GCColor {}
impl PNSObject for GCColor {}
impl std::convert::TryFrom<NSObject> for GCColor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCColor, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCColor").unwrap()) };
        if is_kind_of {
            Ok(GCColor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCColor")
        }
    }
}
impl IGCColor for GCColor {}
pub trait IGCColor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRed_green_blue_(&self, red: f32, green: f32, blue: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue)
    }
    unsafe fn red(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, red)
    }
    unsafe fn green(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, green)
    }
    unsafe fn blue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blue)
    }
}
pub trait PGCDevice: Sized + std::ops::Deref {
    unsafe fn handlerQueue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handlerQueue)
    }
    unsafe fn setHandlerQueue_(&self, handlerQueue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandlerQueue : handlerQueue)
    }
    unsafe fn vendorName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vendorName)
    }
    unsafe fn productCategory(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productCategory)
    }
    unsafe fn physicalInputProfile(&self) -> GCPhysicalInputProfile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicalInputProfile)
    }
}
pub trait PGCPhysicalInputElementName: Sized + std::ops::Deref {}
pub trait PGCButtonElementName: Sized + std::ops::Deref {}
pub trait PGCAxisElementName: Sized + std::ops::Deref {}
pub trait PGCSwitchElementName: Sized + std::ops::Deref {}
pub trait PGCDirectionPadElementName: Sized + std::ops::Deref {}
pub type GCInputElementName = NSString;
pub type GCInputButtonName = NSString;
pub type GCInputAxisName = NSString;
pub type GCInputDirectionPadName = NSString;
pub trait PGCPhysicalInputElement: Sized + std::ops::Deref {
    unsafe fn aliases(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aliases)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn sfSymbolsName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sfSymbolsName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCPhysicalInputElementCollection(pub id);
impl std::ops::Deref for GCPhysicalInputElementCollection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCPhysicalInputElementCollection {}
impl GCPhysicalInputElementCollection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCPhysicalInputElementCollection").unwrap(), alloc) })
    }
}
impl PNSFastEnumeration for GCPhysicalInputElementCollection {}
impl INSObject for GCPhysicalInputElementCollection {}
impl PNSObject for GCPhysicalInputElementCollection {}
impl std::convert::TryFrom<NSObject> for GCPhysicalInputElementCollection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCPhysicalInputElementCollection, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCPhysicalInputElementCollection").unwrap())
        };
        if is_kind_of {
            Ok(GCPhysicalInputElementCollection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCPhysicalInputElementCollection")
        }
    }
}
impl<Key: 'static, Element: 'static> IGCPhysicalInputElementCollection<Key, Element>
    for GCPhysicalInputElementCollection
{
}
pub trait IGCPhysicalInputElementCollection<Key: 'static, Element: 'static>:
    Sized + std::ops::Deref
{
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn elementForAlias_(&self, alias: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementForAlias : alias)
    }
    unsafe fn objectForKeyedSubscript_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn elementEnumerator(&self) -> NSEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementEnumerator)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCPhysicalInputElementCollection").unwrap(), new)
    }
}
pub trait PGCLinearInput: Sized + std::ops::Deref {
    unsafe fn valueDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueDidChangeHandler)
    }
    unsafe fn setValueDidChangeHandler_(&self, valueDidChangeHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueDidChangeHandler : valueDidChangeHandler)
    }
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn isAnalog(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnalog)
    }
    unsafe fn canWrap(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canWrap)
    }
    unsafe fn lastValueTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastValueTimestamp)
    }
    unsafe fn lastValueLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastValueLatency)
    }
    unsafe fn physicalExtents(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicalExtents)
    }
    unsafe fn sources(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
pub trait PGCPressedStateInput: Sized + std::ops::Deref {
    unsafe fn pressedDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressedDidChangeHandler)
    }
    unsafe fn setPressedDidChangeHandler_(
        &self,
        pressedDidChangeHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPressedDidChangeHandler : pressedDidChangeHandler)
    }
    unsafe fn isPressed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPressed)
    }
    unsafe fn lastPressedStateTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastPressedStateTimestamp)
    }
    unsafe fn lastPressedStateLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastPressedStateLatency)
    }
    unsafe fn sources(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
pub trait PGCTouchedStateInput: Sized + std::ops::Deref {
    unsafe fn touchedDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchedDidChangeHandler)
    }
    unsafe fn setTouchedDidChangeHandler_(
        &self,
        touchedDidChangeHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTouchedDidChangeHandler : touchedDidChangeHandler)
    }
    unsafe fn isTouched(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTouched)
    }
    unsafe fn lastTouchedStateTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastTouchedStateTimestamp)
    }
    unsafe fn lastTouchedStateLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastTouchedStateLatency)
    }
    unsafe fn sources(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
pub trait PGCButtonElement: Sized + std::ops::Deref {
    unsafe fn pressedInput(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressedInput)
    }
    unsafe fn touchedInput(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchedInput)
    }
    unsafe fn forceInput(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forceInput)
    }
}
pub trait PGCAxisInput: Sized + std::ops::Deref {
    unsafe fn valueDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueDidChangeHandler)
    }
    unsafe fn setValueDidChangeHandler_(&self, valueDidChangeHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueDidChangeHandler : valueDidChangeHandler)
    }
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn isAnalog(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnalog)
    }
    unsafe fn canWrap(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canWrap)
    }
    unsafe fn lastValueTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastValueTimestamp)
    }
    unsafe fn lastValueLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastValueLatency)
    }
    unsafe fn sources(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
pub trait PGCRelativeInput: Sized + std::ops::Deref {
    unsafe fn deltaDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deltaDidChangeHandler)
    }
    unsafe fn setDeltaDidChangeHandler_(&self, deltaDidChangeHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeltaDidChangeHandler : deltaDidChangeHandler)
    }
    unsafe fn delta(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delta)
    }
    unsafe fn isAnalog(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnalog)
    }
    unsafe fn lastDeltaTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastDeltaTimestamp)
    }
    unsafe fn lastDeltaLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastDeltaLatency)
    }
    unsafe fn sources(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
pub trait PGCAxisElement: Sized + std::ops::Deref {
    unsafe fn absoluteInput(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteInput)
    }
    unsafe fn relativeInput(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeInput)
    }
}
pub trait PGCSwitchPositionInput: Sized + std::ops::Deref {
    unsafe fn positionDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, positionDidChangeHandler)
    }
    unsafe fn setPositionDidChangeHandler_(
        &self,
        positionDidChangeHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPositionDidChangeHandler : positionDidChangeHandler)
    }
    unsafe fn position(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn positionRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, positionRange)
    }
    unsafe fn isSequential(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSequential)
    }
    unsafe fn canWrap(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canWrap)
    }
    unsafe fn lastPositionTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastPositionTimestamp)
    }
    unsafe fn lastPositionLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastPositionLatency)
    }
    unsafe fn sources(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
pub trait PGCSwitchElement: Sized + std::ops::Deref {
    unsafe fn positionInput(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, positionInput)
    }
}
pub trait PGCAxis2DInput: Sized + std::ops::Deref {
    unsafe fn valueDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueDidChangeHandler)
    }
    unsafe fn setValueDidChangeHandler_(&self, valueDidChangeHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueDidChangeHandler : valueDidChangeHandler)
    }
    unsafe fn value(&self) -> GCPoint2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn isAnalog(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnalog)
    }
    unsafe fn canWrap(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canWrap)
    }
    unsafe fn lastValueTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastValueTimestamp)
    }
    unsafe fn lastValueLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastValueLatency)
    }
    unsafe fn sources(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
pub trait PGCDirectionPadElement: Sized + std::ops::Deref {
    unsafe fn xyAxes(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xyAxes)
    }
    unsafe fn xAxis(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xAxis)
    }
    unsafe fn yAxis(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yAxis)
    }
    unsafe fn up(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, up)
    }
    unsafe fn down(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, down)
    }
    unsafe fn left(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, left)
    }
    unsafe fn right(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, right)
    }
}
pub trait PGCDevicePhysicalInputState: Sized + std::ops::Deref {
    unsafe fn objectForKeyedSubscript_(&self, key: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn lastEventTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastEventTimestamp)
    }
    unsafe fn lastEventLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastEventLatency)
    }
    unsafe fn elements(&self) -> GCPhysicalInputElementCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn buttons(&self) -> GCPhysicalInputElementCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttons)
    }
    unsafe fn axes(&self) -> GCPhysicalInputElementCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axes)
    }
    unsafe fn switches(&self) -> GCPhysicalInputElementCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, switches)
    }
    unsafe fn dpads(&self) -> GCPhysicalInputElementCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dpads)
    }
}
pub type GCDevicePhysicalInputElementChange = NSInteger;
pub trait PGCDevicePhysicalInputStateDiff: Sized + std::ops::Deref {
    unsafe fn changeForElement_(&self, element: *mut u64) -> GCDevicePhysicalInputElementChange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeForElement : element)
    }
    unsafe fn changedElements(&self) -> NSEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedElements)
    }
}
pub trait PGCDevicePhysicalInput: Sized + std::ops::Deref {
    unsafe fn capture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capture)
    }
    unsafe fn nextInputState(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextInputState)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn queue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queue)
    }
    unsafe fn setQueue_(&self, queue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueue : queue)
    }
    unsafe fn elementValueDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementValueDidChangeHandler)
    }
    unsafe fn setElementValueDidChangeHandler_(
        &self,
        elementValueDidChangeHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElementValueDidChangeHandler : elementValueDidChangeHandler)
    }
    unsafe fn inputStateAvailableHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputStateAvailableHandler)
    }
    unsafe fn setInputStateAvailableHandler_(
        &self,
        inputStateAvailableHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputStateAvailableHandler : inputStateAvailableHandler)
    }
    unsafe fn inputStateQueueDepth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputStateQueueDepth)
    }
    unsafe fn setInputStateQueueDepth_(&self, inputStateQueueDepth: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputStateQueueDepth : inputStateQueueDepth)
    }
}
pub type GCPhysicalInputSourceDirection = NSUInteger;
pub trait PGCPhysicalInputSource: Sized + std::ops::Deref {
    unsafe fn elementAliases(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementAliases)
    }
    unsafe fn elementLocalizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementLocalizedName)
    }
    unsafe fn sfSymbolsName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sfSymbolsName)
    }
    unsafe fn direction(&self) -> GCPhysicalInputSourceDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
}
pub trait PGCPhysicalInputExtents: Sized + std::ops::Deref {
    unsafe fn scaledValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaledValue)
    }
    unsafe fn minimumValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumValue)
    }
    unsafe fn maximumValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCDeviceLight(pub id);
impl std::ops::Deref for GCDeviceLight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCDeviceLight {}
impl GCDeviceLight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCDeviceLight").unwrap(), alloc) })
    }
}
impl INSObject for GCDeviceLight {}
impl PNSObject for GCDeviceLight {}
impl std::convert::TryFrom<NSObject> for GCDeviceLight {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCDeviceLight, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCDeviceLight").unwrap()) };
        if is_kind_of {
            Ok(GCDeviceLight(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCDeviceLight")
        }
    }
}
impl IGCDeviceLight for GCDeviceLight {}
pub trait IGCDeviceLight: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn color(&self) -> GCColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: GCColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub type GCDeviceBatteryState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCDeviceBattery(pub id);
impl std::ops::Deref for GCDeviceBattery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCDeviceBattery {}
impl GCDeviceBattery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCDeviceBattery").unwrap(), alloc) })
    }
}
impl INSObject for GCDeviceBattery {}
impl PNSObject for GCDeviceBattery {}
impl std::convert::TryFrom<NSObject> for GCDeviceBattery {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCDeviceBattery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCDeviceBattery").unwrap()) };
        if is_kind_of {
            Ok(GCDeviceBattery(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCDeviceBattery")
        }
    }
}
impl IGCDeviceBattery for GCDeviceBattery {}
pub trait IGCDeviceBattery: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn batteryLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, batteryLevel)
    }
    unsafe fn batteryState(&self) -> GCDeviceBatteryState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, batteryState)
    }
}
pub type GCSystemGestureState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCControllerElement(pub id);
impl std::ops::Deref for GCControllerElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCControllerElement {}
impl GCControllerElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCControllerElement").unwrap(), alloc) })
    }
}
impl INSObject for GCControllerElement {}
impl PNSObject for GCControllerElement {}
impl std::convert::TryFrom<NSObject> for GCControllerElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCControllerElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCControllerElement").unwrap()) };
        if is_kind_of {
            Ok(GCControllerElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCControllerElement")
        }
    }
}
impl IGCControllerElement for GCControllerElement {}
pub trait IGCControllerElement: Sized + std::ops::Deref {
    unsafe fn collection(&self) -> GCControllerElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collection)
    }
    unsafe fn isAnalog(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnalog)
    }
    unsafe fn isBoundToSystemGesture(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBoundToSystemGesture)
    }
    unsafe fn preferredSystemGestureState(&self) -> GCSystemGestureState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredSystemGestureState)
    }
    unsafe fn setPreferredSystemGestureState_(
        &self,
        preferredSystemGestureState: GCSystemGestureState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredSystemGestureState : preferredSystemGestureState)
    }
    unsafe fn sfSymbolsName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sfSymbolsName)
    }
    unsafe fn setSfSymbolsName_(&self, sfSymbolsName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSfSymbolsName : sfSymbolsName)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn setLocalizedName_(&self, localizedName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedName : localizedName)
    }
    unsafe fn unmappedSfSymbolsName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unmappedSfSymbolsName)
    }
    unsafe fn setUnmappedSfSymbolsName_(&self, unmappedSfSymbolsName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnmappedSfSymbolsName : unmappedSfSymbolsName)
    }
    unsafe fn unmappedLocalizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unmappedLocalizedName)
    }
    unsafe fn setUnmappedLocalizedName_(&self, unmappedLocalizedName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnmappedLocalizedName : unmappedLocalizedName)
    }
    unsafe fn aliases(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aliases)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCControllerAxisInput(pub id);
impl std::ops::Deref for GCControllerAxisInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCControllerAxisInput {}
impl GCControllerAxisInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCControllerAxisInput").unwrap(), alloc) })
    }
}
impl IGCControllerElement for GCControllerAxisInput {}
impl From<GCControllerAxisInput> for GCControllerElement {
    fn from(child: GCControllerAxisInput) -> GCControllerElement {
        GCControllerElement(child.0)
    }
}
impl std::convert::TryFrom<GCControllerElement> for GCControllerAxisInput {
    type Error = &'static str;
    fn try_from(parent: GCControllerElement) -> Result<GCControllerAxisInput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCControllerAxisInput").unwrap()) };
        if is_kind_of {
            Ok(GCControllerAxisInput(parent.0))
        } else {
            Err("This GCControllerElement cannot be downcasted to GCControllerAxisInput")
        }
    }
}
impl INSObject for GCControllerAxisInput {}
impl PNSObject for GCControllerAxisInput {}
impl IGCControllerAxisInput for GCControllerAxisInput {}
pub trait IGCControllerAxisInput: Sized + std::ops::Deref {
    unsafe fn setValue_(&self, value: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn valueChangedHandler(&self) -> GCControllerAxisValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueChangedHandler)
    }
    unsafe fn setValueChangedHandler_(
        &self,
        valueChangedHandler: GCControllerAxisValueChangedHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueChangedHandler : valueChangedHandler)
    }
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
}
pub type GCControllerAxisValueChangedHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCControllerButtonInput(pub id);
impl std::ops::Deref for GCControllerButtonInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCControllerButtonInput {}
impl GCControllerButtonInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCControllerButtonInput").unwrap(), alloc) })
    }
}
impl IGCControllerElement for GCControllerButtonInput {}
impl std::convert::TryFrom<GCControllerElement> for GCControllerButtonInput {
    type Error = &'static str;
    fn try_from(parent: GCControllerElement) -> Result<GCControllerButtonInput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCControllerButtonInput").unwrap()) };
        if is_kind_of {
            Ok(GCControllerButtonInput(parent.0))
        } else {
            Err("This GCControllerElement cannot be downcasted to GCControllerButtonInput")
        }
    }
}
impl INSObject for GCControllerButtonInput {}
impl PNSObject for GCControllerButtonInput {}
impl IGCControllerButtonInput for GCControllerButtonInput {}
pub trait IGCControllerButtonInput: Sized + std::ops::Deref {
    unsafe fn setValue_(&self, value: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn valueChangedHandler(&self) -> GCControllerButtonValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueChangedHandler)
    }
    unsafe fn setValueChangedHandler_(
        &self,
        valueChangedHandler: GCControllerButtonValueChangedHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueChangedHandler : valueChangedHandler)
    }
    unsafe fn pressedChangedHandler(&self) -> GCControllerButtonValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressedChangedHandler)
    }
    unsafe fn setPressedChangedHandler_(
        &self,
        pressedChangedHandler: GCControllerButtonValueChangedHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPressedChangedHandler : pressedChangedHandler)
    }
    unsafe fn touchedChangedHandler(&self) -> GCControllerButtonTouchedChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchedChangedHandler)
    }
    unsafe fn setTouchedChangedHandler_(
        &self,
        touchedChangedHandler: GCControllerButtonTouchedChangedHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTouchedChangedHandler : touchedChangedHandler)
    }
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn isPressed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPressed)
    }
    unsafe fn isTouched(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTouched)
    }
}
pub type GCControllerButtonValueChangedHandler = *mut ::std::os::raw::c_void;
pub type GCControllerButtonTouchedChangedHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCControllerDirectionPad(pub id);
impl std::ops::Deref for GCControllerDirectionPad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCControllerDirectionPad {}
impl GCControllerDirectionPad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCControllerDirectionPad").unwrap(), alloc) })
    }
}
impl IGCControllerElement for GCControllerDirectionPad {}
impl std::convert::TryFrom<GCControllerElement> for GCControllerDirectionPad {
    type Error = &'static str;
    fn try_from(parent: GCControllerElement) -> Result<GCControllerDirectionPad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCControllerDirectionPad").unwrap()) };
        if is_kind_of {
            Ok(GCControllerDirectionPad(parent.0))
        } else {
            Err("This GCControllerElement cannot be downcasted to GCControllerDirectionPad")
        }
    }
}
impl INSObject for GCControllerDirectionPad {}
impl PNSObject for GCControllerDirectionPad {}
impl IGCControllerDirectionPad for GCControllerDirectionPad {}
pub trait IGCControllerDirectionPad: Sized + std::ops::Deref {
    unsafe fn setValueForXAxis_yAxis_(&self, xAxis: f32, yAxis: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueForXAxis : xAxis, yAxis : yAxis)
    }
    unsafe fn valueChangedHandler(&self) -> GCControllerDirectionPadValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueChangedHandler)
    }
    unsafe fn setValueChangedHandler_(
        &self,
        valueChangedHandler: GCControllerDirectionPadValueChangedHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueChangedHandler : valueChangedHandler)
    }
    unsafe fn xAxis(&self) -> GCControllerAxisInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xAxis)
    }
    unsafe fn yAxis(&self) -> GCControllerAxisInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yAxis)
    }
    unsafe fn up(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, up)
    }
    unsafe fn down(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, down)
    }
    unsafe fn left(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, left)
    }
    unsafe fn right(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, right)
    }
}
pub type GCControllerDirectionPadValueChangedHandler = *mut ::std::os::raw::c_void;
pub type GCTouchState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCControllerTouchpad(pub id);
impl std::ops::Deref for GCControllerTouchpad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCControllerTouchpad {}
impl GCControllerTouchpad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCControllerTouchpad").unwrap(), alloc) })
    }
}
impl IGCControllerElement for GCControllerTouchpad {}
impl std::convert::TryFrom<GCControllerElement> for GCControllerTouchpad {
    type Error = &'static str;
    fn try_from(parent: GCControllerElement) -> Result<GCControllerTouchpad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCControllerTouchpad").unwrap()) };
        if is_kind_of {
            Ok(GCControllerTouchpad(parent.0))
        } else {
            Err("This GCControllerElement cannot be downcasted to GCControllerTouchpad")
        }
    }
}
impl INSObject for GCControllerTouchpad {}
impl PNSObject for GCControllerTouchpad {}
impl IGCControllerTouchpad for GCControllerTouchpad {}
pub trait IGCControllerTouchpad: Sized + std::ops::Deref {
    unsafe fn setValueForXAxis_yAxis_touchDown_buttonValue_(
        &self,
        xAxis: f32,
        yAxis: f32,
        touchDown: BOOL,
        buttonValue: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueForXAxis : xAxis, yAxis : yAxis, touchDown : touchDown, buttonValue : buttonValue)
    }
    unsafe fn button(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, button)
    }
    unsafe fn touchDown(&self) -> GCControllerTouchpadHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchDown)
    }
    unsafe fn setTouchDown_(&self, touchDown: GCControllerTouchpadHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTouchDown : touchDown)
    }
    unsafe fn touchMoved(&self) -> GCControllerTouchpadHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchMoved)
    }
    unsafe fn setTouchMoved_(&self, touchMoved: GCControllerTouchpadHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTouchMoved : touchMoved)
    }
    unsafe fn touchUp(&self) -> GCControllerTouchpadHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchUp)
    }
    unsafe fn setTouchUp_(&self, touchUp: GCControllerTouchpadHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTouchUp : touchUp)
    }
    unsafe fn touchSurface(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchSurface)
    }
    unsafe fn touchState(&self) -> GCTouchState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchState)
    }
    unsafe fn reportsAbsoluteTouchSurfaceValues(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportsAbsoluteTouchSurfaceValues)
    }
    unsafe fn setReportsAbsoluteTouchSurfaceValues_(&self, reportsAbsoluteTouchSurfaceValues: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportsAbsoluteTouchSurfaceValues : reportsAbsoluteTouchSurfaceValues)
    }
}
pub type GCControllerTouchpadHandler = *mut ::std::os::raw::c_void;
pub type GCDualSenseAdaptiveTriggerMode = NSInteger;
pub type GCDualSenseAdaptiveTriggerStatus = NSInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GCDualSenseAdaptiveTriggerPositionalAmplitudes {
    pub values: [f32; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GCDualSenseAdaptiveTriggerPositionalResistiveStrengths {
    pub values: [f32; 10usize],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCDualSenseAdaptiveTrigger(pub id);
impl std::ops::Deref for GCDualSenseAdaptiveTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCDualSenseAdaptiveTrigger {}
impl GCDualSenseAdaptiveTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCDualSenseAdaptiveTrigger").unwrap(), alloc) })
    }
}
impl IGCControllerButtonInput for GCDualSenseAdaptiveTrigger {}
impl From<GCDualSenseAdaptiveTrigger> for GCControllerButtonInput {
    fn from(child: GCDualSenseAdaptiveTrigger) -> GCControllerButtonInput {
        GCControllerButtonInput(child.0)
    }
}
impl std::convert::TryFrom<GCControllerButtonInput> for GCDualSenseAdaptiveTrigger {
    type Error = &'static str;
    fn try_from(
        parent: GCControllerButtonInput,
    ) -> Result<GCDualSenseAdaptiveTrigger, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCDualSenseAdaptiveTrigger").unwrap()) };
        if is_kind_of {
            Ok(GCDualSenseAdaptiveTrigger(parent.0))
        } else {
            Err("This GCControllerButtonInput cannot be downcasted to GCDualSenseAdaptiveTrigger")
        }
    }
}
impl IGCControllerElement for GCDualSenseAdaptiveTrigger {}
impl INSObject for GCDualSenseAdaptiveTrigger {}
impl PNSObject for GCDualSenseAdaptiveTrigger {}
impl IGCDualSenseAdaptiveTrigger for GCDualSenseAdaptiveTrigger {}
pub trait IGCDualSenseAdaptiveTrigger: Sized + std::ops::Deref {
    unsafe fn setModeSlopeFeedbackWithStartPosition_endPosition_startStrength_endStrength_(
        &self,
        startPosition: f32,
        endPosition: f32,
        startStrength: f32,
        endStrength: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModeSlopeFeedbackWithStartPosition : startPosition, endPosition : endPosition, startStrength : startStrength, endStrength : endStrength)
    }
    unsafe fn setModeFeedbackWithStartPosition_resistiveStrength_(
        &self,
        startPosition: f32,
        resistiveStrength: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModeFeedbackWithStartPosition : startPosition, resistiveStrength : resistiveStrength)
    }
    unsafe fn setModeFeedbackWithResistiveStrengths_(
        &self,
        positionalResistiveStrengths: GCDualSenseAdaptiveTriggerPositionalResistiveStrengths,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModeFeedbackWithResistiveStrengths : positionalResistiveStrengths)
    }
    unsafe fn setModeWeaponWithStartPosition_endPosition_resistiveStrength_(
        &self,
        startPosition: f32,
        endPosition: f32,
        resistiveStrength: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModeWeaponWithStartPosition : startPosition, endPosition : endPosition, resistiveStrength : resistiveStrength)
    }
    unsafe fn setModeVibrationWithStartPosition_amplitude_frequency_(
        &self,
        startPosition: f32,
        amplitude: f32,
        frequency: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModeVibrationWithStartPosition : startPosition, amplitude : amplitude, frequency : frequency)
    }
    unsafe fn setModeVibrationWithAmplitudes_frequency_(
        &self,
        positionalAmplitudes: GCDualSenseAdaptiveTriggerPositionalAmplitudes,
        frequency: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModeVibrationWithAmplitudes : positionalAmplitudes, frequency : frequency)
    }
    unsafe fn setModeOff(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setModeOff)
    }
    unsafe fn mode(&self) -> GCDualSenseAdaptiveTriggerMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn status(&self) -> GCDualSenseAdaptiveTriggerStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn armPosition(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, armPosition)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCDeviceCursor(pub id);
impl std::ops::Deref for GCDeviceCursor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCDeviceCursor {}
impl GCDeviceCursor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCDeviceCursor").unwrap(), alloc) })
    }
}
impl IGCControllerDirectionPad for GCDeviceCursor {}
impl From<GCDeviceCursor> for GCControllerDirectionPad {
    fn from(child: GCDeviceCursor) -> GCControllerDirectionPad {
        GCControllerDirectionPad(child.0)
    }
}
impl std::convert::TryFrom<GCControllerDirectionPad> for GCDeviceCursor {
    type Error = &'static str;
    fn try_from(parent: GCControllerDirectionPad) -> Result<GCDeviceCursor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCDeviceCursor").unwrap()) };
        if is_kind_of {
            Ok(GCDeviceCursor(parent.0))
        } else {
            Err("This GCControllerDirectionPad cannot be downcasted to GCDeviceCursor")
        }
    }
}
impl IGCControllerElement for GCDeviceCursor {}
impl INSObject for GCDeviceCursor {}
impl PNSObject for GCDeviceCursor {}
impl IGCDeviceCursor for GCDeviceCursor {}
pub trait IGCDeviceCursor: Sized + std::ops::Deref {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GCAcceleration {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GCRotationRate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GCEulerAngles {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GCQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCMotion(pub id);
impl std::ops::Deref for GCMotion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCMotion {}
impl GCMotion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCMotion").unwrap(), alloc) })
    }
}
impl INSObject for GCMotion {}
impl PNSObject for GCMotion {}
impl std::convert::TryFrom<NSObject> for GCMotion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCMotion, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCMotion").unwrap()) };
        if is_kind_of {
            Ok(GCMotion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCMotion")
        }
    }
}
impl IGCMotion for GCMotion {}
pub trait IGCMotion: Sized + std::ops::Deref {
    unsafe fn setGravity_(&self, gravity: GCAcceleration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGravity : gravity)
    }
    unsafe fn setUserAcceleration_(&self, userAcceleration: GCAcceleration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserAcceleration : userAcceleration)
    }
    unsafe fn setAcceleration_(&self, acceleration: GCAcceleration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcceleration : acceleration)
    }
    unsafe fn setAttitude_(&self, attitude: GCQuaternion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttitude : attitude)
    }
    unsafe fn setRotationRate_(&self, rotationRate: GCRotationRate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotationRate : rotationRate)
    }
    unsafe fn setStateFromMotion_(&self, motion: GCMotion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStateFromMotion : motion)
    }
    unsafe fn controller(&self) -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controller)
    }
    unsafe fn valueChangedHandler(&self) -> GCMotionValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueChangedHandler)
    }
    unsafe fn setValueChangedHandler_(&self, valueChangedHandler: GCMotionValueChangedHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueChangedHandler : valueChangedHandler)
    }
    unsafe fn sensorsRequireManualActivation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensorsRequireManualActivation)
    }
    unsafe fn sensorsActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensorsActive)
    }
    unsafe fn setSensorsActive_(&self, sensorsActive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSensorsActive : sensorsActive)
    }
    unsafe fn hasGravityAndUserAcceleration(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasGravityAndUserAcceleration)
    }
    unsafe fn gravity(&self) -> GCAcceleration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gravity)
    }
    unsafe fn userAcceleration(&self) -> GCAcceleration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userAcceleration)
    }
    unsafe fn acceleration(&self) -> GCAcceleration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceleration)
    }
    unsafe fn hasAttitudeAndRotationRate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAttitudeAndRotationRate)
    }
    unsafe fn hasAttitude(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAttitude)
    }
    unsafe fn hasRotationRate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasRotationRate)
    }
    unsafe fn attitude(&self) -> GCQuaternion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attitude)
    }
    unsafe fn rotationRate(&self) -> GCRotationRate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationRate)
    }
}
pub type GCMotionValueChangedHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCPhysicalInputProfile(pub id);
impl std::ops::Deref for GCPhysicalInputProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCPhysicalInputProfile {}
impl GCPhysicalInputProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCPhysicalInputProfile").unwrap(), alloc) })
    }
}
impl INSObject for GCPhysicalInputProfile {}
impl PNSObject for GCPhysicalInputProfile {}
impl std::convert::TryFrom<NSObject> for GCPhysicalInputProfile {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCPhysicalInputProfile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCPhysicalInputProfile").unwrap()) };
        if is_kind_of {
            Ok(GCPhysicalInputProfile(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCPhysicalInputProfile")
        }
    }
}
impl IGCPhysicalInputProfile for GCPhysicalInputProfile {}
pub trait IGCPhysicalInputProfile: Sized + std::ops::Deref {
    unsafe fn objectForKeyedSubscript_(&self, key: NSString) -> GCControllerElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn capture(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capture)
    }
    unsafe fn setStateFromPhysicalInput_(&self, physicalInput: GCPhysicalInputProfile)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStateFromPhysicalInput : physicalInput)
    }
    unsafe fn mappedElementAliasForPhysicalInputName_(&self, inputName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mappedElementAliasForPhysicalInputName : inputName)
    }
    unsafe fn mappedPhysicalInputNamesForElementAlias_(&self, elementAlias: NSString) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mappedPhysicalInputNamesForElementAlias : elementAlias)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn lastEventTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastEventTimestamp)
    }
    unsafe fn hasRemappedElements(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasRemappedElements)
    }
    unsafe fn valueDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueDidChangeHandler)
    }
    unsafe fn setValueDidChangeHandler_(&self, valueDidChangeHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueDidChangeHandler : valueDidChangeHandler)
    }
    unsafe fn elements(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn buttons(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttons)
    }
    unsafe fn axes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axes)
    }
    unsafe fn dpads(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dpads)
    }
    unsafe fn touchpads(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchpads)
    }
    unsafe fn allElements(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allElements)
    }
    unsafe fn allButtons(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allButtons)
    }
    unsafe fn allAxes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allAxes)
    }
    unsafe fn allDpads(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allDpads)
    }
    unsafe fn allTouchpads(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allTouchpads)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCGamepad(pub id);
impl std::ops::Deref for GCGamepad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCGamepad {}
impl GCGamepad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCGamepad").unwrap(), alloc) })
    }
}
impl IGCPhysicalInputProfile for GCGamepad {}
impl From<GCGamepad> for GCPhysicalInputProfile {
    fn from(child: GCGamepad) -> GCPhysicalInputProfile {
        GCPhysicalInputProfile(child.0)
    }
}
impl std::convert::TryFrom<GCPhysicalInputProfile> for GCGamepad {
    type Error = &'static str;
    fn try_from(parent: GCPhysicalInputProfile) -> Result<GCGamepad, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCGamepad").unwrap()) };
        if is_kind_of {
            Ok(GCGamepad(parent.0))
        } else {
            Err("This GCPhysicalInputProfile cannot be downcasted to GCGamepad")
        }
    }
}
impl INSObject for GCGamepad {}
impl PNSObject for GCGamepad {}
impl IGCGamepad for GCGamepad {}
pub trait IGCGamepad: Sized + std::ops::Deref {
    unsafe fn saveSnapshot(&self) -> GCGamepadSnapshot
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saveSnapshot)
    }
    unsafe fn controller(&self) -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controller)
    }
    unsafe fn valueChangedHandler(&self) -> GCGamepadValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueChangedHandler)
    }
    unsafe fn setValueChangedHandler_(&self, valueChangedHandler: GCGamepadValueChangedHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueChangedHandler : valueChangedHandler)
    }
    unsafe fn dpad(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dpad)
    }
    unsafe fn buttonA(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonA)
    }
    unsafe fn buttonB(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonB)
    }
    unsafe fn buttonX(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonX)
    }
    unsafe fn buttonY(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonY)
    }
    unsafe fn leftShoulder(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftShoulder)
    }
    unsafe fn rightShoulder(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightShoulder)
    }
}
pub type GCGamepadValueChangedHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCGamepadSnapshot(pub id);
impl std::ops::Deref for GCGamepadSnapshot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCGamepadSnapshot {}
impl GCGamepadSnapshot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCGamepadSnapshot").unwrap(), alloc) })
    }
}
impl IGCGamepad for GCGamepadSnapshot {}
impl From<GCGamepadSnapshot> for GCGamepad {
    fn from(child: GCGamepadSnapshot) -> GCGamepad {
        GCGamepad(child.0)
    }
}
impl std::convert::TryFrom<GCGamepad> for GCGamepadSnapshot {
    type Error = &'static str;
    fn try_from(parent: GCGamepad) -> Result<GCGamepadSnapshot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCGamepadSnapshot").unwrap()) };
        if is_kind_of {
            Ok(GCGamepadSnapshot(parent.0))
        } else {
            Err("This GCGamepad cannot be downcasted to GCGamepadSnapshot")
        }
    }
}
impl IGCPhysicalInputProfile for GCGamepadSnapshot {}
impl INSObject for GCGamepadSnapshot {}
impl PNSObject for GCGamepadSnapshot {}
impl IGCGamepadSnapshot for GCGamepadSnapshot {}
pub trait IGCGamepadSnapshot: Sized + std::ops::Deref {
    unsafe fn initWithSnapshotData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSnapshotData : data)
    }
    unsafe fn initWithController_snapshotData_(
        &self,
        controller: GCController,
        data: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithController : controller, snapshotData : data)
    }
    unsafe fn snapshotData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotData)
    }
    unsafe fn setSnapshotData_(&self, snapshotData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSnapshotData : snapshotData)
    }
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct GCGamepadSnapShotDataV100 {
    pub version: u16,
    pub size: u16,
    pub dpadX: f32,
    pub dpadY: f32,
    pub buttonA: f32,
    pub buttonB: f32,
    pub buttonX: f32,
    pub buttonY: f32,
    pub leftShoulder: f32,
    pub rightShoulder: f32,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCExtendedGamepad(pub id);
impl std::ops::Deref for GCExtendedGamepad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCExtendedGamepad {}
impl GCExtendedGamepad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCExtendedGamepad").unwrap(), alloc) })
    }
}
impl IGCPhysicalInputProfile for GCExtendedGamepad {}
impl std::convert::TryFrom<GCPhysicalInputProfile> for GCExtendedGamepad {
    type Error = &'static str;
    fn try_from(parent: GCPhysicalInputProfile) -> Result<GCExtendedGamepad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCExtendedGamepad").unwrap()) };
        if is_kind_of {
            Ok(GCExtendedGamepad(parent.0))
        } else {
            Err("This GCPhysicalInputProfile cannot be downcasted to GCExtendedGamepad")
        }
    }
}
impl INSObject for GCExtendedGamepad {}
impl PNSObject for GCExtendedGamepad {}
impl IGCExtendedGamepad for GCExtendedGamepad {}
pub trait IGCExtendedGamepad: Sized + std::ops::Deref {
    unsafe fn saveSnapshot(&self) -> GCExtendedGamepadSnapshot
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saveSnapshot)
    }
    unsafe fn setStateFromExtendedGamepad_(&self, extendedGamepad: GCExtendedGamepad)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStateFromExtendedGamepad : extendedGamepad)
    }
    unsafe fn controller(&self) -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controller)
    }
    unsafe fn valueChangedHandler(&self) -> GCExtendedGamepadValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueChangedHandler)
    }
    unsafe fn setValueChangedHandler_(
        &self,
        valueChangedHandler: GCExtendedGamepadValueChangedHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueChangedHandler : valueChangedHandler)
    }
    unsafe fn dpad(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dpad)
    }
    unsafe fn buttonA(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonA)
    }
    unsafe fn buttonB(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonB)
    }
    unsafe fn buttonX(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonX)
    }
    unsafe fn buttonY(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonY)
    }
    unsafe fn buttonMenu(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonMenu)
    }
    unsafe fn buttonOptions(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonOptions)
    }
    unsafe fn buttonHome(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonHome)
    }
    unsafe fn leftThumbstick(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftThumbstick)
    }
    unsafe fn rightThumbstick(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightThumbstick)
    }
    unsafe fn leftShoulder(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftShoulder)
    }
    unsafe fn rightShoulder(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightShoulder)
    }
    unsafe fn leftTrigger(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftTrigger)
    }
    unsafe fn rightTrigger(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightTrigger)
    }
    unsafe fn leftThumbstickButton(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftThumbstickButton)
    }
    unsafe fn rightThumbstickButton(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightThumbstickButton)
    }
}
pub type GCExtendedGamepadValueChangedHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCExtendedGamepadSnapshot(pub id);
impl std::ops::Deref for GCExtendedGamepadSnapshot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCExtendedGamepadSnapshot {}
impl GCExtendedGamepadSnapshot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCExtendedGamepadSnapshot").unwrap(), alloc) })
    }
}
impl IGCExtendedGamepad for GCExtendedGamepadSnapshot {}
impl From<GCExtendedGamepadSnapshot> for GCExtendedGamepad {
    fn from(child: GCExtendedGamepadSnapshot) -> GCExtendedGamepad {
        GCExtendedGamepad(child.0)
    }
}
impl std::convert::TryFrom<GCExtendedGamepad> for GCExtendedGamepadSnapshot {
    type Error = &'static str;
    fn try_from(parent: GCExtendedGamepad) -> Result<GCExtendedGamepadSnapshot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCExtendedGamepadSnapshot").unwrap()) };
        if is_kind_of {
            Ok(GCExtendedGamepadSnapshot(parent.0))
        } else {
            Err("This GCExtendedGamepad cannot be downcasted to GCExtendedGamepadSnapshot")
        }
    }
}
impl IGCPhysicalInputProfile for GCExtendedGamepadSnapshot {}
impl INSObject for GCExtendedGamepadSnapshot {}
impl PNSObject for GCExtendedGamepadSnapshot {}
impl IGCExtendedGamepadSnapshot for GCExtendedGamepadSnapshot {}
pub trait IGCExtendedGamepadSnapshot: Sized + std::ops::Deref {
    unsafe fn initWithSnapshotData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSnapshotData : data)
    }
    unsafe fn initWithController_snapshotData_(
        &self,
        controller: GCController,
        data: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithController : controller, snapshotData : data)
    }
    unsafe fn snapshotData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotData)
    }
    unsafe fn setSnapshotData_(&self, snapshotData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSnapshotData : snapshotData)
    }
}
pub type GCExtendedGamepadSnapshotDataVersion = NSInteger;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct GCExtendedGamepadSnapshotData {
    pub version: u16,
    pub size: u16,
    pub dpadX: f32,
    pub dpadY: f32,
    pub buttonA: f32,
    pub buttonB: f32,
    pub buttonX: f32,
    pub buttonY: f32,
    pub leftShoulder: f32,
    pub rightShoulder: f32,
    pub leftThumbstickX: f32,
    pub leftThumbstickY: f32,
    pub rightThumbstickX: f32,
    pub rightThumbstickY: f32,
    pub leftTrigger: f32,
    pub rightTrigger: f32,
    pub supportsClickableThumbsticks: BOOL,
    pub leftThumbstickButton: BOOL,
    pub rightThumbstickButton: BOOL,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GCExtendedGamepadSnapShotDataV100 {
    pub version: u16,
    pub size: u16,
    pub dpadX: f32,
    pub dpadY: f32,
    pub buttonA: f32,
    pub buttonB: f32,
    pub buttonX: f32,
    pub buttonY: f32,
    pub leftShoulder: f32,
    pub rightShoulder: f32,
    pub leftThumbstickX: f32,
    pub leftThumbstickY: f32,
    pub rightThumbstickX: f32,
    pub rightThumbstickY: f32,
    pub leftTrigger: f32,
    pub rightTrigger: f32,
}
pub type GCKeyCode = CFIndex;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCKeyboardInput(pub id);
impl std::ops::Deref for GCKeyboardInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCKeyboardInput {}
impl GCKeyboardInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCKeyboardInput").unwrap(), alloc) })
    }
}
impl IGCPhysicalInputProfile for GCKeyboardInput {}
impl std::convert::TryFrom<GCPhysicalInputProfile> for GCKeyboardInput {
    type Error = &'static str;
    fn try_from(parent: GCPhysicalInputProfile) -> Result<GCKeyboardInput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCKeyboardInput").unwrap()) };
        if is_kind_of {
            Ok(GCKeyboardInput(parent.0))
        } else {
            Err("This GCPhysicalInputProfile cannot be downcasted to GCKeyboardInput")
        }
    }
}
impl INSObject for GCKeyboardInput {}
impl PNSObject for GCKeyboardInput {}
impl IGCKeyboardInput for GCKeyboardInput {}
pub trait IGCKeyboardInput: Sized + std::ops::Deref {
    unsafe fn buttonForKeyCode_(&self, code: GCKeyCode) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, buttonForKeyCode : code)
    }
    unsafe fn keyChangedHandler(&self) -> GCKeyboardValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyChangedHandler)
    }
    unsafe fn setKeyChangedHandler_(&self, keyChangedHandler: GCKeyboardValueChangedHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyChangedHandler : keyChangedHandler)
    }
    unsafe fn isAnyKeyPressed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnyKeyPressed)
    }
}
pub type GCKeyboardValueChangedHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCMouseInput(pub id);
impl std::ops::Deref for GCMouseInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCMouseInput {}
impl GCMouseInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCMouseInput").unwrap(), alloc) })
    }
}
impl IGCPhysicalInputProfile for GCMouseInput {}
impl std::convert::TryFrom<GCPhysicalInputProfile> for GCMouseInput {
    type Error = &'static str;
    fn try_from(parent: GCPhysicalInputProfile) -> Result<GCMouseInput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCMouseInput").unwrap()) };
        if is_kind_of {
            Ok(GCMouseInput(parent.0))
        } else {
            Err("This GCPhysicalInputProfile cannot be downcasted to GCMouseInput")
        }
    }
}
impl INSObject for GCMouseInput {}
impl PNSObject for GCMouseInput {}
impl IGCMouseInput for GCMouseInput {}
pub trait IGCMouseInput: Sized + std::ops::Deref {
    unsafe fn mouseMovedHandler(&self) -> GCMouseMoved
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mouseMovedHandler)
    }
    unsafe fn setMouseMovedHandler_(&self, mouseMovedHandler: GCMouseMoved)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMouseMovedHandler : mouseMovedHandler)
    }
    unsafe fn scroll(&self) -> GCDeviceCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scroll)
    }
    unsafe fn leftButton(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftButton)
    }
    unsafe fn rightButton(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightButton)
    }
    unsafe fn middleButton(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, middleButton)
    }
    unsafe fn auxiliaryButtons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, auxiliaryButtons)
    }
}
pub type GCMouseMoved = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCXboxGamepad(pub id);
impl std::ops::Deref for GCXboxGamepad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCXboxGamepad {}
impl GCXboxGamepad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCXboxGamepad").unwrap(), alloc) })
    }
}
impl IGCExtendedGamepad for GCXboxGamepad {}
impl std::convert::TryFrom<GCExtendedGamepad> for GCXboxGamepad {
    type Error = &'static str;
    fn try_from(parent: GCExtendedGamepad) -> Result<GCXboxGamepad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCXboxGamepad").unwrap()) };
        if is_kind_of {
            Ok(GCXboxGamepad(parent.0))
        } else {
            Err("This GCExtendedGamepad cannot be downcasted to GCXboxGamepad")
        }
    }
}
impl IGCPhysicalInputProfile for GCXboxGamepad {}
impl INSObject for GCXboxGamepad {}
impl PNSObject for GCXboxGamepad {}
impl IGCXboxGamepad for GCXboxGamepad {}
pub trait IGCXboxGamepad: Sized + std::ops::Deref {
    unsafe fn paddleButton1(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddleButton1)
    }
    unsafe fn paddleButton2(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddleButton2)
    }
    unsafe fn paddleButton3(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddleButton3)
    }
    unsafe fn paddleButton4(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddleButton4)
    }
    unsafe fn buttonShare(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonShare)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCDualShockGamepad(pub id);
impl std::ops::Deref for GCDualShockGamepad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCDualShockGamepad {}
impl GCDualShockGamepad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCDualShockGamepad").unwrap(), alloc) })
    }
}
impl IGCExtendedGamepad for GCDualShockGamepad {}
impl std::convert::TryFrom<GCExtendedGamepad> for GCDualShockGamepad {
    type Error = &'static str;
    fn try_from(parent: GCExtendedGamepad) -> Result<GCDualShockGamepad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCDualShockGamepad").unwrap()) };
        if is_kind_of {
            Ok(GCDualShockGamepad(parent.0))
        } else {
            Err("This GCExtendedGamepad cannot be downcasted to GCDualShockGamepad")
        }
    }
}
impl IGCPhysicalInputProfile for GCDualShockGamepad {}
impl INSObject for GCDualShockGamepad {}
impl PNSObject for GCDualShockGamepad {}
impl IGCDualShockGamepad for GCDualShockGamepad {}
pub trait IGCDualShockGamepad: Sized + std::ops::Deref {
    unsafe fn touchpadButton(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchpadButton)
    }
    unsafe fn touchpadPrimary(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchpadPrimary)
    }
    unsafe fn touchpadSecondary(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchpadSecondary)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCDualSenseGamepad(pub id);
impl std::ops::Deref for GCDualSenseGamepad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCDualSenseGamepad {}
impl GCDualSenseGamepad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCDualSenseGamepad").unwrap(), alloc) })
    }
}
impl IGCExtendedGamepad for GCDualSenseGamepad {}
impl std::convert::TryFrom<GCExtendedGamepad> for GCDualSenseGamepad {
    type Error = &'static str;
    fn try_from(parent: GCExtendedGamepad) -> Result<GCDualSenseGamepad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCDualSenseGamepad").unwrap()) };
        if is_kind_of {
            Ok(GCDualSenseGamepad(parent.0))
        } else {
            Err("This GCExtendedGamepad cannot be downcasted to GCDualSenseGamepad")
        }
    }
}
impl IGCPhysicalInputProfile for GCDualSenseGamepad {}
impl INSObject for GCDualSenseGamepad {}
impl PNSObject for GCDualSenseGamepad {}
impl IGCDualSenseGamepad for GCDualSenseGamepad {}
pub trait IGCDualSenseGamepad: Sized + std::ops::Deref {
    unsafe fn touchpadButton(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchpadButton)
    }
    unsafe fn touchpadPrimary(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchpadPrimary)
    }
    unsafe fn touchpadSecondary(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchpadSecondary)
    }
    unsafe fn leftTrigger(&self) -> GCDualSenseAdaptiveTrigger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftTrigger)
    }
    unsafe fn rightTrigger(&self) -> GCDualSenseAdaptiveTrigger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightTrigger)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCMicroGamepad(pub id);
impl std::ops::Deref for GCMicroGamepad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCMicroGamepad {}
impl GCMicroGamepad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCMicroGamepad").unwrap(), alloc) })
    }
}
impl IGCPhysicalInputProfile for GCMicroGamepad {}
impl std::convert::TryFrom<GCPhysicalInputProfile> for GCMicroGamepad {
    type Error = &'static str;
    fn try_from(parent: GCPhysicalInputProfile) -> Result<GCMicroGamepad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCMicroGamepad").unwrap()) };
        if is_kind_of {
            Ok(GCMicroGamepad(parent.0))
        } else {
            Err("This GCPhysicalInputProfile cannot be downcasted to GCMicroGamepad")
        }
    }
}
impl INSObject for GCMicroGamepad {}
impl PNSObject for GCMicroGamepad {}
impl IGCMicroGamepad for GCMicroGamepad {}
pub trait IGCMicroGamepad: Sized + std::ops::Deref {
    unsafe fn saveSnapshot(&self) -> GCMicroGamepadSnapshot
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saveSnapshot)
    }
    unsafe fn setStateFromMicroGamepad_(&self, microGamepad: GCMicroGamepad)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStateFromMicroGamepad : microGamepad)
    }
    unsafe fn controller(&self) -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controller)
    }
    unsafe fn valueChangedHandler(&self) -> GCMicroGamepadValueChangedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueChangedHandler)
    }
    unsafe fn setValueChangedHandler_(&self, valueChangedHandler: GCMicroGamepadValueChangedHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueChangedHandler : valueChangedHandler)
    }
    unsafe fn dpad(&self) -> GCControllerDirectionPad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dpad)
    }
    unsafe fn buttonA(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonA)
    }
    unsafe fn buttonX(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonX)
    }
    unsafe fn buttonMenu(&self) -> GCControllerButtonInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonMenu)
    }
    unsafe fn reportsAbsoluteDpadValues(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportsAbsoluteDpadValues)
    }
    unsafe fn setReportsAbsoluteDpadValues_(&self, reportsAbsoluteDpadValues: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportsAbsoluteDpadValues : reportsAbsoluteDpadValues)
    }
    unsafe fn allowsRotation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsRotation)
    }
    unsafe fn setAllowsRotation_(&self, allowsRotation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsRotation : allowsRotation)
    }
}
pub type GCMicroGamepadValueChangedHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCMicroGamepadSnapshot(pub id);
impl std::ops::Deref for GCMicroGamepadSnapshot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCMicroGamepadSnapshot {}
impl GCMicroGamepadSnapshot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCMicroGamepadSnapshot").unwrap(), alloc) })
    }
}
impl IGCMicroGamepad for GCMicroGamepadSnapshot {}
impl From<GCMicroGamepadSnapshot> for GCMicroGamepad {
    fn from(child: GCMicroGamepadSnapshot) -> GCMicroGamepad {
        GCMicroGamepad(child.0)
    }
}
impl std::convert::TryFrom<GCMicroGamepad> for GCMicroGamepadSnapshot {
    type Error = &'static str;
    fn try_from(parent: GCMicroGamepad) -> Result<GCMicroGamepadSnapshot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCMicroGamepadSnapshot").unwrap()) };
        if is_kind_of {
            Ok(GCMicroGamepadSnapshot(parent.0))
        } else {
            Err("This GCMicroGamepad cannot be downcasted to GCMicroGamepadSnapshot")
        }
    }
}
impl IGCPhysicalInputProfile for GCMicroGamepadSnapshot {}
impl INSObject for GCMicroGamepadSnapshot {}
impl PNSObject for GCMicroGamepadSnapshot {}
impl IGCMicroGamepadSnapshot for GCMicroGamepadSnapshot {}
pub trait IGCMicroGamepadSnapshot: Sized + std::ops::Deref {
    unsafe fn initWithSnapshotData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSnapshotData : data)
    }
    unsafe fn initWithController_snapshotData_(
        &self,
        controller: GCController,
        data: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithController : controller, snapshotData : data)
    }
    unsafe fn snapshotData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotData)
    }
    unsafe fn setSnapshotData_(&self, snapshotData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSnapshotData : snapshotData)
    }
}
pub type GCMicroGamepadSnapshotDataVersion = NSInteger;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct GCMicroGamepadSnapshotData {
    pub version: u16,
    pub size: u16,
    pub dpadX: f32,
    pub dpadY: f32,
    pub buttonA: f32,
    pub buttonX: f32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct GCMicroGamepadSnapShotDataV100 {
    pub version: u16,
    pub size: u16,
    pub dpadX: f32,
    pub dpadY: f32,
    pub buttonA: f32,
    pub buttonX: f32,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCDirectionalGamepad(pub id);
impl std::ops::Deref for GCDirectionalGamepad {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCDirectionalGamepad {}
impl GCDirectionalGamepad {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCDirectionalGamepad").unwrap(), alloc) })
    }
}
impl IGCMicroGamepad for GCDirectionalGamepad {}
impl std::convert::TryFrom<GCMicroGamepad> for GCDirectionalGamepad {
    type Error = &'static str;
    fn try_from(parent: GCMicroGamepad) -> Result<GCDirectionalGamepad, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCDirectionalGamepad").unwrap()) };
        if is_kind_of {
            Ok(GCDirectionalGamepad(parent.0))
        } else {
            Err("This GCMicroGamepad cannot be downcasted to GCDirectionalGamepad")
        }
    }
}
impl IGCPhysicalInputProfile for GCDirectionalGamepad {}
impl INSObject for GCDirectionalGamepad {}
impl PNSObject for GCDirectionalGamepad {}
impl IGCDirectionalGamepad for GCDirectionalGamepad {}
pub trait IGCDirectionalGamepad: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCControllerInputState(pub id);
impl std::ops::Deref for GCControllerInputState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCControllerInputState {}
impl GCControllerInputState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCControllerInputState").unwrap(), alloc) })
    }
}
impl PGCDevicePhysicalInputState for GCControllerInputState {}
impl INSObject for GCControllerInputState {}
impl PNSObject for GCControllerInputState {}
impl std::convert::TryFrom<NSObject> for GCControllerInputState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCControllerInputState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCControllerInputState").unwrap()) };
        if is_kind_of {
            Ok(GCControllerInputState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCControllerInputState")
        }
    }
}
impl IGCControllerInputState for GCControllerInputState {}
pub trait IGCControllerInputState: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCControllerLiveInput(pub id);
impl std::ops::Deref for GCControllerLiveInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCControllerLiveInput {}
impl GCControllerLiveInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCControllerLiveInput").unwrap(), alloc) })
    }
}
impl PGCDevicePhysicalInput for GCControllerLiveInput {}
impl IGCControllerInputState for GCControllerLiveInput {}
impl PGCDevicePhysicalInputState for GCControllerLiveInput {}
impl From<GCControllerLiveInput> for GCControllerInputState {
    fn from(child: GCControllerLiveInput) -> GCControllerInputState {
        GCControllerInputState(child.0)
    }
}
impl std::convert::TryFrom<GCControllerInputState> for GCControllerLiveInput {
    type Error = &'static str;
    fn try_from(parent: GCControllerInputState) -> Result<GCControllerLiveInput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCControllerLiveInput").unwrap()) };
        if is_kind_of {
            Ok(GCControllerLiveInput(parent.0))
        } else {
            Err("This GCControllerInputState cannot be downcasted to GCControllerLiveInput")
        }
    }
}
impl INSObject for GCControllerLiveInput {}
impl PNSObject for GCControllerLiveInput {}
impl IGCControllerLiveInput for GCControllerLiveInput {}
pub trait IGCControllerLiveInput: Sized + std::ops::Deref {
    unsafe fn capture(&self) -> GCControllerInputState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capture)
    }
    unsafe fn nextInputState(&self) -> GCControllerInputState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextInputState)
    }
    unsafe fn unmappedInput(&self) -> GCControllerLiveInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unmappedInput)
    }
}
pub type GCControllerPlayerIndex = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCController(pub id);
impl std::ops::Deref for GCController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCController {}
impl GCController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), alloc) })
    }
}
impl PGCDevice for GCController {}
impl INSObject for GCController {}
impl PNSObject for GCController {}
impl std::convert::TryFrom<NSObject> for GCController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCController").unwrap()) };
        if is_kind_of {
            Ok(GCController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCController")
        }
    }
}
impl IGCController for GCController {}
pub trait IGCController: Sized + std::ops::Deref {
    unsafe fn controllerPausedHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerPausedHandler)
    }
    unsafe fn setControllerPausedHandler_(
        &self,
        controllerPausedHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControllerPausedHandler : controllerPausedHandler)
    }
    unsafe fn isAttachedToDevice(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAttachedToDevice)
    }
    unsafe fn playerIndex(&self) -> GCControllerPlayerIndex
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerIndex)
    }
    unsafe fn setPlayerIndex_(&self, playerIndex: GCControllerPlayerIndex)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayerIndex : playerIndex)
    }
    unsafe fn input(&self) -> GCControllerLiveInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, input)
    }
    unsafe fn battery(&self) -> GCDeviceBattery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, battery)
    }
    unsafe fn physicalInputProfile(&self) -> GCPhysicalInputProfile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicalInputProfile)
    }
    unsafe fn gamepad(&self) -> GCGamepad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gamepad)
    }
    unsafe fn microGamepad(&self) -> GCMicroGamepad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, microGamepad)
    }
    unsafe fn extendedGamepad(&self) -> GCExtendedGamepad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extendedGamepad)
    }
    unsafe fn motion(&self) -> GCMotion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motion)
    }
    unsafe fn light(&self) -> GCDeviceLight
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, light)
    }
    unsafe fn haptics(&self) -> GCDeviceHaptics
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, haptics)
    }
    unsafe fn controllers() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), controllers)
    }
    unsafe fn supportsHIDDevice_(device: IOHIDDeviceRef) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), supportsHIDDevice : device)
    }
    unsafe fn current() -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), current)
    }
    unsafe fn shouldMonitorBackgroundEvents() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), shouldMonitorBackgroundEvents)
    }
    unsafe fn setShouldMonitorBackgroundEvents_(shouldMonitorBackgroundEvents: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), setShouldMonitorBackgroundEvents : shouldMonitorBackgroundEvents)
    }
}
impl GCController_Snapshot for GCController {}
pub trait GCController_Snapshot: Sized + std::ops::Deref {
    unsafe fn capture(&self) -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capture)
    }
    unsafe fn isSnapshot(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSnapshot)
    }
    unsafe fn controllerWithMicroGamepad() -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), controllerWithMicroGamepad)
    }
    unsafe fn controllerWithExtendedGamepad() -> GCController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), controllerWithExtendedGamepad)
    }
}
impl GCController_Discovery for GCController {}
pub trait GCController_Discovery: Sized + std::ops::Deref {
    unsafe fn startWirelessControllerDiscoveryWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), startWirelessControllerDiscoveryWithCompletionHandler : completionHandler)
    }
    unsafe fn stopWirelessControllerDiscovery()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCController").unwrap(), stopWirelessControllerDiscovery)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCStylus(pub id);
impl std::ops::Deref for GCStylus {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCStylus {}
impl GCStylus {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCStylus").unwrap(), alloc) })
    }
}
impl PGCDevice for GCStylus {}
impl INSObject for GCStylus {}
impl PNSObject for GCStylus {}
impl std::convert::TryFrom<NSObject> for GCStylus {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCStylus, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCStylus").unwrap()) };
        if is_kind_of {
            Ok(GCStylus(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCStylus")
        }
    }
}
impl IGCStylus for GCStylus {}
pub trait IGCStylus: Sized + std::ops::Deref {
    unsafe fn input(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, input)
    }
    unsafe fn haptics(&self) -> GCDeviceHaptics
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, haptics)
    }
}
impl GCStylus_Discovery for GCStylus {}
pub trait GCStylus_Discovery: Sized + std::ops::Deref {
    unsafe fn styli() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCStylus").unwrap(), styli)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCKeyboard(pub id);
impl std::ops::Deref for GCKeyboard {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCKeyboard {}
impl GCKeyboard {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCKeyboard").unwrap(), alloc) })
    }
}
impl PGCDevice for GCKeyboard {}
impl INSObject for GCKeyboard {}
impl PNSObject for GCKeyboard {}
impl std::convert::TryFrom<NSObject> for GCKeyboard {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCKeyboard, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCKeyboard").unwrap()) };
        if is_kind_of {
            Ok(GCKeyboard(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCKeyboard")
        }
    }
}
impl IGCKeyboard for GCKeyboard {}
pub trait IGCKeyboard: Sized + std::ops::Deref {
    unsafe fn keyboardInput(&self) -> GCKeyboardInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyboardInput)
    }
    unsafe fn coalescedKeyboard() -> GCKeyboard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCKeyboard").unwrap(), coalescedKeyboard)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCMouse(pub id);
impl std::ops::Deref for GCMouse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCMouse {}
impl GCMouse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCMouse").unwrap(), alloc) })
    }
}
impl PGCDevice for GCMouse {}
impl INSObject for GCMouse {}
impl PNSObject for GCMouse {}
impl std::convert::TryFrom<NSObject> for GCMouse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCMouse, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCMouse").unwrap()) };
        if is_kind_of {
            Ok(GCMouse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCMouse")
        }
    }
}
impl IGCMouse for GCMouse {}
pub trait IGCMouse: Sized + std::ops::Deref {
    unsafe fn mouseInput(&self) -> GCMouseInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mouseInput)
    }
    unsafe fn mice() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCMouse").unwrap(), mice)
    }
    unsafe fn current() -> GCMouse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCMouse").unwrap(), current)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCRacingWheel(pub id);
impl std::ops::Deref for GCRacingWheel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCRacingWheel {}
impl GCRacingWheel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCRacingWheel").unwrap(), alloc) })
    }
}
impl PGCDevice for GCRacingWheel {}
impl INSObject for GCRacingWheel {}
impl PNSObject for GCRacingWheel {}
impl std::convert::TryFrom<NSObject> for GCRacingWheel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCRacingWheel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCRacingWheel").unwrap()) };
        if is_kind_of {
            Ok(GCRacingWheel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCRacingWheel")
        }
    }
}
impl IGCRacingWheel for GCRacingWheel {}
pub trait IGCRacingWheel: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn acquireDeviceWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acquireDeviceWithError : error)
    }
    unsafe fn relinquishDevice(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relinquishDevice)
    }
    unsafe fn capture(&self) -> GCRacingWheel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capture)
    }
    unsafe fn isAcquired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAcquired)
    }
    unsafe fn wheelInput(&self) -> GCRacingWheelInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wheelInput)
    }
    unsafe fn isSnapshot(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSnapshot)
    }
    unsafe fn connectedRacingWheels() -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GCRacingWheel").unwrap(), connectedRacingWheels)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCRacingWheelInputState(pub id);
impl std::ops::Deref for GCRacingWheelInputState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCRacingWheelInputState {}
impl GCRacingWheelInputState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCRacingWheelInputState").unwrap(), alloc) })
    }
}
impl PGCDevicePhysicalInputState for GCRacingWheelInputState {}
impl INSObject for GCRacingWheelInputState {}
impl PNSObject for GCRacingWheelInputState {}
impl std::convert::TryFrom<NSObject> for GCRacingWheelInputState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCRacingWheelInputState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCRacingWheelInputState").unwrap()) };
        if is_kind_of {
            Ok(GCRacingWheelInputState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCRacingWheelInputState")
        }
    }
}
impl IGCRacingWheelInputState for GCRacingWheelInputState {}
pub trait IGCRacingWheelInputState: Sized + std::ops::Deref {
    unsafe fn wheel(&self) -> GCSteeringWheelElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wheel)
    }
    unsafe fn acceleratorPedal(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceleratorPedal)
    }
    unsafe fn brakePedal(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, brakePedal)
    }
    unsafe fn clutchPedal(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clutchPedal)
    }
    unsafe fn shifter(&self) -> GCGearShifterElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shifter)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCRacingWheelInput(pub id);
impl std::ops::Deref for GCRacingWheelInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCRacingWheelInput {}
impl GCRacingWheelInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCRacingWheelInput").unwrap(), alloc) })
    }
}
impl PGCDevicePhysicalInput for GCRacingWheelInput {}
impl IGCRacingWheelInputState for GCRacingWheelInput {}
impl PGCDevicePhysicalInputState for GCRacingWheelInput {}
impl From<GCRacingWheelInput> for GCRacingWheelInputState {
    fn from(child: GCRacingWheelInput) -> GCRacingWheelInputState {
        GCRacingWheelInputState(child.0)
    }
}
impl std::convert::TryFrom<GCRacingWheelInputState> for GCRacingWheelInput {
    type Error = &'static str;
    fn try_from(parent: GCRacingWheelInputState) -> Result<GCRacingWheelInput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCRacingWheelInput").unwrap()) };
        if is_kind_of {
            Ok(GCRacingWheelInput(parent.0))
        } else {
            Err("This GCRacingWheelInputState cannot be downcasted to GCRacingWheelInput")
        }
    }
}
impl INSObject for GCRacingWheelInput {}
impl PNSObject for GCRacingWheelInput {}
impl IGCRacingWheelInput for GCRacingWheelInput {}
pub trait IGCRacingWheelInput: Sized + std::ops::Deref {
    unsafe fn capture(&self) -> GCRacingWheelInputState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capture)
    }
    unsafe fn nextInputState(&self) -> GCRacingWheelInputState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextInputState)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCSteeringWheelElement(pub id);
impl std::ops::Deref for GCSteeringWheelElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCSteeringWheelElement {}
impl GCSteeringWheelElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCSteeringWheelElement").unwrap(), alloc) })
    }
}
impl PGCAxisElement for GCSteeringWheelElement {}
impl INSObject for GCSteeringWheelElement {}
impl PNSObject for GCSteeringWheelElement {}
impl std::convert::TryFrom<NSObject> for GCSteeringWheelElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCSteeringWheelElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCSteeringWheelElement").unwrap()) };
        if is_kind_of {
            Ok(GCSteeringWheelElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCSteeringWheelElement")
        }
    }
}
impl IGCSteeringWheelElement for GCSteeringWheelElement {}
pub trait IGCSteeringWheelElement: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn maximumDegreesOfRotation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDegreesOfRotation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCGearShifterElement(pub id);
impl std::ops::Deref for GCGearShifterElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCGearShifterElement {}
impl GCGearShifterElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCGearShifterElement").unwrap(), alloc) })
    }
}
impl PGCPhysicalInputElement for GCGearShifterElement {}
impl INSObject for GCGearShifterElement {}
impl PNSObject for GCGearShifterElement {}
impl std::convert::TryFrom<NSObject> for GCGearShifterElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCGearShifterElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCGearShifterElement").unwrap()) };
        if is_kind_of {
            Ok(GCGearShifterElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCGearShifterElement")
        }
    }
}
impl IGCGearShifterElement for GCGearShifterElement {}
pub trait IGCGearShifterElement: Sized + std::ops::Deref {
    unsafe fn patternInput(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, patternInput)
    }
    unsafe fn sequentialInput(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sequentialInput)
    }
}
pub type GCHapticsLocality = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCDeviceHaptics(pub id);
impl std::ops::Deref for GCDeviceHaptics {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCDeviceHaptics {}
impl GCDeviceHaptics {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCDeviceHaptics").unwrap(), alloc) })
    }
}
impl INSObject for GCDeviceHaptics {}
impl PNSObject for GCDeviceHaptics {}
impl std::convert::TryFrom<NSObject> for GCDeviceHaptics {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCDeviceHaptics, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCDeviceHaptics").unwrap()) };
        if is_kind_of {
            Ok(GCDeviceHaptics(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCDeviceHaptics")
        }
    }
}
impl IGCDeviceHaptics for GCDeviceHaptics {}
pub trait IGCDeviceHaptics: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn createEngineWithLocality_(&self, locality: NSString) -> CHHapticEngine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createEngineWithLocality : locality)
    }
    unsafe fn supportedLocalities(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedLocalities)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GCEventViewController(pub id);
impl std::ops::Deref for GCEventViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GCEventViewController {}
impl GCEventViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GCEventViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for GCEventViewController {}
impl INSObject for GCEventViewController {}
impl PNSObject for GCEventViewController {}
impl std::convert::TryFrom<NSObject> for GCEventViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GCEventViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GCEventViewController").unwrap()) };
        if is_kind_of {
            Ok(GCEventViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GCEventViewController")
        }
    }
}
impl IGCEventViewController for GCEventViewController {}
pub trait IGCEventViewController: Sized + std::ops::Deref {
    unsafe fn controllerUserInteractionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerUserInteractionEnabled)
    }
    unsafe fn setControllerUserInteractionEnabled_(&self, controllerUserInteractionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControllerUserInteractionEnabled : controllerUserInteractionEnabled)
    }
}
pub trait PNSEditor: Sized + std::ops::Deref {
    unsafe fn discardEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardEditing)
    }
    unsafe fn commitEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commitEditing)
    }
    unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo_(
        &self,
        delegate: id,
        didCommitSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingWithDelegate : delegate, didCommitSelector : didCommitSelector, contextInfo : contextInfo)
    }
    unsafe fn commitEditingAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingAndReturnError : error)
    }
}
impl PNSEditor for GCEventViewController {}
unsafe extern "C" {
    pub static GCPoint2Zero: GCPoint2;
}
unsafe extern "C" {
    pub fn NSStringFromGCPoint2(point: GCPoint2) -> NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryDualSense: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryDualShock4: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryMFi: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryXboxOne: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryHID: NSString;
}
unsafe extern "C" {
    pub static GCProductCategorySpatialController: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryArcadeStick: NSString;
}
unsafe extern "C" {
    pub static GCProductCategorySiriRemote1stGen: NSString;
}
unsafe extern "C" {
    pub static GCProductCategorySiriRemote2ndGen: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryControlCenterRemote: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryUniversalElectronicsRemote: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryCoalescedRemote: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryMouse: NSString;
}
unsafe extern "C" {
    pub static GCProductCategoryKeyboard: NSString;
}
unsafe extern "C" {
    pub static GCProductCategorySpatialStylus: NSString;
}
unsafe extern "C" {
    pub static mut GCInputButtonA: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputButtonB: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputButtonX: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputButtonY: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputDirectionPad: GCInputDirectionPadName;
}
unsafe extern "C" {
    pub static mut GCInputThumbstick: GCInputDirectionPadName;
}
unsafe extern "C" {
    pub static mut GCInputLeftThumbstick: GCInputDirectionPadName;
}
unsafe extern "C" {
    pub static mut GCInputRightThumbstick: GCInputDirectionPadName;
}
unsafe extern "C" {
    pub static mut GCInputThumbstickButton: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputLeftThumbstickButton: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputRightThumbstickButton: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputGripButton: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputLeftShoulder: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputRightShoulder: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputLeftBumper: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputRightBumper: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputTrigger: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputLeftTrigger: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputRightTrigger: GCInputButtonName;
}
unsafe extern "C" {
    pub fn GCInputBackLeftButton(position: NSInteger) -> GCInputButtonName;
}
unsafe extern "C" {
    pub fn GCInputBackRightButton(position: NSInteger) -> GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputButtonHome: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputButtonMenu: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputButtonOptions: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputButtonShare: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputXboxPaddleOne: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputXboxPaddleTwo: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputXboxPaddleThree: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputXboxPaddleFour: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputDualShockTouchpadOne: GCInputDirectionPadName;
}
unsafe extern "C" {
    pub static mut GCInputDualShockTouchpadTwo: GCInputDirectionPadName;
}
unsafe extern "C" {
    pub static mut GCInputDualShockTouchpadButton: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputSteeringWheel: GCInputAxisName;
}
unsafe extern "C" {
    pub static mut GCInputShifter: GCInputElementName;
}
unsafe extern "C" {
    pub static mut GCInputPedalAccelerator: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputPedalBrake: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputPedalClutch: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputLeftPaddle: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputRightPaddle: GCInputButtonName;
}
unsafe extern "C" {
    pub fn GCInputArcadeButtonName(row: NSInteger, column: NSInteger) -> GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputStylusTip: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputStylusPrimaryButton: GCInputButtonName;
}
unsafe extern "C" {
    pub static mut GCInputStylusSecondaryButton: GCInputButtonName;
}
unsafe extern "C" {
    pub fn GCGamepadSnapShotDataV100FromNSData(
        snapshotData: *mut GCGamepadSnapShotDataV100,
        data: NSData,
    ) -> BOOL;
}
unsafe extern "C" {
    pub fn NSDataFromGCGamepadSnapShotDataV100(
        snapshotData: *mut GCGamepadSnapShotDataV100,
    ) -> NSData;
}
unsafe extern "C" {
    pub static GCCurrentExtendedGamepadSnapshotDataVersion: GCExtendedGamepadSnapshotDataVersion;
}
unsafe extern "C" {
    pub fn GCExtendedGamepadSnapshotDataFromNSData(
        snapshotData: *mut GCExtendedGamepadSnapshotData,
        data: NSData,
    ) -> BOOL;
}
unsafe extern "C" {
    pub fn NSDataFromGCExtendedGamepadSnapshotData(
        snapshotData: *mut GCExtendedGamepadSnapshotData,
    ) -> NSData;
}
unsafe extern "C" {
    pub fn GCExtendedGamepadSnapShotDataV100FromNSData(
        snapshotData: *mut GCExtendedGamepadSnapShotDataV100,
        data: NSData,
    ) -> BOOL;
}
unsafe extern "C" {
    pub fn NSDataFromGCExtendedGamepadSnapShotDataV100(
        snapshotData: *mut GCExtendedGamepadSnapShotDataV100,
    ) -> NSData;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyA: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyB: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyC: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyD: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyE: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyF: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyG: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyH: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyI: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyJ: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyK: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyL: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyM: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyN: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyO: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyP: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyQ: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyR: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyS: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyT: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyU: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyV: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyW: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyX: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyY: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeyZ: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeOne: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeTwo: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeThree: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeFour: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeFive: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeSix: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeSeven: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeEight: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeNine: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeZero: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeReturnOrEnter: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeEscape: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeDeleteOrBackspace: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeTab: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeSpacebar: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeHyphen: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeEqualSign: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeOpenBracket: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeCloseBracket: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeBackslash: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeNonUSPound: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeSemicolon: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeQuote: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeGraveAccentAndTilde: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeComma: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodePeriod: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeSlash: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeCapsLock: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF1: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF2: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF3: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF4: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF5: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF6: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF7: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF8: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF9: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF10: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF11: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF12: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF13: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF14: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF15: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF16: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF17: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF18: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF19: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeF20: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodePrintScreen: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeScrollLock: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodePause: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInsert: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeHome: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodePageUp: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeDeleteForward: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeEnd: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodePageDown: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeRightArrow: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLeftArrow: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeDownArrow: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeUpArrow: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypadNumLock: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypadSlash: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypadAsterisk: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypadHyphen: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypadPlus: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypadEnter: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad1: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad2: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad3: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad4: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad5: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad6: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad7: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad8: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad9: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypad0: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypadPeriod: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeKeypadEqualSign: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeNonUSBackslash: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeApplication: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodePower: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational1: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational2: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational3: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational4: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational5: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational6: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational7: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational8: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeInternational9: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG1: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG2: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG3: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG4: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG5: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG6: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG7: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG8: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLANG9: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLeftControl: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLeftShift: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLeftAlt: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeLeftGUI: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeRightControl: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeRightShift: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeRightAlt: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyCodeRightGUI: GCKeyCode;
}
unsafe extern "C" {
    pub static GCKeyA: NSString;
}
unsafe extern "C" {
    pub static GCKeyB: NSString;
}
unsafe extern "C" {
    pub static GCKeyC: NSString;
}
unsafe extern "C" {
    pub static GCKeyD: NSString;
}
unsafe extern "C" {
    pub static GCKeyE: NSString;
}
unsafe extern "C" {
    pub static GCKeyF: NSString;
}
unsafe extern "C" {
    pub static GCKeyG: NSString;
}
unsafe extern "C" {
    pub static GCKeyH: NSString;
}
unsafe extern "C" {
    pub static GCKeyI: NSString;
}
unsafe extern "C" {
    pub static GCKeyJ: NSString;
}
unsafe extern "C" {
    pub static GCKeyK: NSString;
}
unsafe extern "C" {
    pub static GCKeyL: NSString;
}
unsafe extern "C" {
    pub static GCKeyM: NSString;
}
unsafe extern "C" {
    pub static GCKeyN: NSString;
}
unsafe extern "C" {
    pub static GCKeyO: NSString;
}
unsafe extern "C" {
    pub static GCKeyP: NSString;
}
unsafe extern "C" {
    pub static GCKeyQ: NSString;
}
unsafe extern "C" {
    pub static GCKeyR: NSString;
}
unsafe extern "C" {
    pub static GCKeyS: NSString;
}
unsafe extern "C" {
    pub static GCKeyT: NSString;
}
unsafe extern "C" {
    pub static GCKeyU: NSString;
}
unsafe extern "C" {
    pub static GCKeyV: NSString;
}
unsafe extern "C" {
    pub static GCKeyW: NSString;
}
unsafe extern "C" {
    pub static GCKeyX: NSString;
}
unsafe extern "C" {
    pub static GCKeyY: NSString;
}
unsafe extern "C" {
    pub static GCKeyZ: NSString;
}
unsafe extern "C" {
    pub static GCKeyOne: NSString;
}
unsafe extern "C" {
    pub static GCKeyTwo: NSString;
}
unsafe extern "C" {
    pub static GCKeyThree: NSString;
}
unsafe extern "C" {
    pub static GCKeyFour: NSString;
}
unsafe extern "C" {
    pub static GCKeyFive: NSString;
}
unsafe extern "C" {
    pub static GCKeySix: NSString;
}
unsafe extern "C" {
    pub static GCKeySeven: NSString;
}
unsafe extern "C" {
    pub static GCKeyEight: NSString;
}
unsafe extern "C" {
    pub static GCKeyNine: NSString;
}
unsafe extern "C" {
    pub static GCKeyZero: NSString;
}
unsafe extern "C" {
    pub static GCKeyReturnOrEnter: NSString;
}
unsafe extern "C" {
    pub static GCKeyEscape: NSString;
}
unsafe extern "C" {
    pub static GCKeyDeleteOrBackspace: NSString;
}
unsafe extern "C" {
    pub static GCKeyTab: NSString;
}
unsafe extern "C" {
    pub static GCKeySpacebar: NSString;
}
unsafe extern "C" {
    pub static GCKeyHyphen: NSString;
}
unsafe extern "C" {
    pub static GCKeyEqualSign: NSString;
}
unsafe extern "C" {
    pub static GCKeyOpenBracket: NSString;
}
unsafe extern "C" {
    pub static GCKeyCloseBracket: NSString;
}
unsafe extern "C" {
    pub static GCKeyBackslash: NSString;
}
unsafe extern "C" {
    pub static GCKeyNonUSPound: NSString;
}
unsafe extern "C" {
    pub static GCKeySemicolon: NSString;
}
unsafe extern "C" {
    pub static GCKeyQuote: NSString;
}
unsafe extern "C" {
    pub static GCKeyGraveAccentAndTilde: NSString;
}
unsafe extern "C" {
    pub static GCKeyComma: NSString;
}
unsafe extern "C" {
    pub static GCKeyPeriod: NSString;
}
unsafe extern "C" {
    pub static GCKeySlash: NSString;
}
unsafe extern "C" {
    pub static GCKeyCapsLock: NSString;
}
unsafe extern "C" {
    pub static GCKeyF1: NSString;
}
unsafe extern "C" {
    pub static GCKeyF2: NSString;
}
unsafe extern "C" {
    pub static GCKeyF3: NSString;
}
unsafe extern "C" {
    pub static GCKeyF4: NSString;
}
unsafe extern "C" {
    pub static GCKeyF5: NSString;
}
unsafe extern "C" {
    pub static GCKeyF6: NSString;
}
unsafe extern "C" {
    pub static GCKeyF7: NSString;
}
unsafe extern "C" {
    pub static GCKeyF8: NSString;
}
unsafe extern "C" {
    pub static GCKeyF9: NSString;
}
unsafe extern "C" {
    pub static GCKeyF10: NSString;
}
unsafe extern "C" {
    pub static GCKeyF11: NSString;
}
unsafe extern "C" {
    pub static GCKeyF12: NSString;
}
unsafe extern "C" {
    pub static GCKeyF13: NSString;
}
unsafe extern "C" {
    pub static GCKeyF14: NSString;
}
unsafe extern "C" {
    pub static GCKeyF15: NSString;
}
unsafe extern "C" {
    pub static GCKeyF16: NSString;
}
unsafe extern "C" {
    pub static GCKeyF17: NSString;
}
unsafe extern "C" {
    pub static GCKeyF18: NSString;
}
unsafe extern "C" {
    pub static GCKeyF19: NSString;
}
unsafe extern "C" {
    pub static GCKeyF20: NSString;
}
unsafe extern "C" {
    pub static GCKeyPrintScreen: NSString;
}
unsafe extern "C" {
    pub static GCKeyScrollLock: NSString;
}
unsafe extern "C" {
    pub static GCKeyPause: NSString;
}
unsafe extern "C" {
    pub static GCKeyInsert: NSString;
}
unsafe extern "C" {
    pub static GCKeyHome: NSString;
}
unsafe extern "C" {
    pub static GCKeyPageUp: NSString;
}
unsafe extern "C" {
    pub static GCKeyDeleteForward: NSString;
}
unsafe extern "C" {
    pub static GCKeyEnd: NSString;
}
unsafe extern "C" {
    pub static GCKeyPageDown: NSString;
}
unsafe extern "C" {
    pub static GCKeyRightArrow: NSString;
}
unsafe extern "C" {
    pub static GCKeyLeftArrow: NSString;
}
unsafe extern "C" {
    pub static GCKeyDownArrow: NSString;
}
unsafe extern "C" {
    pub static GCKeyUpArrow: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypadNumLock: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypadSlash: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypadAsterisk: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypadHyphen: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypadPlus: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypadEnter: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad1: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad2: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad3: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad4: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad5: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad6: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad7: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad8: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad9: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypad0: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypadPeriod: NSString;
}
unsafe extern "C" {
    pub static GCKeyKeypadEqualSign: NSString;
}
unsafe extern "C" {
    pub static GCKeyNonUSBackslash: NSString;
}
unsafe extern "C" {
    pub static GCKeyApplication: NSString;
}
unsafe extern "C" {
    pub static GCKeyPower: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational1: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational2: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational3: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational4: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational5: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational6: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational7: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational8: NSString;
}
unsafe extern "C" {
    pub static GCKeyInternational9: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG1: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG2: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG3: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG4: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG5: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG6: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG7: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG8: NSString;
}
unsafe extern "C" {
    pub static GCKeyLANG9: NSString;
}
unsafe extern "C" {
    pub static GCKeyLeftControl: NSString;
}
unsafe extern "C" {
    pub static GCKeyLeftShift: NSString;
}
unsafe extern "C" {
    pub static GCKeyLeftAlt: NSString;
}
unsafe extern "C" {
    pub static GCKeyLeftGUI: NSString;
}
unsafe extern "C" {
    pub static GCKeyRightControl: NSString;
}
unsafe extern "C" {
    pub static GCKeyRightShift: NSString;
}
unsafe extern "C" {
    pub static GCKeyRightAlt: NSString;
}
unsafe extern "C" {
    pub static GCKeyRightGUI: NSString;
}
unsafe extern "C" {
    pub static GCInputMicroGamepadDpad: NSString;
}
unsafe extern "C" {
    pub static GCInputMicroGamepadButtonA: NSString;
}
unsafe extern "C" {
    pub static GCInputMicroGamepadButtonX: NSString;
}
unsafe extern "C" {
    pub static GCInputMicroGamepadButtonMenu: NSString;
}
unsafe extern "C" {
    pub static GCCurrentMicroGamepadSnapshotDataVersion: GCMicroGamepadSnapshotDataVersion;
}
unsafe extern "C" {
    pub fn GCMicroGamepadSnapshotDataFromNSData(
        snapshotData: *mut GCMicroGamepadSnapshotData,
        data: NSData,
    ) -> BOOL;
}
unsafe extern "C" {
    pub fn NSDataFromGCMicroGamepadSnapshotData(
        snapshotData: *mut GCMicroGamepadSnapshotData,
    ) -> NSData;
}
unsafe extern "C" {
    pub fn GCMicroGamepadSnapShotDataV100FromNSData(
        snapshotData: *mut GCMicroGamepadSnapShotDataV100,
        data: NSData,
    ) -> BOOL;
}
unsafe extern "C" {
    pub fn NSDataFromGCMicroGamepadSnapShotDataV100(
        snapshotData: *mut GCMicroGamepadSnapShotDataV100,
    ) -> NSData;
}
unsafe extern "C" {
    pub static GCInputDirectionalDpad: NSString;
}
unsafe extern "C" {
    pub static GCInputDirectionalTouchSurfaceButton: NSString;
}
unsafe extern "C" {
    pub static GCInputDirectionalCardinalDpad: NSString;
}
unsafe extern "C" {
    pub static GCInputDirectionalCenterButton: NSString;
}
unsafe extern "C" {
    pub static GCControllerDidConnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCControllerDidDisconnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCControllerDidBecomeCurrentNotification: NSString;
}
unsafe extern "C" {
    pub static GCControllerDidStopBeingCurrentNotification: NSString;
}
unsafe extern "C" {
    pub static GCControllerUserCustomizationsDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static GCStylusDidConnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCStylusDidDisconnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCKeyboardDidConnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCKeyboardDidDisconnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCMouseDidConnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCMouseDidDisconnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCMouseDidBecomeCurrentNotification: NSString;
}
unsafe extern "C" {
    pub static GCMouseDidStopBeingCurrentNotification: NSString;
}
unsafe extern "C" {
    pub static GCRacingWheelDidConnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCRacingWheelDidDisconnectNotification: NSString;
}
unsafe extern "C" {
    pub static GCHapticsLocalityDefault: GCHapticsLocality;
}
unsafe extern "C" {
    pub static GCHapticsLocalityAll: GCHapticsLocality;
}
unsafe extern "C" {
    pub static GCHapticsLocalityHandles: GCHapticsLocality;
}
unsafe extern "C" {
    pub static GCHapticsLocalityLeftHandle: GCHapticsLocality;
}
unsafe extern "C" {
    pub static GCHapticsLocalityRightHandle: GCHapticsLocality;
}
unsafe extern "C" {
    pub static GCHapticsLocalityTriggers: GCHapticsLocality;
}
unsafe extern "C" {
    pub static GCHapticsLocalityLeftTrigger: GCHapticsLocality;
}
unsafe extern "C" {
    pub static GCHapticsLocalityRightTrigger: GCHapticsLocality;
}
unsafe extern "C" {
    pub static GCHapticDurationInfinite: f32;
}

unsafe impl objc2::encode::RefEncode for __IOHIDDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDDevice", &[]);
}
unsafe impl objc2::encode::RefEncode for GCPoint2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCPoint2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCPoint2", &[]);
}
unsafe impl objc2::encode::RefEncode for GCColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCPhysicalInputElementCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCPhysicalInputElementCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCDeviceLight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDeviceLight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCDeviceBattery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDeviceBattery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCControllerElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCControllerElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCControllerAxisInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCControllerAxisInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCControllerButtonInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCControllerButtonInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCControllerDirectionPad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCControllerDirectionPad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCControllerTouchpad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCControllerTouchpad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCDualSenseAdaptiveTriggerPositionalAmplitudes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDualSenseAdaptiveTriggerPositionalAmplitudes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCDualSenseAdaptiveTriggerPositionalAmplitudes", &[]);
}
unsafe impl objc2::encode::RefEncode for GCDualSenseAdaptiveTriggerPositionalResistiveStrengths {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDualSenseAdaptiveTriggerPositionalResistiveStrengths {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCDualSenseAdaptiveTriggerPositionalResistiveStrengths", &[]);
}
unsafe impl objc2::encode::RefEncode for GCDualSenseAdaptiveTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDualSenseAdaptiveTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCDeviceCursor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDeviceCursor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCAcceleration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCAcceleration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCAcceleration", &[]);
}
unsafe impl objc2::encode::RefEncode for GCRotationRate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCRotationRate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCRotationRate", &[]);
}
unsafe impl objc2::encode::RefEncode for GCEulerAngles {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCEulerAngles {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCEulerAngles", &[]);
}
unsafe impl objc2::encode::RefEncode for GCQuaternion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCQuaternion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCQuaternion", &[]);
}
unsafe impl objc2::encode::RefEncode for GCMotion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCMotion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCPhysicalInputProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCPhysicalInputProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCGamepad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCGamepad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCGamepadSnapshot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCGamepadSnapshot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCGamepadSnapShotDataV100 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCGamepadSnapShotDataV100 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCGamepadSnapShotDataV100", &[]);
}
unsafe impl objc2::encode::RefEncode for GCExtendedGamepad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCExtendedGamepad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCExtendedGamepadSnapshot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCExtendedGamepadSnapshot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCExtendedGamepadSnapshotData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCExtendedGamepadSnapshotData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCExtendedGamepadSnapshotData", &[]);
}
unsafe impl objc2::encode::RefEncode for GCExtendedGamepadSnapShotDataV100 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCExtendedGamepadSnapShotDataV100 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCExtendedGamepadSnapShotDataV100", &[]);
}
unsafe impl objc2::encode::RefEncode for GCKeyboardInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCKeyboardInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCMouseInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCMouseInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCXboxGamepad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCXboxGamepad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCDualShockGamepad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDualShockGamepad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCDualSenseGamepad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDualSenseGamepad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCMicroGamepad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCMicroGamepad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCMicroGamepadSnapshot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCMicroGamepadSnapshot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCMicroGamepadSnapshotData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCMicroGamepadSnapshotData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCMicroGamepadSnapshotData", &[]);
}
unsafe impl objc2::encode::RefEncode for GCMicroGamepadSnapShotDataV100 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCMicroGamepadSnapShotDataV100 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GCMicroGamepadSnapShotDataV100", &[]);
}
unsafe impl objc2::encode::RefEncode for GCDirectionalGamepad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDirectionalGamepad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCControllerInputState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCControllerInputState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCControllerLiveInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCControllerLiveInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCStylus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCStylus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCKeyboard {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCKeyboard {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCMouse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCMouse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCRacingWheel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCRacingWheel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCRacingWheelInputState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCRacingWheelInputState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCRacingWheelInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCRacingWheelInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCSteeringWheelElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCSteeringWheelElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCGearShifterElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCGearShifterElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCDeviceHaptics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCDeviceHaptics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GCEventViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GCEventViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
