#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ENErrorCode = NSInteger;
pub type ENAttenuation = u8;
pub type ENAuthorizationStatus = NSInteger;
pub type ENCalibrationConfidence = u8;
pub type ENDiagnosisReportType = u32;
pub type ENInfectiousness = u32;
pub type ENVariantOfConcernType = u32;
pub type ENIntervalNumber = u32;
pub type ENRiskLevel = u8;
pub type ENRiskScore = u8;
pub type ENErrorHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENExposureConfiguration(pub id);
impl std::ops::Deref for ENExposureConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENExposureConfiguration {}
impl ENExposureConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENExposureConfiguration").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENExposureConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENExposureConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENExposureConfiguration").unwrap()) };
        if is_kind_of {
            Ok(ENExposureConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENExposureConfiguration")
        }
    }
}
impl IENExposureConfiguration for ENExposureConfiguration {}
pub trait IENExposureConfiguration: Sized + std::ops::Deref {
    unsafe fn immediateDurationWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, immediateDurationWeight)
    }
    unsafe fn setImmediateDurationWeight_(&self, immediateDurationWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImmediateDurationWeight : immediateDurationWeight)
    }
    unsafe fn nearDurationWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nearDurationWeight)
    }
    unsafe fn setNearDurationWeight_(&self, nearDurationWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNearDurationWeight : nearDurationWeight)
    }
    unsafe fn mediumDurationWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediumDurationWeight)
    }
    unsafe fn setMediumDurationWeight_(&self, mediumDurationWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediumDurationWeight : mediumDurationWeight)
    }
    unsafe fn otherDurationWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, otherDurationWeight)
    }
    unsafe fn setOtherDurationWeight_(&self, otherDurationWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOtherDurationWeight : otherDurationWeight)
    }
    unsafe fn infectiousnessForDaysSinceOnsetOfSymptoms(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, infectiousnessForDaysSinceOnsetOfSymptoms)
    }
    unsafe fn setInfectiousnessForDaysSinceOnsetOfSymptoms_(
        &self,
        infectiousnessForDaysSinceOnsetOfSymptoms: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInfectiousnessForDaysSinceOnsetOfSymptoms : infectiousnessForDaysSinceOnsetOfSymptoms)
    }
    unsafe fn infectiousnessStandardWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, infectiousnessStandardWeight)
    }
    unsafe fn setInfectiousnessStandardWeight_(&self, infectiousnessStandardWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInfectiousnessStandardWeight : infectiousnessStandardWeight)
    }
    unsafe fn infectiousnessHighWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, infectiousnessHighWeight)
    }
    unsafe fn setInfectiousnessHighWeight_(&self, infectiousnessHighWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInfectiousnessHighWeight : infectiousnessHighWeight)
    }
    unsafe fn reportTypeConfirmedTestWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportTypeConfirmedTestWeight)
    }
    unsafe fn setReportTypeConfirmedTestWeight_(&self, reportTypeConfirmedTestWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportTypeConfirmedTestWeight : reportTypeConfirmedTestWeight)
    }
    unsafe fn reportTypeConfirmedClinicalDiagnosisWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportTypeConfirmedClinicalDiagnosisWeight)
    }
    unsafe fn setReportTypeConfirmedClinicalDiagnosisWeight_(
        &self,
        reportTypeConfirmedClinicalDiagnosisWeight: f64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportTypeConfirmedClinicalDiagnosisWeight : reportTypeConfirmedClinicalDiagnosisWeight)
    }
    unsafe fn reportTypeSelfReportedWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportTypeSelfReportedWeight)
    }
    unsafe fn setReportTypeSelfReportedWeight_(&self, reportTypeSelfReportedWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportTypeSelfReportedWeight : reportTypeSelfReportedWeight)
    }
    unsafe fn reportTypeRecursiveWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportTypeRecursiveWeight)
    }
    unsafe fn setReportTypeRecursiveWeight_(&self, reportTypeRecursiveWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportTypeRecursiveWeight : reportTypeRecursiveWeight)
    }
    unsafe fn reportTypeNoneMap(&self) -> ENDiagnosisReportType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportTypeNoneMap)
    }
    unsafe fn setReportTypeNoneMap_(&self, reportTypeNoneMap: ENDiagnosisReportType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportTypeNoneMap : reportTypeNoneMap)
    }
    unsafe fn attenuationDurationThresholds(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationDurationThresholds)
    }
    unsafe fn setAttenuationDurationThresholds_(&self, attenuationDurationThresholds: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttenuationDurationThresholds : attenuationDurationThresholds)
    }
    unsafe fn daysSinceLastExposureThreshold(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daysSinceLastExposureThreshold)
    }
    unsafe fn setDaysSinceLastExposureThreshold_(&self, daysSinceLastExposureThreshold: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDaysSinceLastExposureThreshold : daysSinceLastExposureThreshold)
    }
    unsafe fn minimumRiskScoreFullRange(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumRiskScoreFullRange)
    }
    unsafe fn setMinimumRiskScoreFullRange_(&self, minimumRiskScoreFullRange: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumRiskScoreFullRange : minimumRiskScoreFullRange)
    }
    unsafe fn attenuationLevelValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationLevelValues)
    }
    unsafe fn setAttenuationLevelValues_(&self, attenuationLevelValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttenuationLevelValues : attenuationLevelValues)
    }
    unsafe fn attenuationWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationWeight)
    }
    unsafe fn setAttenuationWeight_(&self, attenuationWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttenuationWeight : attenuationWeight)
    }
    unsafe fn daysSinceLastExposureLevelValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daysSinceLastExposureLevelValues)
    }
    unsafe fn setDaysSinceLastExposureLevelValues_(&self, daysSinceLastExposureLevelValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDaysSinceLastExposureLevelValues : daysSinceLastExposureLevelValues)
    }
    unsafe fn daysSinceLastExposureWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daysSinceLastExposureWeight)
    }
    unsafe fn setDaysSinceLastExposureWeight_(&self, daysSinceLastExposureWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDaysSinceLastExposureWeight : daysSinceLastExposureWeight)
    }
    unsafe fn durationLevelValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, durationLevelValues)
    }
    unsafe fn setDurationLevelValues_(&self, durationLevelValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDurationLevelValues : durationLevelValues)
    }
    unsafe fn durationWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, durationWeight)
    }
    unsafe fn setDurationWeight_(&self, durationWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDurationWeight : durationWeight)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn setMetadata_(&self, metadata: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMetadata : metadata)
    }
    unsafe fn minimumRiskScore(&self) -> ENRiskScore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumRiskScore)
    }
    unsafe fn setMinimumRiskScore_(&self, minimumRiskScore: ENRiskScore)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumRiskScore : minimumRiskScore)
    }
    unsafe fn transmissionRiskLevelValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transmissionRiskLevelValues)
    }
    unsafe fn setTransmissionRiskLevelValues_(&self, transmissionRiskLevelValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransmissionRiskLevelValues : transmissionRiskLevelValues)
    }
    unsafe fn transmissionRiskWeight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transmissionRiskWeight)
    }
    unsafe fn setTransmissionRiskWeight_(&self, transmissionRiskWeight: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransmissionRiskWeight : transmissionRiskWeight)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENExposureDaySummary(pub id);
impl std::ops::Deref for ENExposureDaySummary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENExposureDaySummary {}
impl ENExposureDaySummary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENExposureDaySummary").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENExposureDaySummary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENExposureDaySummary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENExposureDaySummary").unwrap()) };
        if is_kind_of {
            Ok(ENExposureDaySummary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENExposureDaySummary")
        }
    }
}
impl IENExposureDaySummary for ENExposureDaySummary {}
pub trait IENExposureDaySummary: Sized + std::ops::Deref {
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn confirmedTestSummary(&self) -> ENExposureSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confirmedTestSummary)
    }
    unsafe fn confirmedClinicalDiagnosisSummary(&self) -> ENExposureSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confirmedClinicalDiagnosisSummary)
    }
    unsafe fn recursiveSummary(&self) -> ENExposureSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recursiveSummary)
    }
    unsafe fn selfReportedSummary(&self) -> ENExposureSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selfReportedSummary)
    }
    unsafe fn daySummary(&self) -> ENExposureSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daySummary)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENExposureDetectionSummary(pub id);
impl std::ops::Deref for ENExposureDetectionSummary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENExposureDetectionSummary {}
impl ENExposureDetectionSummary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENExposureDetectionSummary").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENExposureDetectionSummary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENExposureDetectionSummary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENExposureDetectionSummary").unwrap()) };
        if is_kind_of {
            Ok(ENExposureDetectionSummary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENExposureDetectionSummary")
        }
    }
}
impl IENExposureDetectionSummary for ENExposureDetectionSummary {}
pub trait IENExposureDetectionSummary: Sized + std::ops::Deref {
    unsafe fn attenuationDurations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationDurations)
    }
    unsafe fn daysSinceLastExposure(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daysSinceLastExposure)
    }
    unsafe fn matchedKeyCount(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchedKeyCount)
    }
    unsafe fn maximumRiskScore(&self) -> ENRiskScore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumRiskScore)
    }
    unsafe fn maximumRiskScoreFullRange(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumRiskScoreFullRange)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn riskScoreSumFullRange(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, riskScoreSumFullRange)
    }
    unsafe fn daySummaries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daySummaries)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENExposureInfo(pub id);
impl std::ops::Deref for ENExposureInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENExposureInfo {}
impl ENExposureInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENExposureInfo").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENExposureInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENExposureInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENExposureInfo").unwrap()) };
        if is_kind_of {
            Ok(ENExposureInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENExposureInfo")
        }
    }
}
impl IENExposureInfo for ENExposureInfo {}
pub trait IENExposureInfo: Sized + std::ops::Deref {
    unsafe fn attenuationDurations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationDurations)
    }
    unsafe fn attenuationValue(&self) -> ENAttenuation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationValue)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn daysSinceOnsetOfSymptoms(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daysSinceOnsetOfSymptoms)
    }
    unsafe fn diagnosisReportType(&self) -> ENDiagnosisReportType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diagnosisReportType)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn totalRiskScore(&self) -> ENRiskScore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalRiskScore)
    }
    unsafe fn totalRiskScoreFullRange(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalRiskScoreFullRange)
    }
    unsafe fn transmissionRiskLevel(&self) -> ENRiskLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transmissionRiskLevel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENExposureSummaryItem(pub id);
impl std::ops::Deref for ENExposureSummaryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENExposureSummaryItem {}
impl ENExposureSummaryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENExposureSummaryItem").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENExposureSummaryItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENExposureSummaryItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENExposureSummaryItem").unwrap()) };
        if is_kind_of {
            Ok(ENExposureSummaryItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENExposureSummaryItem")
        }
    }
}
impl IENExposureSummaryItem for ENExposureSummaryItem {}
pub trait IENExposureSummaryItem: Sized + std::ops::Deref {
    unsafe fn maximumScore(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumScore)
    }
    unsafe fn scoreSum(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scoreSum)
    }
    unsafe fn weightedDurationSum(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weightedDurationSum)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENExposureWindow(pub id);
impl std::ops::Deref for ENExposureWindow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENExposureWindow {}
impl ENExposureWindow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENExposureWindow").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENExposureWindow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENExposureWindow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENExposureWindow").unwrap()) };
        if is_kind_of {
            Ok(ENExposureWindow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENExposureWindow")
        }
    }
}
impl IENExposureWindow for ENExposureWindow {}
pub trait IENExposureWindow: Sized + std::ops::Deref {
    unsafe fn calibrationConfidence(&self) -> ENCalibrationConfidence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calibrationConfidence)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn diagnosisReportType(&self) -> ENDiagnosisReportType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diagnosisReportType)
    }
    unsafe fn infectiousness(&self) -> ENInfectiousness
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, infectiousness)
    }
    unsafe fn scanInstances(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scanInstances)
    }
    unsafe fn variantOfConcernType(&self) -> ENVariantOfConcernType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, variantOfConcernType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENScanInstance(pub id);
impl std::ops::Deref for ENScanInstance {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENScanInstance {}
impl ENScanInstance {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENScanInstance").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENScanInstance {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENScanInstance, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENScanInstance").unwrap()) };
        if is_kind_of {
            Ok(ENScanInstance(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENScanInstance")
        }
    }
}
impl IENScanInstance for ENScanInstance {}
pub trait IENScanInstance: Sized + std::ops::Deref {
    unsafe fn minimumAttenuation(&self) -> ENAttenuation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumAttenuation)
    }
    unsafe fn typicalAttenuation(&self) -> ENAttenuation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typicalAttenuation)
    }
    unsafe fn secondsSinceLastScan(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secondsSinceLastScan)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENTemporaryExposureKey(pub id);
impl std::ops::Deref for ENTemporaryExposureKey {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENTemporaryExposureKey {}
impl ENTemporaryExposureKey {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENTemporaryExposureKey").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENTemporaryExposureKey {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENTemporaryExposureKey, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENTemporaryExposureKey").unwrap()) };
        if is_kind_of {
            Ok(ENTemporaryExposureKey(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENTemporaryExposureKey")
        }
    }
}
impl IENTemporaryExposureKey for ENTemporaryExposureKey {}
pub trait IENTemporaryExposureKey: Sized + std::ops::Deref {
    unsafe fn keyData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyData)
    }
    unsafe fn setKeyData_(&self, keyData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyData : keyData)
    }
    unsafe fn rollingPeriod(&self) -> ENIntervalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rollingPeriod)
    }
    unsafe fn setRollingPeriod_(&self, rollingPeriod: ENIntervalNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRollingPeriod : rollingPeriod)
    }
    unsafe fn rollingStartNumber(&self) -> ENIntervalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rollingStartNumber)
    }
    unsafe fn setRollingStartNumber_(&self, rollingStartNumber: ENIntervalNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRollingStartNumber : rollingStartNumber)
    }
    unsafe fn transmissionRiskLevel(&self) -> ENRiskLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transmissionRiskLevel)
    }
    unsafe fn setTransmissionRiskLevel_(&self, transmissionRiskLevel: ENRiskLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransmissionRiskLevel : transmissionRiskLevel)
    }
}
pub type ENStatus = NSInteger;
pub type ENActivityFlags = u32;
pub type ENActivityHandler = *mut ::std::os::raw::c_void;
pub type ENGetDiagnosisKeysHandler = *mut ::std::os::raw::c_void;
pub type ENDetectExposuresHandler = *mut ::std::os::raw::c_void;
pub type ENDiagnosisKeysAvailableHandler = *mut ::std::os::raw::c_void;
pub type ENGetExposureInfoHandler = *mut ::std::os::raw::c_void;
pub type ENGetExposureWindowsHandler = *mut ::std::os::raw::c_void;
pub type ENGetUserTraveledHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ENManager(pub id);
impl std::ops::Deref for ENManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ENManager {}
impl ENManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ENManager").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ENManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ENManager, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ENManager").unwrap()) };
        if is_kind_of {
            Ok(ENManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ENManager")
        }
    }
}
impl IENManager for ENManager {}
pub trait IENManager: Sized + std::ops::Deref {
    unsafe fn activateWithCompletionHandler_(&self, completionHandler: ENErrorHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateWithCompletionHandler : completionHandler)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn getUserTraveledWithCompletionHandler_(
        &self,
        completionHandler: ENGetUserTraveledHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getUserTraveledWithCompletionHandler : completionHandler)
    }
    unsafe fn setExposureNotificationEnabled_completionHandler_(
        &self,
        enabled: BOOL,
        completionHandler: ENErrorHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureNotificationEnabled : enabled, completionHandler : completionHandler)
    }
    unsafe fn detectExposuresWithConfiguration_completionHandler_(
        &self,
        configuration: ENExposureConfiguration,
        completionHandler: ENDetectExposuresHandler,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectExposuresWithConfiguration : configuration, completionHandler : completionHandler)
    }
    unsafe fn detectExposuresWithConfiguration_diagnosisKeyURLs_completionHandler_(
        &self,
        configuration: ENExposureConfiguration,
        diagnosisKeyURLs: NSArray,
        completionHandler: ENDetectExposuresHandler,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectExposuresWithConfiguration : configuration, diagnosisKeyURLs : diagnosisKeyURLs, completionHandler : completionHandler)
    }
    unsafe fn getExposureInfoFromSummary_userExplanation_completionHandler_(
        &self,
        summary: ENExposureDetectionSummary,
        userExplanation: NSString,
        completionHandler: ENGetExposureInfoHandler,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getExposureInfoFromSummary : summary, userExplanation : userExplanation, completionHandler : completionHandler)
    }
    unsafe fn getExposureWindowsFromSummary_completionHandler_(
        &self,
        summary: ENExposureDetectionSummary,
        completionHandler: ENGetExposureWindowsHandler,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getExposureWindowsFromSummary : summary, completionHandler : completionHandler)
    }
    unsafe fn getDiagnosisKeysWithCompletionHandler_(
        &self,
        completionHandler: ENGetDiagnosisKeysHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDiagnosisKeysWithCompletionHandler : completionHandler)
    }
    unsafe fn getTestDiagnosisKeysWithCompletionHandler_(
        &self,
        completionHandler: ENGetDiagnosisKeysHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getTestDiagnosisKeysWithCompletionHandler : completionHandler)
    }
    unsafe fn preAuthorizeDiagnosisKeysWithCompletionHandler_(
        &self,
        completionHandler: ENErrorHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preAuthorizeDiagnosisKeysWithCompletionHandler : completionHandler)
    }
    unsafe fn requestPreAuthorizedDiagnosisKeysWithCompletionHandler_(
        &self,
        completionHandler: ENErrorHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestPreAuthorizedDiagnosisKeysWithCompletionHandler : completionHandler)
    }
    unsafe fn activityHandler(&self) -> ENActivityHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityHandler)
    }
    unsafe fn setActivityHandler_(&self, activityHandler: ENActivityHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivityHandler : activityHandler)
    }
    unsafe fn dispatchQueue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dispatchQueue)
    }
    unsafe fn setDispatchQueue_(&self, dispatchQueue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDispatchQueue : dispatchQueue)
    }
    unsafe fn exposureNotificationStatus(&self) -> ENStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureNotificationStatus)
    }
    unsafe fn invalidationHandler(&self) -> dispatch_block_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidationHandler)
    }
    unsafe fn setInvalidationHandler_(&self, invalidationHandler: dispatch_block_t)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInvalidationHandler : invalidationHandler)
    }
    unsafe fn exposureNotificationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureNotificationEnabled)
    }
    unsafe fn diagnosisKeysAvailableHandler(&self) -> ENDiagnosisKeysAvailableHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diagnosisKeysAvailableHandler)
    }
    unsafe fn setDiagnosisKeysAvailableHandler_(
        &self,
        diagnosisKeysAvailableHandler: ENDiagnosisKeysAvailableHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiagnosisKeysAvailableHandler : diagnosisKeysAvailableHandler)
    }
    unsafe fn authorizationStatus() -> ENAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ENManager").unwrap(), authorizationStatus)
    }
}
unsafe extern "C" {
    pub static ENErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for ENExposureConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENExposureConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ENExposureDaySummary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENExposureDaySummary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ENExposureDetectionSummary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENExposureDetectionSummary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ENExposureInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENExposureInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ENExposureSummaryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENExposureSummaryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ENExposureWindow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENExposureWindow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ENScanInstance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENScanInstance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ENTemporaryExposureKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENTemporaryExposureKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ENManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ENManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
