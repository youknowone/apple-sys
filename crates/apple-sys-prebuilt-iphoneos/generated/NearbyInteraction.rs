#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::ModelIO::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type NIErrorCode = NSInteger;
pub type NIAlgorithmConvergenceStatusReason = NSString;
pub trait PNIDeviceCapability: Sized + std::ops::Deref {
    unsafe fn supportsPreciseDistanceMeasurement(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsPreciseDistanceMeasurement)
    }
    unsafe fn supportsDirectionMeasurement(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsDirectionMeasurement)
    }
    unsafe fn supportsCameraAssistance(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsCameraAssistance)
    }
    unsafe fn supportsExtendedDistanceMeasurement(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsExtendedDistanceMeasurement)
    }
    unsafe fn supportsDLTDOAMeasurement(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsDLTDOAMeasurement)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NIDiscoveryToken(pub id);
impl std::ops::Deref for NIDiscoveryToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NIDiscoveryToken {}
impl NIDiscoveryToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NIDiscoveryToken").unwrap(), alloc) })
    }
}
impl PNSCopying for NIDiscoveryToken {}
impl PNSSecureCoding for NIDiscoveryToken {}
impl INSObject for NIDiscoveryToken {}
impl PNSObject for NIDiscoveryToken {}
impl std::convert::TryFrom<NSObject> for NIDiscoveryToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NIDiscoveryToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NIDiscoveryToken").unwrap()) };
        if is_kind_of {
            Ok(NIDiscoveryToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NIDiscoveryToken")
        }
    }
}
impl INIDiscoveryToken for NIDiscoveryToken {}
pub trait INIDiscoveryToken: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn deviceCapabilities(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceCapabilities)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NIDiscoveryToken").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NIConfiguration(pub id);
impl std::ops::Deref for NIConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NIConfiguration {}
impl NIConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NIConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for NIConfiguration {}
impl PNSSecureCoding for NIConfiguration {}
impl INSObject for NIConfiguration {}
impl PNSObject for NIConfiguration {}
impl std::convert::TryFrom<NSObject> for NIConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NIConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NIConfiguration").unwrap()) };
        if is_kind_of {
            Ok(NIConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NIConfiguration")
        }
    }
}
impl INIConfiguration for NIConfiguration {}
pub trait INIConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"NIConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NINearbyPeerConfiguration(pub id);
impl std::ops::Deref for NINearbyPeerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NINearbyPeerConfiguration {}
impl NINearbyPeerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NINearbyPeerConfiguration").unwrap(), alloc) })
    }
}
impl INIConfiguration for NINearbyPeerConfiguration {}
impl PNSCopying for NINearbyPeerConfiguration {}
impl PNSSecureCoding for NINearbyPeerConfiguration {}
impl From<NINearbyPeerConfiguration> for NIConfiguration {
    fn from(child: NINearbyPeerConfiguration) -> NIConfiguration {
        NIConfiguration(child.0)
    }
}
impl std::convert::TryFrom<NIConfiguration> for NINearbyPeerConfiguration {
    type Error = &'static str;
    fn try_from(parent: NIConfiguration) -> Result<NINearbyPeerConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NINearbyPeerConfiguration").unwrap()) };
        if is_kind_of {
            Ok(NINearbyPeerConfiguration(parent.0))
        } else {
            Err("This NIConfiguration cannot be downcasted to NINearbyPeerConfiguration")
        }
    }
}
impl INSObject for NINearbyPeerConfiguration {}
impl PNSObject for NINearbyPeerConfiguration {}
impl ININearbyPeerConfiguration for NINearbyPeerConfiguration {}
pub trait ININearbyPeerConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithPeerToken_(&self, peerToken: NIDiscoveryToken) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPeerToken : peerToken)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn peerDiscoveryToken(&self) -> NIDiscoveryToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peerDiscoveryToken)
    }
    unsafe fn isCameraAssistanceEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCameraAssistanceEnabled)
    }
    unsafe fn setCameraAssistanceEnabled_(&self, cameraAssistanceEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraAssistanceEnabled : cameraAssistanceEnabled)
    }
    unsafe fn isExtendedDistanceMeasurementEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExtendedDistanceMeasurementEnabled)
    }
    unsafe fn setExtendedDistanceMeasurementEnabled_(
        &self,
        extendedDistanceMeasurementEnabled: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtendedDistanceMeasurementEnabled : extendedDistanceMeasurementEnabled)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NINearbyPeerConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NINearbyAccessoryConfiguration(pub id);
impl std::ops::Deref for NINearbyAccessoryConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NINearbyAccessoryConfiguration {}
impl NINearbyAccessoryConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NINearbyAccessoryConfiguration").unwrap(), alloc) })
    }
}
impl INIConfiguration for NINearbyAccessoryConfiguration {}
impl PNSCopying for NINearbyAccessoryConfiguration {}
impl PNSSecureCoding for NINearbyAccessoryConfiguration {}
impl std::convert::TryFrom<NIConfiguration> for NINearbyAccessoryConfiguration {
    type Error = &'static str;
    fn try_from(parent: NIConfiguration) -> Result<NINearbyAccessoryConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NINearbyAccessoryConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(NINearbyAccessoryConfiguration(parent.0))
        } else {
            Err("This NIConfiguration cannot be downcasted to NINearbyAccessoryConfiguration")
        }
    }
}
impl INSObject for NINearbyAccessoryConfiguration {}
impl PNSObject for NINearbyAccessoryConfiguration {}
impl ININearbyAccessoryConfiguration for NINearbyAccessoryConfiguration {}
pub trait ININearbyAccessoryConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithData_error_(&self, data: NSData, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, error : error)
    }
    unsafe fn initWithAccessoryData_bluetoothPeerIdentifier_error_(
        &self,
        accessoryData: NSData,
        identifier: NSUUID,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAccessoryData : accessoryData, bluetoothPeerIdentifier : identifier, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn accessoryDiscoveryToken(&self) -> NIDiscoveryToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryDiscoveryToken)
    }
    unsafe fn isCameraAssistanceEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCameraAssistanceEnabled)
    }
    unsafe fn setCameraAssistanceEnabled_(&self, cameraAssistanceEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraAssistanceEnabled : cameraAssistanceEnabled)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NINearbyAccessoryConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NIDLTDOAConfiguration(pub id);
impl std::ops::Deref for NIDLTDOAConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NIDLTDOAConfiguration {}
impl NIDLTDOAConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NIDLTDOAConfiguration").unwrap(), alloc) })
    }
}
impl INIConfiguration for NIDLTDOAConfiguration {}
impl PNSCopying for NIDLTDOAConfiguration {}
impl PNSSecureCoding for NIDLTDOAConfiguration {}
impl std::convert::TryFrom<NIConfiguration> for NIDLTDOAConfiguration {
    type Error = &'static str;
    fn try_from(parent: NIConfiguration) -> Result<NIDLTDOAConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NIDLTDOAConfiguration").unwrap()) };
        if is_kind_of {
            Ok(NIDLTDOAConfiguration(parent.0))
        } else {
            Err("This NIConfiguration cannot be downcasted to NIDLTDOAConfiguration")
        }
    }
}
impl INSObject for NIDLTDOAConfiguration {}
impl PNSObject for NIDLTDOAConfiguration {}
impl INIDLTDOAConfiguration for NIDLTDOAConfiguration {}
pub trait INIDLTDOAConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithNetworkIdentifier_(&self, networkIdentifier: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNetworkIdentifier : networkIdentifier)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn networkIdentifier(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkIdentifier)
    }
    unsafe fn setNetworkIdentifier_(&self, networkIdentifier: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkIdentifier : networkIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NIDLTDOAConfiguration").unwrap(), new)
    }
}
pub type NINearbyObjectVerticalDirectionEstimate = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NINearbyObject(pub id);
impl std::ops::Deref for NINearbyObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NINearbyObject {}
impl NINearbyObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NINearbyObject").unwrap(), alloc) })
    }
}
impl PNSCopying for NINearbyObject {}
impl PNSSecureCoding for NINearbyObject {}
impl INSObject for NINearbyObject {}
impl PNSObject for NINearbyObject {}
impl std::convert::TryFrom<NSObject> for NINearbyObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NINearbyObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NINearbyObject").unwrap()) };
        if is_kind_of {
            Ok(NINearbyObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NINearbyObject")
        }
    }
}
impl ININearbyObject for NINearbyObject {}
pub trait ININearbyObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn discoveryToken(&self) -> NIDiscoveryToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveryToken)
    }
    unsafe fn distance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distance)
    }
    unsafe fn direction(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
    unsafe fn verticalDirectionEstimate(&self) -> NINearbyObjectVerticalDirectionEstimate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalDirectionEstimate)
    }
    unsafe fn horizontalAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalAngle)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NINearbyObject").unwrap(), new)
    }
}
pub type NIDLTDOACoordinatesType = NSInteger;
pub type NIDLTDOAMeasurementType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NIDLTDOAMeasurement(pub id);
impl std::ops::Deref for NIDLTDOAMeasurement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NIDLTDOAMeasurement {}
impl NIDLTDOAMeasurement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NIDLTDOAMeasurement").unwrap(), alloc) })
    }
}
impl PNSCopying for NIDLTDOAMeasurement {}
impl PNSSecureCoding for NIDLTDOAMeasurement {}
impl INSObject for NIDLTDOAMeasurement {}
impl PNSObject for NIDLTDOAMeasurement {}
impl std::convert::TryFrom<NSObject> for NIDLTDOAMeasurement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NIDLTDOAMeasurement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NIDLTDOAMeasurement").unwrap()) };
        if is_kind_of {
            Ok(NIDLTDOAMeasurement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NIDLTDOAMeasurement")
        }
    }
}
impl INIDLTDOAMeasurement for NIDLTDOAMeasurement {}
pub trait INIDLTDOAMeasurement: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn address(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn measurementType(&self) -> NIDLTDOAMeasurementType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, measurementType)
    }
    unsafe fn transmitTime(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transmitTime)
    }
    unsafe fn receiveTime(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, receiveTime)
    }
    unsafe fn signalStrength(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signalStrength)
    }
    unsafe fn carrierFrequencyOffset(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, carrierFrequencyOffset)
    }
    unsafe fn coordinatesType(&self) -> NIDLTDOACoordinatesType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinatesType)
    }
    unsafe fn coordinates(&self) -> simd_double3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinates)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NIDLTDOAMeasurement").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ARSession(pub id);
impl std::ops::Deref for ARSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ARSession {}
impl ARSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ARSession").unwrap(), alloc) })
    }
}
impl IARSession for ARSession {}
pub trait IARSession: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NISession(pub id);
impl std::ops::Deref for NISession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NISession {}
impl NISession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NISession").unwrap(), alloc) })
    }
}
impl INSObject for NISession {}
impl PNSObject for NISession {}
impl std::convert::TryFrom<NSObject> for NISession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NISession, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NISession").unwrap()) };
        if is_kind_of {
            Ok(NISession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NISession")
        }
    }
}
impl INISession for NISession {}
pub trait INISession: Sized + std::ops::Deref {
    unsafe fn runWithConfiguration_(&self, configuration: NIConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWithConfiguration : configuration)
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
    unsafe fn setARSession_(&self, session: ARSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setARSession : session)
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
    unsafe fn delegateQueue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegateQueue)
    }
    unsafe fn setDelegateQueue_(&self, delegateQueue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegateQueue : delegateQueue)
    }
    unsafe fn discoveryToken(&self) -> NIDiscoveryToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveryToken)
    }
    unsafe fn configuration(&self) -> NIConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NISession").unwrap(), isSupported)
    }
    unsafe fn deviceCapabilities() -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NISession").unwrap(), deviceCapabilities)
    }
}
pub type NINearbyObjectRemovalReason = NSInteger;
pub type NIAlgorithmConvergenceStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NIAlgorithmConvergence(pub id);
impl std::ops::Deref for NIAlgorithmConvergence {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NIAlgorithmConvergence {}
impl NIAlgorithmConvergence {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NIAlgorithmConvergence").unwrap(), alloc) })
    }
}
impl PNSCopying for NIAlgorithmConvergence {}
impl PNSSecureCoding for NIAlgorithmConvergence {}
impl INSObject for NIAlgorithmConvergence {}
impl PNSObject for NIAlgorithmConvergence {}
impl std::convert::TryFrom<NSObject> for NIAlgorithmConvergence {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NIAlgorithmConvergence, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NIAlgorithmConvergence").unwrap()) };
        if is_kind_of {
            Ok(NIAlgorithmConvergence(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NIAlgorithmConvergence")
        }
    }
}
impl INIAlgorithmConvergence for NIAlgorithmConvergence {}
pub trait INIAlgorithmConvergence: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn status(&self) -> NIAlgorithmConvergenceStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn reasons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reasons)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NIAlgorithmConvergence").unwrap(), new)
    }
}
pub trait PNISessionDelegate: Sized + std::ops::Deref {
    unsafe fn session_didUpdateNearbyObjects_(&self, session: NISession, nearbyObjects: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didUpdateNearbyObjects : nearbyObjects)
    }
    unsafe fn session_didRemoveNearbyObjects_withReason_(
        &self,
        session: NISession,
        nearbyObjects: NSArray,
        reason: NINearbyObjectRemovalReason,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didRemoveNearbyObjects : nearbyObjects, withReason : reason)
    }
    unsafe fn sessionWasSuspended_(&self, session: NISession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionWasSuspended : session)
    }
    unsafe fn sessionSuspensionEnded_(&self, session: NISession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionSuspensionEnded : session)
    }
    unsafe fn session_didInvalidateWithError_(&self, session: NISession, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didInvalidateWithError : error)
    }
    unsafe fn session_didGenerateShareableConfigurationData_forObject_(
        &self,
        session: NISession,
        shareableConfigurationData: NSData,
        object: NINearbyObject,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didGenerateShareableConfigurationData : shareableConfigurationData, forObject : object)
    }
    unsafe fn session_didUpdateAlgorithmConvergence_forObject_(
        &self,
        session: NISession,
        convergence: NIAlgorithmConvergence,
        object: NINearbyObject,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didUpdateAlgorithmConvergence : convergence, forObject : object)
    }
    unsafe fn session_didUpdateDLTDOAMeasurements_(&self, session: NISession, measurements: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didUpdateDLTDOAMeasurements : measurements)
    }
    unsafe fn sessionDidStartRunning_(&self, session: NISession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDidStartRunning : session)
    }
}
unsafe extern "C" {
    pub static NIErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static NIAlgorithmConvergenceStatusReasonInsufficientHorizontalSweep:
        NIAlgorithmConvergenceStatusReason;
}
unsafe extern "C" {
    pub static NIAlgorithmConvergenceStatusReasonInsufficientVerticalSweep:
        NIAlgorithmConvergenceStatusReason;
}
unsafe extern "C" {
    pub static NIAlgorithmConvergenceStatusReasonInsufficientMovement:
        NIAlgorithmConvergenceStatusReason;
}
unsafe extern "C" {
    pub static NIAlgorithmConvergenceStatusReasonInsufficientLighting:
        NIAlgorithmConvergenceStatusReason;
}
unsafe extern "C" {
    pub fn NIAlgorithmConvergenceStatusReasonDescription(reason: NSString) -> NSString;
}
unsafe extern "C" {
    pub static mut NINearbyObjectDistanceNotAvailable: f32;
}
unsafe extern "C" {
    pub static mut NINearbyObjectDirectionNotAvailable: simd_float3;
}
unsafe extern "C" {
    pub static mut NINearbyObjectAngleNotAvailable: f32;
}
unsafe extern "C" {
    pub static mut NINearbyObjectWorldTransformNotAvailable: simd_float4x4;
}

unsafe impl objc2::encode::RefEncode for NIDiscoveryToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NIDiscoveryToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NIConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NIConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NINearbyPeerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NINearbyPeerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NINearbyAccessoryConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NINearbyAccessoryConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NIDLTDOAConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NIDLTDOAConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NINearbyObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NINearbyObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NIDLTDOAMeasurement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NIDLTDOAMeasurement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ARSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ARSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NISession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NISession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NIAlgorithmConvergence {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NIAlgorithmConvergence {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
