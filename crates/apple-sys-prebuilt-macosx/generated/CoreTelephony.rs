#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CTError {
    pub domain: SInt32,
    pub error: SInt32,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTCall(pub id);
impl std::ops::Deref for CTCall {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTCall {}
impl CTCall {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTCall").unwrap(), alloc) })
    }
}
impl INSObject for CTCall {}
impl PNSObject for CTCall {}
impl std::convert::TryFrom<NSObject> for CTCall {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTCall, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTCall").unwrap()) };
        if is_kind_of {
            Ok(CTCall(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTCall")
        }
    }
}
impl ICTCall for CTCall {}
pub trait ICTCall: Sized + std::ops::Deref {
    unsafe fn callState(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callState)
    }
    unsafe fn callID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTCallCenter(pub id);
impl std::ops::Deref for CTCallCenter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTCallCenter {}
impl CTCallCenter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTCallCenter").unwrap(), alloc) })
    }
}
impl INSObject for CTCallCenter {}
impl PNSObject for CTCallCenter {}
impl std::convert::TryFrom<NSObject> for CTCallCenter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTCallCenter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTCallCenter").unwrap()) };
        if is_kind_of {
            Ok(CTCallCenter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTCallCenter")
        }
    }
}
impl ICTCallCenter for CTCallCenter {}
pub trait ICTCallCenter: Sized + std::ops::Deref {
    unsafe fn currentCalls(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentCalls)
    }
    unsafe fn callEventHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callEventHandler)
    }
    unsafe fn setCallEventHandler_(&self, callEventHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCallEventHandler : callEventHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTCarrier(pub id);
impl std::ops::Deref for CTCarrier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTCarrier {}
impl CTCarrier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTCarrier").unwrap(), alloc) })
    }
}
impl INSObject for CTCarrier {}
impl PNSObject for CTCarrier {}
impl std::convert::TryFrom<NSObject> for CTCarrier {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTCarrier, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTCarrier").unwrap()) };
        if is_kind_of {
            Ok(CTCarrier(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTCarrier")
        }
    }
}
impl ICTCarrier for CTCarrier {}
pub trait ICTCarrier: Sized + std::ops::Deref {
    unsafe fn carrierName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, carrierName)
    }
    unsafe fn mobileCountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mobileCountryCode)
    }
    unsafe fn mobileNetworkCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mobileNetworkCode)
    }
    unsafe fn isoCountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isoCountryCode)
    }
    unsafe fn allowsVOIP(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsVOIP)
    }
}
pub type CTCellularDataRestrictedState = NSUInteger;
pub type CellularDataRestrictionDidUpdateNotifier = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTCellularData(pub id);
impl std::ops::Deref for CTCellularData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTCellularData {}
impl CTCellularData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTCellularData").unwrap(), alloc) })
    }
}
impl INSObject for CTCellularData {}
impl PNSObject for CTCellularData {}
impl std::convert::TryFrom<NSObject> for CTCellularData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTCellularData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTCellularData").unwrap()) };
        if is_kind_of {
            Ok(CTCellularData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTCellularData")
        }
    }
}
impl ICTCellularData for CTCellularData {}
pub trait ICTCellularData: Sized + std::ops::Deref {
    unsafe fn cellularDataRestrictionDidUpdateNotifier(
        &self,
    ) -> CellularDataRestrictionDidUpdateNotifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellularDataRestrictionDidUpdateNotifier)
    }
    unsafe fn setCellularDataRestrictionDidUpdateNotifier_(
        &self,
        cellularDataRestrictionDidUpdateNotifier: CellularDataRestrictionDidUpdateNotifier,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCellularDataRestrictionDidUpdateNotifier : cellularDataRestrictionDidUpdateNotifier)
    }
    unsafe fn restrictedState(&self) -> CTCellularDataRestrictedState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restrictedState)
    }
}
pub type CTCellularPlanCapability = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTCellularPlanProperties(pub id);
impl std::ops::Deref for CTCellularPlanProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTCellularPlanProperties {}
impl CTCellularPlanProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTCellularPlanProperties").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CTCellularPlanProperties {}
impl INSObject for CTCellularPlanProperties {}
impl PNSObject for CTCellularPlanProperties {}
impl std::convert::TryFrom<NSObject> for CTCellularPlanProperties {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTCellularPlanProperties, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTCellularPlanProperties").unwrap()) };
        if is_kind_of {
            Ok(CTCellularPlanProperties(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTCellularPlanProperties")
        }
    }
}
impl ICTCellularPlanProperties for CTCellularPlanProperties {}
pub trait ICTCellularPlanProperties: Sized + std::ops::Deref {
    unsafe fn associatedIccid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, associatedIccid)
    }
    unsafe fn setAssociatedIccid_(&self, associatedIccid: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAssociatedIccid : associatedIccid)
    }
    unsafe fn simCapability(&self) -> CTCellularPlanCapability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simCapability)
    }
    unsafe fn setSimCapability_(&self, simCapability: CTCellularPlanCapability)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSimCapability : simCapability)
    }
    unsafe fn supportedRegionCodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedRegionCodes)
    }
    unsafe fn setSupportedRegionCodes_(&self, supportedRegionCodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedRegionCodes : supportedRegionCodes)
    }
}
pub type CTCellularPlanProvisioningAddPlanResult = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTCellularPlanProvisioningRequest(pub id);
impl std::ops::Deref for CTCellularPlanProvisioningRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTCellularPlanProvisioningRequest {}
impl CTCellularPlanProvisioningRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTCellularPlanProvisioningRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CTCellularPlanProvisioningRequest {}
impl INSObject for CTCellularPlanProvisioningRequest {}
impl PNSObject for CTCellularPlanProvisioningRequest {}
impl std::convert::TryFrom<NSObject> for CTCellularPlanProvisioningRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTCellularPlanProvisioningRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTCellularPlanProvisioningRequest").unwrap())
        };
        if is_kind_of {
            Ok(CTCellularPlanProvisioningRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTCellularPlanProvisioningRequest")
        }
    }
}
impl ICTCellularPlanProvisioningRequest for CTCellularPlanProvisioningRequest {}
pub trait ICTCellularPlanProvisioningRequest: Sized + std::ops::Deref {
    unsafe fn address(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn setAddress_(&self, address: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddress : address)
    }
    unsafe fn matchingID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchingID)
    }
    unsafe fn setMatchingID_(&self, matchingID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchingID : matchingID)
    }
    unsafe fn OID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, OID)
    }
    unsafe fn setOID_(&self, OID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOID : OID)
    }
    unsafe fn confirmationCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confirmationCode)
    }
    unsafe fn setConfirmationCode_(&self, confirmationCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfirmationCode : confirmationCode)
    }
    unsafe fn ICCID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ICCID)
    }
    unsafe fn setICCID_(&self, ICCID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setICCID : ICCID)
    }
    unsafe fn EID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, EID)
    }
    unsafe fn setEID_(&self, EID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEID : EID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTCellularPlanProvisioning(pub id);
impl std::ops::Deref for CTCellularPlanProvisioning {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTCellularPlanProvisioning {}
impl CTCellularPlanProvisioning {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTCellularPlanProvisioning").unwrap(), alloc) })
    }
}
impl INSObject for CTCellularPlanProvisioning {}
impl PNSObject for CTCellularPlanProvisioning {}
impl std::convert::TryFrom<NSObject> for CTCellularPlanProvisioning {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTCellularPlanProvisioning, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTCellularPlanProvisioning").unwrap()) };
        if is_kind_of {
            Ok(CTCellularPlanProvisioning(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTCellularPlanProvisioning")
        }
    }
}
impl ICTCellularPlanProvisioning for CTCellularPlanProvisioning {}
pub trait ICTCellularPlanProvisioning: Sized + std::ops::Deref {
    unsafe fn supportsCellularPlan(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsCellularPlan)
    }
    unsafe fn addPlanWith_completionHandler_(
        &self,
        request: CTCellularPlanProvisioningRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPlanWith : request, completionHandler : completionHandler)
    }
    unsafe fn addPlanWithRequest_properties_completionHandler_(
        &self,
        request: CTCellularPlanProvisioningRequest,
        properties: CTCellularPlanProperties,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPlanWithRequest : request, properties : properties, completionHandler : completionHandler)
    }
    unsafe fn updateCellularPlanProperties_completionHandler_(
        &self,
        properties: CTCellularPlanProperties,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateCellularPlanProperties : properties, completionHandler : completionHandler)
    }
    unsafe fn supportsEmbeddedSIM(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsEmbeddedSIM)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTCellularPlanStatus(pub id);
impl std::ops::Deref for CTCellularPlanStatus {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTCellularPlanStatus {}
impl CTCellularPlanStatus {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTCellularPlanStatus").unwrap(), alloc) })
    }
}
impl INSObject for CTCellularPlanStatus {}
impl PNSObject for CTCellularPlanStatus {}
impl std::convert::TryFrom<NSObject> for CTCellularPlanStatus {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTCellularPlanStatus, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTCellularPlanStatus").unwrap()) };
        if is_kind_of {
            Ok(CTCellularPlanStatus(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTCellularPlanStatus")
        }
    }
}
impl ICTCellularPlanStatus for CTCellularPlanStatus {}
pub trait ICTCellularPlanStatus: Sized + std::ops::Deref {
    unsafe fn getTokenWithCompletion_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CTCellularPlanStatus").unwrap(), getTokenWithCompletion : completionHandler)
    }
    unsafe fn checkValidityOfToken_completionHandler_(
        token: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CTCellularPlanStatus").unwrap(), checkValidityOfToken : token, completionHandler : completionHandler)
    }
}
pub trait PCTSubscriberDelegate: Sized + std::ops::Deref {
    unsafe fn subscriberTokenRefreshed_(&self, subscriber: CTSubscriber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, subscriberTokenRefreshed : subscriber)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTSubscriber(pub id);
impl std::ops::Deref for CTSubscriber {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTSubscriber {}
impl CTSubscriber {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTSubscriber").unwrap(), alloc) })
    }
}
impl INSObject for CTSubscriber {}
impl PNSObject for CTSubscriber {}
impl std::convert::TryFrom<NSObject> for CTSubscriber {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTSubscriber, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTSubscriber").unwrap()) };
        if is_kind_of {
            Ok(CTSubscriber(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTSubscriber")
        }
    }
}
impl ICTSubscriber for CTSubscriber {}
pub trait ICTSubscriber: Sized + std::ops::Deref {
    unsafe fn refreshCarrierToken(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refreshCarrierToken)
    }
    unsafe fn carrierToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, carrierToken)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn isSIMInserted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSIMInserted)
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTSubscriberInfo(pub id);
impl std::ops::Deref for CTSubscriberInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTSubscriberInfo {}
impl CTSubscriberInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTSubscriberInfo").unwrap(), alloc) })
    }
}
impl INSObject for CTSubscriberInfo {}
impl PNSObject for CTSubscriberInfo {}
impl std::convert::TryFrom<NSObject> for CTSubscriberInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTSubscriberInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTSubscriberInfo").unwrap()) };
        if is_kind_of {
            Ok(CTSubscriberInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTSubscriberInfo")
        }
    }
}
impl ICTSubscriberInfo for CTSubscriberInfo {}
pub trait ICTSubscriberInfo: Sized + std::ops::Deref {
    unsafe fn subscribers() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CTSubscriberInfo").unwrap(), subscribers)
    }
    unsafe fn subscriber() -> CTSubscriber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CTSubscriberInfo").unwrap(), subscriber)
    }
}
pub trait PCTTelephonyNetworkInfoDelegate: Sized + std::ops::Deref {
    unsafe fn dataServiceIdentifierDidChange_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataServiceIdentifierDidChange : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CTTelephonyNetworkInfo(pub id);
impl std::ops::Deref for CTTelephonyNetworkInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CTTelephonyNetworkInfo {}
impl CTTelephonyNetworkInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CTTelephonyNetworkInfo").unwrap(), alloc) })
    }
}
impl INSObject for CTTelephonyNetworkInfo {}
impl PNSObject for CTTelephonyNetworkInfo {}
impl std::convert::TryFrom<NSObject> for CTTelephonyNetworkInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CTTelephonyNetworkInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CTTelephonyNetworkInfo").unwrap()) };
        if is_kind_of {
            Ok(CTTelephonyNetworkInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CTTelephonyNetworkInfo")
        }
    }
}
impl ICTTelephonyNetworkInfo for CTTelephonyNetworkInfo {}
pub trait ICTTelephonyNetworkInfo: Sized + std::ops::Deref {
    unsafe fn dataServiceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataServiceIdentifier)
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
    unsafe fn serviceSubscriberCellularProviders(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceSubscriberCellularProviders)
    }
    unsafe fn subscriberCellularProvider(&self) -> CTCarrier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriberCellularProvider)
    }
    unsafe fn serviceSubscriberCellularProvidersDidUpdateNotifier(
        &self,
    ) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceSubscriberCellularProvidersDidUpdateNotifier)
    }
    unsafe fn setServiceSubscriberCellularProvidersDidUpdateNotifier_(
        &self,
        serviceSubscriberCellularProvidersDidUpdateNotifier: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServiceSubscriberCellularProvidersDidUpdateNotifier : serviceSubscriberCellularProvidersDidUpdateNotifier)
    }
    unsafe fn subscriberCellularProviderDidUpdateNotifier(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriberCellularProviderDidUpdateNotifier)
    }
    unsafe fn setSubscriberCellularProviderDidUpdateNotifier_(
        &self,
        subscriberCellularProviderDidUpdateNotifier: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubscriberCellularProviderDidUpdateNotifier : subscriberCellularProviderDidUpdateNotifier)
    }
    unsafe fn serviceCurrentRadioAccessTechnology(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceCurrentRadioAccessTechnology)
    }
    unsafe fn currentRadioAccessTechnology(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentRadioAccessTechnology)
    }
}
unsafe extern "C" {
    pub static CTCallStateDialing: NSString;
}
unsafe extern "C" {
    pub static CTCallStateIncoming: NSString;
}
unsafe extern "C" {
    pub static CTCallStateConnected: NSString;
}
unsafe extern "C" {
    pub static CTCallStateDisconnected: NSString;
}
unsafe extern "C" {
    pub static CTSubscriberTokenRefreshed: NSString;
}
unsafe extern "C" {
    pub static CTServiceRadioAccessTechnologyDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyGPRS: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyEdge: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyWCDMA: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyHSDPA: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyHSUPA: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyCDMA1x: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyCDMAEVDORev0: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyCDMAEVDORevA: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyCDMAEVDORevB: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyeHRPD: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyLTE: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyNRNSA: NSString;
}
unsafe extern "C" {
    pub static CTRadioAccessTechnologyNR: NSString;
}

unsafe impl objc2::encode::RefEncode for CTError {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTError {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CTError", &[]);
}
unsafe impl objc2::encode::RefEncode for CTCall {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTCall {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTCallCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTCallCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTCarrier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTCarrier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTCellularData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTCellularData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTCellularPlanProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTCellularPlanProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTCellularPlanProvisioningRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTCellularPlanProvisioningRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTCellularPlanProvisioning {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTCellularPlanProvisioning {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTCellularPlanStatus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTCellularPlanStatus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTSubscriber {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTSubscriber {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTSubscriberInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTSubscriberInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CTTelephonyNetworkInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CTTelephonyNetworkInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
