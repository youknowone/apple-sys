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
pub type CLError = NSInteger;
pub type CLRegionState = NSInteger;
pub type CLProximity = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLRegion(pub id);
impl std::ops::Deref for CLRegion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLRegion {}
impl CLRegion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLRegion").unwrap(), alloc) })
    }
}
impl PNSCopying for CLRegion {}
impl PNSSecureCoding for CLRegion {}
impl INSObject for CLRegion {}
impl PNSObject for CLRegion {}
impl std::convert::TryFrom<NSObject> for CLRegion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLRegion, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLRegion").unwrap()) };
        if is_kind_of {
            Ok(CLRegion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLRegion")
        }
    }
}
impl ICLRegion for CLRegion {}
pub trait ICLRegion: Sized + std::ops::Deref {
    unsafe fn initCircularRegionWithCenter_radius_identifier_(
        &self,
        center: CLLocationCoordinate2D,
        radius: CLLocationDistance,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initCircularRegionWithCenter : center, radius : radius, identifier : identifier)
    }
    unsafe fn containsCoordinate_(&self, coordinate: CLLocationCoordinate2D) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsCoordinate : coordinate)
    }
    unsafe fn center(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn radius(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn notifyOnEntry(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notifyOnEntry)
    }
    unsafe fn setNotifyOnEntry_(&self, notifyOnEntry: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotifyOnEntry : notifyOnEntry)
    }
    unsafe fn notifyOnExit(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notifyOnExit)
    }
    unsafe fn setNotifyOnExit_(&self, notifyOnExit: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotifyOnExit : notifyOnExit)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLCondition(pub id);
impl std::ops::Deref for CLCondition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLCondition {}
impl CLCondition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLCondition").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CLCondition {}
impl PNSCopying for CLCondition {}
impl INSObject for CLCondition {}
impl PNSObject for CLCondition {}
impl std::convert::TryFrom<NSObject> for CLCondition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLCondition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLCondition").unwrap()) };
        if is_kind_of {
            Ok(CLCondition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLCondition")
        }
    }
}
impl ICLCondition for CLCondition {}
pub trait ICLCondition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLCondition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLCircularRegion(pub id);
impl std::ops::Deref for CLCircularRegion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLCircularRegion {}
impl CLCircularRegion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLCircularRegion").unwrap(), alloc) })
    }
}
impl ICLRegion for CLCircularRegion {}
impl PNSCopying for CLCircularRegion {}
impl PNSSecureCoding for CLCircularRegion {}
impl From<CLCircularRegion> for CLRegion {
    fn from(child: CLCircularRegion) -> CLRegion {
        CLRegion(child.0)
    }
}
impl std::convert::TryFrom<CLRegion> for CLCircularRegion {
    type Error = &'static str;
    fn try_from(parent: CLRegion) -> Result<CLCircularRegion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLCircularRegion").unwrap()) };
        if is_kind_of {
            Ok(CLCircularRegion(parent.0))
        } else {
            Err("This CLRegion cannot be downcasted to CLCircularRegion")
        }
    }
}
impl INSObject for CLCircularRegion {}
impl PNSObject for CLCircularRegion {}
impl ICLCircularRegion for CLCircularRegion {}
pub trait ICLCircularRegion: Sized + std::ops::Deref {
    unsafe fn initWithCenter_radius_identifier_(
        &self,
        center: CLLocationCoordinate2D,
        radius: CLLocationDistance,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCenter : center, radius : radius, identifier : identifier)
    }
    unsafe fn containsCoordinate_(&self, coordinate: CLLocationCoordinate2D) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsCoordinate : coordinate)
    }
    unsafe fn center(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn radius(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLCircularGeographicCondition(pub id);
impl std::ops::Deref for CLCircularGeographicCondition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLCircularGeographicCondition {}
impl CLCircularGeographicCondition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLCircularGeographicCondition").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CLCircularGeographicCondition {}
impl ICLCondition for CLCircularGeographicCondition {}
impl PNSCopying for CLCircularGeographicCondition {}
impl From<CLCircularGeographicCondition> for CLCondition {
    fn from(child: CLCircularGeographicCondition) -> CLCondition {
        CLCondition(child.0)
    }
}
impl std::convert::TryFrom<CLCondition> for CLCircularGeographicCondition {
    type Error = &'static str;
    fn try_from(parent: CLCondition) -> Result<CLCircularGeographicCondition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLCircularGeographicCondition").unwrap())
        };
        if is_kind_of {
            Ok(CLCircularGeographicCondition(parent.0))
        } else {
            Err("This CLCondition cannot be downcasted to CLCircularGeographicCondition")
        }
    }
}
impl INSObject for CLCircularGeographicCondition {}
impl PNSObject for CLCircularGeographicCondition {}
impl ICLCircularGeographicCondition for CLCircularGeographicCondition {}
pub trait ICLCircularGeographicCondition: Sized + std::ops::Deref {
    unsafe fn initWithCenter_radius_(
        &self,
        center: CLLocationCoordinate2D,
        radius: CLLocationDistance,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCenter : center, radius : radius)
    }
    unsafe fn center(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn radius(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
}
pub type CLBeaconMajorValue = u16;
pub type CLBeaconMinorValue = u16;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLBeaconIdentityCondition(pub id);
impl std::ops::Deref for CLBeaconIdentityCondition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLBeaconIdentityCondition {}
impl CLBeaconIdentityCondition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLBeaconIdentityCondition").unwrap(), alloc) })
    }
}
impl PNSCopying for CLBeaconIdentityCondition {}
impl PNSSecureCoding for CLBeaconIdentityCondition {}
impl ICLCondition for CLBeaconIdentityCondition {}
impl std::convert::TryFrom<CLCondition> for CLBeaconIdentityCondition {
    type Error = &'static str;
    fn try_from(parent: CLCondition) -> Result<CLBeaconIdentityCondition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLBeaconIdentityCondition").unwrap()) };
        if is_kind_of {
            Ok(CLBeaconIdentityCondition(parent.0))
        } else {
            Err("This CLCondition cannot be downcasted to CLBeaconIdentityCondition")
        }
    }
}
impl INSObject for CLBeaconIdentityCondition {}
impl PNSObject for CLBeaconIdentityCondition {}
impl ICLBeaconIdentityCondition for CLBeaconIdentityCondition {}
pub trait ICLBeaconIdentityCondition: Sized + std::ops::Deref {
    unsafe fn initWithUUID_(&self, uuid: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid)
    }
    unsafe fn initWithUUID_major_(&self, uuid: NSUUID, major: CLBeaconMajorValue) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid, major : major)
    }
    unsafe fn initWithUUID_major_minor_(
        &self,
        uuid: NSUUID,
        major: CLBeaconMajorValue,
        minor: CLBeaconMinorValue,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid, major : major, minor : minor)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn major(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, major)
    }
    unsafe fn minor(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLBeaconIdentityConstraint(pub id);
impl std::ops::Deref for CLBeaconIdentityConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLBeaconIdentityConstraint {}
impl CLBeaconIdentityConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLBeaconIdentityConstraint").unwrap(), alloc) })
    }
}
impl PNSCopying for CLBeaconIdentityConstraint {}
impl PNSSecureCoding for CLBeaconIdentityConstraint {}
impl ICLBeaconIdentityCondition for CLBeaconIdentityConstraint {}
impl From<CLBeaconIdentityConstraint> for CLBeaconIdentityCondition {
    fn from(child: CLBeaconIdentityConstraint) -> CLBeaconIdentityCondition {
        CLBeaconIdentityCondition(child.0)
    }
}
impl std::convert::TryFrom<CLBeaconIdentityCondition> for CLBeaconIdentityConstraint {
    type Error = &'static str;
    fn try_from(
        parent: CLBeaconIdentityCondition,
    ) -> Result<CLBeaconIdentityConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLBeaconIdentityConstraint").unwrap()) };
        if is_kind_of {
            Ok(CLBeaconIdentityConstraint(parent.0))
        } else {
            Err("This CLBeaconIdentityCondition cannot be downcasted to CLBeaconIdentityConstraint")
        }
    }
}
impl ICLCondition for CLBeaconIdentityConstraint {}
impl INSObject for CLBeaconIdentityConstraint {}
impl PNSObject for CLBeaconIdentityConstraint {}
impl ICLBeaconIdentityConstraint for CLBeaconIdentityConstraint {}
pub trait ICLBeaconIdentityConstraint: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLBeaconRegion(pub id);
impl std::ops::Deref for CLBeaconRegion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLBeaconRegion {}
impl CLBeaconRegion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLBeaconRegion").unwrap(), alloc) })
    }
}
impl ICLRegion for CLBeaconRegion {}
impl PNSCopying for CLBeaconRegion {}
impl PNSSecureCoding for CLBeaconRegion {}
impl std::convert::TryFrom<CLRegion> for CLBeaconRegion {
    type Error = &'static str;
    fn try_from(parent: CLRegion) -> Result<CLBeaconRegion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLBeaconRegion").unwrap()) };
        if is_kind_of {
            Ok(CLBeaconRegion(parent.0))
        } else {
            Err("This CLRegion cannot be downcasted to CLBeaconRegion")
        }
    }
}
impl INSObject for CLBeaconRegion {}
impl PNSObject for CLBeaconRegion {}
impl ICLBeaconRegion for CLBeaconRegion {}
pub trait ICLBeaconRegion: Sized + std::ops::Deref {
    unsafe fn initWithUUID_identifier_(&self, uuid: NSUUID, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid, identifier : identifier)
    }
    unsafe fn initWithProximityUUID_identifier_(
        &self,
        proximityUUID: NSUUID,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProximityUUID : proximityUUID, identifier : identifier)
    }
    unsafe fn initWithUUID_major_identifier_(
        &self,
        uuid: NSUUID,
        major: CLBeaconMajorValue,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid, major : major, identifier : identifier)
    }
    unsafe fn initWithProximityUUID_major_identifier_(
        &self,
        proximityUUID: NSUUID,
        major: CLBeaconMajorValue,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProximityUUID : proximityUUID, major : major, identifier : identifier)
    }
    unsafe fn initWithUUID_major_minor_identifier_(
        &self,
        uuid: NSUUID,
        major: CLBeaconMajorValue,
        minor: CLBeaconMinorValue,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid, major : major, minor : minor, identifier : identifier)
    }
    unsafe fn initWithProximityUUID_major_minor_identifier_(
        &self,
        proximityUUID: NSUUID,
        major: CLBeaconMajorValue,
        minor: CLBeaconMinorValue,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProximityUUID : proximityUUID, major : major, minor : minor, identifier : identifier)
    }
    unsafe fn initWithBeaconIdentityConstraint_identifier_(
        &self,
        beaconIdentityConstraint: CLBeaconIdentityConstraint,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBeaconIdentityConstraint : beaconIdentityConstraint, identifier : identifier)
    }
    unsafe fn peripheralDataWithMeasuredPower_(
        &self,
        measuredPower: NSNumber,
    ) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralDataWithMeasuredPower : measuredPower)
    }
    unsafe fn beaconIdentityConstraint(&self) -> CLBeaconIdentityConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beaconIdentityConstraint)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn proximityUUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proximityUUID)
    }
    unsafe fn major(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, major)
    }
    unsafe fn minor(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minor)
    }
    unsafe fn notifyEntryStateOnDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notifyEntryStateOnDisplay)
    }
    unsafe fn setNotifyEntryStateOnDisplay_(&self, notifyEntryStateOnDisplay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotifyEntryStateOnDisplay : notifyEntryStateOnDisplay)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLBeacon(pub id);
impl std::ops::Deref for CLBeacon {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLBeacon {}
impl CLBeacon {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLBeacon").unwrap(), alloc) })
    }
}
impl PNSCopying for CLBeacon {}
impl PNSSecureCoding for CLBeacon {}
impl INSObject for CLBeacon {}
impl PNSObject for CLBeacon {}
impl std::convert::TryFrom<NSObject> for CLBeacon {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLBeacon, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLBeacon").unwrap()) };
        if is_kind_of {
            Ok(CLBeacon(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLBeacon")
        }
    }
}
impl ICLBeacon for CLBeacon {}
pub trait ICLBeacon: Sized + std::ops::Deref {
    unsafe fn timestamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn proximityUUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proximityUUID)
    }
    unsafe fn major(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, major)
    }
    unsafe fn minor(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minor)
    }
    unsafe fn proximity(&self) -> CLProximity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proximity)
    }
    unsafe fn accuracy(&self) -> CLLocationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accuracy)
    }
    unsafe fn rssi(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rssi)
    }
}
pub type CLHeadingComponentValue = f64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLHeading(pub id);
impl std::ops::Deref for CLHeading {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLHeading {}
impl CLHeading {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLHeading").unwrap(), alloc) })
    }
}
impl PNSCopying for CLHeading {}
impl PNSSecureCoding for CLHeading {}
impl INSObject for CLHeading {}
impl PNSObject for CLHeading {}
impl std::convert::TryFrom<NSObject> for CLHeading {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLHeading, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLHeading").unwrap()) };
        if is_kind_of {
            Ok(CLHeading(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLHeading")
        }
    }
}
impl ICLHeading for CLHeading {}
pub trait ICLHeading: Sized + std::ops::Deref {
    unsafe fn magneticHeading(&self) -> CLLocationDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magneticHeading)
    }
    unsafe fn trueHeading(&self) -> CLLocationDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trueHeading)
    }
    unsafe fn headingAccuracy(&self) -> CLLocationDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headingAccuracy)
    }
    unsafe fn x(&self) -> CLHeadingComponentValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, x)
    }
    unsafe fn y(&self) -> CLHeadingComponentValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, y)
    }
    unsafe fn z(&self) -> CLHeadingComponentValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, z)
    }
    unsafe fn timestamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
}
pub type CLLiveUpdateConfiguration = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLUpdate(pub id);
impl std::ops::Deref for CLUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLUpdate {}
impl CLUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLUpdate").unwrap(), alloc) })
    }
}
impl INSObject for CLUpdate {}
impl PNSObject for CLUpdate {}
impl std::convert::TryFrom<NSObject> for CLUpdate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLUpdate, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLUpdate").unwrap()) };
        if is_kind_of {
            Ok(CLUpdate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLUpdate")
        }
    }
}
impl ICLUpdate for CLUpdate {}
pub trait ICLUpdate: Sized + std::ops::Deref {
    unsafe fn authorizationDenied(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationDenied)
    }
    unsafe fn authorizationDeniedGlobally(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationDeniedGlobally)
    }
    unsafe fn authorizationRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRestricted)
    }
    unsafe fn isStationary(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStationary)
    }
    unsafe fn stationary(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stationary)
    }
    unsafe fn insufficientlyInUse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insufficientlyInUse)
    }
    unsafe fn locationUnavailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationUnavailable)
    }
    unsafe fn accuracyLimited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accuracyLimited)
    }
    unsafe fn serviceSessionRequired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceSessionRequired)
    }
    unsafe fn authorizationRequestInProgress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRequestInProgress)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLLocationUpdater(pub id);
impl std::ops::Deref for CLLocationUpdater {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLLocationUpdater {}
impl CLLocationUpdater {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationUpdater").unwrap(), alloc) })
    }
}
impl INSObject for CLLocationUpdater {}
impl PNSObject for CLLocationUpdater {}
impl std::convert::TryFrom<NSObject> for CLLocationUpdater {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLLocationUpdater, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLLocationUpdater").unwrap()) };
        if is_kind_of {
            Ok(CLLocationUpdater(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLLocationUpdater")
        }
    }
}
impl ICLLocationUpdater for CLLocationUpdater {}
pub trait ICLLocationUpdater: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn resume(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resume)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationUpdater").unwrap(), new)
    }
    unsafe fn liveUpdaterWithQueue_handler_(
        queue: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationUpdater").unwrap(), liveUpdaterWithQueue : queue, handler : handler)
    }
    unsafe fn liveUpdaterWithConfiguration_queue_handler_(
        configuration: CLLiveUpdateConfiguration,
        queue: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationUpdater").unwrap(), liveUpdaterWithConfiguration : configuration, queue : queue, handler : handler)
    }
}
pub type CLMonitoringState = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLMonitoringEvent(pub id);
impl std::ops::Deref for CLMonitoringEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLMonitoringEvent {}
impl CLMonitoringEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitoringEvent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CLMonitoringEvent {}
impl INSObject for CLMonitoringEvent {}
impl PNSObject for CLMonitoringEvent {}
impl std::convert::TryFrom<NSObject> for CLMonitoringEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLMonitoringEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLMonitoringEvent").unwrap()) };
        if is_kind_of {
            Ok(CLMonitoringEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLMonitoringEvent")
        }
    }
}
impl ICLMonitoringEvent for CLMonitoringEvent {}
pub trait ICLMonitoringEvent: Sized + std::ops::Deref {
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
    unsafe fn refinement(&self) -> CLCondition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refinement)
    }
    unsafe fn state(&self) -> CLMonitoringState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn authorizationDenied(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationDenied)
    }
    unsafe fn authorizationDeniedGlobally(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationDeniedGlobally)
    }
    unsafe fn authorizationRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRestricted)
    }
    unsafe fn insufficientlyInUse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insufficientlyInUse)
    }
    unsafe fn accuracyLimited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accuracyLimited)
    }
    unsafe fn conditionUnsupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conditionUnsupported)
    }
    unsafe fn conditionLimitExceeded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conditionLimitExceeded)
    }
    unsafe fn persistenceUnavailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistenceUnavailable)
    }
    unsafe fn serviceSessionRequired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceSessionRequired)
    }
    unsafe fn authorizationRequestInProgress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRequestInProgress)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitoringEvent").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLMonitoringRecord(pub id);
impl std::ops::Deref for CLMonitoringRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLMonitoringRecord {}
impl CLMonitoringRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitoringRecord").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CLMonitoringRecord {}
impl INSObject for CLMonitoringRecord {}
impl PNSObject for CLMonitoringRecord {}
impl std::convert::TryFrom<NSObject> for CLMonitoringRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLMonitoringRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLMonitoringRecord").unwrap()) };
        if is_kind_of {
            Ok(CLMonitoringRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLMonitoringRecord")
        }
    }
}
impl ICLMonitoringRecord for CLMonitoringRecord {}
pub trait ICLMonitoringRecord: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn condition(&self) -> CLCondition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, condition)
    }
    unsafe fn lastEvent(&self) -> CLMonitoringEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastEvent)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitoringRecord").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLMonitorConfiguration(pub id);
impl std::ops::Deref for CLMonitorConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLMonitorConfiguration {}
impl CLMonitorConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitorConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for CLMonitorConfiguration {}
impl PNSObject for CLMonitorConfiguration {}
impl std::convert::TryFrom<NSObject> for CLMonitorConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLMonitorConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLMonitorConfiguration").unwrap()) };
        if is_kind_of {
            Ok(CLMonitorConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLMonitorConfiguration")
        }
    }
}
impl ICLMonitorConfiguration for CLMonitorConfiguration {}
pub trait ICLMonitorConfiguration: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn queue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queue)
    }
    unsafe fn eventHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventHandler)
    }
    unsafe fn configWithMonitorName_queue_eventHandler_(
        name: NSString,
        queue: NSObject,
        eventHandler: *mut ::std::os::raw::c_void,
    ) -> CLMonitorConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitorConfiguration").unwrap(), configWithMonitorName : name, queue : queue, eventHandler : eventHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLMonitor(pub id);
impl std::ops::Deref for CLMonitor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLMonitor {}
impl CLMonitor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitor").unwrap(), alloc) })
    }
}
impl INSObject for CLMonitor {}
impl PNSObject for CLMonitor {}
impl std::convert::TryFrom<NSObject> for CLMonitor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLMonitor, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLMonitor").unwrap()) };
        if is_kind_of {
            Ok(CLMonitor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLMonitor")
        }
    }
}
impl ICLMonitor for CLMonitor {}
pub trait ICLMonitor: Sized + std::ops::Deref {
    unsafe fn addConditionForMonitoring_identifier_(
        &self,
        condition: CLCondition,
        identifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConditionForMonitoring : condition, identifier : identifier)
    }
    unsafe fn addConditionForMonitoring_identifier_assumedState_(
        &self,
        condition: CLCondition,
        identifier: NSString,
        state: CLMonitoringState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConditionForMonitoring : condition, identifier : identifier, assumedState : state)
    }
    unsafe fn removeConditionFromMonitoringWithIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeConditionFromMonitoringWithIdentifier : identifier)
    }
    unsafe fn monitoringRecordForIdentifier_(&self, identifier: NSString) -> CLMonitoringRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, monitoringRecordForIdentifier : identifier)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn monitoredIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, monitoredIdentifiers)
    }
    unsafe fn requestMonitorWithConfiguration_completion_(
        config: CLMonitorConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitor").unwrap(), requestMonitorWithConfiguration : config, completion : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLMonitor").unwrap(), new)
    }
}
pub type CLDeviceOrientation = ::std::os::raw::c_int;
pub type CLAuthorizationStatus = ::std::os::raw::c_int;
pub type CLAccuracyAuthorization = NSInteger;
pub type CLActivityType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLLocationManager(pub id);
impl std::ops::Deref for CLLocationManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLLocationManager {}
impl CLLocationManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), alloc) })
    }
}
impl INSObject for CLLocationManager {}
impl PNSObject for CLLocationManager {}
impl std::convert::TryFrom<NSObject> for CLLocationManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLLocationManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap()) };
        if is_kind_of {
            Ok(CLLocationManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLLocationManager")
        }
    }
}
impl ICLLocationManager for CLLocationManager {}
pub trait ICLLocationManager: Sized + std::ops::Deref {
    unsafe fn requestWhenInUseAuthorization(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestWhenInUseAuthorization)
    }
    unsafe fn requestAlwaysAuthorization(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestAlwaysAuthorization)
    }
    unsafe fn requestTemporaryFullAccuracyAuthorizationWithPurposeKey_completion_(
        &self,
        purposeKey: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestTemporaryFullAccuracyAuthorizationWithPurposeKey : purposeKey, completion : completion)
    }
    unsafe fn requestTemporaryFullAccuracyAuthorizationWithPurposeKey_(&self, purposeKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestTemporaryFullAccuracyAuthorizationWithPurposeKey : purposeKey)
    }
    unsafe fn startUpdatingLocation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startUpdatingLocation)
    }
    unsafe fn stopUpdatingLocation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopUpdatingLocation)
    }
    unsafe fn requestLocation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestLocation)
    }
    unsafe fn startUpdatingHeading(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startUpdatingHeading)
    }
    unsafe fn stopUpdatingHeading(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopUpdatingHeading)
    }
    unsafe fn dismissHeadingCalibrationDisplay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dismissHeadingCalibrationDisplay)
    }
    unsafe fn startMonitoringSignificantLocationChanges(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startMonitoringSignificantLocationChanges)
    }
    unsafe fn stopMonitoringSignificantLocationChanges(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopMonitoringSignificantLocationChanges)
    }
    unsafe fn startMonitoringLocationPushesWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startMonitoringLocationPushesWithCompletion : completion)
    }
    unsafe fn stopMonitoringLocationPushes(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopMonitoringLocationPushes)
    }
    unsafe fn startMonitoringForRegion_desiredAccuracy_(
        &self,
        region: CLRegion,
        accuracy: CLLocationAccuracy,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startMonitoringForRegion : region, desiredAccuracy : accuracy)
    }
    unsafe fn stopMonitoringForRegion_(&self, region: CLRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopMonitoringForRegion : region)
    }
    unsafe fn startMonitoringForRegion_(&self, region: CLRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startMonitoringForRegion : region)
    }
    unsafe fn requestStateForRegion_(&self, region: CLRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestStateForRegion : region)
    }
    unsafe fn startRangingBeaconsInRegion_(&self, region: CLBeaconRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startRangingBeaconsInRegion : region)
    }
    unsafe fn stopRangingBeaconsInRegion_(&self, region: CLBeaconRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopRangingBeaconsInRegion : region)
    }
    unsafe fn startRangingBeaconsSatisfyingConstraint_(
        &self,
        constraint: CLBeaconIdentityConstraint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startRangingBeaconsSatisfyingConstraint : constraint)
    }
    unsafe fn stopRangingBeaconsSatisfyingConstraint_(&self, constraint: CLBeaconIdentityConstraint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopRangingBeaconsSatisfyingConstraint : constraint)
    }
    unsafe fn allowDeferredLocationUpdatesUntilTraveled_timeout_(
        &self,
        distance: CLLocationDistance,
        timeout: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, allowDeferredLocationUpdatesUntilTraveled : distance, timeout : timeout)
    }
    unsafe fn disallowDeferredLocationUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disallowDeferredLocationUpdates)
    }
    unsafe fn requestHistoricalLocationsWithPurposeKey_sampleCount_completionHandler_(
        &self,
        purposeKey: NSString,
        sampleCount: NSInteger,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestHistoricalLocationsWithPurposeKey : purposeKey, sampleCount : sampleCount, completionHandler : handler)
    }
    unsafe fn authorizationStatus(&self) -> CLAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationStatus)
    }
    unsafe fn accuracyAuthorization(&self) -> CLAccuracyAuthorization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accuracyAuthorization)
    }
    unsafe fn isAuthorizedForWidgetUpdates(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAuthorizedForWidgetUpdates)
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
    unsafe fn locationServicesEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationServicesEnabled)
    }
    unsafe fn purpose(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, purpose)
    }
    unsafe fn setPurpose_(&self, purpose: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPurpose : purpose)
    }
    unsafe fn activityType(&self) -> CLActivityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityType)
    }
    unsafe fn setActivityType_(&self, activityType: CLActivityType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivityType : activityType)
    }
    unsafe fn distanceFilter(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distanceFilter)
    }
    unsafe fn setDistanceFilter_(&self, distanceFilter: CLLocationDistance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDistanceFilter : distanceFilter)
    }
    unsafe fn desiredAccuracy(&self) -> CLLocationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredAccuracy)
    }
    unsafe fn setDesiredAccuracy_(&self, desiredAccuracy: CLLocationAccuracy)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredAccuracy : desiredAccuracy)
    }
    unsafe fn pausesLocationUpdatesAutomatically(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pausesLocationUpdatesAutomatically)
    }
    unsafe fn setPausesLocationUpdatesAutomatically_(
        &self,
        pausesLocationUpdatesAutomatically: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPausesLocationUpdatesAutomatically : pausesLocationUpdatesAutomatically)
    }
    unsafe fn allowsBackgroundLocationUpdates(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsBackgroundLocationUpdates)
    }
    unsafe fn setAllowsBackgroundLocationUpdates_(&self, allowsBackgroundLocationUpdates: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsBackgroundLocationUpdates : allowsBackgroundLocationUpdates)
    }
    unsafe fn showsBackgroundLocationIndicator(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsBackgroundLocationIndicator)
    }
    unsafe fn setShowsBackgroundLocationIndicator_(&self, showsBackgroundLocationIndicator: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsBackgroundLocationIndicator : showsBackgroundLocationIndicator)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn headingAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headingAvailable)
    }
    unsafe fn headingFilter(&self) -> CLLocationDegrees
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headingFilter)
    }
    unsafe fn setHeadingFilter_(&self, headingFilter: CLLocationDegrees)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeadingFilter : headingFilter)
    }
    unsafe fn headingOrientation(&self) -> CLDeviceOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headingOrientation)
    }
    unsafe fn setHeadingOrientation_(&self, headingOrientation: CLDeviceOrientation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeadingOrientation : headingOrientation)
    }
    unsafe fn heading(&self) -> CLHeading
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heading)
    }
    unsafe fn maximumRegionMonitoringDistance(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumRegionMonitoringDistance)
    }
    unsafe fn monitoredRegions(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, monitoredRegions)
    }
    unsafe fn rangedRegions(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangedRegions)
    }
    unsafe fn rangedBeaconConstraints(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangedBeaconConstraints)
    }
    unsafe fn class_locationServicesEnabled() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), locationServicesEnabled)
    }
    unsafe fn class_headingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), headingAvailable)
    }
    unsafe fn significantLocationChangeMonitoringAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), significantLocationChangeMonitoringAvailable)
    }
    unsafe fn isMonitoringAvailableForClass_(regionClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), isMonitoringAvailableForClass : regionClass)
    }
    unsafe fn regionMonitoringAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), regionMonitoringAvailable)
    }
    unsafe fn regionMonitoringEnabled() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), regionMonitoringEnabled)
    }
    unsafe fn isRangingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), isRangingAvailable)
    }
    unsafe fn class_authorizationStatus() -> CLAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), authorizationStatus)
    }
    unsafe fn deferredLocationUpdatesAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationManager").unwrap(), deferredLocationUpdatesAvailable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLVisit(pub id);
impl std::ops::Deref for CLVisit {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLVisit {}
impl CLVisit {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLVisit").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CLVisit {}
impl PNSCopying for CLVisit {}
impl INSObject for CLVisit {}
impl PNSObject for CLVisit {}
impl std::convert::TryFrom<NSObject> for CLVisit {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLVisit, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLVisit").unwrap()) };
        if is_kind_of {
            Ok(CLVisit(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLVisit")
        }
    }
}
impl ICLVisit for CLVisit {}
pub trait ICLVisit: Sized + std::ops::Deref {
    unsafe fn arrivalDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrivalDate)
    }
    unsafe fn departureDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, departureDate)
    }
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn horizontalAccuracy(&self) -> CLLocationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalAccuracy)
    }
}
pub trait PCLLocationManagerDelegate: Sized + std::ops::Deref {
    unsafe fn locationManager_didUpdateToLocation_fromLocation_(
        &self,
        manager: CLLocationManager,
        newLocation: CLLocation,
        oldLocation: CLLocation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didUpdateToLocation : newLocation, fromLocation : oldLocation)
    }
    unsafe fn locationManager_didUpdateLocations_(
        &self,
        manager: CLLocationManager,
        locations: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didUpdateLocations : locations)
    }
    unsafe fn locationManager_didUpdateHeading_(
        &self,
        manager: CLLocationManager,
        newHeading: CLHeading,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didUpdateHeading : newHeading)
    }
    unsafe fn locationManagerShouldDisplayHeadingCalibration_(
        &self,
        manager: CLLocationManager,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManagerShouldDisplayHeadingCalibration : manager)
    }
    unsafe fn locationManager_didDetermineState_forRegion_(
        &self,
        manager: CLLocationManager,
        state: CLRegionState,
        region: CLRegion,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didDetermineState : state, forRegion : region)
    }
    unsafe fn locationManager_didRangeBeacons_inRegion_(
        &self,
        manager: CLLocationManager,
        beacons: NSArray,
        region: CLBeaconRegion,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didRangeBeacons : beacons, inRegion : region)
    }
    unsafe fn locationManager_rangingBeaconsDidFailForRegion_withError_(
        &self,
        manager: CLLocationManager,
        region: CLBeaconRegion,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, rangingBeaconsDidFailForRegion : region, withError : error)
    }
    unsafe fn locationManager_didRangeBeacons_satisfyingConstraint_(
        &self,
        manager: CLLocationManager,
        beacons: NSArray,
        beaconConstraint: CLBeaconIdentityConstraint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didRangeBeacons : beacons, satisfyingConstraint : beaconConstraint)
    }
    unsafe fn locationManager_didFailRangingBeaconsForConstraint_error_(
        &self,
        manager: CLLocationManager,
        beaconConstraint: CLBeaconIdentityConstraint,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didFailRangingBeaconsForConstraint : beaconConstraint, error : error)
    }
    unsafe fn locationManager_didEnterRegion_(&self, manager: CLLocationManager, region: CLRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didEnterRegion : region)
    }
    unsafe fn locationManager_didExitRegion_(&self, manager: CLLocationManager, region: CLRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didExitRegion : region)
    }
    unsafe fn locationManager_didFailWithError_(&self, manager: CLLocationManager, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didFailWithError : error)
    }
    unsafe fn locationManager_monitoringDidFailForRegion_withError_(
        &self,
        manager: CLLocationManager,
        region: CLRegion,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, monitoringDidFailForRegion : region, withError : error)
    }
    unsafe fn locationManager_didChangeAuthorizationStatus_(
        &self,
        manager: CLLocationManager,
        status: CLAuthorizationStatus,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didChangeAuthorizationStatus : status)
    }
    unsafe fn locationManagerDidChangeAuthorization_(&self, manager: CLLocationManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManagerDidChangeAuthorization : manager)
    }
    unsafe fn locationManager_didStartMonitoringForRegion_(
        &self,
        manager: CLLocationManager,
        region: CLRegion,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didStartMonitoringForRegion : region)
    }
    unsafe fn locationManagerDidPauseLocationUpdates_(&self, manager: CLLocationManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManagerDidPauseLocationUpdates : manager)
    }
    unsafe fn locationManagerDidResumeLocationUpdates_(&self, manager: CLLocationManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManagerDidResumeLocationUpdates : manager)
    }
    unsafe fn locationManager_didFinishDeferredUpdatesWithError_(
        &self,
        manager: CLLocationManager,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didFinishDeferredUpdatesWithError : error)
    }
    unsafe fn locationManager_didVisit_(&self, manager: CLLocationManager, visit: CLVisit)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationManager : manager, didVisit : visit)
    }
}
impl CLLocationManager_CLVisitExtensions for CLLocationManager {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLPlacemark(pub id);
impl std::ops::Deref for CLPlacemark {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLPlacemark {}
impl CLPlacemark {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLPlacemark").unwrap(), alloc) })
    }
}
impl PNSCopying for CLPlacemark {}
impl PNSSecureCoding for CLPlacemark {}
impl INSObject for CLPlacemark {}
impl PNSObject for CLPlacemark {}
impl std::convert::TryFrom<NSObject> for CLPlacemark {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLPlacemark, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLPlacemark").unwrap()) };
        if is_kind_of {
            Ok(CLPlacemark(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLPlacemark")
        }
    }
}
impl ICLPlacemark for CLPlacemark {}
pub trait ICLPlacemark: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPlacemark_(&self, placemark: CLPlacemark) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlacemark : placemark)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn region(&self) -> CLRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn timeZone(&self) -> NSTimeZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeZone)
    }
    unsafe fn addressDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressDictionary)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn thoroughfare(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thoroughfare)
    }
    unsafe fn subThoroughfare(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subThoroughfare)
    }
    unsafe fn locality(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locality)
    }
    unsafe fn subLocality(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subLocality)
    }
    unsafe fn administrativeArea(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, administrativeArea)
    }
    unsafe fn subAdministrativeArea(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subAdministrativeArea)
    }
    unsafe fn postalCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postalCode)
    }
    unsafe fn ISOcountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ISOcountryCode)
    }
    unsafe fn country(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, country)
    }
    unsafe fn inlandWater(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inlandWater)
    }
    unsafe fn ocean(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ocean)
    }
    unsafe fn areasOfInterest(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areasOfInterest)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLPlacemark").unwrap(), new)
    }
}
impl CLPlacemark_ContactsAdditions for CLPlacemark {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLGeocoder(pub id);
impl std::ops::Deref for CLGeocoder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLGeocoder {}
impl CLGeocoder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLGeocoder").unwrap(), alloc) })
    }
}
impl INSObject for CLGeocoder {}
impl PNSObject for CLGeocoder {}
impl std::convert::TryFrom<NSObject> for CLGeocoder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLGeocoder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLGeocoder").unwrap()) };
        if is_kind_of {
            Ok(CLGeocoder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLGeocoder")
        }
    }
}
impl ICLGeocoder for CLGeocoder {}
pub trait ICLGeocoder: Sized + std::ops::Deref {
    unsafe fn reverseGeocodeLocation_completionHandler_(
        &self,
        location: CLLocation,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reverseGeocodeLocation : location, completionHandler : completionHandler)
    }
    unsafe fn reverseGeocodeLocation_preferredLocale_completionHandler_(
        &self,
        location: CLLocation,
        locale: NSLocale,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reverseGeocodeLocation : location, preferredLocale : locale, completionHandler : completionHandler)
    }
    unsafe fn geocodeAddressDictionary_completionHandler_(
        &self,
        addressDictionary: NSDictionary,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geocodeAddressDictionary : addressDictionary, completionHandler : completionHandler)
    }
    unsafe fn geocodeAddressString_inRegion_completionHandler_(
        &self,
        addressString: NSString,
        region: CLRegion,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geocodeAddressString : addressString, inRegion : region, completionHandler : completionHandler)
    }
    unsafe fn geocodeAddressString_inRegion_preferredLocale_completionHandler_(
        &self,
        addressString: NSString,
        region: CLRegion,
        locale: NSLocale,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geocodeAddressString : addressString, inRegion : region, preferredLocale : locale, completionHandler : completionHandler)
    }
    unsafe fn geocodeAddressString_inRegionCenteredAt_inRegionRadius_preferredLocale_completionHandler_(
        &self,
        addressString: NSString,
        centroid: CLLocationCoordinate2D,
        radius: CLLocationDistance,
        locale: NSLocale,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geocodeAddressString : addressString, inRegionCenteredAt : centroid, inRegionRadius : radius, preferredLocale : locale, completionHandler : completionHandler)
    }
    unsafe fn geocodeAddressString_completionHandler_(
        &self,
        addressString: NSString,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geocodeAddressString : addressString, completionHandler : completionHandler)
    }
    unsafe fn cancelGeocode(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelGeocode)
    }
    unsafe fn isGeocoding(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGeocoding)
    }
}
impl CLGeocoder_ContactsAdditions for CLGeocoder {}
pub trait PCLLocationPushServiceExtension: Sized + std::ops::Deref {
    unsafe fn didReceiveLocationPushPayload_completion_(
        &self,
        payload: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveLocationPushPayload : payload, completion : completion)
    }
    unsafe fn serviceExtensionWillTerminate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceExtensionWillTerminate)
    }
}
pub type CLLocationPushServiceError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLBackgroundActivitySessionDiagnostic(pub id);
impl std::ops::Deref for CLBackgroundActivitySessionDiagnostic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLBackgroundActivitySessionDiagnostic {}
impl CLBackgroundActivitySessionDiagnostic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLBackgroundActivitySessionDiagnostic").unwrap(), alloc) })
    }
}
impl INSObject for CLBackgroundActivitySessionDiagnostic {}
impl PNSObject for CLBackgroundActivitySessionDiagnostic {}
impl std::convert::TryFrom<NSObject> for CLBackgroundActivitySessionDiagnostic {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLBackgroundActivitySessionDiagnostic, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLBackgroundActivitySessionDiagnostic").unwrap())
        };
        if is_kind_of {
            Ok(CLBackgroundActivitySessionDiagnostic(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLBackgroundActivitySessionDiagnostic")
        }
    }
}
impl ICLBackgroundActivitySessionDiagnostic for CLBackgroundActivitySessionDiagnostic {}
pub trait ICLBackgroundActivitySessionDiagnostic: Sized + std::ops::Deref {
    unsafe fn authorizationDenied(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationDenied)
    }
    unsafe fn authorizationDeniedGlobally(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationDeniedGlobally)
    }
    unsafe fn authorizationRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRestricted)
    }
    unsafe fn insufficientlyInUse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insufficientlyInUse)
    }
    unsafe fn serviceSessionRequired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceSessionRequired)
    }
    unsafe fn authorizationRequestInProgress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRequestInProgress)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLBackgroundActivitySession(pub id);
impl std::ops::Deref for CLBackgroundActivitySession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLBackgroundActivitySession {}
impl CLBackgroundActivitySession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLBackgroundActivitySession").unwrap(), alloc) })
    }
}
impl INSObject for CLBackgroundActivitySession {}
impl PNSObject for CLBackgroundActivitySession {}
impl std::convert::TryFrom<NSObject> for CLBackgroundActivitySession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLBackgroundActivitySession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLBackgroundActivitySession").unwrap()) };
        if is_kind_of {
            Ok(CLBackgroundActivitySession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLBackgroundActivitySession")
        }
    }
}
impl ICLBackgroundActivitySession for CLBackgroundActivitySession {}
pub trait ICLBackgroundActivitySession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLBackgroundActivitySession").unwrap(), new)
    }
    unsafe fn backgroundActivitySession() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLBackgroundActivitySession").unwrap(), backgroundActivitySession)
    }
    unsafe fn backgroundActivitySessionWithQueue_handler_(
        queue: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLBackgroundActivitySession").unwrap(), backgroundActivitySessionWithQueue : queue, handler : handler)
    }
}
pub type CLServiceSessionAuthorizationRequirement = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLServiceSessionDiagnostic(pub id);
impl std::ops::Deref for CLServiceSessionDiagnostic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLServiceSessionDiagnostic {}
impl CLServiceSessionDiagnostic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLServiceSessionDiagnostic").unwrap(), alloc) })
    }
}
impl INSObject for CLServiceSessionDiagnostic {}
impl PNSObject for CLServiceSessionDiagnostic {}
impl std::convert::TryFrom<NSObject> for CLServiceSessionDiagnostic {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLServiceSessionDiagnostic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLServiceSessionDiagnostic").unwrap()) };
        if is_kind_of {
            Ok(CLServiceSessionDiagnostic(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLServiceSessionDiagnostic")
        }
    }
}
impl ICLServiceSessionDiagnostic for CLServiceSessionDiagnostic {}
pub trait ICLServiceSessionDiagnostic: Sized + std::ops::Deref {
    unsafe fn authorizationDenied(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationDenied)
    }
    unsafe fn authorizationDeniedGlobally(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationDeniedGlobally)
    }
    unsafe fn authorizationRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRestricted)
    }
    unsafe fn insufficientlyInUse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insufficientlyInUse)
    }
    unsafe fn serviceSessionRequired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceSessionRequired)
    }
    unsafe fn fullAccuracyDenied(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullAccuracyDenied)
    }
    unsafe fn alwaysAuthorizationDenied(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alwaysAuthorizationDenied)
    }
    unsafe fn authorizationRequestInProgress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRequestInProgress)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLServiceSession(pub id);
impl std::ops::Deref for CLServiceSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLServiceSession {}
impl CLServiceSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLServiceSession").unwrap(), alloc) })
    }
}
impl INSObject for CLServiceSession {}
impl PNSObject for CLServiceSession {}
impl std::convert::TryFrom<NSObject> for CLServiceSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLServiceSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLServiceSession").unwrap()) };
        if is_kind_of {
            Ok(CLServiceSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLServiceSession")
        }
    }
}
impl ICLServiceSession for CLServiceSession {}
pub trait ICLServiceSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLServiceSession").unwrap(), new)
    }
    unsafe fn sessionRequiringAuthorization_(
        authorizationRequirement: CLServiceSessionAuthorizationRequirement,
    ) -> CLServiceSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLServiceSession").unwrap(), sessionRequiringAuthorization : authorizationRequirement)
    }
    unsafe fn sessionRequiringAuthorization_queue_handler_(
        authorizationRequirement: CLServiceSessionAuthorizationRequirement,
        queue: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> CLServiceSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLServiceSession").unwrap(), sessionRequiringAuthorization : authorizationRequirement, queue : queue, handler : handler)
    }
    unsafe fn sessionRequiringAuthorization_fullAccuracyPurposeKey_(
        authorizationRequirement: CLServiceSessionAuthorizationRequirement,
        purposeKey: NSString,
    ) -> CLServiceSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLServiceSession").unwrap(), sessionRequiringAuthorization : authorizationRequirement, fullAccuracyPurposeKey : purposeKey)
    }
    unsafe fn sessionRequiringAuthorization_fullAccuracyPurposeKey_queue_handler_(
        authorizationRequirement: CLServiceSessionAuthorizationRequirement,
        purposeKey: NSString,
        queue: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> CLServiceSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLServiceSession").unwrap(), sessionRequiringAuthorization : authorizationRequirement, fullAccuracyPurposeKey : purposeKey, queue : queue, handler : handler)
    }
}
unsafe extern "C" {
    pub static kCLErrorDomain: NSString;
}
unsafe extern "C" {
    pub static kCLErrorUserInfoAlternateRegionKey: NSString;
}
unsafe extern "C" {
    pub static kCLHeadingFilterNone: CLLocationDegrees;
}
unsafe extern "C" {
    pub static CLLocationPushServiceErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for CLRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLCondition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLCondition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLCircularRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLCircularRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLCircularGeographicCondition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLCircularGeographicCondition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLBeaconIdentityCondition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLBeaconIdentityCondition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLBeaconIdentityConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLBeaconIdentityConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLBeaconRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLBeaconRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLBeacon {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLBeacon {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLHeading {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLHeading {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLLocationUpdater {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLLocationUpdater {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLMonitoringEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLMonitoringEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLMonitoringRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLMonitoringRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLMonitorConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLMonitorConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLMonitor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLMonitor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLLocationManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLLocationManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLVisit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLVisit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLPlacemark {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLPlacemark {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLGeocoder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLGeocoder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLBackgroundActivitySessionDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLBackgroundActivitySessionDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLBackgroundActivitySession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLBackgroundActivitySession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLServiceSessionDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLServiceSessionDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLServiceSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLServiceSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
