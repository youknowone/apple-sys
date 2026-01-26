#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::SoundAnalysis::*;
#[allow(unused_imports)]
use crate::Speech::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SRSensor = NSString;
pub type SRAbsoluteTime = CFTimeInterval;
pub trait NSDate_SensorKit: Sized + std::ops::Deref {
    unsafe fn initWithSRAbsoluteTime_(&self, time: SRAbsoluteTime) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSRAbsoluteTime : time)
    }
    unsafe fn srAbsoluteTime(&self) -> SRAbsoluteTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, srAbsoluteTime)
    }
    unsafe fn dateWithSRAbsoluteTime_(time: SRAbsoluteTime) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSDate").unwrap(), dateWithSRAbsoluteTime : time)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRFetchResult(pub id);
impl std::ops::Deref for SRFetchResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRFetchResult {}
impl SRFetchResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRFetchResult").unwrap(), alloc) })
    }
}
impl PNSCopying for SRFetchResult {}
impl INSObject for SRFetchResult {}
impl PNSObject for SRFetchResult {}
impl std::convert::TryFrom<NSObject> for SRFetchResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRFetchResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRFetchResult").unwrap()) };
        if is_kind_of {
            Ok(SRFetchResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRFetchResult")
        }
    }
}
impl<SampleType: 'static> ISRFetchResult<SampleType> for SRFetchResult {}
pub trait ISRFetchResult<SampleType: 'static>: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sample(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sample)
    }
    unsafe fn timestamp(&self) -> SRAbsoluteTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRFetchResult").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRDevice(pub id);
impl std::ops::Deref for SRDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRDevice {}
impl SRDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRDevice").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SRDevice {}
impl PNSCopying for SRDevice {}
impl INSObject for SRDevice {}
impl PNSObject for SRDevice {}
impl std::convert::TryFrom<NSObject> for SRDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRDevice, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRDevice").unwrap()) };
        if is_kind_of {
            Ok(SRDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRDevice")
        }
    }
}
impl ISRDevice for SRDevice {}
pub trait ISRDevice: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn model(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn systemName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemName)
    }
    unsafe fn systemVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemVersion)
    }
    unsafe fn productType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productType)
    }
    unsafe fn currentDevice() -> SRDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRDevice").unwrap(), currentDevice)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRFetchRequest(pub id);
impl std::ops::Deref for SRFetchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRFetchRequest {}
impl SRFetchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRFetchRequest").unwrap(), alloc) })
    }
}
impl INSObject for SRFetchRequest {}
impl PNSObject for SRFetchRequest {}
impl std::convert::TryFrom<NSObject> for SRFetchRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRFetchRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRFetchRequest").unwrap()) };
        if is_kind_of {
            Ok(SRFetchRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRFetchRequest")
        }
    }
}
impl ISRFetchRequest for SRFetchRequest {}
pub trait ISRFetchRequest: Sized + std::ops::Deref {
    unsafe fn from(&self) -> SRAbsoluteTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, from)
    }
    unsafe fn setFrom_(&self, from: SRAbsoluteTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrom : from)
    }
    unsafe fn to(&self) -> SRAbsoluteTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, to)
    }
    unsafe fn setTo_(&self, to: SRAbsoluteTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTo : to)
    }
    unsafe fn device(&self) -> SRDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn setDevice_(&self, device: SRDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDevice : device)
    }
}
pub type SRAuthorizationStatus = NSInteger;
pub trait PSRSensorReaderDelegate: Sized + std::ops::Deref {
    unsafe fn sensorReader_fetchingRequest_didFetchResult_(
        &self,
        reader: SRSensorReader,
        fetchRequest: SRFetchRequest,
        result: SRFetchResult,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReader : reader, fetchingRequest : fetchRequest, didFetchResult : result)
    }
    unsafe fn sensorReader_didCompleteFetch_(
        &self,
        reader: SRSensorReader,
        fetchRequest: SRFetchRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReader : reader, didCompleteFetch : fetchRequest)
    }
    unsafe fn sensorReader_fetchingRequest_failedWithError_(
        &self,
        reader: SRSensorReader,
        fetchRequest: SRFetchRequest,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReader : reader, fetchingRequest : fetchRequest, failedWithError : error)
    }
    unsafe fn sensorReader_didChangeAuthorizationStatus_(
        &self,
        reader: SRSensorReader,
        authorizationStatus: SRAuthorizationStatus,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReader : reader, didChangeAuthorizationStatus : authorizationStatus)
    }
    unsafe fn sensorReaderWillStartRecording_(&self, reader: SRSensorReader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReaderWillStartRecording : reader)
    }
    unsafe fn sensorReader_startRecordingFailedWithError_(
        &self,
        reader: SRSensorReader,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReader : reader, startRecordingFailedWithError : error)
    }
    unsafe fn sensorReaderDidStopRecording_(&self, reader: SRSensorReader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReaderDidStopRecording : reader)
    }
    unsafe fn sensorReader_stopRecordingFailedWithError_(
        &self,
        reader: SRSensorReader,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReader : reader, stopRecordingFailedWithError : error)
    }
    unsafe fn sensorReader_didFetchDevices_(&self, reader: SRSensorReader, devices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReader : reader, didFetchDevices : devices)
    }
    unsafe fn sensorReader_fetchDevicesDidFailWithError_(
        &self,
        reader: SRSensorReader,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sensorReader : reader, fetchDevicesDidFailWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRSensorReader(pub id);
impl std::ops::Deref for SRSensorReader {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRSensorReader {}
impl SRSensorReader {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRSensorReader").unwrap(), alloc) })
    }
}
impl INSObject for SRSensorReader {}
impl PNSObject for SRSensorReader {}
impl std::convert::TryFrom<NSObject> for SRSensorReader {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRSensorReader, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRSensorReader").unwrap()) };
        if is_kind_of {
            Ok(SRSensorReader(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRSensorReader")
        }
    }
}
impl ISRSensorReader for SRSensorReader {}
pub trait ISRSensorReader: Sized + std::ops::Deref {
    unsafe fn initWithSensor_(&self, sensor: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSensor : sensor)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn startRecording(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startRecording)
    }
    unsafe fn stopRecording(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopRecording)
    }
    unsafe fn fetchDevices(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchDevices)
    }
    unsafe fn fetch_(&self, request: SRFetchRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetch : request)
    }
    unsafe fn authorizationStatus(&self) -> SRAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationStatus)
    }
    unsafe fn sensor(&self) -> SRSensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensor)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRSensorReader").unwrap(), new)
    }
    unsafe fn requestAuthorizationForSensors_completion_(
        sensors: NSSet,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRSensorReader").unwrap(), requestAuthorizationForSensors : sensors, completion : completion)
    }
}
pub type SRErrorCode = NSInteger;
pub type SRAmbientLightSensorPlacement = NSInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SRAmbientLightChromaticity {
    pub x: Float32,
    pub y: Float32,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRAmbientLightSample(pub id);
impl std::ops::Deref for SRAmbientLightSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRAmbientLightSample {}
impl SRAmbientLightSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRAmbientLightSample").unwrap(), alloc) })
    }
}
impl INSObject for SRAmbientLightSample {}
impl PNSObject for SRAmbientLightSample {}
impl std::convert::TryFrom<NSObject> for SRAmbientLightSample {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRAmbientLightSample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRAmbientLightSample").unwrap()) };
        if is_kind_of {
            Ok(SRAmbientLightSample(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRAmbientLightSample")
        }
    }
}
impl ISRAmbientLightSample for SRAmbientLightSample {}
pub trait ISRAmbientLightSample: Sized + std::ops::Deref {
    unsafe fn placement(&self) -> SRAmbientLightSensorPlacement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placement)
    }
    unsafe fn chromaticity(&self) -> SRAmbientLightChromaticity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chromaticity)
    }
    unsafe fn lux(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lux)
    }
}
pub type SRLocationCategory = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRVisit(pub id);
impl std::ops::Deref for SRVisit {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRVisit {}
impl SRVisit {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRVisit").unwrap(), alloc) })
    }
}
impl INSObject for SRVisit {}
impl PNSObject for SRVisit {}
impl std::convert::TryFrom<NSObject> for SRVisit {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRVisit, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRVisit").unwrap()) };
        if is_kind_of {
            Ok(SRVisit(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRVisit")
        }
    }
}
impl ISRVisit for SRVisit {}
pub trait ISRVisit: Sized + std::ops::Deref {
    unsafe fn distanceFromHome(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distanceFromHome)
    }
    unsafe fn arrivalDateInterval(&self) -> NSDateInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrivalDateInterval)
    }
    unsafe fn departureDateInterval(&self) -> NSDateInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, departureDateInterval)
    }
    unsafe fn locationCategory(&self) -> SRLocationCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationCategory)
    }
    unsafe fn identifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
pub type SRDeviceUsageCategoryKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRSupplementalCategory(pub id);
impl std::ops::Deref for SRSupplementalCategory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRSupplementalCategory {}
impl SRSupplementalCategory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRSupplementalCategory").unwrap(), alloc) })
    }
}
impl PNSCopying for SRSupplementalCategory {}
impl PNSSecureCoding for SRSupplementalCategory {}
impl INSObject for SRSupplementalCategory {}
impl PNSObject for SRSupplementalCategory {}
impl std::convert::TryFrom<NSObject> for SRSupplementalCategory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRSupplementalCategory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRSupplementalCategory").unwrap()) };
        if is_kind_of {
            Ok(SRSupplementalCategory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRSupplementalCategory")
        }
    }
}
impl ISRSupplementalCategory for SRSupplementalCategory {}
pub trait ISRSupplementalCategory: Sized + std::ops::Deref {
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRSupplementalCategory").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRDeviceUsageReport(pub id);
impl std::ops::Deref for SRDeviceUsageReport {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRDeviceUsageReport {}
impl SRDeviceUsageReport {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRDeviceUsageReport").unwrap(), alloc) })
    }
}
impl INSObject for SRDeviceUsageReport {}
impl PNSObject for SRDeviceUsageReport {}
impl std::convert::TryFrom<NSObject> for SRDeviceUsageReport {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRDeviceUsageReport, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRDeviceUsageReport").unwrap()) };
        if is_kind_of {
            Ok(SRDeviceUsageReport(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRDeviceUsageReport")
        }
    }
}
impl ISRDeviceUsageReport for SRDeviceUsageReport {}
pub trait ISRDeviceUsageReport: Sized + std::ops::Deref {
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn applicationUsageByCategory(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationUsageByCategory)
    }
    unsafe fn notificationUsageByCategory(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationUsageByCategory)
    }
    unsafe fn webUsageByCategory(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webUsageByCategory)
    }
    unsafe fn totalScreenWakes(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalScreenWakes)
    }
    unsafe fn totalUnlocks(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalUnlocks)
    }
    unsafe fn totalUnlockDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalUnlockDuration)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
}
pub type SRTextInputSessionType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRTextInputSession(pub id);
impl std::ops::Deref for SRTextInputSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRTextInputSession {}
impl SRTextInputSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRTextInputSession").unwrap(), alloc) })
    }
}
impl INSObject for SRTextInputSession {}
impl PNSObject for SRTextInputSession {}
impl std::convert::TryFrom<NSObject> for SRTextInputSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRTextInputSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRTextInputSession").unwrap()) };
        if is_kind_of {
            Ok(SRTextInputSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRTextInputSession")
        }
    }
}
impl ISRTextInputSession for SRTextInputSession {}
pub trait ISRTextInputSession: Sized + std::ops::Deref {
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn sessionType(&self) -> SRTextInputSessionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionType)
    }
    unsafe fn sessionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRApplicationUsage(pub id);
impl std::ops::Deref for SRApplicationUsage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRApplicationUsage {}
impl SRApplicationUsage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRApplicationUsage").unwrap(), alloc) })
    }
}
impl INSObject for SRApplicationUsage {}
impl PNSObject for SRApplicationUsage {}
impl std::convert::TryFrom<NSObject> for SRApplicationUsage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRApplicationUsage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRApplicationUsage").unwrap()) };
        if is_kind_of {
            Ok(SRApplicationUsage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRApplicationUsage")
        }
    }
}
impl ISRApplicationUsage for SRApplicationUsage {}
pub trait ISRApplicationUsage: Sized + std::ops::Deref {
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn usageTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usageTime)
    }
    unsafe fn reportApplicationIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportApplicationIdentifier)
    }
    unsafe fn textInputSessions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textInputSessions)
    }
    unsafe fn supplementalCategories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supplementalCategories)
    }
    unsafe fn relativeStartTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeStartTime)
    }
}
pub type SRNotificationEvent = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRNotificationUsage(pub id);
impl std::ops::Deref for SRNotificationUsage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRNotificationUsage {}
impl SRNotificationUsage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRNotificationUsage").unwrap(), alloc) })
    }
}
impl INSObject for SRNotificationUsage {}
impl PNSObject for SRNotificationUsage {}
impl std::convert::TryFrom<NSObject> for SRNotificationUsage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRNotificationUsage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRNotificationUsage").unwrap()) };
        if is_kind_of {
            Ok(SRNotificationUsage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRNotificationUsage")
        }
    }
}
impl ISRNotificationUsage for SRNotificationUsage {}
pub trait ISRNotificationUsage: Sized + std::ops::Deref {
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn event(&self) -> SRNotificationEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, event)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRWebUsage(pub id);
impl std::ops::Deref for SRWebUsage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRWebUsage {}
impl SRWebUsage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRWebUsage").unwrap(), alloc) })
    }
}
impl INSObject for SRWebUsage {}
impl PNSObject for SRWebUsage {}
impl std::convert::TryFrom<NSObject> for SRWebUsage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRWebUsage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRWebUsage").unwrap()) };
        if is_kind_of {
            Ok(SRWebUsage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRWebUsage")
        }
    }
}
impl ISRWebUsage for SRWebUsage {}
pub trait ISRWebUsage: Sized + std::ops::Deref {
    unsafe fn totalUsageTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalUsageTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRMessagesUsageReport(pub id);
impl std::ops::Deref for SRMessagesUsageReport {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRMessagesUsageReport {}
impl SRMessagesUsageReport {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRMessagesUsageReport").unwrap(), alloc) })
    }
}
impl INSObject for SRMessagesUsageReport {}
impl PNSObject for SRMessagesUsageReport {}
impl std::convert::TryFrom<NSObject> for SRMessagesUsageReport {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRMessagesUsageReport, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRMessagesUsageReport").unwrap()) };
        if is_kind_of {
            Ok(SRMessagesUsageReport(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRMessagesUsageReport")
        }
    }
}
impl ISRMessagesUsageReport for SRMessagesUsageReport {}
pub trait ISRMessagesUsageReport: Sized + std::ops::Deref {
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn totalOutgoingMessages(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalOutgoingMessages)
    }
    unsafe fn totalIncomingMessages(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalIncomingMessages)
    }
    unsafe fn totalUniqueContacts(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalUniqueContacts)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRPhoneUsageReport(pub id);
impl std::ops::Deref for SRPhoneUsageReport {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRPhoneUsageReport {}
impl SRPhoneUsageReport {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRPhoneUsageReport").unwrap(), alloc) })
    }
}
impl INSObject for SRPhoneUsageReport {}
impl PNSObject for SRPhoneUsageReport {}
impl std::convert::TryFrom<NSObject> for SRPhoneUsageReport {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRPhoneUsageReport, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRPhoneUsageReport").unwrap()) };
        if is_kind_of {
            Ok(SRPhoneUsageReport(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRPhoneUsageReport")
        }
    }
}
impl ISRPhoneUsageReport for SRPhoneUsageReport {}
pub trait ISRPhoneUsageReport: Sized + std::ops::Deref {
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn totalOutgoingCalls(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalOutgoingCalls)
    }
    unsafe fn totalIncomingCalls(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalIncomingCalls)
    }
    unsafe fn totalUniqueContacts(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalUniqueContacts)
    }
    unsafe fn totalPhoneCallDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalPhoneCallDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRKeyboardMetrics(pub id);
impl std::ops::Deref for SRKeyboardMetrics {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRKeyboardMetrics {}
impl SRKeyboardMetrics {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRKeyboardMetrics").unwrap(), alloc) })
    }
}
impl INSObject for SRKeyboardMetrics {}
impl PNSObject for SRKeyboardMetrics {}
impl std::convert::TryFrom<NSObject> for SRKeyboardMetrics {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRKeyboardMetrics, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRKeyboardMetrics").unwrap()) };
        if is_kind_of {
            Ok(SRKeyboardMetrics(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRKeyboardMetrics")
        }
    }
}
impl ISRKeyboardMetrics for SRKeyboardMetrics {}
pub trait ISRKeyboardMetrics: Sized + std::ops::Deref {
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn keyboardIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyboardIdentifier)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn width(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn inputModes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputModes)
    }
    unsafe fn sessionIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionIdentifiers)
    }
}
impl SRKeyboardMetrics_ScalarMetrics for SRKeyboardMetrics {}
pub trait SRKeyboardMetrics_ScalarMetrics: Sized + std::ops::Deref {
    unsafe fn totalWords(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalWords)
    }
    unsafe fn totalAlteredWords(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalAlteredWords)
    }
    unsafe fn totalTaps(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalTaps)
    }
    unsafe fn totalDrags(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalDrags)
    }
    unsafe fn totalDeletes(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalDeletes)
    }
    unsafe fn totalEmojis(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalEmojis)
    }
    unsafe fn totalPaths(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalPaths)
    }
    unsafe fn totalPathTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalPathTime)
    }
    unsafe fn totalPathLength(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalPathLength)
    }
    unsafe fn totalAutoCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalAutoCorrections)
    }
    unsafe fn totalSpaceCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalSpaceCorrections)
    }
    unsafe fn totalRetroCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalRetroCorrections)
    }
    unsafe fn totalTranspositionCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalTranspositionCorrections)
    }
    unsafe fn totalInsertKeyCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalInsertKeyCorrections)
    }
    unsafe fn totalSkipTouchCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalSkipTouchCorrections)
    }
    unsafe fn totalNearKeyCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalNearKeyCorrections)
    }
    unsafe fn totalSubstitutionCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalSubstitutionCorrections)
    }
    unsafe fn totalHitTestCorrections(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalHitTestCorrections)
    }
    unsafe fn totalTypingDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalTypingDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRKeyboardProbabilityMetric(pub id);
impl std::ops::Deref for SRKeyboardProbabilityMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRKeyboardProbabilityMetric {}
impl SRKeyboardProbabilityMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRKeyboardProbabilityMetric").unwrap(), alloc) })
    }
}
impl INSObject for SRKeyboardProbabilityMetric {}
impl PNSObject for SRKeyboardProbabilityMetric {}
impl std::convert::TryFrom<NSObject> for SRKeyboardProbabilityMetric {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRKeyboardProbabilityMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRKeyboardProbabilityMetric").unwrap()) };
        if is_kind_of {
            Ok(SRKeyboardProbabilityMetric(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRKeyboardProbabilityMetric")
        }
    }
}
impl<UnitType: 'static> ISRKeyboardProbabilityMetric<UnitType> for SRKeyboardProbabilityMetric {}
pub trait ISRKeyboardProbabilityMetric<UnitType: 'static>: Sized + std::ops::Deref {
    unsafe fn distributionSampleValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distributionSampleValues)
    }
}
impl SRKeyboardMetrics_ProbabilityMetrics for SRKeyboardMetrics {}
pub trait SRKeyboardMetrics_ProbabilityMetrics: Sized + std::ops::Deref {
    unsafe fn upErrorDistance(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upErrorDistance)
    }
    unsafe fn downErrorDistance(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downErrorDistance)
    }
    unsafe fn spaceUpErrorDistance(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceUpErrorDistance)
    }
    unsafe fn spaceDownErrorDistance(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceDownErrorDistance)
    }
    unsafe fn deleteUpErrorDistance(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteUpErrorDistance)
    }
    unsafe fn deleteDownErrorDistance(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteDownErrorDistance)
    }
    unsafe fn shortWordCharKeyUpErrorDistance(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shortWordCharKeyUpErrorDistance)
    }
    unsafe fn shortWordCharKeyDownErrorDistance(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shortWordCharKeyDownErrorDistance)
    }
    unsafe fn touchDownUp(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchDownUp)
    }
    unsafe fn spaceTouchDownUp(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceTouchDownUp)
    }
    unsafe fn deleteTouchDownUp(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteTouchDownUp)
    }
    unsafe fn shortWordCharKeyTouchDownUp(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shortWordCharKeyTouchDownUp)
    }
    unsafe fn touchDownDown(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchDownDown)
    }
    unsafe fn touchUpDown(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchUpDown)
    }
    unsafe fn charKeyToPrediction(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charKeyToPrediction)
    }
    unsafe fn shortWordCharKeyToCharKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shortWordCharKeyToCharKey)
    }
    unsafe fn charKeyToAnyTapKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charKeyToAnyTapKey)
    }
    unsafe fn anyTapToCharKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anyTapToCharKey)
    }
    unsafe fn spaceToCharKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceToCharKey)
    }
    unsafe fn charKeyToSpaceKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charKeyToSpaceKey)
    }
    unsafe fn spaceToDeleteKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceToDeleteKey)
    }
    unsafe fn deleteToSpaceKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteToSpaceKey)
    }
    unsafe fn spaceToSpaceKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceToSpaceKey)
    }
    unsafe fn spaceToShiftKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceToShiftKey)
    }
    unsafe fn spaceToPlaneChangeKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceToPlaneChangeKey)
    }
    unsafe fn spaceToPredictionKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceToPredictionKey)
    }
    unsafe fn deleteToCharKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteToCharKey)
    }
    unsafe fn charKeyToDelete(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charKeyToDelete)
    }
    unsafe fn deleteToDelete(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteToDelete)
    }
    unsafe fn deleteToShiftKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteToShiftKey)
    }
    unsafe fn deleteToPlaneChangeKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteToPlaneChangeKey)
    }
    unsafe fn anyTapToPlaneChangeKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anyTapToPlaneChangeKey)
    }
    unsafe fn planeChangeToAnyTap(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, planeChangeToAnyTap)
    }
    unsafe fn charKeyToPlaneChangeKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charKeyToPlaneChangeKey)
    }
    unsafe fn planeChangeKeyToCharKey(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, planeChangeKeyToCharKey)
    }
    unsafe fn pathErrorDistanceRatio(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathErrorDistanceRatio)
    }
    unsafe fn deleteToPath(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteToPath)
    }
    unsafe fn pathToDelete(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathToDelete)
    }
    unsafe fn spaceToPath(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spaceToPath)
    }
    unsafe fn pathToSpace(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathToSpace)
    }
    unsafe fn pathToPath(&self) -> SRKeyboardProbabilityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathToPath)
    }
}
impl SRKeyboardMetrics_PositionalMetrics for SRKeyboardMetrics {}
pub trait SRKeyboardMetrics_PositionalMetrics: Sized + std::ops::Deref {
    unsafe fn longWordUpErrorDistance(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longWordUpErrorDistance)
    }
    unsafe fn longWordDownErrorDistance(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longWordDownErrorDistance)
    }
    unsafe fn longWordTouchDownUp(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longWordTouchDownUp)
    }
    unsafe fn longWordTouchDownDown(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longWordTouchDownDown)
    }
    unsafe fn longWordTouchUpDown(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longWordTouchUpDown)
    }
    unsafe fn deleteToDeletes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteToDeletes)
    }
}
impl SRKeyboardMetrics_SpeedMetrics for SRKeyboardMetrics {}
pub trait SRKeyboardMetrics_SpeedMetrics: Sized + std::ops::Deref {
    unsafe fn totalPauses(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalPauses)
    }
    unsafe fn totalPathPauses(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalPathPauses)
    }
    unsafe fn typingSpeed(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typingSpeed)
    }
    unsafe fn pathTypingSpeed(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathTypingSpeed)
    }
    unsafe fn totalTypingEpisodes(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalTypingEpisodes)
    }
}
pub type SRKeyboardMetricsSentimentCategory = NSInteger;
impl SRKeyboardMetrics_SentimentCounts for SRKeyboardMetrics {}
pub trait SRKeyboardMetrics_SentimentCounts: Sized + std::ops::Deref {
    unsafe fn wordCountForSentimentCategory_(
        &self,
        category: SRKeyboardMetricsSentimentCategory,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, wordCountForSentimentCategory : category)
    }
    unsafe fn emojiCountForSentimentCategory_(
        &self,
        category: SRKeyboardMetricsSentimentCategory,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, emojiCountForSentimentCategory : category)
    }
}
pub type SRDeletionReason = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRDeletionRecord(pub id);
impl std::ops::Deref for SRDeletionRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRDeletionRecord {}
impl SRDeletionRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRDeletionRecord").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SRDeletionRecord {}
impl INSObject for SRDeletionRecord {}
impl PNSObject for SRDeletionRecord {}
impl std::convert::TryFrom<NSObject> for SRDeletionRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRDeletionRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRDeletionRecord").unwrap()) };
        if is_kind_of {
            Ok(SRDeletionRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRDeletionRecord")
        }
    }
}
impl ISRDeletionRecord for SRDeletionRecord {}
pub trait ISRDeletionRecord: Sized + std::ops::Deref {
    unsafe fn startTime(&self) -> SRAbsoluteTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startTime)
    }
    unsafe fn endTime(&self) -> SRAbsoluteTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endTime)
    }
    unsafe fn reason(&self) -> SRDeletionReason
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reason)
    }
}
pub trait NSString_SRDeletionRecord: Sized + std::ops::Deref {
    unsafe fn sr_sensorForDeletionRecordsFromSensor(&self) -> SRSensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sr_sensorForDeletionRecordsFromSensor)
    }
}
pub type SRWristLocation = NSInteger;
pub type SRCrownOrientation = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRWristDetection(pub id);
impl std::ops::Deref for SRWristDetection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRWristDetection {}
impl SRWristDetection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRWristDetection").unwrap(), alloc) })
    }
}
impl INSObject for SRWristDetection {}
impl PNSObject for SRWristDetection {}
impl std::convert::TryFrom<NSObject> for SRWristDetection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRWristDetection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRWristDetection").unwrap()) };
        if is_kind_of {
            Ok(SRWristDetection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRWristDetection")
        }
    }
}
impl ISRWristDetection for SRWristDetection {}
pub trait ISRWristDetection: Sized + std::ops::Deref {
    unsafe fn onWrist(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, onWrist)
    }
    unsafe fn wristLocation(&self) -> SRWristLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wristLocation)
    }
    unsafe fn crownOrientation(&self) -> SRCrownOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crownOrientation)
    }
    unsafe fn onWristDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, onWristDate)
    }
    unsafe fn offWristDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offWristDate)
    }
}
pub type SRWristTemperatureCondition = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRWristTemperature(pub id);
impl std::ops::Deref for SRWristTemperature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRWristTemperature {}
impl SRWristTemperature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRWristTemperature").unwrap(), alloc) })
    }
}
impl PNSCopying for SRWristTemperature {}
impl PNSSecureCoding for SRWristTemperature {}
impl INSObject for SRWristTemperature {}
impl PNSObject for SRWristTemperature {}
impl std::convert::TryFrom<NSObject> for SRWristTemperature {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRWristTemperature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRWristTemperature").unwrap()) };
        if is_kind_of {
            Ok(SRWristTemperature(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRWristTemperature")
        }
    }
}
impl ISRWristTemperature for SRWristTemperature {}
pub trait ISRWristTemperature: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn timestamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn value(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn condition(&self) -> SRWristTemperatureCondition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, condition)
    }
    unsafe fn errorEstimate(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorEstimate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRWristTemperature").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRWristTemperatureSession(pub id);
impl std::ops::Deref for SRWristTemperatureSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRWristTemperatureSession {}
impl SRWristTemperatureSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRWristTemperatureSession").unwrap(), alloc) })
    }
}
impl PNSCopying for SRWristTemperatureSession {}
impl PNSSecureCoding for SRWristTemperatureSession {}
impl INSObject for SRWristTemperatureSession {}
impl PNSObject for SRWristTemperatureSession {}
impl std::convert::TryFrom<NSObject> for SRWristTemperatureSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRWristTemperatureSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRWristTemperatureSession").unwrap()) };
        if is_kind_of {
            Ok(SRWristTemperatureSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRWristTemperatureSession")
        }
    }
}
impl ISRWristTemperatureSession for SRWristTemperatureSession {}
pub trait ISRWristTemperatureSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn temperatures(&self) -> NSEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, temperatures)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRWristTemperatureSession").unwrap(), new)
    }
}
pub type SRMediaEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRMediaEvent(pub id);
impl std::ops::Deref for SRMediaEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRMediaEvent {}
impl SRMediaEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRMediaEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for SRMediaEvent {}
impl PNSSecureCoding for SRMediaEvent {}
impl INSObject for SRMediaEvent {}
impl PNSObject for SRMediaEvent {}
impl std::convert::TryFrom<NSObject> for SRMediaEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRMediaEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRMediaEvent").unwrap()) };
        if is_kind_of {
            Ok(SRMediaEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRMediaEvent")
        }
    }
}
impl ISRMediaEvent for SRMediaEvent {}
pub trait ISRMediaEvent: Sized + std::ops::Deref {
    unsafe fn mediaIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIdentifier)
    }
    unsafe fn eventType(&self) -> SRMediaEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRSpeechExpression(pub id);
impl std::ops::Deref for SRSpeechExpression {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRSpeechExpression {}
impl SRSpeechExpression {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRSpeechExpression").unwrap(), alloc) })
    }
}
impl PNSCopying for SRSpeechExpression {}
impl PNSSecureCoding for SRSpeechExpression {}
impl INSObject for SRSpeechExpression {}
impl PNSObject for SRSpeechExpression {}
impl std::convert::TryFrom<NSObject> for SRSpeechExpression {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRSpeechExpression, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRSpeechExpression").unwrap()) };
        if is_kind_of {
            Ok(SRSpeechExpression(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRSpeechExpression")
        }
    }
}
impl ISRSpeechExpression for SRSpeechExpression {}
pub trait ISRSpeechExpression: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn timeRange(&self) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRange)
    }
    unsafe fn confidence(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
    unsafe fn mood(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mood)
    }
    unsafe fn valence(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valence)
    }
    unsafe fn activation(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activation)
    }
    unsafe fn dominance(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dominance)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRSpeechExpression").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRAudioLevel(pub id);
impl std::ops::Deref for SRAudioLevel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRAudioLevel {}
impl SRAudioLevel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRAudioLevel").unwrap(), alloc) })
    }
}
impl PNSCopying for SRAudioLevel {}
impl PNSSecureCoding for SRAudioLevel {}
impl INSObject for SRAudioLevel {}
impl PNSObject for SRAudioLevel {}
impl std::convert::TryFrom<NSObject> for SRAudioLevel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRAudioLevel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRAudioLevel").unwrap()) };
        if is_kind_of {
            Ok(SRAudioLevel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRAudioLevel")
        }
    }
}
impl ISRAudioLevel for SRAudioLevel {}
pub trait ISRAudioLevel: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn timeRange(&self) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRange)
    }
    unsafe fn loudness(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loudness)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRAudioLevel").unwrap(), new)
    }
}
pub type SRSpeechMetricsSessionFlags = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRSpeechMetrics(pub id);
impl std::ops::Deref for SRSpeechMetrics {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRSpeechMetrics {}
impl SRSpeechMetrics {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRSpeechMetrics").unwrap(), alloc) })
    }
}
impl PNSCopying for SRSpeechMetrics {}
impl PNSSecureCoding for SRSpeechMetrics {}
impl INSObject for SRSpeechMetrics {}
impl PNSObject for SRSpeechMetrics {}
impl std::convert::TryFrom<NSObject> for SRSpeechMetrics {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRSpeechMetrics, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRSpeechMetrics").unwrap()) };
        if is_kind_of {
            Ok(SRSpeechMetrics(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRSpeechMetrics")
        }
    }
}
impl ISRSpeechMetrics for SRSpeechMetrics {}
pub trait ISRSpeechMetrics: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sessionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionIdentifier)
    }
    unsafe fn sessionFlags(&self) -> SRSpeechMetricsSessionFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionFlags)
    }
    unsafe fn timestamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn timeSinceAudioStart(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeSinceAudioStart)
    }
    unsafe fn audioLevel(&self) -> SRAudioLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioLevel)
    }
    unsafe fn speechRecognition(&self) -> SFSpeechRecognitionResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechRecognition)
    }
    unsafe fn soundClassification(&self) -> SNClassificationResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, soundClassification)
    }
    unsafe fn speechExpression(&self) -> SRSpeechExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechExpression)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRSpeechMetrics").unwrap(), new)
    }
}
pub type SRFaceMetricsContext = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRFaceMetricsExpression(pub id);
impl std::ops::Deref for SRFaceMetricsExpression {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRFaceMetricsExpression {}
impl SRFaceMetricsExpression {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRFaceMetricsExpression").unwrap(), alloc) })
    }
}
impl PNSCopying for SRFaceMetricsExpression {}
impl PNSSecureCoding for SRFaceMetricsExpression {}
impl INSObject for SRFaceMetricsExpression {}
impl PNSObject for SRFaceMetricsExpression {}
impl std::convert::TryFrom<NSObject> for SRFaceMetricsExpression {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRFaceMetricsExpression, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRFaceMetricsExpression").unwrap()) };
        if is_kind_of {
            Ok(SRFaceMetricsExpression(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRFaceMetricsExpression")
        }
    }
}
impl ISRFaceMetricsExpression for SRFaceMetricsExpression {}
pub trait ISRFaceMetricsExpression: Sized + std::ops::Deref {
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
    unsafe fn value(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRFaceMetricsExpression").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRFaceMetrics(pub id);
impl std::ops::Deref for SRFaceMetrics {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRFaceMetrics {}
impl SRFaceMetrics {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRFaceMetrics").unwrap(), alloc) })
    }
}
impl PNSCopying for SRFaceMetrics {}
impl PNSSecureCoding for SRFaceMetrics {}
impl INSObject for SRFaceMetrics {}
impl PNSObject for SRFaceMetrics {}
impl std::convert::TryFrom<NSObject> for SRFaceMetrics {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRFaceMetrics, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRFaceMetrics").unwrap()) };
        if is_kind_of {
            Ok(SRFaceMetrics(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRFaceMetrics")
        }
    }
}
impl ISRFaceMetrics for SRFaceMetrics {}
pub trait ISRFaceMetrics: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn sessionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionIdentifier)
    }
    unsafe fn context(&self) -> SRFaceMetricsContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn wholeFaceExpressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wholeFaceExpressions)
    }
    unsafe fn partialFaceExpressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, partialFaceExpressions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRFaceMetrics").unwrap(), new)
    }
}
pub type SRElectrocardiogramSessionState = NSInteger;
pub type SRElectrocardiogramSessionGuidance = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRElectrocardiogramSession(pub id);
impl std::ops::Deref for SRElectrocardiogramSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRElectrocardiogramSession {}
impl SRElectrocardiogramSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRElectrocardiogramSession").unwrap(), alloc) })
    }
}
impl PNSCopying for SRElectrocardiogramSession {}
impl PNSSecureCoding for SRElectrocardiogramSession {}
impl INSObject for SRElectrocardiogramSession {}
impl PNSObject for SRElectrocardiogramSession {}
impl std::convert::TryFrom<NSObject> for SRElectrocardiogramSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRElectrocardiogramSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRElectrocardiogramSession").unwrap()) };
        if is_kind_of {
            Ok(SRElectrocardiogramSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRElectrocardiogramSession")
        }
    }
}
impl ISRElectrocardiogramSession for SRElectrocardiogramSession {}
pub trait ISRElectrocardiogramSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn state(&self) -> SRElectrocardiogramSessionState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn sessionGuidance(&self) -> SRElectrocardiogramSessionGuidance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionGuidance)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRElectrocardiogramSession").unwrap(), new)
    }
}
pub type SRElectrocardiogramDataFlags = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRElectrocardiogramData(pub id);
impl std::ops::Deref for SRElectrocardiogramData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRElectrocardiogramData {}
impl SRElectrocardiogramData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRElectrocardiogramData").unwrap(), alloc) })
    }
}
impl PNSCopying for SRElectrocardiogramData {}
impl PNSSecureCoding for SRElectrocardiogramData {}
impl INSObject for SRElectrocardiogramData {}
impl PNSObject for SRElectrocardiogramData {}
impl std::convert::TryFrom<NSObject> for SRElectrocardiogramData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRElectrocardiogramData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRElectrocardiogramData").unwrap()) };
        if is_kind_of {
            Ok(SRElectrocardiogramData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRElectrocardiogramData")
        }
    }
}
impl ISRElectrocardiogramData for SRElectrocardiogramData {}
pub trait ISRElectrocardiogramData: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn flags(&self) -> SRElectrocardiogramDataFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flags)
    }
    unsafe fn value(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRElectrocardiogramData").unwrap(), new)
    }
}
pub type SRElectrocardiogramLead = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRElectrocardiogramSample(pub id);
impl std::ops::Deref for SRElectrocardiogramSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRElectrocardiogramSample {}
impl SRElectrocardiogramSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRElectrocardiogramSample").unwrap(), alloc) })
    }
}
impl PNSCopying for SRElectrocardiogramSample {}
impl PNSSecureCoding for SRElectrocardiogramSample {}
impl INSObject for SRElectrocardiogramSample {}
impl PNSObject for SRElectrocardiogramSample {}
impl std::convert::TryFrom<NSObject> for SRElectrocardiogramSample {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRElectrocardiogramSample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRElectrocardiogramSample").unwrap()) };
        if is_kind_of {
            Ok(SRElectrocardiogramSample(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRElectrocardiogramSample")
        }
    }
}
impl ISRElectrocardiogramSample for SRElectrocardiogramSample {}
pub trait ISRElectrocardiogramSample: Sized + std::ops::Deref {
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
    unsafe fn frequency(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn session(&self) -> SRElectrocardiogramSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, session)
    }
    unsafe fn lead(&self) -> SRElectrocardiogramLead
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lead)
    }
    unsafe fn data(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRElectrocardiogramSample").unwrap(), new)
    }
}
pub type SRPhotoplethysmogramOpticalSampleCondition = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRPhotoplethysmogramOpticalSample(pub id);
impl std::ops::Deref for SRPhotoplethysmogramOpticalSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRPhotoplethysmogramOpticalSample {}
impl SRPhotoplethysmogramOpticalSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramOpticalSample").unwrap(), alloc) })
    }
}
impl PNSCopying for SRPhotoplethysmogramOpticalSample {}
impl PNSSecureCoding for SRPhotoplethysmogramOpticalSample {}
impl INSObject for SRPhotoplethysmogramOpticalSample {}
impl PNSObject for SRPhotoplethysmogramOpticalSample {}
impl std::convert::TryFrom<NSObject> for SRPhotoplethysmogramOpticalSample {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRPhotoplethysmogramOpticalSample, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramOpticalSample").unwrap())
        };
        if is_kind_of {
            Ok(SRPhotoplethysmogramOpticalSample(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRPhotoplethysmogramOpticalSample")
        }
    }
}
impl ISRPhotoplethysmogramOpticalSample for SRPhotoplethysmogramOpticalSample {}
pub trait ISRPhotoplethysmogramOpticalSample: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn emitter(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitter)
    }
    unsafe fn activePhotodiodeIndexes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activePhotodiodeIndexes)
    }
    unsafe fn signalIdentifier(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signalIdentifier)
    }
    unsafe fn nominalWavelength(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nominalWavelength)
    }
    unsafe fn effectiveWavelength(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectiveWavelength)
    }
    unsafe fn samplingFrequency(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, samplingFrequency)
    }
    unsafe fn nanosecondsSinceStart(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nanosecondsSinceStart)
    }
    unsafe fn normalizedReflectance(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedReflectance)
    }
    unsafe fn whiteNoise(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, whiteNoise)
    }
    unsafe fn pinkNoise(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pinkNoise)
    }
    unsafe fn backgroundNoise(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundNoise)
    }
    unsafe fn backgroundNoiseOffset(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundNoiseOffset)
    }
    unsafe fn conditions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conditions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramOpticalSample").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRPhotoplethysmogramAccelerometerSample(pub id);
impl std::ops::Deref for SRPhotoplethysmogramAccelerometerSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRPhotoplethysmogramAccelerometerSample {}
impl SRPhotoplethysmogramAccelerometerSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramAccelerometerSample").unwrap(), alloc) })
    }
}
impl PNSCopying for SRPhotoplethysmogramAccelerometerSample {}
impl PNSSecureCoding for SRPhotoplethysmogramAccelerometerSample {}
impl INSObject for SRPhotoplethysmogramAccelerometerSample {}
impl PNSObject for SRPhotoplethysmogramAccelerometerSample {}
impl std::convert::TryFrom<NSObject> for SRPhotoplethysmogramAccelerometerSample {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRPhotoplethysmogramAccelerometerSample, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramAccelerometerSample").unwrap())
        };
        if is_kind_of {
            Ok(SRPhotoplethysmogramAccelerometerSample(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRPhotoplethysmogramAccelerometerSample")
        }
    }
}
impl ISRPhotoplethysmogramAccelerometerSample for SRPhotoplethysmogramAccelerometerSample {}
pub trait ISRPhotoplethysmogramAccelerometerSample: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn nanosecondsSinceStart(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nanosecondsSinceStart)
    }
    unsafe fn samplingFrequency(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, samplingFrequency)
    }
    unsafe fn x(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, x)
    }
    unsafe fn y(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, y)
    }
    unsafe fn z(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, z)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramAccelerometerSample").unwrap(), new)
    }
}
pub type SRPhotoplethysmogramSampleUsage = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRPhotoplethysmogramSample(pub id);
impl std::ops::Deref for SRPhotoplethysmogramSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRPhotoplethysmogramSample {}
impl SRPhotoplethysmogramSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramSample").unwrap(), alloc) })
    }
}
impl PNSCopying for SRPhotoplethysmogramSample {}
impl PNSSecureCoding for SRPhotoplethysmogramSample {}
impl INSObject for SRPhotoplethysmogramSample {}
impl PNSObject for SRPhotoplethysmogramSample {}
impl std::convert::TryFrom<NSObject> for SRPhotoplethysmogramSample {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRPhotoplethysmogramSample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramSample").unwrap()) };
        if is_kind_of {
            Ok(SRPhotoplethysmogramSample(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRPhotoplethysmogramSample")
        }
    }
}
impl ISRPhotoplethysmogramSample for SRPhotoplethysmogramSample {}
pub trait ISRPhotoplethysmogramSample: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn nanosecondsSinceStart(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nanosecondsSinceStart)
    }
    unsafe fn usage(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usage)
    }
    unsafe fn opticalSamples(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opticalSamples)
    }
    unsafe fn accelerometerSamples(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accelerometerSamples)
    }
    unsafe fn temperature(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, temperature)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRPhotoplethysmogramSample").unwrap(), new)
    }
}
pub type SRAcousticSettingsSampleLifetime = NSInteger;
pub type SRAcousticSettingsAccessibilityBackgroundSoundsName = NSInteger;
pub type SRAcousticSettingsAccessibilityHeadphoneAccommodationsMediaEnhanceTuning = NSInteger;
pub type SRAcousticSettingsAccessibilityHeadphoneAccommodationsMediaEnhanceBoosting = NSInteger;
pub type SRAcousticSettingsAccessibilityHeadphoneAccommodationsMediaEnhanceApplication = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRAcousticSettingsMusicEQ(pub id);
impl std::ops::Deref for SRAcousticSettingsMusicEQ {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRAcousticSettingsMusicEQ {}
impl SRAcousticSettingsMusicEQ {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRAcousticSettingsMusicEQ").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SRAcousticSettingsMusicEQ {}
impl PNSCopying for SRAcousticSettingsMusicEQ {}
impl INSObject for SRAcousticSettingsMusicEQ {}
impl PNSObject for SRAcousticSettingsMusicEQ {}
impl std::convert::TryFrom<NSObject> for SRAcousticSettingsMusicEQ {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRAcousticSettingsMusicEQ, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRAcousticSettingsMusicEQ").unwrap()) };
        if is_kind_of {
            Ok(SRAcousticSettingsMusicEQ(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRAcousticSettingsMusicEQ")
        }
    }
}
impl ISRAcousticSettingsMusicEQ for SRAcousticSettingsMusicEQ {}
pub trait ISRAcousticSettingsMusicEQ: Sized + std::ops::Deref {
    unsafe fn isSoundCheckEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSoundCheckEnabled)
    }
    unsafe fn isLateNightModeEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLateNightModeEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRAcousticSettingsAccessibilityBackgroundSounds(pub id);
impl std::ops::Deref for SRAcousticSettingsAccessibilityBackgroundSounds {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRAcousticSettingsAccessibilityBackgroundSounds {}
impl SRAcousticSettingsAccessibilityBackgroundSounds {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"SRAcousticSettingsAccessibilityBackgroundSounds").unwrap(), alloc)
        })
    }
}
impl PNSSecureCoding for SRAcousticSettingsAccessibilityBackgroundSounds {}
impl PNSCopying for SRAcousticSettingsAccessibilityBackgroundSounds {}
impl INSObject for SRAcousticSettingsAccessibilityBackgroundSounds {}
impl PNSObject for SRAcousticSettingsAccessibilityBackgroundSounds {}
impl std::convert::TryFrom<NSObject> for SRAcousticSettingsAccessibilityBackgroundSounds {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<SRAcousticSettingsAccessibilityBackgroundSounds, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRAcousticSettingsAccessibilityBackgroundSounds").unwrap())
        };
        if is_kind_of {
            Ok(SRAcousticSettingsAccessibilityBackgroundSounds(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to SRAcousticSettingsAccessibilityBackgroundSounds" ,)
        }
    }
}
impl ISRAcousticSettingsAccessibilityBackgroundSounds
    for SRAcousticSettingsAccessibilityBackgroundSounds
{
}
pub trait ISRAcousticSettingsAccessibilityBackgroundSounds: Sized + std::ops::Deref {
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn soundName(&self) -> SRAcousticSettingsAccessibilityBackgroundSoundsName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, soundName)
    }
    unsafe fn relativeVolume(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeVolume)
    }
    unsafe fn isPlayWithMediaEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPlayWithMediaEnabled)
    }
    unsafe fn relativeVolumeWithMedia(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeVolumeWithMedia)
    }
    unsafe fn isStopOnLockEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStopOnLockEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRAcousticSettingsAccessibilityHeadphoneAccommodations(pub id);
impl std::ops::Deref for SRAcousticSettingsAccessibilityHeadphoneAccommodations {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRAcousticSettingsAccessibilityHeadphoneAccommodations {}
impl SRAcousticSettingsAccessibilityHeadphoneAccommodations {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"SRAcousticSettingsAccessibilityHeadphoneAccommodations").unwrap(), alloc)
        })
    }
}
impl PNSSecureCoding for SRAcousticSettingsAccessibilityHeadphoneAccommodations {}
impl PNSCopying for SRAcousticSettingsAccessibilityHeadphoneAccommodations {}
impl INSObject for SRAcousticSettingsAccessibilityHeadphoneAccommodations {}
impl PNSObject for SRAcousticSettingsAccessibilityHeadphoneAccommodations {}
impl std::convert::TryFrom<NSObject> for SRAcousticSettingsAccessibilityHeadphoneAccommodations {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<SRAcousticSettingsAccessibilityHeadphoneAccommodations, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRAcousticSettingsAccessibilityHeadphoneAccommodations").unwrap())
        };
        if is_kind_of {
            Ok(SRAcousticSettingsAccessibilityHeadphoneAccommodations(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to SRAcousticSettingsAccessibilityHeadphoneAccommodations" ,)
        }
    }
}
impl ISRAcousticSettingsAccessibilityHeadphoneAccommodations
    for SRAcousticSettingsAccessibilityHeadphoneAccommodations
{
}
pub trait ISRAcousticSettingsAccessibilityHeadphoneAccommodations: Sized + std::ops::Deref {
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn mediaEnhanceTuning(
        &self,
    ) -> SRAcousticSettingsAccessibilityHeadphoneAccommodationsMediaEnhanceTuning
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaEnhanceTuning)
    }
    unsafe fn mediaEnhanceBoosting(
        &self,
    ) -> SRAcousticSettingsAccessibilityHeadphoneAccommodationsMediaEnhanceBoosting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaEnhanceBoosting)
    }
    unsafe fn mediaEnhanceApplication(
        &self,
    ) -> SRAcousticSettingsAccessibilityHeadphoneAccommodationsMediaEnhanceApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaEnhanceApplication)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRAcousticSettingsAccessibility(pub id);
impl std::ops::Deref for SRAcousticSettingsAccessibility {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRAcousticSettingsAccessibility {}
impl SRAcousticSettingsAccessibility {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRAcousticSettingsAccessibility").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SRAcousticSettingsAccessibility {}
impl PNSCopying for SRAcousticSettingsAccessibility {}
impl INSObject for SRAcousticSettingsAccessibility {}
impl PNSObject for SRAcousticSettingsAccessibility {}
impl std::convert::TryFrom<NSObject> for SRAcousticSettingsAccessibility {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRAcousticSettingsAccessibility, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRAcousticSettingsAccessibility").unwrap())
        };
        if is_kind_of {
            Ok(SRAcousticSettingsAccessibility(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRAcousticSettingsAccessibility")
        }
    }
}
impl ISRAcousticSettingsAccessibility for SRAcousticSettingsAccessibility {}
pub trait ISRAcousticSettingsAccessibility: Sized + std::ops::Deref {
    unsafe fn leftRightBalance(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftRightBalance)
    }
    unsafe fn isMonoAudioEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMonoAudioEnabled)
    }
    unsafe fn backgroundSounds(&self) -> SRAcousticSettingsAccessibilityBackgroundSounds
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundSounds)
    }
    unsafe fn headphoneAccommodations(
        &self,
    ) -> SRAcousticSettingsAccessibilityHeadphoneAccommodations
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headphoneAccommodations)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRAcousticSettings(pub id);
impl std::ops::Deref for SRAcousticSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRAcousticSettings {}
impl SRAcousticSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRAcousticSettings").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SRAcousticSettings {}
impl PNSCopying for SRAcousticSettings {}
impl INSObject for SRAcousticSettings {}
impl PNSObject for SRAcousticSettings {}
impl std::convert::TryFrom<NSObject> for SRAcousticSettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRAcousticSettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRAcousticSettings").unwrap()) };
        if is_kind_of {
            Ok(SRAcousticSettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRAcousticSettings")
        }
    }
}
impl ISRAcousticSettings for SRAcousticSettings {}
pub trait ISRAcousticSettings: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isEnvironmentalSoundMeasurementsEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnvironmentalSoundMeasurementsEnabled)
    }
    unsafe fn audioExposureSampleLifetime(&self) -> SRAcousticSettingsSampleLifetime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioExposureSampleLifetime)
    }
    unsafe fn headphoneSafetyAudioLevel(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headphoneSafetyAudioLevel)
    }
    unsafe fn musicEQSettings(&self) -> SRAcousticSettingsMusicEQ
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, musicEQSettings)
    }
    unsafe fn accessibilitySettings(&self) -> SRAcousticSettingsAccessibility
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilitySettings)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRAcousticSettings").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SRSleepSession(pub id);
impl std::ops::Deref for SRSleepSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SRSleepSession {}
impl SRSleepSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SRSleepSession").unwrap(), alloc) })
    }
}
impl PNSCopying for SRSleepSession {}
impl PNSSecureCoding for SRSleepSession {}
impl INSObject for SRSleepSession {}
impl PNSObject for SRSleepSession {}
impl std::convert::TryFrom<NSObject> for SRSleepSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SRSleepSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SRSleepSession").unwrap()) };
        if is_kind_of {
            Ok(SRSleepSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SRSleepSession")
        }
    }
}
impl ISRSleepSession for SRSleepSession {}
pub trait ISRSleepSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SRSleepSession").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static SRSensorAmbientLightSensor: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorAccelerometer: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorRotationRate: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorVisits: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorPedometerData: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorDeviceUsageReport: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorMessagesUsageReport: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorPhoneUsageReport: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorOnWristState: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorKeyboardMetrics: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorSiriSpeechMetrics: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorTelephonySpeechMetrics: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorAmbientPressure: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorMediaEvents: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorWristTemperature: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorHeartRate: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorFaceMetrics: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorOdometer: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorElectrocardiogram: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorPhotoplethysmogram: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorAcousticSettings: SRSensor;
}
unsafe extern "C" {
    pub static SRSensorSleepSessions: SRSensor;
}
unsafe extern "C" {
    pub fn SRAbsoluteTimeGetCurrent() -> SRAbsoluteTime;
}
unsafe extern "C" {
    pub fn SRAbsoluteTimeFromContinuousTime(cont: u64) -> SRAbsoluteTime;
}
unsafe extern "C" {
    pub fn SRAbsoluteTimeToCFAbsoluteTime(sr: SRAbsoluteTime) -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub fn SRAbsoluteTimeFromCFAbsoluteTime(cf: CFAbsoluteTime) -> SRAbsoluteTime;
}
unsafe extern "C" {
    pub static mut SRErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryGames: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryBusiness: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryWeather: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryUtilities: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryTravel: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategorySports: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategorySocialNetworking: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryReference: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryProductivity: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryPhotoAndVideo: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryNews: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryNavigation: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryMusic: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryLifestyle: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryHealthAndFitness: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryFinance: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryEntertainment: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryEducation: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryBooks: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryMedical: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryNewsstand: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryCatalogs: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryKids: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryMiscellaneous: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryFoodAndDrink: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryDeveloperTools: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryGraphicsAndDesign: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryShopping: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRDeviceUsageCategoryStickers: SRDeviceUsageCategoryKey;
}
unsafe extern "C" {
    pub static SRPhotoplethysmogramOpticalSampleConditionSignalSaturation:
        SRPhotoplethysmogramOpticalSampleCondition;
}
unsafe extern "C" {
    pub static SRPhotoplethysmogramOpticalSampleConditionUnreliableNoise:
        SRPhotoplethysmogramOpticalSampleCondition;
}
unsafe extern "C" {
    pub static SRPhotoplethysmogramSampleUsageForegroundHeartRate: SRPhotoplethysmogramSampleUsage;
}
unsafe extern "C" {
    pub static SRPhotoplethysmogramSampleUsageDeepBreathing: SRPhotoplethysmogramSampleUsage;
}
unsafe extern "C" {
    pub static SRPhotoplethysmogramSampleUsageForegroundBloodOxygen:
        SRPhotoplethysmogramSampleUsage;
}
unsafe extern "C" {
    pub static SRPhotoplethysmogramSampleUsageBackgroundSystem: SRPhotoplethysmogramSampleUsage;
}

unsafe impl objc2::encode::RefEncode for SRFetchResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRFetchResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRFetchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRFetchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRSensorReader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRSensorReader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRAmbientLightChromaticity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRAmbientLightChromaticity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SRAmbientLightChromaticity", &[]);
}
unsafe impl objc2::encode::RefEncode for SRAmbientLightSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRAmbientLightSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRVisit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRVisit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRSupplementalCategory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRSupplementalCategory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRDeviceUsageReport {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRDeviceUsageReport {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRTextInputSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRTextInputSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRApplicationUsage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRApplicationUsage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRNotificationUsage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRNotificationUsage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRWebUsage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRWebUsage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRMessagesUsageReport {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRMessagesUsageReport {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRPhoneUsageReport {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRPhoneUsageReport {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRKeyboardMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRKeyboardMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRKeyboardProbabilityMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRKeyboardProbabilityMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRDeletionRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRDeletionRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRWristDetection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRWristDetection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRWristTemperature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRWristTemperature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRWristTemperatureSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRWristTemperatureSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRMediaEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRMediaEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRSpeechExpression {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRSpeechExpression {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRAudioLevel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRAudioLevel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRSpeechMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRSpeechMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRFaceMetricsExpression {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRFaceMetricsExpression {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRFaceMetrics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRFaceMetrics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRElectrocardiogramSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRElectrocardiogramSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRElectrocardiogramData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRElectrocardiogramData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRElectrocardiogramSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRElectrocardiogramSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRPhotoplethysmogramOpticalSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRPhotoplethysmogramOpticalSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRPhotoplethysmogramAccelerometerSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRPhotoplethysmogramAccelerometerSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRPhotoplethysmogramSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRPhotoplethysmogramSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRAcousticSettingsMusicEQ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRAcousticSettingsMusicEQ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRAcousticSettingsAccessibilityBackgroundSounds {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRAcousticSettingsAccessibilityBackgroundSounds {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRAcousticSettingsAccessibilityHeadphoneAccommodations {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRAcousticSettingsAccessibilityHeadphoneAccommodations {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRAcousticSettingsAccessibility {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRAcousticSettingsAccessibility {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRAcousticSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRAcousticSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SRSleepSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SRSleepSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
