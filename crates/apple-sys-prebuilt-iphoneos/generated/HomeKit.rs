#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreLocation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Matter::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type HMErrorBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessory(pub id);
impl std::ops::Deref for HMAccessory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessory {}
impl HMAccessory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessory").unwrap(), alloc) })
    }
}
impl INSObject for HMAccessory {}
impl PNSObject for HMAccessory {}
impl std::convert::TryFrom<NSObject> for HMAccessory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessory").unwrap()) };
        if is_kind_of {
            Ok(HMAccessory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessory")
        }
    }
}
impl IHMAccessory for HMAccessory {}
pub trait IHMAccessory: Sized + std::ops::Deref {
    unsafe fn updateName_completionHandler_(
        &self,
        name: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateName : name, completionHandler : completion)
    }
    unsafe fn identifyWithCompletionHandler_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, identifyWithCompletionHandler : completion)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn home(&self) -> HMHome
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, home)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn identifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
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
    unsafe fn isReachable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReachable)
    }
    unsafe fn isBridged(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBridged)
    }
    unsafe fn identifiersForBridgedAccessories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifiersForBridgedAccessories)
    }
    unsafe fn uniqueIdentifiersForBridgedAccessories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifiersForBridgedAccessories)
    }
    unsafe fn bridgedAccessories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bridgedAccessories)
    }
    unsafe fn category(&self) -> HMAccessoryCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
    unsafe fn room(&self) -> HMRoom
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, room)
    }
    unsafe fn services(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, services)
    }
    unsafe fn profiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profiles)
    }
    unsafe fn isBlocked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlocked)
    }
    unsafe fn model(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn manufacturer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturer)
    }
    unsafe fn firmwareVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firmwareVersion)
    }
    unsafe fn supportsIdentify(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsIdentify)
    }
    unsafe fn matterNodeID(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matterNodeID)
    }
}
pub trait PHMAccessoryDelegate: Sized + std::ops::Deref {
    unsafe fn accessoryDidUpdateName_(&self, accessory: HMAccessory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryDidUpdateName : accessory)
    }
    unsafe fn accessory_didUpdateNameForService_(&self, accessory: HMAccessory, service: HMService)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessory : accessory, didUpdateNameForService : service)
    }
    unsafe fn accessory_didUpdateAssociatedServiceTypeForService_(
        &self,
        accessory: HMAccessory,
        service: HMService,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessory : accessory, didUpdateAssociatedServiceTypeForService : service)
    }
    unsafe fn accessoryDidUpdateServices_(&self, accessory: HMAccessory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryDidUpdateServices : accessory)
    }
    unsafe fn accessory_didAddProfile_(&self, accessory: HMAccessory, profile: HMAccessoryProfile)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessory : accessory, didAddProfile : profile)
    }
    unsafe fn accessory_didRemoveProfile_(
        &self,
        accessory: HMAccessory,
        profile: HMAccessoryProfile,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessory : accessory, didRemoveProfile : profile)
    }
    unsafe fn accessoryDidUpdateReachability_(&self, accessory: HMAccessory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryDidUpdateReachability : accessory)
    }
    unsafe fn accessory_service_didUpdateValueForCharacteristic_(
        &self,
        accessory: HMAccessory,
        service: HMService,
        characteristic: HMCharacteristic,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessory : accessory, service : service, didUpdateValueForCharacteristic : characteristic)
    }
    unsafe fn accessory_didUpdateFirmwareVersion_(
        &self,
        accessory: HMAccessory,
        firmwareVersion: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessory : accessory, didUpdateFirmwareVersion : firmwareVersion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessoryBrowser(pub id);
impl std::ops::Deref for HMAccessoryBrowser {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessoryBrowser {}
impl HMAccessoryBrowser {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessoryBrowser").unwrap(), alloc) })
    }
}
impl INSObject for HMAccessoryBrowser {}
impl PNSObject for HMAccessoryBrowser {}
impl std::convert::TryFrom<NSObject> for HMAccessoryBrowser {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessoryBrowser, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessoryBrowser").unwrap()) };
        if is_kind_of {
            Ok(HMAccessoryBrowser(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessoryBrowser")
        }
    }
}
impl IHMAccessoryBrowser for HMAccessoryBrowser {}
pub trait IHMAccessoryBrowser: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn startSearchingForNewAccessories(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startSearchingForNewAccessories)
    }
    unsafe fn stopSearchingForNewAccessories(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopSearchingForNewAccessories)
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
    unsafe fn discoveredAccessories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveredAccessories)
    }
}
pub trait PHMAccessoryBrowserDelegate: Sized + std::ops::Deref {
    unsafe fn accessoryBrowser_didFindNewAccessory_(
        &self,
        browser: HMAccessoryBrowser,
        accessory: HMAccessory,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryBrowser : browser, didFindNewAccessory : accessory)
    }
    unsafe fn accessoryBrowser_didRemoveNewAccessory_(
        &self,
        browser: HMAccessoryBrowser,
        accessory: HMAccessory,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryBrowser : browser, didRemoveNewAccessory : accessory)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessoryCategory(pub id);
impl std::ops::Deref for HMAccessoryCategory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessoryCategory {}
impl HMAccessoryCategory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessoryCategory").unwrap(), alloc) })
    }
}
impl INSObject for HMAccessoryCategory {}
impl PNSObject for HMAccessoryCategory {}
impl std::convert::TryFrom<NSObject> for HMAccessoryCategory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessoryCategory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessoryCategory").unwrap()) };
        if is_kind_of {
            Ok(HMAccessoryCategory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessoryCategory")
        }
    }
}
impl IHMAccessoryCategory for HMAccessoryCategory {}
pub trait IHMAccessoryCategory: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn categoryType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryType)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAction(pub id);
impl std::ops::Deref for HMAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAction {}
impl HMAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAction").unwrap(), alloc) })
    }
}
impl INSObject for HMAction {}
impl PNSObject for HMAction {}
impl std::convert::TryFrom<NSObject> for HMAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAction, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAction").unwrap()) };
        if is_kind_of {
            Ok(HMAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAction")
        }
    }
}
impl IHMAction for HMAction {}
pub trait IHMAction: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMAction").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMActionSet(pub id);
impl std::ops::Deref for HMActionSet {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMActionSet {}
impl HMActionSet {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMActionSet").unwrap(), alloc) })
    }
}
impl INSObject for HMActionSet {}
impl PNSObject for HMActionSet {}
impl std::convert::TryFrom<NSObject> for HMActionSet {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMActionSet, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMActionSet").unwrap()) };
        if is_kind_of {
            Ok(HMActionSet(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMActionSet")
        }
    }
}
impl IHMActionSet for HMActionSet {}
pub trait IHMActionSet: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateName_completionHandler_(&self, name: NSString, completion: HMErrorBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateName : name, completionHandler : completion)
    }
    unsafe fn addAction_completionHandler_(&self, action: HMAction, completion: HMErrorBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAction : action, completionHandler : completion)
    }
    unsafe fn removeAction_completionHandler_(&self, action: HMAction, completion: HMErrorBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAction : action, completionHandler : completion)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn actions(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
    unsafe fn isExecuting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExecuting)
    }
    unsafe fn actionSetType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionSetType)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn lastExecutionDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastExecutionDate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCharacteristic(pub id);
impl std::ops::Deref for HMCharacteristic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCharacteristic {}
impl HMCharacteristic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCharacteristic").unwrap(), alloc) })
    }
}
impl INSObject for HMCharacteristic {}
impl PNSObject for HMCharacteristic {}
impl std::convert::TryFrom<NSObject> for HMCharacteristic {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMCharacteristic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCharacteristic").unwrap()) };
        if is_kind_of {
            Ok(HMCharacteristic(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMCharacteristic")
        }
    }
}
impl IHMCharacteristic for HMCharacteristic {}
pub trait IHMCharacteristic: Sized + std::ops::Deref {
    unsafe fn writeValue_completionHandler_(
        &self,
        value: id,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeValue : value, completionHandler : completion)
    }
    unsafe fn readValueWithCompletionHandler_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readValueWithCompletionHandler : completion)
    }
    unsafe fn enableNotification_completionHandler_(
        &self,
        enable: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enableNotification : enable, completionHandler : completion)
    }
    unsafe fn updateAuthorizationData_completionHandler_(
        &self,
        data: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAuthorizationData : data, completionHandler : completion)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn characteristicType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristicType)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn service(&self) -> HMService
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, service)
    }
    unsafe fn properties(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn metadata(&self) -> HMCharacteristicMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn isNotificationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNotificationEnabled)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
}
pub type HMCharacteristicValueLockMechanismLastKnownAction = NSInteger;
pub type HMCharacteristicValueAirParticulateSize = NSInteger;
pub type HMCharacteristicValueAirQuality = NSInteger;
pub type HMCharacteristicValuePositionState = NSInteger;
pub type HMCharacteristicValueCurrentSecuritySystemState = NSInteger;
pub type HMCharacteristicValueTargetSecuritySystemState = NSInteger;
pub type HMCharacteristicValueJammedStatus = NSInteger;
pub type HMCharacteristicValueTamperedStatus = NSInteger;
pub type HMCharacteristicValueLeakStatus = NSInteger;
pub type HMCharacteristicValueContactState = NSInteger;
pub type HMCharacteristicValueStatusFault = NSInteger;
pub type HMCharacteristicValueCarbonMonoxideDetectionStatus = NSInteger;
pub type HMCharacteristicValueCarbonDioxideDetectionStatus = NSInteger;
pub type HMCharacteristicValueOccupancyStatus = NSInteger;
pub type HMCharacteristicValueSecuritySystemAlarmType = NSInteger;
pub type HMCharacteristicValueCurrentAirPurifierState = NSInteger;
pub type HMCharacteristicValueTargetAirPurifierState = NSInteger;
pub type HMCharacteristicValueCurrentSlatState = NSInteger;
pub type HMCharacteristicValueSlatType = NSInteger;
pub type HMCharacteristicValueFilterChange = NSInteger;
pub type HMCharacteristicValueLabelNamespace = NSInteger;
pub type HMCharacteristicValueProgramMode = NSInteger;
pub type HMCharacteristicValueUsageState = NSInteger;
pub type HMCharacteristicValueValveType = NSInteger;
pub type HMCharacteristicValueVolumeControlType = NSInteger;
pub type HMCharacteristicValueVolumeSelector = NSInteger;
pub type HMCharacteristicValueDoorState = NSInteger;
pub type HMCharacteristicValueCurrentHeatingCooling = NSInteger;
pub type HMCharacteristicValueLockMechanismState = NSInteger;
pub type HMCharacteristicValueTargetLockMechanismState = NSInteger;
pub type HMCharacteristicValueRotationDirection = NSInteger;
pub type HMCharacteristicValueTargetDoorState = NSInteger;
pub type HMCharacteristicValueHeatingCooling = NSInteger;
pub type HMCharacteristicValueTemperatureUnit = NSInteger;
pub type HMCharacteristicValueInputEvent = NSInteger;
pub type HMCharacteristicValueSmokeDetectionStatus = NSInteger;
pub type HMCharacteristicValueBatteryStatus = NSInteger;
pub type HMCharacteristicValueChargingState = NSInteger;
pub type HMCharacteristicValueLockPhysicalControlsState = NSInteger;
pub type HMCharacteristicValueCurrentFanState = NSInteger;
pub type HMCharacteristicValueActivationState = NSInteger;
pub type HMCharacteristicValueCurrentHeaterCoolerState = NSInteger;
pub type HMCharacteristicValueTargetHeaterCoolerState = NSInteger;
pub type HMCharacteristicValueCurrentHumidifierDehumidifierState = NSInteger;
pub type HMCharacteristicValueTargetHumidifierDehumidifierState = NSInteger;
pub type HMCharacteristicValueSwingMode = NSInteger;
pub type HMCharacteristicValueTargetFanState = NSInteger;
pub type HMCharacteristicValueConfigurationState = NSInteger;
pub type HMCharacteristicValueInputSourceType = NSInteger;
pub type HMCharacteristicValueInputDeviceType = NSInteger;
pub type HMCharacteristicValueClosedCaptions = NSInteger;
pub type HMCharacteristicValuePowerModeSelection = NSInteger;
pub type HMCharacteristicValueCurrentMediaState = NSInteger;
pub type HMCharacteristicValueRemoteKey = NSInteger;
pub type HMCharacteristicValuePictureMode = NSInteger;
pub type HMCharacteristicValueTargetVisibilityState = NSInteger;
pub type HMCharacteristicValueCurrentVisibilityState = NSInteger;
pub type HMCharacteristicValueTargetMediaState = NSInteger;
pub type HMCharacteristicValueRouterStatus = NSInteger;
pub type HMCharacteristicValueWiFiSatelliteStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCharacteristicMetadata(pub id);
impl std::ops::Deref for HMCharacteristicMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCharacteristicMetadata {}
impl HMCharacteristicMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCharacteristicMetadata").unwrap(), alloc) })
    }
}
impl INSObject for HMCharacteristicMetadata {}
impl PNSObject for HMCharacteristicMetadata {}
impl std::convert::TryFrom<NSObject> for HMCharacteristicMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMCharacteristicMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCharacteristicMetadata").unwrap()) };
        if is_kind_of {
            Ok(HMCharacteristicMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMCharacteristicMetadata")
        }
    }
}
impl IHMCharacteristicMetadata for HMCharacteristicMetadata {}
pub trait IHMCharacteristicMetadata: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn minimumValue(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumValue)
    }
    unsafe fn maximumValue(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumValue)
    }
    unsafe fn stepValue(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stepValue)
    }
    unsafe fn maxLength(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxLength)
    }
    unsafe fn format(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn units(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, units)
    }
    unsafe fn manufacturerDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturerDescription)
    }
    unsafe fn validValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, validValues)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCharacteristicWriteAction(pub id);
impl std::ops::Deref for HMCharacteristicWriteAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCharacteristicWriteAction {}
impl HMCharacteristicWriteAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCharacteristicWriteAction").unwrap(), alloc) })
    }
}
impl IHMAction for HMCharacteristicWriteAction {}
impl From<HMCharacteristicWriteAction> for HMAction {
    fn from(child: HMCharacteristicWriteAction) -> HMAction {
        HMAction(child.0)
    }
}
impl std::convert::TryFrom<HMAction> for HMCharacteristicWriteAction {
    type Error = &'static str;
    fn try_from(parent: HMAction) -> Result<HMCharacteristicWriteAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCharacteristicWriteAction").unwrap()) };
        if is_kind_of {
            Ok(HMCharacteristicWriteAction(parent.0))
        } else {
            Err("This HMAction cannot be downcasted to HMCharacteristicWriteAction")
        }
    }
}
impl INSObject for HMCharacteristicWriteAction {}
impl PNSObject for HMCharacteristicWriteAction {}
impl<TargetValueType: 'static> IHMCharacteristicWriteAction<TargetValueType>
    for HMCharacteristicWriteAction
{
}
pub trait IHMCharacteristicWriteAction<TargetValueType: 'static>: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCharacteristic_targetValue_(
        &self,
        characteristic: HMCharacteristic,
        targetValue: id,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCharacteristic : characteristic, targetValue : targetValue)
    }
    unsafe fn updateTargetValue_completionHandler_(
        &self,
        targetValue: id,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTargetValue : targetValue, completionHandler : completion)
    }
    unsafe fn characteristic(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristic)
    }
    unsafe fn targetValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetValue)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMCharacteristicWriteAction").unwrap(), new)
    }
}
pub type HMHomeHubState = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMHome(pub id);
impl std::ops::Deref for HMHome {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMHome {}
impl HMHome {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMHome").unwrap(), alloc) })
    }
}
impl INSObject for HMHome {}
impl PNSObject for HMHome {}
impl std::convert::TryFrom<NSObject> for HMHome {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMHome, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMHome").unwrap()) };
        if is_kind_of {
            Ok(HMHome(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMHome")
        }
    }
}
impl IHMHome for HMHome {}
pub trait IHMHome: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateName_completionHandler_(
        &self,
        name: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateName : name, completionHandler : completion)
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
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn isPrimary(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPrimary)
    }
    unsafe fn homeHubState(&self) -> HMHomeHubState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, homeHubState)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
}
impl HMHome_HMAccessory for HMHome {}
pub trait HMHome_HMAccessory: Sized + std::ops::Deref {
    unsafe fn addAccessory_completionHandler_(
        &self,
        accessory: HMAccessory,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAccessory : accessory, completionHandler : completion)
    }
    unsafe fn removeAccessory_completionHandler_(
        &self,
        accessory: HMAccessory,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAccessory : accessory, completionHandler : completion)
    }
    unsafe fn assignAccessory_toRoom_completionHandler_(
        &self,
        accessory: HMAccessory,
        room: HMRoom,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assignAccessory : accessory, toRoom : room, completionHandler : completion)
    }
    unsafe fn servicesWithTypes_(&self, serviceTypes: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, servicesWithTypes : serviceTypes)
    }
    unsafe fn unblockAccessory_completionHandler_(
        &self,
        accessory: HMAccessory,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unblockAccessory : accessory, completionHandler : completion)
    }
    unsafe fn addAndSetupAccessoriesWithCompletionHandler_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAndSetupAccessoriesWithCompletionHandler : completion)
    }
    unsafe fn addAndSetupAccessoriesWithPayload_completionHandler_(
        &self,
        payload: HMAccessorySetupPayload,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAndSetupAccessoriesWithPayload : payload, completionHandler : completion)
    }
    unsafe fn accessories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessories)
    }
    unsafe fn supportsAddingNetworkRouter(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsAddingNetworkRouter)
    }
}
impl HMHome_HMUser for HMHome {}
pub trait HMHome_HMUser: Sized + std::ops::Deref {
    unsafe fn manageUsersWithCompletionHandler_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, manageUsersWithCompletionHandler : completion)
    }
    unsafe fn addUserWithCompletionHandler_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addUserWithCompletionHandler : completion)
    }
    unsafe fn removeUser_completionHandler_(
        &self,
        user: HMUser,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeUser : user, completionHandler : completion)
    }
    unsafe fn homeAccessControlForUser_(&self, user: HMUser) -> HMHomeAccessControl
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeAccessControlForUser : user)
    }
    unsafe fn currentUser(&self) -> HMUser
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentUser)
    }
    unsafe fn users(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, users)
    }
}
impl HMHome_HMRoom for HMHome {}
pub trait HMHome_HMRoom: Sized + std::ops::Deref {
    unsafe fn addRoomWithName_completionHandler_(
        &self,
        roomName: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRoomWithName : roomName, completionHandler : completion)
    }
    unsafe fn removeRoom_completionHandler_(
        &self,
        room: HMRoom,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRoom : room, completionHandler : completion)
    }
    unsafe fn roomForEntireHome(&self) -> HMRoom
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roomForEntireHome)
    }
    unsafe fn rooms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rooms)
    }
}
impl HMHome_HMZone for HMHome {}
pub trait HMHome_HMZone: Sized + std::ops::Deref {
    unsafe fn addZoneWithName_completionHandler_(
        &self,
        zoneName: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addZoneWithName : zoneName, completionHandler : completion)
    }
    unsafe fn removeZone_completionHandler_(
        &self,
        zone: HMZone,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeZone : zone, completionHandler : completion)
    }
    unsafe fn zones(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zones)
    }
}
impl HMHome_HMServiceGroup for HMHome {}
pub trait HMHome_HMServiceGroup: Sized + std::ops::Deref {
    unsafe fn addServiceGroupWithName_completionHandler_(
        &self,
        serviceGroupName: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addServiceGroupWithName : serviceGroupName, completionHandler : completion)
    }
    unsafe fn removeServiceGroup_completionHandler_(
        &self,
        group: HMServiceGroup,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeServiceGroup : group, completionHandler : completion)
    }
    unsafe fn serviceGroups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceGroups)
    }
}
impl HMHome_HMActionSet for HMHome {}
pub trait HMHome_HMActionSet: Sized + std::ops::Deref {
    unsafe fn addActionSetWithName_completionHandler_(
        &self,
        actionSetName: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addActionSetWithName : actionSetName, completionHandler : completion)
    }
    unsafe fn removeActionSet_completionHandler_(
        &self,
        actionSet: HMActionSet,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeActionSet : actionSet, completionHandler : completion)
    }
    unsafe fn executeActionSet_completionHandler_(
        &self,
        actionSet: HMActionSet,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeActionSet : actionSet, completionHandler : completion)
    }
    unsafe fn builtinActionSetOfType_(&self, actionSetType: NSString) -> HMActionSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, builtinActionSetOfType : actionSetType)
    }
    unsafe fn actionSets(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionSets)
    }
}
impl HMHome_HMTrigger for HMHome {}
pub trait HMHome_HMTrigger: Sized + std::ops::Deref {
    unsafe fn addTrigger_completionHandler_(
        &self,
        trigger: HMTrigger,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTrigger : trigger, completionHandler : completion)
    }
    unsafe fn removeTrigger_completionHandler_(
        &self,
        trigger: HMTrigger,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTrigger : trigger, completionHandler : completion)
    }
    unsafe fn triggers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triggers)
    }
}
impl HMHome_Matter for HMHome {}
pub trait HMHome_Matter: Sized + std::ops::Deref {
    unsafe fn matterControllerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matterControllerID)
    }
    unsafe fn matterControllerXPCConnectBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matterControllerXPCConnectBlock)
    }
    unsafe fn matterStartupParametersXPCConnectBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matterStartupParametersXPCConnectBlock)
    }
}
pub trait PHMHomeDelegate: Sized + std::ops::Deref {
    unsafe fn homeDidUpdateName_(&self, home: HMHome)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeDidUpdateName : home)
    }
    unsafe fn homeDidUpdateAccessControlForCurrentUser_(&self, home: HMHome)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeDidUpdateAccessControlForCurrentUser : home)
    }
    unsafe fn home_didAddAccessory_(&self, home: HMHome, accessory: HMAccessory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddAccessory : accessory)
    }
    unsafe fn home_didRemoveAccessory_(&self, home: HMHome, accessory: HMAccessory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveAccessory : accessory)
    }
    unsafe fn home_didAddUser_(&self, home: HMHome, user: HMUser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddUser : user)
    }
    unsafe fn home_didRemoveUser_(&self, home: HMHome, user: HMUser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveUser : user)
    }
    unsafe fn home_didUpdateRoom_forAccessory_(
        &self,
        home: HMHome,
        room: HMRoom,
        accessory: HMAccessory,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateRoom : room, forAccessory : accessory)
    }
    unsafe fn home_didAddRoom_(&self, home: HMHome, room: HMRoom)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddRoom : room)
    }
    unsafe fn home_didRemoveRoom_(&self, home: HMHome, room: HMRoom)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveRoom : room)
    }
    unsafe fn home_didUpdateNameForRoom_(&self, home: HMHome, room: HMRoom)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateNameForRoom : room)
    }
    unsafe fn home_didAddZone_(&self, home: HMHome, zone: HMZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddZone : zone)
    }
    unsafe fn home_didRemoveZone_(&self, home: HMHome, zone: HMZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveZone : zone)
    }
    unsafe fn home_didUpdateNameForZone_(&self, home: HMHome, zone: HMZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateNameForZone : zone)
    }
    unsafe fn home_didAddRoom_toZone_(&self, home: HMHome, room: HMRoom, zone: HMZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddRoom : room, toZone : zone)
    }
    unsafe fn home_didRemoveRoom_fromZone_(&self, home: HMHome, room: HMRoom, zone: HMZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveRoom : room, fromZone : zone)
    }
    unsafe fn home_didAddServiceGroup_(&self, home: HMHome, group: HMServiceGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddServiceGroup : group)
    }
    unsafe fn home_didRemoveServiceGroup_(&self, home: HMHome, group: HMServiceGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveServiceGroup : group)
    }
    unsafe fn home_didUpdateNameForServiceGroup_(&self, home: HMHome, group: HMServiceGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateNameForServiceGroup : group)
    }
    unsafe fn home_didAddService_toServiceGroup_(
        &self,
        home: HMHome,
        service: HMService,
        group: HMServiceGroup,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddService : service, toServiceGroup : group)
    }
    unsafe fn home_didRemoveService_fromServiceGroup_(
        &self,
        home: HMHome,
        service: HMService,
        group: HMServiceGroup,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveService : service, fromServiceGroup : group)
    }
    unsafe fn home_didAddActionSet_(&self, home: HMHome, actionSet: HMActionSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddActionSet : actionSet)
    }
    unsafe fn home_didRemoveActionSet_(&self, home: HMHome, actionSet: HMActionSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveActionSet : actionSet)
    }
    unsafe fn home_didUpdateNameForActionSet_(&self, home: HMHome, actionSet: HMActionSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateNameForActionSet : actionSet)
    }
    unsafe fn home_didUpdateActionsForActionSet_(&self, home: HMHome, actionSet: HMActionSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateActionsForActionSet : actionSet)
    }
    unsafe fn home_didAddTrigger_(&self, home: HMHome, trigger: HMTrigger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didAddTrigger : trigger)
    }
    unsafe fn home_didRemoveTrigger_(&self, home: HMHome, trigger: HMTrigger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didRemoveTrigger : trigger)
    }
    unsafe fn home_didUpdateNameForTrigger_(&self, home: HMHome, trigger: HMTrigger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateNameForTrigger : trigger)
    }
    unsafe fn home_didUpdateTrigger_(&self, home: HMHome, trigger: HMTrigger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateTrigger : trigger)
    }
    unsafe fn home_didUnblockAccessory_(&self, home: HMHome, accessory: HMAccessory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUnblockAccessory : accessory)
    }
    unsafe fn home_didEncounterError_forAccessory_(
        &self,
        home: HMHome,
        error: NSError,
        accessory: HMAccessory,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didEncounterError : error, forAccessory : accessory)
    }
    unsafe fn home_didUpdateHomeHubState_(&self, home: HMHome, homeHubState: HMHomeHubState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, home : home, didUpdateHomeHubState : homeHubState)
    }
    unsafe fn homeDidUpdateSupportedFeatures_(&self, home: HMHome)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeDidUpdateSupportedFeatures : home)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessControl(pub id);
impl std::ops::Deref for HMAccessControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessControl {}
impl HMAccessControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessControl").unwrap(), alloc) })
    }
}
impl INSObject for HMAccessControl {}
impl PNSObject for HMAccessControl {}
impl std::convert::TryFrom<NSObject> for HMAccessControl {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessControl").unwrap()) };
        if is_kind_of {
            Ok(HMAccessControl(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessControl")
        }
    }
}
impl IHMAccessControl for HMAccessControl {}
pub trait IHMAccessControl: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMHomeAccessControl(pub id);
impl std::ops::Deref for HMHomeAccessControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMHomeAccessControl {}
impl HMHomeAccessControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMHomeAccessControl").unwrap(), alloc) })
    }
}
impl IHMAccessControl for HMHomeAccessControl {}
impl From<HMHomeAccessControl> for HMAccessControl {
    fn from(child: HMHomeAccessControl) -> HMAccessControl {
        HMAccessControl(child.0)
    }
}
impl std::convert::TryFrom<HMAccessControl> for HMHomeAccessControl {
    type Error = &'static str;
    fn try_from(parent: HMAccessControl) -> Result<HMHomeAccessControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMHomeAccessControl").unwrap()) };
        if is_kind_of {
            Ok(HMHomeAccessControl(parent.0))
        } else {
            Err("This HMAccessControl cannot be downcasted to HMHomeAccessControl")
        }
    }
}
impl INSObject for HMHomeAccessControl {}
impl PNSObject for HMHomeAccessControl {}
impl IHMHomeAccessControl for HMHomeAccessControl {}
pub trait IHMHomeAccessControl: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isAdministrator(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAdministrator)
    }
}
pub type HMHomeManagerAuthorizationStatus = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMHomeManager(pub id);
impl std::ops::Deref for HMHomeManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMHomeManager {}
impl HMHomeManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMHomeManager").unwrap(), alloc) })
    }
}
impl INSObject for HMHomeManager {}
impl PNSObject for HMHomeManager {}
impl std::convert::TryFrom<NSObject> for HMHomeManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMHomeManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMHomeManager").unwrap()) };
        if is_kind_of {
            Ok(HMHomeManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMHomeManager")
        }
    }
}
impl IHMHomeManager for HMHomeManager {}
pub trait IHMHomeManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updatePrimaryHome_completionHandler_(
        &self,
        home: HMHome,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updatePrimaryHome : home, completionHandler : completion)
    }
    unsafe fn addHomeWithName_completionHandler_(
        &self,
        homeName: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addHomeWithName : homeName, completionHandler : completion)
    }
    unsafe fn removeHome_completionHandler_(
        &self,
        home: HMHome,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeHome : home, completionHandler : completion)
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
    unsafe fn authorizationStatus(&self) -> HMHomeManagerAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationStatus)
    }
    unsafe fn primaryHome(&self) -> HMHome
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryHome)
    }
    unsafe fn homes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, homes)
    }
}
pub trait PHMHomeManagerDelegate: Sized + std::ops::Deref {
    unsafe fn homeManager_didUpdateAuthorizationStatus_(
        &self,
        manager: HMHomeManager,
        status: HMHomeManagerAuthorizationStatus,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeManager : manager, didUpdateAuthorizationStatus : status)
    }
    unsafe fn homeManagerDidUpdateHomes_(&self, manager: HMHomeManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeManagerDidUpdateHomes : manager)
    }
    unsafe fn homeManagerDidUpdatePrimaryHome_(&self, manager: HMHomeManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeManagerDidUpdatePrimaryHome : manager)
    }
    unsafe fn homeManager_didAddHome_(&self, manager: HMHomeManager, home: HMHome)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeManager : manager, didAddHome : home)
    }
    unsafe fn homeManager_didRemoveHome_(&self, manager: HMHomeManager, home: HMHome)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeManager : manager, didRemoveHome : home)
    }
    unsafe fn homeManager_didReceiveAddAccessoryRequest_(
        &self,
        manager: HMHomeManager,
        request: HMAddAccessoryRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, homeManager : manager, didReceiveAddAccessoryRequest : request)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMRoom(pub id);
impl std::ops::Deref for HMRoom {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMRoom {}
impl HMRoom {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMRoom").unwrap(), alloc) })
    }
}
impl INSObject for HMRoom {}
impl PNSObject for HMRoom {}
impl std::convert::TryFrom<NSObject> for HMRoom {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMRoom, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMRoom").unwrap()) };
        if is_kind_of {
            Ok(HMRoom(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMRoom")
        }
    }
}
impl IHMRoom for HMRoom {}
pub trait IHMRoom: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateName_completionHandler_(
        &self,
        name: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateName : name, completionHandler : completion)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn accessories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessories)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMService(pub id);
impl std::ops::Deref for HMService {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMService {}
impl HMService {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMService").unwrap(), alloc) })
    }
}
impl INSObject for HMService {}
impl PNSObject for HMService {}
impl std::convert::TryFrom<NSObject> for HMService {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMService, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMService").unwrap()) };
        if is_kind_of {
            Ok(HMService(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMService")
        }
    }
}
impl IHMService for HMService {}
pub trait IHMService: Sized + std::ops::Deref {
    unsafe fn updateName_completionHandler_(
        &self,
        name: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateName : name, completionHandler : completion)
    }
    unsafe fn updateAssociatedServiceType_completionHandler_(
        &self,
        serviceType: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAssociatedServiceType : serviceType, completionHandler : completion)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn accessory(&self) -> HMAccessory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessory)
    }
    unsafe fn serviceType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceType)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn associatedServiceType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, associatedServiceType)
    }
    unsafe fn characteristics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristics)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn isUserInteractive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserInteractive)
    }
    unsafe fn isPrimaryService(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPrimaryService)
    }
    unsafe fn linkedServices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkedServices)
    }
    unsafe fn matterEndpointID(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matterEndpointID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMServiceGroup(pub id);
impl std::ops::Deref for HMServiceGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMServiceGroup {}
impl HMServiceGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMServiceGroup").unwrap(), alloc) })
    }
}
impl INSObject for HMServiceGroup {}
impl PNSObject for HMServiceGroup {}
impl std::convert::TryFrom<NSObject> for HMServiceGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMServiceGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMServiceGroup").unwrap()) };
        if is_kind_of {
            Ok(HMServiceGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMServiceGroup")
        }
    }
}
impl IHMServiceGroup for HMServiceGroup {}
pub trait IHMServiceGroup: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateName_completionHandler_(
        &self,
        name: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateName : name, completionHandler : completion)
    }
    unsafe fn addService_completionHandler_(
        &self,
        service: HMService,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addService : service, completionHandler : completion)
    }
    unsafe fn removeService_completionHandler_(
        &self,
        service: HMService,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeService : service, completionHandler : completion)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn services(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, services)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMTrigger(pub id);
impl std::ops::Deref for HMTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMTrigger {}
impl HMTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMTrigger").unwrap(), alloc) })
    }
}
impl INSObject for HMTrigger {}
impl PNSObject for HMTrigger {}
impl std::convert::TryFrom<NSObject> for HMTrigger {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMTrigger, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMTrigger").unwrap()) };
        if is_kind_of {
            Ok(HMTrigger(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMTrigger")
        }
    }
}
impl IHMTrigger for HMTrigger {}
pub trait IHMTrigger: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateName_completionHandler_(
        &self,
        name: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateName : name, completionHandler : completion)
    }
    unsafe fn addActionSet_completionHandler_(
        &self,
        actionSet: HMActionSet,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addActionSet : actionSet, completionHandler : completion)
    }
    unsafe fn removeActionSet_completionHandler_(
        &self,
        actionSet: HMActionSet,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeActionSet : actionSet, completionHandler : completion)
    }
    unsafe fn enable_completionHandler_(
        &self,
        enable: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enable : enable, completionHandler : completion)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn actionSets(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionSets)
    }
    unsafe fn lastFireDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastFireDate)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMTimerTrigger(pub id);
impl std::ops::Deref for HMTimerTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMTimerTrigger {}
impl HMTimerTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMTimerTrigger").unwrap(), alloc) })
    }
}
impl IHMTrigger for HMTimerTrigger {}
impl From<HMTimerTrigger> for HMTrigger {
    fn from(child: HMTimerTrigger) -> HMTrigger {
        HMTrigger(child.0)
    }
}
impl std::convert::TryFrom<HMTrigger> for HMTimerTrigger {
    type Error = &'static str;
    fn try_from(parent: HMTrigger) -> Result<HMTimerTrigger, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMTimerTrigger").unwrap()) };
        if is_kind_of {
            Ok(HMTimerTrigger(parent.0))
        } else {
            Err("This HMTrigger cannot be downcasted to HMTimerTrigger")
        }
    }
}
impl INSObject for HMTimerTrigger {}
impl PNSObject for HMTimerTrigger {}
impl IHMTimerTrigger for HMTimerTrigger {}
pub trait IHMTimerTrigger: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithName_fireDate_recurrence_(
        &self,
        name: NSString,
        fireDate: NSDate,
        recurrence: NSDateComponents,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, fireDate : fireDate, recurrence : recurrence)
    }
    unsafe fn initWithName_fireDate_timeZone_recurrence_recurrenceCalendar_(
        &self,
        name: NSString,
        fireDate: NSDate,
        timeZone: NSTimeZone,
        recurrence: NSDateComponents,
        recurrenceCalendar: NSCalendar,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, fireDate : fireDate, timeZone : timeZone, recurrence : recurrence, recurrenceCalendar : recurrenceCalendar)
    }
    unsafe fn updateFireDate_completionHandler_(
        &self,
        fireDate: NSDate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateFireDate : fireDate, completionHandler : completion)
    }
    unsafe fn updateTimeZone_completionHandler_(
        &self,
        timeZone: NSTimeZone,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTimeZone : timeZone, completionHandler : completion)
    }
    unsafe fn updateRecurrence_completionHandler_(
        &self,
        recurrence: NSDateComponents,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateRecurrence : recurrence, completionHandler : completion)
    }
    unsafe fn fireDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fireDate)
    }
    unsafe fn timeZone(&self) -> NSTimeZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeZone)
    }
    unsafe fn recurrence(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrence)
    }
    unsafe fn recurrenceCalendar(&self) -> NSCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrenceCalendar)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMUser(pub id);
impl std::ops::Deref for HMUser {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMUser {}
impl HMUser {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMUser").unwrap(), alloc) })
    }
}
impl INSObject for HMUser {}
impl PNSObject for HMUser {}
impl std::convert::TryFrom<NSObject> for HMUser {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMUser, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMUser").unwrap()) };
        if is_kind_of {
            Ok(HMUser(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMUser")
        }
    }
}
impl IHMUser for HMUser {}
pub trait IHMUser: Sized + std::ops::Deref {
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
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMZone(pub id);
impl std::ops::Deref for HMZone {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMZone {}
impl HMZone {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMZone").unwrap(), alloc) })
    }
}
impl INSObject for HMZone {}
impl PNSObject for HMZone {}
impl std::convert::TryFrom<NSObject> for HMZone {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMZone, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMZone").unwrap()) };
        if is_kind_of {
            Ok(HMZone(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMZone")
        }
    }
}
impl IHMZone for HMZone {}
pub trait IHMZone: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateName_completionHandler_(
        &self,
        name: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateName : name, completionHandler : completion)
    }
    unsafe fn addRoom_completionHandler_(
        &self,
        room: HMRoom,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRoom : room, completionHandler : completion)
    }
    unsafe fn removeRoom_completionHandler_(
        &self,
        room: HMRoom,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRoom : room, completionHandler : completion)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn rooms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rooms)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMEvent(pub id);
impl std::ops::Deref for HMEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMEvent {}
impl HMEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMEvent").unwrap(), alloc) })
    }
}
impl INSObject for HMEvent {}
impl PNSObject for HMEvent {}
impl std::convert::TryFrom<NSObject> for HMEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMEvent, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMEvent").unwrap()) };
        if is_kind_of {
            Ok(HMEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMEvent")
        }
    }
}
impl IHMEvent for HMEvent {}
pub trait IHMEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEvent").unwrap(), new)
    }
    unsafe fn isSupportedForHome_(home: HMHome) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEvent").unwrap(), isSupportedForHome : home)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMTimeEvent(pub id);
impl std::ops::Deref for HMTimeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMTimeEvent {}
impl HMTimeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMTimeEvent").unwrap(), alloc) })
    }
}
impl IHMEvent for HMTimeEvent {}
impl From<HMTimeEvent> for HMEvent {
    fn from(child: HMTimeEvent) -> HMEvent {
        HMEvent(child.0)
    }
}
impl std::convert::TryFrom<HMEvent> for HMTimeEvent {
    type Error = &'static str;
    fn try_from(parent: HMEvent) -> Result<HMTimeEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMTimeEvent").unwrap()) };
        if is_kind_of {
            Ok(HMTimeEvent(parent.0))
        } else {
            Err("This HMEvent cannot be downcasted to HMTimeEvent")
        }
    }
}
impl INSObject for HMTimeEvent {}
impl PNSObject for HMTimeEvent {}
impl IHMTimeEvent for HMTimeEvent {}
pub trait IHMTimeEvent: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCalendarEvent(pub id);
impl std::ops::Deref for HMCalendarEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCalendarEvent {}
impl HMCalendarEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCalendarEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for HMCalendarEvent {}
impl PNSMutableCopying for HMCalendarEvent {}
impl IHMTimeEvent for HMCalendarEvent {}
impl From<HMCalendarEvent> for HMTimeEvent {
    fn from(child: HMCalendarEvent) -> HMTimeEvent {
        HMTimeEvent(child.0)
    }
}
impl std::convert::TryFrom<HMTimeEvent> for HMCalendarEvent {
    type Error = &'static str;
    fn try_from(parent: HMTimeEvent) -> Result<HMCalendarEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCalendarEvent").unwrap()) };
        if is_kind_of {
            Ok(HMCalendarEvent(parent.0))
        } else {
            Err("This HMTimeEvent cannot be downcasted to HMCalendarEvent")
        }
    }
}
impl IHMEvent for HMCalendarEvent {}
impl INSObject for HMCalendarEvent {}
impl PNSObject for HMCalendarEvent {}
impl IHMCalendarEvent for HMCalendarEvent {}
pub trait IHMCalendarEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFireDateComponents_(
        &self,
        fireDateComponents: NSDateComponents,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFireDateComponents : fireDateComponents)
    }
    unsafe fn fireDateComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fireDateComponents)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMMutableCalendarEvent(pub id);
impl std::ops::Deref for HMMutableCalendarEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMMutableCalendarEvent {}
impl HMMutableCalendarEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMMutableCalendarEvent").unwrap(), alloc) })
    }
}
impl IHMCalendarEvent for HMMutableCalendarEvent {}
impl PNSCopying for HMMutableCalendarEvent {}
impl PNSMutableCopying for HMMutableCalendarEvent {}
impl From<HMMutableCalendarEvent> for HMCalendarEvent {
    fn from(child: HMMutableCalendarEvent) -> HMCalendarEvent {
        HMCalendarEvent(child.0)
    }
}
impl std::convert::TryFrom<HMCalendarEvent> for HMMutableCalendarEvent {
    type Error = &'static str;
    fn try_from(parent: HMCalendarEvent) -> Result<HMMutableCalendarEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMMutableCalendarEvent").unwrap()) };
        if is_kind_of {
            Ok(HMMutableCalendarEvent(parent.0))
        } else {
            Err("This HMCalendarEvent cannot be downcasted to HMMutableCalendarEvent")
        }
    }
}
impl IHMTimeEvent for HMMutableCalendarEvent {}
impl IHMEvent for HMMutableCalendarEvent {}
impl INSObject for HMMutableCalendarEvent {}
impl PNSObject for HMMutableCalendarEvent {}
impl IHMMutableCalendarEvent for HMMutableCalendarEvent {}
pub trait IHMMutableCalendarEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fireDateComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fireDateComponents)
    }
    unsafe fn setFireDateComponents_(&self, fireDateComponents: NSDateComponents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFireDateComponents : fireDateComponents)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCharacteristicEvent(pub id);
impl std::ops::Deref for HMCharacteristicEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCharacteristicEvent {}
impl HMCharacteristicEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCharacteristicEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for HMCharacteristicEvent {}
impl PNSMutableCopying for HMCharacteristicEvent {}
impl IHMEvent for HMCharacteristicEvent {}
impl std::convert::TryFrom<HMEvent> for HMCharacteristicEvent {
    type Error = &'static str;
    fn try_from(parent: HMEvent) -> Result<HMCharacteristicEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCharacteristicEvent").unwrap()) };
        if is_kind_of {
            Ok(HMCharacteristicEvent(parent.0))
        } else {
            Err("This HMEvent cannot be downcasted to HMCharacteristicEvent")
        }
    }
}
impl INSObject for HMCharacteristicEvent {}
impl PNSObject for HMCharacteristicEvent {}
impl<TriggerValueType: 'static> IHMCharacteristicEvent<TriggerValueType> for HMCharacteristicEvent {}
pub trait IHMCharacteristicEvent<TriggerValueType: 'static>: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCharacteristic_triggerValue_(
        &self,
        characteristic: HMCharacteristic,
        triggerValue: id,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCharacteristic : characteristic, triggerValue : triggerValue)
    }
    unsafe fn updateTriggerValue_completionHandler_(
        &self,
        triggerValue: id,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTriggerValue : triggerValue, completionHandler : completion)
    }
    unsafe fn characteristic(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristic)
    }
    unsafe fn triggerValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triggerValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMMutableCharacteristicEvent(pub id);
impl std::ops::Deref for HMMutableCharacteristicEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMMutableCharacteristicEvent {}
impl HMMutableCharacteristicEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMMutableCharacteristicEvent").unwrap(), alloc) })
    }
}
impl<TriggerValueType: 'static> IHMCharacteristicEvent<TriggerValueType>
    for HMMutableCharacteristicEvent
{
}
impl PNSCopying for HMMutableCharacteristicEvent {}
impl PNSMutableCopying for HMMutableCharacteristicEvent {}
impl IHMEvent for HMMutableCharacteristicEvent {}
impl std::convert::TryFrom<HMEvent> for HMMutableCharacteristicEvent {
    type Error = &'static str;
    fn try_from(parent: HMEvent) -> Result<HMMutableCharacteristicEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMMutableCharacteristicEvent").unwrap()) };
        if is_kind_of {
            Ok(HMMutableCharacteristicEvent(parent.0))
        } else {
            Err("This HMEvent cannot be downcasted to HMMutableCharacteristicEvent")
        }
    }
}
impl INSObject for HMMutableCharacteristicEvent {}
impl PNSObject for HMMutableCharacteristicEvent {}
impl<TriggerValueType: 'static> IHMMutableCharacteristicEvent<TriggerValueType>
    for HMMutableCharacteristicEvent
{
}
pub trait IHMMutableCharacteristicEvent<TriggerValueType: 'static>:
    Sized + std::ops::Deref
{
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn characteristic(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristic)
    }
    unsafe fn setCharacteristic_(&self, characteristic: HMCharacteristic)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharacteristic : characteristic)
    }
    unsafe fn triggerValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triggerValue)
    }
    unsafe fn setTriggerValue_(&self, triggerValue: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriggerValue : triggerValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMLocationEvent(pub id);
impl std::ops::Deref for HMLocationEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMLocationEvent {}
impl HMLocationEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMLocationEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for HMLocationEvent {}
impl PNSMutableCopying for HMLocationEvent {}
impl IHMEvent for HMLocationEvent {}
impl std::convert::TryFrom<HMEvent> for HMLocationEvent {
    type Error = &'static str;
    fn try_from(parent: HMEvent) -> Result<HMLocationEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMLocationEvent").unwrap()) };
        if is_kind_of {
            Ok(HMLocationEvent(parent.0))
        } else {
            Err("This HMEvent cannot be downcasted to HMLocationEvent")
        }
    }
}
impl INSObject for HMLocationEvent {}
impl PNSObject for HMLocationEvent {}
impl IHMLocationEvent for HMLocationEvent {}
pub trait IHMLocationEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRegion_(&self, region: CLRegion) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRegion : region)
    }
    unsafe fn updateRegion_completionHandler_(
        &self,
        region: CLRegion,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateRegion : region, completionHandler : completion)
    }
    unsafe fn region(&self) -> CLRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMMutableLocationEvent(pub id);
impl std::ops::Deref for HMMutableLocationEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMMutableLocationEvent {}
impl HMMutableLocationEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMMutableLocationEvent").unwrap(), alloc) })
    }
}
impl IHMLocationEvent for HMMutableLocationEvent {}
impl PNSCopying for HMMutableLocationEvent {}
impl PNSMutableCopying for HMMutableLocationEvent {}
impl From<HMMutableLocationEvent> for HMLocationEvent {
    fn from(child: HMMutableLocationEvent) -> HMLocationEvent {
        HMLocationEvent(child.0)
    }
}
impl std::convert::TryFrom<HMLocationEvent> for HMMutableLocationEvent {
    type Error = &'static str;
    fn try_from(parent: HMLocationEvent) -> Result<HMMutableLocationEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMMutableLocationEvent").unwrap()) };
        if is_kind_of {
            Ok(HMMutableLocationEvent(parent.0))
        } else {
            Err("This HMLocationEvent cannot be downcasted to HMMutableLocationEvent")
        }
    }
}
impl IHMEvent for HMMutableLocationEvent {}
impl INSObject for HMMutableLocationEvent {}
impl PNSObject for HMMutableLocationEvent {}
impl IHMMutableLocationEvent for HMMutableLocationEvent {}
pub trait IHMMutableLocationEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn region(&self) -> CLRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn setRegion_(&self, region: CLRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegion : region)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessoryProfile(pub id);
impl std::ops::Deref for HMAccessoryProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessoryProfile {}
impl HMAccessoryProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessoryProfile").unwrap(), alloc) })
    }
}
impl INSObject for HMAccessoryProfile {}
impl PNSObject for HMAccessoryProfile {}
impl std::convert::TryFrom<NSObject> for HMAccessoryProfile {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessoryProfile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessoryProfile").unwrap()) };
        if is_kind_of {
            Ok(HMAccessoryProfile(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessoryProfile")
        }
    }
}
impl IHMAccessoryProfile for HMAccessoryProfile {}
pub trait IHMAccessoryProfile: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn services(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, services)
    }
    unsafe fn accessory(&self) -> HMAccessory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessory)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAddAccessoryRequest(pub id);
impl std::ops::Deref for HMAddAccessoryRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAddAccessoryRequest {}
impl HMAddAccessoryRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAddAccessoryRequest").unwrap(), alloc) })
    }
}
impl INSObject for HMAddAccessoryRequest {}
impl PNSObject for HMAddAccessoryRequest {}
impl std::convert::TryFrom<NSObject> for HMAddAccessoryRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAddAccessoryRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAddAccessoryRequest").unwrap()) };
        if is_kind_of {
            Ok(HMAddAccessoryRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAddAccessoryRequest")
        }
    }
}
impl IHMAddAccessoryRequest for HMAddAccessoryRequest {}
pub trait IHMAddAccessoryRequest: Sized + std::ops::Deref {
    unsafe fn payloadWithOwnershipToken_(
        &self,
        ownershipToken: HMAccessoryOwnershipToken,
    ) -> HMAccessorySetupPayload
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, payloadWithOwnershipToken : ownershipToken)
    }
    unsafe fn payloadWithURL_ownershipToken_(
        &self,
        setupPayloadURL: NSURL,
        ownershipToken: HMAccessoryOwnershipToken,
    ) -> HMAccessorySetupPayload
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, payloadWithURL : setupPayloadURL, ownershipToken : ownershipToken)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn home(&self) -> HMHome
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, home)
    }
    unsafe fn accessoryName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryName)
    }
    unsafe fn accessoryCategory(&self) -> HMAccessoryCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryCategory)
    }
    unsafe fn requiresSetupPayloadURL(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresSetupPayloadURL)
    }
    unsafe fn requiresOwnershipToken(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresOwnershipToken)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCharacteristicThresholdRangeEvent(pub id);
impl std::ops::Deref for HMCharacteristicThresholdRangeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCharacteristicThresholdRangeEvent {}
impl HMCharacteristicThresholdRangeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCharacteristicThresholdRangeEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for HMCharacteristicThresholdRangeEvent {}
impl PNSMutableCopying for HMCharacteristicThresholdRangeEvent {}
impl IHMEvent for HMCharacteristicThresholdRangeEvent {}
impl std::convert::TryFrom<HMEvent> for HMCharacteristicThresholdRangeEvent {
    type Error = &'static str;
    fn try_from(parent: HMEvent) -> Result<HMCharacteristicThresholdRangeEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCharacteristicThresholdRangeEvent").unwrap())
        };
        if is_kind_of {
            Ok(HMCharacteristicThresholdRangeEvent(parent.0))
        } else {
            Err("This HMEvent cannot be downcasted to HMCharacteristicThresholdRangeEvent")
        }
    }
}
impl INSObject for HMCharacteristicThresholdRangeEvent {}
impl PNSObject for HMCharacteristicThresholdRangeEvent {}
impl IHMCharacteristicThresholdRangeEvent for HMCharacteristicThresholdRangeEvent {}
pub trait IHMCharacteristicThresholdRangeEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCharacteristic_thresholdRange_(
        &self,
        characteristic: HMCharacteristic,
        thresholdRange: HMNumberRange,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCharacteristic : characteristic, thresholdRange : thresholdRange)
    }
    unsafe fn characteristic(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristic)
    }
    unsafe fn thresholdRange(&self) -> HMNumberRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thresholdRange)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMMutableCharacteristicThresholdRangeEvent(pub id);
impl std::ops::Deref for HMMutableCharacteristicThresholdRangeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMMutableCharacteristicThresholdRangeEvent {}
impl HMMutableCharacteristicThresholdRangeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMMutableCharacteristicThresholdRangeEvent").unwrap(), alloc) })
    }
}
impl IHMCharacteristicThresholdRangeEvent for HMMutableCharacteristicThresholdRangeEvent {}
impl PNSCopying for HMMutableCharacteristicThresholdRangeEvent {}
impl PNSMutableCopying for HMMutableCharacteristicThresholdRangeEvent {}
impl From<HMMutableCharacteristicThresholdRangeEvent> for HMCharacteristicThresholdRangeEvent {
    fn from(
        child: HMMutableCharacteristicThresholdRangeEvent,
    ) -> HMCharacteristicThresholdRangeEvent {
        HMCharacteristicThresholdRangeEvent(child.0)
    }
}
impl std::convert::TryFrom<HMCharacteristicThresholdRangeEvent>
    for HMMutableCharacteristicThresholdRangeEvent
{
    type Error = &'static str;
    fn try_from(
        parent: HMCharacteristicThresholdRangeEvent,
    ) -> Result<HMMutableCharacteristicThresholdRangeEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMMutableCharacteristicThresholdRangeEvent").unwrap())
        };
        if is_kind_of {
            Ok(HMMutableCharacteristicThresholdRangeEvent(parent.0))
        } else {
            Err ("This HMCharacteristicThresholdRangeEvent cannot be downcasted to HMMutableCharacteristicThresholdRangeEvent" ,)
        }
    }
}
impl IHMEvent for HMMutableCharacteristicThresholdRangeEvent {}
impl INSObject for HMMutableCharacteristicThresholdRangeEvent {}
impl PNSObject for HMMutableCharacteristicThresholdRangeEvent {}
impl IHMMutableCharacteristicThresholdRangeEvent for HMMutableCharacteristicThresholdRangeEvent {}
pub trait IHMMutableCharacteristicThresholdRangeEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn characteristic(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristic)
    }
    unsafe fn setCharacteristic_(&self, characteristic: HMCharacteristic)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharacteristic : characteristic)
    }
    unsafe fn thresholdRange(&self) -> HMNumberRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thresholdRange)
    }
    unsafe fn setThresholdRange_(&self, thresholdRange: HMNumberRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThresholdRange : thresholdRange)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMDurationEvent(pub id);
impl std::ops::Deref for HMDurationEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMDurationEvent {}
impl HMDurationEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMDurationEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for HMDurationEvent {}
impl PNSMutableCopying for HMDurationEvent {}
impl IHMTimeEvent for HMDurationEvent {}
impl std::convert::TryFrom<HMTimeEvent> for HMDurationEvent {
    type Error = &'static str;
    fn try_from(parent: HMTimeEvent) -> Result<HMDurationEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMDurationEvent").unwrap()) };
        if is_kind_of {
            Ok(HMDurationEvent(parent.0))
        } else {
            Err("This HMTimeEvent cannot be downcasted to HMDurationEvent")
        }
    }
}
impl IHMEvent for HMDurationEvent {}
impl INSObject for HMDurationEvent {}
impl PNSObject for HMDurationEvent {}
impl IHMDurationEvent for HMDurationEvent {}
pub trait IHMDurationEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDuration_(&self, duration: NSTimeInterval) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDuration : duration)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMMutableDurationEvent(pub id);
impl std::ops::Deref for HMMutableDurationEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMMutableDurationEvent {}
impl HMMutableDurationEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMMutableDurationEvent").unwrap(), alloc) })
    }
}
impl IHMDurationEvent for HMMutableDurationEvent {}
impl PNSCopying for HMMutableDurationEvent {}
impl PNSMutableCopying for HMMutableDurationEvent {}
impl From<HMMutableDurationEvent> for HMDurationEvent {
    fn from(child: HMMutableDurationEvent) -> HMDurationEvent {
        HMDurationEvent(child.0)
    }
}
impl std::convert::TryFrom<HMDurationEvent> for HMMutableDurationEvent {
    type Error = &'static str;
    fn try_from(parent: HMDurationEvent) -> Result<HMMutableDurationEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMMutableDurationEvent").unwrap()) };
        if is_kind_of {
            Ok(HMMutableDurationEvent(parent.0))
        } else {
            Err("This HMDurationEvent cannot be downcasted to HMMutableDurationEvent")
        }
    }
}
impl IHMTimeEvent for HMMutableDurationEvent {}
impl IHMEvent for HMMutableDurationEvent {}
impl INSObject for HMMutableDurationEvent {}
impl PNSObject for HMMutableDurationEvent {}
impl IHMMutableDurationEvent for HMMutableDurationEvent {}
pub trait IHMMutableDurationEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn setDuration_(&self, duration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuration : duration)
    }
}
pub type HMErrorCode = NSInteger;
pub type HMEventTriggerActivationState = NSUInteger;
pub type HMSignificantEvent = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMSignificantTimeEvent(pub id);
impl std::ops::Deref for HMSignificantTimeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMSignificantTimeEvent {}
impl HMSignificantTimeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMSignificantTimeEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for HMSignificantTimeEvent {}
impl PNSMutableCopying for HMSignificantTimeEvent {}
impl IHMTimeEvent for HMSignificantTimeEvent {}
impl std::convert::TryFrom<HMTimeEvent> for HMSignificantTimeEvent {
    type Error = &'static str;
    fn try_from(parent: HMTimeEvent) -> Result<HMSignificantTimeEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMSignificantTimeEvent").unwrap()) };
        if is_kind_of {
            Ok(HMSignificantTimeEvent(parent.0))
        } else {
            Err("This HMTimeEvent cannot be downcasted to HMSignificantTimeEvent")
        }
    }
}
impl IHMEvent for HMSignificantTimeEvent {}
impl INSObject for HMSignificantTimeEvent {}
impl PNSObject for HMSignificantTimeEvent {}
impl IHMSignificantTimeEvent for HMSignificantTimeEvent {}
pub trait IHMSignificantTimeEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSignificantEvent_offset_(
        &self,
        significantEvent: NSString,
        offset: NSDateComponents,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSignificantEvent : significantEvent, offset : offset)
    }
    unsafe fn significantEvent(&self) -> HMSignificantEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, significantEvent)
    }
    unsafe fn offset(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMMutableSignificantTimeEvent(pub id);
impl std::ops::Deref for HMMutableSignificantTimeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMMutableSignificantTimeEvent {}
impl HMMutableSignificantTimeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMMutableSignificantTimeEvent").unwrap(), alloc) })
    }
}
impl IHMSignificantTimeEvent for HMMutableSignificantTimeEvent {}
impl PNSCopying for HMMutableSignificantTimeEvent {}
impl PNSMutableCopying for HMMutableSignificantTimeEvent {}
impl From<HMMutableSignificantTimeEvent> for HMSignificantTimeEvent {
    fn from(child: HMMutableSignificantTimeEvent) -> HMSignificantTimeEvent {
        HMSignificantTimeEvent(child.0)
    }
}
impl std::convert::TryFrom<HMSignificantTimeEvent> for HMMutableSignificantTimeEvent {
    type Error = &'static str;
    fn try_from(
        parent: HMSignificantTimeEvent,
    ) -> Result<HMMutableSignificantTimeEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMMutableSignificantTimeEvent").unwrap())
        };
        if is_kind_of {
            Ok(HMMutableSignificantTimeEvent(parent.0))
        } else {
            Err("This HMSignificantTimeEvent cannot be downcasted to HMMutableSignificantTimeEvent")
        }
    }
}
impl IHMTimeEvent for HMMutableSignificantTimeEvent {}
impl IHMEvent for HMMutableSignificantTimeEvent {}
impl INSObject for HMMutableSignificantTimeEvent {}
impl PNSObject for HMMutableSignificantTimeEvent {}
impl IHMMutableSignificantTimeEvent for HMMutableSignificantTimeEvent {}
pub trait IHMMutableSignificantTimeEvent: Sized + std::ops::Deref {
    unsafe fn significantEvent(&self) -> HMSignificantEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, significantEvent)
    }
    unsafe fn setSignificantEvent_(&self, significantEvent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSignificantEvent : significantEvent)
    }
    unsafe fn offset(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: NSDateComponents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMEventTrigger(pub id);
impl std::ops::Deref for HMEventTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMEventTrigger {}
impl HMEventTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), alloc) })
    }
}
impl IHMTrigger for HMEventTrigger {}
impl std::convert::TryFrom<HMTrigger> for HMEventTrigger {
    type Error = &'static str;
    fn try_from(parent: HMTrigger) -> Result<HMEventTrigger, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap()) };
        if is_kind_of {
            Ok(HMEventTrigger(parent.0))
        } else {
            Err("This HMTrigger cannot be downcasted to HMEventTrigger")
        }
    }
}
impl INSObject for HMEventTrigger {}
impl PNSObject for HMEventTrigger {}
impl IHMEventTrigger for HMEventTrigger {}
pub trait IHMEventTrigger: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithName_events_predicate_(
        &self,
        name: NSString,
        events: NSArray,
        predicate: NSPredicate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, events : events, predicate : predicate)
    }
    unsafe fn initWithName_events_endEvents_recurrences_predicate_(
        &self,
        name: NSString,
        events: NSArray,
        endEvents: NSArray,
        recurrences: NSArray,
        predicate: NSPredicate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, events : events, endEvents : endEvents, recurrences : recurrences, predicate : predicate)
    }
    unsafe fn addEvent_completionHandler_(
        &self,
        event: HMEvent,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addEvent : event, completionHandler : completion)
    }
    unsafe fn removeEvent_completionHandler_(
        &self,
        event: HMEvent,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeEvent : event, completionHandler : completion)
    }
    unsafe fn updateEvents_completionHandler_(
        &self,
        events: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateEvents : events, completionHandler : completion)
    }
    unsafe fn updateEndEvents_completionHandler_(
        &self,
        endEvents: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateEndEvents : endEvents, completionHandler : completion)
    }
    unsafe fn updatePredicate_completionHandler_(
        &self,
        predicate: NSPredicate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updatePredicate : predicate, completionHandler : completion)
    }
    unsafe fn updateRecurrences_completionHandler_(
        &self,
        recurrences: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateRecurrences : recurrences, completionHandler : completion)
    }
    unsafe fn updateExecuteOnce_completionHandler_(
        &self,
        executeOnce: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateExecuteOnce : executeOnce, completionHandler : completion)
    }
    unsafe fn events(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, events)
    }
    unsafe fn endEvents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endEvents)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
    }
    unsafe fn recurrences(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrences)
    }
    unsafe fn executeOnce(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, executeOnce)
    }
    unsafe fn triggerActivationState(&self) -> HMEventTriggerActivationState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triggerActivationState)
    }
}
impl HMEventTrigger_NSPredicate for HMEventTrigger {}
pub trait HMEventTrigger_NSPredicate: Sized + std::ops::Deref {
    unsafe fn predicateForEvaluatingTriggerOccurringBeforeSignificantEvent_applyingOffset_(
        significantEvent: NSString,
        offset: NSDateComponents,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringBeforeSignificantEvent : significantEvent, applyingOffset : offset)
    }
    unsafe fn predicateForEvaluatingTriggerOccurringBeforeSignificantEvent_(
        significantEvent: HMSignificantTimeEvent,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringBeforeSignificantEvent : significantEvent)
    }
    unsafe fn predicateForEvaluatingTriggerOccurringAfterSignificantEvent_applyingOffset_(
        significantEvent: NSString,
        offset: NSDateComponents,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringAfterSignificantEvent : significantEvent, applyingOffset : offset)
    }
    unsafe fn predicateForEvaluatingTriggerOccurringAfterSignificantEvent_(
        significantEvent: HMSignificantTimeEvent,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringAfterSignificantEvent : significantEvent)
    }
    unsafe fn predicateForEvaluatingTriggerOccurringBetweenSignificantEvent_secondSignificantEvent_(
        firstSignificantEvent: HMSignificantTimeEvent,
        secondSignificantEvent: HMSignificantTimeEvent,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringBetweenSignificantEvent : firstSignificantEvent, secondSignificantEvent : secondSignificantEvent)
    }
    unsafe fn predicateForEvaluatingTriggerOccurringBeforeDateWithComponents_(
        dateComponents: NSDateComponents,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringBeforeDateWithComponents : dateComponents)
    }
    unsafe fn predicateForEvaluatingTriggerOccurringOnDateWithComponents_(
        dateComponents: NSDateComponents,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringOnDateWithComponents : dateComponents)
    }
    unsafe fn predicateForEvaluatingTriggerOccurringAfterDateWithComponents_(
        dateComponents: NSDateComponents,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringAfterDateWithComponents : dateComponents)
    }
    unsafe fn predicateForEvaluatingTriggerOccurringBetweenDateWithComponents_secondDateWithComponents_(
        firstDateComponents: NSDateComponents,
        secondDateWithComponents: NSDateComponents,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerOccurringBetweenDateWithComponents : firstDateComponents, secondDateWithComponents : secondDateWithComponents)
    }
    unsafe fn predicateForEvaluatingTriggerWithCharacteristic_relatedBy_toValue_(
        characteristic: HMCharacteristic,
        operatorType: NSPredicateOperatorType,
        value: id,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerWithCharacteristic : characteristic, relatedBy : operatorType, toValue : value)
    }
    unsafe fn predicateForEvaluatingTriggerWithPresence_(
        presenceEvent: HMPresenceEvent,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMEventTrigger").unwrap(), predicateForEvaluatingTriggerWithPresence : presenceEvent)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMNetworkConfigurationProfile(pub id);
impl std::ops::Deref for HMNetworkConfigurationProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMNetworkConfigurationProfile {}
impl HMNetworkConfigurationProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMNetworkConfigurationProfile").unwrap(), alloc) })
    }
}
impl IHMAccessoryProfile for HMNetworkConfigurationProfile {}
impl From<HMNetworkConfigurationProfile> for HMAccessoryProfile {
    fn from(child: HMNetworkConfigurationProfile) -> HMAccessoryProfile {
        HMAccessoryProfile(child.0)
    }
}
impl std::convert::TryFrom<HMAccessoryProfile> for HMNetworkConfigurationProfile {
    type Error = &'static str;
    fn try_from(parent: HMAccessoryProfile) -> Result<HMNetworkConfigurationProfile, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMNetworkConfigurationProfile").unwrap())
        };
        if is_kind_of {
            Ok(HMNetworkConfigurationProfile(parent.0))
        } else {
            Err("This HMAccessoryProfile cannot be downcasted to HMNetworkConfigurationProfile")
        }
    }
}
impl INSObject for HMNetworkConfigurationProfile {}
impl PNSObject for HMNetworkConfigurationProfile {}
impl IHMNetworkConfigurationProfile for HMNetworkConfigurationProfile {}
pub trait IHMNetworkConfigurationProfile: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn isNetworkAccessRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNetworkAccessRestricted)
    }
}
pub trait PHMNetworkConfigurationProfileDelegate: Sized + std::ops::Deref {
    unsafe fn profileDidUpdateNetworkAccessMode_(&self, profile: HMNetworkConfigurationProfile)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, profileDidUpdateNetworkAccessMode : profile)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMNumberRange(pub id);
impl std::ops::Deref for HMNumberRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMNumberRange {}
impl HMNumberRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMNumberRange").unwrap(), alloc) })
    }
}
impl INSObject for HMNumberRange {}
impl PNSObject for HMNumberRange {}
impl std::convert::TryFrom<NSObject> for HMNumberRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMNumberRange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMNumberRange").unwrap()) };
        if is_kind_of {
            Ok(HMNumberRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMNumberRange")
        }
    }
}
impl IHMNumberRange for HMNumberRange {}
pub trait IHMNumberRange: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn minValue(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minValue)
    }
    unsafe fn maxValue(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxValue)
    }
    unsafe fn numberRangeWithMinValue_maxValue_(
        minValue: NSNumber,
        maxValue: NSNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMNumberRange").unwrap(), numberRangeWithMinValue : minValue, maxValue : maxValue)
    }
    unsafe fn numberRangeWithMinValue_(minValue: NSNumber) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMNumberRange").unwrap(), numberRangeWithMinValue : minValue)
    }
    unsafe fn numberRangeWithMaxValue_(maxValue: NSNumber) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMNumberRange").unwrap(), numberRangeWithMaxValue : maxValue)
    }
}
pub type HMPresenceEventType = NSUInteger;
pub type HMPresenceEventUserType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMPresenceEvent(pub id);
impl std::ops::Deref for HMPresenceEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMPresenceEvent {}
impl HMPresenceEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMPresenceEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for HMPresenceEvent {}
impl PNSMutableCopying for HMPresenceEvent {}
impl IHMEvent for HMPresenceEvent {}
impl std::convert::TryFrom<HMEvent> for HMPresenceEvent {
    type Error = &'static str;
    fn try_from(parent: HMEvent) -> Result<HMPresenceEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMPresenceEvent").unwrap()) };
        if is_kind_of {
            Ok(HMPresenceEvent(parent.0))
        } else {
            Err("This HMEvent cannot be downcasted to HMPresenceEvent")
        }
    }
}
impl INSObject for HMPresenceEvent {}
impl PNSObject for HMPresenceEvent {}
impl IHMPresenceEvent for HMPresenceEvent {}
pub trait IHMPresenceEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPresenceEventType_presenceUserType_(
        &self,
        presenceEventType: HMPresenceEventType,
        presenceUserType: HMPresenceEventUserType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPresenceEventType : presenceEventType, presenceUserType : presenceUserType)
    }
    unsafe fn presenceEventType(&self) -> HMPresenceEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presenceEventType)
    }
    unsafe fn presenceUserType(&self) -> HMPresenceEventUserType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presenceUserType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMMutablePresenceEvent(pub id);
impl std::ops::Deref for HMMutablePresenceEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMMutablePresenceEvent {}
impl HMMutablePresenceEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMMutablePresenceEvent").unwrap(), alloc) })
    }
}
impl IHMPresenceEvent for HMMutablePresenceEvent {}
impl PNSCopying for HMMutablePresenceEvent {}
impl PNSMutableCopying for HMMutablePresenceEvent {}
impl From<HMMutablePresenceEvent> for HMPresenceEvent {
    fn from(child: HMMutablePresenceEvent) -> HMPresenceEvent {
        HMPresenceEvent(child.0)
    }
}
impl std::convert::TryFrom<HMPresenceEvent> for HMMutablePresenceEvent {
    type Error = &'static str;
    fn try_from(parent: HMPresenceEvent) -> Result<HMMutablePresenceEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMMutablePresenceEvent").unwrap()) };
        if is_kind_of {
            Ok(HMMutablePresenceEvent(parent.0))
        } else {
            Err("This HMPresenceEvent cannot be downcasted to HMMutablePresenceEvent")
        }
    }
}
impl IHMEvent for HMMutablePresenceEvent {}
impl INSObject for HMMutablePresenceEvent {}
impl PNSObject for HMMutablePresenceEvent {}
impl IHMMutablePresenceEvent for HMMutablePresenceEvent {}
pub trait IHMMutablePresenceEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn presenceEventType(&self) -> HMPresenceEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presenceEventType)
    }
    unsafe fn setPresenceEventType_(&self, presenceEventType: HMPresenceEventType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresenceEventType : presenceEventType)
    }
    unsafe fn presenceUserType(&self) -> HMPresenceEventUserType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presenceUserType)
    }
    unsafe fn setPresenceUserType_(&self, presenceUserType: HMPresenceEventUserType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresenceUserType : presenceUserType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraView(pub id);
impl std::ops::Deref for HMCameraView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraView {}
impl HMCameraView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraView").unwrap(), alloc) })
    }
}
impl PNSCoding for HMCameraView {}
impl INSObject for HMCameraView {}
impl PNSObject for HMCameraView {}
impl std::convert::TryFrom<NSObject> for HMCameraView {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMCameraView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraView").unwrap()) };
        if is_kind_of {
            Ok(HMCameraView(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMCameraView")
        }
    }
}
impl IHMCameraView for HMCameraView {}
pub trait IHMCameraView: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn cameraSource(&self) -> HMCameraSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraSource)
    }
    unsafe fn setCameraSource_(&self, cameraSource: HMCameraSource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraSource : cameraSource)
    }
}
impl HMAccessory_Camera for HMAccessory {}
pub trait HMAccessory_Camera: Sized + std::ops::Deref {
    unsafe fn cameraProfiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraProfiles)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraControl(pub id);
impl std::ops::Deref for HMCameraControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraControl {}
impl HMCameraControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraControl").unwrap(), alloc) })
    }
}
impl INSObject for HMCameraControl {}
impl PNSObject for HMCameraControl {}
impl std::convert::TryFrom<NSObject> for HMCameraControl {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMCameraControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraControl").unwrap()) };
        if is_kind_of {
            Ok(HMCameraControl(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMCameraControl")
        }
    }
}
impl IHMCameraControl for HMCameraControl {}
pub trait IHMCameraControl: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraProfile(pub id);
impl std::ops::Deref for HMCameraProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraProfile {}
impl HMCameraProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraProfile").unwrap(), alloc) })
    }
}
impl IHMAccessoryProfile for HMCameraProfile {}
impl std::convert::TryFrom<HMAccessoryProfile> for HMCameraProfile {
    type Error = &'static str;
    fn try_from(parent: HMAccessoryProfile) -> Result<HMCameraProfile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraProfile").unwrap()) };
        if is_kind_of {
            Ok(HMCameraProfile(parent.0))
        } else {
            Err("This HMAccessoryProfile cannot be downcasted to HMCameraProfile")
        }
    }
}
impl INSObject for HMCameraProfile {}
impl PNSObject for HMCameraProfile {}
impl IHMCameraProfile for HMCameraProfile {}
pub trait IHMCameraProfile: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn streamControl(&self) -> HMCameraStreamControl
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamControl)
    }
    unsafe fn snapshotControl(&self) -> HMCameraSnapshotControl
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotControl)
    }
    unsafe fn settingsControl(&self) -> HMCameraSettingsControl
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, settingsControl)
    }
    unsafe fn speakerControl(&self) -> HMCameraAudioControl
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speakerControl)
    }
    unsafe fn microphoneControl(&self) -> HMCameraAudioControl
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, microphoneControl)
    }
}
pub type HMCameraStreamState = NSUInteger;
pub type HMCameraAudioStreamSetting = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraSource(pub id);
impl std::ops::Deref for HMCameraSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraSource {}
impl HMCameraSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraSource").unwrap(), alloc) })
    }
}
impl INSObject for HMCameraSource {}
impl PNSObject for HMCameraSource {}
impl std::convert::TryFrom<NSObject> for HMCameraSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMCameraSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraSource").unwrap()) };
        if is_kind_of {
            Ok(HMCameraSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMCameraSource")
        }
    }
}
impl IHMCameraSource for HMCameraSource {}
pub trait IHMCameraSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn aspectRatio(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraStream(pub id);
impl std::ops::Deref for HMCameraStream {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraStream {}
impl HMCameraStream {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraStream").unwrap(), alloc) })
    }
}
impl IHMCameraSource for HMCameraStream {}
impl From<HMCameraStream> for HMCameraSource {
    fn from(child: HMCameraStream) -> HMCameraSource {
        HMCameraSource(child.0)
    }
}
impl std::convert::TryFrom<HMCameraSource> for HMCameraStream {
    type Error = &'static str;
    fn try_from(parent: HMCameraSource) -> Result<HMCameraStream, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraStream").unwrap()) };
        if is_kind_of {
            Ok(HMCameraStream(parent.0))
        } else {
            Err("This HMCameraSource cannot be downcasted to HMCameraStream")
        }
    }
}
impl INSObject for HMCameraStream {}
impl PNSObject for HMCameraStream {}
impl IHMCameraStream for HMCameraStream {}
pub trait IHMCameraStream: Sized + std::ops::Deref {
    unsafe fn setAudioStreamSetting_(&self, audioStreamSetting: HMCameraAudioStreamSetting)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioStreamSetting : audioStreamSetting)
    }
    unsafe fn updateAudioStreamSetting_completionHandler_(
        &self,
        audioStreamSetting: HMCameraAudioStreamSetting,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAudioStreamSetting : audioStreamSetting, completionHandler : completion)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn audioStreamSetting(&self) -> HMCameraAudioStreamSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioStreamSetting)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraStreamControl(pub id);
impl std::ops::Deref for HMCameraStreamControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraStreamControl {}
impl HMCameraStreamControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraStreamControl").unwrap(), alloc) })
    }
}
impl IHMCameraControl for HMCameraStreamControl {}
impl From<HMCameraStreamControl> for HMCameraControl {
    fn from(child: HMCameraStreamControl) -> HMCameraControl {
        HMCameraControl(child.0)
    }
}
impl std::convert::TryFrom<HMCameraControl> for HMCameraStreamControl {
    type Error = &'static str;
    fn try_from(parent: HMCameraControl) -> Result<HMCameraStreamControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraStreamControl").unwrap()) };
        if is_kind_of {
            Ok(HMCameraStreamControl(parent.0))
        } else {
            Err("This HMCameraControl cannot be downcasted to HMCameraStreamControl")
        }
    }
}
impl INSObject for HMCameraStreamControl {}
impl PNSObject for HMCameraStreamControl {}
impl IHMCameraStreamControl for HMCameraStreamControl {}
pub trait IHMCameraStreamControl: Sized + std::ops::Deref {
    unsafe fn startStream(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startStream)
    }
    unsafe fn stopStream(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopStream)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn streamState(&self) -> HMCameraStreamState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamState)
    }
    unsafe fn cameraStream(&self) -> HMCameraStream
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraStream)
    }
}
pub trait PHMCameraStreamControlDelegate: Sized + std::ops::Deref {
    unsafe fn cameraStreamControlDidStartStream_(&self, cameraStreamControl: HMCameraStreamControl)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraStreamControlDidStartStream : cameraStreamControl)
    }
    unsafe fn cameraStreamControl_didStopStreamWithError_(
        &self,
        cameraStreamControl: HMCameraStreamControl,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraStreamControl : cameraStreamControl, didStopStreamWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraSnapshot(pub id);
impl std::ops::Deref for HMCameraSnapshot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraSnapshot {}
impl HMCameraSnapshot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraSnapshot").unwrap(), alloc) })
    }
}
impl IHMCameraSource for HMCameraSnapshot {}
impl std::convert::TryFrom<HMCameraSource> for HMCameraSnapshot {
    type Error = &'static str;
    fn try_from(parent: HMCameraSource) -> Result<HMCameraSnapshot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraSnapshot").unwrap()) };
        if is_kind_of {
            Ok(HMCameraSnapshot(parent.0))
        } else {
            Err("This HMCameraSource cannot be downcasted to HMCameraSnapshot")
        }
    }
}
impl INSObject for HMCameraSnapshot {}
impl PNSObject for HMCameraSnapshot {}
impl IHMCameraSnapshot for HMCameraSnapshot {}
pub trait IHMCameraSnapshot: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn captureDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captureDate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraSnapshotControl(pub id);
impl std::ops::Deref for HMCameraSnapshotControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraSnapshotControl {}
impl HMCameraSnapshotControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraSnapshotControl").unwrap(), alloc) })
    }
}
impl IHMCameraControl for HMCameraSnapshotControl {}
impl std::convert::TryFrom<HMCameraControl> for HMCameraSnapshotControl {
    type Error = &'static str;
    fn try_from(parent: HMCameraControl) -> Result<HMCameraSnapshotControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraSnapshotControl").unwrap()) };
        if is_kind_of {
            Ok(HMCameraSnapshotControl(parent.0))
        } else {
            Err("This HMCameraControl cannot be downcasted to HMCameraSnapshotControl")
        }
    }
}
impl INSObject for HMCameraSnapshotControl {}
impl PNSObject for HMCameraSnapshotControl {}
impl IHMCameraSnapshotControl for HMCameraSnapshotControl {}
pub trait IHMCameraSnapshotControl: Sized + std::ops::Deref {
    unsafe fn takeSnapshot(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, takeSnapshot)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn mostRecentSnapshot(&self) -> HMCameraSnapshot
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mostRecentSnapshot)
    }
}
pub trait PHMCameraSnapshotControlDelegate: Sized + std::ops::Deref {
    unsafe fn cameraSnapshotControl_didTakeSnapshot_error_(
        &self,
        cameraSnapshotControl: HMCameraSnapshotControl,
        snapshot: HMCameraSnapshot,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraSnapshotControl : cameraSnapshotControl, didTakeSnapshot : snapshot, error : error)
    }
    unsafe fn cameraSnapshotControlDidUpdateMostRecentSnapshot_(
        &self,
        cameraSnapshotControl: HMCameraSnapshotControl,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraSnapshotControlDidUpdateMostRecentSnapshot : cameraSnapshotControl)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraAudioControl(pub id);
impl std::ops::Deref for HMCameraAudioControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraAudioControl {}
impl HMCameraAudioControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraAudioControl").unwrap(), alloc) })
    }
}
impl IHMCameraControl for HMCameraAudioControl {}
impl std::convert::TryFrom<HMCameraControl> for HMCameraAudioControl {
    type Error = &'static str;
    fn try_from(parent: HMCameraControl) -> Result<HMCameraAudioControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraAudioControl").unwrap()) };
        if is_kind_of {
            Ok(HMCameraAudioControl(parent.0))
        } else {
            Err("This HMCameraControl cannot be downcasted to HMCameraAudioControl")
        }
    }
}
impl INSObject for HMCameraAudioControl {}
impl PNSObject for HMCameraAudioControl {}
impl IHMCameraAudioControl for HMCameraAudioControl {}
pub trait IHMCameraAudioControl: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn mute(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mute)
    }
    unsafe fn volume(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volume)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMCameraSettingsControl(pub id);
impl std::ops::Deref for HMCameraSettingsControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMCameraSettingsControl {}
impl HMCameraSettingsControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMCameraSettingsControl").unwrap(), alloc) })
    }
}
impl IHMCameraControl for HMCameraSettingsControl {}
impl std::convert::TryFrom<HMCameraControl> for HMCameraSettingsControl {
    type Error = &'static str;
    fn try_from(parent: HMCameraControl) -> Result<HMCameraSettingsControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMCameraSettingsControl").unwrap()) };
        if is_kind_of {
            Ok(HMCameraSettingsControl(parent.0))
        } else {
            Err("This HMCameraControl cannot be downcasted to HMCameraSettingsControl")
        }
    }
}
impl INSObject for HMCameraSettingsControl {}
impl PNSObject for HMCameraSettingsControl {}
impl IHMCameraSettingsControl for HMCameraSettingsControl {}
pub trait IHMCameraSettingsControl: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn nightVision(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nightVision)
    }
    unsafe fn currentHorizontalTilt(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentHorizontalTilt)
    }
    unsafe fn targetHorizontalTilt(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetHorizontalTilt)
    }
    unsafe fn currentVerticalTilt(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentVerticalTilt)
    }
    unsafe fn targetVerticalTilt(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetVerticalTilt)
    }
    unsafe fn opticalZoom(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opticalZoom)
    }
    unsafe fn digitalZoom(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, digitalZoom)
    }
    unsafe fn imageRotation(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageRotation)
    }
    unsafe fn imageMirroring(&self) -> HMCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageMirroring)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessorySetupManager(pub id);
impl std::ops::Deref for HMAccessorySetupManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessorySetupManager {}
impl HMAccessorySetupManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessorySetupManager").unwrap(), alloc) })
    }
}
impl INSObject for HMAccessorySetupManager {}
impl PNSObject for HMAccessorySetupManager {}
impl std::convert::TryFrom<NSObject> for HMAccessorySetupManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessorySetupManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessorySetupManager").unwrap()) };
        if is_kind_of {
            Ok(HMAccessorySetupManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessorySetupManager")
        }
    }
}
impl IHMAccessorySetupManager for HMAccessorySetupManager {}
pub trait IHMAccessorySetupManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn performAccessorySetupUsingRequest_completionHandler_(
        &self,
        request: HMAccessorySetupRequest,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performAccessorySetupUsingRequest : request, completionHandler : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessoryOwnershipToken(pub id);
impl std::ops::Deref for HMAccessoryOwnershipToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessoryOwnershipToken {}
impl HMAccessoryOwnershipToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessoryOwnershipToken").unwrap(), alloc) })
    }
}
impl INSObject for HMAccessoryOwnershipToken {}
impl PNSObject for HMAccessoryOwnershipToken {}
impl std::convert::TryFrom<NSObject> for HMAccessoryOwnershipToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessoryOwnershipToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessoryOwnershipToken").unwrap()) };
        if is_kind_of {
            Ok(HMAccessoryOwnershipToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessoryOwnershipToken")
        }
    }
}
impl IHMAccessoryOwnershipToken for HMAccessoryOwnershipToken {}
pub trait IHMAccessoryOwnershipToken: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessoryOwnershipToken").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessorySetupPayload(pub id);
impl std::ops::Deref for HMAccessorySetupPayload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessorySetupPayload {}
impl HMAccessorySetupPayload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessorySetupPayload").unwrap(), alloc) })
    }
}
impl INSObject for HMAccessorySetupPayload {}
impl PNSObject for HMAccessorySetupPayload {}
impl std::convert::TryFrom<NSObject> for HMAccessorySetupPayload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessorySetupPayload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessorySetupPayload").unwrap()) };
        if is_kind_of {
            Ok(HMAccessorySetupPayload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessorySetupPayload")
        }
    }
}
impl IHMAccessorySetupPayload for HMAccessorySetupPayload {}
pub trait IHMAccessorySetupPayload: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithURL_(&self, setupPayloadURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : setupPayloadURL)
    }
    unsafe fn initWithURL_ownershipToken_(
        &self,
        setupPayloadURL: NSURL,
        ownershipToken: HMAccessoryOwnershipToken,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : setupPayloadURL, ownershipToken : ownershipToken)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessorySetupPayload").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessorySetupRequest(pub id);
impl std::ops::Deref for HMAccessorySetupRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessorySetupRequest {}
impl HMAccessorySetupRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessorySetupRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for HMAccessorySetupRequest {}
impl INSObject for HMAccessorySetupRequest {}
impl PNSObject for HMAccessorySetupRequest {}
impl std::convert::TryFrom<NSObject> for HMAccessorySetupRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessorySetupRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessorySetupRequest").unwrap()) };
        if is_kind_of {
            Ok(HMAccessorySetupRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessorySetupRequest")
        }
    }
}
impl IHMAccessorySetupRequest for HMAccessorySetupRequest {}
pub trait IHMAccessorySetupRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn payload(&self) -> HMAccessorySetupPayload
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, payload)
    }
    unsafe fn setPayload_(&self, payload: HMAccessorySetupPayload)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPayload : payload)
    }
    unsafe fn homeUniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, homeUniqueIdentifier)
    }
    unsafe fn setHomeUniqueIdentifier_(&self, homeUniqueIdentifier: NSUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHomeUniqueIdentifier : homeUniqueIdentifier)
    }
    unsafe fn suggestedRoomUniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suggestedRoomUniqueIdentifier)
    }
    unsafe fn setSuggestedRoomUniqueIdentifier_(&self, suggestedRoomUniqueIdentifier: NSUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuggestedRoomUniqueIdentifier : suggestedRoomUniqueIdentifier)
    }
    unsafe fn suggestedAccessoryName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suggestedAccessoryName)
    }
    unsafe fn setSuggestedAccessoryName_(&self, suggestedAccessoryName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuggestedAccessoryName : suggestedAccessoryName)
    }
    unsafe fn matterPayload(&self) -> MTRSetupPayload
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matterPayload)
    }
    unsafe fn setMatterPayload_(&self, matterPayload: MTRSetupPayload)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatterPayload : matterPayload)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HMAccessorySetupResult(pub id);
impl std::ops::Deref for HMAccessorySetupResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HMAccessorySetupResult {}
impl HMAccessorySetupResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessorySetupResult").unwrap(), alloc) })
    }
}
impl PNSCopying for HMAccessorySetupResult {}
impl INSObject for HMAccessorySetupResult {}
impl PNSObject for HMAccessorySetupResult {}
impl std::convert::TryFrom<NSObject> for HMAccessorySetupResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<HMAccessorySetupResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HMAccessorySetupResult").unwrap()) };
        if is_kind_of {
            Ok(HMAccessorySetupResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to HMAccessorySetupResult")
        }
    }
}
impl IHMAccessorySetupResult for HMAccessorySetupResult {}
pub trait IHMAccessorySetupResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn homeUniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, homeUniqueIdentifier)
    }
    unsafe fn accessoryUniqueIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryUniqueIdentifiers)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"HMAccessorySetupResult").unwrap(), new)
    }
}
impl HMAccessory_ for HMAccessory {}
pub trait HMAccessory_: Sized + std::ops::Deref {
    unsafe fn isVendorAccessory(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVendorAccessory)
    }
    unsafe fn HAPInstanceID(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, HAPInstanceID)
    }
}
impl HMHomeManager_Vendor for HMHomeManager {}
pub trait HMHomeManager_Vendor: Sized + std::ops::Deref {
    unsafe fn findVendorAccessoryWithHAPPublicKey_completionHandler_(
        &self,
        hapPublicKey: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findVendorAccessoryWithHAPPublicKey : hapPublicKey, completionHandler : completion)
    }
}
pub trait PUIAppearanceContainer: Sized + std::ops::Deref {}
impl PUIAppearanceContainer for HMCameraView {}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeOther: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeSecuritySystem: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeBridge: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeDoor: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeDoorLock: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeFan: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeGarageDoorOpener: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeIPCamera: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeLightbulb: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeOutlet: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeProgrammableSwitch: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeRangeExtender: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeSensor: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeSwitch: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeThermostat: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeVideoDoorbell: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeWindow: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeWindowCovering: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeAirPurifier: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeAirHeater: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeAirConditioner: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeAirHumidifier: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeAirDehumidifier: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeSprinkler: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeFaucet: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeShowerHead: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeTelevision: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeTelevisionSetTopBox: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeTelevisionStreamingStick: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeWiFiRouter: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeSpeaker: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeAudioReceiver: NSString;
}
unsafe extern "C" {
    pub static HMAccessoryCategoryTypeAirPort: NSString;
}
unsafe extern "C" {
    pub static HMActionSetTypeWakeUp: NSString;
}
unsafe extern "C" {
    pub static HMActionSetTypeSleep: NSString;
}
unsafe extern "C" {
    pub static HMActionSetTypeHomeDeparture: NSString;
}
unsafe extern "C" {
    pub static HMActionSetTypeHomeArrival: NSString;
}
unsafe extern "C" {
    pub static HMActionSetTypeUserDefined: NSString;
}
unsafe extern "C" {
    pub static HMActionSetTypeTriggerOwned: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicPropertySupportsEventNotification: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicPropertyReadable: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicPropertyWritable: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicPropertyHidden: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicPropertyRequiresAuthorizationData: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetRelativeHumidity: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeOutletInUse: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeLogs: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeAudioFeedback: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeAdminOnlyAccess: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSecuritySystemAlarmType: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeMotionDetected: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeLockMechanismLastKnownAction: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeLockManagementControlPoint: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeLockManagementAutoSecureTimeout: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeAirParticulateDensity: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeAirParticulateSize: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeAirQuality: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCarbonDioxideDetected: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCarbonDioxideLevel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCarbonDioxidePeakLevel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCarbonMonoxideDetected: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCarbonMonoxideLevel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCarbonMonoxidePeakLevel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeContactState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentHorizontalTilt: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentPosition: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentSecuritySystemState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentVerticalTilt: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeHoldPosition: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeLeakDetected: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeOccupancyDetected: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeOutputState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypePositionState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeStatusActive: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeStatusFault: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeStatusJammed: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeStatusTampered: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetHorizontalTilt: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetSecuritySystemState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetPosition: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetVerticalTilt: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeStreamingStatus: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSetupStreamEndpoint: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSupportedVideoStreamConfiguration: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSupportedRTPConfiguration: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSelectedStreamConfiguration: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeOpticalZoom: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeDigitalZoom: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeImageRotation: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeImageMirroring: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeLabelNamespace: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeLabelIndex: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentAirPurifierState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetAirPurifierState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentSlatState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeFilterChangeIndication: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeFilterLifeLevel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeFilterResetChangeIndication: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSlatType: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentTilt: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetTilt: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeOzoneDensity: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeNitrogenDioxideDensity: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSulphurDioxideDensity: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypePM2_5Density: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypePM10Density: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeVolatileOrganicCompoundDensity: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeProgramMode: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeInUse: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSetDuration: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeRemainingDuration: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeValveType: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeVolumeControlType: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeVolumeSelector: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeBrightness: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCoolingThreshold: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentDoorState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentHeatingCooling: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentRelativeHumidity: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentTemperature: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeHeatingThreshold: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeHue: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeIdentify: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentLockMechanismState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetLockMechanismState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeManufacturer: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeModel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeName: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeObstructionDetected: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypePowerState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeRotationDirection: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeRotationSpeed: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSaturation: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSerialNumber: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetDoorState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetHeatingCooling: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetTemperature: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTemperatureUnits: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeVersion: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeFirmwareVersion: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeHardwareVersion: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSoftwareVersion: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeBatteryLevel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentLightLevel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeInputEvent: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSmokeDetected: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeStatusLowBattery: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeChargingState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeLockPhysicalControls: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentFanState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeActive: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentHeaterCoolerState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetHeaterCoolerState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentHumidifierDehumidifierState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetHumidifierDehumidifierState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeWaterLevel: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSwingMode: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetFanState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeDehumidifierThreshold: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeHumidifierThreshold: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeColorTemperature: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeIsConfigured: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeInputSourceType: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeInputDeviceType: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeClosedCaptions: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypePowerModeSelection: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentMediaState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeRemoteKey: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypePictureMode: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeConfiguredName: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeActiveIdentifier: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeSupportedAudioStreamConfiguration: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeVolume: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeMute: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeNightVision: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetVisibilityState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeCurrentVisibilityState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeTargetMediaState: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeRouterStatus: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeWANStatusList: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicTypeWiFiSatelliteStatus: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatBool: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatInt: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatFloat: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatString: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatArray: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatDictionary: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatUInt8: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatUInt16: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatUInt32: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatUInt64: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatData: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataFormatTLV8: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataUnitsCelsius: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataUnitsFahrenheit: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataUnitsPercentage: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataUnitsArcDegree: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataUnitsSeconds: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataUnitsLux: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataUnitsPartsPerMillion: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicMetadataUnitsMicrogramsPerCubicMeter: NSString;
}
unsafe extern "C" {
    pub static HMUserFailedAccessoriesKey: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeSwitch: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeThermostat: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeOutlet: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeLockManagement: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeAirQualitySensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeCarbonDioxideSensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeCarbonMonoxideSensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeContactSensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeDoor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeHumiditySensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeLeakSensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeLightSensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeMotionSensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeOccupancySensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeSecuritySystem: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeStatefulProgrammableSwitch: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeStatelessProgrammableSwitch: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeSmokeSensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeTemperatureSensor: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeWindow: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeWindowCovering: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeCameraRTPStreamManagement: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeCameraControl: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeMicrophone: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeSpeaker: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeAirPurifier: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeFilterMaintenance: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeSlats: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeLabel: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeIrrigationSystem: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeValve: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeFaucet: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeAccessoryInformation: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeFan: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeGarageDoorOpener: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeLightbulb: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeLockMechanism: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeBattery: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeVentilationFan: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeHeaterCooler: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeHumidifierDehumidifier: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeTelevision: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeInputSource: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeDoorbell: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeWiFiRouter: NSString;
}
unsafe extern "C" {
    pub static HMServiceTypeWiFiSatellite: NSString;
}
unsafe extern "C" {
    pub static HMErrorDomain: NSString;
}
unsafe extern "C" {
    pub static HMSignificantEventSunrise: HMSignificantEvent;
}
unsafe extern "C" {
    pub static HMSignificantEventSunset: HMSignificantEvent;
}
unsafe extern "C" {
    pub static HMCharacteristicKeyPath: NSString;
}
unsafe extern "C" {
    pub static HMCharacteristicValueKeyPath: NSString;
}
unsafe extern "C" {
    pub static HMPresenceKeyPath: NSString;
}

unsafe impl objc2::encode::RefEncode for HMAccessory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessoryBrowser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessoryBrowser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessoryCategory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessoryCategory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMActionSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMActionSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCharacteristic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCharacteristic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCharacteristicMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCharacteristicMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCharacteristicWriteAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCharacteristicWriteAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMHome {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMHome {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMHomeAccessControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMHomeAccessControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMHomeManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMHomeManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMRoom {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMRoom {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMServiceGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMServiceGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMTimerTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMTimerTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMUser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMUser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMZone {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMZone {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMTimeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMTimeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCalendarEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCalendarEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMMutableCalendarEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMMutableCalendarEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCharacteristicEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCharacteristicEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMMutableCharacteristicEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMMutableCharacteristicEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMLocationEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMLocationEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMMutableLocationEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMMutableLocationEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessoryProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessoryProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAddAccessoryRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAddAccessoryRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCharacteristicThresholdRangeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCharacteristicThresholdRangeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMMutableCharacteristicThresholdRangeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMMutableCharacteristicThresholdRangeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMDurationEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMDurationEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMMutableDurationEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMMutableDurationEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMSignificantTimeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMSignificantTimeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMMutableSignificantTimeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMMutableSignificantTimeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMEventTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMEventTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMNetworkConfigurationProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMNetworkConfigurationProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMNumberRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMNumberRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMPresenceEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMPresenceEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMMutablePresenceEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMMutablePresenceEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraStreamControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraStreamControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraSnapshot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraSnapshot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraSnapshotControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraSnapshotControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraAudioControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraAudioControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMCameraSettingsControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMCameraSettingsControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessorySetupManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessorySetupManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessoryOwnershipToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessoryOwnershipToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessorySetupPayload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessorySetupPayload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessorySetupRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessorySetupRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for HMAccessorySetupResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HMAccessorySetupResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
