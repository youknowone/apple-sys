#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use libc::{id_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type os_log_t = NSObject;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXMetric(pub id);
impl std::ops::Deref for MXMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXMetric {}
impl MXMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXMetric").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXMetric {}
impl INSObject for MXMetric {}
impl PNSObject for MXMetric {}
impl std::convert::TryFrom<NSObject> for MXMetric {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXMetric, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXMetric").unwrap()) };
        if is_kind_of {
            Ok(MXMetric(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXMetric")
        }
    }
}
impl IMXMetric for MXMetric {}
pub trait IMXMetric: Sized + std::ops::Deref {
    unsafe fn JSONRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSONRepresentation)
    }
    unsafe fn DictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, DictionaryRepresentation)
    }
    unsafe fn dictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryRepresentation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXCPUMetric(pub id);
impl std::ops::Deref for MXCPUMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXCPUMetric {}
impl MXCPUMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXCPUMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXCPUMetric {}
impl PNSSecureCoding for MXCPUMetric {}
impl From<MXCPUMetric> for MXMetric {
    fn from(child: MXCPUMetric) -> MXMetric {
        MXMetric(child.0)
    }
}
impl std::convert::TryFrom<MXMetric> for MXCPUMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXCPUMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXCPUMetric").unwrap()) };
        if is_kind_of {
            Ok(MXCPUMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXCPUMetric")
        }
    }
}
impl INSObject for MXCPUMetric {}
impl PNSObject for MXCPUMetric {}
impl IMXCPUMetric for MXCPUMetric {}
pub trait IMXCPUMetric: Sized + std::ops::Deref {
    unsafe fn cumulativeCPUTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeCPUTime)
    }
    unsafe fn cumulativeCPUInstructions(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeCPUInstructions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXGPUMetric(pub id);
impl std::ops::Deref for MXGPUMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXGPUMetric {}
impl MXGPUMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXGPUMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXGPUMetric {}
impl PNSSecureCoding for MXGPUMetric {}
impl std::convert::TryFrom<MXMetric> for MXGPUMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXGPUMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXGPUMetric").unwrap()) };
        if is_kind_of {
            Ok(MXGPUMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXGPUMetric")
        }
    }
}
impl INSObject for MXGPUMetric {}
impl PNSObject for MXGPUMetric {}
impl IMXGPUMetric for MXGPUMetric {}
pub trait IMXGPUMetric: Sized + std::ops::Deref {
    unsafe fn cumulativeGPUTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeGPUTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXUnitSignalBars(pub id);
impl std::ops::Deref for MXUnitSignalBars {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXUnitSignalBars {}
impl MXUnitSignalBars {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXUnitSignalBars").unwrap(), alloc) })
    }
}
impl INSDimension for MXUnitSignalBars {}
impl PNSSecureCoding for MXUnitSignalBars {}
impl std::convert::TryFrom<NSDimension> for MXUnitSignalBars {
    type Error = &'static str;
    fn try_from(parent: NSDimension) -> Result<MXUnitSignalBars, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXUnitSignalBars").unwrap()) };
        if is_kind_of {
            Ok(MXUnitSignalBars(parent.0))
        } else {
            Err("This NSDimension cannot be downcasted to MXUnitSignalBars")
        }
    }
}
impl INSUnit for MXUnitSignalBars {}
impl PNSCopying for MXUnitSignalBars {}
impl INSObject for MXUnitSignalBars {}
impl PNSObject for MXUnitSignalBars {}
impl IMXUnitSignalBars for MXUnitSignalBars {}
pub trait IMXUnitSignalBars: Sized + std::ops::Deref {
    unsafe fn bars() -> MXUnitSignalBars
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MXUnitSignalBars").unwrap(), bars)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXUnitAveragePixelLuminance(pub id);
impl std::ops::Deref for MXUnitAveragePixelLuminance {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXUnitAveragePixelLuminance {}
impl MXUnitAveragePixelLuminance {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXUnitAveragePixelLuminance").unwrap(), alloc) })
    }
}
impl INSDimension for MXUnitAveragePixelLuminance {}
impl PNSSecureCoding for MXUnitAveragePixelLuminance {}
impl std::convert::TryFrom<NSDimension> for MXUnitAveragePixelLuminance {
    type Error = &'static str;
    fn try_from(parent: NSDimension) -> Result<MXUnitAveragePixelLuminance, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXUnitAveragePixelLuminance").unwrap()) };
        if is_kind_of {
            Ok(MXUnitAveragePixelLuminance(parent.0))
        } else {
            Err("This NSDimension cannot be downcasted to MXUnitAveragePixelLuminance")
        }
    }
}
impl INSUnit for MXUnitAveragePixelLuminance {}
impl PNSCopying for MXUnitAveragePixelLuminance {}
impl INSObject for MXUnitAveragePixelLuminance {}
impl PNSObject for MXUnitAveragePixelLuminance {}
impl IMXUnitAveragePixelLuminance for MXUnitAveragePixelLuminance {}
pub trait IMXUnitAveragePixelLuminance: Sized + std::ops::Deref {
    unsafe fn apl() -> MXUnitAveragePixelLuminance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MXUnitAveragePixelLuminance").unwrap(), apl)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXHistogramBucket(pub id);
impl std::ops::Deref for MXHistogramBucket {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXHistogramBucket {}
impl MXHistogramBucket {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXHistogramBucket").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXHistogramBucket {}
impl INSObject for MXHistogramBucket {}
impl PNSObject for MXHistogramBucket {}
impl std::convert::TryFrom<NSObject> for MXHistogramBucket {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXHistogramBucket, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXHistogramBucket").unwrap()) };
        if is_kind_of {
            Ok(MXHistogramBucket(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXHistogramBucket")
        }
    }
}
impl<UnitType: 'static> IMXHistogramBucket<UnitType> for MXHistogramBucket {}
pub trait IMXHistogramBucket<UnitType: 'static>: Sized + std::ops::Deref {
    unsafe fn bucketStart(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bucketStart)
    }
    unsafe fn bucketEnd(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bucketEnd)
    }
    unsafe fn bucketCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bucketCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXHistogram(pub id);
impl std::ops::Deref for MXHistogram {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXHistogram {}
impl MXHistogram {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXHistogram").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXHistogram {}
impl INSObject for MXHistogram {}
impl PNSObject for MXHistogram {}
impl std::convert::TryFrom<NSObject> for MXHistogram {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXHistogram, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXHistogram").unwrap()) };
        if is_kind_of {
            Ok(MXHistogram(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXHistogram")
        }
    }
}
impl<UnitType: 'static> IMXHistogram<UnitType> for MXHistogram {}
pub trait IMXHistogram<UnitType: 'static>: Sized + std::ops::Deref {
    unsafe fn totalBucketCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalBucketCount)
    }
    unsafe fn bucketEnumerator(&self) -> NSEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bucketEnumerator)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXCellularConditionMetric(pub id);
impl std::ops::Deref for MXCellularConditionMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXCellularConditionMetric {}
impl MXCellularConditionMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXCellularConditionMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXCellularConditionMetric {}
impl PNSSecureCoding for MXCellularConditionMetric {}
impl std::convert::TryFrom<MXMetric> for MXCellularConditionMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXCellularConditionMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXCellularConditionMetric").unwrap()) };
        if is_kind_of {
            Ok(MXCellularConditionMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXCellularConditionMetric")
        }
    }
}
impl INSObject for MXCellularConditionMetric {}
impl PNSObject for MXCellularConditionMetric {}
impl IMXCellularConditionMetric for MXCellularConditionMetric {}
pub trait IMXCellularConditionMetric: Sized + std::ops::Deref {
    unsafe fn histogrammedCellularConditionTime(&self) -> MXHistogram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, histogrammedCellularConditionTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXMetaData(pub id);
impl std::ops::Deref for MXMetaData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXMetaData {}
impl MXMetaData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXMetaData").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXMetaData {}
impl INSObject for MXMetaData {}
impl PNSObject for MXMetaData {}
impl std::convert::TryFrom<NSObject> for MXMetaData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXMetaData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXMetaData").unwrap()) };
        if is_kind_of {
            Ok(MXMetaData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXMetaData")
        }
    }
}
impl IMXMetaData for MXMetaData {}
pub trait IMXMetaData: Sized + std::ops::Deref {
    unsafe fn JSONRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSONRepresentation)
    }
    unsafe fn DictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, DictionaryRepresentation)
    }
    unsafe fn dictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryRepresentation)
    }
    unsafe fn regionFormat(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionFormat)
    }
    unsafe fn osVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, osVersion)
    }
    unsafe fn deviceType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceType)
    }
    unsafe fn applicationBuildVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationBuildVersion)
    }
    unsafe fn platformArchitecture(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, platformArchitecture)
    }
    unsafe fn lowPowerModeEnabled(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowPowerModeEnabled)
    }
    unsafe fn isTestFlightApp(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTestFlightApp)
    }
    unsafe fn pid(&self) -> pid_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pid)
    }
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXAppRunTimeMetric(pub id);
impl std::ops::Deref for MXAppRunTimeMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXAppRunTimeMetric {}
impl MXAppRunTimeMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXAppRunTimeMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXAppRunTimeMetric {}
impl PNSSecureCoding for MXAppRunTimeMetric {}
impl std::convert::TryFrom<MXMetric> for MXAppRunTimeMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXAppRunTimeMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXAppRunTimeMetric").unwrap()) };
        if is_kind_of {
            Ok(MXAppRunTimeMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXAppRunTimeMetric")
        }
    }
}
impl INSObject for MXAppRunTimeMetric {}
impl PNSObject for MXAppRunTimeMetric {}
impl IMXAppRunTimeMetric for MXAppRunTimeMetric {}
pub trait IMXAppRunTimeMetric: Sized + std::ops::Deref {
    unsafe fn cumulativeForegroundTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeForegroundTime)
    }
    unsafe fn cumulativeBackgroundTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeBackgroundTime)
    }
    unsafe fn cumulativeBackgroundAudioTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeBackgroundAudioTime)
    }
    unsafe fn cumulativeBackgroundLocationTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeBackgroundLocationTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXLocationActivityMetric(pub id);
impl std::ops::Deref for MXLocationActivityMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXLocationActivityMetric {}
impl MXLocationActivityMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXLocationActivityMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXLocationActivityMetric {}
impl PNSSecureCoding for MXLocationActivityMetric {}
impl std::convert::TryFrom<MXMetric> for MXLocationActivityMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXLocationActivityMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXLocationActivityMetric").unwrap()) };
        if is_kind_of {
            Ok(MXLocationActivityMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXLocationActivityMetric")
        }
    }
}
impl INSObject for MXLocationActivityMetric {}
impl PNSObject for MXLocationActivityMetric {}
impl IMXLocationActivityMetric for MXLocationActivityMetric {}
pub trait IMXLocationActivityMetric: Sized + std::ops::Deref {
    unsafe fn cumulativeBestAccuracyTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeBestAccuracyTime)
    }
    unsafe fn cumulativeBestAccuracyForNavigationTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeBestAccuracyForNavigationTime)
    }
    unsafe fn cumulativeNearestTenMetersAccuracyTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeNearestTenMetersAccuracyTime)
    }
    unsafe fn cumulativeHundredMetersAccuracyTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeHundredMetersAccuracyTime)
    }
    unsafe fn cumulativeKilometerAccuracyTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeKilometerAccuracyTime)
    }
    unsafe fn cumulativeThreeKilometersAccuracyTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeThreeKilometersAccuracyTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXNetworkTransferMetric(pub id);
impl std::ops::Deref for MXNetworkTransferMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXNetworkTransferMetric {}
impl MXNetworkTransferMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXNetworkTransferMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXNetworkTransferMetric {}
impl PNSSecureCoding for MXNetworkTransferMetric {}
impl std::convert::TryFrom<MXMetric> for MXNetworkTransferMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXNetworkTransferMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXNetworkTransferMetric").unwrap()) };
        if is_kind_of {
            Ok(MXNetworkTransferMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXNetworkTransferMetric")
        }
    }
}
impl INSObject for MXNetworkTransferMetric {}
impl PNSObject for MXNetworkTransferMetric {}
impl IMXNetworkTransferMetric for MXNetworkTransferMetric {}
pub trait IMXNetworkTransferMetric: Sized + std::ops::Deref {
    unsafe fn cumulativeWifiUpload(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeWifiUpload)
    }
    unsafe fn cumulativeWifiDownload(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeWifiDownload)
    }
    unsafe fn cumulativeCellularUpload(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeCellularUpload)
    }
    unsafe fn cumulativeCellularDownload(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeCellularDownload)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXAppLaunchMetric(pub id);
impl std::ops::Deref for MXAppLaunchMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXAppLaunchMetric {}
impl MXAppLaunchMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXAppLaunchMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXAppLaunchMetric {}
impl PNSSecureCoding for MXAppLaunchMetric {}
impl std::convert::TryFrom<MXMetric> for MXAppLaunchMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXAppLaunchMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXAppLaunchMetric").unwrap()) };
        if is_kind_of {
            Ok(MXAppLaunchMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXAppLaunchMetric")
        }
    }
}
impl INSObject for MXAppLaunchMetric {}
impl PNSObject for MXAppLaunchMetric {}
impl IMXAppLaunchMetric for MXAppLaunchMetric {}
pub trait IMXAppLaunchMetric: Sized + std::ops::Deref {
    unsafe fn histogrammedTimeToFirstDraw(&self) -> MXHistogram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, histogrammedTimeToFirstDraw)
    }
    unsafe fn histogrammedApplicationResumeTime(&self) -> MXHistogram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, histogrammedApplicationResumeTime)
    }
    unsafe fn histogrammedOptimizedTimeToFirstDraw(&self) -> MXHistogram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, histogrammedOptimizedTimeToFirstDraw)
    }
    unsafe fn histogrammedExtendedLaunch(&self) -> MXHistogram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, histogrammedExtendedLaunch)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXAppResponsivenessMetric(pub id);
impl std::ops::Deref for MXAppResponsivenessMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXAppResponsivenessMetric {}
impl MXAppResponsivenessMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXAppResponsivenessMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXAppResponsivenessMetric {}
impl PNSSecureCoding for MXAppResponsivenessMetric {}
impl std::convert::TryFrom<MXMetric> for MXAppResponsivenessMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXAppResponsivenessMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXAppResponsivenessMetric").unwrap()) };
        if is_kind_of {
            Ok(MXAppResponsivenessMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXAppResponsivenessMetric")
        }
    }
}
impl INSObject for MXAppResponsivenessMetric {}
impl PNSObject for MXAppResponsivenessMetric {}
impl IMXAppResponsivenessMetric for MXAppResponsivenessMetric {}
pub trait IMXAppResponsivenessMetric: Sized + std::ops::Deref {
    unsafe fn histogrammedApplicationHangTime(&self) -> MXHistogram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, histogrammedApplicationHangTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXDiskIOMetric(pub id);
impl std::ops::Deref for MXDiskIOMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXDiskIOMetric {}
impl MXDiskIOMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXDiskIOMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXDiskIOMetric {}
impl PNSSecureCoding for MXDiskIOMetric {}
impl std::convert::TryFrom<MXMetric> for MXDiskIOMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXDiskIOMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXDiskIOMetric").unwrap()) };
        if is_kind_of {
            Ok(MXDiskIOMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXDiskIOMetric")
        }
    }
}
impl INSObject for MXDiskIOMetric {}
impl PNSObject for MXDiskIOMetric {}
impl IMXDiskIOMetric for MXDiskIOMetric {}
pub trait IMXDiskIOMetric: Sized + std::ops::Deref {
    unsafe fn cumulativeLogicalWrites(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeLogicalWrites)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXAverage(pub id);
impl std::ops::Deref for MXAverage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXAverage {}
impl MXAverage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXAverage").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXAverage {}
impl INSObject for MXAverage {}
impl PNSObject for MXAverage {}
impl std::convert::TryFrom<NSObject> for MXAverage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXAverage, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXAverage").unwrap()) };
        if is_kind_of {
            Ok(MXAverage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXAverage")
        }
    }
}
impl<UnitType: 'static> IMXAverage<UnitType> for MXAverage {}
pub trait IMXAverage<UnitType: 'static>: Sized + std::ops::Deref {
    unsafe fn averageMeasurement(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averageMeasurement)
    }
    unsafe fn sampleCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn standardDeviation(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, standardDeviation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXMemoryMetric(pub id);
impl std::ops::Deref for MXMemoryMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXMemoryMetric {}
impl MXMemoryMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXMemoryMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXMemoryMetric {}
impl PNSSecureCoding for MXMemoryMetric {}
impl std::convert::TryFrom<MXMetric> for MXMemoryMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXMemoryMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXMemoryMetric").unwrap()) };
        if is_kind_of {
            Ok(MXMemoryMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXMemoryMetric")
        }
    }
}
impl INSObject for MXMemoryMetric {}
impl PNSObject for MXMemoryMetric {}
impl IMXMemoryMetric for MXMemoryMetric {}
pub trait IMXMemoryMetric: Sized + std::ops::Deref {
    unsafe fn peakMemoryUsage(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peakMemoryUsage)
    }
    unsafe fn averageSuspendedMemory(&self) -> MXAverage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averageSuspendedMemory)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXDisplayMetric(pub id);
impl std::ops::Deref for MXDisplayMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXDisplayMetric {}
impl MXDisplayMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXDisplayMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXDisplayMetric {}
impl PNSSecureCoding for MXDisplayMetric {}
impl std::convert::TryFrom<MXMetric> for MXDisplayMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXDisplayMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXDisplayMetric").unwrap()) };
        if is_kind_of {
            Ok(MXDisplayMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXDisplayMetric")
        }
    }
}
impl INSObject for MXDisplayMetric {}
impl PNSObject for MXDisplayMetric {}
impl IMXDisplayMetric for MXDisplayMetric {}
pub trait IMXDisplayMetric: Sized + std::ops::Deref {
    unsafe fn averagePixelLuminance(&self) -> MXAverage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averagePixelLuminance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXAnimationMetric(pub id);
impl std::ops::Deref for MXAnimationMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXAnimationMetric {}
impl MXAnimationMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXAnimationMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXAnimationMetric {}
impl PNSSecureCoding for MXAnimationMetric {}
impl std::convert::TryFrom<MXMetric> for MXAnimationMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXAnimationMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXAnimationMetric").unwrap()) };
        if is_kind_of {
            Ok(MXAnimationMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXAnimationMetric")
        }
    }
}
impl INSObject for MXAnimationMetric {}
impl PNSObject for MXAnimationMetric {}
impl IMXAnimationMetric for MXAnimationMetric {}
pub trait IMXAnimationMetric: Sized + std::ops::Deref {
    unsafe fn scrollHitchTimeRatio(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrollHitchTimeRatio)
    }
    unsafe fn hitchTimeRatio(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hitchTimeRatio)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXForegroundExitData(pub id);
impl std::ops::Deref for MXForegroundExitData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXForegroundExitData {}
impl MXForegroundExitData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXForegroundExitData").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXForegroundExitData {}
impl INSObject for MXForegroundExitData {}
impl PNSObject for MXForegroundExitData {}
impl std::convert::TryFrom<NSObject> for MXForegroundExitData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXForegroundExitData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXForegroundExitData").unwrap()) };
        if is_kind_of {
            Ok(MXForegroundExitData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXForegroundExitData")
        }
    }
}
impl IMXForegroundExitData for MXForegroundExitData {}
pub trait IMXForegroundExitData: Sized + std::ops::Deref {
    unsafe fn cumulativeNormalAppExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeNormalAppExitCount)
    }
    unsafe fn cumulativeMemoryResourceLimitExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeMemoryResourceLimitExitCount)
    }
    unsafe fn cumulativeBadAccessExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeBadAccessExitCount)
    }
    unsafe fn cumulativeAbnormalExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeAbnormalExitCount)
    }
    unsafe fn cumulativeIllegalInstructionExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeIllegalInstructionExitCount)
    }
    unsafe fn cumulativeAppWatchdogExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeAppWatchdogExitCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXBackgroundExitData(pub id);
impl std::ops::Deref for MXBackgroundExitData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXBackgroundExitData {}
impl MXBackgroundExitData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXBackgroundExitData").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXBackgroundExitData {}
impl INSObject for MXBackgroundExitData {}
impl PNSObject for MXBackgroundExitData {}
impl std::convert::TryFrom<NSObject> for MXBackgroundExitData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXBackgroundExitData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXBackgroundExitData").unwrap()) };
        if is_kind_of {
            Ok(MXBackgroundExitData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXBackgroundExitData")
        }
    }
}
impl IMXBackgroundExitData for MXBackgroundExitData {}
pub trait IMXBackgroundExitData: Sized + std::ops::Deref {
    unsafe fn cumulativeNormalAppExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeNormalAppExitCount)
    }
    unsafe fn cumulativeMemoryResourceLimitExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeMemoryResourceLimitExitCount)
    }
    unsafe fn cumulativeCPUResourceLimitExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeCPUResourceLimitExitCount)
    }
    unsafe fn cumulativeMemoryPressureExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeMemoryPressureExitCount)
    }
    unsafe fn cumulativeBadAccessExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeBadAccessExitCount)
    }
    unsafe fn cumulativeAbnormalExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeAbnormalExitCount)
    }
    unsafe fn cumulativeIllegalInstructionExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeIllegalInstructionExitCount)
    }
    unsafe fn cumulativeAppWatchdogExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeAppWatchdogExitCount)
    }
    unsafe fn cumulativeSuspendedWithLockedFileExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeSuspendedWithLockedFileExitCount)
    }
    unsafe fn cumulativeBackgroundTaskAssertionTimeoutExitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeBackgroundTaskAssertionTimeoutExitCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXAppExitMetric(pub id);
impl std::ops::Deref for MXAppExitMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXAppExitMetric {}
impl MXAppExitMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXAppExitMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXAppExitMetric {}
impl PNSSecureCoding for MXAppExitMetric {}
impl std::convert::TryFrom<MXMetric> for MXAppExitMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXAppExitMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXAppExitMetric").unwrap()) };
        if is_kind_of {
            Ok(MXAppExitMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXAppExitMetric")
        }
    }
}
impl INSObject for MXAppExitMetric {}
impl PNSObject for MXAppExitMetric {}
impl IMXAppExitMetric for MXAppExitMetric {}
pub trait IMXAppExitMetric: Sized + std::ops::Deref {
    unsafe fn foregroundExitData(&self) -> MXForegroundExitData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foregroundExitData)
    }
    unsafe fn backgroundExitData(&self) -> MXBackgroundExitData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundExitData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXDiskSpaceUsageMetric(pub id);
impl std::ops::Deref for MXDiskSpaceUsageMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXDiskSpaceUsageMetric {}
impl MXDiskSpaceUsageMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXDiskSpaceUsageMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXDiskSpaceUsageMetric {}
impl PNSSecureCoding for MXDiskSpaceUsageMetric {}
impl std::convert::TryFrom<MXMetric> for MXDiskSpaceUsageMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXDiskSpaceUsageMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXDiskSpaceUsageMetric").unwrap()) };
        if is_kind_of {
            Ok(MXDiskSpaceUsageMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXDiskSpaceUsageMetric")
        }
    }
}
impl INSObject for MXDiskSpaceUsageMetric {}
impl PNSObject for MXDiskSpaceUsageMetric {}
impl IMXDiskSpaceUsageMetric for MXDiskSpaceUsageMetric {}
pub trait IMXDiskSpaceUsageMetric: Sized + std::ops::Deref {
    unsafe fn totalBinaryFileSize(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalBinaryFileSize)
    }
    unsafe fn totalBinaryFileCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalBinaryFileCount)
    }
    unsafe fn totalDataFileSize(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalDataFileSize)
    }
    unsafe fn totalDataFileCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalDataFileCount)
    }
    unsafe fn totalCacheFolderSize(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalCacheFolderSize)
    }
    unsafe fn totalCloneSize(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalCloneSize)
    }
    unsafe fn totalDiskSpaceUsedSize(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalDiskSpaceUsedSize)
    }
    unsafe fn totalDiskSpaceCapacity(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalDiskSpaceCapacity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXSignpostIntervalData(pub id);
impl std::ops::Deref for MXSignpostIntervalData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXSignpostIntervalData {}
impl MXSignpostIntervalData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXSignpostIntervalData").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXSignpostIntervalData {}
impl INSObject for MXSignpostIntervalData {}
impl PNSObject for MXSignpostIntervalData {}
impl std::convert::TryFrom<NSObject> for MXSignpostIntervalData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXSignpostIntervalData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXSignpostIntervalData").unwrap()) };
        if is_kind_of {
            Ok(MXSignpostIntervalData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXSignpostIntervalData")
        }
    }
}
impl IMXSignpostIntervalData for MXSignpostIntervalData {}
pub trait IMXSignpostIntervalData: Sized + std::ops::Deref {
    unsafe fn histogrammedSignpostDuration(&self) -> MXHistogram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, histogrammedSignpostDuration)
    }
    unsafe fn cumulativeCPUTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeCPUTime)
    }
    unsafe fn averageMemory(&self) -> MXAverage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averageMemory)
    }
    unsafe fn cumulativeLogicalWrites(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeLogicalWrites)
    }
    unsafe fn cumulativeHitchTimeRatio(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cumulativeHitchTimeRatio)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXSignpostMetric(pub id);
impl std::ops::Deref for MXSignpostMetric {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXSignpostMetric {}
impl MXSignpostMetric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXSignpostMetric").unwrap(), alloc) })
    }
}
impl IMXMetric for MXSignpostMetric {}
impl PNSSecureCoding for MXSignpostMetric {}
impl std::convert::TryFrom<MXMetric> for MXSignpostMetric {
    type Error = &'static str;
    fn try_from(parent: MXMetric) -> Result<MXSignpostMetric, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXSignpostMetric").unwrap()) };
        if is_kind_of {
            Ok(MXSignpostMetric(parent.0))
        } else {
            Err("This MXMetric cannot be downcasted to MXSignpostMetric")
        }
    }
}
impl INSObject for MXSignpostMetric {}
impl PNSObject for MXSignpostMetric {}
impl IMXSignpostMetric for MXSignpostMetric {}
pub trait IMXSignpostMetric: Sized + std::ops::Deref {
    unsafe fn signpostName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signpostName)
    }
    unsafe fn signpostCategory(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signpostCategory)
    }
    unsafe fn signpostIntervalData(&self) -> MXSignpostIntervalData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signpostIntervalData)
    }
    unsafe fn totalCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXMetricPayload(pub id);
impl std::ops::Deref for MXMetricPayload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXMetricPayload {}
impl MXMetricPayload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXMetricPayload").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXMetricPayload {}
impl INSObject for MXMetricPayload {}
impl PNSObject for MXMetricPayload {}
impl std::convert::TryFrom<NSObject> for MXMetricPayload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXMetricPayload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXMetricPayload").unwrap()) };
        if is_kind_of {
            Ok(MXMetricPayload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXMetricPayload")
        }
    }
}
impl IMXMetricPayload for MXMetricPayload {}
pub trait IMXMetricPayload: Sized + std::ops::Deref {
    unsafe fn JSONRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSONRepresentation)
    }
    unsafe fn DictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, DictionaryRepresentation)
    }
    unsafe fn dictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryRepresentation)
    }
    unsafe fn latestApplicationVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, latestApplicationVersion)
    }
    unsafe fn includesMultipleApplicationVersions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includesMultipleApplicationVersions)
    }
    unsafe fn timeStampBegin(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeStampBegin)
    }
    unsafe fn timeStampEnd(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeStampEnd)
    }
    unsafe fn cpuMetrics(&self) -> MXCPUMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cpuMetrics)
    }
    unsafe fn gpuMetrics(&self) -> MXGPUMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuMetrics)
    }
    unsafe fn cellularConditionMetrics(&self) -> MXCellularConditionMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellularConditionMetrics)
    }
    unsafe fn applicationTimeMetrics(&self) -> MXAppRunTimeMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationTimeMetrics)
    }
    unsafe fn locationActivityMetrics(&self) -> MXLocationActivityMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationActivityMetrics)
    }
    unsafe fn networkTransferMetrics(&self) -> MXNetworkTransferMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkTransferMetrics)
    }
    unsafe fn applicationLaunchMetrics(&self) -> MXAppLaunchMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationLaunchMetrics)
    }
    unsafe fn applicationResponsivenessMetrics(&self) -> MXAppResponsivenessMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationResponsivenessMetrics)
    }
    unsafe fn diskIOMetrics(&self) -> MXDiskIOMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diskIOMetrics)
    }
    unsafe fn memoryMetrics(&self) -> MXMemoryMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, memoryMetrics)
    }
    unsafe fn displayMetrics(&self) -> MXDisplayMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayMetrics)
    }
    unsafe fn animationMetrics(&self) -> MXAnimationMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationMetrics)
    }
    unsafe fn applicationExitMetrics(&self) -> MXAppExitMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationExitMetrics)
    }
    unsafe fn diskSpaceUsageMetrics(&self) -> MXDiskSpaceUsageMetric
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diskSpaceUsageMetrics)
    }
    unsafe fn signpostMetrics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signpostMetrics)
    }
    unsafe fn metaData(&self) -> MXMetaData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metaData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXSignpostRecord(pub id);
impl std::ops::Deref for MXSignpostRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXSignpostRecord {}
impl MXSignpostRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXSignpostRecord").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXSignpostRecord {}
impl INSObject for MXSignpostRecord {}
impl PNSObject for MXSignpostRecord {}
impl std::convert::TryFrom<NSObject> for MXSignpostRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXSignpostRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXSignpostRecord").unwrap()) };
        if is_kind_of {
            Ok(MXSignpostRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXSignpostRecord")
        }
    }
}
impl IMXSignpostRecord for MXSignpostRecord {}
pub trait IMXSignpostRecord: Sized + std::ops::Deref {
    unsafe fn JSONRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSONRepresentation)
    }
    unsafe fn dictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryRepresentation)
    }
    unsafe fn subsystem(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subsystem)
    }
    unsafe fn category(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn beginTimeStamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginTimeStamp)
    }
    unsafe fn endTimeStamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endTimeStamp)
    }
    unsafe fn duration(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn isInterval(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInterval)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXDiagnostic(pub id);
impl std::ops::Deref for MXDiagnostic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXDiagnostic {}
impl MXDiagnostic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXDiagnostic").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXDiagnostic {}
impl INSObject for MXDiagnostic {}
impl PNSObject for MXDiagnostic {}
impl std::convert::TryFrom<NSObject> for MXDiagnostic {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXDiagnostic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXDiagnostic").unwrap()) };
        if is_kind_of {
            Ok(MXDiagnostic(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXDiagnostic")
        }
    }
}
impl IMXDiagnostic for MXDiagnostic {}
pub trait IMXDiagnostic: Sized + std::ops::Deref {
    unsafe fn JSONRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSONRepresentation)
    }
    unsafe fn dictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryRepresentation)
    }
    unsafe fn metaData(&self) -> MXMetaData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metaData)
    }
    unsafe fn applicationVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationVersion)
    }
    unsafe fn signpostData(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signpostData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXCallStackTree(pub id);
impl std::ops::Deref for MXCallStackTree {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXCallStackTree {}
impl MXCallStackTree {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXCallStackTree").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXCallStackTree {}
impl INSObject for MXCallStackTree {}
impl PNSObject for MXCallStackTree {}
impl std::convert::TryFrom<NSObject> for MXCallStackTree {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXCallStackTree, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXCallStackTree").unwrap()) };
        if is_kind_of {
            Ok(MXCallStackTree(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXCallStackTree")
        }
    }
}
impl IMXCallStackTree for MXCallStackTree {}
pub trait IMXCallStackTree: Sized + std::ops::Deref {
    unsafe fn JSONRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSONRepresentation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXCPUExceptionDiagnostic(pub id);
impl std::ops::Deref for MXCPUExceptionDiagnostic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXCPUExceptionDiagnostic {}
impl MXCPUExceptionDiagnostic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXCPUExceptionDiagnostic").unwrap(), alloc) })
    }
}
impl IMXDiagnostic for MXCPUExceptionDiagnostic {}
impl PNSSecureCoding for MXCPUExceptionDiagnostic {}
impl From<MXCPUExceptionDiagnostic> for MXDiagnostic {
    fn from(child: MXCPUExceptionDiagnostic) -> MXDiagnostic {
        MXDiagnostic(child.0)
    }
}
impl std::convert::TryFrom<MXDiagnostic> for MXCPUExceptionDiagnostic {
    type Error = &'static str;
    fn try_from(parent: MXDiagnostic) -> Result<MXCPUExceptionDiagnostic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXCPUExceptionDiagnostic").unwrap()) };
        if is_kind_of {
            Ok(MXCPUExceptionDiagnostic(parent.0))
        } else {
            Err("This MXDiagnostic cannot be downcasted to MXCPUExceptionDiagnostic")
        }
    }
}
impl INSObject for MXCPUExceptionDiagnostic {}
impl PNSObject for MXCPUExceptionDiagnostic {}
impl IMXCPUExceptionDiagnostic for MXCPUExceptionDiagnostic {}
pub trait IMXCPUExceptionDiagnostic: Sized + std::ops::Deref {
    unsafe fn callStackTree(&self) -> MXCallStackTree
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callStackTree)
    }
    unsafe fn totalCPUTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalCPUTime)
    }
    unsafe fn totalSampledTime(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalSampledTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXDiskWriteExceptionDiagnostic(pub id);
impl std::ops::Deref for MXDiskWriteExceptionDiagnostic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXDiskWriteExceptionDiagnostic {}
impl MXDiskWriteExceptionDiagnostic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXDiskWriteExceptionDiagnostic").unwrap(), alloc) })
    }
}
impl IMXDiagnostic for MXDiskWriteExceptionDiagnostic {}
impl PNSSecureCoding for MXDiskWriteExceptionDiagnostic {}
impl std::convert::TryFrom<MXDiagnostic> for MXDiskWriteExceptionDiagnostic {
    type Error = &'static str;
    fn try_from(parent: MXDiagnostic) -> Result<MXDiskWriteExceptionDiagnostic, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXDiskWriteExceptionDiagnostic").unwrap())
        };
        if is_kind_of {
            Ok(MXDiskWriteExceptionDiagnostic(parent.0))
        } else {
            Err("This MXDiagnostic cannot be downcasted to MXDiskWriteExceptionDiagnostic")
        }
    }
}
impl INSObject for MXDiskWriteExceptionDiagnostic {}
impl PNSObject for MXDiskWriteExceptionDiagnostic {}
impl IMXDiskWriteExceptionDiagnostic for MXDiskWriteExceptionDiagnostic {}
pub trait IMXDiskWriteExceptionDiagnostic: Sized + std::ops::Deref {
    unsafe fn callStackTree(&self) -> MXCallStackTree
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callStackTree)
    }
    unsafe fn totalWritesCaused(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalWritesCaused)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXHangDiagnostic(pub id);
impl std::ops::Deref for MXHangDiagnostic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXHangDiagnostic {}
impl MXHangDiagnostic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXHangDiagnostic").unwrap(), alloc) })
    }
}
impl IMXDiagnostic for MXHangDiagnostic {}
impl PNSSecureCoding for MXHangDiagnostic {}
impl std::convert::TryFrom<MXDiagnostic> for MXHangDiagnostic {
    type Error = &'static str;
    fn try_from(parent: MXDiagnostic) -> Result<MXHangDiagnostic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXHangDiagnostic").unwrap()) };
        if is_kind_of {
            Ok(MXHangDiagnostic(parent.0))
        } else {
            Err("This MXDiagnostic cannot be downcasted to MXHangDiagnostic")
        }
    }
}
impl INSObject for MXHangDiagnostic {}
impl PNSObject for MXHangDiagnostic {}
impl IMXHangDiagnostic for MXHangDiagnostic {}
pub trait IMXHangDiagnostic: Sized + std::ops::Deref {
    unsafe fn callStackTree(&self) -> MXCallStackTree
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callStackTree)
    }
    unsafe fn hangDuration(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hangDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXAppLaunchDiagnostic(pub id);
impl std::ops::Deref for MXAppLaunchDiagnostic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXAppLaunchDiagnostic {}
impl MXAppLaunchDiagnostic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXAppLaunchDiagnostic").unwrap(), alloc) })
    }
}
impl IMXDiagnostic for MXAppLaunchDiagnostic {}
impl PNSSecureCoding for MXAppLaunchDiagnostic {}
impl std::convert::TryFrom<MXDiagnostic> for MXAppLaunchDiagnostic {
    type Error = &'static str;
    fn try_from(parent: MXDiagnostic) -> Result<MXAppLaunchDiagnostic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXAppLaunchDiagnostic").unwrap()) };
        if is_kind_of {
            Ok(MXAppLaunchDiagnostic(parent.0))
        } else {
            Err("This MXDiagnostic cannot be downcasted to MXAppLaunchDiagnostic")
        }
    }
}
impl INSObject for MXAppLaunchDiagnostic {}
impl PNSObject for MXAppLaunchDiagnostic {}
impl IMXAppLaunchDiagnostic for MXAppLaunchDiagnostic {}
pub trait IMXAppLaunchDiagnostic: Sized + std::ops::Deref {
    unsafe fn callStackTree(&self) -> MXCallStackTree
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callStackTree)
    }
    unsafe fn launchDuration(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, launchDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXCrashDiagnosticObjectiveCExceptionReason(pub id);
impl std::ops::Deref for MXCrashDiagnosticObjectiveCExceptionReason {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXCrashDiagnosticObjectiveCExceptionReason {}
impl MXCrashDiagnosticObjectiveCExceptionReason {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXCrashDiagnosticObjectiveCExceptionReason").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXCrashDiagnosticObjectiveCExceptionReason {}
impl INSObject for MXCrashDiagnosticObjectiveCExceptionReason {}
impl PNSObject for MXCrashDiagnosticObjectiveCExceptionReason {}
impl std::convert::TryFrom<NSObject> for MXCrashDiagnosticObjectiveCExceptionReason {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MXCrashDiagnosticObjectiveCExceptionReason, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXCrashDiagnosticObjectiveCExceptionReason").unwrap())
        };
        if is_kind_of {
            Ok(MXCrashDiagnosticObjectiveCExceptionReason(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXCrashDiagnosticObjectiveCExceptionReason")
        }
    }
}
impl IMXCrashDiagnosticObjectiveCExceptionReason for MXCrashDiagnosticObjectiveCExceptionReason {}
pub trait IMXCrashDiagnosticObjectiveCExceptionReason: Sized + std::ops::Deref {
    unsafe fn JSONRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSONRepresentation)
    }
    unsafe fn dictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryRepresentation)
    }
    unsafe fn composedMessage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composedMessage)
    }
    unsafe fn formatString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatString)
    }
    unsafe fn arguments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arguments)
    }
    unsafe fn exceptionType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionType)
    }
    unsafe fn className(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, className)
    }
    unsafe fn exceptionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXCrashDiagnostic(pub id);
impl std::ops::Deref for MXCrashDiagnostic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXCrashDiagnostic {}
impl MXCrashDiagnostic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXCrashDiagnostic").unwrap(), alloc) })
    }
}
impl IMXDiagnostic for MXCrashDiagnostic {}
impl PNSSecureCoding for MXCrashDiagnostic {}
impl std::convert::TryFrom<MXDiagnostic> for MXCrashDiagnostic {
    type Error = &'static str;
    fn try_from(parent: MXDiagnostic) -> Result<MXCrashDiagnostic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXCrashDiagnostic").unwrap()) };
        if is_kind_of {
            Ok(MXCrashDiagnostic(parent.0))
        } else {
            Err("This MXDiagnostic cannot be downcasted to MXCrashDiagnostic")
        }
    }
}
impl INSObject for MXCrashDiagnostic {}
impl PNSObject for MXCrashDiagnostic {}
impl IMXCrashDiagnostic for MXCrashDiagnostic {}
pub trait IMXCrashDiagnostic: Sized + std::ops::Deref {
    unsafe fn callStackTree(&self) -> MXCallStackTree
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callStackTree)
    }
    unsafe fn terminationReason(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, terminationReason)
    }
    unsafe fn virtualMemoryRegionInfo(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, virtualMemoryRegionInfo)
    }
    unsafe fn exceptionType(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionType)
    }
    unsafe fn exceptionCode(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionCode)
    }
    unsafe fn signal(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signal)
    }
    unsafe fn exceptionReason(&self) -> MXCrashDiagnosticObjectiveCExceptionReason
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionReason)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXDiagnosticPayload(pub id);
impl std::ops::Deref for MXDiagnosticPayload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXDiagnosticPayload {}
impl MXDiagnosticPayload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXDiagnosticPayload").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MXDiagnosticPayload {}
impl INSObject for MXDiagnosticPayload {}
impl PNSObject for MXDiagnosticPayload {}
impl std::convert::TryFrom<NSObject> for MXDiagnosticPayload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXDiagnosticPayload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXDiagnosticPayload").unwrap()) };
        if is_kind_of {
            Ok(MXDiagnosticPayload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXDiagnosticPayload")
        }
    }
}
impl IMXDiagnosticPayload for MXDiagnosticPayload {}
pub trait IMXDiagnosticPayload: Sized + std::ops::Deref {
    unsafe fn JSONRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSONRepresentation)
    }
    unsafe fn dictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryRepresentation)
    }
    unsafe fn cpuExceptionDiagnostics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cpuExceptionDiagnostics)
    }
    unsafe fn diskWriteExceptionDiagnostics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diskWriteExceptionDiagnostics)
    }
    unsafe fn hangDiagnostics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hangDiagnostics)
    }
    unsafe fn appLaunchDiagnostics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appLaunchDiagnostics)
    }
    unsafe fn crashDiagnostics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crashDiagnostics)
    }
    unsafe fn timeStampBegin(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeStampBegin)
    }
    unsafe fn timeStampEnd(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeStampEnd)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MXMetricManager(pub id);
impl std::ops::Deref for MXMetricManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MXMetricManager {}
impl MXMetricManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MXMetricManager").unwrap(), alloc) })
    }
}
impl INSObject for MXMetricManager {}
impl PNSObject for MXMetricManager {}
impl std::convert::TryFrom<NSObject> for MXMetricManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MXMetricManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MXMetricManager").unwrap()) };
        if is_kind_of {
            Ok(MXMetricManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MXMetricManager")
        }
    }
}
impl IMXMetricManager for MXMetricManager {}
pub trait IMXMetricManager: Sized + std::ops::Deref {
    unsafe fn addSubscriber_(&self, subscriber: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSubscriber : subscriber)
    }
    unsafe fn removeSubscriber_(&self, subscriber: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeSubscriber : subscriber)
    }
    unsafe fn pastPayloads(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pastPayloads)
    }
    unsafe fn pastDiagnosticPayloads(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pastDiagnosticPayloads)
    }
    unsafe fn makeLogHandleWithCategory_(category: NSString) -> os_log_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MXMetricManager").unwrap(), makeLogHandleWithCategory : category)
    }
    unsafe fn extendLaunchMeasurementForTaskID_error_(taskID: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MXMetricManager").unwrap(), extendLaunchMeasurementForTaskID : taskID, error : error)
    }
    unsafe fn finishExtendedLaunchMeasurementForTaskID_error_(
        taskID: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MXMetricManager").unwrap(), finishExtendedLaunchMeasurementForTaskID : taskID, error : error)
    }
    unsafe fn sharedManager() -> MXMetricManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MXMetricManager").unwrap(), sharedManager)
    }
}
pub trait PMXMetricManagerSubscriber: Sized + std::ops::Deref {
    unsafe fn didReceiveMetricPayloads_(&self, payloads: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveMetricPayloads : payloads)
    }
    unsafe fn didReceiveDiagnosticPayloads_(&self, payloads: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveDiagnosticPayloads : payloads)
    }
}
pub type MXErrorCode = NSInteger;
unsafe extern "C" {
    pub static MXErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for MXMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXCPUMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXCPUMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXGPUMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXGPUMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXUnitSignalBars {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXUnitSignalBars {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXUnitAveragePixelLuminance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXUnitAveragePixelLuminance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXHistogramBucket {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXHistogramBucket {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXHistogram {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXHistogram {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXCellularConditionMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXCellularConditionMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXMetaData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXMetaData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXAppRunTimeMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXAppRunTimeMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXLocationActivityMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXLocationActivityMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXNetworkTransferMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXNetworkTransferMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXAppLaunchMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXAppLaunchMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXAppResponsivenessMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXAppResponsivenessMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXDiskIOMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXDiskIOMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXAverage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXAverage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXMemoryMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXMemoryMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXDisplayMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXDisplayMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXAnimationMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXAnimationMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXForegroundExitData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXForegroundExitData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXBackgroundExitData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXBackgroundExitData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXAppExitMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXAppExitMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXDiskSpaceUsageMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXDiskSpaceUsageMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXSignpostIntervalData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXSignpostIntervalData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXSignpostMetric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXSignpostMetric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXMetricPayload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXMetricPayload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXSignpostRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXSignpostRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXCallStackTree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXCallStackTree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXCPUExceptionDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXCPUExceptionDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXDiskWriteExceptionDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXDiskWriteExceptionDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXHangDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXHangDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXAppLaunchDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXAppLaunchDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXCrashDiagnosticObjectiveCExceptionReason {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXCrashDiagnosticObjectiveCExceptionReason {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXCrashDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXCrashDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXDiagnosticPayload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXDiagnosticPayload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MXMetricManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MXMetricManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
