#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ISyncChangeType = ::std::os::raw::c_int;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ISyncChange(pub id);
impl std::ops::Deref for ISyncChange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ISyncChange {}
impl ISyncChange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncChange").unwrap(), alloc) })
    }
}
impl INSObject for ISyncChange {}
impl PNSObject for ISyncChange {}
impl std::convert::TryFrom<NSObject> for ISyncChange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ISyncChange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ISyncChange").unwrap()) };
        if is_kind_of {
            Ok(ISyncChange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ISyncChange")
        }
    }
}
impl IISyncChange for ISyncChange {}
pub trait IISyncChange: Sized + std::ops::Deref {
    unsafe fn initWithChangeType_recordIdentifier_changes_(
        &self,
        type_: ISyncChangeType,
        recordIdentifier: NSString,
        changes: NSArray,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChangeType : type_, recordIdentifier : recordIdentifier, changes : changes)
    }
    unsafe fn type_(&self) -> ISyncChangeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn recordIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordIdentifier)
    }
    unsafe fn record(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, record)
    }
    unsafe fn changes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changes)
    }
    unsafe fn changeWithType_recordIdentifier_changes_(
        type_: ISyncChangeType,
        recordIdentifier: NSString,
        changes: NSArray,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncChange").unwrap(), changeWithType : type_, recordIdentifier : recordIdentifier, changes : changes)
    }
}
pub type ISyncStatus = SInt32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ISyncClient(pub id);
impl std::ops::Deref for ISyncClient {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ISyncClient {}
impl ISyncClient {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncClient").unwrap(), alloc) })
    }
}
impl INSObject for ISyncClient {}
impl PNSObject for ISyncClient {}
impl std::convert::TryFrom<NSObject> for ISyncClient {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ISyncClient, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ISyncClient").unwrap()) };
        if is_kind_of {
            Ok(ISyncClient(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ISyncClient")
        }
    }
}
impl IISyncClient for ISyncClient {}
pub trait IISyncClient: Sized + std::ops::Deref {
    unsafe fn clientIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientIdentifier)
    }
    unsafe fn clientType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientType)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn setDisplayName_(&self, displayName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayName : displayName)
    }
    unsafe fn imagePath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imagePath)
    }
    unsafe fn setImagePath_(&self, path: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImagePath : path)
    }
    unsafe fn supportedEntityNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedEntityNames)
    }
    unsafe fn canPushChangesForEntityName_(&self, entityName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canPushChangesForEntityName : entityName)
    }
    unsafe fn canPullChangesForEntityName_(&self, entityName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canPullChangesForEntityName : entityName)
    }
    unsafe fn lastSyncDateForEntityName_(&self, entityName: NSString) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lastSyncDateForEntityName : entityName)
    }
    unsafe fn lastSyncStatusForEntityName_(&self, entityName: NSString) -> ISyncStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lastSyncStatusForEntityName : entityName)
    }
    unsafe fn enabledEntityNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabledEntityNames)
    }
    unsafe fn isEnabledForEntityName_(&self, entityName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEnabledForEntityName : entityName)
    }
    unsafe fn setEnabled_forEntityNames_(&self, flag: BOOL, entityNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : flag, forEntityNames : entityNames)
    }
    unsafe fn formatsRelationships(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatsRelationships)
    }
    unsafe fn setFormatsRelationships_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFormatsRelationships : flag)
    }
    unsafe fn shouldReplaceClientRecordsForEntityName_(&self, entityName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldReplaceClientRecordsForEntityName : entityName)
    }
    unsafe fn setShouldReplaceClientRecords_forEntityNames_(&self, flag: BOOL, entityNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldReplaceClientRecords : flag, forEntityNames : entityNames)
    }
    unsafe fn objectForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKey : key)
    }
    unsafe fn setObject_forKey_(&self, value: *mut u64, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : value, forKey : key)
    }
    unsafe fn filters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filters)
    }
    unsafe fn setFilters_(&self, filters: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilters : filters)
    }
    unsafe fn shouldSynchronizeWithClientsOfType_(&self, clientType: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldSynchronizeWithClientsOfType : clientType)
    }
    unsafe fn setShouldSynchronize_withClientsOfType_(&self, flag: BOOL, clientType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldSynchronize : flag, withClientsOfType : clientType)
    }
    unsafe fn syncAlertToolPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, syncAlertToolPath)
    }
    unsafe fn setSyncAlertToolPath_(&self, path: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSyncAlertToolPath : path)
    }
    unsafe fn setSyncAlertHandler_selector_(&self, handler: id, selector: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSyncAlertHandler : handler, selector : selector)
    }
}
pub trait PISyncFiltering: Sized + std::ops::Deref {
    unsafe fn isEqual_(&self, anotherFilter: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqual : anotherFilter)
    }
    unsafe fn supportedEntityNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedEntityNames)
    }
    unsafe fn shouldApplyRecord_withRecordIdentifier_(
        &self,
        record: NSDictionary,
        recordId: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldApplyRecord : record, withRecordIdentifier : recordId)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ISyncFilter(pub id);
impl std::ops::Deref for ISyncFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ISyncFilter {}
impl ISyncFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncFilter").unwrap(), alloc) })
    }
}
impl INSObject for ISyncFilter {}
impl PNSObject for ISyncFilter {}
impl std::convert::TryFrom<NSObject> for ISyncFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ISyncFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ISyncFilter").unwrap()) };
        if is_kind_of {
            Ok(ISyncFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ISyncFilter")
        }
    }
}
impl IISyncFilter for ISyncFilter {}
pub trait IISyncFilter: Sized + std::ops::Deref {
    unsafe fn filterMatchingAllFilters_(filters: NSArray) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncFilter").unwrap(), filterMatchingAllFilters : filters)
    }
    unsafe fn filterMatchingAtLeastOneFilter_(filters: NSArray) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncFilter").unwrap(), filterMatchingAtLeastOneFilter : filters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ISyncManager(pub id);
impl std::ops::Deref for ISyncManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ISyncManager {}
impl ISyncManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncManager").unwrap(), alloc) })
    }
}
impl INSObject for ISyncManager {}
impl PNSObject for ISyncManager {}
impl std::convert::TryFrom<NSObject> for ISyncManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ISyncManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ISyncManager").unwrap()) };
        if is_kind_of {
            Ok(ISyncManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ISyncManager")
        }
    }
}
impl IISyncManager for ISyncManager {}
pub trait IISyncManager: Sized + std::ops::Deref {
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn syncDisabledReason(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, syncDisabledReason)
    }
    unsafe fn clientWithIdentifier_(&self, clientId: NSString) -> ISyncClient
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientWithIdentifier : clientId)
    }
    unsafe fn registerClientWithIdentifier_descriptionFilePath_(
        &self,
        clientId: NSString,
        descriptionFilePath: NSString,
    ) -> ISyncClient
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerClientWithIdentifier : clientId, descriptionFilePath : descriptionFilePath)
    }
    unsafe fn unregisterClient_(&self, client: ISyncClient)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterClient : client)
    }
    unsafe fn registerSchemaWithBundlePath_(&self, bundlePath: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerSchemaWithBundlePath : bundlePath)
    }
    unsafe fn unregisterSchemaWithName_(&self, schemaName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterSchemaWithName : schemaName)
    }
    unsafe fn clientWithIdentifier_needsSyncing_(&self, clientId: NSString, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientWithIdentifier : clientId, needsSyncing : flag)
    }
    unsafe fn snapshotOfRecordsInTruthWithEntityNames_usingIdentifiersForClient_(
        &self,
        entityNames: NSArray,
        client: ISyncClient,
    ) -> ISyncRecordSnapshot
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, snapshotOfRecordsInTruthWithEntityNames : entityNames, usingIdentifiersForClient : client)
    }
    unsafe fn addRequestMode_(&self, mode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRequestMode : mode)
    }
    unsafe fn removeRequestMode_(&self, mode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRequestMode : mode)
    }
    unsafe fn requestModes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestModes)
    }
    unsafe fn sharedManager() -> ISyncManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncManager").unwrap(), sharedManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ISyncRecordSnapshot(pub id);
impl std::ops::Deref for ISyncRecordSnapshot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ISyncRecordSnapshot {}
impl ISyncRecordSnapshot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncRecordSnapshot").unwrap(), alloc) })
    }
}
impl INSObject for ISyncRecordSnapshot {}
impl PNSObject for ISyncRecordSnapshot {}
impl std::convert::TryFrom<NSObject> for ISyncRecordSnapshot {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ISyncRecordSnapshot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ISyncRecordSnapshot").unwrap()) };
        if is_kind_of {
            Ok(ISyncRecordSnapshot(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ISyncRecordSnapshot")
        }
    }
}
impl IISyncRecordSnapshot for ISyncRecordSnapshot {}
pub trait IISyncRecordSnapshot: Sized + std::ops::Deref {
    unsafe fn recordsWithIdentifiers_(&self, recordIds: NSArray) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordsWithIdentifiers : recordIds)
    }
    unsafe fn targetIdentifiersForRelationshipName_withSourceIdentifier_(
        &self,
        relationshipName: NSString,
        sourceId: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, targetIdentifiersForRelationshipName : relationshipName, withSourceIdentifier : sourceId)
    }
    unsafe fn sourceIdentifiersForRelationshipName_withTargetIdentifier_(
        &self,
        relationshipName: NSString,
        sourceId: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sourceIdentifiersForRelationshipName : relationshipName, withTargetIdentifier : sourceId)
    }
    unsafe fn recordsWithMatchingAttributes_(&self, attributes: NSDictionary) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordsWithMatchingAttributes : attributes)
    }
}
impl ISyncRecordSnapshot_ISyncRecordReference for ISyncRecordSnapshot {}
pub trait ISyncRecordSnapshot_ISyncRecordReference: Sized + std::ops::Deref {
    unsafe fn recordReferenceForRecordWithIdentifier_(
        &self,
        identifier: NSString,
    ) -> ISyncRecordReference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordReferenceForRecordWithIdentifier : identifier)
    }
    unsafe fn recordIdentifierForReference_isModified_(
        &self,
        reference: ISyncRecordReference,
        pModified: *mut BOOL,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordIdentifierForReference : reference, isModified : pModified)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ISyncRecordReference(pub id);
impl std::ops::Deref for ISyncRecordReference {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ISyncRecordReference {}
impl ISyncRecordReference {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncRecordReference").unwrap(), alloc) })
    }
}
impl PNSCoding for ISyncRecordReference {}
impl INSObject for ISyncRecordReference {}
impl PNSObject for ISyncRecordReference {}
impl std::convert::TryFrom<NSObject> for ISyncRecordReference {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ISyncRecordReference, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ISyncRecordReference").unwrap()) };
        if is_kind_of {
            Ok(ISyncRecordReference(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ISyncRecordReference")
        }
    }
}
impl IISyncRecordReference for ISyncRecordReference {}
pub trait IISyncRecordReference: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ISyncSession(pub id);
impl std::ops::Deref for ISyncSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ISyncSession {}
impl ISyncSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncSession").unwrap(), alloc) })
    }
}
impl INSObject for ISyncSession {}
impl PNSObject for ISyncSession {}
impl std::convert::TryFrom<NSObject> for ISyncSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ISyncSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ISyncSession").unwrap()) };
        if is_kind_of {
            Ok(ISyncSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ISyncSession")
        }
    }
}
impl IISyncSession for ISyncSession {}
pub trait IISyncSession: Sized + std::ops::Deref {
    unsafe fn clientDidResetEntityNames_(&self, entityNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientDidResetEntityNames : entityNames)
    }
    unsafe fn clientWantsToPushAllRecordsForEntityNames_(&self, entityNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientWantsToPushAllRecordsForEntityNames : entityNames)
    }
    unsafe fn shouldPushChangesForEntityName_(&self, entityName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldPushChangesForEntityName : entityName)
    }
    unsafe fn shouldPushAllRecordsForEntityName_(&self, entityName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldPushAllRecordsForEntityName : entityName)
    }
    unsafe fn shouldPullChangesForEntityName_(&self, entityName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldPullChangesForEntityName : entityName)
    }
    unsafe fn shouldReplaceAllRecordsOnClientForEntityName_(&self, entityName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldReplaceAllRecordsOnClientForEntityName : entityName)
    }
    unsafe fn pushChange_(&self, change: ISyncChange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushChange : change)
    }
    unsafe fn pushChangesFromRecord_withIdentifier_(&self, record: NSDictionary, recordId: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushChangesFromRecord : record, withIdentifier : recordId)
    }
    unsafe fn deleteRecordWithIdentifier_(&self, recordId: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteRecordWithIdentifier : recordId)
    }
    unsafe fn clientLostRecordWithIdentifier_shouldReplaceOnNextSync_(
        &self,
        recordId: NSString,
        flag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientLostRecordWithIdentifier : recordId, shouldReplaceOnNextSync : flag)
    }
    unsafe fn clientFinishedPushingChangesWithNextAnchors_(&self, anchors: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientFinishedPushingChangesWithNextAnchors : anchors)
    }
    unsafe fn prepareToPullChangesForEntityNames_beforeDate_(
        &self,
        entityNames: NSArray,
        date: NSDate,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareToPullChangesForEntityNames : entityNames, beforeDate : date)
    }
    unsafe fn prepareToPullChangesInBackgroundForEntityNames_target_selector_(
        &self,
        entityNames: NSArray,
        target: id,
        selector: objc2::runtime::Sel,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareToPullChangesInBackgroundForEntityNames : entityNames, target : target, selector : selector)
    }
    unsafe fn changeEnumeratorForEntityNames_(&self, entityNames: NSArray) -> NSEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeEnumeratorForEntityNames : entityNames)
    }
    unsafe fn clientAcceptedChangesForRecordWithIdentifier_formattedRecord_newRecordIdentifier_(
        &self,
        recordId: NSString,
        formattedRecord: NSDictionary,
        recordId2: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientAcceptedChangesForRecordWithIdentifier : recordId, formattedRecord : formattedRecord, newRecordIdentifier : recordId2)
    }
    unsafe fn clientRefusedChangesForRecordWithIdentifier_(&self, recordId: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientRefusedChangesForRecordWithIdentifier : recordId)
    }
    unsafe fn clientCommittedAcceptedChanges(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientCommittedAcceptedChanges)
    }
    unsafe fn clientCommittedAcceptedChangesWithNextAnchors_(&self, anchors: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientCommittedAcceptedChangesWithNextAnchors : anchors)
    }
    unsafe fn clientChangedRecordIdentifiers_(&self, oldToNew: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientChangedRecordIdentifiers : oldToNew)
    }
    unsafe fn isCancelled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCancelled)
    }
    unsafe fn cancelSyncing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelSyncing)
    }
    unsafe fn finishSyncing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finishSyncing)
    }
    unsafe fn clientInfoForRecordWithIdentifier_(&self, recordId: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientInfoForRecordWithIdentifier : recordId)
    }
    unsafe fn setClientInfo_forRecordWithIdentifier_(
        &self,
        clientInfo: *mut u64,
        recordId: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClientInfo : clientInfo, forRecordWithIdentifier : recordId)
    }
    unsafe fn snapshotOfRecordsInTruth(&self) -> ISyncRecordSnapshot
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotOfRecordsInTruth)
    }
    unsafe fn ping(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ping)
    }
    unsafe fn beginSessionWithClient_entityNames_beforeDate_(
        client: ISyncClient,
        entityNames: NSArray,
        date: NSDate,
    ) -> ISyncSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncSession").unwrap(), beginSessionWithClient : client, entityNames : entityNames, beforeDate : date)
    }
    unsafe fn beginSessionInBackgroundWithClient_entityNames_target_selector_(
        client: ISyncClient,
        entityNames: NSArray,
        target: id,
        selector: objc2::runtime::Sel,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncSession").unwrap(), beginSessionInBackgroundWithClient : client, entityNames : entityNames, target : target, selector : selector)
    }
    unsafe fn cancelPreviousBeginSessionWithClient_(client: ISyncClient)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncSession").unwrap(), cancelPreviousBeginSessionWithClient : client)
    }
    unsafe fn beginSessionWithClient_entityNames_beforeDate_lastAnchors_(
        client: ISyncClient,
        entityNames: NSArray,
        date: NSDate,
        anchors: NSDictionary,
    ) -> ISyncSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncSession").unwrap(), beginSessionWithClient : client, entityNames : entityNames, beforeDate : date, lastAnchors : anchors)
    }
    unsafe fn beginSessionInBackgroundWithClient_entityNames_target_selector_lastAnchors_(
        client: ISyncClient,
        entityNames: NSArray,
        target: id,
        selector: objc2::runtime::Sel,
        anchors: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncSession").unwrap(), beginSessionInBackgroundWithClient : client, entityNames : entityNames, target : target, selector : selector, lastAnchors : anchors)
    }
}
pub type ISyncSessionDriverMode = ::std::os::raw::c_uint;
pub type ISyncSessionDriverChangeResult = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ISyncSessionDriver(pub id);
impl std::ops::Deref for ISyncSessionDriver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ISyncSessionDriver {}
impl ISyncSessionDriver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncSessionDriver").unwrap(), alloc) })
    }
}
impl INSObject for ISyncSessionDriver {}
impl PNSObject for ISyncSessionDriver {}
impl std::convert::TryFrom<NSObject> for ISyncSessionDriver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ISyncSessionDriver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ISyncSessionDriver").unwrap()) };
        if is_kind_of {
            Ok(ISyncSessionDriver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ISyncSessionDriver")
        }
    }
}
impl IISyncSessionDriver for ISyncSessionDriver {}
pub trait IISyncSessionDriver: Sized + std::ops::Deref {
    unsafe fn sync(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sync)
    }
    unsafe fn startAsynchronousSync_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAsynchronousSync : outError)
    }
    unsafe fn lastError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastError)
    }
    unsafe fn dataSource(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSource)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setHandlesSyncAlerts_(&self, yesOrNo: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandlesSyncAlerts : yesOrNo)
    }
    unsafe fn handlesSyncAlerts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handlesSyncAlerts)
    }
    unsafe fn client(&self) -> ISyncClient
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, client)
    }
    unsafe fn session(&self) -> ISyncSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, session)
    }
    unsafe fn finishSyncing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finishSyncing)
    }
    unsafe fn sessionDriverWithDataSource_(dataSource: *mut u64) -> ISyncSessionDriver
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ISyncSessionDriver").unwrap(), sessionDriverWithDataSource : dataSource)
    }
}
pub trait PISyncSessionDriverDataSource: Sized + std::ops::Deref {
    unsafe fn clientIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientIdentifier)
    }
    unsafe fn clientDescriptionURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientDescriptionURL)
    }
    unsafe fn schemaBundleURLs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, schemaBundleURLs)
    }
    unsafe fn preferredSyncModeForEntityName_(&self, entity: NSString) -> ISyncSessionDriverMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preferredSyncModeForEntityName : entity)
    }
    unsafe fn recordsForEntityName_moreComing_error_(
        &self,
        entity: NSString,
        moreComing: *mut BOOL,
        outError: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordsForEntityName : entity, moreComing : moreComing, error : outError)
    }
    unsafe fn applyChange_forEntityName_remappedRecordIdentifier_formattedRecord_error_(
        &self,
        change: ISyncChange,
        entityName: NSString,
        outRecordIdentifier: *mut NSString,
        outRecord: *mut NSDictionary,
        outError: *mut NSError,
    ) -> ISyncSessionDriverChangeResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyChange : change, forEntityName : entityName, remappedRecordIdentifier : outRecordIdentifier, formattedRecord : outRecord, error : outError)
    }
    unsafe fn deleteAllRecordsForEntityName_error_(
        &self,
        entityName: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteAllRecordsForEntityName : entityName, error : outError)
    }
    unsafe fn entityNamesToSync(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityNamesToSync)
    }
    unsafe fn entityNamesToPull(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityNamesToPull)
    }
    unsafe fn sessionBeginTimeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionBeginTimeout)
    }
    unsafe fn sessionPullChangesTimeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionPullChangesTimeout)
    }
    unsafe fn lastAnchorForEntityName_(&self, entityName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lastAnchorForEntityName : entityName)
    }
    unsafe fn nextAnchorForEntityName_(&self, entityName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nextAnchorForEntityName : entityName)
    }
    unsafe fn changedRecordsForEntityName_moreComing_error_(
        &self,
        entity: NSString,
        moreComing: *mut BOOL,
        outError: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changedRecordsForEntityName : entity, moreComing : moreComing, error : outError)
    }
    unsafe fn changesForEntityName_moreComing_error_(
        &self,
        entity: NSString,
        moreComing: *mut BOOL,
        outError: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changesForEntityName : entity, moreComing : moreComing, error : outError)
    }
    unsafe fn identifiersForRecordsToDeleteForEntityName_moreComing_error_(
        &self,
        entityName: NSString,
        moreComing: *mut BOOL,
        outError: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, identifiersForRecordsToDeleteForEntityName : entityName, moreComing : moreComing, error : outError)
    }
}
pub trait NSObject_ISyncSessionDriverDelegate: Sized + std::ops::Deref {
    unsafe fn sessionDriver_didRegisterClientAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, didRegisterClientAndReturnError : outError)
    }
    unsafe fn sessionDriver_willNegotiateAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, willNegotiateAndReturnError : outError)
    }
    unsafe fn sessionDriver_didNegotiateAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, didNegotiateAndReturnError : outError)
    }
    unsafe fn sessionDriver_willPushAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, willPushAndReturnError : outError)
    }
    unsafe fn sessionDriver_didPushAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, didPushAndReturnError : outError)
    }
    unsafe fn sessionDriver_willPullAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, willPullAndReturnError : outError)
    }
    unsafe fn sessionDriver_didPullAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, didPullAndReturnError : outError)
    }
    unsafe fn sessionDriver_willFinishSessionAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, willFinishSessionAndReturnError : outError)
    }
    unsafe fn sessionDriverDidFinishSession_(&self, sender: ISyncSessionDriver)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriverDidFinishSession : sender)
    }
    unsafe fn sessionDriverWillCancelSession_(&self, sender: ISyncSessionDriver)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriverWillCancelSession : sender)
    }
    unsafe fn sessionDriverDidCancelSession_(&self, sender: ISyncSessionDriver)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriverDidCancelSession : sender)
    }
    unsafe fn sessionDriver_didReceiveSyncAlertAndReturnError_(
        &self,
        sender: ISyncSessionDriver,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDriver : sender, didReceiveSyncAlertAndReturnError : outError)
    }
}
pub trait NSPersistentStoreCoordinator_CoreDataSync: Sized + std::ops::Deref {
    unsafe fn syncWithClient_inBackground_handler_error_(
        &self,
        client: ISyncClient,
        flag: BOOL,
        syncHandler: *mut u64,
        rError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, syncWithClient : client, inBackground : flag, handler : syncHandler, error : rError)
    }
    unsafe fn setStoresFastSyncDetailsAtURL_forPersistentStore_(
        &self,
        url: NSURL,
        store: NSPersistentStore,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStoresFastSyncDetailsAtURL : url, forPersistentStore : store)
    }
}
pub trait PNSPersistentStoreCoordinatorSyncing: Sized + std::ops::Deref {
    unsafe fn managedObjectContextsToMonitorWhenSyncingPersistentStoreCoordinator_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, managedObjectContextsToMonitorWhenSyncingPersistentStoreCoordinator : coordinator)
    }
    unsafe fn managedObjectContextsToReloadAfterSyncingPersistentStoreCoordinator_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, managedObjectContextsToReloadAfterSyncingPersistentStoreCoordinator : coordinator)
    }
    unsafe fn persistentStoreCoordinatorShouldStartSyncing_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinatorShouldStartSyncing : coordinator)
    }
    unsafe fn persistentStoreCoordinator_willPushChangesInSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        session: ISyncSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, willPushChangesInSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_didPushChangesInSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        session: ISyncSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, didPushChangesInSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_willPullChangesInSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        session: ISyncSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, willPullChangesInSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_didPullChangesInSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        session: ISyncSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, didPullChangesInSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_didFinishSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        session: ISyncSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, didFinishSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_didCancelSyncSession_error_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        session: ISyncSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, didCancelSyncSession : session, error : error)
    }
    unsafe fn persistentStoreCoordinator_willPushRecord_forManagedObject_inSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        record: NSDictionary,
        managedObject: NSManagedObject,
        session: ISyncSession,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, willPushRecord : record, forManagedObject : managedObject, inSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_willDeleteRecordWithIdentifier_inSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        identifier: NSString,
        session: ISyncSession,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, willDeleteRecordWithIdentifier : identifier, inSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_willApplyChange_toManagedObject_inSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        change: ISyncChange,
        managedObject: NSManagedObject,
        session: ISyncSession,
    ) -> ISyncChange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, willApplyChange : change, toManagedObject : managedObject, inSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_didApplyChange_toManagedObject_inSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        change: ISyncChange,
        managedObject: NSManagedObject,
        session: ISyncSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, didApplyChange : change, toManagedObject : managedObject, inSyncSession : session)
    }
    unsafe fn persistentStoreCoordinator_didCommitChanges_inSyncSession_(
        &self,
        coordinator: NSPersistentStoreCoordinator,
        changes: NSDictionary,
        session: ISyncSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentStoreCoordinator : coordinator, didCommitChanges : changes, inSyncSession : session)
    }
}
unsafe extern "C" {
    pub static ISyncChangePropertyActionKey: NSString;
}
unsafe extern "C" {
    pub static ISyncChangePropertySet: NSString;
}
unsafe extern "C" {
    pub static ISyncChangePropertyClear: NSString;
}
unsafe extern "C" {
    pub static ISyncChangePropertyNameKey: NSString;
}
unsafe extern "C" {
    pub static ISyncChangePropertyValueKey: NSString;
}
unsafe extern "C" {
    pub static ISyncChangePropertyValueIsDefaultKey: NSString;
}
unsafe extern "C" {
    pub static ISyncClientTypeApplication: NSString;
}
unsafe extern "C" {
    pub static ISyncClientTypeDevice: NSString;
}
unsafe extern "C" {
    pub static ISyncClientTypeServer: NSString;
}
unsafe extern "C" {
    pub static ISyncClientTypePeer: NSString;
}
unsafe extern "C" {
    pub static ISyncAvailabilityChangedNotification: NSString;
}
unsafe extern "C" {
    pub static ISyncServerUnavailableException: NSString;
}
unsafe extern "C" {
    pub static ISyncSessionCancelledException: NSString;
}
unsafe extern "C" {
    pub static ISyncSessionUnavailableException: NSString;
}
unsafe extern "C" {
    pub static ISyncInvalidRecordException: NSString;
}
unsafe extern "C" {
    pub static ISyncInvalidRecordIdentifiersKey: NSString;
}
unsafe extern "C" {
    pub static ISyncInvalidRecordReasonsKey: NSString;
}
unsafe extern "C" {
    pub static ISyncInvalidRecordsKey: NSString;
}
unsafe extern "C" {
    pub static ISyncInvalidEntityException: NSString;
}
unsafe extern "C" {
    pub static ISyncUnsupportedEntityException: NSString;
}
unsafe extern "C" {
    pub static ISyncRecordEntityNameKey: NSString;
}
unsafe extern "C" {
    pub static ISyncErrorDomain: NSString;
}
unsafe extern "C" {
    pub static ISyncInvalidSchemaException: NSString;
}
unsafe extern "C" {
    pub static ISyncInvalidArgumentsException: NSString;
}

unsafe impl objc2::encode::RefEncode for ISyncChange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ISyncChange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ISyncClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ISyncClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ISyncFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ISyncFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ISyncManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ISyncManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ISyncRecordSnapshot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ISyncRecordSnapshot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ISyncRecordReference {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ISyncRecordReference {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ISyncSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ISyncSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ISyncSessionDriver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ISyncSessionDriver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
