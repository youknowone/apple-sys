#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CloudKit::*;
#[allow(unused_imports)]
use crate::Contacts::*;
#[allow(unused_imports)]
use crate::CoreSpotlight::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CLLocationDegrees = f64;
pub type CLLocationAccuracy = f64;
pub type CLLocationSpeed = f64;
pub type CLLocationSpeedAccuracy = f64;
pub type CLLocationDirection = f64;
pub type CLLocationDirectionAccuracy = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CLLocationCoordinate2D {
    pub latitude: CLLocationDegrees,
    pub longitude: CLLocationDegrees,
}
pub type CLLocationDistance = f64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLFloor(pub id);
impl std::ops::Deref for CLFloor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLFloor {}
impl CLFloor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLFloor").unwrap(), alloc) })
    }
}
impl PNSCopying for CLFloor {}
impl PNSSecureCoding for CLFloor {}
impl INSObject for CLFloor {}
impl PNSObject for CLFloor {}
impl std::convert::TryFrom<NSObject> for CLFloor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLFloor, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLFloor").unwrap()) };
        if is_kind_of {
            Ok(CLFloor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLFloor")
        }
    }
}
impl ICLFloor for CLFloor {}
pub trait ICLFloor: Sized + std::ops::Deref {
    unsafe fn level(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, level)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLLocationSourceInformation(pub id);
impl std::ops::Deref for CLLocationSourceInformation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLLocationSourceInformation {}
impl CLLocationSourceInformation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationSourceInformation").unwrap(), alloc) })
    }
}
impl PNSCopying for CLLocationSourceInformation {}
impl PNSSecureCoding for CLLocationSourceInformation {}
impl INSObject for CLLocationSourceInformation {}
impl PNSObject for CLLocationSourceInformation {}
impl std::convert::TryFrom<NSObject> for CLLocationSourceInformation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLLocationSourceInformation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLLocationSourceInformation").unwrap()) };
        if is_kind_of {
            Ok(CLLocationSourceInformation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLLocationSourceInformation")
        }
    }
}
impl ICLLocationSourceInformation for CLLocationSourceInformation {}
pub trait ICLLocationSourceInformation: Sized + std::ops::Deref {
    unsafe fn initWithSoftwareSimulationState_andExternalAccessoryState_(
        &self,
        isSoftware: BOOL,
        isAccessory: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSoftwareSimulationState : isSoftware, andExternalAccessoryState : isAccessory)
    }
    unsafe fn isSimulatedBySoftware(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSimulatedBySoftware)
    }
    unsafe fn isProducedByAccessory(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isProducedByAccessory)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLLocation(pub id);
impl std::ops::Deref for CLLocation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLLocation {}
impl CLLocation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocation").unwrap(), alloc) })
    }
}
impl PNSCopying for CLLocation {}
impl PNSSecureCoding for CLLocation {}
impl INSObject for CLLocation {}
impl PNSObject for CLLocation {}
impl std::convert::TryFrom<NSObject> for CLLocation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLLocation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLLocation").unwrap()) };
        if is_kind_of {
            Ok(CLLocation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLLocation")
        }
    }
}
impl ICLLocation for CLLocation {}
pub trait ICLLocation: Sized + std::ops::Deref {
    unsafe fn initWithLatitude_longitude_(
        &self,
        latitude: CLLocationDegrees,
        longitude: CLLocationDegrees,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLatitude : latitude, longitude : longitude)
    }
    unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_timestamp_(
        &self,
        coordinate: CLLocationCoordinate2D,
        altitude: CLLocationDistance,
        hAccuracy: CLLocationAccuracy,
        vAccuracy: CLLocationAccuracy,
        timestamp: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate, altitude : altitude, horizontalAccuracy : hAccuracy, verticalAccuracy : vAccuracy, timestamp : timestamp)
    }
    unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_speed_timestamp_(
        &self,
        coordinate: CLLocationCoordinate2D,
        altitude: CLLocationDistance,
        hAccuracy: CLLocationAccuracy,
        vAccuracy: CLLocationAccuracy,
        course: CLLocationDirection,
        speed: CLLocationSpeed,
        timestamp: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate, altitude : altitude, horizontalAccuracy : hAccuracy, verticalAccuracy : vAccuracy, course : course, speed : speed, timestamp : timestamp)
    }
    unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp_(
        &self,
        coordinate: CLLocationCoordinate2D,
        altitude: CLLocationDistance,
        hAccuracy: CLLocationAccuracy,
        vAccuracy: CLLocationAccuracy,
        course: CLLocationDirection,
        courseAccuracy: CLLocationDirectionAccuracy,
        speed: CLLocationSpeed,
        speedAccuracy: CLLocationSpeedAccuracy,
        timestamp: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate, altitude : altitude, horizontalAccuracy : hAccuracy, verticalAccuracy : vAccuracy, course : course, courseAccuracy : courseAccuracy, speed : speed, speedAccuracy : speedAccuracy, timestamp : timestamp)
    }
    unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp_sourceInfo_(
        &self,
        coordinate: CLLocationCoordinate2D,
        altitude: CLLocationDistance,
        hAccuracy: CLLocationAccuracy,
        vAccuracy: CLLocationAccuracy,
        course: CLLocationDirection,
        courseAccuracy: CLLocationDirectionAccuracy,
        speed: CLLocationSpeed,
        speedAccuracy: CLLocationSpeedAccuracy,
        timestamp: NSDate,
        sourceInfo: CLLocationSourceInformation,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate, altitude : altitude, horizontalAccuracy : hAccuracy, verticalAccuracy : vAccuracy, course : course, courseAccuracy : courseAccuracy, speed : speed, speedAccuracy : speedAccuracy, timestamp : timestamp, sourceInfo : sourceInfo)
    }
    unsafe fn getDistanceFrom_(&self, location: CLLocation) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDistanceFrom : location)
    }
    unsafe fn distanceFromLocation_(&self, location: CLLocation) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, distanceFromLocation : location)
    }
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn altitude(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altitude)
    }
    unsafe fn ellipsoidalAltitude(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ellipsoidalAltitude)
    }
    unsafe fn horizontalAccuracy(&self) -> CLLocationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalAccuracy)
    }
    unsafe fn verticalAccuracy(&self) -> CLLocationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalAccuracy)
    }
    unsafe fn course(&self) -> CLLocationDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, course)
    }
    unsafe fn courseAccuracy(&self) -> CLLocationDirectionAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, courseAccuracy)
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
    unsafe fn timestamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn floor(&self) -> CLFloor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floor)
    }
    unsafe fn sourceInformation(&self) -> CLLocationSourceInformation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceInformation)
    }
}
pub trait CLLocationManager_CLVisitExtensions: Sized + std::ops::Deref {
    unsafe fn startMonitoringVisits(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startMonitoringVisits)
    }
    unsafe fn stopMonitoringVisits(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopMonitoringVisits)
    }
}
pub trait CLPlacemark_ContactsAdditions: Sized + std::ops::Deref {
    unsafe fn postalAddress(&self) -> CNPostalAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postalAddress)
    }
}
pub type CLGeocodeCompletionHandler = *mut ::std::os::raw::c_void;
pub trait CLGeocoder_ContactsAdditions: Sized + std::ops::Deref {
    unsafe fn geocodePostalAddress_completionHandler_(
        &self,
        postalAddress: CNPostalAddress,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geocodePostalAddress : postalAddress, completionHandler : completionHandler)
    }
    unsafe fn geocodePostalAddress_preferredLocale_completionHandler_(
        &self,
        postalAddress: CNPostalAddress,
        locale: NSLocale,
        completionHandler: CLGeocodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geocodePostalAddress : postalAddress, preferredLocale : locale, completionHandler : completionHandler)
    }
}
pub trait NSString_CKRecordValue: Sized + std::ops::Deref {}
pub trait NSNumber_CKRecordValue: Sized + std::ops::Deref {}
pub trait NSArray_CKRecordValue: Sized + std::ops::Deref {}
pub trait NSDate_CKRecordValue: Sized + std::ops::Deref {}
pub trait NSData_CKRecordValue: Sized + std::ops::Deref {}
pub trait CKReference_CKRecordValue: Sized + std::ops::Deref {}
pub trait CKAsset_CKRecordValue: Sized + std::ops::Deref {}
impl CLLocation_CKRecordValue for CLLocation {}
pub trait CLLocation_CKRecordValue: Sized + std::ops::Deref {}
pub trait CKRecord_CKRecordKeyValueSettingConformance: Sized + std::ops::Deref {
    unsafe fn encryptedValues(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptedValues)
    }
}
pub trait CKDatabase_ConvenienceMethods: Sized + std::ops::Deref {
    unsafe fn fetchRecordWithID_completionHandler_(
        &self,
        recordID: CKRecordID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchRecordWithID : recordID, completionHandler : completionHandler)
    }
    unsafe fn saveRecord_completionHandler_(
        &self,
        record: CKRecord,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveRecord : record, completionHandler : completionHandler)
    }
    unsafe fn deleteRecordWithID_completionHandler_(
        &self,
        recordID: CKRecordID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteRecordWithID : recordID, completionHandler : completionHandler)
    }
    unsafe fn performQuery_inZoneWithID_completionHandler_(
        &self,
        query: CKQuery,
        zoneID: CKRecordZoneID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performQuery : query, inZoneWithID : zoneID, completionHandler : completionHandler)
    }
    unsafe fn fetchAllRecordZonesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchAllRecordZonesWithCompletionHandler : completionHandler)
    }
    unsafe fn fetchRecordZoneWithID_completionHandler_(
        &self,
        zoneID: CKRecordZoneID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchRecordZoneWithID : zoneID, completionHandler : completionHandler)
    }
    unsafe fn saveRecordZone_completionHandler_(
        &self,
        zone: CKRecordZone,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveRecordZone : zone, completionHandler : completionHandler)
    }
    unsafe fn deleteRecordZoneWithID_completionHandler_(
        &self,
        zoneID: CKRecordZoneID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteRecordZoneWithID : zoneID, completionHandler : completionHandler)
    }
    unsafe fn fetchSubscriptionWithID_completionHandler_(
        &self,
        subscriptionID: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchSubscriptionWithID : subscriptionID, completionHandler : completionHandler)
    }
    unsafe fn fetchAllSubscriptionsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchAllSubscriptionsWithCompletionHandler : completionHandler)
    }
    unsafe fn saveSubscription_completionHandler_(
        &self,
        subscription: CKSubscription,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveSubscription : subscription, completionHandler : completionHandler)
    }
    unsafe fn deleteSubscriptionWithID_completionHandler_(
        &self,
        subscriptionID: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteSubscriptionWithID : subscriptionID, completionHandler : completionHandler)
    }
}
pub trait CKOperation_CKOperationDeprecated: Sized + std::ops::Deref {
    unsafe fn container(&self) -> CKContainer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, container)
    }
    unsafe fn setContainer_(&self, container: CKContainer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainer : container)
    }
    unsafe fn allowsCellularAccess(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCellularAccess)
    }
    unsafe fn setAllowsCellularAccess_(&self, allowsCellularAccess: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsCellularAccess : allowsCellularAccess)
    }
    unsafe fn isLongLived(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLongLived)
    }
    unsafe fn setLongLived_(&self, longLived: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLongLived : longLived)
    }
    unsafe fn timeoutIntervalForRequest(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeoutIntervalForRequest)
    }
    unsafe fn setTimeoutIntervalForRequest_(&self, timeoutIntervalForRequest: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeoutIntervalForRequest : timeoutIntervalForRequest)
    }
    unsafe fn timeoutIntervalForResource(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeoutIntervalForResource)
    }
    unsafe fn setTimeoutIntervalForResource_(&self, timeoutIntervalForResource: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeoutIntervalForResource : timeoutIntervalForResource)
    }
}
pub trait CKContainer_Database: Sized + std::ops::Deref {
    unsafe fn databaseWithDatabaseScope_(&self, databaseScope: CKDatabaseScope) -> CKDatabase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, databaseWithDatabaseScope : databaseScope)
    }
    unsafe fn privateCloudDatabase(&self) -> CKDatabase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, privateCloudDatabase)
    }
    unsafe fn publicCloudDatabase(&self) -> CKDatabase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicCloudDatabase)
    }
    unsafe fn sharedCloudDatabase(&self) -> CKDatabase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharedCloudDatabase)
    }
}
pub trait CKContainer_AccountStatus: Sized + std::ops::Deref {
    unsafe fn accountStatusWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountStatusWithCompletionHandler : completionHandler)
    }
}
pub type CKApplicationPermissionBlock = *mut ::std::os::raw::c_void;
pub trait CKContainer_ApplicationPermission: Sized + std::ops::Deref {
    unsafe fn statusForApplicationPermission_completionHandler_(
        &self,
        applicationPermission: CKApplicationPermissions,
        completionHandler: CKApplicationPermissionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, statusForApplicationPermission : applicationPermission, completionHandler : completionHandler)
    }
    unsafe fn requestApplicationPermission_completionHandler_(
        &self,
        applicationPermission: CKApplicationPermissions,
        completionHandler: CKApplicationPermissionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestApplicationPermission : applicationPermission, completionHandler : completionHandler)
    }
}
pub trait CKContainer_UserRecords: Sized + std::ops::Deref {
    unsafe fn fetchUserRecordIDWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchUserRecordIDWithCompletionHandler : completionHandler)
    }
    unsafe fn discoverAllIdentitiesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverAllIdentitiesWithCompletionHandler : completionHandler)
    }
    unsafe fn discoverUserIdentityWithEmailAddress_completionHandler_(
        &self,
        email: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverUserIdentityWithEmailAddress : email, completionHandler : completionHandler)
    }
    unsafe fn discoverUserIdentityWithPhoneNumber_completionHandler_(
        &self,
        phoneNumber: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverUserIdentityWithPhoneNumber : phoneNumber, completionHandler : completionHandler)
    }
    unsafe fn discoverUserIdentityWithUserRecordID_completionHandler_(
        &self,
        userRecordID: CKRecordID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverUserIdentityWithUserRecordID : userRecordID, completionHandler : completionHandler)
    }
}
pub trait CKContainer_Sharing: Sized + std::ops::Deref {
    unsafe fn fetchShareParticipantWithEmailAddress_completionHandler_(
        &self,
        emailAddress: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchShareParticipantWithEmailAddress : emailAddress, completionHandler : completionHandler)
    }
    unsafe fn fetchShareParticipantWithPhoneNumber_completionHandler_(
        &self,
        phoneNumber: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchShareParticipantWithPhoneNumber : phoneNumber, completionHandler : completionHandler)
    }
    unsafe fn fetchShareParticipantWithUserRecordID_completionHandler_(
        &self,
        userRecordID: CKRecordID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchShareParticipantWithUserRecordID : userRecordID, completionHandler : completionHandler)
    }
    unsafe fn fetchShareMetadataWithURL_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchShareMetadataWithURL : url, completionHandler : completionHandler)
    }
    unsafe fn acceptShareMetadata_completionHandler_(
        &self,
        metadata: CKShareMetadata,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acceptShareMetadata : metadata, completionHandler : completionHandler)
    }
}
pub trait CKContainer_CKLongLivedOperations: Sized + std::ops::Deref {
    unsafe fn fetchAllLongLivedOperationIDsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchAllLongLivedOperationIDsWithCompletionHandler : completionHandler)
    }
    unsafe fn fetchLongLivedOperationWithID_completionHandler_(
        &self,
        operationID: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchLongLivedOperationWithID : operationID, completionHandler : completionHandler)
    }
}
pub trait CKNotification_DeprecatedAPSProperties: Sized + std::ops::Deref {
    unsafe fn alertBody(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertBody)
    }
    unsafe fn alertLocalizationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertLocalizationKey)
    }
    unsafe fn alertLocalizationArgs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertLocalizationArgs)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn titleLocalizationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleLocalizationKey)
    }
    unsafe fn titleLocalizationArgs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleLocalizationArgs)
    }
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn subtitleLocalizationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleLocalizationKey)
    }
    unsafe fn subtitleLocalizationArgs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleLocalizationArgs)
    }
    unsafe fn alertActionLocalizationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertActionLocalizationKey)
    }
    unsafe fn alertLaunchImage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertLaunchImage)
    }
    unsafe fn badge(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, badge)
    }
    unsafe fn soundName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, soundName)
    }
    unsafe fn category(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
}
pub trait CKFetchRecordZoneChangesOperation_Deprecated: Sized + std::ops::Deref {
    unsafe fn initWithRecordZoneIDs_optionsByRecordZoneID_(
        &self,
        recordZoneIDs: NSArray,
        optionsByRecordZoneID: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordZoneIDs : recordZoneIDs, optionsByRecordZoneID : optionsByRecordZoneID)
    }
    unsafe fn optionsByRecordZoneID(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optionsByRecordZoneID)
    }
    unsafe fn setOptionsByRecordZoneID_(&self, optionsByRecordZoneID: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptionsByRecordZoneID : optionsByRecordZoneID)
    }
}
pub type CKSharePreparationHandler = *mut ::std::os::raw::c_void;
pub trait NSItemProvider_CKSharingSupport: Sized + std::ops::Deref {
    unsafe fn registerCKShareWithContainer_allowedSharingOptions_preparationHandler_(
        &self,
        container: CKContainer,
        allowedOptions: CKAllowedSharingOptions,
        preparationHandler: CKSharePreparationHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerCKShareWithContainer : container, allowedSharingOptions : allowedOptions, preparationHandler : preparationHandler)
    }
    unsafe fn registerCKShare_container_allowedSharingOptions_(
        &self,
        share: CKShare,
        container: CKContainer,
        allowedOptions: CKAllowedSharingOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerCKShare : share, container : container, allowedSharingOptions : allowedOptions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPropertyDescription(pub id);
impl std::ops::Deref for NSPropertyDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPropertyDescription {}
impl NSPropertyDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPropertyDescription").unwrap(), alloc) })
    }
}
impl PNSCoding for NSPropertyDescription {}
impl PNSCopying for NSPropertyDescription {}
impl INSObject for NSPropertyDescription {}
impl PNSObject for NSPropertyDescription {}
impl std::convert::TryFrom<NSObject> for NSPropertyDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPropertyDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPropertyDescription").unwrap()) };
        if is_kind_of {
            Ok(NSPropertyDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPropertyDescription")
        }
    }
}
impl INSPropertyDescription for NSPropertyDescription {}
pub trait INSPropertyDescription: Sized + std::ops::Deref {
    unsafe fn setValidationPredicates_withValidationWarnings_(
        &self,
        validationPredicates: NSArray,
        validationWarnings: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValidationPredicates : validationPredicates, withValidationWarnings : validationWarnings)
    }
    unsafe fn entity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn isOptional(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOptional)
    }
    unsafe fn setOptional_(&self, optional: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptional : optional)
    }
    unsafe fn isTransient(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTransient)
    }
    unsafe fn setTransient_(&self, transient: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransient : transient)
    }
    unsafe fn validationPredicates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, validationPredicates)
    }
    unsafe fn validationWarnings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, validationWarnings)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn isIndexed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIndexed)
    }
    unsafe fn setIndexed_(&self, indexed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexed : indexed)
    }
    unsafe fn versionHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionHash)
    }
    unsafe fn versionHashModifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionHashModifier)
    }
    unsafe fn setVersionHashModifier_(&self, versionHashModifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersionHashModifier : versionHashModifier)
    }
    unsafe fn isIndexedBySpotlight(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIndexedBySpotlight)
    }
    unsafe fn setIndexedBySpotlight_(&self, indexedBySpotlight: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexedBySpotlight : indexedBySpotlight)
    }
    unsafe fn isStoredInExternalRecord(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStoredInExternalRecord)
    }
    unsafe fn setStoredInExternalRecord_(&self, storedInExternalRecord: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStoredInExternalRecord : storedInExternalRecord)
    }
    unsafe fn renamingIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renamingIdentifier)
    }
    unsafe fn setRenamingIdentifier_(&self, renamingIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenamingIdentifier : renamingIdentifier)
    }
}
pub type NSAttributeType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAttributeDescription(pub id);
impl std::ops::Deref for NSAttributeDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAttributeDescription {}
impl NSAttributeDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributeDescription").unwrap(), alloc) })
    }
}
impl INSPropertyDescription for NSAttributeDescription {}
impl PNSCoding for NSAttributeDescription {}
impl PNSCopying for NSAttributeDescription {}
impl From<NSAttributeDescription> for NSPropertyDescription {
    fn from(child: NSAttributeDescription) -> NSPropertyDescription {
        NSPropertyDescription(child.0)
    }
}
impl std::convert::TryFrom<NSPropertyDescription> for NSAttributeDescription {
    type Error = &'static str;
    fn try_from(parent: NSPropertyDescription) -> Result<NSAttributeDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAttributeDescription").unwrap()) };
        if is_kind_of {
            Ok(NSAttributeDescription(parent.0))
        } else {
            Err("This NSPropertyDescription cannot be downcasted to NSAttributeDescription")
        }
    }
}
impl INSObject for NSAttributeDescription {}
impl PNSObject for NSAttributeDescription {}
impl INSAttributeDescription for NSAttributeDescription {}
pub trait INSAttributeDescription: Sized + std::ops::Deref {
    unsafe fn attributeType(&self) -> NSAttributeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeType)
    }
    unsafe fn setAttributeType_(&self, attributeType: NSAttributeType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeType : attributeType)
    }
    unsafe fn attributeValueClassName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeValueClassName)
    }
    unsafe fn setAttributeValueClassName_(&self, attributeValueClassName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeValueClassName : attributeValueClassName)
    }
    unsafe fn defaultValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultValue)
    }
    unsafe fn setDefaultValue_(&self, defaultValue: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultValue : defaultValue)
    }
    unsafe fn versionHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionHash)
    }
    unsafe fn valueTransformerName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueTransformerName)
    }
    unsafe fn setValueTransformerName_(&self, valueTransformerName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueTransformerName : valueTransformerName)
    }
    unsafe fn allowsExternalBinaryDataStorage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsExternalBinaryDataStorage)
    }
    unsafe fn setAllowsExternalBinaryDataStorage_(&self, allowsExternalBinaryDataStorage: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsExternalBinaryDataStorage : allowsExternalBinaryDataStorage)
    }
    unsafe fn preservesValueInHistoryOnDeletion(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preservesValueInHistoryOnDeletion)
    }
    unsafe fn setPreservesValueInHistoryOnDeletion_(&self, preservesValueInHistoryOnDeletion: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreservesValueInHistoryOnDeletion : preservesValueInHistoryOnDeletion)
    }
    unsafe fn allowsCloudEncryption(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCloudEncryption)
    }
    unsafe fn setAllowsCloudEncryption_(&self, allowsCloudEncryption: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsCloudEncryption : allowsCloudEncryption)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSDerivedAttributeDescription(pub id);
impl std::ops::Deref for NSDerivedAttributeDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSDerivedAttributeDescription {}
impl NSDerivedAttributeDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSDerivedAttributeDescription").unwrap(), alloc) })
    }
}
impl INSAttributeDescription for NSDerivedAttributeDescription {}
impl From<NSDerivedAttributeDescription> for NSAttributeDescription {
    fn from(child: NSDerivedAttributeDescription) -> NSAttributeDescription {
        NSAttributeDescription(child.0)
    }
}
impl std::convert::TryFrom<NSAttributeDescription> for NSDerivedAttributeDescription {
    type Error = &'static str;
    fn try_from(
        parent: NSAttributeDescription,
    ) -> Result<NSDerivedAttributeDescription, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSDerivedAttributeDescription").unwrap())
        };
        if is_kind_of {
            Ok(NSDerivedAttributeDescription(parent.0))
        } else {
            Err("This NSAttributeDescription cannot be downcasted to NSDerivedAttributeDescription")
        }
    }
}
impl INSPropertyDescription for NSDerivedAttributeDescription {}
impl PNSCoding for NSDerivedAttributeDescription {}
impl PNSCopying for NSDerivedAttributeDescription {}
impl INSObject for NSDerivedAttributeDescription {}
impl PNSObject for NSDerivedAttributeDescription {}
impl INSDerivedAttributeDescription for NSDerivedAttributeDescription {}
pub trait INSDerivedAttributeDescription: Sized + std::ops::Deref {
    unsafe fn derivationExpression(&self) -> NSExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, derivationExpression)
    }
    unsafe fn setDerivationExpression_(&self, derivationExpression: NSExpression)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDerivationExpression : derivationExpression)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSCompositeAttributeDescription(pub id);
impl std::ops::Deref for NSCompositeAttributeDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSCompositeAttributeDescription {}
impl NSCompositeAttributeDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSCompositeAttributeDescription").unwrap(), alloc) })
    }
}
impl INSAttributeDescription for NSCompositeAttributeDescription {}
impl std::convert::TryFrom<NSAttributeDescription> for NSCompositeAttributeDescription {
    type Error = &'static str;
    fn try_from(
        parent: NSAttributeDescription,
    ) -> Result<NSCompositeAttributeDescription, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSCompositeAttributeDescription").unwrap())
        };
        if is_kind_of {
            Ok(NSCompositeAttributeDescription(parent.0))
        } else {
            Err ("This NSAttributeDescription cannot be downcasted to NSCompositeAttributeDescription" ,)
        }
    }
}
impl INSPropertyDescription for NSCompositeAttributeDescription {}
impl PNSCoding for NSCompositeAttributeDescription {}
impl PNSCopying for NSCompositeAttributeDescription {}
impl INSObject for NSCompositeAttributeDescription {}
impl PNSObject for NSCompositeAttributeDescription {}
impl INSCompositeAttributeDescription for NSCompositeAttributeDescription {}
pub trait INSCompositeAttributeDescription: Sized + std::ops::Deref {
    unsafe fn elements(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn setElements_(&self, elements: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElements : elements)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSEntityDescription(pub id);
impl std::ops::Deref for NSEntityDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSEntityDescription {}
impl NSEntityDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSEntityDescription").unwrap(), alloc) })
    }
}
impl PNSCoding for NSEntityDescription {}
impl PNSCopying for NSEntityDescription {}
impl PNSFastEnumeration for NSEntityDescription {}
impl INSObject for NSEntityDescription {}
impl PNSObject for NSEntityDescription {}
impl std::convert::TryFrom<NSObject> for NSEntityDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSEntityDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSEntityDescription").unwrap()) };
        if is_kind_of {
            Ok(NSEntityDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSEntityDescription")
        }
    }
}
impl INSEntityDescription for NSEntityDescription {}
pub trait INSEntityDescription: Sized + std::ops::Deref {
    unsafe fn relationshipsWithDestinationEntity_(&self, entity: NSEntityDescription) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, relationshipsWithDestinationEntity : entity)
    }
    unsafe fn isKindOfEntity_(&self, entity: NSEntityDescription) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isKindOfEntity : entity)
    }
    unsafe fn managedObjectModel(&self) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectModel)
    }
    unsafe fn managedObjectClassName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectClassName)
    }
    unsafe fn setManagedObjectClassName_(&self, managedObjectClassName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManagedObjectClassName : managedObjectClassName)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn isAbstract(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAbstract)
    }
    unsafe fn setAbstract_(&self, abstract_: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAbstract : abstract_)
    }
    unsafe fn subentitiesByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subentitiesByName)
    }
    unsafe fn subentities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subentities)
    }
    unsafe fn setSubentities_(&self, subentities: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubentities : subentities)
    }
    unsafe fn superentity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, superentity)
    }
    unsafe fn propertiesByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertiesByName)
    }
    unsafe fn properties(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn attributesByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributesByName)
    }
    unsafe fn relationshipsByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relationshipsByName)
    }
    unsafe fn versionHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionHash)
    }
    unsafe fn versionHashModifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionHashModifier)
    }
    unsafe fn setVersionHashModifier_(&self, versionHashModifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersionHashModifier : versionHashModifier)
    }
    unsafe fn renamingIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renamingIdentifier)
    }
    unsafe fn setRenamingIdentifier_(&self, renamingIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenamingIdentifier : renamingIdentifier)
    }
    unsafe fn indexes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexes)
    }
    unsafe fn setIndexes_(&self, indexes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexes : indexes)
    }
    unsafe fn uniquenessConstraints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniquenessConstraints)
    }
    unsafe fn setUniquenessConstraints_(&self, uniquenessConstraints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUniquenessConstraints : uniquenessConstraints)
    }
    unsafe fn compoundIndexes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compoundIndexes)
    }
    unsafe fn setCompoundIndexes_(&self, compoundIndexes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompoundIndexes : compoundIndexes)
    }
    unsafe fn coreSpotlightDisplayNameExpression(&self) -> NSExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coreSpotlightDisplayNameExpression)
    }
    unsafe fn setCoreSpotlightDisplayNameExpression_(
        &self,
        coreSpotlightDisplayNameExpression: NSExpression,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCoreSpotlightDisplayNameExpression : coreSpotlightDisplayNameExpression)
    }
    unsafe fn entityForName_inManagedObjectContext_(
        entityName: NSString,
        context: NSManagedObjectContext,
    ) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSEntityDescription").unwrap(), entityForName : entityName, inManagedObjectContext : context)
    }
    unsafe fn insertNewObjectForEntityForName_inManagedObjectContext_(
        entityName: NSString,
        context: NSManagedObjectContext,
    ) -> NSManagedObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSEntityDescription").unwrap(), insertNewObjectForEntityForName : entityName, inManagedObjectContext : context)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFetchedPropertyDescription(pub id);
impl std::ops::Deref for NSFetchedPropertyDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFetchedPropertyDescription {}
impl NSFetchedPropertyDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchedPropertyDescription").unwrap(), alloc) })
    }
}
impl INSPropertyDescription for NSFetchedPropertyDescription {}
impl PNSCoding for NSFetchedPropertyDescription {}
impl PNSCopying for NSFetchedPropertyDescription {}
impl std::convert::TryFrom<NSPropertyDescription> for NSFetchedPropertyDescription {
    type Error = &'static str;
    fn try_from(
        parent: NSPropertyDescription,
    ) -> Result<NSFetchedPropertyDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFetchedPropertyDescription").unwrap()) };
        if is_kind_of {
            Ok(NSFetchedPropertyDescription(parent.0))
        } else {
            Err("This NSPropertyDescription cannot be downcasted to NSFetchedPropertyDescription")
        }
    }
}
impl INSObject for NSFetchedPropertyDescription {}
impl PNSObject for NSFetchedPropertyDescription {}
impl INSFetchedPropertyDescription for NSFetchedPropertyDescription {}
pub trait INSFetchedPropertyDescription: Sized + std::ops::Deref {
    unsafe fn fetchRequest(&self) -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRequest)
    }
    unsafe fn setFetchRequest_(&self, fetchRequest: NSFetchRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchRequest : fetchRequest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSExpressionDescription(pub id);
impl std::ops::Deref for NSExpressionDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSExpressionDescription {}
impl NSExpressionDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSExpressionDescription").unwrap(), alloc) })
    }
}
impl INSPropertyDescription for NSExpressionDescription {}
impl PNSCoding for NSExpressionDescription {}
impl PNSCopying for NSExpressionDescription {}
impl std::convert::TryFrom<NSPropertyDescription> for NSExpressionDescription {
    type Error = &'static str;
    fn try_from(parent: NSPropertyDescription) -> Result<NSExpressionDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSExpressionDescription").unwrap()) };
        if is_kind_of {
            Ok(NSExpressionDescription(parent.0))
        } else {
            Err("This NSPropertyDescription cannot be downcasted to NSExpressionDescription")
        }
    }
}
impl INSObject for NSExpressionDescription {}
impl PNSObject for NSExpressionDescription {}
impl INSExpressionDescription for NSExpressionDescription {}
pub trait INSExpressionDescription: Sized + std::ops::Deref {
    unsafe fn expression(&self) -> NSExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expression)
    }
    unsafe fn setExpression_(&self, expression: NSExpression)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExpression : expression)
    }
    unsafe fn expressionResultType(&self) -> NSAttributeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expressionResultType)
    }
    unsafe fn setExpressionResultType_(&self, expressionResultType: NSAttributeType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExpressionResultType : expressionResultType)
    }
}
pub type NSDeleteRule = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSRelationshipDescription(pub id);
impl std::ops::Deref for NSRelationshipDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSRelationshipDescription {}
impl NSRelationshipDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSRelationshipDescription").unwrap(), alloc) })
    }
}
impl INSPropertyDescription for NSRelationshipDescription {}
impl PNSCoding for NSRelationshipDescription {}
impl PNSCopying for NSRelationshipDescription {}
impl std::convert::TryFrom<NSPropertyDescription> for NSRelationshipDescription {
    type Error = &'static str;
    fn try_from(parent: NSPropertyDescription) -> Result<NSRelationshipDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSRelationshipDescription").unwrap()) };
        if is_kind_of {
            Ok(NSRelationshipDescription(parent.0))
        } else {
            Err("This NSPropertyDescription cannot be downcasted to NSRelationshipDescription")
        }
    }
}
impl INSObject for NSRelationshipDescription {}
impl PNSObject for NSRelationshipDescription {}
impl INSRelationshipDescription for NSRelationshipDescription {}
pub trait INSRelationshipDescription: Sized + std::ops::Deref {
    unsafe fn destinationEntity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationEntity)
    }
    unsafe fn setDestinationEntity_(&self, destinationEntity: NSEntityDescription)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationEntity : destinationEntity)
    }
    unsafe fn inverseRelationship(&self) -> NSRelationshipDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inverseRelationship)
    }
    unsafe fn setInverseRelationship_(&self, inverseRelationship: NSRelationshipDescription)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInverseRelationship : inverseRelationship)
    }
    unsafe fn maxCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCount)
    }
    unsafe fn setMaxCount_(&self, maxCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxCount : maxCount)
    }
    unsafe fn minCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minCount)
    }
    unsafe fn setMinCount_(&self, minCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinCount : minCount)
    }
    unsafe fn deleteRule(&self) -> NSDeleteRule
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteRule)
    }
    unsafe fn setDeleteRule_(&self, deleteRule: NSDeleteRule)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeleteRule : deleteRule)
    }
    unsafe fn isToMany(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isToMany)
    }
    unsafe fn versionHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionHash)
    }
    unsafe fn isOrdered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOrdered)
    }
    unsafe fn setOrdered_(&self, ordered: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrdered : ordered)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFetchIndexDescription(pub id);
impl std::ops::Deref for NSFetchIndexDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFetchIndexDescription {}
impl NSFetchIndexDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchIndexDescription").unwrap(), alloc) })
    }
}
impl PNSCoding for NSFetchIndexDescription {}
impl PNSCopying for NSFetchIndexDescription {}
impl INSObject for NSFetchIndexDescription {}
impl PNSObject for NSFetchIndexDescription {}
impl std::convert::TryFrom<NSObject> for NSFetchIndexDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFetchIndexDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFetchIndexDescription").unwrap()) };
        if is_kind_of {
            Ok(NSFetchIndexDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFetchIndexDescription")
        }
    }
}
impl INSFetchIndexDescription for NSFetchIndexDescription {}
pub trait INSFetchIndexDescription: Sized + std::ops::Deref {
    unsafe fn initWithName_elements_(&self, name: NSString, elements: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, elements : elements)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn elements(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn setElements_(&self, elements: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElements : elements)
    }
    unsafe fn entity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn partialIndexPredicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, partialIndexPredicate)
    }
    unsafe fn setPartialIndexPredicate_(&self, partialIndexPredicate: NSPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPartialIndexPredicate : partialIndexPredicate)
    }
}
pub type NSFetchIndexElementType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFetchIndexElementDescription(pub id);
impl std::ops::Deref for NSFetchIndexElementDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFetchIndexElementDescription {}
impl NSFetchIndexElementDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchIndexElementDescription").unwrap(), alloc) })
    }
}
impl PNSCoding for NSFetchIndexElementDescription {}
impl PNSCopying for NSFetchIndexElementDescription {}
impl INSObject for NSFetchIndexElementDescription {}
impl PNSObject for NSFetchIndexElementDescription {}
impl std::convert::TryFrom<NSObject> for NSFetchIndexElementDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFetchIndexElementDescription, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFetchIndexElementDescription").unwrap())
        };
        if is_kind_of {
            Ok(NSFetchIndexElementDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFetchIndexElementDescription")
        }
    }
}
impl INSFetchIndexElementDescription for NSFetchIndexElementDescription {}
pub trait INSFetchIndexElementDescription: Sized + std::ops::Deref {
    unsafe fn initWithProperty_collationType_(
        &self,
        property: NSPropertyDescription,
        collationType: NSFetchIndexElementType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProperty : property, collationType : collationType)
    }
    unsafe fn property(&self) -> NSPropertyDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, property)
    }
    unsafe fn propertyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertyName)
    }
    unsafe fn collationType(&self) -> NSFetchIndexElementType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collationType)
    }
    unsafe fn setCollationType_(&self, collationType: NSFetchIndexElementType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCollationType : collationType)
    }
    unsafe fn isAscending(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAscending)
    }
    unsafe fn setAscending_(&self, ascending: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAscending : ascending)
    }
    unsafe fn indexDescription(&self) -> NSFetchIndexDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexDescription)
    }
}
pub type NSPersistentStoreRequestType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentStoreRequest(pub id);
impl std::ops::Deref for NSPersistentStoreRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentStoreRequest {}
impl NSPersistentStoreRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for NSPersistentStoreRequest {}
impl INSObject for NSPersistentStoreRequest {}
impl PNSObject for NSPersistentStoreRequest {}
impl std::convert::TryFrom<NSObject> for NSPersistentStoreRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentStoreRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentStoreRequest").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentStoreRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentStoreRequest")
        }
    }
}
impl INSPersistentStoreRequest for NSPersistentStoreRequest {}
pub trait INSPersistentStoreRequest: Sized + std::ops::Deref {
    unsafe fn affectedStores(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, affectedStores)
    }
    unsafe fn setAffectedStores_(&self, affectedStores: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAffectedStores : affectedStores)
    }
    unsafe fn requestType(&self) -> NSPersistentStoreRequestType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestType)
    }
}
pub type NSSnapshotEventType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSManagedObject(pub id);
impl std::ops::Deref for NSManagedObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSManagedObject {}
impl NSManagedObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObject").unwrap(), alloc) })
    }
}
impl INSObject for NSManagedObject {}
impl PNSObject for NSManagedObject {}
impl std::convert::TryFrom<NSObject> for NSManagedObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSManagedObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSManagedObject").unwrap()) };
        if is_kind_of {
            Ok(NSManagedObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSManagedObject")
        }
    }
}
impl INSManagedObject for NSManagedObject {}
pub trait INSManagedObject: Sized + std::ops::Deref {
    unsafe fn initWithEntity_insertIntoManagedObjectContext_(
        &self,
        entity: NSEntityDescription,
        context: NSManagedObjectContext,
    ) -> NSManagedObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntity : entity, insertIntoManagedObjectContext : context)
    }
    unsafe fn initWithContext_(&self, moc: NSManagedObjectContext) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContext : moc)
    }
    unsafe fn hasFaultForRelationshipNamed_(&self, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasFaultForRelationshipNamed : key)
    }
    unsafe fn objectIDsForRelationshipNamed_(&self, key: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectIDsForRelationshipNamed : key)
    }
    unsafe fn willAccessValueForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willAccessValueForKey : key)
    }
    unsafe fn didAccessValueForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didAccessValueForKey : key)
    }
    unsafe fn willChangeValueForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willChangeValueForKey : key)
    }
    unsafe fn didChangeValueForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didChangeValueForKey : key)
    }
    unsafe fn willChangeValueForKey_withSetMutation_usingObjects_(
        &self,
        inKey: NSString,
        inMutationKind: NSKeyValueSetMutationKind,
        inObjects: NSSet,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willChangeValueForKey : inKey, withSetMutation : inMutationKind, usingObjects : inObjects)
    }
    unsafe fn didChangeValueForKey_withSetMutation_usingObjects_(
        &self,
        inKey: NSString,
        inMutationKind: NSKeyValueSetMutationKind,
        inObjects: NSSet,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didChangeValueForKey : inKey, withSetMutation : inMutationKind, usingObjects : inObjects)
    }
    unsafe fn awakeFromFetch(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, awakeFromFetch)
    }
    unsafe fn awakeFromInsert(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, awakeFromInsert)
    }
    unsafe fn awakeFromSnapshotEvents_(&self, flags: NSSnapshotEventType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, awakeFromSnapshotEvents : flags)
    }
    unsafe fn prepareForDeletion(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareForDeletion)
    }
    unsafe fn willSave(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willSave)
    }
    unsafe fn didSave(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didSave)
    }
    unsafe fn willTurnIntoFault(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willTurnIntoFault)
    }
    unsafe fn didTurnIntoFault(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didTurnIntoFault)
    }
    unsafe fn valueForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForKey : key)
    }
    unsafe fn setValue_forKey_(&self, value: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forKey : key)
    }
    unsafe fn primitiveValueForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, primitiveValueForKey : key)
    }
    unsafe fn setPrimitiveValue_forKey_(&self, value: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveValue : value, forKey : key)
    }
    unsafe fn committedValuesForKeys_(&self, keys: NSArray) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, committedValuesForKeys : keys)
    }
    unsafe fn changedValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedValues)
    }
    unsafe fn changedValuesForCurrentEvent(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedValuesForCurrentEvent)
    }
    unsafe fn validateValue_forKey_error_(
        &self,
        value: *mut id,
        key: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateValue : value, forKey : key, error : error)
    }
    unsafe fn validateForDelete_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateForDelete : error)
    }
    unsafe fn validateForInsert_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateForInsert : error)
    }
    unsafe fn validateForUpdate_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateForUpdate : error)
    }
    unsafe fn setObservationInfo_(&self, inObservationInfo: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObservationInfo : inObservationInfo)
    }
    unsafe fn observationInfo(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, observationInfo)
    }
    unsafe fn managedObjectContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectContext)
    }
    unsafe fn entity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn objectID(&self) -> NSManagedObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectID)
    }
    unsafe fn isInserted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInserted)
    }
    unsafe fn isUpdated(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUpdated)
    }
    unsafe fn isDeleted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDeleted)
    }
    unsafe fn hasChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasChanges)
    }
    unsafe fn hasPersistentChangedValues(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasPersistentChangedValues)
    }
    unsafe fn isFault(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFault)
    }
    unsafe fn faultingState(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, faultingState)
    }
    unsafe fn class_entity() -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObject").unwrap(), entity)
    }
    unsafe fn fetchRequest() -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObject").unwrap(), fetchRequest)
    }
    unsafe fn contextShouldIgnoreUnmodeledPropertyChanges() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObject").unwrap(), contextShouldIgnoreUnmodeledPropertyChanges)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSManagedObjectID(pub id);
impl std::ops::Deref for NSManagedObjectID {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSManagedObjectID {}
impl NSManagedObjectID {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectID").unwrap(), alloc) })
    }
}
impl PNSCopying for NSManagedObjectID {}
impl INSObject for NSManagedObjectID {}
impl PNSObject for NSManagedObjectID {}
impl std::convert::TryFrom<NSObject> for NSManagedObjectID {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSManagedObjectID, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSManagedObjectID").unwrap()) };
        if is_kind_of {
            Ok(NSManagedObjectID(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSManagedObjectID")
        }
    }
}
impl INSManagedObjectID for NSManagedObjectID {}
pub trait INSManagedObjectID: Sized + std::ops::Deref {
    unsafe fn URIRepresentation(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URIRepresentation)
    }
    unsafe fn entity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn persistentStore(&self) -> NSPersistentStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentStore)
    }
    unsafe fn isTemporaryID(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTemporaryID)
    }
}
pub type NSFetchRequestResultType = NSUInteger;
pub trait PNSFetchRequestResult: Sized + std::ops::Deref {}
pub trait NSNumber_NSFetchedResultSupport: Sized + std::ops::Deref {}
pub trait NSDictionary_NSFetchedResultSupport: Sized + std::ops::Deref {}
impl NSManagedObject_NSFetchedResultSupport for NSManagedObject {}
pub trait NSManagedObject_NSFetchedResultSupport: Sized + std::ops::Deref {}
impl NSManagedObjectID_NSFetchedResultSupport for NSManagedObjectID {}
pub trait NSManagedObjectID_NSFetchedResultSupport: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFetchRequest(pub id);
impl std::ops::Deref for NSFetchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFetchRequest {}
impl NSFetchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchRequest").unwrap(), alloc) })
    }
}
impl PNSCoding for NSFetchRequest {}
impl PNSCopying for NSFetchRequest {}
impl INSPersistentStoreRequest for NSFetchRequest {}
impl From<NSFetchRequest> for NSPersistentStoreRequest {
    fn from(child: NSFetchRequest) -> NSPersistentStoreRequest {
        NSPersistentStoreRequest(child.0)
    }
}
impl std::convert::TryFrom<NSPersistentStoreRequest> for NSFetchRequest {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreRequest) -> Result<NSFetchRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFetchRequest").unwrap()) };
        if is_kind_of {
            Ok(NSFetchRequest(parent.0))
        } else {
            Err("This NSPersistentStoreRequest cannot be downcasted to NSFetchRequest")
        }
    }
}
impl INSObject for NSFetchRequest {}
impl PNSObject for NSFetchRequest {}
impl<ResultType: 'static> INSFetchRequest<ResultType> for NSFetchRequest {}
pub trait INSFetchRequest<ResultType: 'static>: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEntityName_(&self, entityName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntityName : entityName)
    }
    unsafe fn execute_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, execute : error)
    }
    unsafe fn entity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn setEntity_(&self, entity: NSEntityDescription)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntity : entity)
    }
    unsafe fn entityName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityName)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
    }
    unsafe fn setPredicate_(&self, predicate: NSPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPredicate : predicate)
    }
    unsafe fn sortDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortDescriptors)
    }
    unsafe fn setSortDescriptors_(&self, sortDescriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSortDescriptors : sortDescriptors)
    }
    unsafe fn fetchLimit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchLimit)
    }
    unsafe fn setFetchLimit_(&self, fetchLimit: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchLimit : fetchLimit)
    }
    unsafe fn affectedStores(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, affectedStores)
    }
    unsafe fn setAffectedStores_(&self, affectedStores: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAffectedStores : affectedStores)
    }
    unsafe fn resultType(&self) -> NSFetchRequestResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
    unsafe fn setResultType_(&self, resultType: NSFetchRequestResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultType : resultType)
    }
    unsafe fn includesSubentities(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includesSubentities)
    }
    unsafe fn setIncludesSubentities_(&self, includesSubentities: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludesSubentities : includesSubentities)
    }
    unsafe fn includesPropertyValues(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includesPropertyValues)
    }
    unsafe fn setIncludesPropertyValues_(&self, includesPropertyValues: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludesPropertyValues : includesPropertyValues)
    }
    unsafe fn returnsObjectsAsFaults(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, returnsObjectsAsFaults)
    }
    unsafe fn setReturnsObjectsAsFaults_(&self, returnsObjectsAsFaults: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReturnsObjectsAsFaults : returnsObjectsAsFaults)
    }
    unsafe fn relationshipKeyPathsForPrefetching(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relationshipKeyPathsForPrefetching)
    }
    unsafe fn setRelationshipKeyPathsForPrefetching_(
        &self,
        relationshipKeyPathsForPrefetching: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelationshipKeyPathsForPrefetching : relationshipKeyPathsForPrefetching)
    }
    unsafe fn includesPendingChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includesPendingChanges)
    }
    unsafe fn setIncludesPendingChanges_(&self, includesPendingChanges: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludesPendingChanges : includesPendingChanges)
    }
    unsafe fn returnsDistinctResults(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, returnsDistinctResults)
    }
    unsafe fn setReturnsDistinctResults_(&self, returnsDistinctResults: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReturnsDistinctResults : returnsDistinctResults)
    }
    unsafe fn propertiesToFetch(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertiesToFetch)
    }
    unsafe fn setPropertiesToFetch_(&self, propertiesToFetch: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertiesToFetch : propertiesToFetch)
    }
    unsafe fn fetchOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchOffset)
    }
    unsafe fn setFetchOffset_(&self, fetchOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchOffset : fetchOffset)
    }
    unsafe fn fetchBatchSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchBatchSize)
    }
    unsafe fn setFetchBatchSize_(&self, fetchBatchSize: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchBatchSize : fetchBatchSize)
    }
    unsafe fn shouldRefreshRefetchedObjects(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldRefreshRefetchedObjects)
    }
    unsafe fn setShouldRefreshRefetchedObjects_(&self, shouldRefreshRefetchedObjects: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldRefreshRefetchedObjects : shouldRefreshRefetchedObjects)
    }
    unsafe fn propertiesToGroupBy(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertiesToGroupBy)
    }
    unsafe fn setPropertiesToGroupBy_(&self, propertiesToGroupBy: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertiesToGroupBy : propertiesToGroupBy)
    }
    unsafe fn havingPredicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, havingPredicate)
    }
    unsafe fn setHavingPredicate_(&self, havingPredicate: NSPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHavingPredicate : havingPredicate)
    }
    unsafe fn fetchRequestWithEntityName_(entityName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchRequest").unwrap(), fetchRequestWithEntityName : entityName)
    }
}
pub type NSPersistentStoreAsynchronousFetchResultCompletionBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAsynchronousFetchRequest(pub id);
impl std::ops::Deref for NSAsynchronousFetchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAsynchronousFetchRequest {}
impl NSAsynchronousFetchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAsynchronousFetchRequest").unwrap(), alloc) })
    }
}
impl INSPersistentStoreRequest for NSAsynchronousFetchRequest {}
impl PNSCopying for NSAsynchronousFetchRequest {}
impl std::convert::TryFrom<NSPersistentStoreRequest> for NSAsynchronousFetchRequest {
    type Error = &'static str;
    fn try_from(
        parent: NSPersistentStoreRequest,
    ) -> Result<NSAsynchronousFetchRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAsynchronousFetchRequest").unwrap()) };
        if is_kind_of {
            Ok(NSAsynchronousFetchRequest(parent.0))
        } else {
            Err("This NSPersistentStoreRequest cannot be downcasted to NSAsynchronousFetchRequest")
        }
    }
}
impl INSObject for NSAsynchronousFetchRequest {}
impl PNSObject for NSAsynchronousFetchRequest {}
impl<ResultType: 'static> INSAsynchronousFetchRequest<ResultType> for NSAsynchronousFetchRequest {}
pub trait INSAsynchronousFetchRequest<ResultType: 'static>: Sized + std::ops::Deref {
    unsafe fn initWithFetchRequest_completionBlock_(
        &self,
        request: NSFetchRequest,
        blk: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFetchRequest : request, completionBlock : blk)
    }
    unsafe fn fetchRequest(&self) -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRequest)
    }
    unsafe fn completionBlock(&self) -> NSPersistentStoreAsynchronousFetchResultCompletionBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionBlock)
    }
    unsafe fn estimatedResultCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, estimatedResultCount)
    }
    unsafe fn setEstimatedResultCount_(&self, estimatedResultCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEstimatedResultCount : estimatedResultCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFetchRequestExpression(pub id);
impl std::ops::Deref for NSFetchRequestExpression {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFetchRequestExpression {}
impl NSFetchRequestExpression {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchRequestExpression").unwrap(), alloc) })
    }
}
impl INSExpression for NSFetchRequestExpression {}
impl PNSSecureCoding for NSFetchRequestExpression {}
impl PNSCopying for NSFetchRequestExpression {}
impl std::convert::TryFrom<NSExpression> for NSFetchRequestExpression {
    type Error = &'static str;
    fn try_from(parent: NSExpression) -> Result<NSFetchRequestExpression, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFetchRequestExpression").unwrap()) };
        if is_kind_of {
            Ok(NSFetchRequestExpression(parent.0))
        } else {
            Err("This NSExpression cannot be downcasted to NSFetchRequestExpression")
        }
    }
}
impl INSObject for NSFetchRequestExpression {}
impl PNSObject for NSFetchRequestExpression {}
impl INSFetchRequestExpression for NSFetchRequestExpression {}
pub trait INSFetchRequestExpression: Sized + std::ops::Deref {
    unsafe fn requestExpression(&self) -> NSExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestExpression)
    }
    unsafe fn contextExpression(&self) -> NSExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contextExpression)
    }
    unsafe fn isCountOnlyRequest(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCountOnlyRequest)
    }
    unsafe fn expressionForFetch_context_countOnly_(
        fetch: NSExpression,
        context: NSExpression,
        countFlag: BOOL,
    ) -> NSExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchRequestExpression").unwrap(), expressionForFetch : fetch, context : context, countOnly : countFlag)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSManagedObjectModel(pub id);
impl std::ops::Deref for NSManagedObjectModel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSManagedObjectModel {}
impl NSManagedObjectModel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectModel").unwrap(), alloc) })
    }
}
impl PNSCoding for NSManagedObjectModel {}
impl PNSCopying for NSManagedObjectModel {}
impl PNSFastEnumeration for NSManagedObjectModel {}
impl INSObject for NSManagedObjectModel {}
impl PNSObject for NSManagedObjectModel {}
impl std::convert::TryFrom<NSObject> for NSManagedObjectModel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSManagedObjectModel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSManagedObjectModel").unwrap()) };
        if is_kind_of {
            Ok(NSManagedObjectModel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSManagedObjectModel")
        }
    }
}
impl INSManagedObjectModel for NSManagedObjectModel {}
pub trait INSManagedObjectModel: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithContentsOfURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url)
    }
    unsafe fn entitiesForConfiguration_(&self, configuration: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, entitiesForConfiguration : configuration)
    }
    unsafe fn setEntities_forConfiguration_(&self, entities: NSArray, configuration: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntities : entities, forConfiguration : configuration)
    }
    unsafe fn setFetchRequestTemplate_forName_(
        &self,
        fetchRequestTemplate: NSFetchRequest,
        name: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchRequestTemplate : fetchRequestTemplate, forName : name)
    }
    unsafe fn fetchRequestTemplateForName_(&self, name: NSString) -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchRequestTemplateForName : name)
    }
    unsafe fn fetchRequestFromTemplateWithName_substitutionVariables_(
        &self,
        name: NSString,
        variables: NSDictionary,
    ) -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchRequestFromTemplateWithName : name, substitutionVariables : variables)
    }
    unsafe fn isConfiguration_compatibleWithStoreMetadata_(
        &self,
        configuration: NSString,
        metadata: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isConfiguration : configuration, compatibleWithStoreMetadata : metadata)
    }
    unsafe fn entitiesByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entitiesByName)
    }
    unsafe fn entities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entities)
    }
    unsafe fn setEntities_(&self, entities: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntities : entities)
    }
    unsafe fn configurations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurations)
    }
    unsafe fn localizationDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizationDictionary)
    }
    unsafe fn setLocalizationDictionary_(&self, localizationDictionary: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizationDictionary : localizationDictionary)
    }
    unsafe fn fetchRequestTemplatesByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRequestTemplatesByName)
    }
    unsafe fn versionIdentifiers(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionIdentifiers)
    }
    unsafe fn setVersionIdentifiers_(&self, versionIdentifiers: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersionIdentifiers : versionIdentifiers)
    }
    unsafe fn entityVersionHashesByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityVersionHashesByName)
    }
    unsafe fn versionChecksum(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionChecksum)
    }
    unsafe fn mergedModelFromBundles_(bundles: NSArray) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectModel").unwrap(), mergedModelFromBundles : bundles)
    }
    unsafe fn modelByMergingModels_(models: NSArray) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectModel").unwrap(), modelByMergingModels : models)
    }
    unsafe fn mergedModelFromBundles_forStoreMetadata_(
        bundles: NSArray,
        metadata: NSDictionary,
    ) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectModel").unwrap(), mergedModelFromBundles : bundles, forStoreMetadata : metadata)
    }
    unsafe fn modelByMergingModels_forStoreMetadata_(
        models: NSArray,
        metadata: NSDictionary,
    ) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectModel").unwrap(), modelByMergingModels : models, forStoreMetadata : metadata)
    }
    unsafe fn checksumsForVersionedModelAtURL_error_(
        modelURL: NSURL,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectModel").unwrap(), checksumsForVersionedModelAtURL : modelURL, error : error)
    }
}
pub type NSManagedObjectContextConcurrencyType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSManagedObjectContext(pub id);
impl std::ops::Deref for NSManagedObjectContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSManagedObjectContext {}
impl NSManagedObjectContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectContext").unwrap(), alloc) })
    }
}
impl PNSCoding for NSManagedObjectContext {}
impl PNSLocking for NSManagedObjectContext {}
impl INSObject for NSManagedObjectContext {}
impl PNSObject for NSManagedObjectContext {}
impl std::convert::TryFrom<NSObject> for NSManagedObjectContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSManagedObjectContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSManagedObjectContext").unwrap()) };
        if is_kind_of {
            Ok(NSManagedObjectContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSManagedObjectContext")
        }
    }
}
impl INSManagedObjectContext for NSManagedObjectContext {}
pub trait INSManagedObjectContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithConcurrencyType_(
        &self,
        ct: NSManagedObjectContextConcurrencyType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConcurrencyType : ct)
    }
    unsafe fn performBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performBlock : block)
    }
    unsafe fn performBlockAndWait_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performBlockAndWait : block)
    }
    unsafe fn objectRegisteredForID_(&self, objectID: NSManagedObjectID) -> NSManagedObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectRegisteredForID : objectID)
    }
    unsafe fn objectWithID_(&self, objectID: NSManagedObjectID) -> NSManagedObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectWithID : objectID)
    }
    unsafe fn existingObjectWithID_error_(
        &self,
        objectID: NSManagedObjectID,
        error: *mut NSError,
    ) -> NSManagedObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, existingObjectWithID : objectID, error : error)
    }
    unsafe fn executeFetchRequest_error_(
        &self,
        request: NSFetchRequest,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeFetchRequest : request, error : error)
    }
    unsafe fn countForFetchRequest_error_(
        &self,
        request: NSFetchRequest,
        error: *mut NSError,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, countForFetchRequest : request, error : error)
    }
    unsafe fn executeRequest_error_(
        &self,
        request: NSPersistentStoreRequest,
        error: *mut NSError,
    ) -> NSPersistentStoreResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeRequest : request, error : error)
    }
    unsafe fn insertObject_(&self, object: NSManagedObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertObject : object)
    }
    unsafe fn deleteObject_(&self, object: NSManagedObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteObject : object)
    }
    unsafe fn refreshObject_mergeChanges_(&self, object: NSManagedObject, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, refreshObject : object, mergeChanges : flag)
    }
    unsafe fn detectConflictsForObject_(&self, object: NSManagedObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectConflictsForObject : object)
    }
    unsafe fn observeValueForKeyPath_ofObject_change_context_(
        &self,
        keyPath: NSString,
        object: id,
        change: NSDictionary,
        context: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, observeValueForKeyPath : keyPath, ofObject : object, change : change, context : context)
    }
    unsafe fn processPendingChanges(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processPendingChanges)
    }
    unsafe fn assignObject_toPersistentStore_(&self, object: id, store: NSPersistentStore)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assignObject : object, toPersistentStore : store)
    }
    unsafe fn undo(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, undo)
    }
    unsafe fn redo(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, redo)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn rollback(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rollback)
    }
    unsafe fn save_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, save : error)
    }
    unsafe fn refreshAllObjects(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refreshAllObjects)
    }
    unsafe fn lock(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lock)
    }
    unsafe fn unlock(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unlock)
    }
    unsafe fn tryLock(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tryLock)
    }
    unsafe fn shouldHandleInaccessibleFault_forObjectID_triggeredByProperty_(
        &self,
        fault: NSManagedObject,
        oid: NSManagedObjectID,
        property: NSPropertyDescription,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldHandleInaccessibleFault : fault, forObjectID : oid, triggeredByProperty : property)
    }
    unsafe fn obtainPermanentIDsForObjects_error_(
        &self,
        objects: NSArray,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, obtainPermanentIDsForObjects : objects, error : error)
    }
    unsafe fn mergeChangesFromContextDidSaveNotification_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mergeChangesFromContextDidSaveNotification : notification)
    }
    unsafe fn setQueryGenerationFromToken_error_(
        &self,
        generation: NSQueryGenerationToken,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueryGenerationFromToken : generation, error : error)
    }
    unsafe fn persistentStoreCoordinator(&self) -> NSPersistentStoreCoordinator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentStoreCoordinator)
    }
    unsafe fn setPersistentStoreCoordinator_(
        &self,
        persistentStoreCoordinator: NSPersistentStoreCoordinator,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPersistentStoreCoordinator : persistentStoreCoordinator)
    }
    unsafe fn parentContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentContext)
    }
    unsafe fn setParentContext_(&self, parentContext: NSManagedObjectContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParentContext : parentContext)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn undoManager(&self) -> NSUndoManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, undoManager)
    }
    unsafe fn setUndoManager_(&self, undoManager: NSUndoManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUndoManager : undoManager)
    }
    unsafe fn hasChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasChanges)
    }
    unsafe fn userInfo(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn concurrencyType(&self) -> NSManagedObjectContextConcurrencyType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, concurrencyType)
    }
    unsafe fn insertedObjects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insertedObjects)
    }
    unsafe fn updatedObjects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updatedObjects)
    }
    unsafe fn deletedObjects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deletedObjects)
    }
    unsafe fn registeredObjects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registeredObjects)
    }
    unsafe fn propagatesDeletesAtEndOfEvent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propagatesDeletesAtEndOfEvent)
    }
    unsafe fn setPropagatesDeletesAtEndOfEvent_(&self, propagatesDeletesAtEndOfEvent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropagatesDeletesAtEndOfEvent : propagatesDeletesAtEndOfEvent)
    }
    unsafe fn retainsRegisteredObjects(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retainsRegisteredObjects)
    }
    unsafe fn setRetainsRegisteredObjects_(&self, retainsRegisteredObjects: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRetainsRegisteredObjects : retainsRegisteredObjects)
    }
    unsafe fn shouldDeleteInaccessibleFaults(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldDeleteInaccessibleFaults)
    }
    unsafe fn setShouldDeleteInaccessibleFaults_(&self, shouldDeleteInaccessibleFaults: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldDeleteInaccessibleFaults : shouldDeleteInaccessibleFaults)
    }
    unsafe fn stalenessInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stalenessInterval)
    }
    unsafe fn setStalenessInterval_(&self, stalenessInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStalenessInterval : stalenessInterval)
    }
    unsafe fn mergePolicy(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mergePolicy)
    }
    unsafe fn setMergePolicy_(&self, mergePolicy: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMergePolicy : mergePolicy)
    }
    unsafe fn queryGenerationToken(&self) -> NSQueryGenerationToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queryGenerationToken)
    }
    unsafe fn automaticallyMergesChangesFromParent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticallyMergesChangesFromParent)
    }
    unsafe fn setAutomaticallyMergesChangesFromParent_(
        &self,
        automaticallyMergesChangesFromParent: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticallyMergesChangesFromParent : automaticallyMergesChangesFromParent)
    }
    unsafe fn transactionAuthor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionAuthor)
    }
    unsafe fn setTransactionAuthor_(&self, transactionAuthor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransactionAuthor : transactionAuthor)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectContext").unwrap(), new)
    }
    unsafe fn mergeChangesFromRemoteContextSave_intoContexts_(
        changeNotificationData: NSDictionary,
        contexts: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectContext").unwrap(), mergeChangesFromRemoteContextSave : changeNotificationData, intoContexts : contexts)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentStoreCoordinator(pub id);
impl std::ops::Deref for NSPersistentStoreCoordinator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentStoreCoordinator {}
impl NSPersistentStoreCoordinator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), alloc) })
    }
}
impl PNSLocking for NSPersistentStoreCoordinator {}
impl INSObject for NSPersistentStoreCoordinator {}
impl PNSObject for NSPersistentStoreCoordinator {}
impl std::convert::TryFrom<NSObject> for NSPersistentStoreCoordinator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentStoreCoordinator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentStoreCoordinator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentStoreCoordinator")
        }
    }
}
impl INSPersistentStoreCoordinator for NSPersistentStoreCoordinator {}
pub trait INSPersistentStoreCoordinator: Sized + std::ops::Deref {
    unsafe fn initWithManagedObjectModel_(&self, model: NSManagedObjectModel) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithManagedObjectModel : model)
    }
    unsafe fn persistentStoreForURL_(&self, URL: NSURL) -> NSPersistentStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreForURL : URL)
    }
    unsafe fn URLForPersistentStore_(&self, store: NSPersistentStore) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLForPersistentStore : store)
    }
    unsafe fn setURL_forPersistentStore_(&self, url: NSURL, store: NSPersistentStore) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : url, forPersistentStore : store)
    }
    unsafe fn addPersistentStoreWithType_configuration_URL_options_error_(
        &self,
        storeType: NSString,
        configuration: NSString,
        storeURL: NSURL,
        options: NSDictionary,
        error: *mut NSError,
    ) -> NSPersistentStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPersistentStoreWithType : storeType, configuration : configuration, URL : storeURL, options : options, error : error)
    }
    unsafe fn addPersistentStoreWithDescription_completionHandler_(
        &self,
        storeDescription: NSPersistentStoreDescription,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPersistentStoreWithDescription : storeDescription, completionHandler : block)
    }
    unsafe fn removePersistentStore_error_(
        &self,
        store: NSPersistentStore,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePersistentStore : store, error : error)
    }
    unsafe fn setMetadata_forPersistentStore_(
        &self,
        metadata: NSDictionary,
        store: NSPersistentStore,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMetadata : metadata, forPersistentStore : store)
    }
    unsafe fn metadataForPersistentStore_(&self, store: NSPersistentStore) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, metadataForPersistentStore : store)
    }
    unsafe fn managedObjectIDForURIRepresentation_(&self, url: NSURL) -> NSManagedObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, managedObjectIDForURIRepresentation : url)
    }
    unsafe fn executeRequest_withContext_error_(
        &self,
        request: NSPersistentStoreRequest,
        context: NSManagedObjectContext,
        error: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeRequest : request, withContext : context, error : error)
    }
    unsafe fn importStoreWithIdentifier_fromExternalRecordsDirectory_toURL_options_withType_error_(
        &self,
        storeIdentifier: NSString,
        externalRecordsURL: NSURL,
        destinationURL: NSURL,
        options: NSDictionary,
        storeType: NSString,
        error: *mut NSError,
    ) -> NSPersistentStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, importStoreWithIdentifier : storeIdentifier, fromExternalRecordsDirectory : externalRecordsURL, toURL : destinationURL, options : options, withType : storeType, error : error)
    }
    unsafe fn migratePersistentStore_toURL_options_withType_error_(
        &self,
        store: NSPersistentStore,
        URL: NSURL,
        options: NSDictionary,
        storeType: NSString,
        error: *mut NSError,
    ) -> NSPersistentStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, migratePersistentStore : store, toURL : URL, options : options, withType : storeType, error : error)
    }
    unsafe fn destroyPersistentStoreAtURL_withType_options_error_(
        &self,
        url: NSURL,
        storeType: NSString,
        options: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, destroyPersistentStoreAtURL : url, withType : storeType, options : options, error : error)
    }
    unsafe fn replacePersistentStoreAtURL_destinationOptions_withPersistentStoreFromURL_sourceOptions_storeType_error_(
        &self,
        destinationURL: NSURL,
        destinationOptions: NSDictionary,
        sourceURL: NSURL,
        sourceOptions: NSDictionary,
        storeType: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replacePersistentStoreAtURL : destinationURL, destinationOptions : destinationOptions, withPersistentStoreFromURL : sourceURL, sourceOptions : sourceOptions, storeType : storeType, error : error)
    }
    unsafe fn performBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performBlock : block)
    }
    unsafe fn performBlockAndWait_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performBlockAndWait : block)
    }
    unsafe fn currentPersistentHistoryTokenFromStores_(
        &self,
        stores: NSArray,
    ) -> NSPersistentHistoryToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, currentPersistentHistoryTokenFromStores : stores)
    }
    unsafe fn finishDeferredLightweightMigration_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishDeferredLightweightMigration : error)
    }
    unsafe fn finishDeferredLightweightMigrationTask_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishDeferredLightweightMigrationTask : error)
    }
    unsafe fn managedObjectIDFromUTF8String_length_(
        &self,
        utf8string: *const ::std::os::raw::c_char,
        len: NSUInteger,
    ) -> NSManagedObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, managedObjectIDFromUTF8String : utf8string, length : len)
    }
    unsafe fn lock(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lock)
    }
    unsafe fn unlock(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unlock)
    }
    unsafe fn tryLock(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tryLock)
    }
    unsafe fn managedObjectModel(&self) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectModel)
    }
    unsafe fn persistentStores(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentStores)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn registerStoreClass_forStoreType_(storeClass: Class, storeType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), registerStoreClass : storeClass, forStoreType : storeType)
    }
    unsafe fn metadataForPersistentStoreOfType_URL_options_error_(
        storeType: NSString,
        url: NSURL,
        options: NSDictionary,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), metadataForPersistentStoreOfType : storeType, URL : url, options : options, error : error)
    }
    unsafe fn setMetadata_forPersistentStoreOfType_URL_options_error_(
        metadata: NSDictionary,
        storeType: NSString,
        url: NSURL,
        options: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), setMetadata : metadata, forPersistentStoreOfType : storeType, URL : url, options : options, error : error)
    }
    unsafe fn elementsDerivedFromExternalRecordURL_(fileURL: NSURL) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), elementsDerivedFromExternalRecordURL : fileURL)
    }
    unsafe fn metadataForPersistentStoreWithURL_error_(
        url: NSURL,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), metadataForPersistentStoreWithURL : url, error : error)
    }
    unsafe fn metadataForPersistentStoreOfType_URL_error_(
        storeType: NSString,
        url: NSURL,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), metadataForPersistentStoreOfType : storeType, URL : url, error : error)
    }
    unsafe fn setMetadata_forPersistentStoreOfType_URL_error_(
        metadata: NSDictionary,
        storeType: NSString,
        url: NSURL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), setMetadata : metadata, forPersistentStoreOfType : storeType, URL : url, error : error)
    }
    unsafe fn removeUbiquitousContentAndPersistentStoreAtURL_options_error_(
        storeURL: NSURL,
        options: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), removeUbiquitousContentAndPersistentStoreAtURL : storeURL, options : options, error : error)
    }
    unsafe fn registeredStoreTypes() -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreCoordinator").unwrap(), registeredStoreTypes)
    }
}
pub type NSPersistentStoreUbiquitousTransitionType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentStore(pub id);
impl std::ops::Deref for NSPersistentStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentStore {}
impl NSPersistentStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStore").unwrap(), alloc) })
    }
}
impl INSObject for NSPersistentStore {}
impl PNSObject for NSPersistentStore {}
impl std::convert::TryFrom<NSObject> for NSPersistentStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentStore").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentStore")
        }
    }
}
impl INSPersistentStore for NSPersistentStore {}
pub trait INSPersistentStore: Sized + std::ops::Deref {
    unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options_(
        &self,
        root: NSPersistentStoreCoordinator,
        name: NSString,
        url: NSURL,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPersistentStoreCoordinator : root, configurationName : name, URL : url, options : options)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn loadMetadata_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadMetadata : error)
    }
    unsafe fn didAddToPersistentStoreCoordinator_(&self, coordinator: NSPersistentStoreCoordinator)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didAddToPersistentStoreCoordinator : coordinator)
    }
    unsafe fn willRemoveFromPersistentStoreCoordinator_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willRemoveFromPersistentStoreCoordinator : coordinator)
    }
    unsafe fn persistentStoreCoordinator(&self) -> NSPersistentStoreCoordinator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentStoreCoordinator)
    }
    unsafe fn configurationName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationName)
    }
    unsafe fn options(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn isReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadOnly)
    }
    unsafe fn setReadOnly_(&self, readOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadOnly : readOnly)
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
    unsafe fn coreSpotlightExporter(&self) -> NSCoreDataCoreSpotlightDelegate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coreSpotlightExporter)
    }
    unsafe fn metadataForPersistentStoreWithURL_error_(
        url: NSURL,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStore").unwrap(), metadataForPersistentStoreWithURL : url, error : error)
    }
    unsafe fn setMetadata_forPersistentStoreWithURL_error_(
        metadata: NSDictionary,
        url: NSURL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStore").unwrap(), setMetadata : metadata, forPersistentStoreWithURL : url, error : error)
    }
    unsafe fn migrationManagerClass() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStore").unwrap(), migrationManagerClass)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAtomicStoreCacheNode(pub id);
impl std::ops::Deref for NSAtomicStoreCacheNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAtomicStoreCacheNode {}
impl NSAtomicStoreCacheNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAtomicStoreCacheNode").unwrap(), alloc) })
    }
}
impl INSObject for NSAtomicStoreCacheNode {}
impl PNSObject for NSAtomicStoreCacheNode {}
impl std::convert::TryFrom<NSObject> for NSAtomicStoreCacheNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSAtomicStoreCacheNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAtomicStoreCacheNode").unwrap()) };
        if is_kind_of {
            Ok(NSAtomicStoreCacheNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSAtomicStoreCacheNode")
        }
    }
}
impl INSAtomicStoreCacheNode for NSAtomicStoreCacheNode {}
pub trait INSAtomicStoreCacheNode: Sized + std::ops::Deref {
    unsafe fn initWithObjectID_(&self, moid: NSManagedObjectID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithObjectID : moid)
    }
    unsafe fn valueForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForKey : key)
    }
    unsafe fn setValue_forKey_(&self, value: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forKey : key)
    }
    unsafe fn objectID(&self) -> NSManagedObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectID)
    }
    unsafe fn propertyCache(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertyCache)
    }
    unsafe fn setPropertyCache_(&self, propertyCache: NSMutableDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertyCache : propertyCache)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAtomicStore(pub id);
impl std::ops::Deref for NSAtomicStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAtomicStore {}
impl NSAtomicStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAtomicStore").unwrap(), alloc) })
    }
}
impl INSPersistentStore for NSAtomicStore {}
impl From<NSAtomicStore> for NSPersistentStore {
    fn from(child: NSAtomicStore) -> NSPersistentStore {
        NSPersistentStore(child.0)
    }
}
impl std::convert::TryFrom<NSPersistentStore> for NSAtomicStore {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStore) -> Result<NSAtomicStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAtomicStore").unwrap()) };
        if is_kind_of {
            Ok(NSAtomicStore(parent.0))
        } else {
            Err("This NSPersistentStore cannot be downcasted to NSAtomicStore")
        }
    }
}
impl INSObject for NSAtomicStore {}
impl PNSObject for NSAtomicStore {}
impl INSAtomicStore for NSAtomicStore {}
pub trait INSAtomicStore: Sized + std::ops::Deref {
    unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        configurationName: NSString,
        url: NSURL,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPersistentStoreCoordinator : coordinator, configurationName : configurationName, URL : url, options : options)
    }
    unsafe fn load_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, load : error)
    }
    unsafe fn save_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, save : error)
    }
    unsafe fn newCacheNodeForManagedObject_(
        &self,
        managedObject: NSManagedObject,
    ) -> NSAtomicStoreCacheNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCacheNodeForManagedObject : managedObject)
    }
    unsafe fn updateCacheNode_fromManagedObject_(
        &self,
        node: NSAtomicStoreCacheNode,
        managedObject: NSManagedObject,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateCacheNode : node, fromManagedObject : managedObject)
    }
    unsafe fn cacheNodes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cacheNodes)
    }
    unsafe fn addCacheNodes_(&self, cacheNodes: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addCacheNodes : cacheNodes)
    }
    unsafe fn willRemoveCacheNodes_(&self, cacheNodes: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willRemoveCacheNodes : cacheNodes)
    }
    unsafe fn cacheNodeForObjectID_(&self, objectID: NSManagedObjectID) -> NSAtomicStoreCacheNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cacheNodeForObjectID : objectID)
    }
    unsafe fn objectIDForEntity_referenceObject_(
        &self,
        entity: NSEntityDescription,
        data: id,
    ) -> NSManagedObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectIDForEntity : entity, referenceObject : data)
    }
    unsafe fn newReferenceObjectForManagedObject_(&self, managedObject: NSManagedObject) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newReferenceObjectForManagedObject : managedObject)
    }
    unsafe fn referenceObjectForObjectID_(&self, objectID: NSManagedObjectID) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, referenceObjectForObjectID : objectID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSEntityMigrationPolicy(pub id);
impl std::ops::Deref for NSEntityMigrationPolicy {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSEntityMigrationPolicy {}
impl NSEntityMigrationPolicy {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSEntityMigrationPolicy").unwrap(), alloc) })
    }
}
impl INSObject for NSEntityMigrationPolicy {}
impl PNSObject for NSEntityMigrationPolicy {}
impl std::convert::TryFrom<NSObject> for NSEntityMigrationPolicy {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSEntityMigrationPolicy, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSEntityMigrationPolicy").unwrap()) };
        if is_kind_of {
            Ok(NSEntityMigrationPolicy(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSEntityMigrationPolicy")
        }
    }
}
impl INSEntityMigrationPolicy for NSEntityMigrationPolicy {}
pub trait INSEntityMigrationPolicy: Sized + std::ops::Deref {
    unsafe fn beginEntityMapping_manager_error_(
        &self,
        mapping: NSEntityMapping,
        manager: NSMigrationManager,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginEntityMapping : mapping, manager : manager, error : error)
    }
    unsafe fn createDestinationInstancesForSourceInstance_entityMapping_manager_error_(
        &self,
        sInstance: NSManagedObject,
        mapping: NSEntityMapping,
        manager: NSMigrationManager,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createDestinationInstancesForSourceInstance : sInstance, entityMapping : mapping, manager : manager, error : error)
    }
    unsafe fn endInstanceCreationForEntityMapping_manager_error_(
        &self,
        mapping: NSEntityMapping,
        manager: NSMigrationManager,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endInstanceCreationForEntityMapping : mapping, manager : manager, error : error)
    }
    unsafe fn createRelationshipsForDestinationInstance_entityMapping_manager_error_(
        &self,
        dInstance: NSManagedObject,
        mapping: NSEntityMapping,
        manager: NSMigrationManager,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createRelationshipsForDestinationInstance : dInstance, entityMapping : mapping, manager : manager, error : error)
    }
    unsafe fn endRelationshipCreationForEntityMapping_manager_error_(
        &self,
        mapping: NSEntityMapping,
        manager: NSMigrationManager,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endRelationshipCreationForEntityMapping : mapping, manager : manager, error : error)
    }
    unsafe fn performCustomValidationForEntityMapping_manager_error_(
        &self,
        mapping: NSEntityMapping,
        manager: NSMigrationManager,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performCustomValidationForEntityMapping : mapping, manager : manager, error : error)
    }
    unsafe fn endEntityMapping_manager_error_(
        &self,
        mapping: NSEntityMapping,
        manager: NSMigrationManager,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endEntityMapping : mapping, manager : manager, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSMappingModel(pub id);
impl std::ops::Deref for NSMappingModel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSMappingModel {}
impl NSMappingModel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSMappingModel").unwrap(), alloc) })
    }
}
impl INSObject for NSMappingModel {}
impl PNSObject for NSMappingModel {}
impl std::convert::TryFrom<NSObject> for NSMappingModel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSMappingModel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSMappingModel").unwrap()) };
        if is_kind_of {
            Ok(NSMappingModel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSMappingModel")
        }
    }
}
impl INSMappingModel for NSMappingModel {}
pub trait INSMappingModel: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url)
    }
    unsafe fn entityMappings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityMappings)
    }
    unsafe fn setEntityMappings_(&self, entityMappings: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntityMappings : entityMappings)
    }
    unsafe fn entityMappingsByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityMappingsByName)
    }
    unsafe fn mappingModelFromBundles_forSourceModel_destinationModel_(
        bundles: NSArray,
        sourceModel: NSManagedObjectModel,
        destinationModel: NSManagedObjectModel,
    ) -> NSMappingModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMappingModel").unwrap(), mappingModelFromBundles : bundles, forSourceModel : sourceModel, destinationModel : destinationModel)
    }
    unsafe fn inferredMappingModelForSourceModel_destinationModel_error_(
        sourceModel: NSManagedObjectModel,
        destinationModel: NSManagedObjectModel,
        error: *mut NSError,
    ) -> NSMappingModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMappingModel").unwrap(), inferredMappingModelForSourceModel : sourceModel, destinationModel : destinationModel, error : error)
    }
}
pub type NSEntityMappingType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSEntityMapping(pub id);
impl std::ops::Deref for NSEntityMapping {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSEntityMapping {}
impl NSEntityMapping {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSEntityMapping").unwrap(), alloc) })
    }
}
impl INSObject for NSEntityMapping {}
impl PNSObject for NSEntityMapping {}
impl std::convert::TryFrom<NSObject> for NSEntityMapping {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSEntityMapping, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSEntityMapping").unwrap()) };
        if is_kind_of {
            Ok(NSEntityMapping(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSEntityMapping")
        }
    }
}
impl INSEntityMapping for NSEntityMapping {}
pub trait INSEntityMapping: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn mappingType(&self) -> NSEntityMappingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mappingType)
    }
    unsafe fn setMappingType_(&self, mappingType: NSEntityMappingType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMappingType : mappingType)
    }
    unsafe fn sourceEntityName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceEntityName)
    }
    unsafe fn setSourceEntityName_(&self, sourceEntityName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceEntityName : sourceEntityName)
    }
    unsafe fn sourceEntityVersionHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceEntityVersionHash)
    }
    unsafe fn setSourceEntityVersionHash_(&self, sourceEntityVersionHash: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceEntityVersionHash : sourceEntityVersionHash)
    }
    unsafe fn destinationEntityName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationEntityName)
    }
    unsafe fn setDestinationEntityName_(&self, destinationEntityName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationEntityName : destinationEntityName)
    }
    unsafe fn destinationEntityVersionHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationEntityVersionHash)
    }
    unsafe fn setDestinationEntityVersionHash_(&self, destinationEntityVersionHash: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationEntityVersionHash : destinationEntityVersionHash)
    }
    unsafe fn attributeMappings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeMappings)
    }
    unsafe fn setAttributeMappings_(&self, attributeMappings: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeMappings : attributeMappings)
    }
    unsafe fn relationshipMappings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relationshipMappings)
    }
    unsafe fn setRelationshipMappings_(&self, relationshipMappings: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelationshipMappings : relationshipMappings)
    }
    unsafe fn sourceExpression(&self) -> NSExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceExpression)
    }
    unsafe fn setSourceExpression_(&self, sourceExpression: NSExpression)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceExpression : sourceExpression)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn entityMigrationPolicyClassName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityMigrationPolicyClassName)
    }
    unsafe fn setEntityMigrationPolicyClassName_(&self, entityMigrationPolicyClassName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntityMigrationPolicyClassName : entityMigrationPolicyClassName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPropertyMapping(pub id);
impl std::ops::Deref for NSPropertyMapping {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPropertyMapping {}
impl NSPropertyMapping {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPropertyMapping").unwrap(), alloc) })
    }
}
impl INSObject for NSPropertyMapping {}
impl PNSObject for NSPropertyMapping {}
impl std::convert::TryFrom<NSObject> for NSPropertyMapping {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPropertyMapping, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPropertyMapping").unwrap()) };
        if is_kind_of {
            Ok(NSPropertyMapping(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPropertyMapping")
        }
    }
}
impl INSPropertyMapping for NSPropertyMapping {}
pub trait INSPropertyMapping: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn valueExpression(&self) -> NSExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueExpression)
    }
    unsafe fn setValueExpression_(&self, valueExpression: NSExpression)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueExpression : valueExpression)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSMigrationManager(pub id);
impl std::ops::Deref for NSMigrationManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSMigrationManager {}
impl NSMigrationManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSMigrationManager").unwrap(), alloc) })
    }
}
impl INSObject for NSMigrationManager {}
impl PNSObject for NSMigrationManager {}
impl std::convert::TryFrom<NSObject> for NSMigrationManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSMigrationManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSMigrationManager").unwrap()) };
        if is_kind_of {
            Ok(NSMigrationManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSMigrationManager")
        }
    }
}
impl INSMigrationManager for NSMigrationManager {}
pub trait INSMigrationManager: Sized + std::ops::Deref {
    unsafe fn initWithSourceModel_destinationModel_(
        &self,
        sourceModel: NSManagedObjectModel,
        destinationModel: NSManagedObjectModel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceModel : sourceModel, destinationModel : destinationModel)
    }
    unsafe fn migrateStoreFromURL_type_options_withMappingModel_toDestinationURL_destinationType_destinationOptions_error_(
        &self,
        sourceURL: NSURL,
        sStoreType: NSString,
        sOptions: NSDictionary,
        mappings: NSMappingModel,
        dURL: NSURL,
        dStoreType: NSString,
        dOptions: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, migrateStoreFromURL : sourceURL, r#type : sStoreType, options : sOptions, withMappingModel : mappings, toDestinationURL : dURL, destinationType : dStoreType, destinationOptions : dOptions, error : error)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn sourceEntityForEntityMapping_(&self, mEntity: NSEntityMapping) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sourceEntityForEntityMapping : mEntity)
    }
    unsafe fn destinationEntityForEntityMapping_(
        &self,
        mEntity: NSEntityMapping,
    ) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, destinationEntityForEntityMapping : mEntity)
    }
    unsafe fn associateSourceInstance_withDestinationInstance_forEntityMapping_(
        &self,
        sourceInstance: NSManagedObject,
        destinationInstance: NSManagedObject,
        entityMapping: NSEntityMapping,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, associateSourceInstance : sourceInstance, withDestinationInstance : destinationInstance, forEntityMapping : entityMapping)
    }
    unsafe fn destinationInstancesForEntityMappingNamed_sourceInstances_(
        &self,
        mappingName: NSString,
        sourceInstances: NSArray,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, destinationInstancesForEntityMappingNamed : mappingName, sourceInstances : sourceInstances)
    }
    unsafe fn sourceInstancesForEntityMappingNamed_destinationInstances_(
        &self,
        mappingName: NSString,
        destinationInstances: NSArray,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sourceInstancesForEntityMappingNamed : mappingName, destinationInstances : destinationInstances)
    }
    unsafe fn cancelMigrationWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelMigrationWithError : error)
    }
    unsafe fn usesStoreSpecificMigrationManager(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesStoreSpecificMigrationManager)
    }
    unsafe fn setUsesStoreSpecificMigrationManager_(&self, usesStoreSpecificMigrationManager: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesStoreSpecificMigrationManager : usesStoreSpecificMigrationManager)
    }
    unsafe fn mappingModel(&self) -> NSMappingModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mappingModel)
    }
    unsafe fn sourceModel(&self) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceModel)
    }
    unsafe fn destinationModel(&self) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationModel)
    }
    unsafe fn sourceContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceContext)
    }
    unsafe fn destinationContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationContext)
    }
    unsafe fn currentEntityMapping(&self) -> NSEntityMapping
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentEntityMapping)
    }
    unsafe fn migrationProgress(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, migrationProgress)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSIncrementalStore(pub id);
impl std::ops::Deref for NSIncrementalStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSIncrementalStore {}
impl NSIncrementalStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSIncrementalStore").unwrap(), alloc) })
    }
}
impl INSPersistentStore for NSIncrementalStore {}
impl std::convert::TryFrom<NSPersistentStore> for NSIncrementalStore {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStore) -> Result<NSIncrementalStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSIncrementalStore").unwrap()) };
        if is_kind_of {
            Ok(NSIncrementalStore(parent.0))
        } else {
            Err("This NSPersistentStore cannot be downcasted to NSIncrementalStore")
        }
    }
}
impl INSObject for NSIncrementalStore {}
impl PNSObject for NSIncrementalStore {}
impl INSIncrementalStore for NSIncrementalStore {}
pub trait INSIncrementalStore: Sized + std::ops::Deref {
    unsafe fn loadMetadata_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadMetadata : error)
    }
    unsafe fn executeRequest_withContext_error_(
        &self,
        request: NSPersistentStoreRequest,
        context: NSManagedObjectContext,
        error: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeRequest : request, withContext : context, error : error)
    }
    unsafe fn newValuesForObjectWithID_withContext_error_(
        &self,
        objectID: NSManagedObjectID,
        context: NSManagedObjectContext,
        error: *mut NSError,
    ) -> NSIncrementalStoreNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newValuesForObjectWithID : objectID, withContext : context, error : error)
    }
    unsafe fn newValueForRelationship_forObjectWithID_withContext_error_(
        &self,
        relationship: NSRelationshipDescription,
        objectID: NSManagedObjectID,
        context: NSManagedObjectContext,
        error: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newValueForRelationship : relationship, forObjectWithID : objectID, withContext : context, error : error)
    }
    unsafe fn obtainPermanentIDsForObjects_error_(
        &self,
        array: NSArray,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, obtainPermanentIDsForObjects : array, error : error)
    }
    unsafe fn managedObjectContextDidRegisterObjectsWithIDs_(&self, objectIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, managedObjectContextDidRegisterObjectsWithIDs : objectIDs)
    }
    unsafe fn managedObjectContextDidUnregisterObjectsWithIDs_(&self, objectIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, managedObjectContextDidUnregisterObjectsWithIDs : objectIDs)
    }
    unsafe fn newObjectIDForEntity_referenceObject_(
        &self,
        entity: NSEntityDescription,
        data: id,
    ) -> NSManagedObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newObjectIDForEntity : entity, referenceObject : data)
    }
    unsafe fn referenceObjectForObjectID_(&self, objectID: NSManagedObjectID) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, referenceObjectForObjectID : objectID)
    }
    unsafe fn identifierForNewStoreAtURL_(storeURL: NSURL) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSIncrementalStore").unwrap(), identifierForNewStoreAtURL : storeURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSIncrementalStoreNode(pub id);
impl std::ops::Deref for NSIncrementalStoreNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSIncrementalStoreNode {}
impl NSIncrementalStoreNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSIncrementalStoreNode").unwrap(), alloc) })
    }
}
impl INSObject for NSIncrementalStoreNode {}
impl PNSObject for NSIncrementalStoreNode {}
impl std::convert::TryFrom<NSObject> for NSIncrementalStoreNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSIncrementalStoreNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSIncrementalStoreNode").unwrap()) };
        if is_kind_of {
            Ok(NSIncrementalStoreNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSIncrementalStoreNode")
        }
    }
}
impl INSIncrementalStoreNode for NSIncrementalStoreNode {}
pub trait INSIncrementalStoreNode: Sized + std::ops::Deref {
    unsafe fn initWithObjectID_withValues_version_(
        &self,
        objectID: NSManagedObjectID,
        values: NSDictionary,
        version: u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithObjectID : objectID, withValues : values, version : version)
    }
    unsafe fn updateWithValues_version_(&self, values: NSDictionary, version: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithValues : values, version : version)
    }
    unsafe fn valueForPropertyDescription_(&self, prop: NSPropertyDescription) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForPropertyDescription : prop)
    }
    unsafe fn objectID(&self) -> NSManagedObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectID)
    }
    unsafe fn version(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
}
pub type NSBatchInsertRequestResultType = NSUInteger;
pub type NSBatchUpdateRequestResultType = NSUInteger;
pub type NSBatchDeleteRequestResultType = NSUInteger;
pub type NSPersistentHistoryResultType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentStoreResult(pub id);
impl std::ops::Deref for NSPersistentStoreResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentStoreResult {}
impl NSPersistentStoreResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreResult").unwrap(), alloc) })
    }
}
impl INSObject for NSPersistentStoreResult {}
impl PNSObject for NSPersistentStoreResult {}
impl std::convert::TryFrom<NSObject> for NSPersistentStoreResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentStoreResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentStoreResult").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentStoreResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentStoreResult")
        }
    }
}
impl INSPersistentStoreResult for NSPersistentStoreResult {}
pub trait INSPersistentStoreResult: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentStoreAsynchronousResult(pub id);
impl std::ops::Deref for NSPersistentStoreAsynchronousResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentStoreAsynchronousResult {}
impl NSPersistentStoreAsynchronousResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreAsynchronousResult").unwrap(), alloc) })
    }
}
impl INSPersistentStoreResult for NSPersistentStoreAsynchronousResult {}
impl From<NSPersistentStoreAsynchronousResult> for NSPersistentStoreResult {
    fn from(child: NSPersistentStoreAsynchronousResult) -> NSPersistentStoreResult {
        NSPersistentStoreResult(child.0)
    }
}
impl std::convert::TryFrom<NSPersistentStoreResult> for NSPersistentStoreAsynchronousResult {
    type Error = &'static str;
    fn try_from(
        parent: NSPersistentStoreResult,
    ) -> Result<NSPersistentStoreAsynchronousResult, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentStoreAsynchronousResult").unwrap())
        };
        if is_kind_of {
            Ok(NSPersistentStoreAsynchronousResult(parent.0))
        } else {
            Err ("This NSPersistentStoreResult cannot be downcasted to NSPersistentStoreAsynchronousResult" ,)
        }
    }
}
impl INSObject for NSPersistentStoreAsynchronousResult {}
impl PNSObject for NSPersistentStoreAsynchronousResult {}
impl INSPersistentStoreAsynchronousResult for NSPersistentStoreAsynchronousResult {}
pub trait INSPersistentStoreAsynchronousResult: Sized + std::ops::Deref {
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn managedObjectContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectContext)
    }
    unsafe fn operationError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operationError)
    }
    unsafe fn progress(&self) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progress)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAsynchronousFetchResult(pub id);
impl std::ops::Deref for NSAsynchronousFetchResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAsynchronousFetchResult {}
impl NSAsynchronousFetchResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAsynchronousFetchResult").unwrap(), alloc) })
    }
}
impl INSPersistentStoreAsynchronousResult for NSAsynchronousFetchResult {}
impl From<NSAsynchronousFetchResult> for NSPersistentStoreAsynchronousResult {
    fn from(child: NSAsynchronousFetchResult) -> NSPersistentStoreAsynchronousResult {
        NSPersistentStoreAsynchronousResult(child.0)
    }
}
impl std::convert::TryFrom<NSPersistentStoreAsynchronousResult> for NSAsynchronousFetchResult {
    type Error = &'static str;
    fn try_from(
        parent: NSPersistentStoreAsynchronousResult,
    ) -> Result<NSAsynchronousFetchResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAsynchronousFetchResult").unwrap()) };
        if is_kind_of {
            Ok(NSAsynchronousFetchResult(parent.0))
        } else {
            Err ("This NSPersistentStoreAsynchronousResult cannot be downcasted to NSAsynchronousFetchResult" ,)
        }
    }
}
impl INSPersistentStoreResult for NSAsynchronousFetchResult {}
impl INSObject for NSAsynchronousFetchResult {}
impl PNSObject for NSAsynchronousFetchResult {}
impl<ResultType: 'static> INSAsynchronousFetchResult<ResultType> for NSAsynchronousFetchResult {}
pub trait INSAsynchronousFetchResult<ResultType: 'static>: Sized + std::ops::Deref {
    unsafe fn fetchRequest(&self) -> NSAsynchronousFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRequest)
    }
    unsafe fn finalResult(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finalResult)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSBatchInsertResult(pub id);
impl std::ops::Deref for NSBatchInsertResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSBatchInsertResult {}
impl NSBatchInsertResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchInsertResult").unwrap(), alloc) })
    }
}
impl INSPersistentStoreResult for NSBatchInsertResult {}
impl std::convert::TryFrom<NSPersistentStoreResult> for NSBatchInsertResult {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreResult) -> Result<NSBatchInsertResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSBatchInsertResult").unwrap()) };
        if is_kind_of {
            Ok(NSBatchInsertResult(parent.0))
        } else {
            Err("This NSPersistentStoreResult cannot be downcasted to NSBatchInsertResult")
        }
    }
}
impl INSObject for NSBatchInsertResult {}
impl PNSObject for NSBatchInsertResult {}
impl INSBatchInsertResult for NSBatchInsertResult {}
pub trait INSBatchInsertResult: Sized + std::ops::Deref {
    unsafe fn result(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, result)
    }
    unsafe fn resultType(&self) -> NSBatchInsertRequestResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSBatchUpdateResult(pub id);
impl std::ops::Deref for NSBatchUpdateResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSBatchUpdateResult {}
impl NSBatchUpdateResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchUpdateResult").unwrap(), alloc) })
    }
}
impl INSPersistentStoreResult for NSBatchUpdateResult {}
impl std::convert::TryFrom<NSPersistentStoreResult> for NSBatchUpdateResult {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreResult) -> Result<NSBatchUpdateResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSBatchUpdateResult").unwrap()) };
        if is_kind_of {
            Ok(NSBatchUpdateResult(parent.0))
        } else {
            Err("This NSPersistentStoreResult cannot be downcasted to NSBatchUpdateResult")
        }
    }
}
impl INSObject for NSBatchUpdateResult {}
impl PNSObject for NSBatchUpdateResult {}
impl INSBatchUpdateResult for NSBatchUpdateResult {}
pub trait INSBatchUpdateResult: Sized + std::ops::Deref {
    unsafe fn result(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, result)
    }
    unsafe fn resultType(&self) -> NSBatchUpdateRequestResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSBatchDeleteResult(pub id);
impl std::ops::Deref for NSBatchDeleteResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSBatchDeleteResult {}
impl NSBatchDeleteResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchDeleteResult").unwrap(), alloc) })
    }
}
impl INSPersistentStoreResult for NSBatchDeleteResult {}
impl std::convert::TryFrom<NSPersistentStoreResult> for NSBatchDeleteResult {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreResult) -> Result<NSBatchDeleteResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSBatchDeleteResult").unwrap()) };
        if is_kind_of {
            Ok(NSBatchDeleteResult(parent.0))
        } else {
            Err("This NSPersistentStoreResult cannot be downcasted to NSBatchDeleteResult")
        }
    }
}
impl INSObject for NSBatchDeleteResult {}
impl PNSObject for NSBatchDeleteResult {}
impl INSBatchDeleteResult for NSBatchDeleteResult {}
pub trait INSBatchDeleteResult: Sized + std::ops::Deref {
    unsafe fn result(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, result)
    }
    unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentHistoryResult(pub id);
impl std::ops::Deref for NSPersistentHistoryResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentHistoryResult {}
impl NSPersistentHistoryResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryResult").unwrap(), alloc) })
    }
}
impl INSPersistentStoreResult for NSPersistentHistoryResult {}
impl std::convert::TryFrom<NSPersistentStoreResult> for NSPersistentHistoryResult {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreResult) -> Result<NSPersistentHistoryResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentHistoryResult").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentHistoryResult(parent.0))
        } else {
            Err("This NSPersistentStoreResult cannot be downcasted to NSPersistentHistoryResult")
        }
    }
}
impl INSObject for NSPersistentHistoryResult {}
impl PNSObject for NSPersistentHistoryResult {}
impl INSPersistentHistoryResult for NSPersistentHistoryResult {}
pub trait INSPersistentHistoryResult: Sized + std::ops::Deref {
    unsafe fn result(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, result)
    }
    unsafe fn resultType(&self) -> NSPersistentHistoryResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
}
pub type NSPersistentCloudKitContainerEventResultType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentCloudKitContainerEventResult(pub id);
impl std::ops::Deref for NSPersistentCloudKitContainerEventResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentCloudKitContainerEventResult {}
impl NSPersistentCloudKitContainerEventResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventResult").unwrap(), alloc) })
    }
}
impl INSPersistentStoreResult for NSPersistentCloudKitContainerEventResult {}
impl std::convert::TryFrom<NSPersistentStoreResult> for NSPersistentCloudKitContainerEventResult {
    type Error = &'static str;
    fn try_from(
        parent: NSPersistentStoreResult,
    ) -> Result<NSPersistentCloudKitContainerEventResult, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventResult").unwrap())
        };
        if is_kind_of {
            Ok(NSPersistentCloudKitContainerEventResult(parent.0))
        } else {
            Err ("This NSPersistentStoreResult cannot be downcasted to NSPersistentCloudKitContainerEventResult" ,)
        }
    }
}
impl INSObject for NSPersistentCloudKitContainerEventResult {}
impl PNSObject for NSPersistentCloudKitContainerEventResult {}
impl INSPersistentCloudKitContainerEventResult for NSPersistentCloudKitContainerEventResult {}
pub trait INSPersistentCloudKitContainerEventResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn result(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, result)
    }
    unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventResult").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSaveChangesRequest(pub id);
impl std::ops::Deref for NSSaveChangesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSaveChangesRequest {}
impl NSSaveChangesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSaveChangesRequest").unwrap(), alloc) })
    }
}
impl INSPersistentStoreRequest for NSSaveChangesRequest {}
impl PNSCopying for NSSaveChangesRequest {}
impl std::convert::TryFrom<NSPersistentStoreRequest> for NSSaveChangesRequest {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreRequest) -> Result<NSSaveChangesRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSaveChangesRequest").unwrap()) };
        if is_kind_of {
            Ok(NSSaveChangesRequest(parent.0))
        } else {
            Err("This NSPersistentStoreRequest cannot be downcasted to NSSaveChangesRequest")
        }
    }
}
impl INSObject for NSSaveChangesRequest {}
impl PNSObject for NSSaveChangesRequest {}
impl INSSaveChangesRequest for NSSaveChangesRequest {}
pub trait INSSaveChangesRequest: Sized + std::ops::Deref {
    unsafe fn initWithInsertedObjects_updatedObjects_deletedObjects_lockedObjects_(
        &self,
        insertedObjects: NSSet,
        updatedObjects: NSSet,
        deletedObjects: NSSet,
        lockedObjects: NSSet,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInsertedObjects : insertedObjects, updatedObjects : updatedObjects, deletedObjects : deletedObjects, lockedObjects : lockedObjects)
    }
    unsafe fn insertedObjects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insertedObjects)
    }
    unsafe fn updatedObjects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updatedObjects)
    }
    unsafe fn deletedObjects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deletedObjects)
    }
    unsafe fn lockedObjects(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lockedObjects)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSBatchUpdateRequest(pub id);
impl std::ops::Deref for NSBatchUpdateRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSBatchUpdateRequest {}
impl NSBatchUpdateRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchUpdateRequest").unwrap(), alloc) })
    }
}
impl INSPersistentStoreRequest for NSBatchUpdateRequest {}
impl PNSCopying for NSBatchUpdateRequest {}
impl std::convert::TryFrom<NSPersistentStoreRequest> for NSBatchUpdateRequest {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreRequest) -> Result<NSBatchUpdateRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSBatchUpdateRequest").unwrap()) };
        if is_kind_of {
            Ok(NSBatchUpdateRequest(parent.0))
        } else {
            Err("This NSPersistentStoreRequest cannot be downcasted to NSBatchUpdateRequest")
        }
    }
}
impl INSObject for NSBatchUpdateRequest {}
impl PNSObject for NSBatchUpdateRequest {}
impl INSBatchUpdateRequest for NSBatchUpdateRequest {}
pub trait INSBatchUpdateRequest: Sized + std::ops::Deref {
    unsafe fn initWithEntityName_(&self, entityName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntityName : entityName)
    }
    unsafe fn initWithEntity_(&self, entity: NSEntityDescription) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntity : entity)
    }
    unsafe fn entityName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityName)
    }
    unsafe fn entity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
    }
    unsafe fn setPredicate_(&self, predicate: NSPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPredicate : predicate)
    }
    unsafe fn includesSubentities(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includesSubentities)
    }
    unsafe fn setIncludesSubentities_(&self, includesSubentities: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludesSubentities : includesSubentities)
    }
    unsafe fn resultType(&self) -> NSBatchUpdateRequestResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
    unsafe fn setResultType_(&self, resultType: NSBatchUpdateRequestResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultType : resultType)
    }
    unsafe fn propertiesToUpdate(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertiesToUpdate)
    }
    unsafe fn setPropertiesToUpdate_(&self, propertiesToUpdate: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertiesToUpdate : propertiesToUpdate)
    }
    unsafe fn batchUpdateRequestWithEntityName_(entityName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchUpdateRequest").unwrap(), batchUpdateRequestWithEntityName : entityName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSBatchDeleteRequest(pub id);
impl std::ops::Deref for NSBatchDeleteRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSBatchDeleteRequest {}
impl NSBatchDeleteRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchDeleteRequest").unwrap(), alloc) })
    }
}
impl INSPersistentStoreRequest for NSBatchDeleteRequest {}
impl PNSCopying for NSBatchDeleteRequest {}
impl std::convert::TryFrom<NSPersistentStoreRequest> for NSBatchDeleteRequest {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreRequest) -> Result<NSBatchDeleteRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSBatchDeleteRequest").unwrap()) };
        if is_kind_of {
            Ok(NSBatchDeleteRequest(parent.0))
        } else {
            Err("This NSPersistentStoreRequest cannot be downcasted to NSBatchDeleteRequest")
        }
    }
}
impl INSObject for NSBatchDeleteRequest {}
impl PNSObject for NSBatchDeleteRequest {}
impl INSBatchDeleteRequest for NSBatchDeleteRequest {}
pub trait INSBatchDeleteRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFetchRequest_(&self, fetch: NSFetchRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFetchRequest : fetch)
    }
    unsafe fn initWithObjectIDs_(&self, objects: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithObjectIDs : objects)
    }
    unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
    unsafe fn setResultType_(&self, resultType: NSBatchDeleteRequestResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultType : resultType)
    }
    unsafe fn fetchRequest(&self) -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRequest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSBatchInsertRequest(pub id);
impl std::ops::Deref for NSBatchInsertRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSBatchInsertRequest {}
impl NSBatchInsertRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchInsertRequest").unwrap(), alloc) })
    }
}
impl INSPersistentStoreRequest for NSBatchInsertRequest {}
impl PNSCopying for NSBatchInsertRequest {}
impl std::convert::TryFrom<NSPersistentStoreRequest> for NSBatchInsertRequest {
    type Error = &'static str;
    fn try_from(parent: NSPersistentStoreRequest) -> Result<NSBatchInsertRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSBatchInsertRequest").unwrap()) };
        if is_kind_of {
            Ok(NSBatchInsertRequest(parent.0))
        } else {
            Err("This NSPersistentStoreRequest cannot be downcasted to NSBatchInsertRequest")
        }
    }
}
impl INSObject for NSBatchInsertRequest {}
impl PNSObject for NSBatchInsertRequest {}
impl INSBatchInsertRequest for NSBatchInsertRequest {}
pub trait INSBatchInsertRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEntityName_objects_(
        &self,
        entityName: NSString,
        dictionaries: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntityName : entityName, objects : dictionaries)
    }
    unsafe fn initWithEntity_objects_(
        &self,
        entity: NSEntityDescription,
        dictionaries: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntity : entity, objects : dictionaries)
    }
    unsafe fn initWithEntity_dictionaryHandler_(
        &self,
        entity: NSEntityDescription,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntity : entity, dictionaryHandler : handler)
    }
    unsafe fn initWithEntity_managedObjectHandler_(
        &self,
        entity: NSEntityDescription,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntity : entity, managedObjectHandler : handler)
    }
    unsafe fn initWithEntityName_dictionaryHandler_(
        &self,
        entityName: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntityName : entityName, dictionaryHandler : handler)
    }
    unsafe fn initWithEntityName_managedObjectHandler_(
        &self,
        entityName: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntityName : entityName, managedObjectHandler : handler)
    }
    unsafe fn entityName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityName)
    }
    unsafe fn entity(&self) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn objectsToInsert(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectsToInsert)
    }
    unsafe fn setObjectsToInsert_(&self, objectsToInsert: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectsToInsert : objectsToInsert)
    }
    unsafe fn dictionaryHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryHandler)
    }
    unsafe fn setDictionaryHandler_(&self, dictionaryHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDictionaryHandler : dictionaryHandler)
    }
    unsafe fn managedObjectHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectHandler)
    }
    unsafe fn setManagedObjectHandler_(&self, managedObjectHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManagedObjectHandler : managedObjectHandler)
    }
    unsafe fn resultType(&self) -> NSBatchInsertRequestResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
    unsafe fn setResultType_(&self, resultType: NSBatchInsertRequestResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultType : resultType)
    }
    unsafe fn batchInsertRequestWithEntityName_objects_(
        entityName: NSString,
        dictionaries: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchInsertRequest").unwrap(), batchInsertRequestWithEntityName : entityName, objects : dictionaries)
    }
    unsafe fn batchInsertRequestWithEntityName_dictionaryHandler_(
        entityName: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchInsertRequest").unwrap(), batchInsertRequestWithEntityName : entityName, dictionaryHandler : handler)
    }
    unsafe fn batchInsertRequestWithEntityName_managedObjectHandler_(
        entityName: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSBatchInsertRequest").unwrap(), batchInsertRequestWithEntityName : entityName, managedObjectHandler : handler)
    }
}
pub type NSMergePolicyType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSMergeConflict(pub id);
impl std::ops::Deref for NSMergeConflict {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSMergeConflict {}
impl NSMergeConflict {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSMergeConflict").unwrap(), alloc) })
    }
}
impl INSObject for NSMergeConflict {}
impl PNSObject for NSMergeConflict {}
impl std::convert::TryFrom<NSObject> for NSMergeConflict {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSMergeConflict, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSMergeConflict").unwrap()) };
        if is_kind_of {
            Ok(NSMergeConflict(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSMergeConflict")
        }
    }
}
impl INSMergeConflict for NSMergeConflict {}
pub trait INSMergeConflict: Sized + std::ops::Deref {
    unsafe fn initWithSource_newVersion_oldVersion_cachedSnapshot_persistedSnapshot_(
        &self,
        srcObject: NSManagedObject,
        newvers: NSUInteger,
        oldvers: NSUInteger,
        cachesnap: NSDictionary,
        persnap: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : srcObject, newVersion : newvers, oldVersion : oldvers, cachedSnapshot : cachesnap, persistedSnapshot : persnap)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sourceObject(&self) -> NSManagedObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceObject)
    }
    unsafe fn objectSnapshot(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectSnapshot)
    }
    unsafe fn cachedSnapshot(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cachedSnapshot)
    }
    unsafe fn persistedSnapshot(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistedSnapshot)
    }
    unsafe fn newVersionNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newVersionNumber)
    }
    unsafe fn oldVersionNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, oldVersionNumber)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSConstraintConflict(pub id);
impl std::ops::Deref for NSConstraintConflict {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSConstraintConflict {}
impl NSConstraintConflict {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSConstraintConflict").unwrap(), alloc) })
    }
}
impl INSObject for NSConstraintConflict {}
impl PNSObject for NSConstraintConflict {}
impl std::convert::TryFrom<NSObject> for NSConstraintConflict {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSConstraintConflict, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSConstraintConflict").unwrap()) };
        if is_kind_of {
            Ok(NSConstraintConflict(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSConstraintConflict")
        }
    }
}
impl INSConstraintConflict for NSConstraintConflict {}
pub trait INSConstraintConflict: Sized + std::ops::Deref {
    unsafe fn initWithConstraint_databaseObject_databaseSnapshot_conflictingObjects_conflictingSnapshots_(
        &self,
        contraint: NSArray,
        databaseObject: NSManagedObject,
        databaseSnapshot: NSDictionary,
        conflictingObjects: NSArray,
        conflictingSnapshots: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConstraint : contraint, databaseObject : databaseObject, databaseSnapshot : databaseSnapshot, conflictingObjects : conflictingObjects, conflictingSnapshots : conflictingSnapshots)
    }
    unsafe fn constraint(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constraint)
    }
    unsafe fn constraintValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constraintValues)
    }
    unsafe fn databaseObject(&self) -> NSManagedObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, databaseObject)
    }
    unsafe fn databaseSnapshot(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, databaseSnapshot)
    }
    unsafe fn conflictingObjects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conflictingObjects)
    }
    unsafe fn conflictingSnapshots(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conflictingSnapshots)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSMergePolicy(pub id);
impl std::ops::Deref for NSMergePolicy {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSMergePolicy {}
impl NSMergePolicy {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSMergePolicy").unwrap(), alloc) })
    }
}
impl INSObject for NSMergePolicy {}
impl PNSObject for NSMergePolicy {}
impl std::convert::TryFrom<NSObject> for NSMergePolicy {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSMergePolicy, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSMergePolicy").unwrap()) };
        if is_kind_of {
            Ok(NSMergePolicy(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSMergePolicy")
        }
    }
}
impl INSMergePolicy for NSMergePolicy {}
pub trait INSMergePolicy: Sized + std::ops::Deref {
    unsafe fn initWithMergeType_(&self, ty: NSMergePolicyType) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMergeType : ty)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn resolveConflicts_error_(&self, list: NSArray, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveConflicts : list, error : error)
    }
    unsafe fn resolveOptimisticLockingVersionConflicts_error_(
        &self,
        list: NSArray,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveOptimisticLockingVersionConflicts : list, error : error)
    }
    unsafe fn resolveConstraintConflicts_error_(&self, list: NSArray, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveConstraintConflicts : list, error : error)
    }
    unsafe fn mergeType(&self) -> NSMergePolicyType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mergeType)
    }
    unsafe fn errorMergePolicy() -> NSMergePolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMergePolicy").unwrap(), errorMergePolicy)
    }
    unsafe fn rollbackMergePolicy() -> NSMergePolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMergePolicy").unwrap(), rollbackMergePolicy)
    }
    unsafe fn overwriteMergePolicy() -> NSMergePolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMergePolicy").unwrap(), overwriteMergePolicy)
    }
    unsafe fn mergeByPropertyObjectTrumpMergePolicy() -> NSMergePolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMergePolicy").unwrap(), mergeByPropertyObjectTrumpMergePolicy)
    }
    unsafe fn mergeByPropertyStoreTrumpMergePolicy() -> NSMergePolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMergePolicy").unwrap(), mergeByPropertyStoreTrumpMergePolicy)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFetchedResultsController(pub id);
impl std::ops::Deref for NSFetchedResultsController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFetchedResultsController {}
impl NSFetchedResultsController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchedResultsController").unwrap(), alloc) })
    }
}
impl INSObject for NSFetchedResultsController {}
impl PNSObject for NSFetchedResultsController {}
impl std::convert::TryFrom<NSObject> for NSFetchedResultsController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFetchedResultsController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFetchedResultsController").unwrap()) };
        if is_kind_of {
            Ok(NSFetchedResultsController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFetchedResultsController")
        }
    }
}
impl<ResultType: 'static> INSFetchedResultsController<ResultType> for NSFetchedResultsController {}
pub trait INSFetchedResultsController<ResultType: 'static>: Sized + std::ops::Deref {
    unsafe fn initWithFetchRequest_managedObjectContext_sectionNameKeyPath_cacheName_(
        &self,
        fetchRequest: NSFetchRequest,
        context: NSManagedObjectContext,
        sectionNameKeyPath: NSString,
        name: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFetchRequest : fetchRequest, managedObjectContext : context, sectionNameKeyPath : sectionNameKeyPath, cacheName : name)
    }
    unsafe fn performFetch_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performFetch : error)
    }
    unsafe fn objectAtIndexPath_(&self, indexPath: NSIndexPath) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexPath : indexPath)
    }
    unsafe fn indexPathForObject_(&self, object: id) -> NSIndexPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexPathForObject : object)
    }
    unsafe fn sectionIndexTitleForSectionName_(&self, sectionName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sectionIndexTitleForSectionName : sectionName)
    }
    unsafe fn sectionForSectionIndexTitle_atIndex_(
        &self,
        title: NSString,
        sectionIndex: NSInteger,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sectionForSectionIndexTitle : title, atIndex : sectionIndex)
    }
    unsafe fn fetchRequest(&self) -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRequest)
    }
    unsafe fn managedObjectContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectContext)
    }
    unsafe fn sectionNameKeyPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sectionNameKeyPath)
    }
    unsafe fn cacheName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cacheName)
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
    unsafe fn fetchedObjects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchedObjects)
    }
    unsafe fn sectionIndexTitles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sectionIndexTitles)
    }
    unsafe fn sections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sections)
    }
    unsafe fn deleteCacheWithName_(name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFetchedResultsController").unwrap(), deleteCacheWithName : name)
    }
}
pub trait PNSFetchedResultsSectionInfo: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn indexTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexTitle)
    }
    unsafe fn numberOfObjects(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfObjects)
    }
    unsafe fn objects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objects)
    }
}
pub type NSFetchedResultsChangeType = NSUInteger;
pub trait PNSFetchedResultsControllerDelegate: Sized + std::ops::Deref {
    unsafe fn controller_didChangeContentWithSnapshot_(
        &self,
        controller: NSFetchedResultsController,
        snapshot: NSDiffableDataSourceSnapshot,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controller : controller, didChangeContentWithSnapshot : snapshot)
    }
    unsafe fn controller_didChangeContentWithDifference_(
        &self,
        controller: NSFetchedResultsController,
        diff: NSOrderedCollectionDifference,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controller : controller, didChangeContentWithDifference : diff)
    }
    unsafe fn controller_didChangeObject_atIndexPath_forChangeType_newIndexPath_(
        &self,
        controller: NSFetchedResultsController,
        anObject: id,
        indexPath: NSIndexPath,
        type_: NSFetchedResultsChangeType,
        newIndexPath: NSIndexPath,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controller : controller, didChangeObject : anObject, atIndexPath : indexPath, forChangeType : type_, newIndexPath : newIndexPath)
    }
    unsafe fn controller_didChangeSection_atIndex_forChangeType_(
        &self,
        controller: NSFetchedResultsController,
        sectionInfo: *mut u64,
        sectionIndex: NSUInteger,
        type_: NSFetchedResultsChangeType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controller : controller, didChangeSection : sectionInfo, atIndex : sectionIndex, forChangeType : type_)
    }
    unsafe fn controllerWillChangeContent_(&self, controller: NSFetchedResultsController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controllerWillChangeContent : controller)
    }
    unsafe fn controllerDidChangeContent_(&self, controller: NSFetchedResultsController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controllerDidChangeContent : controller)
    }
    unsafe fn controller_sectionIndexTitleForSectionName_(
        &self,
        controller: NSFetchedResultsController,
        sectionName: NSString,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controller : controller, sectionIndexTitleForSectionName : sectionName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSQueryGenerationToken(pub id);
impl std::ops::Deref for NSQueryGenerationToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSQueryGenerationToken {}
impl NSQueryGenerationToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSQueryGenerationToken").unwrap(), alloc) })
    }
}
impl PNSCopying for NSQueryGenerationToken {}
impl PNSSecureCoding for NSQueryGenerationToken {}
impl INSObject for NSQueryGenerationToken {}
impl PNSObject for NSQueryGenerationToken {}
impl std::convert::TryFrom<NSObject> for NSQueryGenerationToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSQueryGenerationToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSQueryGenerationToken").unwrap()) };
        if is_kind_of {
            Ok(NSQueryGenerationToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSQueryGenerationToken")
        }
    }
}
impl INSQueryGenerationToken for NSQueryGenerationToken {}
pub trait INSQueryGenerationToken: Sized + std::ops::Deref {
    unsafe fn currentQueryGenerationToken() -> NSQueryGenerationToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSQueryGenerationToken").unwrap(), currentQueryGenerationToken)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentStoreDescription(pub id);
impl std::ops::Deref for NSPersistentStoreDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentStoreDescription {}
impl NSPersistentStoreDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreDescription").unwrap(), alloc) })
    }
}
impl PNSCopying for NSPersistentStoreDescription {}
impl INSObject for NSPersistentStoreDescription {}
impl PNSObject for NSPersistentStoreDescription {}
impl std::convert::TryFrom<NSObject> for NSPersistentStoreDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentStoreDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentStoreDescription").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentStoreDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentStoreDescription")
        }
    }
}
impl INSPersistentStoreDescription for NSPersistentStoreDescription {}
pub trait INSPersistentStoreDescription: Sized + std::ops::Deref {
    unsafe fn setOption_forKey_(&self, option: NSObject, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOption : option, forKey : key)
    }
    unsafe fn setValue_forPragmaNamed_(&self, value: NSObject, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forPragmaNamed : name)
    }
    unsafe fn initWithURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn configuration(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn setConfiguration_(&self, configuration: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfiguration : configuration)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
    unsafe fn options(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn isReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadOnly)
    }
    unsafe fn setReadOnly_(&self, readOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadOnly : readOnly)
    }
    unsafe fn timeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeout)
    }
    unsafe fn setTimeout_(&self, timeout: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeout : timeout)
    }
    unsafe fn sqlitePragmas(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sqlitePragmas)
    }
    unsafe fn shouldAddStoreAsynchronously(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldAddStoreAsynchronously)
    }
    unsafe fn setShouldAddStoreAsynchronously_(&self, shouldAddStoreAsynchronously: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldAddStoreAsynchronously : shouldAddStoreAsynchronously)
    }
    unsafe fn shouldMigrateStoreAutomatically(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldMigrateStoreAutomatically)
    }
    unsafe fn setShouldMigrateStoreAutomatically_(&self, shouldMigrateStoreAutomatically: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldMigrateStoreAutomatically : shouldMigrateStoreAutomatically)
    }
    unsafe fn shouldInferMappingModelAutomatically(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldInferMappingModelAutomatically)
    }
    unsafe fn setShouldInferMappingModelAutomatically_(
        &self,
        shouldInferMappingModelAutomatically: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldInferMappingModelAutomatically : shouldInferMappingModelAutomatically)
    }
    unsafe fn persistentStoreDescriptionWithURL_(URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentStoreDescription").unwrap(), persistentStoreDescriptionWithURL : URL)
    }
}
impl NSPersistentStoreDescription_NSPersistentCloudKitContainerAdditions
    for NSPersistentStoreDescription
{
}
pub trait NSPersistentStoreDescription_NSPersistentCloudKitContainerAdditions:
    Sized + std::ops::Deref
{
    unsafe fn cloudKitContainerOptions(&self) -> NSPersistentCloudKitContainerOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloudKitContainerOptions)
    }
    unsafe fn setCloudKitContainerOptions_(
        &self,
        cloudKitContainerOptions: NSPersistentCloudKitContainerOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloudKitContainerOptions : cloudKitContainerOptions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentContainer(pub id);
impl std::ops::Deref for NSPersistentContainer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentContainer {}
impl NSPersistentContainer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentContainer").unwrap(), alloc) })
    }
}
impl INSObject for NSPersistentContainer {}
impl PNSObject for NSPersistentContainer {}
impl std::convert::TryFrom<NSObject> for NSPersistentContainer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentContainer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentContainer").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentContainer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentContainer")
        }
    }
}
impl INSPersistentContainer for NSPersistentContainer {}
pub trait INSPersistentContainer: Sized + std::ops::Deref {
    unsafe fn initWithName_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name)
    }
    unsafe fn initWithName_managedObjectModel_(
        &self,
        name: NSString,
        model: NSManagedObjectModel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, managedObjectModel : model)
    }
    unsafe fn loadPersistentStoresWithCompletionHandler_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadPersistentStoresWithCompletionHandler : block)
    }
    unsafe fn newBackgroundContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newBackgroundContext)
    }
    unsafe fn performBackgroundTask_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performBackgroundTask : block)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn viewContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, viewContext)
    }
    unsafe fn managedObjectModel(&self) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectModel)
    }
    unsafe fn persistentStoreCoordinator(&self) -> NSPersistentStoreCoordinator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentStoreCoordinator)
    }
    unsafe fn persistentStoreDescriptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentStoreDescriptions)
    }
    unsafe fn setPersistentStoreDescriptions_(&self, persistentStoreDescriptions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPersistentStoreDescriptions : persistentStoreDescriptions)
    }
    unsafe fn persistentContainerWithName_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentContainer").unwrap(), persistentContainerWithName : name)
    }
    unsafe fn persistentContainerWithName_managedObjectModel_(
        name: NSString,
        model: NSManagedObjectModel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentContainer").unwrap(), persistentContainerWithName : name, managedObjectModel : model)
    }
    unsafe fn defaultDirectoryURL() -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentContainer").unwrap(), defaultDirectoryURL)
    }
}
pub type NSPersistentHistoryChangeType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentHistoryChange(pub id);
impl std::ops::Deref for NSPersistentHistoryChange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentHistoryChange {}
impl NSPersistentHistoryChange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChange").unwrap(), alloc) })
    }
}
impl PNSCopying for NSPersistentHistoryChange {}
impl INSObject for NSPersistentHistoryChange {}
impl PNSObject for NSPersistentHistoryChange {}
impl std::convert::TryFrom<NSObject> for NSPersistentHistoryChange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentHistoryChange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentHistoryChange").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentHistoryChange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentHistoryChange")
        }
    }
}
impl INSPersistentHistoryChange for NSPersistentHistoryChange {}
pub trait INSPersistentHistoryChange: Sized + std::ops::Deref {
    unsafe fn changeID(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeID)
    }
    unsafe fn changedObjectID(&self) -> NSManagedObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedObjectID)
    }
    unsafe fn changeType(&self) -> NSPersistentHistoryChangeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeType)
    }
    unsafe fn tombstone(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tombstone)
    }
    unsafe fn transaction(&self) -> NSPersistentHistoryTransaction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transaction)
    }
    unsafe fn updatedProperties(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updatedProperties)
    }
    unsafe fn entityDescriptionWithContext_(context: NSManagedObjectContext) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChange").unwrap(), entityDescriptionWithContext : context)
    }
    unsafe fn entityDescription() -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChange").unwrap(), entityDescription)
    }
    unsafe fn fetchRequest() -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChange").unwrap(), fetchRequest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentHistoryChangeRequest(pub id);
impl std::ops::Deref for NSPersistentHistoryChangeRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentHistoryChangeRequest {}
impl NSPersistentHistoryChangeRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap(), alloc) })
    }
}
impl INSPersistentStoreRequest for NSPersistentHistoryChangeRequest {}
impl PNSCopying for NSPersistentHistoryChangeRequest {}
impl std::convert::TryFrom<NSPersistentStoreRequest> for NSPersistentHistoryChangeRequest {
    type Error = &'static str;
    fn try_from(
        parent: NSPersistentStoreRequest,
    ) -> Result<NSPersistentHistoryChangeRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap())
        };
        if is_kind_of {
            Ok(NSPersistentHistoryChangeRequest(parent.0))
        } else {
            Err ("This NSPersistentStoreRequest cannot be downcasted to NSPersistentHistoryChangeRequest" ,)
        }
    }
}
impl INSObject for NSPersistentHistoryChangeRequest {}
impl PNSObject for NSPersistentHistoryChangeRequest {}
impl INSPersistentHistoryChangeRequest for NSPersistentHistoryChangeRequest {}
pub trait INSPersistentHistoryChangeRequest: Sized + std::ops::Deref {
    unsafe fn resultType(&self) -> NSPersistentHistoryResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
    unsafe fn setResultType_(&self, resultType: NSPersistentHistoryResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultType : resultType)
    }
    unsafe fn token(&self) -> NSPersistentHistoryToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, token)
    }
    unsafe fn fetchRequest(&self) -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRequest)
    }
    unsafe fn setFetchRequest_(&self, fetchRequest: NSFetchRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchRequest : fetchRequest)
    }
    unsafe fn fetchHistoryAfterDate_(date: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap(), fetchHistoryAfterDate : date)
    }
    unsafe fn fetchHistoryAfterToken_(token: NSPersistentHistoryToken) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap(), fetchHistoryAfterToken : token)
    }
    unsafe fn fetchHistoryAfterTransaction_(
        transaction: NSPersistentHistoryTransaction,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap(), fetchHistoryAfterTransaction : transaction)
    }
    unsafe fn fetchHistoryWithFetchRequest_(fetchRequest: NSFetchRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap(), fetchHistoryWithFetchRequest : fetchRequest)
    }
    unsafe fn deleteHistoryBeforeDate_(date: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap(), deleteHistoryBeforeDate : date)
    }
    unsafe fn deleteHistoryBeforeToken_(token: NSPersistentHistoryToken) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap(), deleteHistoryBeforeToken : token)
    }
    unsafe fn deleteHistoryBeforeTransaction_(
        transaction: NSPersistentHistoryTransaction,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryChangeRequest").unwrap(), deleteHistoryBeforeTransaction : transaction)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentHistoryToken(pub id);
impl std::ops::Deref for NSPersistentHistoryToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentHistoryToken {}
impl NSPersistentHistoryToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryToken").unwrap(), alloc) })
    }
}
impl PNSCopying for NSPersistentHistoryToken {}
impl PNSSecureCoding for NSPersistentHistoryToken {}
impl INSObject for NSPersistentHistoryToken {}
impl PNSObject for NSPersistentHistoryToken {}
impl std::convert::TryFrom<NSObject> for NSPersistentHistoryToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentHistoryToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentHistoryToken").unwrap()) };
        if is_kind_of {
            Ok(NSPersistentHistoryToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentHistoryToken")
        }
    }
}
impl INSPersistentHistoryToken for NSPersistentHistoryToken {}
pub trait INSPersistentHistoryToken: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentHistoryTransaction(pub id);
impl std::ops::Deref for NSPersistentHistoryTransaction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentHistoryTransaction {}
impl NSPersistentHistoryTransaction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryTransaction").unwrap(), alloc) })
    }
}
impl PNSCopying for NSPersistentHistoryTransaction {}
impl INSObject for NSPersistentHistoryTransaction {}
impl PNSObject for NSPersistentHistoryTransaction {}
impl std::convert::TryFrom<NSObject> for NSPersistentHistoryTransaction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentHistoryTransaction, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentHistoryTransaction").unwrap())
        };
        if is_kind_of {
            Ok(NSPersistentHistoryTransaction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentHistoryTransaction")
        }
    }
}
impl INSPersistentHistoryTransaction for NSPersistentHistoryTransaction {}
pub trait INSPersistentHistoryTransaction: Sized + std::ops::Deref {
    unsafe fn objectIDNotification(&self) -> NSNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectIDNotification)
    }
    unsafe fn timestamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn changes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changes)
    }
    unsafe fn transactionNumber(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionNumber)
    }
    unsafe fn storeID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storeID)
    }
    unsafe fn bundleID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleID)
    }
    unsafe fn processID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processID)
    }
    unsafe fn contextName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contextName)
    }
    unsafe fn author(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, author)
    }
    unsafe fn token(&self) -> NSPersistentHistoryToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, token)
    }
    unsafe fn entityDescriptionWithContext_(context: NSManagedObjectContext) -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryTransaction").unwrap(), entityDescriptionWithContext : context)
    }
    unsafe fn entityDescription() -> NSEntityDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryTransaction").unwrap(), entityDescription)
    }
    unsafe fn fetchRequest() -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentHistoryTransaction").unwrap(), fetchRequest)
    }
}
pub type NSPersistentCloudKitContainerSchemaInitializationOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentCloudKitContainer(pub id);
impl std::ops::Deref for NSPersistentCloudKitContainer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentCloudKitContainer {}
impl NSPersistentCloudKitContainer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainer").unwrap(), alloc) })
    }
}
impl INSPersistentContainer for NSPersistentCloudKitContainer {}
impl From<NSPersistentCloudKitContainer> for NSPersistentContainer {
    fn from(child: NSPersistentCloudKitContainer) -> NSPersistentContainer {
        NSPersistentContainer(child.0)
    }
}
impl std::convert::TryFrom<NSPersistentContainer> for NSPersistentCloudKitContainer {
    type Error = &'static str;
    fn try_from(
        parent: NSPersistentContainer,
    ) -> Result<NSPersistentCloudKitContainer, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainer").unwrap())
        };
        if is_kind_of {
            Ok(NSPersistentCloudKitContainer(parent.0))
        } else {
            Err("This NSPersistentContainer cannot be downcasted to NSPersistentCloudKitContainer")
        }
    }
}
impl INSObject for NSPersistentCloudKitContainer {}
impl PNSObject for NSPersistentCloudKitContainer {}
impl INSPersistentCloudKitContainer for NSPersistentCloudKitContainer {}
pub trait INSPersistentCloudKitContainer: Sized + std::ops::Deref {
    unsafe fn initializeCloudKitSchemaWithOptions_error_(
        &self,
        options: NSPersistentCloudKitContainerSchemaInitializationOptions,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initializeCloudKitSchemaWithOptions : options, error : error)
    }
    unsafe fn recordForManagedObjectID_(&self, managedObjectID: NSManagedObjectID) -> CKRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordForManagedObjectID : managedObjectID)
    }
    unsafe fn recordsForManagedObjectIDs_(&self, managedObjectIDs: NSArray) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordsForManagedObjectIDs : managedObjectIDs)
    }
    unsafe fn recordIDForManagedObjectID_(&self, managedObjectID: NSManagedObjectID) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordIDForManagedObjectID : managedObjectID)
    }
    unsafe fn recordIDsForManagedObjectIDs_(&self, managedObjectIDs: NSArray) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordIDsForManagedObjectIDs : managedObjectIDs)
    }
    unsafe fn canUpdateRecordForManagedObjectWithID_(&self, objectID: NSManagedObjectID) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canUpdateRecordForManagedObjectWithID : objectID)
    }
    unsafe fn canDeleteRecordForManagedObjectWithID_(&self, objectID: NSManagedObjectID) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canDeleteRecordForManagedObjectWithID : objectID)
    }
    unsafe fn canModifyManagedObjectsInStore_(&self, store: NSPersistentStore) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canModifyManagedObjectsInStore : store)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentCloudKitContainerOptions(pub id);
impl std::ops::Deref for NSPersistentCloudKitContainerOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentCloudKitContainerOptions {}
impl NSPersistentCloudKitContainerOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerOptions").unwrap(), alloc) })
    }
}
impl INSObject for NSPersistentCloudKitContainerOptions {}
impl PNSObject for NSPersistentCloudKitContainerOptions {}
impl std::convert::TryFrom<NSObject> for NSPersistentCloudKitContainerOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentCloudKitContainerOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerOptions").unwrap())
        };
        if is_kind_of {
            Ok(NSPersistentCloudKitContainerOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentCloudKitContainerOptions")
        }
    }
}
impl INSPersistentCloudKitContainerOptions for NSPersistentCloudKitContainerOptions {}
pub trait INSPersistentCloudKitContainerOptions: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithContainerIdentifier_(&self, containerIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContainerIdentifier : containerIdentifier)
    }
    unsafe fn containerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerIdentifier)
    }
    unsafe fn databaseScope(&self) -> CKDatabaseScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, databaseScope)
    }
    unsafe fn setDatabaseScope_(&self, databaseScope: CKDatabaseScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDatabaseScope : databaseScope)
    }
}
pub type NSPersistentCloudKitContainerEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentCloudKitContainerEvent(pub id);
impl std::ops::Deref for NSPersistentCloudKitContainerEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentCloudKitContainerEvent {}
impl NSPersistentCloudKitContainerEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for NSPersistentCloudKitContainerEvent {}
impl INSObject for NSPersistentCloudKitContainerEvent {}
impl PNSObject for NSPersistentCloudKitContainerEvent {}
impl std::convert::TryFrom<NSObject> for NSPersistentCloudKitContainerEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPersistentCloudKitContainerEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEvent").unwrap())
        };
        if is_kind_of {
            Ok(NSPersistentCloudKitContainerEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPersistentCloudKitContainerEvent")
        }
    }
}
impl INSPersistentCloudKitContainerEvent for NSPersistentCloudKitContainerEvent {}
pub trait INSPersistentCloudKitContainerEvent: Sized + std::ops::Deref {
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
    unsafe fn storeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storeIdentifier)
    }
    unsafe fn type_(&self) -> NSPersistentCloudKitContainerEventType
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
    unsafe fn succeeded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, succeeded)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEvent").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPersistentCloudKitContainerEventRequest(pub id);
impl std::ops::Deref for NSPersistentCloudKitContainerEventRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPersistentCloudKitContainerEventRequest {}
impl NSPersistentCloudKitContainerEventRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventRequest").unwrap(), alloc) })
    }
}
impl INSPersistentStoreRequest for NSPersistentCloudKitContainerEventRequest {}
impl PNSCopying for NSPersistentCloudKitContainerEventRequest {}
impl std::convert::TryFrom<NSPersistentStoreRequest> for NSPersistentCloudKitContainerEventRequest {
    type Error = &'static str;
    fn try_from(
        parent: NSPersistentStoreRequest,
    ) -> Result<NSPersistentCloudKitContainerEventRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventRequest").unwrap())
        };
        if is_kind_of {
            Ok(NSPersistentCloudKitContainerEventRequest(parent.0))
        } else {
            Err ("This NSPersistentStoreRequest cannot be downcasted to NSPersistentCloudKitContainerEventRequest" ,)
        }
    }
}
impl INSObject for NSPersistentCloudKitContainerEventRequest {}
impl PNSObject for NSPersistentCloudKitContainerEventRequest {}
impl INSPersistentCloudKitContainerEventRequest for NSPersistentCloudKitContainerEventRequest {}
pub trait INSPersistentCloudKitContainerEventRequest: Sized + std::ops::Deref {
    unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
    unsafe fn setResultType_(&self, resultType: NSPersistentCloudKitContainerEventResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultType : resultType)
    }
    unsafe fn fetchEventsAfterDate_(date: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventRequest").unwrap(), fetchEventsAfterDate : date)
    }
    unsafe fn fetchEventsAfterEvent_(event: NSPersistentCloudKitContainerEvent) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventRequest").unwrap(), fetchEventsAfterEvent : event)
    }
    unsafe fn fetchEventsMatchingFetchRequest_(fetchRequest: NSFetchRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventRequest").unwrap(), fetchEventsMatchingFetchRequest : fetchRequest)
    }
    unsafe fn fetchRequestForEvents() -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPersistentCloudKitContainerEventRequest").unwrap(), fetchRequestForEvents)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSStagedMigrationManager(pub id);
impl std::ops::Deref for NSStagedMigrationManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSStagedMigrationManager {}
impl NSStagedMigrationManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSStagedMigrationManager").unwrap(), alloc) })
    }
}
impl INSObject for NSStagedMigrationManager {}
impl PNSObject for NSStagedMigrationManager {}
impl std::convert::TryFrom<NSObject> for NSStagedMigrationManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSStagedMigrationManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSStagedMigrationManager").unwrap()) };
        if is_kind_of {
            Ok(NSStagedMigrationManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSStagedMigrationManager")
        }
    }
}
impl INSStagedMigrationManager for NSStagedMigrationManager {}
pub trait INSStagedMigrationManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMigrationStages_(&self, stages: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMigrationStages : stages)
    }
    unsafe fn stages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stages)
    }
    unsafe fn container(&self) -> NSPersistentContainer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, container)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSMigrationStage(pub id);
impl std::ops::Deref for NSMigrationStage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSMigrationStage {}
impl NSMigrationStage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSMigrationStage").unwrap(), alloc) })
    }
}
impl INSObject for NSMigrationStage {}
impl PNSObject for NSMigrationStage {}
impl std::convert::TryFrom<NSObject> for NSMigrationStage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSMigrationStage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSMigrationStage").unwrap()) };
        if is_kind_of {
            Ok(NSMigrationStage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSMigrationStage")
        }
    }
}
impl INSMigrationStage for NSMigrationStage {}
pub trait INSMigrationStage: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSCustomMigrationStage(pub id);
impl std::ops::Deref for NSCustomMigrationStage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSCustomMigrationStage {}
impl NSCustomMigrationStage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSCustomMigrationStage").unwrap(), alloc) })
    }
}
impl INSMigrationStage for NSCustomMigrationStage {}
impl From<NSCustomMigrationStage> for NSMigrationStage {
    fn from(child: NSCustomMigrationStage) -> NSMigrationStage {
        NSMigrationStage(child.0)
    }
}
impl std::convert::TryFrom<NSMigrationStage> for NSCustomMigrationStage {
    type Error = &'static str;
    fn try_from(parent: NSMigrationStage) -> Result<NSCustomMigrationStage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSCustomMigrationStage").unwrap()) };
        if is_kind_of {
            Ok(NSCustomMigrationStage(parent.0))
        } else {
            Err("This NSMigrationStage cannot be downcasted to NSCustomMigrationStage")
        }
    }
}
impl INSObject for NSCustomMigrationStage {}
impl PNSObject for NSCustomMigrationStage {}
impl INSCustomMigrationStage for NSCustomMigrationStage {}
pub trait INSCustomMigrationStage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCurrentModelReference_nextModelReference_(
        &self,
        currentModel: NSManagedObjectModelReference,
        nextModel: NSManagedObjectModelReference,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCurrentModelReference : currentModel, nextModelReference : nextModel)
    }
    unsafe fn currentModel(&self) -> NSManagedObjectModelReference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentModel)
    }
    unsafe fn nextModel(&self) -> NSManagedObjectModelReference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextModel)
    }
    unsafe fn willMigrateHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willMigrateHandler)
    }
    unsafe fn setWillMigrateHandler_(&self, willMigrateHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWillMigrateHandler : willMigrateHandler)
    }
    unsafe fn didMigrateHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didMigrateHandler)
    }
    unsafe fn setDidMigrateHandler_(&self, didMigrateHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDidMigrateHandler : didMigrateHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSLightweightMigrationStage(pub id);
impl std::ops::Deref for NSLightweightMigrationStage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSLightweightMigrationStage {}
impl NSLightweightMigrationStage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSLightweightMigrationStage").unwrap(), alloc) })
    }
}
impl INSMigrationStage for NSLightweightMigrationStage {}
impl std::convert::TryFrom<NSMigrationStage> for NSLightweightMigrationStage {
    type Error = &'static str;
    fn try_from(parent: NSMigrationStage) -> Result<NSLightweightMigrationStage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSLightweightMigrationStage").unwrap()) };
        if is_kind_of {
            Ok(NSLightweightMigrationStage(parent.0))
        } else {
            Err("This NSMigrationStage cannot be downcasted to NSLightweightMigrationStage")
        }
    }
}
impl INSObject for NSLightweightMigrationStage {}
impl PNSObject for NSLightweightMigrationStage {}
impl INSLightweightMigrationStage for NSLightweightMigrationStage {}
pub trait INSLightweightMigrationStage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithVersionChecksums_(&self, versionChecksums: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVersionChecksums : versionChecksums)
    }
    unsafe fn versionChecksums(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionChecksums)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSManagedObjectModelReference(pub id);
impl std::ops::Deref for NSManagedObjectModelReference {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSManagedObjectModelReference {}
impl NSManagedObjectModelReference {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSManagedObjectModelReference").unwrap(), alloc) })
    }
}
impl INSObject for NSManagedObjectModelReference {}
impl PNSObject for NSManagedObjectModelReference {}
impl std::convert::TryFrom<NSObject> for NSManagedObjectModelReference {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSManagedObjectModelReference, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSManagedObjectModelReference").unwrap())
        };
        if is_kind_of {
            Ok(NSManagedObjectModelReference(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSManagedObjectModelReference")
        }
    }
}
impl INSManagedObjectModelReference for NSManagedObjectModelReference {}
pub trait INSManagedObjectModelReference: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithModel_versionChecksum_(
        &self,
        model: NSManagedObjectModel,
        versionChecksum: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithModel : model, versionChecksum : versionChecksum)
    }
    unsafe fn initWithFileURL_versionChecksum_(
        &self,
        fileURL: NSURL,
        versionChecksum: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileURL : fileURL, versionChecksum : versionChecksum)
    }
    unsafe fn initWithEntityVersionHashes_inBundle_versionChecksum_(
        &self,
        versionHash: NSDictionary,
        bundle: NSBundle,
        versionChecksum: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEntityVersionHashes : versionHash, inBundle : bundle, versionChecksum : versionChecksum)
    }
    unsafe fn initWithName_inBundle_versionChecksum_(
        &self,
        modelName: NSString,
        bundle: NSBundle,
        versionChecksum: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : modelName, inBundle : bundle, versionChecksum : versionChecksum)
    }
    unsafe fn resolvedModel(&self) -> NSManagedObjectModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolvedModel)
    }
    unsafe fn versionChecksum(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionChecksum)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSCoreDataCoreSpotlightDelegate(pub id);
impl std::ops::Deref for NSCoreDataCoreSpotlightDelegate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSCoreDataCoreSpotlightDelegate {}
impl NSCoreDataCoreSpotlightDelegate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSCoreDataCoreSpotlightDelegate").unwrap(), alloc) })
    }
}
impl INSObject for NSCoreDataCoreSpotlightDelegate {}
impl PNSObject for NSCoreDataCoreSpotlightDelegate {}
impl std::convert::TryFrom<NSObject> for NSCoreDataCoreSpotlightDelegate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSCoreDataCoreSpotlightDelegate, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSCoreDataCoreSpotlightDelegate").unwrap())
        };
        if is_kind_of {
            Ok(NSCoreDataCoreSpotlightDelegate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSCoreDataCoreSpotlightDelegate")
        }
    }
}
impl INSCoreDataCoreSpotlightDelegate for NSCoreDataCoreSpotlightDelegate {}
pub trait INSCoreDataCoreSpotlightDelegate: Sized + std::ops::Deref {
    unsafe fn domainIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainIdentifier)
    }
    unsafe fn indexName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexName)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initForStoreWithDescription_coordinator_(
        &self,
        description: NSPersistentStoreDescription,
        psc: NSPersistentStoreCoordinator,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForStoreWithDescription : description, coordinator : psc)
    }
    unsafe fn initForStoreWithDescription_model_(
        &self,
        description: NSPersistentStoreDescription,
        model: NSManagedObjectModel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForStoreWithDescription : description, model : model)
    }
    unsafe fn startSpotlightIndexing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startSpotlightIndexing)
    }
    unsafe fn stopSpotlightIndexing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopSpotlightIndexing)
    }
    unsafe fn deleteSpotlightIndexWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteSpotlightIndexWithCompletionHandler : completionHandler)
    }
    unsafe fn attributeSetForObject_(&self, object: NSManagedObject) -> CSSearchableItemAttributeSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributeSetForObject : object)
    }
    unsafe fn searchableIndex_reindexAllSearchableItemsWithAcknowledgementHandler_(
        &self,
        searchableIndex: CSSearchableIndex,
        acknowledgementHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchableIndex : searchableIndex, reindexAllSearchableItemsWithAcknowledgementHandler : acknowledgementHandler)
    }
    unsafe fn searchableIndex_reindexSearchableItemsWithIdentifiers_acknowledgementHandler_(
        &self,
        searchableIndex: CSSearchableIndex,
        identifiers: NSArray,
        acknowledgementHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchableIndex : searchableIndex, reindexSearchableItemsWithIdentifiers : identifiers, acknowledgementHandler : acknowledgementHandler)
    }
    unsafe fn isIndexingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIndexingEnabled)
    }
}
impl NSPersistentCloudKitContainer_Sharing for NSPersistentCloudKitContainer {}
pub trait NSPersistentCloudKitContainer_Sharing: Sized + std::ops::Deref {
    unsafe fn acceptShareInvitationsFromMetadata_intoPersistentStore_completion_(
        &self,
        metadata: NSArray,
        persistentStore: NSPersistentStore,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acceptShareInvitationsFromMetadata : metadata, intoPersistentStore : persistentStore, completion : completion)
    }
    unsafe fn purgeObjectsAndRecordsInZoneWithID_inPersistentStore_completion_(
        &self,
        zoneID: CKRecordZoneID,
        persistentStore: NSPersistentStore,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, purgeObjectsAndRecordsInZoneWithID : zoneID, inPersistentStore : persistentStore, completion : completion)
    }
    unsafe fn persistUpdatedShare_inPersistentStore_completion_(
        &self,
        share: CKShare,
        persistentStore: NSPersistentStore,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistUpdatedShare : share, inPersistentStore : persistentStore, completion : completion)
    }
    unsafe fn fetchParticipantsMatchingLookupInfos_intoPersistentStore_completion_(
        &self,
        lookupInfos: NSArray,
        persistentStore: NSPersistentStore,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchParticipantsMatchingLookupInfos : lookupInfos, intoPersistentStore : persistentStore, completion : completion)
    }
    unsafe fn fetchSharesMatchingObjectIDs_error_(
        &self,
        objectIDs: NSArray,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchSharesMatchingObjectIDs : objectIDs, error : error)
    }
    unsafe fn fetchSharesInPersistentStore_error_(
        &self,
        persistentStore: NSPersistentStore,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchSharesInPersistentStore : persistentStore, error : error)
    }
    unsafe fn shareManagedObjects_toShare_completion_(
        &self,
        managedObjects: NSArray,
        share: CKShare,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shareManagedObjects : managedObjects, toShare : share, completion : completion)
    }
}
unsafe extern "C" {
    pub static mut NSCoreDataVersionNumber: f64;
}
unsafe extern "C" {
    pub static NSDetailedErrorsKey: NSString;
}
unsafe extern "C" {
    pub static NSValidationObjectErrorKey: NSString;
}
unsafe extern "C" {
    pub static NSValidationKeyErrorKey: NSString;
}
unsafe extern "C" {
    pub static NSValidationPredicateErrorKey: NSString;
}
unsafe extern "C" {
    pub static NSValidationValueErrorKey: NSString;
}
unsafe extern "C" {
    pub static NSAffectedStoresErrorKey: NSString;
}
unsafe extern "C" {
    pub static NSAffectedObjectsErrorKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreSaveConflictsErrorKey: NSString;
}
unsafe extern "C" {
    pub static NSSQLiteErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NSManagedObjectContextWillSaveNotification: NSString;
}
unsafe extern "C" {
    pub static NSManagedObjectContextDidSaveNotification: NSString;
}
unsafe extern "C" {
    pub static NSManagedObjectContextObjectsDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static NSManagedObjectContextDidSaveObjectIDsNotification: NSString;
}
unsafe extern "C" {
    pub static NSManagedObjectContextDidMergeChangesObjectIDsNotification: NSString;
}
unsafe extern "C" {
    pub static NSInsertedObjectsKey: NSString;
}
unsafe extern "C" {
    pub static NSUpdatedObjectsKey: NSString;
}
unsafe extern "C" {
    pub static NSDeletedObjectsKey: NSString;
}
unsafe extern "C" {
    pub static NSRefreshedObjectsKey: NSString;
}
unsafe extern "C" {
    pub static NSInvalidatedObjectsKey: NSString;
}
unsafe extern "C" {
    pub static NSManagedObjectContextQueryGenerationKey: NSString;
}
unsafe extern "C" {
    pub static NSInvalidatedAllObjectsKey: NSString;
}
unsafe extern "C" {
    pub static NSInsertedObjectIDsKey: NSString;
}
unsafe extern "C" {
    pub static NSUpdatedObjectIDsKey: NSString;
}
unsafe extern "C" {
    pub static NSDeletedObjectIDsKey: NSString;
}
unsafe extern "C" {
    pub static NSRefreshedObjectIDsKey: NSString;
}
unsafe extern "C" {
    pub static NSInvalidatedObjectIDsKey: NSString;
}
unsafe extern "C" {
    pub static mut NSErrorMergePolicy: id;
}
unsafe extern "C" {
    pub static mut NSMergeByPropertyStoreTrumpMergePolicy: id;
}
unsafe extern "C" {
    pub static mut NSMergeByPropertyObjectTrumpMergePolicy: id;
}
unsafe extern "C" {
    pub static mut NSOverwriteMergePolicy: id;
}
unsafe extern "C" {
    pub static mut NSRollbackMergePolicy: id;
}
unsafe extern "C" {
    pub static NSSQLiteStoreType: NSString;
}
unsafe extern "C" {
    pub static NSBinaryStoreType: NSString;
}
unsafe extern "C" {
    pub static NSInMemoryStoreType: NSString;
}
unsafe extern "C" {
    pub static NSStoreTypeKey: NSString;
}
unsafe extern "C" {
    pub static NSStoreUUIDKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreCoordinatorStoresWillChangeNotification: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreCoordinatorStoresDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreCoordinatorWillRemoveStoreNotification: NSString;
}
unsafe extern "C" {
    pub static NSAddedPersistentStoresKey: NSString;
}
unsafe extern "C" {
    pub static NSRemovedPersistentStoresKey: NSString;
}
unsafe extern "C" {
    pub static NSUUIDChangedPersistentStoresKey: NSString;
}
unsafe extern "C" {
    pub static NSReadOnlyPersistentStoreOption: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreTimeoutOption: NSString;
}
unsafe extern "C" {
    pub static NSSQLitePragmasOption: NSString;
}
unsafe extern "C" {
    pub static NSSQLiteAnalyzeOption: NSString;
}
unsafe extern "C" {
    pub static NSSQLiteManualVacuumOption: NSString;
}
unsafe extern "C" {
    pub static NSIgnorePersistentStoreVersioningOption: NSString;
}
unsafe extern "C" {
    pub static NSMigratePersistentStoresAutomaticallyOption: NSString;
}
unsafe extern "C" {
    pub static NSInferMappingModelAutomaticallyOption: NSString;
}
unsafe extern "C" {
    pub static NSStoreModelVersionHashesKey: NSString;
}
unsafe extern "C" {
    pub static NSStoreModelVersionIdentifiersKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreOSCompatibility: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreConnectionPoolMaxSizeKey: NSString;
}
unsafe extern "C" {
    pub static NSCoreDataCoreSpotlightExporter: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreStagedMigrationManagerOptionKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreForceDestroyOption: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreFileProtectionKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentHistoryTrackingKey: NSString;
}
unsafe extern "C" {
    pub static NSBinaryStoreSecureDecodingClasses: NSString;
}
unsafe extern "C" {
    pub static NSBinaryStoreInsecureDecodingCompatibilityOption: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreRemoteChangeNotificationPostOptionKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreRemoteChangeNotification: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreURLKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentHistoryTokenKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreDeferredLightweightMigrationOptionKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreModelVersionChecksumKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreUbiquitousContentNameKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreUbiquitousContentURLKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreDidImportUbiquitousContentChangesNotification: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreUbiquitousTransitionTypeKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreUbiquitousPeerTokenOption: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreRemoveUbiquitousMetadataOption: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreUbiquitousContainerIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentStoreRebuildFromUbiquitousContentOption: NSString;
}
unsafe extern "C" {
    pub static NSMigrationManagerKey: NSString;
}
unsafe extern "C" {
    pub static NSMigrationSourceObjectKey: NSString;
}
unsafe extern "C" {
    pub static NSMigrationDestinationObjectKey: NSString;
}
unsafe extern "C" {
    pub static NSMigrationEntityMappingKey: NSString;
}
unsafe extern "C" {
    pub static NSMigrationPropertyMappingKey: NSString;
}
unsafe extern "C" {
    pub static NSMigrationEntityPolicyKey: NSString;
}
unsafe extern "C" {
    pub static NSPersistentCloudKitContainerEventChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static NSPersistentCloudKitContainerEventUserInfoKey: NSString;
}
unsafe extern "C" {
    pub static NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification: NSNotificationName;
}

unsafe impl objc2::encode::RefEncode for CLLocationCoordinate2D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLLocationCoordinate2D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CLLocationCoordinate2D", &[]);
}
unsafe impl objc2::encode::RefEncode for CLFloor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLFloor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLLocationSourceInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLLocationSourceInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPropertyDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPropertyDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSAttributeDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAttributeDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSDerivedAttributeDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSDerivedAttributeDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSCompositeAttributeDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSCompositeAttributeDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSEntityDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSEntityDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFetchedPropertyDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFetchedPropertyDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSExpressionDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSExpressionDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSRelationshipDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSRelationshipDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFetchIndexDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFetchIndexDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFetchIndexElementDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFetchIndexElementDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentStoreRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentStoreRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSManagedObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSManagedObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSManagedObjectID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSManagedObjectID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFetchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFetchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSAsynchronousFetchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAsynchronousFetchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFetchRequestExpression {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFetchRequestExpression {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSManagedObjectModel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSManagedObjectModel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSManagedObjectContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSManagedObjectContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentStoreCoordinator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentStoreCoordinator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSAtomicStoreCacheNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAtomicStoreCacheNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSAtomicStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAtomicStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSEntityMigrationPolicy {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSEntityMigrationPolicy {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSMappingModel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSMappingModel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSEntityMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSEntityMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPropertyMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPropertyMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSMigrationManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSMigrationManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSIncrementalStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSIncrementalStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSIncrementalStoreNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSIncrementalStoreNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentStoreResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentStoreResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentStoreAsynchronousResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentStoreAsynchronousResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSAsynchronousFetchResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAsynchronousFetchResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSBatchInsertResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSBatchInsertResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSBatchUpdateResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSBatchUpdateResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSBatchDeleteResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSBatchDeleteResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentHistoryResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentHistoryResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentCloudKitContainerEventResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentCloudKitContainerEventResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSaveChangesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSaveChangesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSBatchUpdateRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSBatchUpdateRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSBatchDeleteRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSBatchDeleteRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSBatchInsertRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSBatchInsertRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSMergeConflict {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSMergeConflict {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSConstraintConflict {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSConstraintConflict {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSMergePolicy {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSMergePolicy {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFetchedResultsController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFetchedResultsController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSQueryGenerationToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSQueryGenerationToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentStoreDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentStoreDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentContainer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentContainer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentHistoryChange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentHistoryChange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentHistoryChangeRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentHistoryChangeRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentHistoryToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentHistoryToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentHistoryTransaction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentHistoryTransaction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentCloudKitContainer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentCloudKitContainer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentCloudKitContainerOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentCloudKitContainerOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentCloudKitContainerEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentCloudKitContainerEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSPersistentCloudKitContainerEventRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPersistentCloudKitContainerEventRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSStagedMigrationManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSStagedMigrationManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSMigrationStage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSMigrationStage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSCustomMigrationStage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSCustomMigrationStage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSLightweightMigrationStage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSLightweightMigrationStage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSManagedObjectModelReference {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSManagedObjectModelReference {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSCoreDataCoreSpotlightDelegate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSCoreDataCoreSpotlightDelegate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
