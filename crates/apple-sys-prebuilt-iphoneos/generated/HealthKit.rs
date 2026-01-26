#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type HKErrorCode = NSInteger;
pub type HKUpdateFrequency = NSInteger;
pub type HKAuthorizationStatus = NSInteger;
pub type HKAuthorizationRequestStatus = NSInteger;
pub type HKCategoryValue = NSInteger;
pub type HKCategoryValueAppetiteChanges = NSInteger;
pub type HKCategoryValueAppleStandHour = NSInteger;
pub type HKCategoryValueAppleWalkingSteadinessEvent = NSInteger;
pub type HKCategoryValueCervicalMucusQuality = NSInteger;
pub type HKCategoryValueContraceptive = NSInteger;
pub type HKCategoryValueEnvironmentalAudioExposureEvent = NSInteger;
pub type HKCategoryValueHeadphoneAudioExposureEvent = NSInteger;
pub type HKCategoryValueLowCardioFitnessEvent = NSInteger;
pub type HKCategoryValueMenstrualFlow = NSInteger;
pub type HKCategoryValueOvulationTestResult = NSInteger;
pub type HKCategoryValuePregnancyTestResult = NSInteger;
pub type HKCategoryValuePresence = NSInteger;
pub type HKCategoryValueProgesteroneTestResult = NSInteger;
pub type HKCategoryValueSeverity = NSInteger;
pub type HKCategoryValueSleepAnalysis = NSInteger;
pub type HKCategoryValueVaginalBleeding = NSInteger;
pub type HKCategoryValueAudioExposureEvent = NSInteger;
pub type HKActivityMoveMode = NSInteger;
pub type HKBiologicalSex = NSInteger;
pub type HKBloodType = NSInteger;
pub type HKFitzpatrickSkinType = NSInteger;
pub type HKWheelchairUse = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKActivitySummary(pub id);
impl std::ops::Deref for HKActivitySummary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKActivitySummary {}
impl HKActivitySummary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKActivitySummary").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKActivitySummary {}
impl PNSCopying for HKActivitySummary {}
impl INSObject for HKActivitySummary {}
impl PNSObject for HKActivitySummary {}
impl std::convert::TryFrom<NSObject> for HKActivitySummary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKActivitySummary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKActivitySummary").unwrap()) };
        if is_kind_of {
            Ok(HKActivitySummary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKActivitySummary")
        }
    }
}
impl IHKActivitySummary for HKActivitySummary {}
pub trait IHKActivitySummary: Sized + std::ops::Deref {
    unsafe fn dateComponentsForCalendar_(&self, calendar: NSCalendar) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dateComponentsForCalendar : calendar)
    }
    unsafe fn activityMoveMode(&self) -> HKActivityMoveMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityMoveMode)
    }
    unsafe fn setActivityMoveMode_(&self, activityMoveMode: HKActivityMoveMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivityMoveMode : activityMoveMode)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn setPaused_(&self, paused: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaused : paused)
    }
    unsafe fn activeEnergyBurned(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeEnergyBurned)
    }
    unsafe fn setActiveEnergyBurned_(&self, activeEnergyBurned: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActiveEnergyBurned : activeEnergyBurned)
    }
    unsafe fn appleMoveTime(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleMoveTime)
    }
    unsafe fn setAppleMoveTime_(&self, appleMoveTime: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppleMoveTime : appleMoveTime)
    }
    unsafe fn appleExerciseTime(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleExerciseTime)
    }
    unsafe fn setAppleExerciseTime_(&self, appleExerciseTime: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppleExerciseTime : appleExerciseTime)
    }
    unsafe fn appleStandHours(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleStandHours)
    }
    unsafe fn setAppleStandHours_(&self, appleStandHours: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppleStandHours : appleStandHours)
    }
    unsafe fn activeEnergyBurnedGoal(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeEnergyBurnedGoal)
    }
    unsafe fn setActiveEnergyBurnedGoal_(&self, activeEnergyBurnedGoal: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActiveEnergyBurnedGoal : activeEnergyBurnedGoal)
    }
    unsafe fn appleMoveTimeGoal(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleMoveTimeGoal)
    }
    unsafe fn setAppleMoveTimeGoal_(&self, appleMoveTimeGoal: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppleMoveTimeGoal : appleMoveTimeGoal)
    }
    unsafe fn appleExerciseTimeGoal(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleExerciseTimeGoal)
    }
    unsafe fn setAppleExerciseTimeGoal_(&self, appleExerciseTimeGoal: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppleExerciseTimeGoal : appleExerciseTimeGoal)
    }
    unsafe fn exerciseTimeGoal(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exerciseTimeGoal)
    }
    unsafe fn setExerciseTimeGoal_(&self, exerciseTimeGoal: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExerciseTimeGoal : exerciseTimeGoal)
    }
    unsafe fn appleStandHoursGoal(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleStandHoursGoal)
    }
    unsafe fn setAppleStandHoursGoal_(&self, appleStandHoursGoal: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppleStandHoursGoal : appleStandHoursGoal)
    }
    unsafe fn standHoursGoal(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, standHoursGoal)
    }
    unsafe fn setStandHoursGoal_(&self, standHoursGoal: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStandHoursGoal : standHoursGoal)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKObject(pub id);
impl std::ops::Deref for HKObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKObject {}
impl HKObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKObject").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKObject {}
impl INSObject for HKObject {}
impl PNSObject for HKObject {}
impl std::convert::TryFrom<NSObject> for HKObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKObject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKObject").unwrap()) };
        if is_kind_of {
            Ok(HKObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKObject")
        }
    }
}
impl IHKObject for HKObject {}
pub trait IHKObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn source(&self) -> HKSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn sourceRevision(&self) -> HKSourceRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceRevision)
    }
    unsafe fn device(&self) -> HKDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSample(pub id);
impl std::ops::Deref for HKSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSample {}
impl HKSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSample").unwrap(), alloc) })
    }
}
impl IHKObject for HKSample {}
impl PNSSecureCoding for HKSample {}
impl From<HKSample> for HKObject {
    fn from(child: HKSample) -> HKObject {
        HKObject(child.0)
    }
}
impl std::convert::TryFrom<HKObject> for HKSample {
    type Error = &'static str;
    fn try_from(parent: HKObject) -> Result<HKSample, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSample").unwrap()) };
        if is_kind_of {
            Ok(HKSample(parent.0))
        } else {
            Err("This HKObject cannot be downcasted to HKSample")
        }
    }
}
impl INSObject for HKSample {}
impl PNSObject for HKSample {}
impl IHKSample for HKSample {}
pub trait IHKSample: Sized + std::ops::Deref {
    unsafe fn sampleType(&self) -> HKSampleType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleType)
    }
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
    unsafe fn hasUndeterminedDuration(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasUndeterminedDuration)
    }
}
pub type HKElectrocardiogramLead = NSInteger;
pub type HKElectrocardiogramClassification = NSInteger;
pub type HKElectrocardiogramSymptomsStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKElectrocardiogram(pub id);
impl std::ops::Deref for HKElectrocardiogram {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKElectrocardiogram {}
impl HKElectrocardiogram {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKElectrocardiogram").unwrap(), alloc) })
    }
}
impl IHKSample for HKElectrocardiogram {}
impl From<HKElectrocardiogram> for HKSample {
    fn from(child: HKElectrocardiogram) -> HKSample {
        HKSample(child.0)
    }
}
impl std::convert::TryFrom<HKSample> for HKElectrocardiogram {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKElectrocardiogram, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKElectrocardiogram").unwrap()) };
        if is_kind_of {
            Ok(HKElectrocardiogram(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKElectrocardiogram")
        }
    }
}
impl IHKObject for HKElectrocardiogram {}
impl PNSSecureCoding for HKElectrocardiogram {}
impl INSObject for HKElectrocardiogram {}
impl PNSObject for HKElectrocardiogram {}
impl IHKElectrocardiogram for HKElectrocardiogram {}
pub trait IHKElectrocardiogram: Sized + std::ops::Deref {
    unsafe fn numberOfVoltageMeasurements(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfVoltageMeasurements)
    }
    unsafe fn samplingFrequency(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, samplingFrequency)
    }
    unsafe fn classification(&self) -> HKElectrocardiogramClassification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classification)
    }
    unsafe fn averageHeartRate(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averageHeartRate)
    }
    unsafe fn symptomsStatus(&self) -> HKElectrocardiogramSymptomsStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symptomsStatus)
    }
}
pub type HKFHIRResourceType = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKFHIRResource(pub id);
impl std::ops::Deref for HKFHIRResource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKFHIRResource {}
impl HKFHIRResource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKFHIRResource").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKFHIRResource {}
impl PNSCopying for HKFHIRResource {}
impl INSObject for HKFHIRResource {}
impl PNSObject for HKFHIRResource {}
impl std::convert::TryFrom<NSObject> for HKFHIRResource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKFHIRResource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKFHIRResource").unwrap()) };
        if is_kind_of {
            Ok(HKFHIRResource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKFHIRResource")
        }
    }
}
impl IHKFHIRResource for HKFHIRResource {}
pub trait IHKFHIRResource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn FHIRVersion(&self) -> HKFHIRVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, FHIRVersion)
    }
    unsafe fn resourceType(&self) -> HKFHIRResourceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceType)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn sourceURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceURL)
    }
}
pub type HKMedicationDoseEventLogStatus = NSInteger;
pub type HKMedicationDoseEventScheduleType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKMedicationDoseEvent(pub id);
impl std::ops::Deref for HKMedicationDoseEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKMedicationDoseEvent {}
impl HKMedicationDoseEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKMedicationDoseEvent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKMedicationDoseEvent {}
impl PNSCopying for HKMedicationDoseEvent {}
impl IHKSample for HKMedicationDoseEvent {}
impl std::convert::TryFrom<HKSample> for HKMedicationDoseEvent {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKMedicationDoseEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKMedicationDoseEvent").unwrap()) };
        if is_kind_of {
            Ok(HKMedicationDoseEvent(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKMedicationDoseEvent")
        }
    }
}
impl IHKObject for HKMedicationDoseEvent {}
impl INSObject for HKMedicationDoseEvent {}
impl PNSObject for HKMedicationDoseEvent {}
impl IHKMedicationDoseEvent for HKMedicationDoseEvent {}
pub trait IHKMedicationDoseEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn medicationDoseEventType(&self) -> HKMedicationDoseEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, medicationDoseEventType)
    }
    unsafe fn scheduleType(&self) -> HKMedicationDoseEventScheduleType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduleType)
    }
    unsafe fn medicationConceptIdentifier(&self) -> HKHealthConceptIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, medicationConceptIdentifier)
    }
    unsafe fn scheduledDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduledDate)
    }
    unsafe fn scheduledDoseQuantity(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduledDoseQuantity)
    }
    unsafe fn doseQuantity(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doseQuantity)
    }
    unsafe fn logStatus(&self) -> HKMedicationDoseEventLogStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, logStatus)
    }
    unsafe fn unit(&self) -> HKUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unit)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKMedicationDoseEvent").unwrap(), new)
    }
}
pub type HKStateOfMindValenceClassification = NSInteger;
pub type HKStateOfMindLabel = NSInteger;
pub type HKStateOfMindAssociation = NSInteger;
pub type HKStateOfMindKind = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKStateOfMind(pub id);
impl std::ops::Deref for HKStateOfMind {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKStateOfMind {}
impl HKStateOfMind {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKStateOfMind").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKStateOfMind {}
impl PNSCopying for HKStateOfMind {}
impl IHKSample for HKStateOfMind {}
impl std::convert::TryFrom<HKSample> for HKStateOfMind {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKStateOfMind, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKStateOfMind").unwrap()) };
        if is_kind_of {
            Ok(HKStateOfMind(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKStateOfMind")
        }
    }
}
impl IHKObject for HKStateOfMind {}
impl INSObject for HKStateOfMind {}
impl PNSObject for HKStateOfMind {}
impl IHKStateOfMind for HKStateOfMind {}
pub trait IHKStateOfMind: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn kind(&self) -> HKStateOfMindKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kind)
    }
    unsafe fn valence(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valence)
    }
    unsafe fn valenceClassification(&self) -> HKStateOfMindValenceClassification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valenceClassification)
    }
    unsafe fn labels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, labels)
    }
    unsafe fn associations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, associations)
    }
    unsafe fn stateOfMindWithDate_kind_valence_labels_associations_(
        date: NSDate,
        kind: HKStateOfMindKind,
        valence: f64,
        labels: NSArray,
        associations: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKStateOfMind").unwrap(), stateOfMindWithDate : date, kind : kind, valence : valence, labels : labels, associations : associations)
    }
    unsafe fn stateOfMindWithDate_kind_valence_labels_associations_metadata_(
        date: NSDate,
        kind: HKStateOfMindKind,
        valence: f64,
        labels: NSArray,
        associations: NSArray,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKStateOfMind").unwrap(), stateOfMindWithDate : date, kind : kind, valence : valence, labels : labels, associations : associations, metadata : metadata)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKStateOfMind").unwrap(), new)
    }
}
pub type HKWorkoutActivityType = NSUInteger;
pub type HKWorkoutEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutEvent(pub id);
impl std::ops::Deref for HKWorkoutEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutEvent {}
impl HKWorkoutEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutEvent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKWorkoutEvent {}
impl PNSCopying for HKWorkoutEvent {}
impl INSObject for HKWorkoutEvent {}
impl PNSObject for HKWorkoutEvent {}
impl std::convert::TryFrom<NSObject> for HKWorkoutEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKWorkoutEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutEvent").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKWorkoutEvent")
        }
    }
}
impl IHKWorkoutEvent for HKWorkoutEvent {}
pub trait IHKWorkoutEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn type_(&self) -> HKWorkoutEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn dateInterval(&self) -> NSDateInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateInterval)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn workoutEventWithType_date_(type_: HKWorkoutEventType, date: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutEvent").unwrap(), workoutEventWithType : type_, date : date)
    }
    unsafe fn workoutEventWithType_date_metadata_(
        type_: HKWorkoutEventType,
        date: NSDate,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutEvent").unwrap(), workoutEventWithType : type_, date : date, metadata : metadata)
    }
    unsafe fn workoutEventWithType_dateInterval_metadata_(
        type_: HKWorkoutEventType,
        dateInterval: NSDateInterval,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutEvent").unwrap(), workoutEventWithType : type_, dateInterval : dateInterval, metadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkout(pub id);
impl std::ops::Deref for HKWorkout {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkout {}
impl HKWorkout {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkout").unwrap(), alloc) })
    }
}
impl IHKSample for HKWorkout {}
impl std::convert::TryFrom<HKSample> for HKWorkout {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKWorkout, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkout").unwrap()) };
        if is_kind_of {
            Ok(HKWorkout(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKWorkout")
        }
    }
}
impl IHKObject for HKWorkout {}
impl PNSSecureCoding for HKWorkout {}
impl INSObject for HKWorkout {}
impl PNSObject for HKWorkout {}
impl IHKWorkout for HKWorkout {}
pub trait IHKWorkout: Sized + std::ops::Deref {
    unsafe fn statisticsForType_(&self, quantityType: HKQuantityType) -> HKStatistics
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, statisticsForType : quantityType)
    }
    unsafe fn workoutActivityType(&self) -> HKWorkoutActivityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutActivityType)
    }
    unsafe fn workoutEvents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutEvents)
    }
    unsafe fn workoutActivities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutActivities)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn totalEnergyBurned(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalEnergyBurned)
    }
    unsafe fn totalDistance(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalDistance)
    }
    unsafe fn totalSwimmingStrokeCount(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalSwimmingStrokeCount)
    }
    unsafe fn totalFlightsClimbed(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalFlightsClimbed)
    }
    unsafe fn allStatistics(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allStatistics)
    }
    unsafe fn workoutWithActivityType_startDate_endDate_(
        workoutActivityType: HKWorkoutActivityType,
        startDate: NSDate,
        endDate: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkout").unwrap(), workoutWithActivityType : workoutActivityType, startDate : startDate, endDate : endDate)
    }
    unsafe fn workoutWithActivityType_startDate_endDate_workoutEvents_totalEnergyBurned_totalDistance_metadata_(
        workoutActivityType: HKWorkoutActivityType,
        startDate: NSDate,
        endDate: NSDate,
        workoutEvents: NSArray,
        totalEnergyBurned: HKQuantity,
        totalDistance: HKQuantity,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkout").unwrap(), workoutWithActivityType : workoutActivityType, startDate : startDate, endDate : endDate, workoutEvents : workoutEvents, totalEnergyBurned : totalEnergyBurned, totalDistance : totalDistance, metadata : metadata)
    }
    unsafe fn workoutWithActivityType_startDate_endDate_workoutEvents_totalEnergyBurned_totalDistance_device_metadata_(
        workoutActivityType: HKWorkoutActivityType,
        startDate: NSDate,
        endDate: NSDate,
        workoutEvents: NSArray,
        totalEnergyBurned: HKQuantity,
        totalDistance: HKQuantity,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkout").unwrap(), workoutWithActivityType : workoutActivityType, startDate : startDate, endDate : endDate, workoutEvents : workoutEvents, totalEnergyBurned : totalEnergyBurned, totalDistance : totalDistance, device : device, metadata : metadata)
    }
    unsafe fn workoutWithActivityType_startDate_endDate_duration_totalEnergyBurned_totalDistance_metadata_(
        workoutActivityType: HKWorkoutActivityType,
        startDate: NSDate,
        endDate: NSDate,
        duration: NSTimeInterval,
        totalEnergyBurned: HKQuantity,
        totalDistance: HKQuantity,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkout").unwrap(), workoutWithActivityType : workoutActivityType, startDate : startDate, endDate : endDate, duration : duration, totalEnergyBurned : totalEnergyBurned, totalDistance : totalDistance, metadata : metadata)
    }
    unsafe fn workoutWithActivityType_startDate_endDate_duration_totalEnergyBurned_totalDistance_device_metadata_(
        workoutActivityType: HKWorkoutActivityType,
        startDate: NSDate,
        endDate: NSDate,
        duration: NSTimeInterval,
        totalEnergyBurned: HKQuantity,
        totalDistance: HKQuantity,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkout").unwrap(), workoutWithActivityType : workoutActivityType, startDate : startDate, endDate : endDate, duration : duration, totalEnergyBurned : totalEnergyBurned, totalDistance : totalDistance, device : device, metadata : metadata)
    }
    unsafe fn workoutWithActivityType_startDate_endDate_workoutEvents_totalEnergyBurned_totalDistance_totalSwimmingStrokeCount_device_metadata_(
        workoutActivityType: HKWorkoutActivityType,
        startDate: NSDate,
        endDate: NSDate,
        workoutEvents: NSArray,
        totalEnergyBurned: HKQuantity,
        totalDistance: HKQuantity,
        totalSwimmingStrokeCount: HKQuantity,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkout").unwrap(), workoutWithActivityType : workoutActivityType, startDate : startDate, endDate : endDate, workoutEvents : workoutEvents, totalEnergyBurned : totalEnergyBurned, totalDistance : totalDistance, totalSwimmingStrokeCount : totalSwimmingStrokeCount, device : device, metadata : metadata)
    }
    unsafe fn workoutWithActivityType_startDate_endDate_workoutEvents_totalEnergyBurned_totalDistance_totalFlightsClimbed_device_metadata_(
        workoutActivityType: HKWorkoutActivityType,
        startDate: NSDate,
        endDate: NSDate,
        workoutEvents: NSArray,
        totalEnergyBurned: HKQuantity,
        totalDistance: HKQuantity,
        totalFlightsClimbed: HKQuantity,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkout").unwrap(), workoutWithActivityType : workoutActivityType, startDate : startDate, endDate : endDate, workoutEvents : workoutEvents, totalEnergyBurned : totalEnergyBurned, totalDistance : totalDistance, totalFlightsClimbed : totalFlightsClimbed, device : device, metadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKQuery(pub id);
impl std::ops::Deref for HKQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKQuery {}
impl HKQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), alloc) })
    }
}
impl INSObject for HKQuery {}
impl PNSObject for HKQuery {}
impl std::convert::TryFrom<NSObject> for HKQuery {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKQuery, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKQuery").unwrap()) };
        if is_kind_of {
            Ok(HKQuery(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKQuery")
        }
    }
}
impl IHKQuery for HKQuery {}
pub trait IHKQuery: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn objectType(&self) -> HKObjectType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectType)
    }
    unsafe fn sampleType(&self) -> HKSampleType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleType)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
    }
}
pub type HKQueryOptions = NSUInteger;
impl HKQuery_HKObjectPredicates for HKQuery {}
pub trait HKQuery_HKObjectPredicates: Sized + std::ops::Deref {
    unsafe fn predicateForObjectsWithMetadataKey_(key: NSString) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsWithMetadataKey : key)
    }
    unsafe fn predicateForObjectsWithMetadataKey_allowedValues_(
        key: NSString,
        allowedValues: NSArray,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsWithMetadataKey : key, allowedValues : allowedValues)
    }
    unsafe fn predicateForObjectsWithMetadataKey_operatorType_value_(
        key: NSString,
        operatorType: NSPredicateOperatorType,
        value: id,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsWithMetadataKey : key, operatorType : operatorType, value : value)
    }
    unsafe fn predicateForObjectsFromSource_(source: HKSource) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsFromSource : source)
    }
    unsafe fn predicateForObjectsFromSources_(sources: NSSet) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsFromSources : sources)
    }
    unsafe fn predicateForObjectsFromSourceRevisions_(sourceRevisions: NSSet) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsFromSourceRevisions : sourceRevisions)
    }
    unsafe fn predicateForObjectsFromDevices_(devices: NSSet) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsFromDevices : devices)
    }
    unsafe fn predicateForObjectsWithDeviceProperty_allowedValues_(
        key: NSString,
        allowedValues: NSSet,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsWithDeviceProperty : key, allowedValues : allowedValues)
    }
    unsafe fn predicateForObjectWithUUID_(UUID: NSUUID) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectWithUUID : UUID)
    }
    unsafe fn predicateForObjectsWithUUIDs_(UUIDs: NSSet) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsWithUUIDs : UUIDs)
    }
    unsafe fn predicateForObjectsWithNoCorrelation() -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsWithNoCorrelation)
    }
    unsafe fn predicateForObjectsFromWorkout_(workout: HKWorkout) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsFromWorkout : workout)
    }
    unsafe fn predicateForObjectsAssociatedWithElectrocardiogram_(
        electrocardiogram: HKElectrocardiogram,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForObjectsAssociatedWithElectrocardiogram : electrocardiogram)
    }
    unsafe fn predicateForWorkoutEffortSamplesRelatedToWorkout_activity_(
        workout: HKWorkout,
        activity: HKWorkoutActivity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutEffortSamplesRelatedToWorkout : workout, activity : activity)
    }
}
impl HKQuery_HKSamplePredicates for HKQuery {}
pub trait HKQuery_HKSamplePredicates: Sized + std::ops::Deref {
    unsafe fn predicateForSamplesWithStartDate_endDate_options_(
        startDate: NSDate,
        endDate: NSDate,
        options: HKQueryOptions,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForSamplesWithStartDate : startDate, endDate : endDate, options : options)
    }
}
impl HKQuery_HKQuantitySamplePredicates for HKQuery {}
pub trait HKQuery_HKQuantitySamplePredicates: Sized + std::ops::Deref {
    unsafe fn predicateForQuantitySamplesWithOperatorType_quantity_(
        operatorType: NSPredicateOperatorType,
        quantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForQuantitySamplesWithOperatorType : operatorType, quantity : quantity)
    }
}
impl HKQuery_HKCategorySamplePredicates for HKQuery {}
pub trait HKQuery_HKCategorySamplePredicates: Sized + std::ops::Deref {
    unsafe fn predicateForCategorySamplesWithOperatorType_value_(
        operatorType: NSPredicateOperatorType,
        value: NSInteger,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForCategorySamplesWithOperatorType : operatorType, value : value)
    }
    unsafe fn predicateForCategorySamplesEqualToValues_(values: NSSet) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForCategorySamplesEqualToValues : values)
    }
}
impl HKQuery_HKWorkoutPredicates for HKQuery {}
pub trait HKQuery_HKWorkoutPredicates: Sized + std::ops::Deref {
    unsafe fn predicateForWorkoutsWithWorkoutActivityType_(
        workoutActivityType: HKWorkoutActivityType,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithWorkoutActivityType : workoutActivityType)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_duration_(
        operatorType: NSPredicateOperatorType,
        duration: NSTimeInterval,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, duration : duration)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_totalEnergyBurned_(
        operatorType: NSPredicateOperatorType,
        totalEnergyBurned: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, totalEnergyBurned : totalEnergyBurned)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_totalDistance_(
        operatorType: NSPredicateOperatorType,
        totalDistance: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, totalDistance : totalDistance)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_totalSwimmingStrokeCount_(
        operatorType: NSPredicateOperatorType,
        totalSwimmingStrokeCount: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, totalSwimmingStrokeCount : totalSwimmingStrokeCount)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_totalFlightsClimbed_(
        operatorType: NSPredicateOperatorType,
        totalFlightsClimbed: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, totalFlightsClimbed : totalFlightsClimbed)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_quantityType_sumQuantity_(
        operatorType: NSPredicateOperatorType,
        quantityType: HKQuantityType,
        sumQuantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, quantityType : quantityType, sumQuantity : sumQuantity)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_quantityType_minimumQuantity_(
        operatorType: NSPredicateOperatorType,
        quantityType: HKQuantityType,
        minimumQuantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, quantityType : quantityType, minimumQuantity : minimumQuantity)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_quantityType_maximumQuantity_(
        operatorType: NSPredicateOperatorType,
        quantityType: HKQuantityType,
        maximumQuantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, quantityType : quantityType, maximumQuantity : maximumQuantity)
    }
    unsafe fn predicateForWorkoutsWithOperatorType_quantityType_averageQuantity_(
        operatorType: NSPredicateOperatorType,
        quantityType: HKQuantityType,
        averageQuantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithOperatorType : operatorType, quantityType : quantityType, averageQuantity : averageQuantity)
    }
}
impl HKQuery_HKWorkoutActivityPredicates for HKQuery {}
pub trait HKQuery_HKWorkoutActivityPredicates: Sized + std::ops::Deref {
    unsafe fn predicateForWorkoutActivitiesWithWorkoutActivityType_(
        workoutActivityType: HKWorkoutActivityType,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutActivitiesWithWorkoutActivityType : workoutActivityType)
    }
    unsafe fn predicateForWorkoutActivitiesWithOperatorType_duration_(
        operatorType: NSPredicateOperatorType,
        duration: NSTimeInterval,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutActivitiesWithOperatorType : operatorType, duration : duration)
    }
    unsafe fn predicateForWorkoutActivitiesWithStartDate_endDate_options_(
        startDate: NSDate,
        endDate: NSDate,
        options: HKQueryOptions,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutActivitiesWithStartDate : startDate, endDate : endDate, options : options)
    }
    unsafe fn predicateForWorkoutActivitiesWithOperatorType_quantityType_sumQuantity_(
        operatorType: NSPredicateOperatorType,
        quantityType: HKQuantityType,
        sumQuantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutActivitiesWithOperatorType : operatorType, quantityType : quantityType, sumQuantity : sumQuantity)
    }
    unsafe fn predicateForWorkoutActivitiesWithOperatorType_quantityType_minimumQuantity_(
        operatorType: NSPredicateOperatorType,
        quantityType: HKQuantityType,
        minimumQuantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutActivitiesWithOperatorType : operatorType, quantityType : quantityType, minimumQuantity : minimumQuantity)
    }
    unsafe fn predicateForWorkoutActivitiesWithOperatorType_quantityType_maximumQuantity_(
        operatorType: NSPredicateOperatorType,
        quantityType: HKQuantityType,
        maximumQuantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutActivitiesWithOperatorType : operatorType, quantityType : quantityType, maximumQuantity : maximumQuantity)
    }
    unsafe fn predicateForWorkoutActivitiesWithOperatorType_quantityType_averageQuantity_(
        operatorType: NSPredicateOperatorType,
        quantityType: HKQuantityType,
        averageQuantity: HKQuantity,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutActivitiesWithOperatorType : operatorType, quantityType : quantityType, averageQuantity : averageQuantity)
    }
    unsafe fn predicateForWorkoutsWithActivityPredicate_(
        activityPredicate: NSPredicate,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForWorkoutsWithActivityPredicate : activityPredicate)
    }
}
impl HKQuery_HKActivitySummaryPredicates for HKQuery {}
pub trait HKQuery_HKActivitySummaryPredicates: Sized + std::ops::Deref {
    unsafe fn predicateForActivitySummaryWithDateComponents_(
        dateComponents: NSDateComponents,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForActivitySummaryWithDateComponents : dateComponents)
    }
    unsafe fn predicateForActivitySummariesBetweenStartDateComponents_endDateComponents_(
        startDateComponents: NSDateComponents,
        endDateComponents: NSDateComponents,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForActivitySummariesBetweenStartDateComponents : startDateComponents, endDateComponents : endDateComponents)
    }
}
impl HKQuery_HKClinicalRecordPredicates for HKQuery {}
pub trait HKQuery_HKClinicalRecordPredicates: Sized + std::ops::Deref {
    unsafe fn predicateForClinicalRecordsWithFHIRResourceType_(
        resourceType: NSString,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForClinicalRecordsWithFHIRResourceType : resourceType)
    }
    unsafe fn predicateForClinicalRecordsFromSource_FHIRResourceType_identifier_(
        source: HKSource,
        resourceType: NSString,
        identifier: NSString,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForClinicalRecordsFromSource : source, FHIRResourceType : resourceType, identifier : identifier)
    }
}
impl HKQuery_HKElectrocardiogramPredicates for HKQuery {}
pub trait HKQuery_HKElectrocardiogramPredicates: Sized + std::ops::Deref {
    unsafe fn predicateForElectrocardiogramsWithClassification_(
        classification: HKElectrocardiogramClassification,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForElectrocardiogramsWithClassification : classification)
    }
    unsafe fn predicateForElectrocardiogramsWithSymptomsStatus_(
        symptomsStatus: HKElectrocardiogramSymptomsStatus,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForElectrocardiogramsWithSymptomsStatus : symptomsStatus)
    }
}
impl HKQuery_HKVerifiableClinicalRecordPredicates for HKQuery {}
pub trait HKQuery_HKVerifiableClinicalRecordPredicates: Sized + std::ops::Deref {
    unsafe fn predicateForVerifiableClinicalRecordsWithRelevantDateWithinDateInterval_(
        dateInterval: NSDateInterval,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForVerifiableClinicalRecordsWithRelevantDateWithinDateInterval : dateInterval)
    }
}
impl HKQuery_HKStateOfMind for HKQuery {}
pub trait HKQuery_HKStateOfMind: Sized + std::ops::Deref {
    unsafe fn predicateForStatesOfMindWithValence_operatorType_(
        valence: f64,
        operatorType: NSPredicateOperatorType,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForStatesOfMindWithValence : valence, operatorType : operatorType)
    }
    unsafe fn predicateForStatesOfMindWithKind_(kind: HKStateOfMindKind) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForStatesOfMindWithKind : kind)
    }
    unsafe fn predicateForStatesOfMindWithLabel_(label: HKStateOfMindLabel) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForStatesOfMindWithLabel : label)
    }
    unsafe fn predicateForStatesOfMindWithAssociation_(
        association: HKStateOfMindAssociation,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForStatesOfMindWithAssociation : association)
    }
}
impl HKQuery_HKMedicationDoseEvent for HKQuery {}
pub trait HKQuery_HKMedicationDoseEvent: Sized + std::ops::Deref {
    unsafe fn predicateForMedicationDoseEventWithStatus_(
        status: HKMedicationDoseEventLogStatus,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForMedicationDoseEventWithStatus : status)
    }
    unsafe fn predicateForMedicationDoseEventWithStatuses_(statuses: NSSet) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForMedicationDoseEventWithStatuses : statuses)
    }
    unsafe fn predicateForMedicationDoseEventWithScheduledDate_(
        scheduledDate: NSDate,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForMedicationDoseEventWithScheduledDate : scheduledDate)
    }
    unsafe fn predicateForMedicationDoseEventWithScheduledDates_(
        scheduledDates: NSSet,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForMedicationDoseEventWithScheduledDates : scheduledDates)
    }
    unsafe fn predicateForMedicationDoseEventWithScheduledStartDate_endDate_(
        startDate: NSDate,
        endDate: NSDate,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForMedicationDoseEventWithScheduledStartDate : startDate, endDate : endDate)
    }
    unsafe fn predicateForMedicationDoseEventWithMedicationConceptIdentifier_(
        medicationConceptIdentifier: HKHealthConceptIdentifier,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForMedicationDoseEventWithMedicationConceptIdentifier : medicationConceptIdentifier)
    }
    unsafe fn predicateForMedicationDoseEventWithMedicationConceptIdentifiers_(
        medicationConceptIdentifiers: NSSet,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForMedicationDoseEventWithMedicationConceptIdentifiers : medicationConceptIdentifiers)
    }
}
impl HKQuery_HKUserAnnotatedMedications for HKQuery {}
pub trait HKQuery_HKUserAnnotatedMedications: Sized + std::ops::Deref {
    unsafe fn predicateForUserAnnotatedMedicationsWithIsArchived_(isArchived: BOOL) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForUserAnnotatedMedicationsWithIsArchived : isArchived)
    }
    unsafe fn predicateForUserAnnotatedMedicationsWithHasSchedule_(hasSchedule: BOOL) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuery").unwrap(), predicateForUserAnnotatedMedicationsWithHasSchedule : hasSchedule)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKActivitySummaryQuery(pub id);
impl std::ops::Deref for HKActivitySummaryQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKActivitySummaryQuery {}
impl HKActivitySummaryQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKActivitySummaryQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKActivitySummaryQuery {}
impl From<HKActivitySummaryQuery> for HKQuery {
    fn from(child: HKActivitySummaryQuery) -> HKQuery {
        HKQuery(child.0)
    }
}
impl std::convert::TryFrom<HKQuery> for HKActivitySummaryQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKActivitySummaryQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKActivitySummaryQuery").unwrap()) };
        if is_kind_of {
            Ok(HKActivitySummaryQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKActivitySummaryQuery")
        }
    }
}
impl INSObject for HKActivitySummaryQuery {}
impl PNSObject for HKActivitySummaryQuery {}
impl IHKActivitySummaryQuery for HKActivitySummaryQuery {}
pub trait IHKActivitySummaryQuery: Sized + std::ops::Deref {
    unsafe fn initWithPredicate_resultsHandler_(
        &self,
        predicate: NSPredicate,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPredicate : predicate, resultsHandler : handler)
    }
    unsafe fn updateHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateHandler)
    }
    unsafe fn setUpdateHandler_(&self, updateHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpdateHandler : updateHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKQueryAnchor(pub id);
impl std::ops::Deref for HKQueryAnchor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKQueryAnchor {}
impl HKQueryAnchor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKQueryAnchor").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKQueryAnchor {}
impl PNSCopying for HKQueryAnchor {}
impl INSObject for HKQueryAnchor {}
impl PNSObject for HKQueryAnchor {}
impl std::convert::TryFrom<NSObject> for HKQueryAnchor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKQueryAnchor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKQueryAnchor").unwrap()) };
        if is_kind_of {
            Ok(HKQueryAnchor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKQueryAnchor")
        }
    }
}
impl IHKQueryAnchor for HKQueryAnchor {}
pub trait IHKQueryAnchor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn anchorFromValue_(value: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQueryAnchor").unwrap(), anchorFromValue : value)
    }
}
pub type HKQuantityAggregationStyle = NSInteger;
pub type HKQuantityTypeIdentifier = NSString;
pub type HKCategoryTypeIdentifier = NSString;
pub type HKCharacteristicTypeIdentifier = NSString;
pub type HKCorrelationTypeIdentifier = NSString;
pub type HKDocumentTypeIdentifier = NSString;
pub type HKScoredAssessmentTypeIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKObjectType(pub id);
impl std::ops::Deref for HKObjectType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKObjectType {}
impl HKObjectType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKObjectType {}
impl PNSCopying for HKObjectType {}
impl INSObject for HKObjectType {}
impl PNSObject for HKObjectType {}
impl std::convert::TryFrom<NSObject> for HKObjectType {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKObjectType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKObjectType").unwrap()) };
        if is_kind_of {
            Ok(HKObjectType(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKObjectType")
        }
    }
}
impl IHKObjectType for HKObjectType {}
pub trait IHKObjectType: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn requiresPerObjectAuthorization(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresPerObjectAuthorization)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn quantityTypeForIdentifier_(identifier: NSString) -> HKQuantityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), quantityTypeForIdentifier : identifier)
    }
    unsafe fn categoryTypeForIdentifier_(identifier: NSString) -> HKCategoryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), categoryTypeForIdentifier : identifier)
    }
    unsafe fn characteristicTypeForIdentifier_(identifier: NSString) -> HKCharacteristicType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), characteristicTypeForIdentifier : identifier)
    }
    unsafe fn correlationTypeForIdentifier_(identifier: NSString) -> HKCorrelationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), correlationTypeForIdentifier : identifier)
    }
    unsafe fn documentTypeForIdentifier_(identifier: NSString) -> HKDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), documentTypeForIdentifier : identifier)
    }
    unsafe fn scoredAssessmentTypeForIdentifier_(identifier: NSString) -> HKScoredAssessmentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), scoredAssessmentTypeForIdentifier : identifier)
    }
    unsafe fn seriesTypeForIdentifier_(identifier: NSString) -> HKSeriesType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), seriesTypeForIdentifier : identifier)
    }
    unsafe fn workoutType() -> HKWorkoutType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), workoutType)
    }
    unsafe fn activitySummaryType() -> HKActivitySummaryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), activitySummaryType)
    }
    unsafe fn audiogramSampleType() -> HKAudiogramSampleType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), audiogramSampleType)
    }
    unsafe fn electrocardiogramType() -> HKElectrocardiogramType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), electrocardiogramType)
    }
    unsafe fn medicationDoseEventType() -> HKMedicationDoseEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), medicationDoseEventType)
    }
    unsafe fn visionPrescriptionType() -> HKPrescriptionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), visionPrescriptionType)
    }
    unsafe fn stateOfMindType() -> HKStateOfMindType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), stateOfMindType)
    }
    unsafe fn userAnnotatedMedicationType() -> HKUserAnnotatedMedicationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), userAnnotatedMedicationType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCharacteristicType(pub id);
impl std::ops::Deref for HKCharacteristicType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCharacteristicType {}
impl HKCharacteristicType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCharacteristicType").unwrap(), alloc) })
    }
}
impl IHKObjectType for HKCharacteristicType {}
impl PNSSecureCoding for HKCharacteristicType {}
impl PNSCopying for HKCharacteristicType {}
impl From<HKCharacteristicType> for HKObjectType {
    fn from(child: HKCharacteristicType) -> HKObjectType {
        HKObjectType(child.0)
    }
}
impl std::convert::TryFrom<HKObjectType> for HKCharacteristicType {
    type Error = &'static str;
    fn try_from(parent: HKObjectType) -> Result<HKCharacteristicType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCharacteristicType").unwrap()) };
        if is_kind_of {
            Ok(HKCharacteristicType(parent.0))
        } else {
            Err("This HKObjectType cannot be downcasted to HKCharacteristicType")
        }
    }
}
impl INSObject for HKCharacteristicType {}
impl PNSObject for HKCharacteristicType {}
impl IHKCharacteristicType for HKCharacteristicType {}
pub trait IHKCharacteristicType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSampleType(pub id);
impl std::ops::Deref for HKSampleType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSampleType {}
impl HKSampleType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSampleType").unwrap(), alloc) })
    }
}
impl IHKObjectType for HKSampleType {}
impl PNSSecureCoding for HKSampleType {}
impl PNSCopying for HKSampleType {}
impl std::convert::TryFrom<HKObjectType> for HKSampleType {
    type Error = &'static str;
    fn try_from(parent: HKObjectType) -> Result<HKSampleType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSampleType").unwrap()) };
        if is_kind_of {
            Ok(HKSampleType(parent.0))
        } else {
            Err("This HKObjectType cannot be downcasted to HKSampleType")
        }
    }
}
impl INSObject for HKSampleType {}
impl PNSObject for HKSampleType {}
impl IHKSampleType for HKSampleType {}
pub trait IHKSampleType: Sized + std::ops::Deref {
    unsafe fn isMaximumDurationRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMaximumDurationRestricted)
    }
    unsafe fn maximumAllowedDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumAllowedDuration)
    }
    unsafe fn isMinimumDurationRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMinimumDurationRestricted)
    }
    unsafe fn minimumAllowedDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumAllowedDuration)
    }
    unsafe fn allowsRecalibrationForEstimates(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsRecalibrationForEstimates)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCategoryType(pub id);
impl std::ops::Deref for HKCategoryType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCategoryType {}
impl HKCategoryType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCategoryType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKCategoryType {}
impl From<HKCategoryType> for HKSampleType {
    fn from(child: HKCategoryType) -> HKSampleType {
        HKSampleType(child.0)
    }
}
impl std::convert::TryFrom<HKSampleType> for HKCategoryType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKCategoryType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCategoryType").unwrap()) };
        if is_kind_of {
            Ok(HKCategoryType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKCategoryType")
        }
    }
}
impl IHKObjectType for HKCategoryType {}
impl PNSSecureCoding for HKCategoryType {}
impl PNSCopying for HKCategoryType {}
impl INSObject for HKCategoryType {}
impl PNSObject for HKCategoryType {}
impl IHKCategoryType for HKCategoryType {}
pub trait IHKCategoryType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCorrelationType(pub id);
impl std::ops::Deref for HKCorrelationType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCorrelationType {}
impl HKCorrelationType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCorrelationType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKCorrelationType {}
impl std::convert::TryFrom<HKSampleType> for HKCorrelationType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKCorrelationType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCorrelationType").unwrap()) };
        if is_kind_of {
            Ok(HKCorrelationType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKCorrelationType")
        }
    }
}
impl IHKObjectType for HKCorrelationType {}
impl PNSSecureCoding for HKCorrelationType {}
impl PNSCopying for HKCorrelationType {}
impl INSObject for HKCorrelationType {}
impl PNSObject for HKCorrelationType {}
impl IHKCorrelationType for HKCorrelationType {}
pub trait IHKCorrelationType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKDocumentType(pub id);
impl std::ops::Deref for HKDocumentType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKDocumentType {}
impl HKDocumentType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKDocumentType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKDocumentType {}
impl std::convert::TryFrom<HKSampleType> for HKDocumentType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKDocumentType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKDocumentType").unwrap()) };
        if is_kind_of {
            Ok(HKDocumentType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKDocumentType")
        }
    }
}
impl IHKObjectType for HKDocumentType {}
impl PNSSecureCoding for HKDocumentType {}
impl PNSCopying for HKDocumentType {}
impl INSObject for HKDocumentType {}
impl PNSObject for HKDocumentType {}
impl IHKDocumentType for HKDocumentType {}
pub trait IHKDocumentType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKQuantityType(pub id);
impl std::ops::Deref for HKQuantityType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKQuantityType {}
impl HKQuantityType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantityType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKQuantityType {}
impl std::convert::TryFrom<HKSampleType> for HKQuantityType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKQuantityType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKQuantityType").unwrap()) };
        if is_kind_of {
            Ok(HKQuantityType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKQuantityType")
        }
    }
}
impl IHKObjectType for HKQuantityType {}
impl PNSSecureCoding for HKQuantityType {}
impl PNSCopying for HKQuantityType {}
impl INSObject for HKQuantityType {}
impl PNSObject for HKQuantityType {}
impl IHKQuantityType for HKQuantityType {}
pub trait IHKQuantityType: Sized + std::ops::Deref {
    unsafe fn isCompatibleWithUnit_(&self, unit: HKUnit) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isCompatibleWithUnit : unit)
    }
    unsafe fn aggregationStyle(&self) -> HKQuantityAggregationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aggregationStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutType(pub id);
impl std::ops::Deref for HKWorkoutType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutType {}
impl HKWorkoutType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKWorkoutType {}
impl std::convert::TryFrom<HKSampleType> for HKWorkoutType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKWorkoutType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutType").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKWorkoutType")
        }
    }
}
impl IHKObjectType for HKWorkoutType {}
impl PNSSecureCoding for HKWorkoutType {}
impl PNSCopying for HKWorkoutType {}
impl INSObject for HKWorkoutType {}
impl PNSObject for HKWorkoutType {}
impl IHKWorkoutType for HKWorkoutType {}
pub trait IHKWorkoutType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSeriesType(pub id);
impl std::ops::Deref for HKSeriesType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSeriesType {}
impl HKSeriesType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSeriesType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKSeriesType {}
impl std::convert::TryFrom<HKSampleType> for HKSeriesType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKSeriesType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSeriesType").unwrap()) };
        if is_kind_of {
            Ok(HKSeriesType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKSeriesType")
        }
    }
}
impl IHKObjectType for HKSeriesType {}
impl PNSSecureCoding for HKSeriesType {}
impl PNSCopying for HKSeriesType {}
impl INSObject for HKSeriesType {}
impl PNSObject for HKSeriesType {}
impl IHKSeriesType for HKSeriesType {}
pub trait IHKSeriesType: Sized + std::ops::Deref {
    unsafe fn workoutRouteType() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKSeriesType").unwrap(), workoutRouteType)
    }
    unsafe fn heartbeatSeriesType() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKSeriesType").unwrap(), heartbeatSeriesType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKActivitySummaryType(pub id);
impl std::ops::Deref for HKActivitySummaryType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKActivitySummaryType {}
impl HKActivitySummaryType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKActivitySummaryType").unwrap(), alloc) })
    }
}
impl IHKObjectType for HKActivitySummaryType {}
impl PNSSecureCoding for HKActivitySummaryType {}
impl PNSCopying for HKActivitySummaryType {}
impl std::convert::TryFrom<HKObjectType> for HKActivitySummaryType {
    type Error = &'static str;
    fn try_from(parent: HKObjectType) -> Result<HKActivitySummaryType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKActivitySummaryType").unwrap()) };
        if is_kind_of {
            Ok(HKActivitySummaryType(parent.0))
        } else {
            Err("This HKObjectType cannot be downcasted to HKActivitySummaryType")
        }
    }
}
impl INSObject for HKActivitySummaryType {}
impl PNSObject for HKActivitySummaryType {}
impl IHKActivitySummaryType for HKActivitySummaryType {}
pub trait IHKActivitySummaryType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKAudiogramSampleType(pub id);
impl std::ops::Deref for HKAudiogramSampleType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKAudiogramSampleType {}
impl HKAudiogramSampleType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSampleType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKAudiogramSampleType {}
impl std::convert::TryFrom<HKSampleType> for HKAudiogramSampleType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKAudiogramSampleType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKAudiogramSampleType").unwrap()) };
        if is_kind_of {
            Ok(HKAudiogramSampleType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKAudiogramSampleType")
        }
    }
}
impl IHKObjectType for HKAudiogramSampleType {}
impl PNSSecureCoding for HKAudiogramSampleType {}
impl PNSCopying for HKAudiogramSampleType {}
impl INSObject for HKAudiogramSampleType {}
impl PNSObject for HKAudiogramSampleType {}
impl IHKAudiogramSampleType for HKAudiogramSampleType {}
pub trait IHKAudiogramSampleType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKElectrocardiogramType(pub id);
impl std::ops::Deref for HKElectrocardiogramType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKElectrocardiogramType {}
impl HKElectrocardiogramType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKElectrocardiogramType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKElectrocardiogramType {}
impl std::convert::TryFrom<HKSampleType> for HKElectrocardiogramType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKElectrocardiogramType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKElectrocardiogramType").unwrap()) };
        if is_kind_of {
            Ok(HKElectrocardiogramType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKElectrocardiogramType")
        }
    }
}
impl IHKObjectType for HKElectrocardiogramType {}
impl PNSSecureCoding for HKElectrocardiogramType {}
impl PNSCopying for HKElectrocardiogramType {}
impl INSObject for HKElectrocardiogramType {}
impl PNSObject for HKElectrocardiogramType {}
impl IHKElectrocardiogramType for HKElectrocardiogramType {}
pub trait IHKElectrocardiogramType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKMedicationDoseEventType(pub id);
impl std::ops::Deref for HKMedicationDoseEventType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKMedicationDoseEventType {}
impl HKMedicationDoseEventType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKMedicationDoseEventType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKMedicationDoseEventType {}
impl std::convert::TryFrom<HKSampleType> for HKMedicationDoseEventType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKMedicationDoseEventType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKMedicationDoseEventType").unwrap()) };
        if is_kind_of {
            Ok(HKMedicationDoseEventType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKMedicationDoseEventType")
        }
    }
}
impl IHKObjectType for HKMedicationDoseEventType {}
impl PNSSecureCoding for HKMedicationDoseEventType {}
impl PNSCopying for HKMedicationDoseEventType {}
impl INSObject for HKMedicationDoseEventType {}
impl PNSObject for HKMedicationDoseEventType {}
impl IHKMedicationDoseEventType for HKMedicationDoseEventType {}
pub trait IHKMedicationDoseEventType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKPrescriptionType(pub id);
impl std::ops::Deref for HKPrescriptionType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKPrescriptionType {}
impl HKPrescriptionType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKPrescriptionType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKPrescriptionType {}
impl std::convert::TryFrom<HKSampleType> for HKPrescriptionType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKPrescriptionType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKPrescriptionType").unwrap()) };
        if is_kind_of {
            Ok(HKPrescriptionType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKPrescriptionType")
        }
    }
}
impl IHKObjectType for HKPrescriptionType {}
impl PNSSecureCoding for HKPrescriptionType {}
impl PNSCopying for HKPrescriptionType {}
impl INSObject for HKPrescriptionType {}
impl PNSObject for HKPrescriptionType {}
impl IHKPrescriptionType for HKPrescriptionType {}
pub trait IHKPrescriptionType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKScoredAssessmentType(pub id);
impl std::ops::Deref for HKScoredAssessmentType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKScoredAssessmentType {}
impl HKScoredAssessmentType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKScoredAssessmentType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKScoredAssessmentType {}
impl std::convert::TryFrom<HKSampleType> for HKScoredAssessmentType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKScoredAssessmentType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKScoredAssessmentType").unwrap()) };
        if is_kind_of {
            Ok(HKScoredAssessmentType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKScoredAssessmentType")
        }
    }
}
impl IHKObjectType for HKScoredAssessmentType {}
impl PNSSecureCoding for HKScoredAssessmentType {}
impl PNSCopying for HKScoredAssessmentType {}
impl INSObject for HKScoredAssessmentType {}
impl PNSObject for HKScoredAssessmentType {}
impl IHKScoredAssessmentType for HKScoredAssessmentType {}
pub trait IHKScoredAssessmentType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKStateOfMindType(pub id);
impl std::ops::Deref for HKStateOfMindType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKStateOfMindType {}
impl HKStateOfMindType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKStateOfMindType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKStateOfMindType {}
impl std::convert::TryFrom<HKSampleType> for HKStateOfMindType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKStateOfMindType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKStateOfMindType").unwrap()) };
        if is_kind_of {
            Ok(HKStateOfMindType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKStateOfMindType")
        }
    }
}
impl IHKObjectType for HKStateOfMindType {}
impl PNSSecureCoding for HKStateOfMindType {}
impl PNSCopying for HKStateOfMindType {}
impl INSObject for HKStateOfMindType {}
impl PNSObject for HKStateOfMindType {}
impl IHKStateOfMindType for HKStateOfMindType {}
pub trait IHKStateOfMindType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKUserAnnotatedMedicationType(pub id);
impl std::ops::Deref for HKUserAnnotatedMedicationType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKUserAnnotatedMedicationType {}
impl HKUserAnnotatedMedicationType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKUserAnnotatedMedicationType").unwrap(), alloc) })
    }
}
impl IHKObjectType for HKUserAnnotatedMedicationType {}
impl PNSSecureCoding for HKUserAnnotatedMedicationType {}
impl PNSCopying for HKUserAnnotatedMedicationType {}
impl std::convert::TryFrom<HKObjectType> for HKUserAnnotatedMedicationType {
    type Error = &'static str;
    fn try_from(parent: HKObjectType) -> Result<HKUserAnnotatedMedicationType, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKUserAnnotatedMedicationType").unwrap())
        };
        if is_kind_of {
            Ok(HKUserAnnotatedMedicationType(parent.0))
        } else {
            Err("This HKObjectType cannot be downcasted to HKUserAnnotatedMedicationType")
        }
    }
}
impl INSObject for HKUserAnnotatedMedicationType {}
impl PNSObject for HKUserAnnotatedMedicationType {}
impl IHKUserAnnotatedMedicationType for HKUserAnnotatedMedicationType {}
pub trait IHKUserAnnotatedMedicationType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKQueryDescriptor(pub id);
impl std::ops::Deref for HKQueryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKQueryDescriptor {}
impl HKQueryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKQueryDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for HKQueryDescriptor {}
impl PNSSecureCoding for HKQueryDescriptor {}
impl INSObject for HKQueryDescriptor {}
impl PNSObject for HKQueryDescriptor {}
impl std::convert::TryFrom<NSObject> for HKQueryDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKQueryDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKQueryDescriptor").unwrap()) };
        if is_kind_of {
            Ok(HKQueryDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKQueryDescriptor")
        }
    }
}
impl IHKQueryDescriptor for HKQueryDescriptor {}
pub trait IHKQueryDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSampleType_predicate_(
        &self,
        sampleType: HKSampleType,
        predicate: NSPredicate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleType : sampleType, predicate : predicate)
    }
    unsafe fn sampleType(&self) -> HKSampleType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleType)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQueryDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKAnchoredObjectQuery(pub id);
impl std::ops::Deref for HKAnchoredObjectQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKAnchoredObjectQuery {}
impl HKAnchoredObjectQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKAnchoredObjectQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKAnchoredObjectQuery {}
impl std::convert::TryFrom<HKQuery> for HKAnchoredObjectQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKAnchoredObjectQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKAnchoredObjectQuery").unwrap()) };
        if is_kind_of {
            Ok(HKAnchoredObjectQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKAnchoredObjectQuery")
        }
    }
}
impl INSObject for HKAnchoredObjectQuery {}
impl PNSObject for HKAnchoredObjectQuery {}
impl IHKAnchoredObjectQuery for HKAnchoredObjectQuery {}
pub trait IHKAnchoredObjectQuery: Sized + std::ops::Deref {
    unsafe fn initWithType_predicate_anchor_limit_resultsHandler_(
        &self,
        type_: HKSampleType,
        predicate: NSPredicate,
        anchor: HKQueryAnchor,
        limit: NSUInteger,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, predicate : predicate, anchor : anchor, limit : limit, resultsHandler : handler)
    }
    unsafe fn initWithType_predicate_anchor_limit_completionHandler_(
        &self,
        type_: HKSampleType,
        predicate: NSPredicate,
        anchor: NSUInteger,
        limit: NSUInteger,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, predicate : predicate, anchor : anchor, limit : limit, completionHandler : handler)
    }
    unsafe fn initWithQueryDescriptors_anchor_limit_resultsHandler_(
        &self,
        queryDescriptors: NSArray,
        anchor: HKQueryAnchor,
        limit: NSInteger,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQueryDescriptors : queryDescriptors, anchor : anchor, limit : limit, resultsHandler : handler)
    }
    unsafe fn updateHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateHandler)
    }
    unsafe fn setUpdateHandler_(&self, updateHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpdateHandler : updateHandler)
    }
}
pub type HKAppleSleepingBreathingDisturbancesClassification = NSInteger;
pub type HKAppleWalkingSteadinessClassification = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKAttachment(pub id);
impl std::ops::Deref for HKAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKAttachment {}
impl HKAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKAttachment").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKAttachment {}
impl PNSCopying for HKAttachment {}
impl INSObject for HKAttachment {}
impl PNSObject for HKAttachment {}
impl std::convert::TryFrom<NSObject> for HKAttachment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKAttachment").unwrap()) };
        if is_kind_of {
            Ok(HKAttachment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKAttachment")
        }
    }
}
impl IHKAttachment for HKAttachment {}
pub trait IHKAttachment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn size(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKAttachment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKAttachmentStore(pub id);
impl std::ops::Deref for HKAttachmentStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKAttachmentStore {}
impl HKAttachmentStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKAttachmentStore").unwrap(), alloc) })
    }
}
impl INSObject for HKAttachmentStore {}
impl PNSObject for HKAttachmentStore {}
impl std::convert::TryFrom<NSObject> for HKAttachmentStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKAttachmentStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKAttachmentStore").unwrap()) };
        if is_kind_of {
            Ok(HKAttachmentStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKAttachmentStore")
        }
    }
}
impl IHKAttachmentStore for HKAttachmentStore {}
pub trait IHKAttachmentStore: Sized + std::ops::Deref {
    unsafe fn initWithHealthStore_(&self, healthStore: HKHealthStore) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHealthStore : healthStore)
    }
    unsafe fn addAttachmentToObject_name_contentType_URL_metadata_completion_(
        &self,
        object: HKObject,
        name: NSString,
        contentType: UTType,
        URL: NSURL,
        metadata: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAttachmentToObject : object, name : name, contentType : contentType, URL : URL, metadata : metadata, completion : completion)
    }
    unsafe fn removeAttachment_fromObject_completion_(
        &self,
        attachment: HKAttachment,
        object: HKObject,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAttachment : attachment, fromObject : object, completion : completion)
    }
    unsafe fn getAttachmentsForObject_completion_(
        &self,
        object: HKObject,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttachmentsForObject : object, completion : completion)
    }
    unsafe fn getDataForAttachment_completion_(
        &self,
        attachment: HKAttachment,
        completion: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDataForAttachment : attachment, completion : completion)
    }
    unsafe fn streamDataForAttachment_dataHandler_(
        &self,
        attachment: HKAttachment,
        dataHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, streamDataForAttachment : attachment, dataHandler : dataHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKQuantity(pub id);
impl std::ops::Deref for HKQuantity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKQuantity {}
impl HKQuantity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantity").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKQuantity {}
impl PNSCopying for HKQuantity {}
impl INSObject for HKQuantity {}
impl PNSObject for HKQuantity {}
impl std::convert::TryFrom<NSObject> for HKQuantity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKQuantity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKQuantity").unwrap()) };
        if is_kind_of {
            Ok(HKQuantity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKQuantity")
        }
    }
}
impl IHKQuantity for HKQuantity {}
pub trait IHKQuantity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isCompatibleWithUnit_(&self, unit: HKUnit) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isCompatibleWithUnit : unit)
    }
    unsafe fn doubleValueForUnit_(&self, unit: HKUnit) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, doubleValueForUnit : unit)
    }
    unsafe fn compare_(&self, quantity: HKQuantity) -> NSComparisonResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compare : quantity)
    }
    unsafe fn quantityWithUnit_doubleValue_(unit: HKUnit, value: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantity").unwrap(), quantityWithUnit : unit, doubleValue : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKAudiogramSample(pub id);
impl std::ops::Deref for HKAudiogramSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKAudiogramSample {}
impl HKAudiogramSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSample").unwrap(), alloc) })
    }
}
impl IHKSample for HKAudiogramSample {}
impl std::convert::TryFrom<HKSample> for HKAudiogramSample {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKAudiogramSample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKAudiogramSample").unwrap()) };
        if is_kind_of {
            Ok(HKAudiogramSample(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKAudiogramSample")
        }
    }
}
impl IHKObject for HKAudiogramSample {}
impl PNSSecureCoding for HKAudiogramSample {}
impl INSObject for HKAudiogramSample {}
impl PNSObject for HKAudiogramSample {}
impl IHKAudiogramSample for HKAudiogramSample {}
pub trait IHKAudiogramSample: Sized + std::ops::Deref {
    unsafe fn sensitivityPoints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensitivityPoints)
    }
    unsafe fn audiogramSampleWithSensitivityPoints_startDate_endDate_metadata_(
        sensitivityPoints: NSArray,
        startDate: NSDate,
        endDate: NSDate,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSample").unwrap(), audiogramSampleWithSensitivityPoints : sensitivityPoints, startDate : startDate, endDate : endDate, metadata : metadata)
    }
    unsafe fn audiogramSampleWithSensitivityPoints_startDate_endDate_device_metadata_(
        sensitivityPoints: NSArray,
        startDate: NSDate,
        endDate: NSDate,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSample").unwrap(), audiogramSampleWithSensitivityPoints : sensitivityPoints, startDate : startDate, endDate : endDate, device : device, metadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKAudiogramSensitivityPoint(pub id);
impl std::ops::Deref for HKAudiogramSensitivityPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKAudiogramSensitivityPoint {}
impl HKAudiogramSensitivityPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityPoint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKAudiogramSensitivityPoint {}
impl INSObject for HKAudiogramSensitivityPoint {}
impl PNSObject for HKAudiogramSensitivityPoint {}
impl std::convert::TryFrom<NSObject> for HKAudiogramSensitivityPoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKAudiogramSensitivityPoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityPoint").unwrap()) };
        if is_kind_of {
            Ok(HKAudiogramSensitivityPoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKAudiogramSensitivityPoint")
        }
    }
}
impl IHKAudiogramSensitivityPoint for HKAudiogramSensitivityPoint {}
pub trait IHKAudiogramSensitivityPoint: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn frequency(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn leftEarSensitivity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEarSensitivity)
    }
    unsafe fn rightEarSensitivity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEarSensitivity)
    }
    unsafe fn tests(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tests)
    }
    unsafe fn sensitivityPointWithFrequency_leftEarSensitivity_rightEarSensitivity_error_(
        frequency: HKQuantity,
        leftEarSensitivity: HKQuantity,
        rightEarSensitivity: HKQuantity,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityPoint").unwrap(), sensitivityPointWithFrequency : frequency, leftEarSensitivity : leftEarSensitivity, rightEarSensitivity : rightEarSensitivity, error : error)
    }
    unsafe fn sensitivityPointWithFrequency_tests_error_(
        frequency: HKQuantity,
        tests: NSArray,
        errorOut: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityPoint").unwrap(), sensitivityPointWithFrequency : frequency, tests : tests, error : errorOut)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKAudiogramSensitivityPointClampingRange(pub id);
impl std::ops::Deref for HKAudiogramSensitivityPointClampingRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKAudiogramSensitivityPointClampingRange {}
impl HKAudiogramSensitivityPointClampingRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityPointClampingRange").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKAudiogramSensitivityPointClampingRange {}
impl PNSCopying for HKAudiogramSensitivityPointClampingRange {}
impl INSObject for HKAudiogramSensitivityPointClampingRange {}
impl PNSObject for HKAudiogramSensitivityPointClampingRange {}
impl std::convert::TryFrom<NSObject> for HKAudiogramSensitivityPointClampingRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKAudiogramSensitivityPointClampingRange, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityPointClampingRange").unwrap())
        };
        if is_kind_of {
            Ok(HKAudiogramSensitivityPointClampingRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKAudiogramSensitivityPointClampingRange")
        }
    }
}
impl IHKAudiogramSensitivityPointClampingRange for HKAudiogramSensitivityPointClampingRange {}
pub trait IHKAudiogramSensitivityPointClampingRange: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn lowerBound(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowerBound)
    }
    unsafe fn upperBound(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperBound)
    }
    unsafe fn clampingRangeWithLowerBound_upperBound_error_(
        lowerBound: NSNumber,
        upperBound: NSNumber,
        errorOut: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityPointClampingRange").unwrap(), clampingRangeWithLowerBound : lowerBound, upperBound : upperBound, error : errorOut)
    }
}
pub type HKAudiogramConductionType = NSInteger;
pub type HKAudiogramSensitivityTestSide = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKAudiogramSensitivityTest(pub id);
impl std::ops::Deref for HKAudiogramSensitivityTest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKAudiogramSensitivityTest {}
impl HKAudiogramSensitivityTest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityTest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKAudiogramSensitivityTest {}
impl PNSCopying for HKAudiogramSensitivityTest {}
impl INSObject for HKAudiogramSensitivityTest {}
impl PNSObject for HKAudiogramSensitivityTest {}
impl std::convert::TryFrom<NSObject> for HKAudiogramSensitivityTest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKAudiogramSensitivityTest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityTest").unwrap()) };
        if is_kind_of {
            Ok(HKAudiogramSensitivityTest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKAudiogramSensitivityTest")
        }
    }
}
impl IHKAudiogramSensitivityTest for HKAudiogramSensitivityTest {}
pub trait IHKAudiogramSensitivityTest: Sized + std::ops::Deref {
    unsafe fn initWithSensitivity_type_masked_side_clampingRange_error_(
        &self,
        sensitivity: HKQuantity,
        type_: HKAudiogramConductionType,
        masked: BOOL,
        side: HKAudiogramSensitivityTestSide,
        clampingRange: HKAudiogramSensitivityPointClampingRange,
        errorOut: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSensitivity : sensitivity, r#type : type_, masked : masked, side : side, clampingRange : clampingRange, error : errorOut)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sensitivity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensitivity)
    }
    unsafe fn type_(&self) -> HKAudiogramConductionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn masked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, masked)
    }
    unsafe fn side(&self) -> HKAudiogramSensitivityTestSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, side)
    }
    unsafe fn clampingRange(&self) -> HKAudiogramSensitivityPointClampingRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clampingRange)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKAudiogramSensitivityTest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCategorySample(pub id);
impl std::ops::Deref for HKCategorySample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCategorySample {}
impl HKCategorySample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCategorySample").unwrap(), alloc) })
    }
}
impl IHKSample for HKCategorySample {}
impl std::convert::TryFrom<HKSample> for HKCategorySample {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKCategorySample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCategorySample").unwrap()) };
        if is_kind_of {
            Ok(HKCategorySample(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKCategorySample")
        }
    }
}
impl IHKObject for HKCategorySample {}
impl PNSSecureCoding for HKCategorySample {}
impl INSObject for HKCategorySample {}
impl PNSObject for HKCategorySample {}
impl IHKCategorySample for HKCategorySample {}
pub trait IHKCategorySample: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn categoryType(&self) -> HKCategoryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryType)
    }
    unsafe fn value(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn categorySampleWithType_value_startDate_endDate_metadata_(
        type_: HKCategoryType,
        value: NSInteger,
        startDate: NSDate,
        endDate: NSDate,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKCategorySample").unwrap(), categorySampleWithType : type_, value : value, startDate : startDate, endDate : endDate, metadata : metadata)
    }
    unsafe fn categorySampleWithType_value_startDate_endDate_(
        type_: HKCategoryType,
        value: NSInteger,
        startDate: NSDate,
        endDate: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKCategorySample").unwrap(), categorySampleWithType : type_, value : value, startDate : startDate, endDate : endDate)
    }
    unsafe fn categorySampleWithType_value_startDate_endDate_device_metadata_(
        type_: HKCategoryType,
        value: NSInteger,
        startDate: NSDate,
        endDate: NSDate,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKCategorySample").unwrap(), categorySampleWithType : type_, value : value, startDate : startDate, endDate : endDate, device : device, metadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKDocumentSample(pub id);
impl std::ops::Deref for HKDocumentSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKDocumentSample {}
impl HKDocumentSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKDocumentSample").unwrap(), alloc) })
    }
}
impl IHKSample for HKDocumentSample {}
impl std::convert::TryFrom<HKSample> for HKDocumentSample {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKDocumentSample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKDocumentSample").unwrap()) };
        if is_kind_of {
            Ok(HKDocumentSample(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKDocumentSample")
        }
    }
}
impl IHKObject for HKDocumentSample {}
impl PNSSecureCoding for HKDocumentSample {}
impl INSObject for HKDocumentSample {}
impl PNSObject for HKDocumentSample {}
impl IHKDocumentSample for HKDocumentSample {}
pub trait IHKDocumentSample: Sized + std::ops::Deref {
    unsafe fn documentType(&self) -> HKDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCDADocumentSample(pub id);
impl std::ops::Deref for HKCDADocumentSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCDADocumentSample {}
impl HKCDADocumentSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCDADocumentSample").unwrap(), alloc) })
    }
}
impl IHKDocumentSample for HKCDADocumentSample {}
impl From<HKCDADocumentSample> for HKDocumentSample {
    fn from(child: HKCDADocumentSample) -> HKDocumentSample {
        HKDocumentSample(child.0)
    }
}
impl std::convert::TryFrom<HKDocumentSample> for HKCDADocumentSample {
    type Error = &'static str;
    fn try_from(parent: HKDocumentSample) -> Result<HKCDADocumentSample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCDADocumentSample").unwrap()) };
        if is_kind_of {
            Ok(HKCDADocumentSample(parent.0))
        } else {
            Err("This HKDocumentSample cannot be downcasted to HKCDADocumentSample")
        }
    }
}
impl IHKSample for HKCDADocumentSample {}
impl IHKObject for HKCDADocumentSample {}
impl PNSSecureCoding for HKCDADocumentSample {}
impl INSObject for HKCDADocumentSample {}
impl PNSObject for HKCDADocumentSample {}
impl IHKCDADocumentSample for HKCDADocumentSample {}
pub trait IHKCDADocumentSample: Sized + std::ops::Deref {
    unsafe fn document(&self) -> HKCDADocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, document)
    }
    unsafe fn CDADocumentSampleWithData_startDate_endDate_metadata_validationError_(
        documentData: NSData,
        startDate: NSDate,
        endDate: NSDate,
        metadata: NSDictionary,
        validationError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKCDADocumentSample").unwrap(), CDADocumentSampleWithData : documentData, startDate : startDate, endDate : endDate, metadata : metadata, validationError : validationError)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCDADocument(pub id);
impl std::ops::Deref for HKCDADocument {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCDADocument {}
impl HKCDADocument {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCDADocument").unwrap(), alloc) })
    }
}
impl INSObject for HKCDADocument {}
impl PNSObject for HKCDADocument {}
impl std::convert::TryFrom<NSObject> for HKCDADocument {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKCDADocument, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCDADocument").unwrap()) };
        if is_kind_of {
            Ok(HKCDADocument(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKCDADocument")
        }
    }
}
impl IHKCDADocument for HKCDADocument {}
pub trait IHKCDADocument: Sized + std::ops::Deref {
    unsafe fn documentData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentData)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn patientName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, patientName)
    }
    unsafe fn authorName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorName)
    }
    unsafe fn custodianName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, custodianName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKBiologicalSexObject(pub id);
impl std::ops::Deref for HKBiologicalSexObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKBiologicalSexObject {}
impl HKBiologicalSexObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKBiologicalSexObject").unwrap(), alloc) })
    }
}
impl PNSCopying for HKBiologicalSexObject {}
impl PNSSecureCoding for HKBiologicalSexObject {}
impl INSObject for HKBiologicalSexObject {}
impl PNSObject for HKBiologicalSexObject {}
impl std::convert::TryFrom<NSObject> for HKBiologicalSexObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKBiologicalSexObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKBiologicalSexObject").unwrap()) };
        if is_kind_of {
            Ok(HKBiologicalSexObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKBiologicalSexObject")
        }
    }
}
impl IHKBiologicalSexObject for HKBiologicalSexObject {}
pub trait IHKBiologicalSexObject: Sized + std::ops::Deref {
    unsafe fn biologicalSex(&self) -> HKBiologicalSex
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biologicalSex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKBloodTypeObject(pub id);
impl std::ops::Deref for HKBloodTypeObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKBloodTypeObject {}
impl HKBloodTypeObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKBloodTypeObject").unwrap(), alloc) })
    }
}
impl PNSCopying for HKBloodTypeObject {}
impl PNSSecureCoding for HKBloodTypeObject {}
impl INSObject for HKBloodTypeObject {}
impl PNSObject for HKBloodTypeObject {}
impl std::convert::TryFrom<NSObject> for HKBloodTypeObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKBloodTypeObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKBloodTypeObject").unwrap()) };
        if is_kind_of {
            Ok(HKBloodTypeObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKBloodTypeObject")
        }
    }
}
impl IHKBloodTypeObject for HKBloodTypeObject {}
pub trait IHKBloodTypeObject: Sized + std::ops::Deref {
    unsafe fn bloodType(&self) -> HKBloodType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bloodType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKFitzpatrickSkinTypeObject(pub id);
impl std::ops::Deref for HKFitzpatrickSkinTypeObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKFitzpatrickSkinTypeObject {}
impl HKFitzpatrickSkinTypeObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKFitzpatrickSkinTypeObject").unwrap(), alloc) })
    }
}
impl PNSCopying for HKFitzpatrickSkinTypeObject {}
impl PNSSecureCoding for HKFitzpatrickSkinTypeObject {}
impl INSObject for HKFitzpatrickSkinTypeObject {}
impl PNSObject for HKFitzpatrickSkinTypeObject {}
impl std::convert::TryFrom<NSObject> for HKFitzpatrickSkinTypeObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKFitzpatrickSkinTypeObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKFitzpatrickSkinTypeObject").unwrap()) };
        if is_kind_of {
            Ok(HKFitzpatrickSkinTypeObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKFitzpatrickSkinTypeObject")
        }
    }
}
impl IHKFitzpatrickSkinTypeObject for HKFitzpatrickSkinTypeObject {}
pub trait IHKFitzpatrickSkinTypeObject: Sized + std::ops::Deref {
    unsafe fn skinType(&self) -> HKFitzpatrickSkinType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skinType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWheelchairUseObject(pub id);
impl std::ops::Deref for HKWheelchairUseObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWheelchairUseObject {}
impl HKWheelchairUseObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWheelchairUseObject").unwrap(), alloc) })
    }
}
impl PNSCopying for HKWheelchairUseObject {}
impl PNSSecureCoding for HKWheelchairUseObject {}
impl INSObject for HKWheelchairUseObject {}
impl PNSObject for HKWheelchairUseObject {}
impl std::convert::TryFrom<NSObject> for HKWheelchairUseObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKWheelchairUseObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWheelchairUseObject").unwrap()) };
        if is_kind_of {
            Ok(HKWheelchairUseObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKWheelchairUseObject")
        }
    }
}
impl IHKWheelchairUseObject for HKWheelchairUseObject {}
pub trait IHKWheelchairUseObject: Sized + std::ops::Deref {
    unsafe fn wheelchairUse(&self) -> HKWheelchairUse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wheelchairUse)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKActivityMoveModeObject(pub id);
impl std::ops::Deref for HKActivityMoveModeObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKActivityMoveModeObject {}
impl HKActivityMoveModeObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKActivityMoveModeObject").unwrap(), alloc) })
    }
}
impl PNSCopying for HKActivityMoveModeObject {}
impl PNSSecureCoding for HKActivityMoveModeObject {}
impl INSObject for HKActivityMoveModeObject {}
impl PNSObject for HKActivityMoveModeObject {}
impl std::convert::TryFrom<NSObject> for HKActivityMoveModeObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKActivityMoveModeObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKActivityMoveModeObject").unwrap()) };
        if is_kind_of {
            Ok(HKActivityMoveModeObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKActivityMoveModeObject")
        }
    }
}
impl IHKActivityMoveModeObject for HKActivityMoveModeObject {}
pub trait IHKActivityMoveModeObject: Sized + std::ops::Deref {
    unsafe fn activityMoveMode(&self) -> HKActivityMoveMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityMoveMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKClinicalCoding(pub id);
impl std::ops::Deref for HKClinicalCoding {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKClinicalCoding {}
impl HKClinicalCoding {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKClinicalCoding").unwrap(), alloc) })
    }
}
impl PNSCopying for HKClinicalCoding {}
impl PNSSecureCoding for HKClinicalCoding {}
impl INSObject for HKClinicalCoding {}
impl PNSObject for HKClinicalCoding {}
impl std::convert::TryFrom<NSObject> for HKClinicalCoding {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKClinicalCoding, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKClinicalCoding").unwrap()) };
        if is_kind_of {
            Ok(HKClinicalCoding(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKClinicalCoding")
        }
    }
}
impl IHKClinicalCoding for HKClinicalCoding {}
pub trait IHKClinicalCoding: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSystem_version_code_(
        &self,
        system: NSString,
        version: NSString,
        code: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSystem : system, version : version, code : code)
    }
    unsafe fn system(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, system)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn code(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, code)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKClinicalRecord(pub id);
impl std::ops::Deref for HKClinicalRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKClinicalRecord {}
impl HKClinicalRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKClinicalRecord").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKClinicalRecord {}
impl PNSCopying for HKClinicalRecord {}
impl IHKSample for HKClinicalRecord {}
impl std::convert::TryFrom<HKSample> for HKClinicalRecord {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKClinicalRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKClinicalRecord").unwrap()) };
        if is_kind_of {
            Ok(HKClinicalRecord(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKClinicalRecord")
        }
    }
}
impl IHKObject for HKClinicalRecord {}
impl INSObject for HKClinicalRecord {}
impl PNSObject for HKClinicalRecord {}
impl IHKClinicalRecord for HKClinicalRecord {}
pub trait IHKClinicalRecord: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn clinicalType(&self) -> HKClinicalType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clinicalType)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn FHIRResource(&self) -> HKFHIRResource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, FHIRResource)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKClinicalRecord").unwrap(), new)
    }
}
pub type HKClinicalTypeIdentifier = NSString;
impl HKObjectType_ClinicalType for HKObjectType {}
pub trait HKObjectType_ClinicalType: Sized + std::ops::Deref {
    unsafe fn clinicalTypeForIdentifier_(identifier: NSString) -> HKClinicalType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKObjectType").unwrap(), clinicalTypeForIdentifier : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKClinicalType(pub id);
impl std::ops::Deref for HKClinicalType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKClinicalType {}
impl HKClinicalType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKClinicalType").unwrap(), alloc) })
    }
}
impl IHKSampleType for HKClinicalType {}
impl std::convert::TryFrom<HKSampleType> for HKClinicalType {
    type Error = &'static str;
    fn try_from(parent: HKSampleType) -> Result<HKClinicalType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKClinicalType").unwrap()) };
        if is_kind_of {
            Ok(HKClinicalType(parent.0))
        } else {
            Err("This HKSampleType cannot be downcasted to HKClinicalType")
        }
    }
}
impl IHKObjectType for HKClinicalType {}
impl PNSSecureCoding for HKClinicalType {}
impl PNSCopying for HKClinicalType {}
impl INSObject for HKClinicalType {}
impl PNSObject for HKClinicalType {}
impl IHKClinicalType for HKClinicalType {}
pub trait IHKClinicalType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKLensSpecification(pub id);
impl std::ops::Deref for HKLensSpecification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKLensSpecification {}
impl HKLensSpecification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKLensSpecification").unwrap(), alloc) })
    }
}
impl INSObject for HKLensSpecification {}
impl PNSObject for HKLensSpecification {}
impl std::convert::TryFrom<NSObject> for HKLensSpecification {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKLensSpecification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKLensSpecification").unwrap()) };
        if is_kind_of {
            Ok(HKLensSpecification(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKLensSpecification")
        }
    }
}
impl IHKLensSpecification for HKLensSpecification {}
pub trait IHKLensSpecification: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sphere(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sphere)
    }
    unsafe fn cylinder(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cylinder)
    }
    unsafe fn axis(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axis)
    }
    unsafe fn addPower(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addPower)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKLensSpecification").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKContactsLensSpecification(pub id);
impl std::ops::Deref for HKContactsLensSpecification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKContactsLensSpecification {}
impl HKContactsLensSpecification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKContactsLensSpecification").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKContactsLensSpecification {}
impl PNSCopying for HKContactsLensSpecification {}
impl IHKLensSpecification for HKContactsLensSpecification {}
impl From<HKContactsLensSpecification> for HKLensSpecification {
    fn from(child: HKContactsLensSpecification) -> HKLensSpecification {
        HKLensSpecification(child.0)
    }
}
impl std::convert::TryFrom<HKLensSpecification> for HKContactsLensSpecification {
    type Error = &'static str;
    fn try_from(parent: HKLensSpecification) -> Result<HKContactsLensSpecification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKContactsLensSpecification").unwrap()) };
        if is_kind_of {
            Ok(HKContactsLensSpecification(parent.0))
        } else {
            Err("This HKLensSpecification cannot be downcasted to HKContactsLensSpecification")
        }
    }
}
impl INSObject for HKContactsLensSpecification {}
impl PNSObject for HKContactsLensSpecification {}
impl IHKContactsLensSpecification for HKContactsLensSpecification {}
pub trait IHKContactsLensSpecification: Sized + std::ops::Deref {
    unsafe fn initWithSphere_cylinder_axis_addPower_baseCurve_diameter_(
        &self,
        sphere: HKQuantity,
        cylinder: HKQuantity,
        axis: HKQuantity,
        addPower: HKQuantity,
        baseCurve: HKQuantity,
        diameter: HKQuantity,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSphere : sphere, cylinder : cylinder, axis : axis, addPower : addPower, baseCurve : baseCurve, diameter : diameter)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn baseCurve(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseCurve)
    }
    unsafe fn diameter(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diameter)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKContactsLensSpecification").unwrap(), new)
    }
}
pub type HKVisionPrescriptionType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKVisionPrescription(pub id);
impl std::ops::Deref for HKVisionPrescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKVisionPrescription {}
impl HKVisionPrescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKVisionPrescription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKVisionPrescription {}
impl PNSCopying for HKVisionPrescription {}
impl IHKSample for HKVisionPrescription {}
impl std::convert::TryFrom<HKSample> for HKVisionPrescription {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKVisionPrescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKVisionPrescription").unwrap()) };
        if is_kind_of {
            Ok(HKVisionPrescription(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKVisionPrescription")
        }
    }
}
impl IHKObject for HKVisionPrescription {}
impl INSObject for HKVisionPrescription {}
impl PNSObject for HKVisionPrescription {}
impl IHKVisionPrescription for HKVisionPrescription {}
pub trait IHKVisionPrescription: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn prescriptionType(&self) -> HKVisionPrescriptionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prescriptionType)
    }
    unsafe fn dateIssued(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateIssued)
    }
    unsafe fn expirationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expirationDate)
    }
    unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata_(
        type_: HKVisionPrescriptionType,
        dateIssued: NSDate,
        expirationDate: NSDate,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKVisionPrescription").unwrap(), prescriptionWithType : type_, dateIssued : dateIssued, expirationDate : expirationDate, device : device, metadata : metadata)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKVisionPrescription").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKContactsPrescription(pub id);
impl std::ops::Deref for HKContactsPrescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKContactsPrescription {}
impl HKContactsPrescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKContactsPrescription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKContactsPrescription {}
impl PNSCopying for HKContactsPrescription {}
impl IHKVisionPrescription for HKContactsPrescription {}
impl From<HKContactsPrescription> for HKVisionPrescription {
    fn from(child: HKContactsPrescription) -> HKVisionPrescription {
        HKVisionPrescription(child.0)
    }
}
impl std::convert::TryFrom<HKVisionPrescription> for HKContactsPrescription {
    type Error = &'static str;
    fn try_from(parent: HKVisionPrescription) -> Result<HKContactsPrescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKContactsPrescription").unwrap()) };
        if is_kind_of {
            Ok(HKContactsPrescription(parent.0))
        } else {
            Err("This HKVisionPrescription cannot be downcasted to HKContactsPrescription")
        }
    }
}
impl IHKSample for HKContactsPrescription {}
impl IHKObject for HKContactsPrescription {}
impl INSObject for HKContactsPrescription {}
impl PNSObject for HKContactsPrescription {}
impl IHKContactsPrescription for HKContactsPrescription {}
pub trait IHKContactsPrescription: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn rightEye(&self) -> HKContactsLensSpecification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEye)
    }
    unsafe fn leftEye(&self) -> HKContactsLensSpecification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEye)
    }
    unsafe fn brand(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, brand)
    }
    unsafe fn prescriptionWithRightEyeSpecification_leftEyeSpecification_brand_dateIssued_expirationDate_device_metadata_(
        rightEyeSpecification: HKContactsLensSpecification,
        leftEyeSpecification: HKContactsLensSpecification,
        brand: NSString,
        dateIssued: NSDate,
        expirationDate: NSDate,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKContactsPrescription").unwrap(), prescriptionWithRightEyeSpecification : rightEyeSpecification, leftEyeSpecification : leftEyeSpecification, brand : brand, dateIssued : dateIssued, expirationDate : expirationDate, device : device, metadata : metadata)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKContactsPrescription").unwrap(), new)
    }
    unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata_(
        type_: HKVisionPrescriptionType,
        dateIssued: NSDate,
        expirationDate: NSDate,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKContactsPrescription").unwrap(), prescriptionWithType : type_, dateIssued : dateIssued, expirationDate : expirationDate, device : device, metadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCorrelation(pub id);
impl std::ops::Deref for HKCorrelation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCorrelation {}
impl HKCorrelation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCorrelation").unwrap(), alloc) })
    }
}
impl IHKSample for HKCorrelation {}
impl std::convert::TryFrom<HKSample> for HKCorrelation {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKCorrelation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCorrelation").unwrap()) };
        if is_kind_of {
            Ok(HKCorrelation(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKCorrelation")
        }
    }
}
impl IHKObject for HKCorrelation {}
impl PNSSecureCoding for HKCorrelation {}
impl INSObject for HKCorrelation {}
impl PNSObject for HKCorrelation {}
impl IHKCorrelation for HKCorrelation {}
pub trait IHKCorrelation: Sized + std::ops::Deref {
    unsafe fn objectsForType_(&self, objectType: HKObjectType) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectsForType : objectType)
    }
    unsafe fn correlationType(&self) -> HKCorrelationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correlationType)
    }
    unsafe fn objects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objects)
    }
    unsafe fn correlationWithType_startDate_endDate_objects_(
        correlationType: HKCorrelationType,
        startDate: NSDate,
        endDate: NSDate,
        objects: NSSet,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKCorrelation").unwrap(), correlationWithType : correlationType, startDate : startDate, endDate : endDate, objects : objects)
    }
    unsafe fn correlationWithType_startDate_endDate_objects_metadata_(
        correlationType: HKCorrelationType,
        startDate: NSDate,
        endDate: NSDate,
        objects: NSSet,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKCorrelation").unwrap(), correlationWithType : correlationType, startDate : startDate, endDate : endDate, objects : objects, metadata : metadata)
    }
    unsafe fn correlationWithType_startDate_endDate_objects_device_metadata_(
        correlationType: HKCorrelationType,
        startDate: NSDate,
        endDate: NSDate,
        objects: NSSet,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKCorrelation").unwrap(), correlationWithType : correlationType, startDate : startDate, endDate : endDate, objects : objects, device : device, metadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCorrelationQuery(pub id);
impl std::ops::Deref for HKCorrelationQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCorrelationQuery {}
impl HKCorrelationQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCorrelationQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKCorrelationQuery {}
impl std::convert::TryFrom<HKQuery> for HKCorrelationQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKCorrelationQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCorrelationQuery").unwrap()) };
        if is_kind_of {
            Ok(HKCorrelationQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKCorrelationQuery")
        }
    }
}
impl INSObject for HKCorrelationQuery {}
impl PNSObject for HKCorrelationQuery {}
impl IHKCorrelationQuery for HKCorrelationQuery {}
pub trait IHKCorrelationQuery: Sized + std::ops::Deref {
    unsafe fn initWithType_predicate_samplePredicates_completion_(
        &self,
        correlationType: HKCorrelationType,
        predicate: NSPredicate,
        samplePredicates: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : correlationType, predicate : predicate, samplePredicates : samplePredicates, completion : completion)
    }
    unsafe fn correlationType(&self) -> HKCorrelationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correlationType)
    }
    unsafe fn samplePredicates(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, samplePredicates)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKQuantitySample(pub id);
impl std::ops::Deref for HKQuantitySample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKQuantitySample {}
impl HKQuantitySample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantitySample").unwrap(), alloc) })
    }
}
impl IHKSample for HKQuantitySample {}
impl std::convert::TryFrom<HKSample> for HKQuantitySample {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKQuantitySample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKQuantitySample").unwrap()) };
        if is_kind_of {
            Ok(HKQuantitySample(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKQuantitySample")
        }
    }
}
impl IHKObject for HKQuantitySample {}
impl PNSSecureCoding for HKQuantitySample {}
impl INSObject for HKQuantitySample {}
impl PNSObject for HKQuantitySample {}
impl IHKQuantitySample for HKQuantitySample {}
pub trait IHKQuantitySample: Sized + std::ops::Deref {
    unsafe fn quantityType(&self) -> HKQuantityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quantityType)
    }
    unsafe fn quantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quantity)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn quantitySampleWithType_quantity_startDate_endDate_(
        quantityType: HKQuantityType,
        quantity: HKQuantity,
        startDate: NSDate,
        endDate: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantitySample").unwrap(), quantitySampleWithType : quantityType, quantity : quantity, startDate : startDate, endDate : endDate)
    }
    unsafe fn quantitySampleWithType_quantity_startDate_endDate_metadata_(
        quantityType: HKQuantityType,
        quantity: HKQuantity,
        startDate: NSDate,
        endDate: NSDate,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantitySample").unwrap(), quantitySampleWithType : quantityType, quantity : quantity, startDate : startDate, endDate : endDate, metadata : metadata)
    }
    unsafe fn quantitySampleWithType_quantity_startDate_endDate_device_metadata_(
        quantityType: HKQuantityType,
        quantity: HKQuantity,
        startDate: NSDate,
        endDate: NSDate,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantitySample").unwrap(), quantitySampleWithType : quantityType, quantity : quantity, startDate : startDate, endDate : endDate, device : device, metadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCumulativeQuantitySample(pub id);
impl std::ops::Deref for HKCumulativeQuantitySample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCumulativeQuantitySample {}
impl HKCumulativeQuantitySample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCumulativeQuantitySample").unwrap(), alloc) })
    }
}
impl IHKQuantitySample for HKCumulativeQuantitySample {}
impl From<HKCumulativeQuantitySample> for HKQuantitySample {
    fn from(child: HKCumulativeQuantitySample) -> HKQuantitySample {
        HKQuantitySample(child.0)
    }
}
impl std::convert::TryFrom<HKQuantitySample> for HKCumulativeQuantitySample {
    type Error = &'static str;
    fn try_from(parent: HKQuantitySample) -> Result<HKCumulativeQuantitySample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCumulativeQuantitySample").unwrap()) };
        if is_kind_of {
            Ok(HKCumulativeQuantitySample(parent.0))
        } else {
            Err("This HKQuantitySample cannot be downcasted to HKCumulativeQuantitySample")
        }
    }
}
impl IHKSample for HKCumulativeQuantitySample {}
impl IHKObject for HKCumulativeQuantitySample {}
impl PNSSecureCoding for HKCumulativeQuantitySample {}
impl INSObject for HKCumulativeQuantitySample {}
impl PNSObject for HKCumulativeQuantitySample {}
impl IHKCumulativeQuantitySample for HKCumulativeQuantitySample {}
pub trait IHKCumulativeQuantitySample: Sized + std::ops::Deref {
    unsafe fn sumQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sumQuantity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKCumulativeQuantitySeriesSample(pub id);
impl std::ops::Deref for HKCumulativeQuantitySeriesSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKCumulativeQuantitySeriesSample {}
impl HKCumulativeQuantitySeriesSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKCumulativeQuantitySeriesSample").unwrap(), alloc) })
    }
}
impl IHKCumulativeQuantitySample for HKCumulativeQuantitySeriesSample {}
impl From<HKCumulativeQuantitySeriesSample> for HKCumulativeQuantitySample {
    fn from(child: HKCumulativeQuantitySeriesSample) -> HKCumulativeQuantitySample {
        HKCumulativeQuantitySample(child.0)
    }
}
impl std::convert::TryFrom<HKCumulativeQuantitySample> for HKCumulativeQuantitySeriesSample {
    type Error = &'static str;
    fn try_from(
        parent: HKCumulativeQuantitySample,
    ) -> Result<HKCumulativeQuantitySeriesSample, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKCumulativeQuantitySeriesSample").unwrap())
        };
        if is_kind_of {
            Ok(HKCumulativeQuantitySeriesSample(parent.0))
        } else {
            Err ("This HKCumulativeQuantitySample cannot be downcasted to HKCumulativeQuantitySeriesSample" ,)
        }
    }
}
impl IHKQuantitySample for HKCumulativeQuantitySeriesSample {}
impl IHKSample for HKCumulativeQuantitySeriesSample {}
impl IHKObject for HKCumulativeQuantitySeriesSample {}
impl PNSSecureCoding for HKCumulativeQuantitySeriesSample {}
impl INSObject for HKCumulativeQuantitySeriesSample {}
impl PNSObject for HKCumulativeQuantitySeriesSample {}
impl IHKCumulativeQuantitySeriesSample for HKCumulativeQuantitySeriesSample {}
pub trait IHKCumulativeQuantitySeriesSample: Sized + std::ops::Deref {
    unsafe fn sum(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sum)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKDeletedObject(pub id);
impl std::ops::Deref for HKDeletedObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKDeletedObject {}
impl HKDeletedObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKDeletedObject").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKDeletedObject {}
impl INSObject for HKDeletedObject {}
impl PNSObject for HKDeletedObject {}
impl std::convert::TryFrom<NSObject> for HKDeletedObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKDeletedObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKDeletedObject").unwrap()) };
        if is_kind_of {
            Ok(HKDeletedObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKDeletedObject")
        }
    }
}
impl IHKDeletedObject for HKDeletedObject {}
pub trait IHKDeletedObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKDevice(pub id);
impl std::ops::Deref for HKDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKDevice {}
impl HKDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKDevice").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKDevice {}
impl PNSCopying for HKDevice {}
impl INSObject for HKDevice {}
impl PNSObject for HKDevice {}
impl std::convert::TryFrom<NSObject> for HKDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKDevice, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKDevice").unwrap()) };
        if is_kind_of {
            Ok(HKDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKDevice")
        }
    }
}
impl IHKDevice for HKDevice {}
pub trait IHKDevice: Sized + std::ops::Deref {
    unsafe fn initWithName_manufacturer_model_hardwareVersion_firmwareVersion_softwareVersion_localIdentifier_UDIDeviceIdentifier_(
        &self,
        name: NSString,
        manufacturer: NSString,
        model: NSString,
        hardwareVersion: NSString,
        firmwareVersion: NSString,
        softwareVersion: NSString,
        localIdentifier: NSString,
        UDIDeviceIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, manufacturer : manufacturer, model : model, hardwareVersion : hardwareVersion, firmwareVersion : firmwareVersion, softwareVersion : softwareVersion, localIdentifier : localIdentifier, UDIDeviceIdentifier : UDIDeviceIdentifier)
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
    unsafe fn manufacturer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturer)
    }
    unsafe fn model(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn hardwareVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hardwareVersion)
    }
    unsafe fn firmwareVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firmwareVersion)
    }
    unsafe fn softwareVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, softwareVersion)
    }
    unsafe fn localIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localIdentifier)
    }
    unsafe fn UDIDeviceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UDIDeviceIdentifier)
    }
    unsafe fn localDevice() -> HKDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKDevice").unwrap(), localDevice)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKDiscreteQuantitySample(pub id);
impl std::ops::Deref for HKDiscreteQuantitySample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKDiscreteQuantitySample {}
impl HKDiscreteQuantitySample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKDiscreteQuantitySample").unwrap(), alloc) })
    }
}
impl IHKQuantitySample for HKDiscreteQuantitySample {}
impl std::convert::TryFrom<HKQuantitySample> for HKDiscreteQuantitySample {
    type Error = &'static str;
    fn try_from(parent: HKQuantitySample) -> Result<HKDiscreteQuantitySample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKDiscreteQuantitySample").unwrap()) };
        if is_kind_of {
            Ok(HKDiscreteQuantitySample(parent.0))
        } else {
            Err("This HKQuantitySample cannot be downcasted to HKDiscreteQuantitySample")
        }
    }
}
impl IHKSample for HKDiscreteQuantitySample {}
impl IHKObject for HKDiscreteQuantitySample {}
impl PNSSecureCoding for HKDiscreteQuantitySample {}
impl INSObject for HKDiscreteQuantitySample {}
impl PNSObject for HKDiscreteQuantitySample {}
impl IHKDiscreteQuantitySample for HKDiscreteQuantitySample {}
pub trait IHKDiscreteQuantitySample: Sized + std::ops::Deref {
    unsafe fn minimumQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumQuantity)
    }
    unsafe fn averageQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averageQuantity)
    }
    unsafe fn maximumQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumQuantity)
    }
    unsafe fn mostRecentQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mostRecentQuantity)
    }
    unsafe fn mostRecentQuantityDateInterval(&self) -> NSDateInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mostRecentQuantityDateInterval)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKDocumentQuery(pub id);
impl std::ops::Deref for HKDocumentQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKDocumentQuery {}
impl HKDocumentQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKDocumentQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKDocumentQuery {}
impl std::convert::TryFrom<HKQuery> for HKDocumentQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKDocumentQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKDocumentQuery").unwrap()) };
        if is_kind_of {
            Ok(HKDocumentQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKDocumentQuery")
        }
    }
}
impl INSObject for HKDocumentQuery {}
impl PNSObject for HKDocumentQuery {}
impl IHKDocumentQuery for HKDocumentQuery {}
pub trait IHKDocumentQuery: Sized + std::ops::Deref {
    unsafe fn initWithDocumentType_predicate_limit_sortDescriptors_includeDocumentData_resultsHandler_(
        &self,
        documentType: HKDocumentType,
        predicate: NSPredicate,
        limit: NSUInteger,
        sortDescriptors: NSArray,
        includeDocumentData: BOOL,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDocumentType : documentType, predicate : predicate, limit : limit, sortDescriptors : sortDescriptors, includeDocumentData : includeDocumentData, resultsHandler : resultsHandler)
    }
    unsafe fn limit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, limit)
    }
    unsafe fn sortDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortDescriptors)
    }
    unsafe fn includeDocumentData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeDocumentData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKElectrocardiogramVoltageMeasurement(pub id);
impl std::ops::Deref for HKElectrocardiogramVoltageMeasurement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKElectrocardiogramVoltageMeasurement {}
impl HKElectrocardiogramVoltageMeasurement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKElectrocardiogramVoltageMeasurement").unwrap(), alloc) })
    }
}
impl PNSCopying for HKElectrocardiogramVoltageMeasurement {}
impl INSObject for HKElectrocardiogramVoltageMeasurement {}
impl PNSObject for HKElectrocardiogramVoltageMeasurement {}
impl std::convert::TryFrom<NSObject> for HKElectrocardiogramVoltageMeasurement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKElectrocardiogramVoltageMeasurement, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKElectrocardiogramVoltageMeasurement").unwrap())
        };
        if is_kind_of {
            Ok(HKElectrocardiogramVoltageMeasurement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKElectrocardiogramVoltageMeasurement")
        }
    }
}
impl IHKElectrocardiogramVoltageMeasurement for HKElectrocardiogramVoltageMeasurement {}
pub trait IHKElectrocardiogramVoltageMeasurement: Sized + std::ops::Deref {
    unsafe fn quantityForLead_(&self, lead: HKElectrocardiogramLead) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quantityForLead : lead)
    }
    unsafe fn timeSinceSampleStart(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeSinceSampleStart)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKElectrocardiogramQuery(pub id);
impl std::ops::Deref for HKElectrocardiogramQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKElectrocardiogramQuery {}
impl HKElectrocardiogramQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKElectrocardiogramQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKElectrocardiogramQuery {}
impl std::convert::TryFrom<HKQuery> for HKElectrocardiogramQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKElectrocardiogramQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKElectrocardiogramQuery").unwrap()) };
        if is_kind_of {
            Ok(HKElectrocardiogramQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKElectrocardiogramQuery")
        }
    }
}
impl INSObject for HKElectrocardiogramQuery {}
impl PNSObject for HKElectrocardiogramQuery {}
impl IHKElectrocardiogramQuery for HKElectrocardiogramQuery {}
pub trait IHKElectrocardiogramQuery: Sized + std::ops::Deref {
    unsafe fn initWithElectrocardiogram_dataHandler_(
        &self,
        electrocardiogram: HKElectrocardiogram,
        dataHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElectrocardiogram : electrocardiogram, dataHandler : dataHandler)
    }
}
pub type HKFHIRRelease = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKFHIRVersion(pub id);
impl std::ops::Deref for HKFHIRVersion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKFHIRVersion {}
impl HKFHIRVersion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKFHIRVersion").unwrap(), alloc) })
    }
}
impl PNSCopying for HKFHIRVersion {}
impl PNSSecureCoding for HKFHIRVersion {}
impl INSObject for HKFHIRVersion {}
impl PNSObject for HKFHIRVersion {}
impl std::convert::TryFrom<NSObject> for HKFHIRVersion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKFHIRVersion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKFHIRVersion").unwrap()) };
        if is_kind_of {
            Ok(HKFHIRVersion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKFHIRVersion")
        }
    }
}
impl IHKFHIRVersion for HKFHIRVersion {}
pub trait IHKFHIRVersion: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn patchVersion(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, patchVersion)
    }
    unsafe fn FHIRRelease(&self) -> HKFHIRRelease
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, FHIRRelease)
    }
    unsafe fn stringRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringRepresentation)
    }
    unsafe fn versionFromVersionString_error_(
        versionString: NSString,
        errorOut: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKFHIRVersion").unwrap(), versionFromVersionString : versionString, error : errorOut)
    }
    unsafe fn primaryDSTU2Version() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKFHIRVersion").unwrap(), primaryDSTU2Version)
    }
    unsafe fn primaryR4Version() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKFHIRVersion").unwrap(), primaryR4Version)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKScoredAssessment(pub id);
impl std::ops::Deref for HKScoredAssessment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKScoredAssessment {}
impl HKScoredAssessment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKScoredAssessment").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKScoredAssessment {}
impl PNSCopying for HKScoredAssessment {}
impl IHKSample for HKScoredAssessment {}
impl std::convert::TryFrom<HKSample> for HKScoredAssessment {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKScoredAssessment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKScoredAssessment").unwrap()) };
        if is_kind_of {
            Ok(HKScoredAssessment(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKScoredAssessment")
        }
    }
}
impl IHKObject for HKScoredAssessment {}
impl INSObject for HKScoredAssessment {}
impl PNSObject for HKScoredAssessment {}
impl IHKScoredAssessment for HKScoredAssessment {}
pub trait IHKScoredAssessment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn score(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, score)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKScoredAssessment").unwrap(), new)
    }
}
pub type HKGAD7AssessmentRisk = NSInteger;
pub type HKGAD7AssessmentAnswer = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKGAD7Assessment(pub id);
impl std::ops::Deref for HKGAD7Assessment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKGAD7Assessment {}
impl HKGAD7Assessment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKGAD7Assessment").unwrap(), alloc) })
    }
}
impl IHKScoredAssessment for HKGAD7Assessment {}
impl PNSSecureCoding for HKGAD7Assessment {}
impl PNSCopying for HKGAD7Assessment {}
impl From<HKGAD7Assessment> for HKScoredAssessment {
    fn from(child: HKGAD7Assessment) -> HKScoredAssessment {
        HKScoredAssessment(child.0)
    }
}
impl std::convert::TryFrom<HKScoredAssessment> for HKGAD7Assessment {
    type Error = &'static str;
    fn try_from(parent: HKScoredAssessment) -> Result<HKGAD7Assessment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKGAD7Assessment").unwrap()) };
        if is_kind_of {
            Ok(HKGAD7Assessment(parent.0))
        } else {
            Err("This HKScoredAssessment cannot be downcasted to HKGAD7Assessment")
        }
    }
}
impl IHKSample for HKGAD7Assessment {}
impl IHKObject for HKGAD7Assessment {}
impl INSObject for HKGAD7Assessment {}
impl PNSObject for HKGAD7Assessment {}
impl IHKGAD7Assessment for HKGAD7Assessment {}
pub trait IHKGAD7Assessment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn answers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, answers)
    }
    unsafe fn risk(&self) -> HKGAD7AssessmentRisk
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, risk)
    }
    unsafe fn assessmentWithDate_answers_(date: NSDate, answers: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKGAD7Assessment").unwrap(), assessmentWithDate : date, answers : answers)
    }
    unsafe fn assessmentWithDate_answers_metadata_(
        date: NSDate,
        answers: NSArray,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKGAD7Assessment").unwrap(), assessmentWithDate : date, answers : answers, metadata : metadata)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKGAD7Assessment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKGlassesLensSpecification(pub id);
impl std::ops::Deref for HKGlassesLensSpecification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKGlassesLensSpecification {}
impl HKGlassesLensSpecification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKGlassesLensSpecification").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKGlassesLensSpecification {}
impl PNSCopying for HKGlassesLensSpecification {}
impl IHKLensSpecification for HKGlassesLensSpecification {}
impl std::convert::TryFrom<HKLensSpecification> for HKGlassesLensSpecification {
    type Error = &'static str;
    fn try_from(parent: HKLensSpecification) -> Result<HKGlassesLensSpecification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKGlassesLensSpecification").unwrap()) };
        if is_kind_of {
            Ok(HKGlassesLensSpecification(parent.0))
        } else {
            Err("This HKLensSpecification cannot be downcasted to HKGlassesLensSpecification")
        }
    }
}
impl INSObject for HKGlassesLensSpecification {}
impl PNSObject for HKGlassesLensSpecification {}
impl IHKGlassesLensSpecification for HKGlassesLensSpecification {}
pub trait IHKGlassesLensSpecification: Sized + std::ops::Deref {
    unsafe fn initWithSphere_cylinder_axis_addPower_vertexDistance_prism_farPupillaryDistance_nearPupillaryDistance_(
        &self,
        sphere: HKQuantity,
        cylinder: HKQuantity,
        axis: HKQuantity,
        addPower: HKQuantity,
        vertexDistance: HKQuantity,
        prism: HKVisionPrism,
        farPupillaryDistance: HKQuantity,
        nearPupillaryDistance: HKQuantity,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSphere : sphere, cylinder : cylinder, axis : axis, addPower : addPower, vertexDistance : vertexDistance, prism : prism, farPupillaryDistance : farPupillaryDistance, nearPupillaryDistance : nearPupillaryDistance)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn vertexDistance(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexDistance)
    }
    unsafe fn prism(&self) -> HKVisionPrism
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prism)
    }
    unsafe fn farPupillaryDistance(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, farPupillaryDistance)
    }
    unsafe fn nearPupillaryDistance(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nearPupillaryDistance)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKGlassesLensSpecification").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKGlassesPrescription(pub id);
impl std::ops::Deref for HKGlassesPrescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKGlassesPrescription {}
impl HKGlassesPrescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKGlassesPrescription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKGlassesPrescription {}
impl PNSCopying for HKGlassesPrescription {}
impl IHKVisionPrescription for HKGlassesPrescription {}
impl std::convert::TryFrom<HKVisionPrescription> for HKGlassesPrescription {
    type Error = &'static str;
    fn try_from(parent: HKVisionPrescription) -> Result<HKGlassesPrescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKGlassesPrescription").unwrap()) };
        if is_kind_of {
            Ok(HKGlassesPrescription(parent.0))
        } else {
            Err("This HKVisionPrescription cannot be downcasted to HKGlassesPrescription")
        }
    }
}
impl IHKSample for HKGlassesPrescription {}
impl IHKObject for HKGlassesPrescription {}
impl INSObject for HKGlassesPrescription {}
impl PNSObject for HKGlassesPrescription {}
impl IHKGlassesPrescription for HKGlassesPrescription {}
pub trait IHKGlassesPrescription: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn rightEye(&self) -> HKGlassesLensSpecification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEye)
    }
    unsafe fn leftEye(&self) -> HKGlassesLensSpecification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEye)
    }
    unsafe fn prescriptionWithRightEyeSpecification_leftEyeSpecification_dateIssued_expirationDate_device_metadata_(
        rightEyeSpecification: HKGlassesLensSpecification,
        leftEyeSpecification: HKGlassesLensSpecification,
        dateIssued: NSDate,
        expirationDate: NSDate,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKGlassesPrescription").unwrap(), prescriptionWithRightEyeSpecification : rightEyeSpecification, leftEyeSpecification : leftEyeSpecification, dateIssued : dateIssued, expirationDate : expirationDate, device : device, metadata : metadata)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKGlassesPrescription").unwrap(), new)
    }
    unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata_(
        type_: HKVisionPrescriptionType,
        dateIssued: NSDate,
        expirationDate: NSDate,
        device: HKDevice,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKGlassesPrescription").unwrap(), prescriptionWithType : type_, dateIssued : dateIssued, expirationDate : expirationDate, device : device, metadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKHealthStore(pub id);
impl std::ops::Deref for HKHealthStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKHealthStore {}
impl HKHealthStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKHealthStore").unwrap(), alloc) })
    }
}
impl INSObject for HKHealthStore {}
impl PNSObject for HKHealthStore {}
impl std::convert::TryFrom<NSObject> for HKHealthStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKHealthStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKHealthStore").unwrap()) };
        if is_kind_of {
            Ok(HKHealthStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKHealthStore")
        }
    }
}
impl IHKHealthStore for HKHealthStore {}
pub trait IHKHealthStore: Sized + std::ops::Deref {
    unsafe fn supportsHealthRecords(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsHealthRecords)
    }
    unsafe fn authorizationStatusForType_(&self, type_: HKObjectType) -> HKAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationStatusForType : type_)
    }
    unsafe fn requestAuthorizationToShareTypes_readTypes_completion_(
        &self,
        typesToShare: NSSet,
        typesToRead: NSSet,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAuthorizationToShareTypes : typesToShare, readTypes : typesToRead, completion : completion)
    }
    unsafe fn requestPerObjectReadAuthorizationForType_predicate_completion_(
        &self,
        objectType: HKObjectType,
        predicate: NSPredicate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestPerObjectReadAuthorizationForType : objectType, predicate : predicate, completion : completion)
    }
    unsafe fn getRequestStatusForAuthorizationToShareTypes_readTypes_completion_(
        &self,
        typesToShare: NSSet,
        typesToRead: NSSet,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getRequestStatusForAuthorizationToShareTypes : typesToShare, readTypes : typesToRead, completion : completion)
    }
    unsafe fn handleAuthorizationForExtensionWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleAuthorizationForExtensionWithCompletion : completion)
    }
    unsafe fn earliestPermittedSampleDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, earliestPermittedSampleDate)
    }
    unsafe fn saveObject_withCompletion_(
        &self,
        object: HKObject,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveObject : object, withCompletion : completion)
    }
    unsafe fn saveObjects_withCompletion_(
        &self,
        objects: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveObjects : objects, withCompletion : completion)
    }
    unsafe fn deleteObject_withCompletion_(
        &self,
        object: HKObject,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteObject : object, withCompletion : completion)
    }
    unsafe fn deleteObjects_withCompletion_(
        &self,
        objects: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteObjects : objects, withCompletion : completion)
    }
    unsafe fn deleteObjectsOfType_predicate_withCompletion_(
        &self,
        objectType: HKObjectType,
        predicate: NSPredicate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteObjectsOfType : objectType, predicate : predicate, withCompletion : completion)
    }
    unsafe fn executeQuery_(&self, query: HKQuery)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeQuery : query)
    }
    unsafe fn stopQuery_(&self, query: HKQuery)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopQuery : query)
    }
    unsafe fn splitTotalEnergy_startDate_endDate_resultsHandler_(
        &self,
        totalEnergy: HKQuantity,
        startDate: NSDate,
        endDate: NSDate,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, splitTotalEnergy : totalEnergy, startDate : startDate, endDate : endDate, resultsHandler : resultsHandler)
    }
    unsafe fn dateOfBirthWithError_(&self, error: *mut NSError) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dateOfBirthWithError : error)
    }
    unsafe fn dateOfBirthComponentsWithError_(&self, error: *mut NSError) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dateOfBirthComponentsWithError : error)
    }
    unsafe fn biologicalSexWithError_(&self, error: *mut NSError) -> HKBiologicalSexObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, biologicalSexWithError : error)
    }
    unsafe fn bloodTypeWithError_(&self, error: *mut NSError) -> HKBloodTypeObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bloodTypeWithError : error)
    }
    unsafe fn fitzpatrickSkinTypeWithError_(
        &self,
        error: *mut NSError,
    ) -> HKFitzpatrickSkinTypeObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fitzpatrickSkinTypeWithError : error)
    }
    unsafe fn wheelchairUseWithError_(&self, error: *mut NSError) -> HKWheelchairUseObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, wheelchairUseWithError : error)
    }
    unsafe fn activityMoveModeWithError_(&self, error: *mut NSError) -> HKActivityMoveModeObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activityMoveModeWithError : error)
    }
    unsafe fn isHealthDataAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKHealthStore").unwrap(), isHealthDataAvailable)
    }
}
impl HKHealthStore_HKWorkout for HKHealthStore {}
pub trait HKHealthStore_HKWorkout: Sized + std::ops::Deref {
    unsafe fn addSamples_toWorkout_completion_(
        &self,
        samples: NSArray,
        workout: HKWorkout,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSamples : samples, toWorkout : workout, completion : completion)
    }
    unsafe fn startWorkoutSession_(&self, workoutSession: HKWorkoutSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWorkoutSession : workoutSession)
    }
    unsafe fn endWorkoutSession_(&self, workoutSession: HKWorkoutSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endWorkoutSession : workoutSession)
    }
    unsafe fn pauseWorkoutSession_(&self, workoutSession: HKWorkoutSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseWorkoutSession : workoutSession)
    }
    unsafe fn resumeWorkoutSession_(&self, workoutSession: HKWorkoutSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeWorkoutSession : workoutSession)
    }
    unsafe fn startWatchAppWithWorkoutConfiguration_completion_(
        &self,
        workoutConfiguration: HKWorkoutConfiguration,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWatchAppWithWorkoutConfiguration : workoutConfiguration, completion : completion)
    }
    unsafe fn recoverActiveWorkoutSessionWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recoverActiveWorkoutSessionWithCompletion : completion)
    }
    unsafe fn workoutSessionMirroringStartHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutSessionMirroringStartHandler)
    }
    unsafe fn setWorkoutSessionMirroringStartHandler_(
        &self,
        workoutSessionMirroringStartHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorkoutSessionMirroringStartHandler : workoutSessionMirroringStartHandler)
    }
}
impl HKHealthStore_HKBackgroundDelivery for HKHealthStore {}
pub trait HKHealthStore_HKBackgroundDelivery: Sized + std::ops::Deref {
    unsafe fn enableBackgroundDeliveryForType_frequency_withCompletion_(
        &self,
        type_: HKObjectType,
        frequency: HKUpdateFrequency,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enableBackgroundDeliveryForType : type_, frequency : frequency, withCompletion : completion)
    }
    unsafe fn disableBackgroundDeliveryForType_withCompletion_(
        &self,
        type_: HKObjectType,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disableBackgroundDeliveryForType : type_, withCompletion : completion)
    }
    unsafe fn disableAllBackgroundDeliveryWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disableAllBackgroundDeliveryWithCompletion : completion)
    }
}
impl HKHealthStore_HKUserPreferences for HKHealthStore {}
pub trait HKHealthStore_HKUserPreferences: Sized + std::ops::Deref {
    unsafe fn preferredUnitsForQuantityTypes_completion_(
        &self,
        quantityTypes: NSSet,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preferredUnitsForQuantityTypes : quantityTypes, completion : completion)
    }
}
impl HKHealthStore_HKRecalibrateEstimates for HKHealthStore {}
pub trait HKHealthStore_HKRecalibrateEstimates: Sized + std::ops::Deref {
    unsafe fn recalibrateEstimatesForSampleType_atDate_completion_(
        &self,
        sampleType: HKSampleType,
        date: NSDate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recalibrateEstimatesForSampleType : sampleType, atDate : date, completion : completion)
    }
}
impl HKHealthStore_HKWorkoutRelationship for HKHealthStore {}
pub trait HKHealthStore_HKWorkoutRelationship: Sized + std::ops::Deref {
    unsafe fn relateWorkoutEffortSample_withWorkout_activity_completion_(
        &self,
        sample: HKSample,
        workout: HKWorkout,
        activity: HKWorkoutActivity,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, relateWorkoutEffortSample : sample, withWorkout : workout, activity : activity, completion : completion)
    }
    unsafe fn unrelateWorkoutEffortSample_fromWorkout_activity_completion_(
        &self,
        sample: HKSample,
        workout: HKWorkout,
        activity: HKWorkoutActivity,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unrelateWorkoutEffortSample : sample, fromWorkout : workout, activity : activity, completion : completion)
    }
}
pub type HKHealthConceptDomain = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKHealthConceptIdentifier(pub id);
impl std::ops::Deref for HKHealthConceptIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKHealthConceptIdentifier {}
impl HKHealthConceptIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKHealthConceptIdentifier").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKHealthConceptIdentifier {}
impl PNSCopying for HKHealthConceptIdentifier {}
impl INSObject for HKHealthConceptIdentifier {}
impl PNSObject for HKHealthConceptIdentifier {}
impl std::convert::TryFrom<NSObject> for HKHealthConceptIdentifier {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKHealthConceptIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKHealthConceptIdentifier").unwrap()) };
        if is_kind_of {
            Ok(HKHealthConceptIdentifier(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKHealthConceptIdentifier")
        }
    }
}
impl IHKHealthConceptIdentifier for HKHealthConceptIdentifier {}
pub trait IHKHealthConceptIdentifier: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn domain(&self) -> HKHealthConceptDomain
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domain)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSeriesBuilder(pub id);
impl std::ops::Deref for HKSeriesBuilder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSeriesBuilder {}
impl HKSeriesBuilder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSeriesBuilder").unwrap(), alloc) })
    }
}
impl INSObject for HKSeriesBuilder {}
impl PNSObject for HKSeriesBuilder {}
impl std::convert::TryFrom<NSObject> for HKSeriesBuilder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKSeriesBuilder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSeriesBuilder").unwrap()) };
        if is_kind_of {
            Ok(HKSeriesBuilder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKSeriesBuilder")
        }
    }
}
impl IHKSeriesBuilder for HKSeriesBuilder {}
pub trait IHKSeriesBuilder: Sized + std::ops::Deref {
    unsafe fn discard(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discard)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKHeartbeatSeriesBuilder(pub id);
impl std::ops::Deref for HKHeartbeatSeriesBuilder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKHeartbeatSeriesBuilder {}
impl HKHeartbeatSeriesBuilder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKHeartbeatSeriesBuilder").unwrap(), alloc) })
    }
}
impl IHKSeriesBuilder for HKHeartbeatSeriesBuilder {}
impl From<HKHeartbeatSeriesBuilder> for HKSeriesBuilder {
    fn from(child: HKHeartbeatSeriesBuilder) -> HKSeriesBuilder {
        HKSeriesBuilder(child.0)
    }
}
impl std::convert::TryFrom<HKSeriesBuilder> for HKHeartbeatSeriesBuilder {
    type Error = &'static str;
    fn try_from(parent: HKSeriesBuilder) -> Result<HKHeartbeatSeriesBuilder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKHeartbeatSeriesBuilder").unwrap()) };
        if is_kind_of {
            Ok(HKHeartbeatSeriesBuilder(parent.0))
        } else {
            Err("This HKSeriesBuilder cannot be downcasted to HKHeartbeatSeriesBuilder")
        }
    }
}
impl INSObject for HKHeartbeatSeriesBuilder {}
impl PNSObject for HKHeartbeatSeriesBuilder {}
impl IHKHeartbeatSeriesBuilder for HKHeartbeatSeriesBuilder {}
pub trait IHKHeartbeatSeriesBuilder: Sized + std::ops::Deref {
    unsafe fn initWithHealthStore_device_startDate_(
        &self,
        healthStore: HKHealthStore,
        device: HKDevice,
        startDate: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHealthStore : healthStore, device : device, startDate : startDate)
    }
    unsafe fn addHeartbeatWithTimeIntervalSinceSeriesStartDate_precededByGap_completion_(
        &self,
        timeIntervalSinceStart: NSTimeInterval,
        precededByGap: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addHeartbeatWithTimeIntervalSinceSeriesStartDate : timeIntervalSinceStart, precededByGap : precededByGap, completion : completion)
    }
    unsafe fn addMetadata_completion_(
        &self,
        metadata: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMetadata : metadata, completion : completion)
    }
    unsafe fn finishSeriesWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishSeriesWithCompletion : completion)
    }
    unsafe fn maximumCount() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKHeartbeatSeriesBuilder").unwrap(), maximumCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKHeartbeatSeriesQuery(pub id);
impl std::ops::Deref for HKHeartbeatSeriesQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKHeartbeatSeriesQuery {}
impl HKHeartbeatSeriesQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKHeartbeatSeriesQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKHeartbeatSeriesQuery {}
impl std::convert::TryFrom<HKQuery> for HKHeartbeatSeriesQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKHeartbeatSeriesQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKHeartbeatSeriesQuery").unwrap()) };
        if is_kind_of {
            Ok(HKHeartbeatSeriesQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKHeartbeatSeriesQuery")
        }
    }
}
impl INSObject for HKHeartbeatSeriesQuery {}
impl PNSObject for HKHeartbeatSeriesQuery {}
impl IHKHeartbeatSeriesQuery for HKHeartbeatSeriesQuery {}
pub trait IHKHeartbeatSeriesQuery: Sized + std::ops::Deref {
    unsafe fn initWithHeartbeatSeries_dataHandler_(
        &self,
        heartbeatSeries: HKHeartbeatSeriesSample,
        dataHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeartbeatSeries : heartbeatSeries, dataHandler : dataHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSeriesSample(pub id);
impl std::ops::Deref for HKSeriesSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSeriesSample {}
impl HKSeriesSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSeriesSample").unwrap(), alloc) })
    }
}
impl IHKSample for HKSeriesSample {}
impl std::convert::TryFrom<HKSample> for HKSeriesSample {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKSeriesSample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSeriesSample").unwrap()) };
        if is_kind_of {
            Ok(HKSeriesSample(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKSeriesSample")
        }
    }
}
impl IHKObject for HKSeriesSample {}
impl PNSSecureCoding for HKSeriesSample {}
impl INSObject for HKSeriesSample {}
impl PNSObject for HKSeriesSample {}
impl IHKSeriesSample for HKSeriesSample {}
pub trait IHKSeriesSample: Sized + std::ops::Deref {
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKHeartbeatSeriesSample(pub id);
impl std::ops::Deref for HKHeartbeatSeriesSample {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKHeartbeatSeriesSample {}
impl HKHeartbeatSeriesSample {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKHeartbeatSeriesSample").unwrap(), alloc) })
    }
}
impl IHKSeriesSample for HKHeartbeatSeriesSample {}
impl From<HKHeartbeatSeriesSample> for HKSeriesSample {
    fn from(child: HKHeartbeatSeriesSample) -> HKSeriesSample {
        HKSeriesSample(child.0)
    }
}
impl std::convert::TryFrom<HKSeriesSample> for HKHeartbeatSeriesSample {
    type Error = &'static str;
    fn try_from(parent: HKSeriesSample) -> Result<HKHeartbeatSeriesSample, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKHeartbeatSeriesSample").unwrap()) };
        if is_kind_of {
            Ok(HKHeartbeatSeriesSample(parent.0))
        } else {
            Err("This HKSeriesSample cannot be downcasted to HKHeartbeatSeriesSample")
        }
    }
}
impl IHKSample for HKHeartbeatSeriesSample {}
impl IHKObject for HKHeartbeatSeriesSample {}
impl PNSSecureCoding for HKHeartbeatSeriesSample {}
impl INSObject for HKHeartbeatSeriesSample {}
impl PNSObject for HKHeartbeatSeriesSample {}
impl IHKHeartbeatSeriesSample for HKHeartbeatSeriesSample {}
pub trait IHKHeartbeatSeriesSample: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutBuilder(pub id);
impl std::ops::Deref for HKWorkoutBuilder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutBuilder {}
impl HKWorkoutBuilder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutBuilder").unwrap(), alloc) })
    }
}
impl INSObject for HKWorkoutBuilder {}
impl PNSObject for HKWorkoutBuilder {}
impl std::convert::TryFrom<NSObject> for HKWorkoutBuilder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKWorkoutBuilder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutBuilder").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutBuilder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKWorkoutBuilder")
        }
    }
}
impl IHKWorkoutBuilder for HKWorkoutBuilder {}
pub trait IHKWorkoutBuilder: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithHealthStore_configuration_device_(
        &self,
        healthStore: HKHealthStore,
        configuration: HKWorkoutConfiguration,
        device: HKDevice,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHealthStore : healthStore, configuration : configuration, device : device)
    }
    unsafe fn beginCollectionWithStartDate_completion_(
        &self,
        startDate: NSDate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginCollectionWithStartDate : startDate, completion : completion)
    }
    unsafe fn addSamples_completion_(
        &self,
        samples: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSamples : samples, completion : completion)
    }
    unsafe fn addWorkoutEvents_completion_(
        &self,
        workoutEvents: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addWorkoutEvents : workoutEvents, completion : completion)
    }
    unsafe fn addMetadata_completion_(
        &self,
        metadata: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMetadata : metadata, completion : completion)
    }
    unsafe fn addWorkoutActivity_completion_(
        &self,
        workoutActivity: HKWorkoutActivity,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addWorkoutActivity : workoutActivity, completion : completion)
    }
    unsafe fn updateActivityWithUUID_endDate_completion_(
        &self,
        UUID: NSUUID,
        endDate: NSDate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateActivityWithUUID : UUID, endDate : endDate, completion : completion)
    }
    unsafe fn updateActivityWithUUID_addMedatata_completion_(
        &self,
        UUID: NSUUID,
        metadata: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateActivityWithUUID : UUID, addMedatata : metadata, completion : completion)
    }
    unsafe fn endCollectionWithEndDate_completion_(
        &self,
        endDate: NSDate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endCollectionWithEndDate : endDate, completion : completion)
    }
    unsafe fn finishWorkoutWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishWorkoutWithCompletion : completion)
    }
    unsafe fn discardWorkout(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardWorkout)
    }
    unsafe fn elapsedTimeAtDate_(&self, date: NSDate) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elapsedTimeAtDate : date)
    }
    unsafe fn statisticsForType_(&self, quantityType: HKQuantityType) -> HKStatistics
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, statisticsForType : quantityType)
    }
    unsafe fn seriesBuilderForType_(&self, seriesType: HKSeriesType) -> HKSeriesBuilder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, seriesBuilderForType : seriesType)
    }
    unsafe fn device(&self) -> HKDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
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
    unsafe fn workoutConfiguration(&self) -> HKWorkoutConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutConfiguration)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn workoutEvents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutEvents)
    }
    unsafe fn workoutActivities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutActivities)
    }
    unsafe fn allStatistics(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allStatistics)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKLiveWorkoutDataSource(pub id);
impl std::ops::Deref for HKLiveWorkoutDataSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKLiveWorkoutDataSource {}
impl HKLiveWorkoutDataSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKLiveWorkoutDataSource").unwrap(), alloc) })
    }
}
impl INSObject for HKLiveWorkoutDataSource {}
impl PNSObject for HKLiveWorkoutDataSource {}
impl std::convert::TryFrom<NSObject> for HKLiveWorkoutDataSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKLiveWorkoutDataSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKLiveWorkoutDataSource").unwrap()) };
        if is_kind_of {
            Ok(HKLiveWorkoutDataSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKLiveWorkoutDataSource")
        }
    }
}
impl IHKLiveWorkoutDataSource for HKLiveWorkoutDataSource {}
pub trait IHKLiveWorkoutDataSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithHealthStore_workoutConfiguration_(
        &self,
        healthStore: HKHealthStore,
        configuration: HKWorkoutConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHealthStore : healthStore, workoutConfiguration : configuration)
    }
    unsafe fn enableCollectionForType_predicate_(
        &self,
        quantityType: HKQuantityType,
        predicate: NSPredicate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enableCollectionForType : quantityType, predicate : predicate)
    }
    unsafe fn disableCollectionForType_(&self, quantityType: HKQuantityType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disableCollectionForType : quantityType)
    }
    unsafe fn typesToCollect(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typesToCollect)
    }
}
pub trait PHKLiveWorkoutBuilderDelegate: Sized + std::ops::Deref {
    unsafe fn workoutBuilder_didCollectDataOfTypes_(
        &self,
        workoutBuilder: HKLiveWorkoutBuilder,
        collectedTypes: NSSet,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutBuilder : workoutBuilder, didCollectDataOfTypes : collectedTypes)
    }
    unsafe fn workoutBuilderDidCollectEvent_(&self, workoutBuilder: HKLiveWorkoutBuilder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutBuilderDidCollectEvent : workoutBuilder)
    }
    unsafe fn workoutBuilder_didBeginActivity_(
        &self,
        workoutBuilder: HKLiveWorkoutBuilder,
        workoutActivity: HKWorkoutActivity,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutBuilder : workoutBuilder, didBeginActivity : workoutActivity)
    }
    unsafe fn workoutBuilder_didEndActivity_(
        &self,
        workoutBuilder: HKLiveWorkoutBuilder,
        workoutActivity: HKWorkoutActivity,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutBuilder : workoutBuilder, didEndActivity : workoutActivity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKLiveWorkoutBuilder(pub id);
impl std::ops::Deref for HKLiveWorkoutBuilder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKLiveWorkoutBuilder {}
impl HKLiveWorkoutBuilder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKLiveWorkoutBuilder").unwrap(), alloc) })
    }
}
impl IHKWorkoutBuilder for HKLiveWorkoutBuilder {}
impl From<HKLiveWorkoutBuilder> for HKWorkoutBuilder {
    fn from(child: HKLiveWorkoutBuilder) -> HKWorkoutBuilder {
        HKWorkoutBuilder(child.0)
    }
}
impl std::convert::TryFrom<HKWorkoutBuilder> for HKLiveWorkoutBuilder {
    type Error = &'static str;
    fn try_from(parent: HKWorkoutBuilder) -> Result<HKLiveWorkoutBuilder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKLiveWorkoutBuilder").unwrap()) };
        if is_kind_of {
            Ok(HKLiveWorkoutBuilder(parent.0))
        } else {
            Err("This HKWorkoutBuilder cannot be downcasted to HKLiveWorkoutBuilder")
        }
    }
}
impl INSObject for HKLiveWorkoutBuilder {}
impl PNSObject for HKLiveWorkoutBuilder {}
impl IHKLiveWorkoutBuilder for HKLiveWorkoutBuilder {}
pub trait IHKLiveWorkoutBuilder: Sized + std::ops::Deref {
    unsafe fn initWithHealthStore_configuration_device_(
        &self,
        healthStore: HKHealthStore,
        configuration: HKWorkoutConfiguration,
        device: HKDevice,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHealthStore : healthStore, configuration : configuration, device : device)
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
    unsafe fn workoutSession(&self) -> HKWorkoutSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutSession)
    }
    unsafe fn shouldCollectWorkoutEvents(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCollectWorkoutEvents)
    }
    unsafe fn setShouldCollectWorkoutEvents_(&self, shouldCollectWorkoutEvents: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldCollectWorkoutEvents : shouldCollectWorkoutEvents)
    }
    unsafe fn dataSource(&self) -> HKLiveWorkoutDataSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSource)
    }
    unsafe fn setDataSource_(&self, dataSource: HKLiveWorkoutDataSource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataSource : dataSource)
    }
    unsafe fn elapsedTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elapsedTime)
    }
    unsafe fn currentWorkoutActivity(&self) -> HKWorkoutActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentWorkoutActivity)
    }
}
pub type HKMedicationGeneralForm = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKMedicationConcept(pub id);
impl std::ops::Deref for HKMedicationConcept {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKMedicationConcept {}
impl HKMedicationConcept {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKMedicationConcept").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKMedicationConcept {}
impl PNSCopying for HKMedicationConcept {}
impl INSObject for HKMedicationConcept {}
impl PNSObject for HKMedicationConcept {}
impl std::convert::TryFrom<NSObject> for HKMedicationConcept {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKMedicationConcept, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKMedicationConcept").unwrap()) };
        if is_kind_of {
            Ok(HKMedicationConcept(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKMedicationConcept")
        }
    }
}
impl IHKMedicationConcept for HKMedicationConcept {}
pub trait IHKMedicationConcept: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> HKHealthConceptIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn displayText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayText)
    }
    unsafe fn generalForm(&self) -> HKMedicationGeneralForm
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, generalForm)
    }
    unsafe fn relatedCodings(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relatedCodings)
    }
}
pub type HKAppleECGAlgorithmVersion = NSInteger;
pub type HKBloodGlucoseMealTime = NSInteger;
pub type HKBodyTemperatureSensorLocation = NSInteger;
pub type HKCyclingFunctionalThresholdPowerTestType = NSInteger;
pub type HKDevicePlacementSide = NSInteger;
pub type HKHeartRateMotionContext = NSInteger;
pub type HKHeartRateRecoveryTestType = NSInteger;
pub type HKHeartRateSensorLocation = NSInteger;
pub type HKInsulinDeliveryReason = NSInteger;
pub type HKPhysicalEffortEstimationType = NSInteger;
pub type HKSwimmingStrokeStyle = NSInteger;
pub type HKUserMotionContext = NSInteger;
pub type HKVO2MaxTestType = NSInteger;
pub type HKWaterSalinity = NSInteger;
pub type HKWeatherCondition = NSInteger;
pub type HKWorkoutSwimmingLocationType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKObserverQuery(pub id);
impl std::ops::Deref for HKObserverQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKObserverQuery {}
impl HKObserverQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKObserverQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKObserverQuery {}
impl std::convert::TryFrom<HKQuery> for HKObserverQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKObserverQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKObserverQuery").unwrap()) };
        if is_kind_of {
            Ok(HKObserverQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKObserverQuery")
        }
    }
}
impl INSObject for HKObserverQuery {}
impl PNSObject for HKObserverQuery {}
impl IHKObserverQuery for HKObserverQuery {}
pub trait IHKObserverQuery: Sized + std::ops::Deref {
    unsafe fn initWithSampleType_predicate_updateHandler_(
        &self,
        sampleType: HKSampleType,
        predicate: NSPredicate,
        updateHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleType : sampleType, predicate : predicate, updateHandler : updateHandler)
    }
    unsafe fn initWithQueryDescriptors_updateHandler_(
        &self,
        queryDescriptors: NSArray,
        updateHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQueryDescriptors : queryDescriptors, updateHandler : updateHandler)
    }
}
pub type HKPHQ9AssessmentRisk = NSInteger;
pub type HKPHQ9AssessmentAnswer = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKPHQ9Assessment(pub id);
impl std::ops::Deref for HKPHQ9Assessment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKPHQ9Assessment {}
impl HKPHQ9Assessment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKPHQ9Assessment").unwrap(), alloc) })
    }
}
impl IHKScoredAssessment for HKPHQ9Assessment {}
impl PNSSecureCoding for HKPHQ9Assessment {}
impl PNSCopying for HKPHQ9Assessment {}
impl std::convert::TryFrom<HKScoredAssessment> for HKPHQ9Assessment {
    type Error = &'static str;
    fn try_from(parent: HKScoredAssessment) -> Result<HKPHQ9Assessment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKPHQ9Assessment").unwrap()) };
        if is_kind_of {
            Ok(HKPHQ9Assessment(parent.0))
        } else {
            Err("This HKScoredAssessment cannot be downcasted to HKPHQ9Assessment")
        }
    }
}
impl IHKSample for HKPHQ9Assessment {}
impl IHKObject for HKPHQ9Assessment {}
impl INSObject for HKPHQ9Assessment {}
impl PNSObject for HKPHQ9Assessment {}
impl IHKPHQ9Assessment for HKPHQ9Assessment {}
pub trait IHKPHQ9Assessment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn answers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, answers)
    }
    unsafe fn risk(&self) -> HKPHQ9AssessmentRisk
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, risk)
    }
    unsafe fn assessmentWithDate_answers_(date: NSDate, answers: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKPHQ9Assessment").unwrap(), assessmentWithDate : date, answers : answers)
    }
    unsafe fn assessmentWithDate_answers_metadata_(
        date: NSDate,
        answers: NSArray,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKPHQ9Assessment").unwrap(), assessmentWithDate : date, answers : answers, metadata : metadata)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKPHQ9Assessment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKQuantitySeriesSampleBuilder(pub id);
impl std::ops::Deref for HKQuantitySeriesSampleBuilder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKQuantitySeriesSampleBuilder {}
impl HKQuantitySeriesSampleBuilder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantitySeriesSampleBuilder").unwrap(), alloc) })
    }
}
impl INSObject for HKQuantitySeriesSampleBuilder {}
impl PNSObject for HKQuantitySeriesSampleBuilder {}
impl std::convert::TryFrom<NSObject> for HKQuantitySeriesSampleBuilder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKQuantitySeriesSampleBuilder, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKQuantitySeriesSampleBuilder").unwrap())
        };
        if is_kind_of {
            Ok(HKQuantitySeriesSampleBuilder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKQuantitySeriesSampleBuilder")
        }
    }
}
impl IHKQuantitySeriesSampleBuilder for HKQuantitySeriesSampleBuilder {}
pub trait IHKQuantitySeriesSampleBuilder: Sized + std::ops::Deref {
    unsafe fn initWithHealthStore_quantityType_startDate_device_(
        &self,
        healthStore: HKHealthStore,
        quantityType: HKQuantityType,
        startDate: NSDate,
        device: HKDevice,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHealthStore : healthStore, quantityType : quantityType, startDate : startDate, device : device)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn insertQuantity_dateInterval_error_(
        &self,
        quantity: HKQuantity,
        dateInterval: NSDateInterval,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertQuantity : quantity, dateInterval : dateInterval, error : error)
    }
    unsafe fn insertQuantity_date_error_(
        &self,
        quantity: HKQuantity,
        date: NSDate,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertQuantity : quantity, date : date, error : error)
    }
    unsafe fn finishSeriesWithMetadata_endDate_completion_(
        &self,
        metadata: NSDictionary,
        endDate: NSDate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishSeriesWithMetadata : metadata, endDate : endDate, completion : completion)
    }
    unsafe fn finishSeriesWithMetadata_completion_(
        &self,
        metadata: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishSeriesWithMetadata : metadata, completion : completion)
    }
    unsafe fn discard(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discard)
    }
    unsafe fn quantityType(&self) -> HKQuantityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quantityType)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn device(&self) -> HKDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKQuantitySeriesSampleQuery(pub id);
impl std::ops::Deref for HKQuantitySeriesSampleQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKQuantitySeriesSampleQuery {}
impl HKQuantitySeriesSampleQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKQuantitySeriesSampleQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKQuantitySeriesSampleQuery {}
impl std::convert::TryFrom<HKQuery> for HKQuantitySeriesSampleQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKQuantitySeriesSampleQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKQuantitySeriesSampleQuery").unwrap()) };
        if is_kind_of {
            Ok(HKQuantitySeriesSampleQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKQuantitySeriesSampleQuery")
        }
    }
}
impl INSObject for HKQuantitySeriesSampleQuery {}
impl PNSObject for HKQuantitySeriesSampleQuery {}
impl IHKQuantitySeriesSampleQuery for HKQuantitySeriesSampleQuery {}
pub trait IHKQuantitySeriesSampleQuery: Sized + std::ops::Deref {
    unsafe fn initWithQuantityType_predicate_quantityHandler_(
        &self,
        quantityType: HKQuantityType,
        predicate: NSPredicate,
        quantityHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQuantityType : quantityType, predicate : predicate, quantityHandler : quantityHandler)
    }
    unsafe fn initWithSample_quantityHandler_(
        &self,
        quantitySample: HKQuantitySample,
        quantityHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSample : quantitySample, quantityHandler : quantityHandler)
    }
    unsafe fn includeSample(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeSample)
    }
    unsafe fn setIncludeSample_(&self, includeSample: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeSample : includeSample)
    }
    unsafe fn orderByQuantitySampleStartDate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orderByQuantitySampleStartDate)
    }
    unsafe fn setOrderByQuantitySampleStartDate_(&self, orderByQuantitySampleStartDate: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrderByQuantitySampleStartDate : orderByQuantitySampleStartDate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSampleQuery(pub id);
impl std::ops::Deref for HKSampleQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSampleQuery {}
impl HKSampleQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSampleQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKSampleQuery {}
impl std::convert::TryFrom<HKQuery> for HKSampleQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKSampleQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSampleQuery").unwrap()) };
        if is_kind_of {
            Ok(HKSampleQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKSampleQuery")
        }
    }
}
impl INSObject for HKSampleQuery {}
impl PNSObject for HKSampleQuery {}
impl IHKSampleQuery for HKSampleQuery {}
pub trait IHKSampleQuery: Sized + std::ops::Deref {
    unsafe fn initWithSampleType_predicate_limit_sortDescriptors_resultsHandler_(
        &self,
        sampleType: HKSampleType,
        predicate: NSPredicate,
        limit: NSUInteger,
        sortDescriptors: NSArray,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleType : sampleType, predicate : predicate, limit : limit, sortDescriptors : sortDescriptors, resultsHandler : resultsHandler)
    }
    unsafe fn initWithQueryDescriptors_limit_resultsHandler_(
        &self,
        queryDescriptors: NSArray,
        limit: NSInteger,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQueryDescriptors : queryDescriptors, limit : limit, resultsHandler : resultsHandler)
    }
    unsafe fn initWithQueryDescriptors_limit_sortDescriptors_resultsHandler_(
        &self,
        queryDescriptors: NSArray,
        limit: NSInteger,
        sortDescriptors: NSArray,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQueryDescriptors : queryDescriptors, limit : limit, sortDescriptors : sortDescriptors, resultsHandler : resultsHandler)
    }
    unsafe fn limit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, limit)
    }
    unsafe fn sortDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortDescriptors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSource(pub id);
impl std::ops::Deref for HKSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSource {}
impl HKSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSource").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKSource {}
impl PNSCopying for HKSource {}
impl INSObject for HKSource {}
impl PNSObject for HKSource {}
impl std::convert::TryFrom<NSObject> for HKSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKSource, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSource").unwrap()) };
        if is_kind_of {
            Ok(HKSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKSource")
        }
    }
}
impl IHKSource for HKSource {}
pub trait IHKSource: Sized + std::ops::Deref {
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
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn defaultSource() -> HKSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKSource").unwrap(), defaultSource)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSourceQuery(pub id);
impl std::ops::Deref for HKSourceQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSourceQuery {}
impl HKSourceQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSourceQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKSourceQuery {}
impl std::convert::TryFrom<HKQuery> for HKSourceQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKSourceQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSourceQuery").unwrap()) };
        if is_kind_of {
            Ok(HKSourceQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKSourceQuery")
        }
    }
}
impl INSObject for HKSourceQuery {}
impl PNSObject for HKSourceQuery {}
impl IHKSourceQuery for HKSourceQuery {}
pub trait IHKSourceQuery: Sized + std::ops::Deref {
    unsafe fn initWithSampleType_samplePredicate_completionHandler_(
        &self,
        sampleType: HKSampleType,
        objectPredicate: NSPredicate,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleType : sampleType, samplePredicate : objectPredicate, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKSourceRevision(pub id);
impl std::ops::Deref for HKSourceRevision {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKSourceRevision {}
impl HKSourceRevision {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKSourceRevision").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKSourceRevision {}
impl PNSCopying for HKSourceRevision {}
impl INSObject for HKSourceRevision {}
impl PNSObject for HKSourceRevision {}
impl std::convert::TryFrom<NSObject> for HKSourceRevision {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKSourceRevision, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKSourceRevision").unwrap()) };
        if is_kind_of {
            Ok(HKSourceRevision(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKSourceRevision")
        }
    }
}
impl IHKSourceRevision for HKSourceRevision {}
pub trait IHKSourceRevision: Sized + std::ops::Deref {
    unsafe fn initWithSource_version_productType_operatingSystemVersion_(
        &self,
        source: HKSource,
        version: NSString,
        productType: NSString,
        operatingSystemVersion: NSOperatingSystemVersion,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source, version : version, productType : productType, operatingSystemVersion : operatingSystemVersion)
    }
    unsafe fn initWithSource_version_(&self, source: HKSource, version: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source, version : version)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn source(&self) -> HKSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn productType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productType)
    }
    unsafe fn operatingSystemVersion(&self) -> NSOperatingSystemVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operatingSystemVersion)
    }
}
pub type HKStatisticsOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKStatistics(pub id);
impl std::ops::Deref for HKStatistics {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKStatistics {}
impl HKStatistics {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKStatistics").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKStatistics {}
impl PNSCopying for HKStatistics {}
impl INSObject for HKStatistics {}
impl PNSObject for HKStatistics {}
impl std::convert::TryFrom<NSObject> for HKStatistics {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKStatistics, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKStatistics").unwrap()) };
        if is_kind_of {
            Ok(HKStatistics(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKStatistics")
        }
    }
}
impl IHKStatistics for HKStatistics {}
pub trait IHKStatistics: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn averageQuantityForSource_(&self, source: HKSource) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, averageQuantityForSource : source)
    }
    unsafe fn averageQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averageQuantity)
    }
    unsafe fn minimumQuantityForSource_(&self, source: HKSource) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, minimumQuantityForSource : source)
    }
    unsafe fn minimumQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumQuantity)
    }
    unsafe fn maximumQuantityForSource_(&self, source: HKSource) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maximumQuantityForSource : source)
    }
    unsafe fn maximumQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumQuantity)
    }
    unsafe fn mostRecentQuantityForSource_(&self, source: HKSource) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mostRecentQuantityForSource : source)
    }
    unsafe fn mostRecentQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mostRecentQuantity)
    }
    unsafe fn mostRecentQuantityDateIntervalForSource_(&self, source: HKSource) -> NSDateInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mostRecentQuantityDateIntervalForSource : source)
    }
    unsafe fn mostRecentQuantityDateInterval(&self) -> NSDateInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mostRecentQuantityDateInterval)
    }
    unsafe fn sumQuantityForSource_(&self, source: HKSource) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sumQuantityForSource : source)
    }
    unsafe fn sumQuantity(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sumQuantity)
    }
    unsafe fn duration(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn durationForSource_(&self, source: HKSource) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, durationForSource : source)
    }
    unsafe fn quantityType(&self) -> HKQuantityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quantityType)
    }
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
    unsafe fn sources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKStatisticsCollection(pub id);
impl std::ops::Deref for HKStatisticsCollection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKStatisticsCollection {}
impl HKStatisticsCollection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKStatisticsCollection").unwrap(), alloc) })
    }
}
impl INSObject for HKStatisticsCollection {}
impl PNSObject for HKStatisticsCollection {}
impl std::convert::TryFrom<NSObject> for HKStatisticsCollection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKStatisticsCollection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKStatisticsCollection").unwrap()) };
        if is_kind_of {
            Ok(HKStatisticsCollection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKStatisticsCollection")
        }
    }
}
impl IHKStatisticsCollection for HKStatisticsCollection {}
pub trait IHKStatisticsCollection: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn statisticsForDate_(&self, date: NSDate) -> HKStatistics
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, statisticsForDate : date)
    }
    unsafe fn enumerateStatisticsFromDate_toDate_withBlock_(
        &self,
        startDate: NSDate,
        endDate: NSDate,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateStatisticsFromDate : startDate, toDate : endDate, withBlock : block)
    }
    unsafe fn statistics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, statistics)
    }
    unsafe fn sources(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKStatisticsCollectionQuery(pub id);
impl std::ops::Deref for HKStatisticsCollectionQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKStatisticsCollectionQuery {}
impl HKStatisticsCollectionQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKStatisticsCollectionQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKStatisticsCollectionQuery {}
impl std::convert::TryFrom<HKQuery> for HKStatisticsCollectionQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKStatisticsCollectionQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKStatisticsCollectionQuery").unwrap()) };
        if is_kind_of {
            Ok(HKStatisticsCollectionQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKStatisticsCollectionQuery")
        }
    }
}
impl INSObject for HKStatisticsCollectionQuery {}
impl PNSObject for HKStatisticsCollectionQuery {}
impl IHKStatisticsCollectionQuery for HKStatisticsCollectionQuery {}
pub trait IHKStatisticsCollectionQuery: Sized + std::ops::Deref {
    unsafe fn initWithQuantityType_quantitySamplePredicate_options_anchorDate_intervalComponents_(
        &self,
        quantityType: HKQuantityType,
        quantitySamplePredicate: NSPredicate,
        options: HKStatisticsOptions,
        anchorDate: NSDate,
        intervalComponents: NSDateComponents,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQuantityType : quantityType, quantitySamplePredicate : quantitySamplePredicate, options : options, anchorDate : anchorDate, intervalComponents : intervalComponents)
    }
    unsafe fn anchorDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorDate)
    }
    unsafe fn options(&self) -> HKStatisticsOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn intervalComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intervalComponents)
    }
    unsafe fn initialResultsHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialResultsHandler)
    }
    unsafe fn setInitialResultsHandler_(&self, initialResultsHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialResultsHandler : initialResultsHandler)
    }
    unsafe fn statisticsUpdateHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, statisticsUpdateHandler)
    }
    unsafe fn setStatisticsUpdateHandler_(
        &self,
        statisticsUpdateHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatisticsUpdateHandler : statisticsUpdateHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKStatisticsQuery(pub id);
impl std::ops::Deref for HKStatisticsQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKStatisticsQuery {}
impl HKStatisticsQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKStatisticsQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKStatisticsQuery {}
impl std::convert::TryFrom<HKQuery> for HKStatisticsQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKStatisticsQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKStatisticsQuery").unwrap()) };
        if is_kind_of {
            Ok(HKStatisticsQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKStatisticsQuery")
        }
    }
}
impl INSObject for HKStatisticsQuery {}
impl PNSObject for HKStatisticsQuery {}
impl IHKStatisticsQuery for HKStatisticsQuery {}
pub trait IHKStatisticsQuery: Sized + std::ops::Deref {
    unsafe fn initWithQuantityType_quantitySamplePredicate_options_completionHandler_(
        &self,
        quantityType: HKQuantityType,
        quantitySamplePredicate: NSPredicate,
        options: HKStatisticsOptions,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQuantityType : quantityType, quantitySamplePredicate : quantitySamplePredicate, options : options, completionHandler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKUnit(pub id);
impl std::ops::Deref for HKUnit {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKUnit {}
impl HKUnit {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKUnit {}
impl PNSCopying for HKUnit {}
impl INSObject for HKUnit {}
impl PNSObject for HKUnit {}
impl std::convert::TryFrom<NSObject> for HKUnit {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKUnit, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKUnit").unwrap()) };
        if is_kind_of {
            Ok(HKUnit(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKUnit")
        }
    }
}
impl IHKUnit for HKUnit {}
pub trait IHKUnit: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isNull(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNull)
    }
    unsafe fn unitString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unitString)
    }
    unsafe fn unitFromString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), unitFromString : string)
    }
    unsafe fn unitFromMassFormatterUnit_(massFormatterUnit: NSMassFormatterUnit) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), unitFromMassFormatterUnit : massFormatterUnit)
    }
    unsafe fn massFormatterUnitFromUnit_(unit: HKUnit) -> NSMassFormatterUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), massFormatterUnitFromUnit : unit)
    }
    unsafe fn unitFromLengthFormatterUnit_(
        lengthFormatterUnit: NSLengthFormatterUnit,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), unitFromLengthFormatterUnit : lengthFormatterUnit)
    }
    unsafe fn lengthFormatterUnitFromUnit_(unit: HKUnit) -> NSLengthFormatterUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), lengthFormatterUnitFromUnit : unit)
    }
    unsafe fn unitFromEnergyFormatterUnit_(
        energyFormatterUnit: NSEnergyFormatterUnit,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), unitFromEnergyFormatterUnit : energyFormatterUnit)
    }
    unsafe fn energyFormatterUnitFromUnit_(unit: HKUnit) -> NSEnergyFormatterUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), energyFormatterUnitFromUnit : unit)
    }
}
pub type HKMetricPrefix = NSInteger;
impl HKUnit_Mass for HKUnit {}
pub trait HKUnit_Mass: Sized + std::ops::Deref {
    unsafe fn gramUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), gramUnitWithMetricPrefix : prefix)
    }
    unsafe fn gramUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), gramUnit)
    }
    unsafe fn ounceUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), ounceUnit)
    }
    unsafe fn poundUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), poundUnit)
    }
    unsafe fn stoneUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), stoneUnit)
    }
    unsafe fn moleUnitWithMetricPrefix_molarMass_(
        prefix: HKMetricPrefix,
        gramsPerMole: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), moleUnitWithMetricPrefix : prefix, molarMass : gramsPerMole)
    }
    unsafe fn moleUnitWithMolarMass_(gramsPerMole: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), moleUnitWithMolarMass : gramsPerMole)
    }
}
impl HKUnit_Length for HKUnit {}
pub trait HKUnit_Length: Sized + std::ops::Deref {
    unsafe fn meterUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), meterUnitWithMetricPrefix : prefix)
    }
    unsafe fn meterUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), meterUnit)
    }
    unsafe fn inchUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), inchUnit)
    }
    unsafe fn footUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), footUnit)
    }
    unsafe fn yardUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), yardUnit)
    }
    unsafe fn mileUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), mileUnit)
    }
}
impl HKUnit_Volume for HKUnit {}
pub trait HKUnit_Volume: Sized + std::ops::Deref {
    unsafe fn literUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), literUnitWithMetricPrefix : prefix)
    }
    unsafe fn literUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), literUnit)
    }
    unsafe fn fluidOunceUSUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), fluidOunceUSUnit)
    }
    unsafe fn fluidOunceImperialUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), fluidOunceImperialUnit)
    }
    unsafe fn pintUSUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), pintUSUnit)
    }
    unsafe fn pintImperialUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), pintImperialUnit)
    }
    unsafe fn cupUSUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), cupUSUnit)
    }
    unsafe fn cupImperialUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), cupImperialUnit)
    }
}
impl HKUnit_Pressure for HKUnit {}
pub trait HKUnit_Pressure: Sized + std::ops::Deref {
    unsafe fn pascalUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), pascalUnitWithMetricPrefix : prefix)
    }
    unsafe fn pascalUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), pascalUnit)
    }
    unsafe fn millimeterOfMercuryUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), millimeterOfMercuryUnit)
    }
    unsafe fn centimeterOfWaterUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), centimeterOfWaterUnit)
    }
    unsafe fn atmosphereUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), atmosphereUnit)
    }
    unsafe fn decibelAWeightedSoundPressureLevelUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), decibelAWeightedSoundPressureLevelUnit)
    }
    unsafe fn inchesOfMercuryUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), inchesOfMercuryUnit)
    }
}
impl HKUnit_Time for HKUnit {}
pub trait HKUnit_Time: Sized + std::ops::Deref {
    unsafe fn secondUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), secondUnitWithMetricPrefix : prefix)
    }
    unsafe fn secondUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), secondUnit)
    }
    unsafe fn minuteUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), minuteUnit)
    }
    unsafe fn hourUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), hourUnit)
    }
    unsafe fn dayUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), dayUnit)
    }
}
impl HKUnit_Energy for HKUnit {}
pub trait HKUnit_Energy: Sized + std::ops::Deref {
    unsafe fn jouleUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), jouleUnitWithMetricPrefix : prefix)
    }
    unsafe fn jouleUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), jouleUnit)
    }
    unsafe fn kilocalorieUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), kilocalorieUnit)
    }
    unsafe fn smallCalorieUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), smallCalorieUnit)
    }
    unsafe fn largeCalorieUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), largeCalorieUnit)
    }
    unsafe fn calorieUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), calorieUnit)
    }
}
impl HKUnit_Temperature for HKUnit {}
pub trait HKUnit_Temperature: Sized + std::ops::Deref {
    unsafe fn degreeCelsiusUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), degreeCelsiusUnit)
    }
    unsafe fn degreeFahrenheitUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), degreeFahrenheitUnit)
    }
    unsafe fn kelvinUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), kelvinUnit)
    }
}
impl HKUnit_Conductance for HKUnit {}
pub trait HKUnit_Conductance: Sized + std::ops::Deref {
    unsafe fn siemenUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), siemenUnitWithMetricPrefix : prefix)
    }
    unsafe fn siemenUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), siemenUnit)
    }
}
impl HKUnit_Pharmacology for HKUnit {}
pub trait HKUnit_Pharmacology: Sized + std::ops::Deref {
    unsafe fn internationalUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), internationalUnit)
    }
}
impl HKUnit_Scalar for HKUnit {}
pub trait HKUnit_Scalar: Sized + std::ops::Deref {
    unsafe fn countUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), countUnit)
    }
    unsafe fn percentUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), percentUnit)
    }
}
impl HKUnit_HearingSensitivity for HKUnit {}
pub trait HKUnit_HearingSensitivity: Sized + std::ops::Deref {
    unsafe fn decibelHearingLevelUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), decibelHearingLevelUnit)
    }
}
impl HKUnit_Math for HKUnit {}
pub trait HKUnit_Math: Sized + std::ops::Deref {
    unsafe fn unitMultipliedByUnit_(&self, unit: HKUnit) -> HKUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unitMultipliedByUnit : unit)
    }
    unsafe fn unitDividedByUnit_(&self, unit: HKUnit) -> HKUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unitDividedByUnit : unit)
    }
    unsafe fn unitRaisedToPower_(&self, power: NSInteger) -> HKUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unitRaisedToPower : power)
    }
    unsafe fn reciprocalUnit(&self) -> HKUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reciprocalUnit)
    }
}
impl HKUnit_Frequency for HKUnit {}
pub trait HKUnit_Frequency: Sized + std::ops::Deref {
    unsafe fn hertzUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), hertzUnitWithMetricPrefix : prefix)
    }
    unsafe fn hertzUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), hertzUnit)
    }
}
impl HKUnit_ElectricPotentialDifference for HKUnit {}
pub trait HKUnit_ElectricPotentialDifference: Sized + std::ops::Deref {
    unsafe fn voltUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), voltUnitWithMetricPrefix : prefix)
    }
    unsafe fn voltUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), voltUnit)
    }
}
impl HKUnit_Power for HKUnit {}
pub trait HKUnit_Power: Sized + std::ops::Deref {
    unsafe fn wattUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), wattUnitWithMetricPrefix : prefix)
    }
    unsafe fn wattUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), wattUnit)
    }
}
impl HKUnit_OpticalPower for HKUnit {}
pub trait HKUnit_OpticalPower: Sized + std::ops::Deref {
    unsafe fn diopterUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), diopterUnit)
    }
    unsafe fn prismDiopterUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), prismDiopterUnit)
    }
}
impl HKUnit_Angle for HKUnit {}
pub trait HKUnit_Angle: Sized + std::ops::Deref {
    unsafe fn radianAngleUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), radianAngleUnitWithMetricPrefix : prefix)
    }
    unsafe fn radianAngleUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), radianAngleUnit)
    }
    unsafe fn degreeAngleUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), degreeAngleUnit)
    }
}
impl HKUnit_Illuminance for HKUnit {}
pub trait HKUnit_Illuminance: Sized + std::ops::Deref {
    unsafe fn luxUnitWithMetricPrefix_(prefix: HKMetricPrefix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), luxUnitWithMetricPrefix : prefix)
    }
    unsafe fn luxUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), luxUnit)
    }
}
impl HKUnit_UnitLess for HKUnit {}
pub trait HKUnit_UnitLess: Sized + std::ops::Deref {
    unsafe fn appleEffortScoreUnit() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKUnit").unwrap(), appleEffortScoreUnit)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKUserAnnotatedMedication(pub id);
impl std::ops::Deref for HKUserAnnotatedMedication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKUserAnnotatedMedication {}
impl HKUserAnnotatedMedication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKUserAnnotatedMedication").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKUserAnnotatedMedication {}
impl PNSCopying for HKUserAnnotatedMedication {}
impl INSObject for HKUserAnnotatedMedication {}
impl PNSObject for HKUserAnnotatedMedication {}
impl std::convert::TryFrom<NSObject> for HKUserAnnotatedMedication {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKUserAnnotatedMedication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKUserAnnotatedMedication").unwrap()) };
        if is_kind_of {
            Ok(HKUserAnnotatedMedication(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKUserAnnotatedMedication")
        }
    }
}
impl IHKUserAnnotatedMedication for HKUserAnnotatedMedication {}
pub trait IHKUserAnnotatedMedication: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn nickname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nickname)
    }
    unsafe fn isArchived(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isArchived)
    }
    unsafe fn hasSchedule(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasSchedule)
    }
    unsafe fn medication(&self) -> HKMedicationConcept
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, medication)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKUserAnnotatedMedicationQuery(pub id);
impl std::ops::Deref for HKUserAnnotatedMedicationQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKUserAnnotatedMedicationQuery {}
impl HKUserAnnotatedMedicationQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKUserAnnotatedMedicationQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKUserAnnotatedMedicationQuery {}
impl std::convert::TryFrom<HKQuery> for HKUserAnnotatedMedicationQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKUserAnnotatedMedicationQuery, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKUserAnnotatedMedicationQuery").unwrap())
        };
        if is_kind_of {
            Ok(HKUserAnnotatedMedicationQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKUserAnnotatedMedicationQuery")
        }
    }
}
impl INSObject for HKUserAnnotatedMedicationQuery {}
impl PNSObject for HKUserAnnotatedMedicationQuery {}
impl IHKUserAnnotatedMedicationQuery for HKUserAnnotatedMedicationQuery {}
pub trait IHKUserAnnotatedMedicationQuery: Sized + std::ops::Deref {
    unsafe fn initWithPredicate_limit_resultsHandler_(
        &self,
        predicate: NSPredicate,
        limit: NSUInteger,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPredicate : predicate, limit : limit, resultsHandler : resultsHandler)
    }
}
pub type HKVerifiableClinicalRecordSourceType = NSString;
pub type HKVerifiableClinicalRecordCredentialType = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKVerifiableClinicalRecord(pub id);
impl std::ops::Deref for HKVerifiableClinicalRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKVerifiableClinicalRecord {}
impl HKVerifiableClinicalRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecord").unwrap(), alloc) })
    }
}
impl IHKSample for HKVerifiableClinicalRecord {}
impl std::convert::TryFrom<HKSample> for HKVerifiableClinicalRecord {
    type Error = &'static str;
    fn try_from(parent: HKSample) -> Result<HKVerifiableClinicalRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecord").unwrap()) };
        if is_kind_of {
            Ok(HKVerifiableClinicalRecord(parent.0))
        } else {
            Err("This HKSample cannot be downcasted to HKVerifiableClinicalRecord")
        }
    }
}
impl IHKObject for HKVerifiableClinicalRecord {}
impl PNSSecureCoding for HKVerifiableClinicalRecord {}
impl INSObject for HKVerifiableClinicalRecord {}
impl PNSObject for HKVerifiableClinicalRecord {}
impl IHKVerifiableClinicalRecord for HKVerifiableClinicalRecord {}
pub trait IHKVerifiableClinicalRecord: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn recordTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordTypes)
    }
    unsafe fn issuerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issuerIdentifier)
    }
    unsafe fn subject(&self) -> HKVerifiableClinicalRecordSubject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subject)
    }
    unsafe fn issuedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issuedDate)
    }
    unsafe fn relevantDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relevantDate)
    }
    unsafe fn expirationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expirationDate)
    }
    unsafe fn itemNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemNames)
    }
    unsafe fn sourceType(&self) -> HKVerifiableClinicalRecordSourceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceType)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
    unsafe fn JWSRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JWSRepresentation)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecord").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKVerifiableClinicalRecordQuery(pub id);
impl std::ops::Deref for HKVerifiableClinicalRecordQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKVerifiableClinicalRecordQuery {}
impl HKVerifiableClinicalRecordQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecordQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKVerifiableClinicalRecordQuery {}
impl std::convert::TryFrom<HKQuery> for HKVerifiableClinicalRecordQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKVerifiableClinicalRecordQuery, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecordQuery").unwrap())
        };
        if is_kind_of {
            Ok(HKVerifiableClinicalRecordQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKVerifiableClinicalRecordQuery")
        }
    }
}
impl INSObject for HKVerifiableClinicalRecordQuery {}
impl PNSObject for HKVerifiableClinicalRecordQuery {}
impl IHKVerifiableClinicalRecordQuery for HKVerifiableClinicalRecordQuery {}
pub trait IHKVerifiableClinicalRecordQuery: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordTypes_predicate_resultsHandler_(
        &self,
        recordTypes: NSArray,
        predicate: NSPredicate,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordTypes : recordTypes, predicate : predicate, resultsHandler : resultsHandler)
    }
    unsafe fn initWithRecordTypes_sourceTypes_predicate_resultsHandler_(
        &self,
        recordTypes: NSArray,
        sourceTypes: NSArray,
        predicate: NSPredicate,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordTypes : recordTypes, sourceTypes : sourceTypes, predicate : predicate, resultsHandler : resultsHandler)
    }
    unsafe fn recordTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordTypes)
    }
    unsafe fn sourceTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceTypes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecordQuery").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKVerifiableClinicalRecordSubject(pub id);
impl std::ops::Deref for HKVerifiableClinicalRecordSubject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKVerifiableClinicalRecordSubject {}
impl HKVerifiableClinicalRecordSubject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecordSubject").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKVerifiableClinicalRecordSubject {}
impl PNSCopying for HKVerifiableClinicalRecordSubject {}
impl INSObject for HKVerifiableClinicalRecordSubject {}
impl PNSObject for HKVerifiableClinicalRecordSubject {}
impl std::convert::TryFrom<NSObject> for HKVerifiableClinicalRecordSubject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKVerifiableClinicalRecordSubject, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecordSubject").unwrap())
        };
        if is_kind_of {
            Ok(HKVerifiableClinicalRecordSubject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKVerifiableClinicalRecordSubject")
        }
    }
}
impl IHKVerifiableClinicalRecordSubject for HKVerifiableClinicalRecordSubject {}
pub trait IHKVerifiableClinicalRecordSubject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fullName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullName)
    }
    unsafe fn dateOfBirthComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateOfBirthComponents)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKVerifiableClinicalRecordSubject").unwrap(), new)
    }
}
pub type HKPrismBase = NSInteger;
pub type HKVisionEye = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKVisionPrism(pub id);
impl std::ops::Deref for HKVisionPrism {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKVisionPrism {}
impl HKVisionPrism {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKVisionPrism").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKVisionPrism {}
impl PNSCopying for HKVisionPrism {}
impl INSObject for HKVisionPrism {}
impl PNSObject for HKVisionPrism {}
impl std::convert::TryFrom<NSObject> for HKVisionPrism {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKVisionPrism, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKVisionPrism").unwrap()) };
        if is_kind_of {
            Ok(HKVisionPrism(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKVisionPrism")
        }
    }
}
impl IHKVisionPrism for HKVisionPrism {}
pub trait IHKVisionPrism: Sized + std::ops::Deref {
    unsafe fn initWithAmount_angle_eye_(
        &self,
        amount: HKQuantity,
        angle: HKQuantity,
        eye: HKVisionEye,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAmount : amount, angle : angle, eye : eye)
    }
    unsafe fn initWithVerticalAmount_verticalBase_horizontalAmount_horizontalBase_eye_(
        &self,
        verticalAmount: HKQuantity,
        verticalBase: HKPrismBase,
        horizontalAmount: HKQuantity,
        horizontalBase: HKPrismBase,
        eye: HKVisionEye,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVerticalAmount : verticalAmount, verticalBase : verticalBase, horizontalAmount : horizontalAmount, horizontalBase : horizontalBase, eye : eye)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn amount(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn angle(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn verticalAmount(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalAmount)
    }
    unsafe fn horizontalAmount(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalAmount)
    }
    unsafe fn verticalBase(&self) -> HKPrismBase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalBase)
    }
    unsafe fn horizontalBase(&self) -> HKPrismBase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalBase)
    }
    unsafe fn eye(&self) -> HKVisionEye
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eye)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKVisionPrism").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutActivity(pub id);
impl std::ops::Deref for HKWorkoutActivity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutActivity {}
impl HKWorkoutActivity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutActivity").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKWorkoutActivity {}
impl PNSCopying for HKWorkoutActivity {}
impl INSObject for HKWorkoutActivity {}
impl PNSObject for HKWorkoutActivity {}
impl std::convert::TryFrom<NSObject> for HKWorkoutActivity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKWorkoutActivity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutActivity").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutActivity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKWorkoutActivity")
        }
    }
}
impl IHKWorkoutActivity for HKWorkoutActivity {}
pub trait IHKWorkoutActivity: Sized + std::ops::Deref {
    unsafe fn statisticsForType_(&self, quantityType: HKQuantityType) -> HKStatistics
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, statisticsForType : quantityType)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithWorkoutConfiguration_startDate_endDate_metadata_(
        &self,
        workoutConfiguration: HKWorkoutConfiguration,
        startDate: NSDate,
        endDate: NSDate,
        metadata: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWorkoutConfiguration : workoutConfiguration, startDate : startDate, endDate : endDate, metadata : metadata)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn workoutConfiguration(&self) -> HKWorkoutConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutConfiguration)
    }
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
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn workoutEvents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutEvents)
    }
    unsafe fn allStatistics(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allStatistics)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutActivity").unwrap(), new)
    }
}
pub type HKWorkoutSessionLocationType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutConfiguration(pub id);
impl std::ops::Deref for HKWorkoutConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutConfiguration {}
impl HKWorkoutConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for HKWorkoutConfiguration {}
impl PNSSecureCoding for HKWorkoutConfiguration {}
impl INSObject for HKWorkoutConfiguration {}
impl PNSObject for HKWorkoutConfiguration {}
impl std::convert::TryFrom<NSObject> for HKWorkoutConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKWorkoutConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutConfiguration").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKWorkoutConfiguration")
        }
    }
}
impl IHKWorkoutConfiguration for HKWorkoutConfiguration {}
pub trait IHKWorkoutConfiguration: Sized + std::ops::Deref {
    unsafe fn activityType(&self) -> HKWorkoutActivityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityType)
    }
    unsafe fn setActivityType_(&self, activityType: HKWorkoutActivityType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivityType : activityType)
    }
    unsafe fn locationType(&self) -> HKWorkoutSessionLocationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationType)
    }
    unsafe fn setLocationType_(&self, locationType: HKWorkoutSessionLocationType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocationType : locationType)
    }
    unsafe fn swimmingLocationType(&self) -> HKWorkoutSwimmingLocationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, swimmingLocationType)
    }
    unsafe fn setSwimmingLocationType_(&self, swimmingLocationType: HKWorkoutSwimmingLocationType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSwimmingLocationType : swimmingLocationType)
    }
    unsafe fn lapLength(&self) -> HKQuantity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lapLength)
    }
    unsafe fn setLapLength_(&self, lapLength: HKQuantity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLapLength : lapLength)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutEffortRelationship(pub id);
impl std::ops::Deref for HKWorkoutEffortRelationship {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutEffortRelationship {}
impl HKWorkoutEffortRelationship {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutEffortRelationship").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKWorkoutEffortRelationship {}
impl PNSCopying for HKWorkoutEffortRelationship {}
impl INSObject for HKWorkoutEffortRelationship {}
impl PNSObject for HKWorkoutEffortRelationship {}
impl std::convert::TryFrom<NSObject> for HKWorkoutEffortRelationship {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKWorkoutEffortRelationship, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutEffortRelationship").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutEffortRelationship(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKWorkoutEffortRelationship")
        }
    }
}
impl IHKWorkoutEffortRelationship for HKWorkoutEffortRelationship {}
pub trait IHKWorkoutEffortRelationship: Sized + std::ops::Deref {
    unsafe fn workout(&self) -> HKWorkout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workout)
    }
    unsafe fn activity(&self) -> HKWorkoutActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activity)
    }
    unsafe fn samples(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, samples)
    }
}
pub type HKWorkoutEffortRelationshipQueryOptions = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutEffortRelationshipQuery(pub id);
impl std::ops::Deref for HKWorkoutEffortRelationshipQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutEffortRelationshipQuery {}
impl HKWorkoutEffortRelationshipQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutEffortRelationshipQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKWorkoutEffortRelationshipQuery {}
impl std::convert::TryFrom<HKQuery> for HKWorkoutEffortRelationshipQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKWorkoutEffortRelationshipQuery, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutEffortRelationshipQuery").unwrap())
        };
        if is_kind_of {
            Ok(HKWorkoutEffortRelationshipQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKWorkoutEffortRelationshipQuery")
        }
    }
}
impl INSObject for HKWorkoutEffortRelationshipQuery {}
impl PNSObject for HKWorkoutEffortRelationshipQuery {}
impl IHKWorkoutEffortRelationshipQuery for HKWorkoutEffortRelationshipQuery {}
pub trait IHKWorkoutEffortRelationshipQuery: Sized + std::ops::Deref {
    unsafe fn initWithPredicate_anchor_options_resultsHandler_(
        &self,
        predicate: NSPredicate,
        anchor: HKQueryAnchor,
        options: HKWorkoutEffortRelationshipQueryOptions,
        resultsHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPredicate : predicate, anchor : anchor, options : options, resultsHandler : resultsHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutRoute(pub id);
impl std::ops::Deref for HKWorkoutRoute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutRoute {}
impl HKWorkoutRoute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutRoute").unwrap(), alloc) })
    }
}
impl IHKSeriesSample for HKWorkoutRoute {}
impl std::convert::TryFrom<HKSeriesSample> for HKWorkoutRoute {
    type Error = &'static str;
    fn try_from(parent: HKSeriesSample) -> Result<HKWorkoutRoute, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutRoute").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutRoute(parent.0))
        } else {
            Err("This HKSeriesSample cannot be downcasted to HKWorkoutRoute")
        }
    }
}
impl IHKSample for HKWorkoutRoute {}
impl IHKObject for HKWorkoutRoute {}
impl PNSSecureCoding for HKWorkoutRoute {}
impl INSObject for HKWorkoutRoute {}
impl PNSObject for HKWorkoutRoute {}
impl IHKWorkoutRoute for HKWorkoutRoute {}
pub trait IHKWorkoutRoute: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutRouteBuilder(pub id);
impl std::ops::Deref for HKWorkoutRouteBuilder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutRouteBuilder {}
impl HKWorkoutRouteBuilder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutRouteBuilder").unwrap(), alloc) })
    }
}
impl IHKSeriesBuilder for HKWorkoutRouteBuilder {}
impl std::convert::TryFrom<HKSeriesBuilder> for HKWorkoutRouteBuilder {
    type Error = &'static str;
    fn try_from(parent: HKSeriesBuilder) -> Result<HKWorkoutRouteBuilder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutRouteBuilder").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutRouteBuilder(parent.0))
        } else {
            Err("This HKSeriesBuilder cannot be downcasted to HKWorkoutRouteBuilder")
        }
    }
}
impl INSObject for HKWorkoutRouteBuilder {}
impl PNSObject for HKWorkoutRouteBuilder {}
impl IHKWorkoutRouteBuilder for HKWorkoutRouteBuilder {}
pub trait IHKWorkoutRouteBuilder: Sized + std::ops::Deref {
    unsafe fn initWithHealthStore_device_(
        &self,
        healthStore: HKHealthStore,
        device: HKDevice,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHealthStore : healthStore, device : device)
    }
    unsafe fn insertRouteData_completion_(
        &self,
        routeData: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertRouteData : routeData, completion : completion)
    }
    unsafe fn addMetadata_completion_(
        &self,
        metadata: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMetadata : metadata, completion : completion)
    }
    unsafe fn finishRouteWithWorkout_metadata_completion_(
        &self,
        workout: HKWorkout,
        metadata: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishRouteWithWorkout : workout, metadata : metadata, completion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutRouteQuery(pub id);
impl std::ops::Deref for HKWorkoutRouteQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutRouteQuery {}
impl HKWorkoutRouteQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutRouteQuery").unwrap(), alloc) })
    }
}
impl IHKQuery for HKWorkoutRouteQuery {}
impl std::convert::TryFrom<HKQuery> for HKWorkoutRouteQuery {
    type Error = &'static str;
    fn try_from(parent: HKQuery) -> Result<HKWorkoutRouteQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutRouteQuery").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutRouteQuery(parent.0))
        } else {
            Err("This HKQuery cannot be downcasted to HKWorkoutRouteQuery")
        }
    }
}
impl INSObject for HKWorkoutRouteQuery {}
impl PNSObject for HKWorkoutRouteQuery {}
impl IHKWorkoutRouteQuery for HKWorkoutRouteQuery {}
pub trait IHKWorkoutRouteQuery: Sized + std::ops::Deref {
    unsafe fn initWithRoute_dataHandler_(
        &self,
        workoutRoute: HKWorkoutRoute,
        dataHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRoute : workoutRoute, dataHandler : dataHandler)
    }
    unsafe fn initWithRoute_dateInterval_dataHandler_(
        &self,
        workoutRoute: HKWorkoutRoute,
        dateInterval: NSDateInterval,
        dataHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRoute : workoutRoute, dateInterval : dateInterval, dataHandler : dataHandler)
    }
}
pub type HKWorkoutSessionState = NSInteger;
pub type HKWorkoutSessionType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKWorkoutSession(pub id);
impl std::ops::Deref for HKWorkoutSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKWorkoutSession {}
impl HKWorkoutSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKWorkoutSession").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for HKWorkoutSession {}
impl INSObject for HKWorkoutSession {}
impl PNSObject for HKWorkoutSession {}
impl std::convert::TryFrom<NSObject> for HKWorkoutSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HKWorkoutSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKWorkoutSession").unwrap()) };
        if is_kind_of {
            Ok(HKWorkoutSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HKWorkoutSession")
        }
    }
}
impl IHKWorkoutSession for HKWorkoutSession {}
pub trait IHKWorkoutSession: Sized + std::ops::Deref {
    unsafe fn initWithActivityType_locationType_(
        &self,
        activityType: HKWorkoutActivityType,
        locationType: HKWorkoutSessionLocationType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithActivityType : activityType, locationType : locationType)
    }
    unsafe fn initWithConfiguration_error_(
        &self,
        workoutConfiguration: HKWorkoutConfiguration,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : workoutConfiguration, error : error)
    }
    unsafe fn initWithHealthStore_configuration_error_(
        &self,
        healthStore: HKHealthStore,
        workoutConfiguration: HKWorkoutConfiguration,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHealthStore : healthStore, configuration : workoutConfiguration, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn prepare(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepare)
    }
    unsafe fn startActivityWithDate_(&self, date: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startActivityWithDate : date)
    }
    unsafe fn stopActivityWithDate_(&self, date: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopActivityWithDate : date)
    }
    unsafe fn end(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, end)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn resume(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resume)
    }
    unsafe fn associatedWorkoutBuilder(&self) -> HKLiveWorkoutBuilder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, associatedWorkoutBuilder)
    }
    unsafe fn beginNewActivityWithConfiguration_date_metadata_(
        &self,
        workoutConfiguration: HKWorkoutConfiguration,
        date: NSDate,
        metadata: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginNewActivityWithConfiguration : workoutConfiguration, date : date, metadata : metadata)
    }
    unsafe fn endCurrentActivityOnDate_(&self, date: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endCurrentActivityOnDate : date)
    }
    unsafe fn startMirroringToCompanionDeviceWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startMirroringToCompanionDeviceWithCompletion : completion)
    }
    unsafe fn stopMirroringToCompanionDeviceWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopMirroringToCompanionDeviceWithCompletion : completion)
    }
    unsafe fn sendDataToRemoteWorkoutSession_completion_(
        &self,
        data: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDataToRemoteWorkoutSession : data, completion : completion)
    }
    unsafe fn activityType(&self) -> HKWorkoutActivityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityType)
    }
    unsafe fn locationType(&self) -> HKWorkoutSessionLocationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationType)
    }
    unsafe fn workoutConfiguration(&self) -> HKWorkoutConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workoutConfiguration)
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
    unsafe fn state(&self) -> HKWorkoutSessionState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn type_(&self) -> HKWorkoutSessionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
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
    unsafe fn currentActivity(&self) -> HKWorkoutActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentActivity)
    }
}
pub trait PHKWorkoutSessionDelegate: Sized + std::ops::Deref {
    unsafe fn workoutSession_didChangeToState_fromState_date_(
        &self,
        workoutSession: HKWorkoutSession,
        toState: HKWorkoutSessionState,
        fromState: HKWorkoutSessionState,
        date: NSDate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutSession : workoutSession, didChangeToState : toState, fromState : fromState, date : date)
    }
    unsafe fn workoutSession_didFailWithError_(
        &self,
        workoutSession: HKWorkoutSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutSession : workoutSession, didFailWithError : error)
    }
    unsafe fn workoutSession_didGenerateEvent_(
        &self,
        workoutSession: HKWorkoutSession,
        event: HKWorkoutEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutSession : workoutSession, didGenerateEvent : event)
    }
    unsafe fn workoutSession_didBeginActivityWithConfiguration_date_(
        &self,
        workoutSession: HKWorkoutSession,
        workoutConfiguration: HKWorkoutConfiguration,
        date: NSDate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutSession : workoutSession, didBeginActivityWithConfiguration : workoutConfiguration, date : date)
    }
    unsafe fn workoutSession_didEndActivityWithConfiguration_date_(
        &self,
        workoutSession: HKWorkoutSession,
        workoutConfiguration: HKWorkoutConfiguration,
        date: NSDate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutSession : workoutSession, didEndActivityWithConfiguration : workoutConfiguration, date : date)
    }
    unsafe fn workoutSession_didReceiveDataFromRemoteWorkoutSession_(
        &self,
        workoutSession: HKWorkoutSession,
        data: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutSession : workoutSession, didReceiveDataFromRemoteWorkoutSession : data)
    }
    unsafe fn workoutSession_didDisconnectFromRemoteDeviceWithError_(
        &self,
        workoutSession: HKWorkoutSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workoutSession : workoutSession, didDisconnectFromRemoteDeviceWithError : error)
    }
}
unsafe extern "C" {
    pub static HKErrorDomain: NSString;
}
unsafe extern "C" {
    pub fn HKCategoryValueSleepAnalysisAsleepValues() -> NSSet;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathDateComponents: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathUUID: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathSource: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathMetadata: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathCorrelation: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkout: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathDevice: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathSourceRevision: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutEffortRelationship: NSString;
}
unsafe extern "C" {
    pub static HKSampleSortIdentifierStartDate: NSString;
}
unsafe extern "C" {
    pub static HKSampleSortIdentifierEndDate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathStartDate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathEndDate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathAverageHeartRate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathECGClassification: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathECGSymptomsStatus: NSString;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeAllergyIntolerance: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeCondition: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeCoverage: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeDiagnosticReport: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeDocumentReference: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeImmunization: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeMedicationDispense: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeMedicationOrder: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeMedicationRequest: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeMedicationStatement: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeObservation: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKFHIRResourceTypeProcedure: HKFHIRResourceType;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathStatus: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathLogOrigin: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathScheduledDate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathMedicationConceptIdentifier: NSString;
}
unsafe extern "C" {
    pub fn HKStateOfMindValenceClassificationForValence(valence: f64) -> NSNumber;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutDuration: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutTotalDistance: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutTotalEnergyBurned: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutType: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutTotalSwimmingStrokeCount: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutTotalFlightsClimbed: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutSumQuantity: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutMinimumQuantity: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutMaximumQuantity: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutAverageQuantity: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivity: NSString;
}
unsafe extern "C" {
    pub static HKWorkoutSortIdentifierDuration: NSString;
}
unsafe extern "C" {
    pub static HKWorkoutSortIdentifierTotalDistance: NSString;
}
unsafe extern "C" {
    pub static HKWorkoutSortIdentifierTotalEnergyBurned: NSString;
}
unsafe extern "C" {
    pub static HKWorkoutSortIdentifierTotalSwimmingStrokeCount: NSString;
}
unsafe extern "C" {
    pub static HKWorkoutSortIdentifierTotalFlightsClimbed: NSString;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierAppleSleepingWristTemperature: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBodyFatPercentage: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBodyMass: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBodyMassIndex: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierElectrodermalActivity: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierHeight: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierLeanBodyMass: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierWaistCircumference: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierActiveEnergyBurned: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierAppleExerciseTime: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierAppleMoveTime: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierAppleStandTime: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBasalEnergyBurned: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierCrossCountrySkiingSpeed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierCyclingCadence: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierCyclingFunctionalThresholdPower: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierCyclingPower: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierCyclingSpeed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistanceCrossCountrySkiing: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistanceCycling: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistanceDownhillSnowSports: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistancePaddleSports: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistanceRowing: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistanceSkatingSports: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistanceSwimming: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistanceWalkingRunning: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDistanceWheelchair: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierEstimatedWorkoutEffortScore: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierFlightsClimbed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierNikeFuel: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierPaddleSportsSpeed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierPhysicalEffort: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierPushCount: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierRowingSpeed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierRunningPower: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierRunningSpeed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierStepCount: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierSwimmingStrokeCount: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierUnderwaterDepth: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierWorkoutEffortScore: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierEnvironmentalAudioExposure: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierEnvironmentalSoundReduction: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierHeadphoneAudioExposure: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierAtrialFibrillationBurden: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierHeartRate: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierHeartRateRecoveryOneMinute: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierHeartRateVariabilitySDNN: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierPeripheralPerfusionIndex: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierRestingHeartRate: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierVO2Max: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierWalkingHeartRateAverage: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierAppleWalkingSteadiness: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierRunningGroundContactTime: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierRunningStrideLength: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierRunningVerticalOscillation: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierSixMinuteWalkTestDistance: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierStairAscentSpeed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierStairDescentSpeed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierWalkingAsymmetryPercentage: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierWalkingDoubleSupportPercentage: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierWalkingSpeed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierWalkingStepLength: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryBiotin: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryCaffeine: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryCalcium: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryCarbohydrates: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryChloride: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryCholesterol: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryChromium: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryCopper: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryEnergyConsumed: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryFatMonounsaturated: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryFatPolyunsaturated: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryFatSaturated: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryFatTotal: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryFiber: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryFolate: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryIodine: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryIron: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryMagnesium: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryManganese: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryMolybdenum: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryNiacin: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryPantothenicAcid: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryPhosphorus: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryPotassium: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryProtein: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryRiboflavin: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietarySelenium: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietarySodium: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietarySugar: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryThiamin: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryVitaminA: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryVitaminB12: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryVitaminB6: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryVitaminC: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryVitaminD: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryVitaminE: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryVitaminK: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryWater: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierDietaryZinc: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBloodAlcoholContent: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBloodPressureDiastolic: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBloodPressureSystolic: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierInsulinDelivery: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierNumberOfAlcoholicBeverages: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierNumberOfTimesFallen: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierTimeInDaylight: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierUVExposure: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierWaterTemperature: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBasalBodyTemperature: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierAppleSleepingBreathingDisturbances: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierForcedExpiratoryVolume1: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierForcedVitalCapacity: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierInhalerUsage: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierOxygenSaturation: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierPeakExpiratoryFlowRate: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierRespiratoryRate: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBloodGlucose: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKQuantityTypeIdentifierBodyTemperature: HKQuantityTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierAppleStandHour: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierEnvironmentalAudioExposureEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierHeadphoneAudioExposureEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierHighHeartRateEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierHypertensionEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierIrregularHeartRhythmEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierLowCardioFitnessEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierLowHeartRateEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierMindfulSession: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierAppleWalkingSteadinessEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierHandwashingEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierToothbrushingEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierBleedingAfterPregnancy: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierBleedingDuringPregnancy: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierCervicalMucusQuality: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierContraceptive: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierInfrequentMenstrualCycles: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierIntermenstrualBleeding: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierIrregularMenstrualCycles: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierLactation: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierMenstrualFlow: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierOvulationTestResult: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierPersistentIntermenstrualBleeding: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierPregnancy: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierPregnancyTestResult: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierProgesteroneTestResult: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierProlongedMenstrualPeriods: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierSexualActivity: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierSleepApneaEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierSleepAnalysis: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierAbdominalCramps: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierAcne: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierAppetiteChanges: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierBladderIncontinence: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierBloating: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierBreastPain: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierChestTightnessOrPain: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierChills: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierConstipation: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierCoughing: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierDiarrhea: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierDizziness: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierDrySkin: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierFainting: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierFatigue: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierFever: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierGeneralizedBodyAche: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierHairLoss: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierHeadache: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierHeartburn: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierHotFlashes: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierLossOfSmell: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierLossOfTaste: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierLowerBackPain: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierMemoryLapse: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierMoodChanges: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierNausea: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierNightSweats: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierPelvicPain: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierRapidPoundingOrFlutteringHeartbeat: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierRunnyNose: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierShortnessOfBreath: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierSinusCongestion: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierSkippedHeartbeat: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierSleepChanges: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierSoreThroat: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierVaginalDryness: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierVomiting: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierWheezing: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCharacteristicTypeIdentifierActivityMoveMode: HKCharacteristicTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCharacteristicTypeIdentifierBiologicalSex: HKCharacteristicTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCharacteristicTypeIdentifierBloodType: HKCharacteristicTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCharacteristicTypeIdentifierDateOfBirth: HKCharacteristicTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCharacteristicTypeIdentifierFitzpatrickSkinType: HKCharacteristicTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCharacteristicTypeIdentifierWheelchairUse: HKCharacteristicTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCorrelationTypeIdentifierBloodPressure: HKCorrelationTypeIdentifier;
}
unsafe extern "C" {
    pub static HKCorrelationTypeIdentifierFood: HKCorrelationTypeIdentifier;
}
unsafe extern "C" {
    pub static HKDocumentTypeIdentifierCDA: HKDocumentTypeIdentifier;
}
unsafe extern "C" {
    pub static HKScoredAssessmentTypeIdentifierGAD7: HKScoredAssessmentTypeIdentifier;
}
unsafe extern "C" {
    pub static HKScoredAssessmentTypeIdentifierPHQ9: HKScoredAssessmentTypeIdentifier;
}
unsafe extern "C" {
    pub static HKWorkoutTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKWorkoutRouteTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKDataTypeIdentifierHeartbeatSeries: NSString;
}
unsafe extern "C" {
    pub static HKVisionPrescriptionTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKDataTypeIdentifierStateOfMind: NSString;
}
unsafe extern "C" {
    pub static HKMedicationDoseEventTypeIdentifierMedicationDoseEvent: NSString;
}
unsafe extern "C" {
    pub static HKDataTypeIdentifierUserAnnotatedMedicationConcept: NSString;
}
unsafe extern "C" {
    pub static HKCategoryTypeIdentifierAudioExposureEvent: HKCategoryTypeIdentifier;
}
unsafe extern "C" {
    pub fn HKAppleSleepingBreathingDisturbancesClassificationForQuantity(
        value: HKQuantity,
    ) -> NSNumber;
}
unsafe extern "C" {
    pub fn HKAppleSleepingBreathingDisturbancesMinimumQuantityForClassification(
        classification: HKAppleSleepingBreathingDisturbancesClassification,
    ) -> HKQuantity;
}
unsafe extern "C" {
    pub fn HKAppleWalkingSteadinessClassificationForQuantity(
        value: HKQuantity,
        classificationOut: *mut HKAppleWalkingSteadinessClassification,
        errorOut: *mut NSError,
    ) -> BOOL;
}
unsafe extern "C" {
    pub fn HKAppleWalkingSteadinessMinimumQuantityForClassification(
        classification: HKAppleWalkingSteadinessClassification,
    ) -> HKQuantity;
}
unsafe extern "C" {
    pub fn HKAppleWalkingSteadinessMaximumQuantityForClassification(
        classification: HKAppleWalkingSteadinessClassification,
    ) -> HKQuantity;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathCategoryValue: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathCDATitle: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathCDAPatientName: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathCDAAuthorName: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathCDACustodianName: NSString;
}
unsafe extern "C" {
    pub static HKDetailedCDAValidationErrorKey: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathClinicalRecordFHIRResourceIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathClinicalRecordFHIRResourceType: NSString;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierAllergyRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierClinicalNoteRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierConditionRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierImmunizationRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierLabResultRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierMedicationRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierProcedureRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierVitalSignRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKClinicalTypeIdentifierCoverageRecord: HKClinicalTypeIdentifier;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathQuantity: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathCount: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathSum: NSString;
}
unsafe extern "C" {
    pub static HKDevicePropertyKeyName: NSString;
}
unsafe extern "C" {
    pub static HKDevicePropertyKeyManufacturer: NSString;
}
unsafe extern "C" {
    pub static HKDevicePropertyKeyModel: NSString;
}
unsafe extern "C" {
    pub static HKDevicePropertyKeyHardwareVersion: NSString;
}
unsafe extern "C" {
    pub static HKDevicePropertyKeyFirmwareVersion: NSString;
}
unsafe extern "C" {
    pub static HKDevicePropertyKeySoftwareVersion: NSString;
}
unsafe extern "C" {
    pub static HKDevicePropertyKeyLocalIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKDevicePropertyKeyUDIDeviceIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathMin: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathAverage: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathMax: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathMostRecent: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathMostRecentStartDate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathMostRecentEndDate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathMostRecentDuration: NSString;
}
unsafe extern "C" {
    pub static HKFHIRReleaseDSTU2: HKFHIRRelease;
}
unsafe extern "C" {
    pub static HKFHIRReleaseR4: HKFHIRRelease;
}
unsafe extern "C" {
    pub static HKFHIRReleaseUnknown: HKFHIRRelease;
}
unsafe extern "C" {
    pub fn HKMinimumScoreForGAD7AssessmentRisk(risk: HKGAD7AssessmentRisk) -> NSInteger;
}
unsafe extern "C" {
    pub fn HKMaximumScoreForGAD7AssessmentRisk(risk: HKGAD7AssessmentRisk) -> NSInteger;
}
unsafe extern "C" {
    pub static HKUserPreferencesDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static HKHealthConceptDomainMedication: HKHealthConceptDomain;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormCapsule: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormCream: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormDevice: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormDrops: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormFoam: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormGel: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormInhaler: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormInjection: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormLiquid: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormLotion: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormOintment: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormPatch: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormPowder: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormSpray: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormSuppository: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormTablet: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormTopical: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMedicationGeneralFormUnknown: HKMedicationGeneralForm;
}
unsafe extern "C" {
    pub static HKMetadataKeyDeviceSerialNumber: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyBodyTemperatureSensorLocation: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyHeartRateSensorLocation: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyHeartRateMotionContext: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyUserMotionContext: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeySessionEstimate: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyHeartRateRecoveryTestType: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyHeartRateRecoveryActivityType: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyHeartRateRecoveryActivityDuration: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyHeartRateRecoveryMaxObservedRecoveryHeartRate: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyFoodType: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyUDIDeviceIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyUDIProductionIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyDigitalSignature: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyExternalUUID: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeySyncIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeySyncVersion: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyTimeZone: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyDeviceName: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyDeviceManufacturerName: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyWasTakenInLab: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyReferenceRangeLowerLimit: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyReferenceRangeUpperLimit: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyWasUserEntered: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyWorkoutBrandName: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyGroupFitness: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAppleFitnessPlusCatalogIdentifier: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAppleFitnessPlusSession: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyIndoorWorkout: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyCoachedWorkout: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyWeatherCondition: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyWeatherTemperature: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyWeatherHumidity: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeySexualActivityProtectionUsed: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyMenstrualCycleStart: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyLapLength: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeySwimmingLocationType: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeySwimmingStrokeStyle: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyInsulinDeliveryReason: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyBloodGlucoseMealTime: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyVO2MaxTestType: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAverageSpeed: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyMaximumSpeed: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAlpineSlopeGrade: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyElevationAscended: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyElevationDescended: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyFitnessMachineDuration: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyIndoorBikeDistance: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyCrossTrainerDistance: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyHeartRateEventThreshold: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAverageMETs: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAudioExposureLevel: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAudioExposureDuration: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAppleECGAlgorithmVersion: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyDevicePlacementSide: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyBarometricPressure: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAppleDeviceCalibrated: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyVO2MaxValue: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyLowCardioFitnessEventThreshold: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyDateOfEarliestDataUsedForEstimate: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyAlgorithmVersion: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeySWOLFScore: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyQuantityClampedToLowerBound: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyQuantityClampedToUpperBound: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyGlassesPrescriptionDescription: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyWaterSalinity: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyHeadphoneGain: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyCyclingFunctionalThresholdPowerTestType: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyActivityType: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyPhysicalEffortEstimationType: NSString;
}
unsafe extern "C" {
    pub static HKMetadataKeyMaximumLightIntensity: NSString;
}
unsafe extern "C" {
    pub fn HKMinimumScoreForPHQ9AssessmentRisk(risk: HKPHQ9AssessmentRisk) -> NSInteger;
}
unsafe extern "C" {
    pub fn HKMaximumScoreForPHQ9AssessmentRisk(risk: HKPHQ9AssessmentRisk) -> NSInteger;
}
unsafe extern "C" {
    pub static HKSourceRevisionAnyVersion: NSString;
}
unsafe extern "C" {
    pub static HKSourceRevisionAnyProductType: NSString;
}
unsafe extern "C" {
    pub static HKSourceRevisionAnyOperatingSystem: NSOperatingSystemVersion;
}
unsafe extern "C" {
    pub static HKUserAnnotatedMedicationPredicateKeyPathIsArchived: NSString;
}
unsafe extern "C" {
    pub static HKUserAnnotatedMedicationPredicateKeyPathHasSchedule: NSString;
}
unsafe extern "C" {
    pub static HKVerifiableClinicalRecordSourceTypeSMARTHealthCard:
        HKVerifiableClinicalRecordSourceType;
}
unsafe extern "C" {
    pub static HKVerifiableClinicalRecordSourceTypeEUDigitalCOVIDCertificate:
        HKVerifiableClinicalRecordSourceType;
}
unsafe extern "C" {
    pub static HKVerifiableClinicalRecordCredentialTypeCOVID19:
        HKVerifiableClinicalRecordCredentialType;
}
unsafe extern "C" {
    pub static HKVerifiableClinicalRecordCredentialTypeImmunization:
        HKVerifiableClinicalRecordCredentialType;
}
unsafe extern "C" {
    pub static HKVerifiableClinicalRecordCredentialTypeLaboratory:
        HKVerifiableClinicalRecordCredentialType;
}
unsafe extern "C" {
    pub static HKVerifiableClinicalRecordCredentialTypeRecovery:
        HKVerifiableClinicalRecordCredentialType;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityType: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityDuration: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityStartDate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityEndDate: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivitySumQuantity: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityMinimumQuantity: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityMaximumQuantity: NSString;
}
unsafe extern "C" {
    pub static HKPredicateKeyPathWorkoutActivityAverageQuantity: NSString;
}

unsafe impl objc2::encode::RefEncode for HKActivitySummary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKActivitySummary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKElectrocardiogram {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKElectrocardiogram {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKFHIRResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKFHIRResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKMedicationDoseEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKMedicationDoseEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKStateOfMind {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKStateOfMind {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKActivitySummaryQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKActivitySummaryQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKQueryAnchor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKQueryAnchor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKObjectType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKObjectType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCharacteristicType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCharacteristicType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSampleType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSampleType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCategoryType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCategoryType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCorrelationType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCorrelationType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKDocumentType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKDocumentType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKQuantityType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKQuantityType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSeriesType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSeriesType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKActivitySummaryType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKActivitySummaryType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKAudiogramSampleType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKAudiogramSampleType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKElectrocardiogramType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKElectrocardiogramType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKMedicationDoseEventType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKMedicationDoseEventType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKPrescriptionType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKPrescriptionType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKScoredAssessmentType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKScoredAssessmentType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKStateOfMindType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKStateOfMindType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKUserAnnotatedMedicationType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKUserAnnotatedMedicationType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKQueryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKQueryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKAnchoredObjectQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKAnchoredObjectQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKAttachmentStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKAttachmentStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKQuantity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKQuantity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKAudiogramSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKAudiogramSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKAudiogramSensitivityPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKAudiogramSensitivityPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKAudiogramSensitivityPointClampingRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKAudiogramSensitivityPointClampingRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKAudiogramSensitivityTest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKAudiogramSensitivityTest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCategorySample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCategorySample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKDocumentSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKDocumentSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCDADocumentSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCDADocumentSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCDADocument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCDADocument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKBiologicalSexObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKBiologicalSexObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKBloodTypeObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKBloodTypeObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKFitzpatrickSkinTypeObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKFitzpatrickSkinTypeObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWheelchairUseObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWheelchairUseObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKActivityMoveModeObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKActivityMoveModeObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKClinicalCoding {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKClinicalCoding {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKClinicalRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKClinicalRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKClinicalType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKClinicalType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKLensSpecification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKLensSpecification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKContactsLensSpecification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKContactsLensSpecification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKVisionPrescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKVisionPrescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKContactsPrescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKContactsPrescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCorrelation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCorrelation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCorrelationQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCorrelationQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKQuantitySample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKQuantitySample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCumulativeQuantitySample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCumulativeQuantitySample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKCumulativeQuantitySeriesSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKCumulativeQuantitySeriesSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKDeletedObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKDeletedObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKDiscreteQuantitySample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKDiscreteQuantitySample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKDocumentQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKDocumentQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKElectrocardiogramVoltageMeasurement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKElectrocardiogramVoltageMeasurement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKElectrocardiogramQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKElectrocardiogramQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKFHIRVersion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKFHIRVersion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKScoredAssessment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKScoredAssessment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKGAD7Assessment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKGAD7Assessment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKGlassesLensSpecification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKGlassesLensSpecification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKGlassesPrescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKGlassesPrescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKHealthStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKHealthStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKHealthConceptIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKHealthConceptIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSeriesBuilder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSeriesBuilder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKHeartbeatSeriesBuilder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKHeartbeatSeriesBuilder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKHeartbeatSeriesQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKHeartbeatSeriesQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSeriesSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSeriesSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKHeartbeatSeriesSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKHeartbeatSeriesSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutBuilder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutBuilder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKLiveWorkoutDataSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKLiveWorkoutDataSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKLiveWorkoutBuilder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKLiveWorkoutBuilder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKMedicationConcept {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKMedicationConcept {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKObserverQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKObserverQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKPHQ9Assessment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKPHQ9Assessment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKQuantitySeriesSampleBuilder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKQuantitySeriesSampleBuilder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKQuantitySeriesSampleQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKQuantitySeriesSampleQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSampleQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSampleQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSourceQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSourceQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKSourceRevision {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKSourceRevision {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKStatistics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKStatistics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKStatisticsCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKStatisticsCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKStatisticsCollectionQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKStatisticsCollectionQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKStatisticsQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKStatisticsQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKUnit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKUnit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKUserAnnotatedMedication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKUserAnnotatedMedication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKUserAnnotatedMedicationQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKUserAnnotatedMedicationQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKVerifiableClinicalRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKVerifiableClinicalRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKVerifiableClinicalRecordQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKVerifiableClinicalRecordQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKVerifiableClinicalRecordSubject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKVerifiableClinicalRecordSubject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKVisionPrism {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKVisionPrism {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutActivity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutActivity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutEffortRelationship {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutEffortRelationship {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutEffortRelationshipQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutEffortRelationshipQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutRoute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutRoute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutRouteBuilder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutRouteBuilder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutRouteQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutRouteQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HKWorkoutSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKWorkoutSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
