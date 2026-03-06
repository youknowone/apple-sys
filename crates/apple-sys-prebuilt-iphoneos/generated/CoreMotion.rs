#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMLogItem(pub id);
impl std::ops::Deref for CMLogItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMLogItem {}
impl CMLogItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMLogItem").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CMLogItem {}
impl PNSCopying for CMLogItem {}
impl INSObject for CMLogItem {}
impl PNSObject for CMLogItem {}
impl std::convert::TryFrom<NSObject> for CMLogItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMLogItem, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMLogItem").unwrap()) };
        if is_kind_of {
            Ok(CMLogItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMLogItem")
        }
    }
}
impl ICMLogItem for CMLogItem {}
pub trait ICMLogItem: Sized + std::ops::Deref {
    unsafe fn timestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMAcceleration {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMAccelerometerData(pub id);
impl std::ops::Deref for CMAccelerometerData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMAccelerometerData {}
impl CMAccelerometerData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMAccelerometerData").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMAccelerometerData {}
impl PNSSecureCoding for CMAccelerometerData {}
impl PNSCopying for CMAccelerometerData {}
impl From<CMAccelerometerData> for CMLogItem {
    fn from(child: CMAccelerometerData) -> CMLogItem {
        CMLogItem(child.0)
    }
}
impl std::convert::TryFrom<CMLogItem> for CMAccelerometerData {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMAccelerometerData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMAccelerometerData").unwrap()) };
        if is_kind_of {
            Ok(CMAccelerometerData(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMAccelerometerData")
        }
    }
}
impl INSObject for CMAccelerometerData {}
impl PNSObject for CMAccelerometerData {}
impl ICMAccelerometerData for CMAccelerometerData {}
pub trait ICMAccelerometerData: Sized + std::ops::Deref {
    unsafe fn acceleration(&self) -> CMAcceleration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceleration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMAltitudeData(pub id);
impl std::ops::Deref for CMAltitudeData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMAltitudeData {}
impl CMAltitudeData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMAltitudeData").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMAltitudeData {}
impl PNSSecureCoding for CMAltitudeData {}
impl PNSCopying for CMAltitudeData {}
impl std::convert::TryFrom<CMLogItem> for CMAltitudeData {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMAltitudeData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMAltitudeData").unwrap()) };
        if is_kind_of {
            Ok(CMAltitudeData(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMAltitudeData")
        }
    }
}
impl INSObject for CMAltitudeData {}
impl PNSObject for CMAltitudeData {}
impl ICMAltitudeData for CMAltitudeData {}
pub trait ICMAltitudeData: Sized + std::ops::Deref {
    unsafe fn relativeAltitude(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeAltitude)
    }
    unsafe fn pressure(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressure)
    }
}
pub type CMAuthorizationStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMAbsoluteAltitudeData(pub id);
impl std::ops::Deref for CMAbsoluteAltitudeData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMAbsoluteAltitudeData {}
impl CMAbsoluteAltitudeData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMAbsoluteAltitudeData").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMAbsoluteAltitudeData {}
impl PNSSecureCoding for CMAbsoluteAltitudeData {}
impl PNSCopying for CMAbsoluteAltitudeData {}
impl std::convert::TryFrom<CMLogItem> for CMAbsoluteAltitudeData {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMAbsoluteAltitudeData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMAbsoluteAltitudeData").unwrap()) };
        if is_kind_of {
            Ok(CMAbsoluteAltitudeData(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMAbsoluteAltitudeData")
        }
    }
}
impl INSObject for CMAbsoluteAltitudeData {}
impl PNSObject for CMAbsoluteAltitudeData {}
impl ICMAbsoluteAltitudeData for CMAbsoluteAltitudeData {}
pub trait ICMAbsoluteAltitudeData: Sized + std::ops::Deref {
    unsafe fn altitude(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altitude)
    }
    unsafe fn accuracy(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accuracy)
    }
    unsafe fn precision(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, precision)
    }
}
pub type CMAltitudeHandler = *mut ::std::os::raw::c_void;
pub type CMAbsoluteAltitudeHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMAltimeter(pub id);
impl std::ops::Deref for CMAltimeter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMAltimeter {}
impl CMAltimeter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMAltimeter").unwrap(), alloc) })
    }
}
impl INSObject for CMAltimeter {}
impl PNSObject for CMAltimeter {}
impl std::convert::TryFrom<NSObject> for CMAltimeter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMAltimeter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMAltimeter").unwrap()) };
        if is_kind_of {
            Ok(CMAltimeter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMAltimeter")
        }
    }
}
impl ICMAltimeter for CMAltimeter {}
pub trait ICMAltimeter: Sized + std::ops::Deref {
    unsafe fn startRelativeAltitudeUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMAltitudeHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startRelativeAltitudeUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopRelativeAltitudeUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopRelativeAltitudeUpdates)
    }
    unsafe fn startAbsoluteAltitudeUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMAbsoluteAltitudeHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAbsoluteAltitudeUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopAbsoluteAltitudeUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopAbsoluteAltitudeUpdates)
    }
    unsafe fn isRelativeAltitudeAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMAltimeter").unwrap(), isRelativeAltitudeAvailable)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMAltimeter").unwrap(), authorizationStatus)
    }
    unsafe fn isAbsoluteAltitudeAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMAltimeter").unwrap(), isAbsoluteAltitudeAvailable)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMRotationMatrix {
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
pub type CMAttitudeReferenceFrame = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMAttitude(pub id);
impl std::ops::Deref for CMAttitude {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMAttitude {}
impl CMAttitude {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMAttitude").unwrap(), alloc) })
    }
}
impl PNSCopying for CMAttitude {}
impl PNSSecureCoding for CMAttitude {}
impl INSObject for CMAttitude {}
impl PNSObject for CMAttitude {}
impl std::convert::TryFrom<NSObject> for CMAttitude {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMAttitude, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMAttitude").unwrap()) };
        if is_kind_of {
            Ok(CMAttitude(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMAttitude")
        }
    }
}
impl ICMAttitude for CMAttitude {}
pub trait ICMAttitude: Sized + std::ops::Deref {
    unsafe fn multiplyByInverseOfAttitude_(&self, attitude: CMAttitude)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, multiplyByInverseOfAttitude : attitude)
    }
    unsafe fn roll(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roll)
    }
    unsafe fn pitch(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn yaw(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yaw)
    }
    unsafe fn rotationMatrix(&self) -> CMRotationMatrix
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationMatrix)
    }
    unsafe fn quaternion(&self) -> CMQuaternion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quaternion)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMRotationRate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMGyroData(pub id);
impl std::ops::Deref for CMGyroData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMGyroData {}
impl CMGyroData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMGyroData").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMGyroData {}
impl PNSSecureCoding for CMGyroData {}
impl PNSCopying for CMGyroData {}
impl std::convert::TryFrom<CMLogItem> for CMGyroData {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMGyroData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMGyroData").unwrap()) };
        if is_kind_of {
            Ok(CMGyroData(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMGyroData")
        }
    }
}
impl INSObject for CMGyroData {}
impl PNSObject for CMGyroData {}
impl ICMGyroData for CMGyroData {}
pub trait ICMGyroData: Sized + std::ops::Deref {
    unsafe fn rotationRate(&self) -> CMRotationRate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationRate)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMMagneticField {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMMagnetometerData(pub id);
impl std::ops::Deref for CMMagnetometerData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMMagnetometerData {}
impl CMMagnetometerData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMMagnetometerData").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMMagnetometerData {}
impl PNSSecureCoding for CMMagnetometerData {}
impl PNSCopying for CMMagnetometerData {}
impl std::convert::TryFrom<CMLogItem> for CMMagnetometerData {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMMagnetometerData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMMagnetometerData").unwrap()) };
        if is_kind_of {
            Ok(CMMagnetometerData(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMMagnetometerData")
        }
    }
}
impl INSObject for CMMagnetometerData {}
impl PNSObject for CMMagnetometerData {}
impl ICMMagnetometerData for CMMagnetometerData {}
pub trait ICMMagnetometerData: Sized + std::ops::Deref {
    unsafe fn magneticField(&self) -> CMMagneticField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magneticField)
    }
}
pub type CMDeviceMotionSensorLocation = NSInteger;
pub type CMMagneticFieldCalibrationAccuracy = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMCalibratedMagneticField {
    pub field: CMMagneticField,
    pub accuracy: CMMagneticFieldCalibrationAccuracy,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMDeviceMotion(pub id);
impl std::ops::Deref for CMDeviceMotion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMDeviceMotion {}
impl CMDeviceMotion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMDeviceMotion").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMDeviceMotion {}
impl PNSSecureCoding for CMDeviceMotion {}
impl PNSCopying for CMDeviceMotion {}
impl std::convert::TryFrom<CMLogItem> for CMDeviceMotion {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMDeviceMotion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMDeviceMotion").unwrap()) };
        if is_kind_of {
            Ok(CMDeviceMotion(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMDeviceMotion")
        }
    }
}
impl INSObject for CMDeviceMotion {}
impl PNSObject for CMDeviceMotion {}
impl ICMDeviceMotion for CMDeviceMotion {}
pub trait ICMDeviceMotion: Sized + std::ops::Deref {
    unsafe fn attitude(&self) -> CMAttitude
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attitude)
    }
    unsafe fn rotationRate(&self) -> CMRotationRate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationRate)
    }
    unsafe fn gravity(&self) -> CMAcceleration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gravity)
    }
    unsafe fn userAcceleration(&self) -> CMAcceleration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userAcceleration)
    }
    unsafe fn magneticField(&self) -> CMCalibratedMagneticField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magneticField)
    }
    unsafe fn heading(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heading)
    }
    unsafe fn sensorLocation(&self) -> CMDeviceMotionSensorLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensorLocation)
    }
}
pub type CMError = ::std::os::raw::c_uint;
pub type CMFallDetectionEventUserResolution = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMFallDetectionEvent(pub id);
impl std::ops::Deref for CMFallDetectionEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMFallDetectionEvent {}
impl CMFallDetectionEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMFallDetectionEvent").unwrap(), alloc) })
    }
}
impl INSObject for CMFallDetectionEvent {}
impl PNSObject for CMFallDetectionEvent {}
impl std::convert::TryFrom<NSObject> for CMFallDetectionEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMFallDetectionEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMFallDetectionEvent").unwrap()) };
        if is_kind_of {
            Ok(CMFallDetectionEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMFallDetectionEvent")
        }
    }
}
impl ICMFallDetectionEvent for CMFallDetectionEvent {}
pub trait ICMFallDetectionEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn resolution(&self) -> CMFallDetectionEventUserResolution
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolution)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMFallDetectionManager(pub id);
impl std::ops::Deref for CMFallDetectionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMFallDetectionManager {}
impl CMFallDetectionManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMFallDetectionManager").unwrap(), alloc) })
    }
}
impl INSObject for CMFallDetectionManager {}
impl PNSObject for CMFallDetectionManager {}
impl std::convert::TryFrom<NSObject> for CMFallDetectionManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMFallDetectionManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMFallDetectionManager").unwrap()) };
        if is_kind_of {
            Ok(CMFallDetectionManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMFallDetectionManager")
        }
    }
}
impl ICMFallDetectionManager for CMFallDetectionManager {}
pub trait ICMFallDetectionManager: Sized + std::ops::Deref {
    unsafe fn requestAuthorizationWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAuthorizationWithHandler : handler)
    }
    unsafe fn authorizationStatus(&self) -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationStatus)
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
    unsafe fn isAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMFallDetectionManager").unwrap(), isAvailable)
    }
}
pub trait PCMFallDetectionDelegate: Sized + std::ops::Deref {
    unsafe fn fallDetectionManager_didDetectEvent_completionHandler_(
        &self,
        fallDetectionManager: CMFallDetectionManager,
        event: CMFallDetectionEvent,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fallDetectionManager : fallDetectionManager, didDetectEvent : event, completionHandler : handler)
    }
    unsafe fn fallDetectionManagerDidChangeAuthorization_(
        &self,
        fallDetectionManager: CMFallDetectionManager,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fallDetectionManagerDidChangeAuthorization : fallDetectionManager)
    }
}
pub type CMMotionActivityConfidence = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMMotionActivity(pub id);
impl std::ops::Deref for CMMotionActivity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMMotionActivity {}
impl CMMotionActivity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMMotionActivity").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMMotionActivity {}
impl PNSSecureCoding for CMMotionActivity {}
impl PNSCopying for CMMotionActivity {}
impl std::convert::TryFrom<CMLogItem> for CMMotionActivity {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMMotionActivity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMMotionActivity").unwrap()) };
        if is_kind_of {
            Ok(CMMotionActivity(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMMotionActivity")
        }
    }
}
impl INSObject for CMMotionActivity {}
impl PNSObject for CMMotionActivity {}
impl ICMMotionActivity for CMMotionActivity {}
pub trait ICMMotionActivity: Sized + std::ops::Deref {
    unsafe fn confidence(&self) -> CMMotionActivityConfidence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn unknown(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unknown)
    }
    unsafe fn stationary(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stationary)
    }
    unsafe fn walking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, walking)
    }
    unsafe fn running(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, running)
    }
    unsafe fn automotive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automotive)
    }
    unsafe fn cycling(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cycling)
    }
}
pub type CMMotionActivityHandler = *mut ::std::os::raw::c_void;
pub type CMMotionActivityQueryHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMMotionActivityManager(pub id);
impl std::ops::Deref for CMMotionActivityManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMMotionActivityManager {}
impl CMMotionActivityManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMMotionActivityManager").unwrap(), alloc) })
    }
}
impl INSObject for CMMotionActivityManager {}
impl PNSObject for CMMotionActivityManager {}
impl std::convert::TryFrom<NSObject> for CMMotionActivityManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMMotionActivityManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMMotionActivityManager").unwrap()) };
        if is_kind_of {
            Ok(CMMotionActivityManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMMotionActivityManager")
        }
    }
}
impl ICMMotionActivityManager for CMMotionActivityManager {}
pub trait ICMMotionActivityManager: Sized + std::ops::Deref {
    unsafe fn queryActivityStartingFromDate_toDate_toQueue_withHandler_(
        &self,
        start: NSDate,
        end: NSDate,
        queue: NSOperationQueue,
        handler: CMMotionActivityQueryHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryActivityStartingFromDate : start, toDate : end, toQueue : queue, withHandler : handler)
    }
    unsafe fn startActivityUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMMotionActivityHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startActivityUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopActivityUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopActivityUpdates)
    }
    unsafe fn isActivityAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMMotionActivityManager").unwrap(), isActivityAvailable)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMMotionActivityManager").unwrap(), authorizationStatus)
    }
}
pub type CMAccelerometerHandler = *mut ::std::os::raw::c_void;
pub type CMGyroHandler = *mut ::std::os::raw::c_void;
pub type CMDeviceMotionHandler = *mut ::std::os::raw::c_void;
pub type CMMagnetometerHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMMotionManager(pub id);
impl std::ops::Deref for CMMotionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMMotionManager {}
impl CMMotionManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMMotionManager").unwrap(), alloc) })
    }
}
impl INSObject for CMMotionManager {}
impl PNSObject for CMMotionManager {}
impl std::convert::TryFrom<NSObject> for CMMotionManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMMotionManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMMotionManager").unwrap()) };
        if is_kind_of {
            Ok(CMMotionManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMMotionManager")
        }
    }
}
impl ICMMotionManager for CMMotionManager {}
pub trait ICMMotionManager: Sized + std::ops::Deref {
    unsafe fn startAccelerometerUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startAccelerometerUpdates)
    }
    unsafe fn startAccelerometerUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMAccelerometerHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAccelerometerUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopAccelerometerUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopAccelerometerUpdates)
    }
    unsafe fn startGyroUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startGyroUpdates)
    }
    unsafe fn startGyroUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMGyroHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startGyroUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopGyroUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopGyroUpdates)
    }
    unsafe fn startMagnetometerUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startMagnetometerUpdates)
    }
    unsafe fn startMagnetometerUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMMagnetometerHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startMagnetometerUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopMagnetometerUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopMagnetometerUpdates)
    }
    unsafe fn startDeviceMotionUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDeviceMotionUpdates)
    }
    unsafe fn startDeviceMotionUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMDeviceMotionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startDeviceMotionUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn startDeviceMotionUpdatesUsingReferenceFrame_(
        &self,
        referenceFrame: CMAttitudeReferenceFrame,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startDeviceMotionUpdatesUsingReferenceFrame : referenceFrame)
    }
    unsafe fn startDeviceMotionUpdatesUsingReferenceFrame_toQueue_withHandler_(
        &self,
        referenceFrame: CMAttitudeReferenceFrame,
        queue: NSOperationQueue,
        handler: CMDeviceMotionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startDeviceMotionUpdatesUsingReferenceFrame : referenceFrame, toQueue : queue, withHandler : handler)
    }
    unsafe fn stopDeviceMotionUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopDeviceMotionUpdates)
    }
    unsafe fn accelerometerUpdateInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accelerometerUpdateInterval)
    }
    unsafe fn setAccelerometerUpdateInterval_(&self, accelerometerUpdateInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccelerometerUpdateInterval : accelerometerUpdateInterval)
    }
    unsafe fn isAccelerometerAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccelerometerAvailable)
    }
    unsafe fn isAccelerometerActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccelerometerActive)
    }
    unsafe fn accelerometerData(&self) -> CMAccelerometerData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accelerometerData)
    }
    unsafe fn gyroUpdateInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gyroUpdateInterval)
    }
    unsafe fn setGyroUpdateInterval_(&self, gyroUpdateInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGyroUpdateInterval : gyroUpdateInterval)
    }
    unsafe fn isGyroAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGyroAvailable)
    }
    unsafe fn isGyroActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGyroActive)
    }
    unsafe fn gyroData(&self) -> CMGyroData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gyroData)
    }
    unsafe fn magnetometerUpdateInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magnetometerUpdateInterval)
    }
    unsafe fn setMagnetometerUpdateInterval_(&self, magnetometerUpdateInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagnetometerUpdateInterval : magnetometerUpdateInterval)
    }
    unsafe fn isMagnetometerAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMagnetometerAvailable)
    }
    unsafe fn isMagnetometerActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMagnetometerActive)
    }
    unsafe fn magnetometerData(&self) -> CMMagnetometerData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magnetometerData)
    }
    unsafe fn deviceMotionUpdateInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceMotionUpdateInterval)
    }
    unsafe fn setDeviceMotionUpdateInterval_(&self, deviceMotionUpdateInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeviceMotionUpdateInterval : deviceMotionUpdateInterval)
    }
    unsafe fn attitudeReferenceFrame(&self) -> CMAttitudeReferenceFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attitudeReferenceFrame)
    }
    unsafe fn isDeviceMotionAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDeviceMotionAvailable)
    }
    unsafe fn isDeviceMotionActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDeviceMotionActive)
    }
    unsafe fn deviceMotion(&self) -> CMDeviceMotion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceMotion)
    }
    unsafe fn showsDeviceMovementDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsDeviceMovementDisplay)
    }
    unsafe fn setShowsDeviceMovementDisplay_(&self, showsDeviceMovementDisplay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsDeviceMovementDisplay : showsDeviceMovementDisplay)
    }
    unsafe fn availableAttitudeReferenceFrames() -> CMAttitudeReferenceFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMMotionManager").unwrap(), availableAttitudeReferenceFrames)
    }
}
pub type CMHeadphoneDeviceMotionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMHeadphoneMotionManager(pub id);
impl std::ops::Deref for CMHeadphoneMotionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMHeadphoneMotionManager {}
impl CMHeadphoneMotionManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMHeadphoneMotionManager").unwrap(), alloc) })
    }
}
impl INSObject for CMHeadphoneMotionManager {}
impl PNSObject for CMHeadphoneMotionManager {}
impl std::convert::TryFrom<NSObject> for CMHeadphoneMotionManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMHeadphoneMotionManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMHeadphoneMotionManager").unwrap()) };
        if is_kind_of {
            Ok(CMHeadphoneMotionManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMHeadphoneMotionManager")
        }
    }
}
impl ICMHeadphoneMotionManager for CMHeadphoneMotionManager {}
pub trait ICMHeadphoneMotionManager: Sized + std::ops::Deref {
    unsafe fn startDeviceMotionUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDeviceMotionUpdates)
    }
    unsafe fn startDeviceMotionUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMHeadphoneDeviceMotionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startDeviceMotionUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopDeviceMotionUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopDeviceMotionUpdates)
    }
    unsafe fn startConnectionStatusUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startConnectionStatusUpdates)
    }
    unsafe fn stopConnectionStatusUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopConnectionStatusUpdates)
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
    unsafe fn isConnectionStatusActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConnectionStatusActive)
    }
    unsafe fn isDeviceMotionAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDeviceMotionAvailable)
    }
    unsafe fn isDeviceMotionActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDeviceMotionActive)
    }
    unsafe fn deviceMotion(&self) -> CMDeviceMotion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceMotion)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMHeadphoneMotionManager").unwrap(), authorizationStatus)
    }
}
pub trait PCMHeadphoneMotionManagerDelegate: Sized + std::ops::Deref {
    unsafe fn headphoneMotionManagerDidConnect_(&self, manager: CMHeadphoneMotionManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, headphoneMotionManagerDidConnect : manager)
    }
    unsafe fn headphoneMotionManagerDidDisconnect_(&self, manager: CMHeadphoneMotionManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, headphoneMotionManagerDidDisconnect : manager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMDyskineticSymptomResult(pub id);
impl std::ops::Deref for CMDyskineticSymptomResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMDyskineticSymptomResult {}
impl CMDyskineticSymptomResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMDyskineticSymptomResult").unwrap(), alloc) })
    }
}
impl PNSCopying for CMDyskineticSymptomResult {}
impl PNSSecureCoding for CMDyskineticSymptomResult {}
impl INSObject for CMDyskineticSymptomResult {}
impl PNSObject for CMDyskineticSymptomResult {}
impl std::convert::TryFrom<NSObject> for CMDyskineticSymptomResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMDyskineticSymptomResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMDyskineticSymptomResult").unwrap()) };
        if is_kind_of {
            Ok(CMDyskineticSymptomResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMDyskineticSymptomResult")
        }
    }
}
impl ICMDyskineticSymptomResult for CMDyskineticSymptomResult {}
pub trait ICMDyskineticSymptomResult: Sized + std::ops::Deref {
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn percentUnlikely(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentUnlikely)
    }
    unsafe fn percentLikely(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentLikely)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMTremorResult(pub id);
impl std::ops::Deref for CMTremorResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMTremorResult {}
impl CMTremorResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMTremorResult").unwrap(), alloc) })
    }
}
impl PNSCopying for CMTremorResult {}
impl PNSSecureCoding for CMTremorResult {}
impl INSObject for CMTremorResult {}
impl PNSObject for CMTremorResult {}
impl std::convert::TryFrom<NSObject> for CMTremorResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMTremorResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMTremorResult").unwrap()) };
        if is_kind_of {
            Ok(CMTremorResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMTremorResult")
        }
    }
}
impl ICMTremorResult for CMTremorResult {}
pub trait ICMTremorResult: Sized + std::ops::Deref {
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn percentUnknown(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentUnknown)
    }
    unsafe fn percentNone(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentNone)
    }
    unsafe fn percentSlight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentSlight)
    }
    unsafe fn percentMild(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentMild)
    }
    unsafe fn percentModerate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentModerate)
    }
    unsafe fn percentStrong(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentStrong)
    }
}
pub type CMDyskineticSymptomResultHandler = *mut ::std::os::raw::c_void;
pub type CMTremorResultHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMMovementDisorderManager(pub id);
impl std::ops::Deref for CMMovementDisorderManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMMovementDisorderManager {}
impl CMMovementDisorderManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMMovementDisorderManager").unwrap(), alloc) })
    }
}
impl INSObject for CMMovementDisorderManager {}
impl PNSObject for CMMovementDisorderManager {}
impl std::convert::TryFrom<NSObject> for CMMovementDisorderManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMMovementDisorderManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMMovementDisorderManager").unwrap()) };
        if is_kind_of {
            Ok(CMMovementDisorderManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMMovementDisorderManager")
        }
    }
}
impl ICMMovementDisorderManager for CMMovementDisorderManager {}
pub trait ICMMovementDisorderManager: Sized + std::ops::Deref {
    unsafe fn monitorKinesiasForDuration_(&self, duration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, monitorKinesiasForDuration : duration)
    }
    unsafe fn queryDyskineticSymptomFromDate_toDate_withHandler_(
        &self,
        fromDate: NSDate,
        toDate: NSDate,
        handler: CMDyskineticSymptomResultHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryDyskineticSymptomFromDate : fromDate, toDate : toDate, withHandler : handler)
    }
    unsafe fn queryTremorFromDate_toDate_withHandler_(
        &self,
        fromDate: NSDate,
        toDate: NSDate,
        handler: CMTremorResultHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryTremorFromDate : fromDate, toDate : toDate, withHandler : handler)
    }
    unsafe fn lastProcessedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastProcessedDate)
    }
    unsafe fn monitorKinesiasExpirationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, monitorKinesiasExpirationDate)
    }
    unsafe fn isAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMMovementDisorderManager").unwrap(), isAvailable)
    }
    unsafe fn version() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMMovementDisorderManager").unwrap(), version)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMMovementDisorderManager").unwrap(), authorizationStatus)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMPedometerData(pub id);
impl std::ops::Deref for CMPedometerData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMPedometerData {}
impl CMPedometerData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometerData").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CMPedometerData {}
impl PNSCopying for CMPedometerData {}
impl INSObject for CMPedometerData {}
impl PNSObject for CMPedometerData {}
impl std::convert::TryFrom<NSObject> for CMPedometerData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMPedometerData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMPedometerData").unwrap()) };
        if is_kind_of {
            Ok(CMPedometerData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMPedometerData")
        }
    }
}
impl ICMPedometerData for CMPedometerData {}
pub trait ICMPedometerData: Sized + std::ops::Deref {
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn numberOfSteps(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfSteps)
    }
    unsafe fn distance(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distance)
    }
    unsafe fn floorsAscended(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floorsAscended)
    }
    unsafe fn floorsDescended(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floorsDescended)
    }
    unsafe fn currentPace(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPace)
    }
    unsafe fn currentCadence(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentCadence)
    }
    unsafe fn averageActivePace(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averageActivePace)
    }
}
pub type CMPedometerEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMPedometerEvent(pub id);
impl std::ops::Deref for CMPedometerEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMPedometerEvent {}
impl CMPedometerEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometerEvent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CMPedometerEvent {}
impl PNSCopying for CMPedometerEvent {}
impl INSObject for CMPedometerEvent {}
impl PNSObject for CMPedometerEvent {}
impl std::convert::TryFrom<NSObject> for CMPedometerEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMPedometerEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMPedometerEvent").unwrap()) };
        if is_kind_of {
            Ok(CMPedometerEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMPedometerEvent")
        }
    }
}
impl ICMPedometerEvent for CMPedometerEvent {}
pub trait ICMPedometerEvent: Sized + std::ops::Deref {
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn type_(&self) -> CMPedometerEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
pub type CMPedometerHandler = *mut ::std::os::raw::c_void;
pub type CMPedometerEventHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMPedometer(pub id);
impl std::ops::Deref for CMPedometer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMPedometer {}
impl CMPedometer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometer").unwrap(), alloc) })
    }
}
impl INSObject for CMPedometer {}
impl PNSObject for CMPedometer {}
impl std::convert::TryFrom<NSObject> for CMPedometer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMPedometer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMPedometer").unwrap()) };
        if is_kind_of {
            Ok(CMPedometer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMPedometer")
        }
    }
}
impl ICMPedometer for CMPedometer {}
pub trait ICMPedometer: Sized + std::ops::Deref {
    unsafe fn queryPedometerDataFromDate_toDate_withHandler_(
        &self,
        start: NSDate,
        end: NSDate,
        handler: CMPedometerHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryPedometerDataFromDate : start, toDate : end, withHandler : handler)
    }
    unsafe fn startPedometerUpdatesFromDate_withHandler_(
        &self,
        start: NSDate,
        handler: CMPedometerHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startPedometerUpdatesFromDate : start, withHandler : handler)
    }
    unsafe fn stopPedometerUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopPedometerUpdates)
    }
    unsafe fn startPedometerEventUpdatesWithHandler_(&self, handler: CMPedometerEventHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startPedometerEventUpdatesWithHandler : handler)
    }
    unsafe fn stopPedometerEventUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopPedometerEventUpdates)
    }
    unsafe fn isStepCountingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometer").unwrap(), isStepCountingAvailable)
    }
    unsafe fn isDistanceAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometer").unwrap(), isDistanceAvailable)
    }
    unsafe fn isFloorCountingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometer").unwrap(), isFloorCountingAvailable)
    }
    unsafe fn isPaceAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometer").unwrap(), isPaceAvailable)
    }
    unsafe fn isCadenceAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometer").unwrap(), isCadenceAvailable)
    }
    unsafe fn isPedometerEventTrackingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometer").unwrap(), isPedometerEventTrackingAvailable)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMPedometer").unwrap(), authorizationStatus)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMRotationRateData(pub id);
impl std::ops::Deref for CMRotationRateData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMRotationRateData {}
impl CMRotationRateData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMRotationRateData").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMRotationRateData {}
impl PNSSecureCoding for CMRotationRateData {}
impl PNSCopying for CMRotationRateData {}
impl std::convert::TryFrom<CMLogItem> for CMRotationRateData {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMRotationRateData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMRotationRateData").unwrap()) };
        if is_kind_of {
            Ok(CMRotationRateData(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMRotationRateData")
        }
    }
}
impl INSObject for CMRotationRateData {}
impl PNSObject for CMRotationRateData {}
impl ICMRotationRateData for CMRotationRateData {}
pub trait ICMRotationRateData: Sized + std::ops::Deref {
    unsafe fn rotationRate(&self) -> CMRotationRate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationRate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMRecordedRotationRateData(pub id);
impl std::ops::Deref for CMRecordedRotationRateData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMRecordedRotationRateData {}
impl CMRecordedRotationRateData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMRecordedRotationRateData").unwrap(), alloc) })
    }
}
impl ICMRotationRateData for CMRecordedRotationRateData {}
impl From<CMRecordedRotationRateData> for CMRotationRateData {
    fn from(child: CMRecordedRotationRateData) -> CMRotationRateData {
        CMRotationRateData(child.0)
    }
}
impl std::convert::TryFrom<CMRotationRateData> for CMRecordedRotationRateData {
    type Error = &'static str;
    fn try_from(parent: CMRotationRateData) -> Result<CMRecordedRotationRateData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMRecordedRotationRateData").unwrap()) };
        if is_kind_of {
            Ok(CMRecordedRotationRateData(parent.0))
        } else {
            Err("This CMRotationRateData cannot be downcasted to CMRecordedRotationRateData")
        }
    }
}
impl ICMLogItem for CMRecordedRotationRateData {}
impl PNSSecureCoding for CMRecordedRotationRateData {}
impl PNSCopying for CMRecordedRotationRateData {}
impl INSObject for CMRecordedRotationRateData {}
impl PNSObject for CMRecordedRotationRateData {}
impl ICMRecordedRotationRateData for CMRecordedRotationRateData {}
pub trait ICMRecordedRotationRateData: Sized + std::ops::Deref {
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
}
pub type CMStepQueryHandler = *mut ::std::os::raw::c_void;
pub type CMStepUpdateHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMStepCounter(pub id);
impl std::ops::Deref for CMStepCounter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMStepCounter {}
impl CMStepCounter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMStepCounter").unwrap(), alloc) })
    }
}
impl INSObject for CMStepCounter {}
impl PNSObject for CMStepCounter {}
impl std::convert::TryFrom<NSObject> for CMStepCounter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMStepCounter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMStepCounter").unwrap()) };
        if is_kind_of {
            Ok(CMStepCounter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMStepCounter")
        }
    }
}
impl ICMStepCounter for CMStepCounter {}
pub trait ICMStepCounter: Sized + std::ops::Deref {
    unsafe fn queryStepCountStartingFrom_to_toQueue_withHandler_(
        &self,
        start: NSDate,
        end: NSDate,
        queue: NSOperationQueue,
        handler: CMStepQueryHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryStepCountStartingFrom : start, to : end, toQueue : queue, withHandler : handler)
    }
    unsafe fn startStepCountingUpdatesToQueue_updateOn_withHandler_(
        &self,
        queue: NSOperationQueue,
        stepCounts: NSInteger,
        handler: CMStepUpdateHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startStepCountingUpdatesToQueue : queue, updateOn : stepCounts, withHandler : handler)
    }
    unsafe fn stopStepCountingUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopStepCountingUpdates)
    }
    unsafe fn isStepCountingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMStepCounter").unwrap(), isStepCountingAvailable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMRecordedAccelerometerData(pub id);
impl std::ops::Deref for CMRecordedAccelerometerData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMRecordedAccelerometerData {}
impl CMRecordedAccelerometerData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMRecordedAccelerometerData").unwrap(), alloc) })
    }
}
impl ICMAccelerometerData for CMRecordedAccelerometerData {}
impl From<CMRecordedAccelerometerData> for CMAccelerometerData {
    fn from(child: CMRecordedAccelerometerData) -> CMAccelerometerData {
        CMAccelerometerData(child.0)
    }
}
impl std::convert::TryFrom<CMAccelerometerData> for CMRecordedAccelerometerData {
    type Error = &'static str;
    fn try_from(parent: CMAccelerometerData) -> Result<CMRecordedAccelerometerData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMRecordedAccelerometerData").unwrap()) };
        if is_kind_of {
            Ok(CMRecordedAccelerometerData(parent.0))
        } else {
            Err("This CMAccelerometerData cannot be downcasted to CMRecordedAccelerometerData")
        }
    }
}
impl ICMLogItem for CMRecordedAccelerometerData {}
impl PNSSecureCoding for CMRecordedAccelerometerData {}
impl PNSCopying for CMRecordedAccelerometerData {}
impl INSObject for CMRecordedAccelerometerData {}
impl PNSObject for CMRecordedAccelerometerData {}
impl ICMRecordedAccelerometerData for CMRecordedAccelerometerData {}
pub trait ICMRecordedAccelerometerData: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMSensorDataList(pub id);
impl std::ops::Deref for CMSensorDataList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMSensorDataList {}
impl CMSensorDataList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMSensorDataList").unwrap(), alloc) })
    }
}
impl PNSFastEnumeration for CMSensorDataList {}
impl INSObject for CMSensorDataList {}
impl PNSObject for CMSensorDataList {}
impl std::convert::TryFrom<NSObject> for CMSensorDataList {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMSensorDataList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMSensorDataList").unwrap()) };
        if is_kind_of {
            Ok(CMSensorDataList(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMSensorDataList")
        }
    }
}
impl ICMSensorDataList for CMSensorDataList {}
pub trait ICMSensorDataList: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMSensorRecorder(pub id);
impl std::ops::Deref for CMSensorRecorder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMSensorRecorder {}
impl CMSensorRecorder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMSensorRecorder").unwrap(), alloc) })
    }
}
impl INSObject for CMSensorRecorder {}
impl PNSObject for CMSensorRecorder {}
impl std::convert::TryFrom<NSObject> for CMSensorRecorder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMSensorRecorder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMSensorRecorder").unwrap()) };
        if is_kind_of {
            Ok(CMSensorRecorder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMSensorRecorder")
        }
    }
}
impl ICMSensorRecorder for CMSensorRecorder {}
pub trait ICMSensorRecorder: Sized + std::ops::Deref {
    unsafe fn accelerometerDataFromDate_toDate_(
        &self,
        fromDate: NSDate,
        toDate: NSDate,
    ) -> CMSensorDataList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accelerometerDataFromDate : fromDate, toDate : toDate)
    }
    unsafe fn recordAccelerometerForDuration_(&self, duration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordAccelerometerForDuration : duration)
    }
    unsafe fn isAccelerometerRecordingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMSensorRecorder").unwrap(), isAccelerometerRecordingAvailable)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMSensorRecorder").unwrap(), authorizationStatus)
    }
    unsafe fn isAuthorizedForRecording() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMSensorRecorder").unwrap(), isAuthorizedForRecording)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMAmbientPressureData(pub id);
impl std::ops::Deref for CMAmbientPressureData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMAmbientPressureData {}
impl CMAmbientPressureData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMAmbientPressureData").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMAmbientPressureData {}
impl PNSSecureCoding for CMAmbientPressureData {}
impl PNSCopying for CMAmbientPressureData {}
impl std::convert::TryFrom<CMLogItem> for CMAmbientPressureData {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMAmbientPressureData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMAmbientPressureData").unwrap()) };
        if is_kind_of {
            Ok(CMAmbientPressureData(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMAmbientPressureData")
        }
    }
}
impl INSObject for CMAmbientPressureData {}
impl PNSObject for CMAmbientPressureData {}
impl ICMAmbientPressureData for CMAmbientPressureData {}
pub trait ICMAmbientPressureData: Sized + std::ops::Deref {
    unsafe fn pressure(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressure)
    }
    unsafe fn temperature(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, temperature)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMRecordedPressureData(pub id);
impl std::ops::Deref for CMRecordedPressureData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMRecordedPressureData {}
impl CMRecordedPressureData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMRecordedPressureData").unwrap(), alloc) })
    }
}
impl ICMAmbientPressureData for CMRecordedPressureData {}
impl From<CMRecordedPressureData> for CMAmbientPressureData {
    fn from(child: CMRecordedPressureData) -> CMAmbientPressureData {
        CMAmbientPressureData(child.0)
    }
}
impl std::convert::TryFrom<CMAmbientPressureData> for CMRecordedPressureData {
    type Error = &'static str;
    fn try_from(parent: CMAmbientPressureData) -> Result<CMRecordedPressureData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMRecordedPressureData").unwrap()) };
        if is_kind_of {
            Ok(CMRecordedPressureData(parent.0))
        } else {
            Err("This CMAmbientPressureData cannot be downcasted to CMRecordedPressureData")
        }
    }
}
impl ICMLogItem for CMRecordedPressureData {}
impl PNSSecureCoding for CMRecordedPressureData {}
impl PNSCopying for CMRecordedPressureData {}
impl INSObject for CMRecordedPressureData {}
impl PNSObject for CMRecordedPressureData {}
impl ICMRecordedPressureData for CMRecordedPressureData {}
pub trait ICMRecordedPressureData: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
}
pub type CMHighFrequencyHeartRateDataConfidence = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMHighFrequencyHeartRateData(pub id);
impl std::ops::Deref for CMHighFrequencyHeartRateData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMHighFrequencyHeartRateData {}
impl CMHighFrequencyHeartRateData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMHighFrequencyHeartRateData").unwrap(), alloc) })
    }
}
impl ICMLogItem for CMHighFrequencyHeartRateData {}
impl PNSSecureCoding for CMHighFrequencyHeartRateData {}
impl PNSCopying for CMHighFrequencyHeartRateData {}
impl std::convert::TryFrom<CMLogItem> for CMHighFrequencyHeartRateData {
    type Error = &'static str;
    fn try_from(parent: CMLogItem) -> Result<CMHighFrequencyHeartRateData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMHighFrequencyHeartRateData").unwrap()) };
        if is_kind_of {
            Ok(CMHighFrequencyHeartRateData(parent.0))
        } else {
            Err("This CMLogItem cannot be downcasted to CMHighFrequencyHeartRateData")
        }
    }
}
impl INSObject for CMHighFrequencyHeartRateData {}
impl PNSObject for CMHighFrequencyHeartRateData {}
impl ICMHighFrequencyHeartRateData for CMHighFrequencyHeartRateData {}
pub trait ICMHighFrequencyHeartRateData: Sized + std::ops::Deref {
    unsafe fn heartRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heartRate)
    }
    unsafe fn confidence(&self) -> CMHighFrequencyHeartRateDataConfidence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
}
pub type CMOdometerOriginDevice = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMOdometerData(pub id);
impl std::ops::Deref for CMOdometerData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMOdometerData {}
impl CMOdometerData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMOdometerData").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CMOdometerData {}
impl PNSCopying for CMOdometerData {}
impl INSObject for CMOdometerData {}
impl PNSObject for CMOdometerData {}
impl std::convert::TryFrom<NSObject> for CMOdometerData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMOdometerData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMOdometerData").unwrap()) };
        if is_kind_of {
            Ok(CMOdometerData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMOdometerData")
        }
    }
}
impl ICMOdometerData for CMOdometerData {}
pub trait ICMOdometerData: Sized + std::ops::Deref {
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn deltaDistance(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deltaDistance)
    }
    unsafe fn deltaDistanceAccuracy(&self) -> CLLocationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deltaDistanceAccuracy)
    }
    unsafe fn speed(&self) -> CLLocationSpeed
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn speedAccuracy(&self) -> CLLocationSpeedAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speedAccuracy)
    }
    unsafe fn gpsDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpsDate)
    }
    unsafe fn deltaAltitude(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deltaAltitude)
    }
    unsafe fn verticalAccuracy(&self) -> CLLocationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalAccuracy)
    }
    unsafe fn originDevice(&self) -> CMOdometerOriginDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originDevice)
    }
    unsafe fn slope(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, slope)
    }
    unsafe fn maxAbsSlope(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxAbsSlope)
    }
}
pub type CMWaterSubmersionState = NSInteger;
pub type CMWaterSubmersionDepthState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMWaterSubmersionEvent(pub id);
impl std::ops::Deref for CMWaterSubmersionEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMWaterSubmersionEvent {}
impl CMWaterSubmersionEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMWaterSubmersionEvent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CMWaterSubmersionEvent {}
impl PNSCopying for CMWaterSubmersionEvent {}
impl INSObject for CMWaterSubmersionEvent {}
impl PNSObject for CMWaterSubmersionEvent {}
impl std::convert::TryFrom<NSObject> for CMWaterSubmersionEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMWaterSubmersionEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMWaterSubmersionEvent").unwrap()) };
        if is_kind_of {
            Ok(CMWaterSubmersionEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMWaterSubmersionEvent")
        }
    }
}
impl ICMWaterSubmersionEvent for CMWaterSubmersionEvent {}
pub trait ICMWaterSubmersionEvent: Sized + std::ops::Deref {
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn state(&self) -> CMWaterSubmersionState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMWaterSubmersionMeasurement(pub id);
impl std::ops::Deref for CMWaterSubmersionMeasurement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMWaterSubmersionMeasurement {}
impl CMWaterSubmersionMeasurement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMWaterSubmersionMeasurement").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CMWaterSubmersionMeasurement {}
impl PNSCopying for CMWaterSubmersionMeasurement {}
impl INSObject for CMWaterSubmersionMeasurement {}
impl PNSObject for CMWaterSubmersionMeasurement {}
impl std::convert::TryFrom<NSObject> for CMWaterSubmersionMeasurement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMWaterSubmersionMeasurement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMWaterSubmersionMeasurement").unwrap()) };
        if is_kind_of {
            Ok(CMWaterSubmersionMeasurement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMWaterSubmersionMeasurement")
        }
    }
}
impl ICMWaterSubmersionMeasurement for CMWaterSubmersionMeasurement {}
pub trait ICMWaterSubmersionMeasurement: Sized + std::ops::Deref {
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn depth(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depth)
    }
    unsafe fn pressure(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressure)
    }
    unsafe fn surfacePressure(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, surfacePressure)
    }
    unsafe fn submersionState(&self) -> CMWaterSubmersionDepthState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submersionState)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMWaterTemperature(pub id);
impl std::ops::Deref for CMWaterTemperature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMWaterTemperature {}
impl CMWaterTemperature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMWaterTemperature").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CMWaterTemperature {}
impl PNSCopying for CMWaterTemperature {}
impl INSObject for CMWaterTemperature {}
impl PNSObject for CMWaterTemperature {}
impl std::convert::TryFrom<NSObject> for CMWaterTemperature {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMWaterTemperature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMWaterTemperature").unwrap()) };
        if is_kind_of {
            Ok(CMWaterTemperature(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMWaterTemperature")
        }
    }
}
impl ICMWaterTemperature for CMWaterTemperature {}
pub trait ICMWaterTemperature: Sized + std::ops::Deref {
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn temperature(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, temperature)
    }
    unsafe fn temperatureUncertainty(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, temperatureUncertainty)
    }
}
pub trait PCMWaterSubmersionManagerDelegate: Sized + std::ops::Deref {
    unsafe fn manager_didUpdateEvent_(
        &self,
        manager: CMWaterSubmersionManager,
        event: CMWaterSubmersionEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, manager : manager, didUpdateEvent : event)
    }
    unsafe fn manager_didUpdateMeasurement_(
        &self,
        manager: CMWaterSubmersionManager,
        measurement: CMWaterSubmersionMeasurement,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, manager : manager, didUpdateMeasurement : measurement)
    }
    unsafe fn manager_didUpdateTemperature_(
        &self,
        manager: CMWaterSubmersionManager,
        measurement: CMWaterTemperature,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, manager : manager, didUpdateTemperature : measurement)
    }
    unsafe fn manager_errorOccurred_(&self, manager: CMWaterSubmersionManager, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, manager : manager, errorOccurred : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMWaterSubmersionManager(pub id);
impl std::ops::Deref for CMWaterSubmersionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMWaterSubmersionManager {}
impl CMWaterSubmersionManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMWaterSubmersionManager").unwrap(), alloc) })
    }
}
impl INSObject for CMWaterSubmersionManager {}
impl PNSObject for CMWaterSubmersionManager {}
impl std::convert::TryFrom<NSObject> for CMWaterSubmersionManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMWaterSubmersionManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMWaterSubmersionManager").unwrap()) };
        if is_kind_of {
            Ok(CMWaterSubmersionManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMWaterSubmersionManager")
        }
    }
}
impl ICMWaterSubmersionManager for CMWaterSubmersionManager {}
pub trait ICMWaterSubmersionManager: Sized + std::ops::Deref {
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
    unsafe fn maximumDepth(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDepth)
    }
    unsafe fn waterSubmersionAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMWaterSubmersionManager").unwrap(), waterSubmersionAvailable)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMWaterSubmersionManager").unwrap(), authorizationStatus)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMBatchedSensorManager(pub id);
impl std::ops::Deref for CMBatchedSensorManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMBatchedSensorManager {}
impl CMBatchedSensorManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMBatchedSensorManager").unwrap(), alloc) })
    }
}
impl INSObject for CMBatchedSensorManager {}
impl PNSObject for CMBatchedSensorManager {}
impl std::convert::TryFrom<NSObject> for CMBatchedSensorManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMBatchedSensorManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMBatchedSensorManager").unwrap()) };
        if is_kind_of {
            Ok(CMBatchedSensorManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMBatchedSensorManager")
        }
    }
}
impl ICMBatchedSensorManager for CMBatchedSensorManager {}
pub trait ICMBatchedSensorManager: Sized + std::ops::Deref {
    unsafe fn startAccelerometerUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startAccelerometerUpdates)
    }
    unsafe fn startAccelerometerUpdatesWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAccelerometerUpdatesWithHandler : handler)
    }
    unsafe fn stopAccelerometerUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopAccelerometerUpdates)
    }
    unsafe fn startDeviceMotionUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDeviceMotionUpdates)
    }
    unsafe fn startDeviceMotionUpdatesWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startDeviceMotionUpdatesWithHandler : handler)
    }
    unsafe fn stopDeviceMotionUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopDeviceMotionUpdates)
    }
    unsafe fn isAccelerometerActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccelerometerActive)
    }
    unsafe fn accelerometerDataFrequency(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accelerometerDataFrequency)
    }
    unsafe fn accelerometerBatch(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accelerometerBatch)
    }
    unsafe fn deviceMotionDataFrequency(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceMotionDataFrequency)
    }
    unsafe fn isDeviceMotionActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDeviceMotionActive)
    }
    unsafe fn deviceMotionBatch(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceMotionBatch)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMBatchedSensorManager").unwrap(), authorizationStatus)
    }
    unsafe fn isAccelerometerSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMBatchedSensorManager").unwrap(), isAccelerometerSupported)
    }
    unsafe fn isDeviceMotionSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMBatchedSensorManager").unwrap(), isDeviceMotionSupported)
    }
}
pub type CMHeadphoneActivityStatus = NSInteger;
pub type CMHeadphoneActivityStatusHandler = *mut ::std::os::raw::c_void;
pub type CMHeadphoneActivityHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMHeadphoneActivityManager(pub id);
impl std::ops::Deref for CMHeadphoneActivityManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMHeadphoneActivityManager {}
impl CMHeadphoneActivityManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMHeadphoneActivityManager").unwrap(), alloc) })
    }
}
impl INSObject for CMHeadphoneActivityManager {}
impl PNSObject for CMHeadphoneActivityManager {}
impl std::convert::TryFrom<NSObject> for CMHeadphoneActivityManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMHeadphoneActivityManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMHeadphoneActivityManager").unwrap()) };
        if is_kind_of {
            Ok(CMHeadphoneActivityManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMHeadphoneActivityManager")
        }
    }
}
impl ICMHeadphoneActivityManager for CMHeadphoneActivityManager {}
pub trait ICMHeadphoneActivityManager: Sized + std::ops::Deref {
    unsafe fn startActivityUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMHeadphoneActivityHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startActivityUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopActivityUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopActivityUpdates)
    }
    unsafe fn startStatusUpdatesToQueue_withHandler_(
        &self,
        queue: NSOperationQueue,
        handler: CMHeadphoneActivityStatusHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startStatusUpdatesToQueue : queue, withHandler : handler)
    }
    unsafe fn stopStatusUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopStatusUpdates)
    }
    unsafe fn isActivityAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActivityAvailable)
    }
    unsafe fn isActivityActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActivityActive)
    }
    unsafe fn isStatusAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStatusAvailable)
    }
    unsafe fn isStatusActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStatusActive)
    }
    unsafe fn authorizationStatus() -> CMAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMHeadphoneActivityManager").unwrap(), authorizationStatus)
    }
}
unsafe extern "C" {
    pub static CMErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for CMLogItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMLogItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMAcceleration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAcceleration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMAcceleration", &[]);
}
unsafe impl objc2::encode::RefEncode for CMAccelerometerData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAccelerometerData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMAltitudeData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAltitudeData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMAbsoluteAltitudeData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAbsoluteAltitudeData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMAltimeter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAltimeter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMRotationMatrix {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMRotationMatrix {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMRotationMatrix", &[]);
}
unsafe impl objc2::encode::RefEncode for CMQuaternion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMQuaternion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMQuaternion", &[]);
}
unsafe impl objc2::encode::RefEncode for CMAttitude {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAttitude {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMRotationRate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMRotationRate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMRotationRate", &[]);
}
unsafe impl objc2::encode::RefEncode for CMGyroData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMGyroData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMMagneticField {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMagneticField {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMMagneticField", &[]);
}
unsafe impl objc2::encode::RefEncode for CMMagnetometerData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMagnetometerData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMCalibratedMagneticField {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMCalibratedMagneticField {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMCalibratedMagneticField", &[]);
}
unsafe impl objc2::encode::RefEncode for CMDeviceMotion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDeviceMotion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMFallDetectionEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMFallDetectionEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMFallDetectionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMFallDetectionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMMotionActivity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMotionActivity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMMotionActivityManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMotionActivityManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMMotionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMotionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMHeadphoneMotionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMHeadphoneMotionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMDyskineticSymptomResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMDyskineticSymptomResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMTremorResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTremorResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMMovementDisorderManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMMovementDisorderManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMPedometerData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMPedometerData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMPedometerEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMPedometerEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMPedometer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMPedometer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMRotationRateData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMRotationRateData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMRecordedRotationRateData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMRecordedRotationRateData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMStepCounter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMStepCounter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMRecordedAccelerometerData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMRecordedAccelerometerData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMSensorDataList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMSensorDataList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMSensorRecorder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMSensorRecorder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMAmbientPressureData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMAmbientPressureData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMRecordedPressureData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMRecordedPressureData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMHighFrequencyHeartRateData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMHighFrequencyHeartRateData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMOdometerData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMOdometerData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMWaterSubmersionEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMWaterSubmersionEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMWaterSubmersionMeasurement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMWaterSubmersionMeasurement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMWaterTemperature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMWaterTemperature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMWaterSubmersionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMWaterSubmersionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMBatchedSensorManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMBatchedSensorManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMHeadphoneActivityManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMHeadphoneActivityManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
