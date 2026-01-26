#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Contacts::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKAsset(pub id);
impl std::ops::Deref for CKAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKAsset {}
impl CKAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKAsset").unwrap(), alloc) })
    }
}
impl INSObject for CKAsset {}
impl PNSObject for CKAsset {}
impl std::convert::TryFrom<NSObject> for CKAsset {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKAsset, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKAsset").unwrap()) };
        if is_kind_of {
            Ok(CKAsset(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKAsset")
        }
    }
}
impl ICKAsset for CKAsset {}
pub trait ICKAsset: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFileURL_(&self, fileURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileURL : fileURL)
    }
    unsafe fn fileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileURL)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKAsset").unwrap(), new)
    }
}
pub type CKReferenceAction = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKReference(pub id);
impl std::ops::Deref for CKReference {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKReference {}
impl CKReference {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKReference").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKReference {}
impl PNSCopying for CKReference {}
impl INSObject for CKReference {}
impl PNSObject for CKReference {}
impl std::convert::TryFrom<NSObject> for CKReference {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKReference, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKReference").unwrap()) };
        if is_kind_of {
            Ok(CKReference(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKReference")
        }
    }
}
impl ICKReference for CKReference {}
pub trait ICKReference: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordID_action_(
        &self,
        recordID: CKRecordID,
        action: CKReferenceAction,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordID : recordID, action : action)
    }
    unsafe fn initWithRecord_action_(
        &self,
        record: CKRecord,
        action: CKReferenceAction,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecord : record, action : action)
    }
    unsafe fn referenceAction(&self) -> CKReferenceAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, referenceAction)
    }
    unsafe fn recordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordID)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKReference").unwrap(), new)
    }
}
pub type CKRecordType = NSString;
pub type CKRecordFieldKey = NSString;
pub trait PCKRecordValue: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKRecord(pub id);
impl std::ops::Deref for CKRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKRecord {}
impl CKRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecord").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKRecord {}
impl PNSCopying for CKRecord {}
impl INSObject for CKRecord {}
impl PNSObject for CKRecord {}
impl std::convert::TryFrom<NSObject> for CKRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKRecord, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKRecord").unwrap()) };
        if is_kind_of {
            Ok(CKRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKRecord")
        }
    }
}
impl ICKRecord for CKRecord {}
pub trait ICKRecord: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordType_(&self, recordType: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType)
    }
    unsafe fn initWithRecordType_recordID_(
        &self,
        recordType: NSString,
        recordID: CKRecordID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType, recordID : recordID)
    }
    unsafe fn initWithRecordType_zoneID_(
        &self,
        recordType: NSString,
        zoneID: CKRecordZoneID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType, zoneID : zoneID)
    }
    unsafe fn objectForKey_(&self, key: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKey : key)
    }
    unsafe fn setObject_forKey_(&self, object: *mut u64, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : object, forKey : key)
    }
    unsafe fn allKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allKeys)
    }
    unsafe fn allTokens(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allTokens)
    }
    unsafe fn objectForKeyedSubscript_(&self, key: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, object: *mut u64, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : object, forKeyedSubscript : key)
    }
    unsafe fn changedKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedKeys)
    }
    unsafe fn encodeSystemFieldsWithCoder_(&self, coder: NSCoder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeSystemFieldsWithCoder : coder)
    }
    unsafe fn setParentReferenceFromRecord_(&self, parentRecord: CKRecord)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParentReferenceFromRecord : parentRecord)
    }
    unsafe fn setParentReferenceFromRecordID_(&self, parentRecordID: CKRecordID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParentReferenceFromRecordID : parentRecordID)
    }
    unsafe fn recordType(&self) -> CKRecordType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordType)
    }
    unsafe fn recordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordID)
    }
    unsafe fn recordChangeTag(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordChangeTag)
    }
    unsafe fn creatorUserRecordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creatorUserRecordID)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn lastModifiedUserRecordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastModifiedUserRecordID)
    }
    unsafe fn modificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modificationDate)
    }
    unsafe fn share(&self) -> CKReference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, share)
    }
    unsafe fn parent(&self) -> CKReference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parent)
    }
    unsafe fn setParent_(&self, parent: CKReference)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParent : parent)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecord").unwrap(), new)
    }
}
impl CKReference_CKRecordValue for CKReference {}
impl CKAsset_CKRecordValue for CKAsset {}
pub trait PCKRecordKeyValueSetting: Sized + std::ops::Deref {
    unsafe fn objectForKey_(&self, key: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKey : key)
    }
    unsafe fn setObject_forKey_(&self, object: *mut u64, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : object, forKey : key)
    }
    unsafe fn objectForKeyedSubscript_(&self, key: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, object: *mut u64, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : object, forKeyedSubscript : key)
    }
    unsafe fn allKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allKeys)
    }
    unsafe fn changedKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedKeys)
    }
}
impl CKRecord_CKRecordKeyValueSettingConformance for CKRecord {}
pub type CKSubscriptionType = NSInteger;
pub type CKSubscriptionID = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSubscription(pub id);
impl std::ops::Deref for CKSubscription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSubscription {}
impl CKSubscription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSubscription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKSubscription {}
impl PNSCopying for CKSubscription {}
impl INSObject for CKSubscription {}
impl PNSObject for CKSubscription {}
impl std::convert::TryFrom<NSObject> for CKSubscription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSubscription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSubscription").unwrap()) };
        if is_kind_of {
            Ok(CKSubscription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSubscription")
        }
    }
}
impl ICKSubscription for CKSubscription {}
pub trait ICKSubscription: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn subscriptionID(&self) -> CKSubscriptionID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionID)
    }
    unsafe fn subscriptionType(&self) -> CKSubscriptionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionType)
    }
    unsafe fn notificationInfo(&self) -> CKNotificationInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationInfo)
    }
    unsafe fn setNotificationInfo_(&self, notificationInfo: CKNotificationInfo)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotificationInfo : notificationInfo)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSubscription").unwrap(), new)
    }
}
pub type CKQuerySubscriptionOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKQuerySubscription(pub id);
impl std::ops::Deref for CKQuerySubscription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKQuerySubscription {}
impl CKQuerySubscription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKQuerySubscription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKQuerySubscription {}
impl PNSCopying for CKQuerySubscription {}
impl ICKSubscription for CKQuerySubscription {}
impl From<CKQuerySubscription> for CKSubscription {
    fn from(child: CKQuerySubscription) -> CKSubscription {
        CKSubscription(child.0)
    }
}
impl std::convert::TryFrom<CKSubscription> for CKQuerySubscription {
    type Error = &'static str;
    fn try_from(parent: CKSubscription) -> Result<CKQuerySubscription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKQuerySubscription").unwrap()) };
        if is_kind_of {
            Ok(CKQuerySubscription(parent.0))
        } else {
            Err("This CKSubscription cannot be downcasted to CKQuerySubscription")
        }
    }
}
impl INSObject for CKQuerySubscription {}
impl PNSObject for CKQuerySubscription {}
impl ICKQuerySubscription for CKQuerySubscription {}
pub trait ICKQuerySubscription: Sized + std::ops::Deref {
    unsafe fn initWithRecordType_predicate_options_(
        &self,
        recordType: NSString,
        predicate: NSPredicate,
        querySubscriptionOptions: CKQuerySubscriptionOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType, predicate : predicate, options : querySubscriptionOptions)
    }
    unsafe fn initWithRecordType_predicate_subscriptionID_options_(
        &self,
        recordType: NSString,
        predicate: NSPredicate,
        subscriptionID: NSString,
        querySubscriptionOptions: CKQuerySubscriptionOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType, predicate : predicate, subscriptionID : subscriptionID, options : querySubscriptionOptions)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn recordType(&self) -> CKRecordType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordType)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
    }
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
    unsafe fn setZoneID_(&self, zoneID: CKRecordZoneID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoneID : zoneID)
    }
    unsafe fn querySubscriptionOptions(&self) -> CKQuerySubscriptionOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, querySubscriptionOptions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKRecordZoneSubscription(pub id);
impl std::ops::Deref for CKRecordZoneSubscription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKRecordZoneSubscription {}
impl CKRecordZoneSubscription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordZoneSubscription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKRecordZoneSubscription {}
impl PNSCopying for CKRecordZoneSubscription {}
impl ICKSubscription for CKRecordZoneSubscription {}
impl std::convert::TryFrom<CKSubscription> for CKRecordZoneSubscription {
    type Error = &'static str;
    fn try_from(parent: CKSubscription) -> Result<CKRecordZoneSubscription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKRecordZoneSubscription").unwrap()) };
        if is_kind_of {
            Ok(CKRecordZoneSubscription(parent.0))
        } else {
            Err("This CKSubscription cannot be downcasted to CKRecordZoneSubscription")
        }
    }
}
impl INSObject for CKRecordZoneSubscription {}
impl PNSObject for CKRecordZoneSubscription {}
impl ICKRecordZoneSubscription for CKRecordZoneSubscription {}
pub trait ICKRecordZoneSubscription: Sized + std::ops::Deref {
    unsafe fn initWithZoneID_(&self, zoneID: CKRecordZoneID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZoneID : zoneID)
    }
    unsafe fn initWithZoneID_subscriptionID_(
        &self,
        zoneID: CKRecordZoneID,
        subscriptionID: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZoneID : zoneID, subscriptionID : subscriptionID)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
    unsafe fn recordType(&self) -> CKRecordType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordType)
    }
    unsafe fn setRecordType_(&self, recordType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordType : recordType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKDatabaseSubscription(pub id);
impl std::ops::Deref for CKDatabaseSubscription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKDatabaseSubscription {}
impl CKDatabaseSubscription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKDatabaseSubscription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKDatabaseSubscription {}
impl PNSCopying for CKDatabaseSubscription {}
impl ICKSubscription for CKDatabaseSubscription {}
impl std::convert::TryFrom<CKSubscription> for CKDatabaseSubscription {
    type Error = &'static str;
    fn try_from(parent: CKSubscription) -> Result<CKDatabaseSubscription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKDatabaseSubscription").unwrap()) };
        if is_kind_of {
            Ok(CKDatabaseSubscription(parent.0))
        } else {
            Err("This CKSubscription cannot be downcasted to CKDatabaseSubscription")
        }
    }
}
impl INSObject for CKDatabaseSubscription {}
impl PNSObject for CKDatabaseSubscription {}
impl ICKDatabaseSubscription for CKDatabaseSubscription {}
pub trait ICKDatabaseSubscription: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSubscriptionID_(&self, subscriptionID: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSubscriptionID : subscriptionID)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn recordType(&self) -> CKRecordType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordType)
    }
    unsafe fn setRecordType_(&self, recordType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordType : recordType)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKDatabaseSubscription").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKNotificationInfo(pub id);
impl std::ops::Deref for CKNotificationInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKNotificationInfo {}
impl CKNotificationInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKNotificationInfo").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKNotificationInfo {}
impl PNSCopying for CKNotificationInfo {}
impl INSObject for CKNotificationInfo {}
impl PNSObject for CKNotificationInfo {}
impl std::convert::TryFrom<NSObject> for CKNotificationInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKNotificationInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKNotificationInfo").unwrap()) };
        if is_kind_of {
            Ok(CKNotificationInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKNotificationInfo")
        }
    }
}
impl ICKNotificationInfo for CKNotificationInfo {}
pub trait ICKNotificationInfo: Sized + std::ops::Deref {
    unsafe fn alertBody(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertBody)
    }
    unsafe fn setAlertBody_(&self, alertBody: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlertBody : alertBody)
    }
    unsafe fn alertLocalizationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertLocalizationKey)
    }
    unsafe fn setAlertLocalizationKey_(&self, alertLocalizationKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlertLocalizationKey : alertLocalizationKey)
    }
    unsafe fn alertLocalizationArgs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertLocalizationArgs)
    }
    unsafe fn setAlertLocalizationArgs_(&self, alertLocalizationArgs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlertLocalizationArgs : alertLocalizationArgs)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn titleLocalizationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleLocalizationKey)
    }
    unsafe fn setTitleLocalizationKey_(&self, titleLocalizationKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleLocalizationKey : titleLocalizationKey)
    }
    unsafe fn titleLocalizationArgs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleLocalizationArgs)
    }
    unsafe fn setTitleLocalizationArgs_(&self, titleLocalizationArgs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleLocalizationArgs : titleLocalizationArgs)
    }
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn setSubtitle_(&self, subtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitle : subtitle)
    }
    unsafe fn subtitleLocalizationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleLocalizationKey)
    }
    unsafe fn setSubtitleLocalizationKey_(&self, subtitleLocalizationKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitleLocalizationKey : subtitleLocalizationKey)
    }
    unsafe fn subtitleLocalizationArgs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleLocalizationArgs)
    }
    unsafe fn setSubtitleLocalizationArgs_(&self, subtitleLocalizationArgs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitleLocalizationArgs : subtitleLocalizationArgs)
    }
    unsafe fn alertActionLocalizationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertActionLocalizationKey)
    }
    unsafe fn setAlertActionLocalizationKey_(&self, alertActionLocalizationKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlertActionLocalizationKey : alertActionLocalizationKey)
    }
    unsafe fn alertLaunchImage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertLaunchImage)
    }
    unsafe fn setAlertLaunchImage_(&self, alertLaunchImage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlertLaunchImage : alertLaunchImage)
    }
    unsafe fn soundName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, soundName)
    }
    unsafe fn setSoundName_(&self, soundName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoundName : soundName)
    }
    unsafe fn desiredKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredKeys)
    }
    unsafe fn setDesiredKeys_(&self, desiredKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredKeys : desiredKeys)
    }
    unsafe fn shouldBadge(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBadge)
    }
    unsafe fn setShouldBadge_(&self, shouldBadge: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldBadge : shouldBadge)
    }
    unsafe fn shouldSendContentAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldSendContentAvailable)
    }
    unsafe fn setShouldSendContentAvailable_(&self, shouldSendContentAvailable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldSendContentAvailable : shouldSendContentAvailable)
    }
    unsafe fn shouldSendMutableContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldSendMutableContent)
    }
    unsafe fn setShouldSendMutableContent_(&self, shouldSendMutableContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldSendMutableContent : shouldSendMutableContent)
    }
    unsafe fn category(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
    unsafe fn setCategory_(&self, category: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategory : category)
    }
    unsafe fn collapseIDKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collapseIDKey)
    }
    unsafe fn setCollapseIDKey_(&self, collapseIDKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCollapseIDKey : collapseIDKey)
    }
}
pub type CKDatabaseScope = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKDatabase(pub id);
impl std::ops::Deref for CKDatabase {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKDatabase {}
impl CKDatabase {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKDatabase").unwrap(), alloc) })
    }
}
impl INSObject for CKDatabase {}
impl PNSObject for CKDatabase {}
impl std::convert::TryFrom<NSObject> for CKDatabase {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKDatabase, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKDatabase").unwrap()) };
        if is_kind_of {
            Ok(CKDatabase(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKDatabase")
        }
    }
}
impl ICKDatabase for CKDatabase {}
pub trait ICKDatabase: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addOperation_(&self, operation: CKDatabaseOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOperation : operation)
    }
    unsafe fn databaseScope(&self) -> CKDatabaseScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, databaseScope)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKDatabase").unwrap(), new)
    }
}
impl CKDatabase_ConvenienceMethods for CKDatabase {}
pub type CKOperationID = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKOperation(pub id);
impl std::ops::Deref for CKOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKOperation {}
impl CKOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKOperation").unwrap(), alloc) })
    }
}
impl INSOperation for CKOperation {}
impl std::convert::TryFrom<NSOperation> for CKOperation {
    type Error = &'static str;
    fn try_from(parent: NSOperation) -> Result<CKOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKOperation").unwrap()) };
        if is_kind_of {
            Ok(CKOperation(parent.0))
        } else {
            Err("This NSOperation cannot be downcasted to CKOperation")
        }
    }
}
impl INSObject for CKOperation {}
impl PNSObject for CKOperation {}
impl ICKOperation for CKOperation {}
pub trait ICKOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn configuration(&self) -> CKOperationConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn setConfiguration_(&self, configuration: CKOperationConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfiguration : configuration)
    }
    unsafe fn group(&self) -> CKOperationGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, group)
    }
    unsafe fn setGroup_(&self, group: CKOperationGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroup : group)
    }
    unsafe fn operationID(&self) -> CKOperationID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operationID)
    }
    unsafe fn longLivedOperationWasPersistedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longLivedOperationWasPersistedBlock)
    }
    unsafe fn setLongLivedOperationWasPersistedBlock_(
        &self,
        longLivedOperationWasPersistedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLongLivedOperationWasPersistedBlock : longLivedOperationWasPersistedBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKOperationConfiguration(pub id);
impl std::ops::Deref for CKOperationConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKOperationConfiguration {}
impl CKOperationConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKOperationConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for CKOperationConfiguration {}
impl PNSObject for CKOperationConfiguration {}
impl std::convert::TryFrom<NSObject> for CKOperationConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKOperationConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKOperationConfiguration").unwrap()) };
        if is_kind_of {
            Ok(CKOperationConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKOperationConfiguration")
        }
    }
}
impl ICKOperationConfiguration for CKOperationConfiguration {}
pub trait ICKOperationConfiguration: Sized + std::ops::Deref {
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
    unsafe fn qualityOfService(&self) -> NSQualityOfService
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualityOfService)
    }
    unsafe fn setQualityOfService_(&self, qualityOfService: NSQualityOfService)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQualityOfService : qualityOfService)
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
impl CKOperation_CKOperationDeprecated for CKOperation {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKContainer(pub id);
impl std::ops::Deref for CKContainer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKContainer {}
impl CKContainer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKContainer").unwrap(), alloc) })
    }
}
impl INSObject for CKContainer {}
impl PNSObject for CKContainer {}
impl std::convert::TryFrom<NSObject> for CKContainer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKContainer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKContainer").unwrap()) };
        if is_kind_of {
            Ok(CKContainer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKContainer")
        }
    }
}
impl ICKContainer for CKContainer {}
pub trait ICKContainer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addOperation_(&self, operation: CKOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOperation : operation)
    }
    unsafe fn containerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKContainer").unwrap(), new)
    }
    unsafe fn defaultContainer() -> CKContainer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKContainer").unwrap(), defaultContainer)
    }
    unsafe fn containerWithIdentifier_(containerIdentifier: NSString) -> CKContainer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKContainer").unwrap(), containerWithIdentifier : containerIdentifier)
    }
}
impl CKContainer_Database for CKContainer {}
pub type CKAccountStatus = NSInteger;
impl CKContainer_AccountStatus for CKContainer {}
pub type CKApplicationPermissions = NSUInteger;
pub type CKApplicationPermissionStatus = NSInteger;
impl CKContainer_ApplicationPermission for CKContainer {}
impl CKContainer_UserRecords for CKContainer {}
impl CKContainer_Sharing for CKContainer {}
impl CKContainer_CKLongLivedOperations for CKContainer {}
pub type CKErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKLocationSortDescriptor(pub id);
impl std::ops::Deref for CKLocationSortDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKLocationSortDescriptor {}
impl CKLocationSortDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKLocationSortDescriptor").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKLocationSortDescriptor {}
impl INSSortDescriptor for CKLocationSortDescriptor {}
impl PNSCopying for CKLocationSortDescriptor {}
impl std::convert::TryFrom<NSSortDescriptor> for CKLocationSortDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSSortDescriptor) -> Result<CKLocationSortDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKLocationSortDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CKLocationSortDescriptor(parent.0))
        } else {
            Err("This NSSortDescriptor cannot be downcasted to CKLocationSortDescriptor")
        }
    }
}
impl INSObject for CKLocationSortDescriptor {}
impl PNSObject for CKLocationSortDescriptor {}
impl ICKLocationSortDescriptor for CKLocationSortDescriptor {}
pub trait ICKLocationSortDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithKey_relativeLocation_(
        &self,
        key: NSString,
        relativeLocation: CLLocation,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKey : key, relativeLocation : relativeLocation)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn relativeLocation(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeLocation)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKLocationSortDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKNotificationID(pub id);
impl std::ops::Deref for CKNotificationID {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKNotificationID {}
impl CKNotificationID {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKNotificationID").unwrap(), alloc) })
    }
}
impl PNSCopying for CKNotificationID {}
impl PNSSecureCoding for CKNotificationID {}
impl INSObject for CKNotificationID {}
impl PNSObject for CKNotificationID {}
impl std::convert::TryFrom<NSObject> for CKNotificationID {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKNotificationID, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKNotificationID").unwrap()) };
        if is_kind_of {
            Ok(CKNotificationID(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKNotificationID")
        }
    }
}
impl ICKNotificationID for CKNotificationID {}
pub trait ICKNotificationID: Sized + std::ops::Deref {}
pub type CKNotificationType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKNotification(pub id);
impl std::ops::Deref for CKNotification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKNotification {}
impl CKNotification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKNotification").unwrap(), alloc) })
    }
}
impl INSObject for CKNotification {}
impl PNSObject for CKNotification {}
impl std::convert::TryFrom<NSObject> for CKNotification {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKNotification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKNotification").unwrap()) };
        if is_kind_of {
            Ok(CKNotification(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKNotification")
        }
    }
}
impl ICKNotification for CKNotification {}
pub trait ICKNotification: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn notificationType(&self) -> CKNotificationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationType)
    }
    unsafe fn notificationID(&self) -> CKNotificationID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationID)
    }
    unsafe fn containerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerIdentifier)
    }
    unsafe fn subscriptionOwnerUserRecordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionOwnerUserRecordID)
    }
    unsafe fn isPruned(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPruned)
    }
    unsafe fn subscriptionID(&self) -> CKSubscriptionID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionID)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKNotification").unwrap(), new)
    }
    unsafe fn notificationFromRemoteNotificationDictionary_(
        notificationDictionary: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKNotification").unwrap(), notificationFromRemoteNotificationDictionary : notificationDictionary)
    }
}
impl CKNotification_DeprecatedAPSProperties for CKNotification {}
pub type CKQueryNotificationReason = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKQueryNotification(pub id);
impl std::ops::Deref for CKQueryNotification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKQueryNotification {}
impl CKQueryNotification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKQueryNotification").unwrap(), alloc) })
    }
}
impl ICKNotification for CKQueryNotification {}
impl From<CKQueryNotification> for CKNotification {
    fn from(child: CKQueryNotification) -> CKNotification {
        CKNotification(child.0)
    }
}
impl std::convert::TryFrom<CKNotification> for CKQueryNotification {
    type Error = &'static str;
    fn try_from(parent: CKNotification) -> Result<CKQueryNotification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKQueryNotification").unwrap()) };
        if is_kind_of {
            Ok(CKQueryNotification(parent.0))
        } else {
            Err("This CKNotification cannot be downcasted to CKQueryNotification")
        }
    }
}
impl INSObject for CKQueryNotification {}
impl PNSObject for CKQueryNotification {}
impl ICKQueryNotification for CKQueryNotification {}
pub trait ICKQueryNotification: Sized + std::ops::Deref {
    unsafe fn queryNotificationReason(&self) -> CKQueryNotificationReason
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queryNotificationReason)
    }
    unsafe fn recordFields(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordFields)
    }
    unsafe fn recordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordID)
    }
    unsafe fn databaseScope(&self) -> CKDatabaseScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, databaseScope)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKRecordZoneNotification(pub id);
impl std::ops::Deref for CKRecordZoneNotification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKRecordZoneNotification {}
impl CKRecordZoneNotification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordZoneNotification").unwrap(), alloc) })
    }
}
impl ICKNotification for CKRecordZoneNotification {}
impl std::convert::TryFrom<CKNotification> for CKRecordZoneNotification {
    type Error = &'static str;
    fn try_from(parent: CKNotification) -> Result<CKRecordZoneNotification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKRecordZoneNotification").unwrap()) };
        if is_kind_of {
            Ok(CKRecordZoneNotification(parent.0))
        } else {
            Err("This CKNotification cannot be downcasted to CKRecordZoneNotification")
        }
    }
}
impl INSObject for CKRecordZoneNotification {}
impl PNSObject for CKRecordZoneNotification {}
impl ICKRecordZoneNotification for CKRecordZoneNotification {}
pub trait ICKRecordZoneNotification: Sized + std::ops::Deref {
    unsafe fn recordZoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneID)
    }
    unsafe fn databaseScope(&self) -> CKDatabaseScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, databaseScope)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKDatabaseNotification(pub id);
impl std::ops::Deref for CKDatabaseNotification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKDatabaseNotification {}
impl CKDatabaseNotification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKDatabaseNotification").unwrap(), alloc) })
    }
}
impl ICKNotification for CKDatabaseNotification {}
impl std::convert::TryFrom<CKNotification> for CKDatabaseNotification {
    type Error = &'static str;
    fn try_from(parent: CKNotification) -> Result<CKDatabaseNotification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKDatabaseNotification").unwrap()) };
        if is_kind_of {
            Ok(CKDatabaseNotification(parent.0))
        } else {
            Err("This CKNotification cannot be downcasted to CKDatabaseNotification")
        }
    }
}
impl INSObject for CKDatabaseNotification {}
impl PNSObject for CKDatabaseNotification {}
impl ICKDatabaseNotification for CKDatabaseNotification {}
pub trait ICKDatabaseNotification: Sized + std::ops::Deref {
    unsafe fn databaseScope(&self) -> CKDatabaseScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, databaseScope)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKQuery(pub id);
impl std::ops::Deref for CKQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKQuery {}
impl CKQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKQuery").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKQuery {}
impl PNSCopying for CKQuery {}
impl INSObject for CKQuery {}
impl PNSObject for CKQuery {}
impl std::convert::TryFrom<NSObject> for CKQuery {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKQuery, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKQuery").unwrap()) };
        if is_kind_of {
            Ok(CKQuery(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKQuery")
        }
    }
}
impl ICKQuery for CKQuery {}
pub trait ICKQuery: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn initWithRecordType_predicate_(
        &self,
        recordType: NSString,
        predicate: NSPredicate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType, predicate : predicate)
    }
    unsafe fn recordType(&self) -> CKRecordType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordType)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKQuery").unwrap(), new)
    }
}
pub type CKRecordZoneCapabilities = NSUInteger;
pub type CKRecordZoneEncryptionScope = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKRecordZone(pub id);
impl std::ops::Deref for CKRecordZone {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKRecordZone {}
impl CKRecordZone {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordZone").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKRecordZone {}
impl PNSCopying for CKRecordZone {}
impl INSObject for CKRecordZone {}
impl PNSObject for CKRecordZone {}
impl std::convert::TryFrom<NSObject> for CKRecordZone {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKRecordZone, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKRecordZone").unwrap()) };
        if is_kind_of {
            Ok(CKRecordZone(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKRecordZone")
        }
    }
}
impl ICKRecordZone for CKRecordZone {}
pub trait ICKRecordZone: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithZoneName_(&self, zoneName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZoneName : zoneName)
    }
    unsafe fn initWithZoneID_(&self, zoneID: CKRecordZoneID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZoneID : zoneID)
    }
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
    unsafe fn capabilities(&self) -> CKRecordZoneCapabilities
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capabilities)
    }
    unsafe fn share(&self) -> CKReference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, share)
    }
    unsafe fn encryptionScope(&self) -> CKRecordZoneEncryptionScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptionScope)
    }
    unsafe fn setEncryptionScope_(&self, encryptionScope: CKRecordZoneEncryptionScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEncryptionScope : encryptionScope)
    }
    unsafe fn defaultRecordZone() -> CKRecordZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordZone").unwrap(), defaultRecordZone)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordZone").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKRecordID(pub id);
impl std::ops::Deref for CKRecordID {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKRecordID {}
impl CKRecordID {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordID").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKRecordID {}
impl PNSCopying for CKRecordID {}
impl INSObject for CKRecordID {}
impl PNSObject for CKRecordID {}
impl std::convert::TryFrom<NSObject> for CKRecordID {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKRecordID, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKRecordID").unwrap()) };
        if is_kind_of {
            Ok(CKRecordID(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKRecordID")
        }
    }
}
impl ICKRecordID for CKRecordID {}
pub trait ICKRecordID: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordName_(&self, recordName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordName : recordName)
    }
    unsafe fn initWithRecordName_zoneID_(
        &self,
        recordName: NSString,
        zoneID: CKRecordZoneID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordName : recordName, zoneID : zoneID)
    }
    unsafe fn recordName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordName)
    }
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordID").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKRecordZoneID(pub id);
impl std::ops::Deref for CKRecordZoneID {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKRecordZoneID {}
impl CKRecordZoneID {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordZoneID").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKRecordZoneID {}
impl PNSCopying for CKRecordZoneID {}
impl INSObject for CKRecordZoneID {}
impl PNSObject for CKRecordZoneID {}
impl std::convert::TryFrom<NSObject> for CKRecordZoneID {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKRecordZoneID, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKRecordZoneID").unwrap()) };
        if is_kind_of {
            Ok(CKRecordZoneID(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKRecordZoneID")
        }
    }
}
impl ICKRecordZoneID for CKRecordZoneID {}
pub trait ICKRecordZoneID: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithZoneName_ownerName_(
        &self,
        zoneName: NSString,
        ownerName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZoneName : zoneName, ownerName : ownerName)
    }
    unsafe fn zoneName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneName)
    }
    unsafe fn ownerName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerName)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKRecordZoneID").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKDatabaseOperation(pub id);
impl std::ops::Deref for CKDatabaseOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKDatabaseOperation {}
impl CKDatabaseOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKDatabaseOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKDatabaseOperation {}
impl From<CKDatabaseOperation> for CKOperation {
    fn from(child: CKDatabaseOperation) -> CKOperation {
        CKOperation(child.0)
    }
}
impl std::convert::TryFrom<CKOperation> for CKDatabaseOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKDatabaseOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKDatabaseOperation").unwrap()) };
        if is_kind_of {
            Ok(CKDatabaseOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKDatabaseOperation")
        }
    }
}
impl INSOperation for CKDatabaseOperation {}
impl INSObject for CKDatabaseOperation {}
impl PNSObject for CKDatabaseOperation {}
impl ICKDatabaseOperation for CKDatabaseOperation {}
pub trait ICKDatabaseOperation: Sized + std::ops::Deref {
    unsafe fn database(&self) -> CKDatabase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, database)
    }
    unsafe fn setDatabase_(&self, database: CKDatabase)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDatabase : database)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKServerChangeToken(pub id);
impl std::ops::Deref for CKServerChangeToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKServerChangeToken {}
impl CKServerChangeToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKServerChangeToken").unwrap(), alloc) })
    }
}
impl PNSCopying for CKServerChangeToken {}
impl PNSSecureCoding for CKServerChangeToken {}
impl INSObject for CKServerChangeToken {}
impl PNSObject for CKServerChangeToken {}
impl std::convert::TryFrom<NSObject> for CKServerChangeToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKServerChangeToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKServerChangeToken").unwrap()) };
        if is_kind_of {
            Ok(CKServerChangeToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKServerChangeToken")
        }
    }
}
impl ICKServerChangeToken for CKServerChangeToken {}
pub trait ICKServerChangeToken: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKServerChangeToken").unwrap(), new)
    }
}
pub type CKShareParticipantAcceptanceStatus = NSInteger;
pub type CKShareParticipantPermission = NSInteger;
pub type CKShareParticipantRole = NSInteger;
pub type CKShareParticipantType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKShareParticipant(pub id);
impl std::ops::Deref for CKShareParticipant {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKShareParticipant {}
impl CKShareParticipant {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareParticipant").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKShareParticipant {}
impl PNSCopying for CKShareParticipant {}
impl INSObject for CKShareParticipant {}
impl PNSObject for CKShareParticipant {}
impl std::convert::TryFrom<NSObject> for CKShareParticipant {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKShareParticipant, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKShareParticipant").unwrap()) };
        if is_kind_of {
            Ok(CKShareParticipant(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKShareParticipant")
        }
    }
}
impl ICKShareParticipant for CKShareParticipant {}
pub trait ICKShareParticipant: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn userIdentity(&self) -> CKUserIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userIdentity)
    }
    unsafe fn role(&self) -> CKShareParticipantRole
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, role)
    }
    unsafe fn setRole_(&self, role: CKShareParticipantRole)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRole : role)
    }
    unsafe fn type_(&self) -> CKShareParticipantType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: CKShareParticipantType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn acceptanceStatus(&self) -> CKShareParticipantAcceptanceStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptanceStatus)
    }
    unsafe fn permission(&self) -> CKShareParticipantPermission
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, permission)
    }
    unsafe fn setPermission_(&self, permission: CKShareParticipantPermission)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPermission : permission)
    }
    unsafe fn participantID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantID)
    }
    unsafe fn isApprovedRequester(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isApprovedRequester)
    }
    unsafe fn dateAddedToShare(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateAddedToShare)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareParticipant").unwrap(), new)
    }
    unsafe fn oneTimeURLParticipant() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareParticipant").unwrap(), oneTimeURLParticipant)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKShare(pub id);
impl std::ops::Deref for CKShare {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKShare {}
impl CKShare {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKShare").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKShare {}
impl PNSCopying for CKShare {}
impl ICKRecord for CKShare {}
impl From<CKShare> for CKRecord {
    fn from(child: CKShare) -> CKRecord {
        CKRecord(child.0)
    }
}
impl std::convert::TryFrom<CKRecord> for CKShare {
    type Error = &'static str;
    fn try_from(parent: CKRecord) -> Result<CKShare, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKShare").unwrap()) };
        if is_kind_of {
            Ok(CKShare(parent.0))
        } else {
            Err("This CKRecord cannot be downcasted to CKShare")
        }
    }
}
impl INSObject for CKShare {}
impl PNSObject for CKShare {}
impl ICKShare for CKShare {}
pub trait ICKShare: Sized + std::ops::Deref {
    unsafe fn initWithRootRecord_(&self, rootRecord: CKRecord) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRootRecord : rootRecord)
    }
    unsafe fn initWithRootRecord_shareID_(
        &self,
        rootRecord: CKRecord,
        shareID: CKRecordID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRootRecord : rootRecord, shareID : shareID)
    }
    unsafe fn initWithRecordZoneID_(&self, recordZoneID: CKRecordZoneID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordZoneID : recordZoneID)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn addParticipant_(&self, participant: CKShareParticipant)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addParticipant : participant)
    }
    unsafe fn removeParticipant_(&self, participant: CKShareParticipant)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeParticipant : participant)
    }
    unsafe fn oneTimeURLForParticipantID_(&self, participantID: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, oneTimeURLForParticipantID : participantID)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordType_(&self, recordType: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType)
    }
    unsafe fn initWithRecordType_recordID_(
        &self,
        recordType: NSString,
        recordID: CKRecordID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType, recordID : recordID)
    }
    unsafe fn initWithRecordType_zoneID_(
        &self,
        recordType: NSString,
        zoneID: CKRecordZoneID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordType : recordType, zoneID : zoneID)
    }
    unsafe fn denyRequesters_(&self, requesters: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, denyRequesters : requesters)
    }
    unsafe fn blockRequesters_(&self, requesters: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, blockRequesters : requesters)
    }
    unsafe fn unblockIdentities_(&self, blockedIdentities: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unblockIdentities : blockedIdentities)
    }
    unsafe fn publicPermission(&self) -> CKShareParticipantPermission
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicPermission)
    }
    unsafe fn setPublicPermission_(&self, publicPermission: CKShareParticipantPermission)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPublicPermission : publicPermission)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn participants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participants)
    }
    unsafe fn owner(&self) -> CKShareParticipant
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, owner)
    }
    unsafe fn currentUserParticipant(&self) -> CKShareParticipant
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentUserParticipant)
    }
    unsafe fn requesters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requesters)
    }
    unsafe fn blockedIdentities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blockedIdentities)
    }
    unsafe fn allowsAccessRequests(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAccessRequests)
    }
    unsafe fn setAllowsAccessRequests_(&self, allowsAccessRequests: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAccessRequests : allowsAccessRequests)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKShare").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKShareAccessRequester(pub id);
impl std::ops::Deref for CKShareAccessRequester {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKShareAccessRequester {}
impl CKShareAccessRequester {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareAccessRequester").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKShareAccessRequester {}
impl PNSCopying for CKShareAccessRequester {}
impl INSObject for CKShareAccessRequester {}
impl PNSObject for CKShareAccessRequester {}
impl std::convert::TryFrom<NSObject> for CKShareAccessRequester {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKShareAccessRequester, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKShareAccessRequester").unwrap()) };
        if is_kind_of {
            Ok(CKShareAccessRequester(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKShareAccessRequester")
        }
    }
}
impl ICKShareAccessRequester for CKShareAccessRequester {}
pub trait ICKShareAccessRequester: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn userIdentity(&self) -> CKUserIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userIdentity)
    }
    unsafe fn participantLookupInfo(&self) -> CKUserIdentityLookupInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantLookupInfo)
    }
    unsafe fn contact(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contact)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareAccessRequester").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKShareBlockedIdentity(pub id);
impl std::ops::Deref for CKShareBlockedIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKShareBlockedIdentity {}
impl CKShareBlockedIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareBlockedIdentity").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKShareBlockedIdentity {}
impl PNSCopying for CKShareBlockedIdentity {}
impl INSObject for CKShareBlockedIdentity {}
impl PNSObject for CKShareBlockedIdentity {}
impl std::convert::TryFrom<NSObject> for CKShareBlockedIdentity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKShareBlockedIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKShareBlockedIdentity").unwrap()) };
        if is_kind_of {
            Ok(CKShareBlockedIdentity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKShareBlockedIdentity")
        }
    }
}
impl ICKShareBlockedIdentity for CKShareBlockedIdentity {}
pub trait ICKShareBlockedIdentity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn userIdentity(&self) -> CKUserIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userIdentity)
    }
    unsafe fn contact(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contact)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareBlockedIdentity").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKShareMetadata(pub id);
impl std::ops::Deref for CKShareMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKShareMetadata {}
impl CKShareMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareMetadata").unwrap(), alloc) })
    }
}
impl PNSCopying for CKShareMetadata {}
impl PNSSecureCoding for CKShareMetadata {}
impl INSObject for CKShareMetadata {}
impl PNSObject for CKShareMetadata {}
impl std::convert::TryFrom<NSObject> for CKShareMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKShareMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKShareMetadata").unwrap()) };
        if is_kind_of {
            Ok(CKShareMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKShareMetadata")
        }
    }
}
impl ICKShareMetadata for CKShareMetadata {}
pub trait ICKShareMetadata: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn containerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerIdentifier)
    }
    unsafe fn share(&self) -> CKShare
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, share)
    }
    unsafe fn hierarchicalRootRecordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hierarchicalRootRecordID)
    }
    unsafe fn participantRole(&self) -> CKShareParticipantRole
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantRole)
    }
    unsafe fn participantStatus(&self) -> CKShareParticipantAcceptanceStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantStatus)
    }
    unsafe fn participantPermission(&self) -> CKShareParticipantPermission
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantPermission)
    }
    unsafe fn ownerIdentity(&self) -> CKUserIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerIdentity)
    }
    unsafe fn rootRecord(&self) -> CKRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootRecord)
    }
    unsafe fn participantType(&self) -> CKShareParticipantType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantType)
    }
    unsafe fn rootRecordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootRecordID)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareMetadata").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKUserIdentity(pub id);
impl std::ops::Deref for CKUserIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKUserIdentity {}
impl CKUserIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKUserIdentity").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKUserIdentity {}
impl PNSCopying for CKUserIdentity {}
impl INSObject for CKUserIdentity {}
impl PNSObject for CKUserIdentity {}
impl std::convert::TryFrom<NSObject> for CKUserIdentity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKUserIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKUserIdentity").unwrap()) };
        if is_kind_of {
            Ok(CKUserIdentity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKUserIdentity")
        }
    }
}
impl ICKUserIdentity for CKUserIdentity {}
pub trait ICKUserIdentity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn userRecordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userRecordID)
    }
    unsafe fn lookupInfo(&self) -> CKUserIdentityLookupInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lookupInfo)
    }
    unsafe fn nameComponents(&self) -> NSPersonNameComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nameComponents)
    }
    unsafe fn hasiCloudAccount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasiCloudAccount)
    }
    unsafe fn contactIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactIdentifiers)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKUserIdentity").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKUserIdentityLookupInfo(pub id);
impl std::ops::Deref for CKUserIdentityLookupInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKUserIdentityLookupInfo {}
impl CKUserIdentityLookupInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKUserIdentityLookupInfo").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKUserIdentityLookupInfo {}
impl PNSCopying for CKUserIdentityLookupInfo {}
impl INSObject for CKUserIdentityLookupInfo {}
impl PNSObject for CKUserIdentityLookupInfo {}
impl std::convert::TryFrom<NSObject> for CKUserIdentityLookupInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKUserIdentityLookupInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKUserIdentityLookupInfo").unwrap()) };
        if is_kind_of {
            Ok(CKUserIdentityLookupInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKUserIdentityLookupInfo")
        }
    }
}
impl ICKUserIdentityLookupInfo for CKUserIdentityLookupInfo {}
pub trait ICKUserIdentityLookupInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEmailAddress_(&self, emailAddress: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEmailAddress : emailAddress)
    }
    unsafe fn initWithPhoneNumber_(&self, phoneNumber: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPhoneNumber : phoneNumber)
    }
    unsafe fn initWithUserRecordID_(&self, userRecordID: CKRecordID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUserRecordID : userRecordID)
    }
    unsafe fn emailAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailAddress)
    }
    unsafe fn phoneNumber(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneNumber)
    }
    unsafe fn userRecordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userRecordID)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKUserIdentityLookupInfo").unwrap(), new)
    }
    unsafe fn lookupInfosWithEmails_(emails: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKUserIdentityLookupInfo").unwrap(), lookupInfosWithEmails : emails)
    }
    unsafe fn lookupInfosWithPhoneNumbers_(phoneNumbers: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKUserIdentityLookupInfo").unwrap(), lookupInfosWithPhoneNumbers : phoneNumbers)
    }
    unsafe fn lookupInfosWithRecordIDs_(recordIDs: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKUserIdentityLookupInfo").unwrap(), lookupInfosWithRecordIDs : recordIDs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKAcceptSharesOperation(pub id);
impl std::ops::Deref for CKAcceptSharesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKAcceptSharesOperation {}
impl CKAcceptSharesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKAcceptSharesOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKAcceptSharesOperation {}
impl std::convert::TryFrom<CKOperation> for CKAcceptSharesOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKAcceptSharesOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKAcceptSharesOperation").unwrap()) };
        if is_kind_of {
            Ok(CKAcceptSharesOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKAcceptSharesOperation")
        }
    }
}
impl INSOperation for CKAcceptSharesOperation {}
impl INSObject for CKAcceptSharesOperation {}
impl PNSObject for CKAcceptSharesOperation {}
impl ICKAcceptSharesOperation for CKAcceptSharesOperation {}
pub trait ICKAcceptSharesOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithShareMetadatas_(&self, shareMetadatas: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithShareMetadatas : shareMetadatas)
    }
    unsafe fn shareMetadatas(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shareMetadatas)
    }
    unsafe fn setShareMetadatas_(&self, shareMetadatas: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShareMetadatas : shareMetadatas)
    }
    unsafe fn perShareCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perShareCompletionBlock)
    }
    unsafe fn setPerShareCompletionBlock_(
        &self,
        perShareCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerShareCompletionBlock : perShareCompletionBlock)
    }
    unsafe fn acceptSharesCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptSharesCompletionBlock)
    }
    unsafe fn setAcceptSharesCompletionBlock_(
        &self,
        acceptSharesCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcceptSharesCompletionBlock : acceptSharesCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKDiscoverAllUserIdentitiesOperation(pub id);
impl std::ops::Deref for CKDiscoverAllUserIdentitiesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKDiscoverAllUserIdentitiesOperation {}
impl CKDiscoverAllUserIdentitiesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKDiscoverAllUserIdentitiesOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKDiscoverAllUserIdentitiesOperation {}
impl std::convert::TryFrom<CKOperation> for CKDiscoverAllUserIdentitiesOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKDiscoverAllUserIdentitiesOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKDiscoverAllUserIdentitiesOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKDiscoverAllUserIdentitiesOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKDiscoverAllUserIdentitiesOperation")
        }
    }
}
impl INSOperation for CKDiscoverAllUserIdentitiesOperation {}
impl INSObject for CKDiscoverAllUserIdentitiesOperation {}
impl PNSObject for CKDiscoverAllUserIdentitiesOperation {}
impl ICKDiscoverAllUserIdentitiesOperation for CKDiscoverAllUserIdentitiesOperation {}
pub trait ICKDiscoverAllUserIdentitiesOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn userIdentityDiscoveredBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userIdentityDiscoveredBlock)
    }
    unsafe fn setUserIdentityDiscoveredBlock_(
        &self,
        userIdentityDiscoveredBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserIdentityDiscoveredBlock : userIdentityDiscoveredBlock)
    }
    unsafe fn discoverAllUserIdentitiesCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoverAllUserIdentitiesCompletionBlock)
    }
    unsafe fn setDiscoverAllUserIdentitiesCompletionBlock_(
        &self,
        discoverAllUserIdentitiesCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiscoverAllUserIdentitiesCompletionBlock : discoverAllUserIdentitiesCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKDiscoverUserIdentitiesOperation(pub id);
impl std::ops::Deref for CKDiscoverUserIdentitiesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKDiscoverUserIdentitiesOperation {}
impl CKDiscoverUserIdentitiesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKDiscoverUserIdentitiesOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKDiscoverUserIdentitiesOperation {}
impl std::convert::TryFrom<CKOperation> for CKDiscoverUserIdentitiesOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKDiscoverUserIdentitiesOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKDiscoverUserIdentitiesOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKDiscoverUserIdentitiesOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKDiscoverUserIdentitiesOperation")
        }
    }
}
impl INSOperation for CKDiscoverUserIdentitiesOperation {}
impl INSObject for CKDiscoverUserIdentitiesOperation {}
impl PNSObject for CKDiscoverUserIdentitiesOperation {}
impl ICKDiscoverUserIdentitiesOperation for CKDiscoverUserIdentitiesOperation {}
pub trait ICKDiscoverUserIdentitiesOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithUserIdentityLookupInfos_(
        &self,
        userIdentityLookupInfos: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUserIdentityLookupInfos : userIdentityLookupInfos)
    }
    unsafe fn userIdentityLookupInfos(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userIdentityLookupInfos)
    }
    unsafe fn setUserIdentityLookupInfos_(&self, userIdentityLookupInfos: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserIdentityLookupInfos : userIdentityLookupInfos)
    }
    unsafe fn userIdentityDiscoveredBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userIdentityDiscoveredBlock)
    }
    unsafe fn setUserIdentityDiscoveredBlock_(
        &self,
        userIdentityDiscoveredBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserIdentityDiscoveredBlock : userIdentityDiscoveredBlock)
    }
    unsafe fn discoverUserIdentitiesCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoverUserIdentitiesCompletionBlock)
    }
    unsafe fn setDiscoverUserIdentitiesCompletionBlock_(
        &self,
        discoverUserIdentitiesCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiscoverUserIdentitiesCompletionBlock : discoverUserIdentitiesCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchDatabaseChangesOperation(pub id);
impl std::ops::Deref for CKFetchDatabaseChangesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchDatabaseChangesOperation {}
impl CKFetchDatabaseChangesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchDatabaseChangesOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKFetchDatabaseChangesOperation {}
impl From<CKFetchDatabaseChangesOperation> for CKDatabaseOperation {
    fn from(child: CKFetchDatabaseChangesOperation) -> CKDatabaseOperation {
        CKDatabaseOperation(child.0)
    }
}
impl std::convert::TryFrom<CKDatabaseOperation> for CKFetchDatabaseChangesOperation {
    type Error = &'static str;
    fn try_from(
        parent: CKDatabaseOperation,
    ) -> Result<CKFetchDatabaseChangesOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchDatabaseChangesOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchDatabaseChangesOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKFetchDatabaseChangesOperation")
        }
    }
}
impl ICKOperation for CKFetchDatabaseChangesOperation {}
impl INSOperation for CKFetchDatabaseChangesOperation {}
impl INSObject for CKFetchDatabaseChangesOperation {}
impl PNSObject for CKFetchDatabaseChangesOperation {}
impl ICKFetchDatabaseChangesOperation for CKFetchDatabaseChangesOperation {}
pub trait ICKFetchDatabaseChangesOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPreviousServerChangeToken_(
        &self,
        previousServerChangeToken: CKServerChangeToken,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPreviousServerChangeToken : previousServerChangeToken)
    }
    unsafe fn previousServerChangeToken(&self) -> CKServerChangeToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousServerChangeToken)
    }
    unsafe fn setPreviousServerChangeToken_(&self, previousServerChangeToken: CKServerChangeToken)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviousServerChangeToken : previousServerChangeToken)
    }
    unsafe fn resultsLimit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultsLimit)
    }
    unsafe fn setResultsLimit_(&self, resultsLimit: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultsLimit : resultsLimit)
    }
    unsafe fn fetchAllChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchAllChanges)
    }
    unsafe fn setFetchAllChanges_(&self, fetchAllChanges: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchAllChanges : fetchAllChanges)
    }
    unsafe fn recordZoneWithIDChangedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneWithIDChangedBlock)
    }
    unsafe fn setRecordZoneWithIDChangedBlock_(
        &self,
        recordZoneWithIDChangedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneWithIDChangedBlock : recordZoneWithIDChangedBlock)
    }
    unsafe fn recordZoneWithIDWasDeletedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneWithIDWasDeletedBlock)
    }
    unsafe fn setRecordZoneWithIDWasDeletedBlock_(
        &self,
        recordZoneWithIDWasDeletedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneWithIDWasDeletedBlock : recordZoneWithIDWasDeletedBlock)
    }
    unsafe fn recordZoneWithIDWasPurgedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneWithIDWasPurgedBlock)
    }
    unsafe fn setRecordZoneWithIDWasPurgedBlock_(
        &self,
        recordZoneWithIDWasPurgedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneWithIDWasPurgedBlock : recordZoneWithIDWasPurgedBlock)
    }
    unsafe fn recordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock(
        &self,
    ) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock)
    }
    unsafe fn setRecordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock_(
        &self,
        recordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock : recordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock)
    }
    unsafe fn changeTokenUpdatedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeTokenUpdatedBlock)
    }
    unsafe fn setChangeTokenUpdatedBlock_(
        &self,
        changeTokenUpdatedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChangeTokenUpdatedBlock : changeTokenUpdatedBlock)
    }
    unsafe fn fetchDatabaseChangesCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchDatabaseChangesCompletionBlock)
    }
    unsafe fn setFetchDatabaseChangesCompletionBlock_(
        &self,
        fetchDatabaseChangesCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchDatabaseChangesCompletionBlock : fetchDatabaseChangesCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchNotificationChangesOperation(pub id);
impl std::ops::Deref for CKFetchNotificationChangesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchNotificationChangesOperation {}
impl CKFetchNotificationChangesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchNotificationChangesOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKFetchNotificationChangesOperation {}
impl std::convert::TryFrom<CKOperation> for CKFetchNotificationChangesOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKFetchNotificationChangesOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchNotificationChangesOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchNotificationChangesOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKFetchNotificationChangesOperation")
        }
    }
}
impl INSOperation for CKFetchNotificationChangesOperation {}
impl INSObject for CKFetchNotificationChangesOperation {}
impl PNSObject for CKFetchNotificationChangesOperation {}
impl ICKFetchNotificationChangesOperation for CKFetchNotificationChangesOperation {}
pub trait ICKFetchNotificationChangesOperation: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchRecordChangesOperation(pub id);
impl std::ops::Deref for CKFetchRecordChangesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchRecordChangesOperation {}
impl CKFetchRecordChangesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchRecordChangesOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKFetchRecordChangesOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKFetchRecordChangesOperation {
    type Error = &'static str;
    fn try_from(parent: CKDatabaseOperation) -> Result<CKFetchRecordChangesOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchRecordChangesOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchRecordChangesOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKFetchRecordChangesOperation")
        }
    }
}
impl ICKOperation for CKFetchRecordChangesOperation {}
impl INSOperation for CKFetchRecordChangesOperation {}
impl INSObject for CKFetchRecordChangesOperation {}
impl PNSObject for CKFetchRecordChangesOperation {}
impl ICKFetchRecordChangesOperation for CKFetchRecordChangesOperation {}
pub trait ICKFetchRecordChangesOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordZoneID_previousServerChangeToken_(
        &self,
        recordZoneID: CKRecordZoneID,
        previousServerChangeToken: CKServerChangeToken,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordZoneID : recordZoneID, previousServerChangeToken : previousServerChangeToken)
    }
    unsafe fn recordZoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneID)
    }
    unsafe fn setRecordZoneID_(&self, recordZoneID: CKRecordZoneID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneID : recordZoneID)
    }
    unsafe fn previousServerChangeToken(&self) -> CKServerChangeToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousServerChangeToken)
    }
    unsafe fn setPreviousServerChangeToken_(&self, previousServerChangeToken: CKServerChangeToken)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviousServerChangeToken : previousServerChangeToken)
    }
    unsafe fn resultsLimit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultsLimit)
    }
    unsafe fn setResultsLimit_(&self, resultsLimit: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultsLimit : resultsLimit)
    }
    unsafe fn desiredKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredKeys)
    }
    unsafe fn setDesiredKeys_(&self, desiredKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredKeys : desiredKeys)
    }
    unsafe fn recordChangedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordChangedBlock)
    }
    unsafe fn setRecordChangedBlock_(&self, recordChangedBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordChangedBlock : recordChangedBlock)
    }
    unsafe fn recordWithIDWasDeletedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordWithIDWasDeletedBlock)
    }
    unsafe fn setRecordWithIDWasDeletedBlock_(
        &self,
        recordWithIDWasDeletedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordWithIDWasDeletedBlock : recordWithIDWasDeletedBlock)
    }
    unsafe fn moreComing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, moreComing)
    }
    unsafe fn fetchRecordChangesCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRecordChangesCompletionBlock)
    }
    unsafe fn setFetchRecordChangesCompletionBlock_(
        &self,
        fetchRecordChangesCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchRecordChangesCompletionBlock : fetchRecordChangesCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchRecordsOperation(pub id);
impl std::ops::Deref for CKFetchRecordsOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchRecordsOperation {}
impl CKFetchRecordsOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchRecordsOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKFetchRecordsOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKFetchRecordsOperation {
    type Error = &'static str;
    fn try_from(parent: CKDatabaseOperation) -> Result<CKFetchRecordsOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchRecordsOperation").unwrap()) };
        if is_kind_of {
            Ok(CKFetchRecordsOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKFetchRecordsOperation")
        }
    }
}
impl ICKOperation for CKFetchRecordsOperation {}
impl INSOperation for CKFetchRecordsOperation {}
impl INSObject for CKFetchRecordsOperation {}
impl PNSObject for CKFetchRecordsOperation {}
impl ICKFetchRecordsOperation for CKFetchRecordsOperation {}
pub trait ICKFetchRecordsOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordIDs_(&self, recordIDs: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordIDs : recordIDs)
    }
    unsafe fn recordIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordIDs)
    }
    unsafe fn setRecordIDs_(&self, recordIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordIDs : recordIDs)
    }
    unsafe fn desiredKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredKeys)
    }
    unsafe fn setDesiredKeys_(&self, desiredKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredKeys : desiredKeys)
    }
    unsafe fn perRecordProgressBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordProgressBlock)
    }
    unsafe fn setPerRecordProgressBlock_(&self, perRecordProgressBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordProgressBlock : perRecordProgressBlock)
    }
    unsafe fn perRecordCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordCompletionBlock)
    }
    unsafe fn setPerRecordCompletionBlock_(
        &self,
        perRecordCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordCompletionBlock : perRecordCompletionBlock)
    }
    unsafe fn fetchRecordsCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRecordsCompletionBlock)
    }
    unsafe fn setFetchRecordsCompletionBlock_(
        &self,
        fetchRecordsCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchRecordsCompletionBlock : fetchRecordsCompletionBlock)
    }
    unsafe fn fetchCurrentUserRecordOperation() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchRecordsOperation").unwrap(), fetchCurrentUserRecordOperation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchRecordZoneChangesOperation(pub id);
impl std::ops::Deref for CKFetchRecordZoneChangesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchRecordZoneChangesOperation {}
impl CKFetchRecordZoneChangesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchRecordZoneChangesOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKFetchRecordZoneChangesOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKFetchRecordZoneChangesOperation {
    type Error = &'static str;
    fn try_from(
        parent: CKDatabaseOperation,
    ) -> Result<CKFetchRecordZoneChangesOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchRecordZoneChangesOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchRecordZoneChangesOperation(parent.0))
        } else {
            Err ("This CKDatabaseOperation cannot be downcasted to CKFetchRecordZoneChangesOperation" ,)
        }
    }
}
impl ICKOperation for CKFetchRecordZoneChangesOperation {}
impl INSOperation for CKFetchRecordZoneChangesOperation {}
impl INSObject for CKFetchRecordZoneChangesOperation {}
impl PNSObject for CKFetchRecordZoneChangesOperation {}
impl ICKFetchRecordZoneChangesOperation for CKFetchRecordZoneChangesOperation {}
pub trait ICKFetchRecordZoneChangesOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordZoneIDs_configurationsByRecordZoneID_(
        &self,
        recordZoneIDs: NSArray,
        configurationsByRecordZoneID: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordZoneIDs : recordZoneIDs, configurationsByRecordZoneID : configurationsByRecordZoneID)
    }
    unsafe fn recordZoneIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneIDs)
    }
    unsafe fn setRecordZoneIDs_(&self, recordZoneIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneIDs : recordZoneIDs)
    }
    unsafe fn configurationsByRecordZoneID(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationsByRecordZoneID)
    }
    unsafe fn setConfigurationsByRecordZoneID_(&self, configurationsByRecordZoneID: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfigurationsByRecordZoneID : configurationsByRecordZoneID)
    }
    unsafe fn fetchAllChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchAllChanges)
    }
    unsafe fn setFetchAllChanges_(&self, fetchAllChanges: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchAllChanges : fetchAllChanges)
    }
    unsafe fn recordChangedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordChangedBlock)
    }
    unsafe fn setRecordChangedBlock_(&self, recordChangedBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordChangedBlock : recordChangedBlock)
    }
    unsafe fn recordWasChangedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordWasChangedBlock)
    }
    unsafe fn setRecordWasChangedBlock_(&self, recordWasChangedBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordWasChangedBlock : recordWasChangedBlock)
    }
    unsafe fn recordWithIDWasDeletedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordWithIDWasDeletedBlock)
    }
    unsafe fn setRecordWithIDWasDeletedBlock_(
        &self,
        recordWithIDWasDeletedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordWithIDWasDeletedBlock : recordWithIDWasDeletedBlock)
    }
    unsafe fn recordZoneChangeTokensUpdatedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneChangeTokensUpdatedBlock)
    }
    unsafe fn setRecordZoneChangeTokensUpdatedBlock_(
        &self,
        recordZoneChangeTokensUpdatedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneChangeTokensUpdatedBlock : recordZoneChangeTokensUpdatedBlock)
    }
    unsafe fn recordZoneFetchCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneFetchCompletionBlock)
    }
    unsafe fn setRecordZoneFetchCompletionBlock_(
        &self,
        recordZoneFetchCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneFetchCompletionBlock : recordZoneFetchCompletionBlock)
    }
    unsafe fn fetchRecordZoneChangesCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRecordZoneChangesCompletionBlock)
    }
    unsafe fn setFetchRecordZoneChangesCompletionBlock_(
        &self,
        fetchRecordZoneChangesCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchRecordZoneChangesCompletionBlock : fetchRecordZoneChangesCompletionBlock)
    }
}
impl CKFetchRecordZoneChangesOperation_Deprecated for CKFetchRecordZoneChangesOperation {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchRecordZoneChangesConfiguration(pub id);
impl std::ops::Deref for CKFetchRecordZoneChangesConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchRecordZoneChangesConfiguration {}
impl CKFetchRecordZoneChangesConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchRecordZoneChangesConfiguration").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKFetchRecordZoneChangesConfiguration {}
impl PNSCopying for CKFetchRecordZoneChangesConfiguration {}
impl INSObject for CKFetchRecordZoneChangesConfiguration {}
impl PNSObject for CKFetchRecordZoneChangesConfiguration {}
impl std::convert::TryFrom<NSObject> for CKFetchRecordZoneChangesConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKFetchRecordZoneChangesConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchRecordZoneChangesConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchRecordZoneChangesConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKFetchRecordZoneChangesConfiguration")
        }
    }
}
impl ICKFetchRecordZoneChangesConfiguration for CKFetchRecordZoneChangesConfiguration {}
pub trait ICKFetchRecordZoneChangesConfiguration: Sized + std::ops::Deref {
    unsafe fn previousServerChangeToken(&self) -> CKServerChangeToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousServerChangeToken)
    }
    unsafe fn setPreviousServerChangeToken_(&self, previousServerChangeToken: CKServerChangeToken)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviousServerChangeToken : previousServerChangeToken)
    }
    unsafe fn resultsLimit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultsLimit)
    }
    unsafe fn setResultsLimit_(&self, resultsLimit: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultsLimit : resultsLimit)
    }
    unsafe fn desiredKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredKeys)
    }
    unsafe fn setDesiredKeys_(&self, desiredKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredKeys : desiredKeys)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchRecordZoneChangesOptions(pub id);
impl std::ops::Deref for CKFetchRecordZoneChangesOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchRecordZoneChangesOptions {}
impl CKFetchRecordZoneChangesOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchRecordZoneChangesOptions").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKFetchRecordZoneChangesOptions {}
impl PNSCopying for CKFetchRecordZoneChangesOptions {}
impl INSObject for CKFetchRecordZoneChangesOptions {}
impl PNSObject for CKFetchRecordZoneChangesOptions {}
impl std::convert::TryFrom<NSObject> for CKFetchRecordZoneChangesOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKFetchRecordZoneChangesOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchRecordZoneChangesOptions").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchRecordZoneChangesOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKFetchRecordZoneChangesOptions")
        }
    }
}
impl ICKFetchRecordZoneChangesOptions for CKFetchRecordZoneChangesOptions {}
pub trait ICKFetchRecordZoneChangesOptions: Sized + std::ops::Deref {
    unsafe fn previousServerChangeToken(&self) -> CKServerChangeToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousServerChangeToken)
    }
    unsafe fn setPreviousServerChangeToken_(&self, previousServerChangeToken: CKServerChangeToken)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviousServerChangeToken : previousServerChangeToken)
    }
    unsafe fn resultsLimit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultsLimit)
    }
    unsafe fn setResultsLimit_(&self, resultsLimit: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultsLimit : resultsLimit)
    }
    unsafe fn desiredKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredKeys)
    }
    unsafe fn setDesiredKeys_(&self, desiredKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredKeys : desiredKeys)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchRecordZonesOperation(pub id);
impl std::ops::Deref for CKFetchRecordZonesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchRecordZonesOperation {}
impl CKFetchRecordZonesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchRecordZonesOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKFetchRecordZonesOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKFetchRecordZonesOperation {
    type Error = &'static str;
    fn try_from(parent: CKDatabaseOperation) -> Result<CKFetchRecordZonesOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchRecordZonesOperation").unwrap()) };
        if is_kind_of {
            Ok(CKFetchRecordZonesOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKFetchRecordZonesOperation")
        }
    }
}
impl ICKOperation for CKFetchRecordZonesOperation {}
impl INSOperation for CKFetchRecordZonesOperation {}
impl INSObject for CKFetchRecordZonesOperation {}
impl PNSObject for CKFetchRecordZonesOperation {}
impl ICKFetchRecordZonesOperation for CKFetchRecordZonesOperation {}
pub trait ICKFetchRecordZonesOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordZoneIDs_(&self, zoneIDs: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordZoneIDs : zoneIDs)
    }
    unsafe fn recordZoneIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneIDs)
    }
    unsafe fn setRecordZoneIDs_(&self, recordZoneIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneIDs : recordZoneIDs)
    }
    unsafe fn perRecordZoneCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordZoneCompletionBlock)
    }
    unsafe fn setPerRecordZoneCompletionBlock_(
        &self,
        perRecordZoneCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordZoneCompletionBlock : perRecordZoneCompletionBlock)
    }
    unsafe fn fetchRecordZonesCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchRecordZonesCompletionBlock)
    }
    unsafe fn setFetchRecordZonesCompletionBlock_(
        &self,
        fetchRecordZonesCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchRecordZonesCompletionBlock : fetchRecordZonesCompletionBlock)
    }
    unsafe fn fetchAllRecordZonesOperation() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchRecordZonesOperation").unwrap(), fetchAllRecordZonesOperation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchShareMetadataOperation(pub id);
impl std::ops::Deref for CKFetchShareMetadataOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchShareMetadataOperation {}
impl CKFetchShareMetadataOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchShareMetadataOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKFetchShareMetadataOperation {}
impl std::convert::TryFrom<CKOperation> for CKFetchShareMetadataOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKFetchShareMetadataOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchShareMetadataOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchShareMetadataOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKFetchShareMetadataOperation")
        }
    }
}
impl INSOperation for CKFetchShareMetadataOperation {}
impl INSObject for CKFetchShareMetadataOperation {}
impl PNSObject for CKFetchShareMetadataOperation {}
impl ICKFetchShareMetadataOperation for CKFetchShareMetadataOperation {}
pub trait ICKFetchShareMetadataOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithShareURLs_(&self, shareURLs: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithShareURLs : shareURLs)
    }
    unsafe fn shareURLs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shareURLs)
    }
    unsafe fn setShareURLs_(&self, shareURLs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShareURLs : shareURLs)
    }
    unsafe fn shouldFetchRootRecord(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldFetchRootRecord)
    }
    unsafe fn setShouldFetchRootRecord_(&self, shouldFetchRootRecord: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldFetchRootRecord : shouldFetchRootRecord)
    }
    unsafe fn rootRecordDesiredKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootRecordDesiredKeys)
    }
    unsafe fn setRootRecordDesiredKeys_(&self, rootRecordDesiredKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRootRecordDesiredKeys : rootRecordDesiredKeys)
    }
    unsafe fn perShareMetadataBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perShareMetadataBlock)
    }
    unsafe fn setPerShareMetadataBlock_(&self, perShareMetadataBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerShareMetadataBlock : perShareMetadataBlock)
    }
    unsafe fn fetchShareMetadataCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchShareMetadataCompletionBlock)
    }
    unsafe fn setFetchShareMetadataCompletionBlock_(
        &self,
        fetchShareMetadataCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchShareMetadataCompletionBlock : fetchShareMetadataCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchShareParticipantsOperation(pub id);
impl std::ops::Deref for CKFetchShareParticipantsOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchShareParticipantsOperation {}
impl CKFetchShareParticipantsOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchShareParticipantsOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKFetchShareParticipantsOperation {}
impl std::convert::TryFrom<CKOperation> for CKFetchShareParticipantsOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKFetchShareParticipantsOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchShareParticipantsOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchShareParticipantsOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKFetchShareParticipantsOperation")
        }
    }
}
impl INSOperation for CKFetchShareParticipantsOperation {}
impl INSObject for CKFetchShareParticipantsOperation {}
impl PNSObject for CKFetchShareParticipantsOperation {}
impl ICKFetchShareParticipantsOperation for CKFetchShareParticipantsOperation {}
pub trait ICKFetchShareParticipantsOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithUserIdentityLookupInfos_(
        &self,
        userIdentityLookupInfos: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUserIdentityLookupInfos : userIdentityLookupInfos)
    }
    unsafe fn userIdentityLookupInfos(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userIdentityLookupInfos)
    }
    unsafe fn setUserIdentityLookupInfos_(&self, userIdentityLookupInfos: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserIdentityLookupInfos : userIdentityLookupInfos)
    }
    unsafe fn shareParticipantFetchedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shareParticipantFetchedBlock)
    }
    unsafe fn setShareParticipantFetchedBlock_(
        &self,
        shareParticipantFetchedBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShareParticipantFetchedBlock : shareParticipantFetchedBlock)
    }
    unsafe fn perShareParticipantCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perShareParticipantCompletionBlock)
    }
    unsafe fn setPerShareParticipantCompletionBlock_(
        &self,
        perShareParticipantCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerShareParticipantCompletionBlock : perShareParticipantCompletionBlock)
    }
    unsafe fn fetchShareParticipantsCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchShareParticipantsCompletionBlock)
    }
    unsafe fn setFetchShareParticipantsCompletionBlock_(
        &self,
        fetchShareParticipantsCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchShareParticipantsCompletionBlock : fetchShareParticipantsCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchSubscriptionsOperation(pub id);
impl std::ops::Deref for CKFetchSubscriptionsOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchSubscriptionsOperation {}
impl CKFetchSubscriptionsOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchSubscriptionsOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKFetchSubscriptionsOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKFetchSubscriptionsOperation {
    type Error = &'static str;
    fn try_from(parent: CKDatabaseOperation) -> Result<CKFetchSubscriptionsOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchSubscriptionsOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKFetchSubscriptionsOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKFetchSubscriptionsOperation")
        }
    }
}
impl ICKOperation for CKFetchSubscriptionsOperation {}
impl INSOperation for CKFetchSubscriptionsOperation {}
impl INSObject for CKFetchSubscriptionsOperation {}
impl PNSObject for CKFetchSubscriptionsOperation {}
impl ICKFetchSubscriptionsOperation for CKFetchSubscriptionsOperation {}
pub trait ICKFetchSubscriptionsOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSubscriptionIDs_(&self, subscriptionIDs: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSubscriptionIDs : subscriptionIDs)
    }
    unsafe fn subscriptionIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionIDs)
    }
    unsafe fn setSubscriptionIDs_(&self, subscriptionIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubscriptionIDs : subscriptionIDs)
    }
    unsafe fn perSubscriptionCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perSubscriptionCompletionBlock)
    }
    unsafe fn setPerSubscriptionCompletionBlock_(
        &self,
        perSubscriptionCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerSubscriptionCompletionBlock : perSubscriptionCompletionBlock)
    }
    unsafe fn fetchSubscriptionCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchSubscriptionCompletionBlock)
    }
    unsafe fn setFetchSubscriptionCompletionBlock_(
        &self,
        fetchSubscriptionCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchSubscriptionCompletionBlock : fetchSubscriptionCompletionBlock)
    }
    unsafe fn fetchAllSubscriptionsOperation() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchSubscriptionsOperation").unwrap(), fetchAllSubscriptionsOperation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKFetchWebAuthTokenOperation(pub id);
impl std::ops::Deref for CKFetchWebAuthTokenOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKFetchWebAuthTokenOperation {}
impl CKFetchWebAuthTokenOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKFetchWebAuthTokenOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKFetchWebAuthTokenOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKFetchWebAuthTokenOperation {
    type Error = &'static str;
    fn try_from(parent: CKDatabaseOperation) -> Result<CKFetchWebAuthTokenOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKFetchWebAuthTokenOperation").unwrap()) };
        if is_kind_of {
            Ok(CKFetchWebAuthTokenOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKFetchWebAuthTokenOperation")
        }
    }
}
impl ICKOperation for CKFetchWebAuthTokenOperation {}
impl INSOperation for CKFetchWebAuthTokenOperation {}
impl INSObject for CKFetchWebAuthTokenOperation {}
impl PNSObject for CKFetchWebAuthTokenOperation {}
impl ICKFetchWebAuthTokenOperation for CKFetchWebAuthTokenOperation {}
pub trait ICKFetchWebAuthTokenOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAPIToken_(&self, APIToken: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAPIToken : APIToken)
    }
    unsafe fn APIToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, APIToken)
    }
    unsafe fn setAPIToken_(&self, APIToken: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAPIToken : APIToken)
    }
    unsafe fn fetchWebAuthTokenCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchWebAuthTokenCompletionBlock)
    }
    unsafe fn setFetchWebAuthTokenCompletionBlock_(
        &self,
        fetchWebAuthTokenCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchWebAuthTokenCompletionBlock : fetchWebAuthTokenCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKMarkNotificationsReadOperation(pub id);
impl std::ops::Deref for CKMarkNotificationsReadOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKMarkNotificationsReadOperation {}
impl CKMarkNotificationsReadOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKMarkNotificationsReadOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKMarkNotificationsReadOperation {}
impl std::convert::TryFrom<CKOperation> for CKMarkNotificationsReadOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKMarkNotificationsReadOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKMarkNotificationsReadOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKMarkNotificationsReadOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKMarkNotificationsReadOperation")
        }
    }
}
impl INSOperation for CKMarkNotificationsReadOperation {}
impl INSObject for CKMarkNotificationsReadOperation {}
impl PNSObject for CKMarkNotificationsReadOperation {}
impl ICKMarkNotificationsReadOperation for CKMarkNotificationsReadOperation {}
pub trait ICKMarkNotificationsReadOperation: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKModifyBadgeOperation(pub id);
impl std::ops::Deref for CKModifyBadgeOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKModifyBadgeOperation {}
impl CKModifyBadgeOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKModifyBadgeOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKModifyBadgeOperation {}
impl std::convert::TryFrom<CKOperation> for CKModifyBadgeOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKModifyBadgeOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKModifyBadgeOperation").unwrap()) };
        if is_kind_of {
            Ok(CKModifyBadgeOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKModifyBadgeOperation")
        }
    }
}
impl INSOperation for CKModifyBadgeOperation {}
impl INSObject for CKModifyBadgeOperation {}
impl PNSObject for CKModifyBadgeOperation {}
impl ICKModifyBadgeOperation for CKModifyBadgeOperation {}
pub trait ICKModifyBadgeOperation: Sized + std::ops::Deref {}
pub type CKRecordSavePolicy = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKModifyRecordsOperation(pub id);
impl std::ops::Deref for CKModifyRecordsOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKModifyRecordsOperation {}
impl CKModifyRecordsOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKModifyRecordsOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKModifyRecordsOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKModifyRecordsOperation {
    type Error = &'static str;
    fn try_from(parent: CKDatabaseOperation) -> Result<CKModifyRecordsOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKModifyRecordsOperation").unwrap()) };
        if is_kind_of {
            Ok(CKModifyRecordsOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKModifyRecordsOperation")
        }
    }
}
impl ICKOperation for CKModifyRecordsOperation {}
impl INSOperation for CKModifyRecordsOperation {}
impl INSObject for CKModifyRecordsOperation {}
impl PNSObject for CKModifyRecordsOperation {}
impl ICKModifyRecordsOperation for CKModifyRecordsOperation {}
pub trait ICKModifyRecordsOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordsToSave_recordIDsToDelete_(
        &self,
        records: NSArray,
        recordIDs: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordsToSave : records, recordIDsToDelete : recordIDs)
    }
    unsafe fn recordsToSave(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordsToSave)
    }
    unsafe fn setRecordsToSave_(&self, recordsToSave: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordsToSave : recordsToSave)
    }
    unsafe fn recordIDsToDelete(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordIDsToDelete)
    }
    unsafe fn setRecordIDsToDelete_(&self, recordIDsToDelete: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordIDsToDelete : recordIDsToDelete)
    }
    unsafe fn savePolicy(&self) -> CKRecordSavePolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, savePolicy)
    }
    unsafe fn setSavePolicy_(&self, savePolicy: CKRecordSavePolicy)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSavePolicy : savePolicy)
    }
    unsafe fn clientChangeTokenData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientChangeTokenData)
    }
    unsafe fn setClientChangeTokenData_(&self, clientChangeTokenData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClientChangeTokenData : clientChangeTokenData)
    }
    unsafe fn atomic(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, atomic)
    }
    unsafe fn setAtomic_(&self, atomic: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAtomic : atomic)
    }
    unsafe fn perRecordProgressBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordProgressBlock)
    }
    unsafe fn setPerRecordProgressBlock_(&self, perRecordProgressBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordProgressBlock : perRecordProgressBlock)
    }
    unsafe fn perRecordCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordCompletionBlock)
    }
    unsafe fn setPerRecordCompletionBlock_(
        &self,
        perRecordCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordCompletionBlock : perRecordCompletionBlock)
    }
    unsafe fn perRecordSaveBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordSaveBlock)
    }
    unsafe fn setPerRecordSaveBlock_(&self, perRecordSaveBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordSaveBlock : perRecordSaveBlock)
    }
    unsafe fn perRecordDeleteBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordDeleteBlock)
    }
    unsafe fn setPerRecordDeleteBlock_(&self, perRecordDeleteBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordDeleteBlock : perRecordDeleteBlock)
    }
    unsafe fn modifyRecordsCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifyRecordsCompletionBlock)
    }
    unsafe fn setModifyRecordsCompletionBlock_(
        &self,
        modifyRecordsCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModifyRecordsCompletionBlock : modifyRecordsCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKModifyRecordZonesOperation(pub id);
impl std::ops::Deref for CKModifyRecordZonesOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKModifyRecordZonesOperation {}
impl CKModifyRecordZonesOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKModifyRecordZonesOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKModifyRecordZonesOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKModifyRecordZonesOperation {
    type Error = &'static str;
    fn try_from(parent: CKDatabaseOperation) -> Result<CKModifyRecordZonesOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKModifyRecordZonesOperation").unwrap()) };
        if is_kind_of {
            Ok(CKModifyRecordZonesOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKModifyRecordZonesOperation")
        }
    }
}
impl ICKOperation for CKModifyRecordZonesOperation {}
impl INSOperation for CKModifyRecordZonesOperation {}
impl INSObject for CKModifyRecordZonesOperation {}
impl PNSObject for CKModifyRecordZonesOperation {}
impl ICKModifyRecordZonesOperation for CKModifyRecordZonesOperation {}
pub trait ICKModifyRecordZonesOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRecordZonesToSave_recordZoneIDsToDelete_(
        &self,
        recordZonesToSave: NSArray,
        recordZoneIDsToDelete: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordZonesToSave : recordZonesToSave, recordZoneIDsToDelete : recordZoneIDsToDelete)
    }
    unsafe fn recordZonesToSave(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZonesToSave)
    }
    unsafe fn setRecordZonesToSave_(&self, recordZonesToSave: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZonesToSave : recordZonesToSave)
    }
    unsafe fn recordZoneIDsToDelete(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZoneIDsToDelete)
    }
    unsafe fn setRecordZoneIDsToDelete_(&self, recordZoneIDsToDelete: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordZoneIDsToDelete : recordZoneIDsToDelete)
    }
    unsafe fn perRecordZoneSaveBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordZoneSaveBlock)
    }
    unsafe fn setPerRecordZoneSaveBlock_(&self, perRecordZoneSaveBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordZoneSaveBlock : perRecordZoneSaveBlock)
    }
    unsafe fn perRecordZoneDeleteBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perRecordZoneDeleteBlock)
    }
    unsafe fn setPerRecordZoneDeleteBlock_(
        &self,
        perRecordZoneDeleteBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerRecordZoneDeleteBlock : perRecordZoneDeleteBlock)
    }
    unsafe fn modifyRecordZonesCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifyRecordZonesCompletionBlock)
    }
    unsafe fn setModifyRecordZonesCompletionBlock_(
        &self,
        modifyRecordZonesCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModifyRecordZonesCompletionBlock : modifyRecordZonesCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKModifySubscriptionsOperation(pub id);
impl std::ops::Deref for CKModifySubscriptionsOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKModifySubscriptionsOperation {}
impl CKModifySubscriptionsOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKModifySubscriptionsOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKModifySubscriptionsOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKModifySubscriptionsOperation {
    type Error = &'static str;
    fn try_from(
        parent: CKDatabaseOperation,
    ) -> Result<CKModifySubscriptionsOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKModifySubscriptionsOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKModifySubscriptionsOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKModifySubscriptionsOperation")
        }
    }
}
impl ICKOperation for CKModifySubscriptionsOperation {}
impl INSOperation for CKModifySubscriptionsOperation {}
impl INSObject for CKModifySubscriptionsOperation {}
impl PNSObject for CKModifySubscriptionsOperation {}
impl ICKModifySubscriptionsOperation for CKModifySubscriptionsOperation {}
pub trait ICKModifySubscriptionsOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSubscriptionsToSave_subscriptionIDsToDelete_(
        &self,
        subscriptionsToSave: NSArray,
        subscriptionIDsToDelete: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSubscriptionsToSave : subscriptionsToSave, subscriptionIDsToDelete : subscriptionIDsToDelete)
    }
    unsafe fn subscriptionsToSave(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionsToSave)
    }
    unsafe fn setSubscriptionsToSave_(&self, subscriptionsToSave: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubscriptionsToSave : subscriptionsToSave)
    }
    unsafe fn subscriptionIDsToDelete(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionIDsToDelete)
    }
    unsafe fn setSubscriptionIDsToDelete_(&self, subscriptionIDsToDelete: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubscriptionIDsToDelete : subscriptionIDsToDelete)
    }
    unsafe fn perSubscriptionSaveBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perSubscriptionSaveBlock)
    }
    unsafe fn setPerSubscriptionSaveBlock_(
        &self,
        perSubscriptionSaveBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerSubscriptionSaveBlock : perSubscriptionSaveBlock)
    }
    unsafe fn perSubscriptionDeleteBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perSubscriptionDeleteBlock)
    }
    unsafe fn setPerSubscriptionDeleteBlock_(
        &self,
        perSubscriptionDeleteBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerSubscriptionDeleteBlock : perSubscriptionDeleteBlock)
    }
    unsafe fn modifySubscriptionsCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifySubscriptionsCompletionBlock)
    }
    unsafe fn setModifySubscriptionsCompletionBlock_(
        &self,
        modifySubscriptionsCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModifySubscriptionsCompletionBlock : modifySubscriptionsCompletionBlock)
    }
}
pub type CKOperationGroupTransferSize = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKOperationGroup(pub id);
impl std::ops::Deref for CKOperationGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKOperationGroup {}
impl CKOperationGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKOperationGroup").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKOperationGroup {}
impl PNSCopying for CKOperationGroup {}
impl INSObject for CKOperationGroup {}
impl PNSObject for CKOperationGroup {}
impl std::convert::TryFrom<NSObject> for CKOperationGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKOperationGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKOperationGroup").unwrap()) };
        if is_kind_of {
            Ok(CKOperationGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKOperationGroup")
        }
    }
}
impl ICKOperationGroup for CKOperationGroup {}
pub trait ICKOperationGroup: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn operationGroupID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operationGroupID)
    }
    unsafe fn defaultConfiguration(&self) -> CKOperationConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultConfiguration)
    }
    unsafe fn setDefaultConfiguration_(&self, defaultConfiguration: CKOperationConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultConfiguration : defaultConfiguration)
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
    unsafe fn quantity(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quantity)
    }
    unsafe fn setQuantity_(&self, quantity: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuantity : quantity)
    }
    unsafe fn expectedSendSize(&self) -> CKOperationGroupTransferSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedSendSize)
    }
    unsafe fn setExpectedSendSize_(&self, expectedSendSize: CKOperationGroupTransferSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExpectedSendSize : expectedSendSize)
    }
    unsafe fn expectedReceiveSize(&self) -> CKOperationGroupTransferSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedReceiveSize)
    }
    unsafe fn setExpectedReceiveSize_(&self, expectedReceiveSize: CKOperationGroupTransferSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExpectedReceiveSize : expectedReceiveSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKQueryCursor(pub id);
impl std::ops::Deref for CKQueryCursor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKQueryCursor {}
impl CKQueryCursor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKQueryCursor").unwrap(), alloc) })
    }
}
impl PNSCopying for CKQueryCursor {}
impl PNSSecureCoding for CKQueryCursor {}
impl INSObject for CKQueryCursor {}
impl PNSObject for CKQueryCursor {}
impl std::convert::TryFrom<NSObject> for CKQueryCursor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKQueryCursor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKQueryCursor").unwrap()) };
        if is_kind_of {
            Ok(CKQueryCursor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKQueryCursor")
        }
    }
}
impl ICKQueryCursor for CKQueryCursor {}
pub trait ICKQueryCursor: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKQueryCursor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKQueryOperation(pub id);
impl std::ops::Deref for CKQueryOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKQueryOperation {}
impl CKQueryOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKQueryOperation").unwrap(), alloc) })
    }
}
impl ICKDatabaseOperation for CKQueryOperation {}
impl std::convert::TryFrom<CKDatabaseOperation> for CKQueryOperation {
    type Error = &'static str;
    fn try_from(parent: CKDatabaseOperation) -> Result<CKQueryOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKQueryOperation").unwrap()) };
        if is_kind_of {
            Ok(CKQueryOperation(parent.0))
        } else {
            Err("This CKDatabaseOperation cannot be downcasted to CKQueryOperation")
        }
    }
}
impl ICKOperation for CKQueryOperation {}
impl INSOperation for CKQueryOperation {}
impl INSObject for CKQueryOperation {}
impl PNSObject for CKQueryOperation {}
impl ICKQueryOperation for CKQueryOperation {}
pub trait ICKQueryOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithQuery_(&self, query: CKQuery) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQuery : query)
    }
    unsafe fn initWithCursor_(&self, cursor: CKQueryCursor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCursor : cursor)
    }
    unsafe fn query(&self) -> CKQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, query)
    }
    unsafe fn setQuery_(&self, query: CKQuery)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuery : query)
    }
    unsafe fn cursor(&self) -> CKQueryCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursor)
    }
    unsafe fn setCursor_(&self, cursor: CKQueryCursor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCursor : cursor)
    }
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
    unsafe fn setZoneID_(&self, zoneID: CKRecordZoneID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoneID : zoneID)
    }
    unsafe fn resultsLimit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultsLimit)
    }
    unsafe fn setResultsLimit_(&self, resultsLimit: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultsLimit : resultsLimit)
    }
    unsafe fn desiredKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredKeys)
    }
    unsafe fn setDesiredKeys_(&self, desiredKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredKeys : desiredKeys)
    }
    unsafe fn recordFetchedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordFetchedBlock)
    }
    unsafe fn setRecordFetchedBlock_(&self, recordFetchedBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordFetchedBlock : recordFetchedBlock)
    }
    unsafe fn recordMatchedBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordMatchedBlock)
    }
    unsafe fn setRecordMatchedBlock_(&self, recordMatchedBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordMatchedBlock : recordMatchedBlock)
    }
    unsafe fn queryCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queryCompletionBlock)
    }
    unsafe fn setQueryCompletionBlock_(&self, queryCompletionBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueryCompletionBlock : queryCompletionBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKShareRequestAccessOperation(pub id);
impl std::ops::Deref for CKShareRequestAccessOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKShareRequestAccessOperation {}
impl CKShareRequestAccessOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKShareRequestAccessOperation").unwrap(), alloc) })
    }
}
impl ICKOperation for CKShareRequestAccessOperation {}
impl std::convert::TryFrom<CKOperation> for CKShareRequestAccessOperation {
    type Error = &'static str;
    fn try_from(parent: CKOperation) -> Result<CKShareRequestAccessOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKShareRequestAccessOperation").unwrap())
        };
        if is_kind_of {
            Ok(CKShareRequestAccessOperation(parent.0))
        } else {
            Err("This CKOperation cannot be downcasted to CKShareRequestAccessOperation")
        }
    }
}
impl INSOperation for CKShareRequestAccessOperation {}
impl INSObject for CKShareRequestAccessOperation {}
impl PNSObject for CKShareRequestAccessOperation {}
impl ICKShareRequestAccessOperation for CKShareRequestAccessOperation {}
pub trait ICKShareRequestAccessOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithShareURLs_(&self, shareURLs: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithShareURLs : shareURLs)
    }
    unsafe fn shareURLs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shareURLs)
    }
    unsafe fn setShareURLs_(&self, shareURLs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShareURLs : shareURLs)
    }
    unsafe fn perShareAccessRequestCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perShareAccessRequestCompletionBlock)
    }
    unsafe fn setPerShareAccessRequestCompletionBlock_(
        &self,
        perShareAccessRequestCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerShareAccessRequestCompletionBlock : perShareAccessRequestCompletionBlock)
    }
    unsafe fn shareRequestAccessCompletionBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shareRequestAccessCompletionBlock)
    }
    unsafe fn setShareRequestAccessCompletionBlock_(
        &self,
        shareRequestAccessCompletionBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShareRequestAccessCompletionBlock : shareRequestAccessCompletionBlock)
    }
}
pub type CKSharingParticipantAccessOption = NSUInteger;
pub type CKSharingParticipantPermissionOption = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKAllowedSharingOptions(pub id);
impl std::ops::Deref for CKAllowedSharingOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKAllowedSharingOptions {}
impl CKAllowedSharingOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKAllowedSharingOptions").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKAllowedSharingOptions {}
impl PNSCopying for CKAllowedSharingOptions {}
impl INSObject for CKAllowedSharingOptions {}
impl PNSObject for CKAllowedSharingOptions {}
impl std::convert::TryFrom<NSObject> for CKAllowedSharingOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKAllowedSharingOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKAllowedSharingOptions").unwrap()) };
        if is_kind_of {
            Ok(CKAllowedSharingOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKAllowedSharingOptions")
        }
    }
}
impl ICKAllowedSharingOptions for CKAllowedSharingOptions {}
pub trait ICKAllowedSharingOptions: Sized + std::ops::Deref {
    unsafe fn initWithAllowedParticipantPermissionOptions_allowedParticipantAccessOptions_(
        &self,
        allowedParticipantPermissionOptions: CKSharingParticipantPermissionOption,
        allowedParticipantAccessOptions: CKSharingParticipantAccessOption,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAllowedParticipantPermissionOptions : allowedParticipantPermissionOptions, allowedParticipantAccessOptions : allowedParticipantAccessOptions)
    }
    unsafe fn allowedParticipantPermissionOptions(&self) -> CKSharingParticipantPermissionOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedParticipantPermissionOptions)
    }
    unsafe fn setAllowedParticipantPermissionOptions_(
        &self,
        allowedParticipantPermissionOptions: CKSharingParticipantPermissionOption,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedParticipantPermissionOptions : allowedParticipantPermissionOptions)
    }
    unsafe fn allowedParticipantAccessOptions(&self) -> CKSharingParticipantAccessOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedParticipantAccessOptions)
    }
    unsafe fn setAllowedParticipantAccessOptions_(
        &self,
        allowedParticipantAccessOptions: CKSharingParticipantAccessOption,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedParticipantAccessOptions : allowedParticipantAccessOptions)
    }
    unsafe fn allowsParticipantsToInviteOthers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsParticipantsToInviteOthers)
    }
    unsafe fn setAllowsParticipantsToInviteOthers_(&self, allowsParticipantsToInviteOthers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsParticipantsToInviteOthers : allowsParticipantsToInviteOthers)
    }
    unsafe fn allowsAccessRequests(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAccessRequests)
    }
    unsafe fn setAllowsAccessRequests_(&self, allowsAccessRequests: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAccessRequests : allowsAccessRequests)
    }
    unsafe fn standardOptions() -> CKAllowedSharingOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKAllowedSharingOptions").unwrap(), standardOptions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSystemSharingUIObserver(pub id);
impl std::ops::Deref for CKSystemSharingUIObserver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSystemSharingUIObserver {}
impl CKSystemSharingUIObserver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSystemSharingUIObserver").unwrap(), alloc) })
    }
}
impl INSObject for CKSystemSharingUIObserver {}
impl PNSObject for CKSystemSharingUIObserver {}
impl std::convert::TryFrom<NSObject> for CKSystemSharingUIObserver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSystemSharingUIObserver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSystemSharingUIObserver").unwrap()) };
        if is_kind_of {
            Ok(CKSystemSharingUIObserver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSystemSharingUIObserver")
        }
    }
}
impl ICKSystemSharingUIObserver for CKSystemSharingUIObserver {}
pub trait ICKSystemSharingUIObserver: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithContainer_(&self, container: CKContainer) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContainer : container)
    }
    unsafe fn systemSharingUIDidSaveShareBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemSharingUIDidSaveShareBlock)
    }
    unsafe fn setSystemSharingUIDidSaveShareBlock_(
        &self,
        systemSharingUIDidSaveShareBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSystemSharingUIDidSaveShareBlock : systemSharingUIDidSaveShareBlock)
    }
    unsafe fn systemSharingUIDidStopSharingBlock(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemSharingUIDidStopSharingBlock)
    }
    unsafe fn setSystemSharingUIDidStopSharingBlock_(
        &self,
        systemSharingUIDidStopSharingBlock: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSystemSharingUIDidStopSharingBlock : systemSharingUIDidStopSharingBlock)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSystemSharingUIObserver").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineState(pub id);
impl std::ops::Deref for CKSyncEngineState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineState {}
impl CKSyncEngineState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineState").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineState {}
impl PNSObject for CKSyncEngineState {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineState").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEngineState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineState")
        }
    }
}
impl ICKSyncEngineState for CKSyncEngineState {}
pub trait ICKSyncEngineState: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addPendingRecordZoneChanges_(&self, changes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPendingRecordZoneChanges : changes)
    }
    unsafe fn removePendingRecordZoneChanges_(&self, changes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePendingRecordZoneChanges : changes)
    }
    unsafe fn addPendingDatabaseChanges_(&self, changes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPendingDatabaseChanges : changes)
    }
    unsafe fn removePendingDatabaseChanges_(&self, changes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePendingDatabaseChanges : changes)
    }
    unsafe fn pendingRecordZoneChanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pendingRecordZoneChanges)
    }
    unsafe fn pendingDatabaseChanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pendingDatabaseChanges)
    }
    unsafe fn hasPendingUntrackedChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasPendingUntrackedChanges)
    }
    unsafe fn setHasPendingUntrackedChanges_(&self, hasPendingUntrackedChanges: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasPendingUntrackedChanges : hasPendingUntrackedChanges)
    }
    unsafe fn zoneIDsWithUnfetchedServerChanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneIDsWithUnfetchedServerChanges)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineState").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineStateSerialization(pub id);
impl std::ops::Deref for CKSyncEngineStateSerialization {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineStateSerialization {}
impl CKSyncEngineStateSerialization {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineStateSerialization").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CKSyncEngineStateSerialization {}
impl INSObject for CKSyncEngineStateSerialization {}
impl PNSObject for CKSyncEngineStateSerialization {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineStateSerialization {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineStateSerialization, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineStateSerialization").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineStateSerialization(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineStateSerialization")
        }
    }
}
impl ICKSyncEngineStateSerialization for CKSyncEngineStateSerialization {}
pub trait ICKSyncEngineStateSerialization: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineStateSerialization").unwrap(), new)
    }
}
pub type CKSyncEnginePendingRecordZoneChangeType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEnginePendingRecordZoneChange(pub id);
impl std::ops::Deref for CKSyncEnginePendingRecordZoneChange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEnginePendingRecordZoneChange {}
impl CKSyncEnginePendingRecordZoneChange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEnginePendingRecordZoneChange").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEnginePendingRecordZoneChange {}
impl PNSObject for CKSyncEnginePendingRecordZoneChange {}
impl std::convert::TryFrom<NSObject> for CKSyncEnginePendingRecordZoneChange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEnginePendingRecordZoneChange, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEnginePendingRecordZoneChange").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEnginePendingRecordZoneChange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEnginePendingRecordZoneChange")
        }
    }
}
impl ICKSyncEnginePendingRecordZoneChange for CKSyncEnginePendingRecordZoneChange {}
pub trait ICKSyncEnginePendingRecordZoneChange: Sized + std::ops::Deref {
    unsafe fn initWithRecordID_type_(
        &self,
        recordID: CKRecordID,
        type_: CKSyncEnginePendingRecordZoneChangeType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordID : recordID, r#type : type_)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn recordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordID)
    }
    unsafe fn type_(&self) -> CKSyncEnginePendingRecordZoneChangeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEnginePendingRecordZoneChange").unwrap(), new)
    }
}
pub type CKSyncEnginePendingDatabaseChangeType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEnginePendingDatabaseChange(pub id);
impl std::ops::Deref for CKSyncEnginePendingDatabaseChange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEnginePendingDatabaseChange {}
impl CKSyncEnginePendingDatabaseChange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEnginePendingDatabaseChange").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEnginePendingDatabaseChange {}
impl PNSObject for CKSyncEnginePendingDatabaseChange {}
impl std::convert::TryFrom<NSObject> for CKSyncEnginePendingDatabaseChange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEnginePendingDatabaseChange, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEnginePendingDatabaseChange").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEnginePendingDatabaseChange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEnginePendingDatabaseChange")
        }
    }
}
impl ICKSyncEnginePendingDatabaseChange for CKSyncEnginePendingDatabaseChange {}
pub trait ICKSyncEnginePendingDatabaseChange: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
    unsafe fn type_(&self) -> CKSyncEnginePendingDatabaseChangeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEnginePendingDatabaseChange").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEnginePendingZoneSave(pub id);
impl std::ops::Deref for CKSyncEnginePendingZoneSave {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEnginePendingZoneSave {}
impl CKSyncEnginePendingZoneSave {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEnginePendingZoneSave").unwrap(), alloc) })
    }
}
impl ICKSyncEnginePendingDatabaseChange for CKSyncEnginePendingZoneSave {}
impl From<CKSyncEnginePendingZoneSave> for CKSyncEnginePendingDatabaseChange {
    fn from(child: CKSyncEnginePendingZoneSave) -> CKSyncEnginePendingDatabaseChange {
        CKSyncEnginePendingDatabaseChange(child.0)
    }
}
impl std::convert::TryFrom<CKSyncEnginePendingDatabaseChange> for CKSyncEnginePendingZoneSave {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEnginePendingDatabaseChange,
    ) -> Result<CKSyncEnginePendingZoneSave, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEnginePendingZoneSave").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEnginePendingZoneSave(parent.0))
        } else {
            Err ("This CKSyncEnginePendingDatabaseChange cannot be downcasted to CKSyncEnginePendingZoneSave" ,)
        }
    }
}
impl INSObject for CKSyncEnginePendingZoneSave {}
impl PNSObject for CKSyncEnginePendingZoneSave {}
impl ICKSyncEnginePendingZoneSave for CKSyncEnginePendingZoneSave {}
pub trait ICKSyncEnginePendingZoneSave: Sized + std::ops::Deref {
    unsafe fn initWithZone_(&self, zone: CKRecordZone) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZone : zone)
    }
    unsafe fn zone(&self) -> CKRecordZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zone)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEnginePendingZoneDelete(pub id);
impl std::ops::Deref for CKSyncEnginePendingZoneDelete {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEnginePendingZoneDelete {}
impl CKSyncEnginePendingZoneDelete {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEnginePendingZoneDelete").unwrap(), alloc) })
    }
}
impl ICKSyncEnginePendingDatabaseChange for CKSyncEnginePendingZoneDelete {}
impl std::convert::TryFrom<CKSyncEnginePendingDatabaseChange> for CKSyncEnginePendingZoneDelete {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEnginePendingDatabaseChange,
    ) -> Result<CKSyncEnginePendingZoneDelete, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEnginePendingZoneDelete").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEnginePendingZoneDelete(parent.0))
        } else {
            Err ("This CKSyncEnginePendingDatabaseChange cannot be downcasted to CKSyncEnginePendingZoneDelete" ,)
        }
    }
}
impl INSObject for CKSyncEnginePendingZoneDelete {}
impl PNSObject for CKSyncEnginePendingZoneDelete {}
impl ICKSyncEnginePendingZoneDelete for CKSyncEnginePendingZoneDelete {}
pub trait ICKSyncEnginePendingZoneDelete: Sized + std::ops::Deref {
    unsafe fn initWithZoneID_(&self, zoneID: CKRecordZoneID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZoneID : zoneID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineRecordZoneChangeBatch(pub id);
impl std::ops::Deref for CKSyncEngineRecordZoneChangeBatch {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineRecordZoneChangeBatch {}
impl CKSyncEngineRecordZoneChangeBatch {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineRecordZoneChangeBatch").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineRecordZoneChangeBatch {}
impl PNSObject for CKSyncEngineRecordZoneChangeBatch {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineRecordZoneChangeBatch {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineRecordZoneChangeBatch, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineRecordZoneChangeBatch").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineRecordZoneChangeBatch(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineRecordZoneChangeBatch")
        }
    }
}
impl ICKSyncEngineRecordZoneChangeBatch for CKSyncEngineRecordZoneChangeBatch {}
pub trait ICKSyncEngineRecordZoneChangeBatch: Sized + std::ops::Deref {
    unsafe fn initWithPendingChanges_recordProvider_(
        &self,
        pendingChanges: NSArray,
        recordProvider: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPendingChanges : pendingChanges, recordProvider : recordProvider)
    }
    unsafe fn initWithRecordsToSave_recordIDsToDelete_atomicByZone_(
        &self,
        recordsToSave: NSArray,
        recordIDsToDelete: NSArray,
        atomicByZone: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordsToSave : recordsToSave, recordIDsToDelete : recordIDsToDelete, atomicByZone : atomicByZone)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn recordsToSave(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordsToSave)
    }
    unsafe fn recordIDsToDelete(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordIDsToDelete)
    }
    unsafe fn atomicByZone(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, atomicByZone)
    }
    unsafe fn setAtomicByZone_(&self, atomicByZone: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAtomicByZone : atomicByZone)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineRecordZoneChangeBatch").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngine(pub id);
impl std::ops::Deref for CKSyncEngine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngine {}
impl CKSyncEngine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngine").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngine {}
impl PNSObject for CKSyncEngine {}
impl std::convert::TryFrom<NSObject> for CKSyncEngine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngine").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEngine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngine")
        }
    }
}
impl ICKSyncEngine for CKSyncEngine {}
pub trait ICKSyncEngine: Sized + std::ops::Deref {
    unsafe fn initWithConfiguration_(
        &self,
        configuration: CKSyncEngineConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fetchChangesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchChangesWithCompletionHandler : completionHandler)
    }
    unsafe fn fetchChangesWithOptions_completionHandler_(
        &self,
        options: CKSyncEngineFetchChangesOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchChangesWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn sendChangesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendChangesWithCompletionHandler : completionHandler)
    }
    unsafe fn sendChangesWithOptions_completionHandler_(
        &self,
        options: CKSyncEngineSendChangesOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendChangesWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn cancelOperationsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelOperationsWithCompletionHandler : completionHandler)
    }
    unsafe fn database(&self) -> CKDatabase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, database)
    }
    unsafe fn state(&self) -> CKSyncEngineState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngine").unwrap(), new)
    }
}
pub trait PCKSyncEngineDelegate: Sized + std::ops::Deref {
    unsafe fn syncEngine_handleEvent_(&self, syncEngine: CKSyncEngine, event: CKSyncEngineEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, syncEngine : syncEngine, handleEvent : event)
    }
    unsafe fn syncEngine_nextRecordZoneChangeBatchForContext_(
        &self,
        syncEngine: CKSyncEngine,
        context: CKSyncEngineSendChangesContext,
    ) -> CKSyncEngineRecordZoneChangeBatch
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, syncEngine : syncEngine, nextRecordZoneChangeBatchForContext : context)
    }
    unsafe fn syncEngine_nextFetchChangesOptionsForContext_(
        &self,
        syncEngine: CKSyncEngine,
        context: CKSyncEngineFetchChangesContext,
    ) -> CKSyncEngineFetchChangesOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, syncEngine : syncEngine, nextFetchChangesOptionsForContext : context)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFetchChangesOptions(pub id);
impl std::ops::Deref for CKSyncEngineFetchChangesOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFetchChangesOptions {}
impl CKSyncEngineFetchChangesOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchChangesOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for CKSyncEngineFetchChangesOptions {}
impl INSObject for CKSyncEngineFetchChangesOptions {}
impl PNSObject for CKSyncEngineFetchChangesOptions {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineFetchChangesOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineFetchChangesOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFetchChangesOptions").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineFetchChangesOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineFetchChangesOptions")
        }
    }
}
impl ICKSyncEngineFetchChangesOptions for CKSyncEngineFetchChangesOptions {}
pub trait ICKSyncEngineFetchChangesOptions: Sized + std::ops::Deref {
    unsafe fn initWithScope_(&self, scope: CKSyncEngineFetchChangesScope) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithScope : scope)
    }
    unsafe fn scope(&self) -> CKSyncEngineFetchChangesScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scope)
    }
    unsafe fn setScope_(&self, scope: CKSyncEngineFetchChangesScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScope : scope)
    }
    unsafe fn operationGroup(&self) -> CKOperationGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operationGroup)
    }
    unsafe fn setOperationGroup_(&self, operationGroup: CKOperationGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOperationGroup : operationGroup)
    }
    unsafe fn prioritizedZoneIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prioritizedZoneIDs)
    }
    unsafe fn setPrioritizedZoneIDs_(&self, prioritizedZoneIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrioritizedZoneIDs : prioritizedZoneIDs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFetchChangesScope(pub id);
impl std::ops::Deref for CKSyncEngineFetchChangesScope {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFetchChangesScope {}
impl CKSyncEngineFetchChangesScope {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchChangesScope").unwrap(), alloc) })
    }
}
impl PNSCopying for CKSyncEngineFetchChangesScope {}
impl INSObject for CKSyncEngineFetchChangesScope {}
impl PNSObject for CKSyncEngineFetchChangesScope {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineFetchChangesScope {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineFetchChangesScope, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFetchChangesScope").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineFetchChangesScope(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineFetchChangesScope")
        }
    }
}
impl ICKSyncEngineFetchChangesScope for CKSyncEngineFetchChangesScope {}
pub trait ICKSyncEngineFetchChangesScope: Sized + std::ops::Deref {
    unsafe fn initWithZoneIDs_(&self, zoneIDs: NSSet) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZoneIDs : zoneIDs)
    }
    unsafe fn initWithExcludedZoneIDs_(&self, zoneIDs: NSSet) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExcludedZoneIDs : zoneIDs)
    }
    unsafe fn containsZoneID_(&self, zoneID: CKRecordZoneID) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsZoneID : zoneID)
    }
    unsafe fn zoneIDs(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneIDs)
    }
    unsafe fn excludedZoneIDs(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedZoneIDs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineSendChangesOptions(pub id);
impl std::ops::Deref for CKSyncEngineSendChangesOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineSendChangesOptions {}
impl CKSyncEngineSendChangesOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineSendChangesOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for CKSyncEngineSendChangesOptions {}
impl INSObject for CKSyncEngineSendChangesOptions {}
impl PNSObject for CKSyncEngineSendChangesOptions {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineSendChangesOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineSendChangesOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineSendChangesOptions").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineSendChangesOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineSendChangesOptions")
        }
    }
}
impl ICKSyncEngineSendChangesOptions for CKSyncEngineSendChangesOptions {}
pub trait ICKSyncEngineSendChangesOptions: Sized + std::ops::Deref {
    unsafe fn initWithScope_(&self, scope: CKSyncEngineSendChangesScope) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithScope : scope)
    }
    unsafe fn scope(&self) -> CKSyncEngineSendChangesScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scope)
    }
    unsafe fn setScope_(&self, scope: CKSyncEngineSendChangesScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScope : scope)
    }
    unsafe fn operationGroup(&self) -> CKOperationGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operationGroup)
    }
    unsafe fn setOperationGroup_(&self, operationGroup: CKOperationGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOperationGroup : operationGroup)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineSendChangesScope(pub id);
impl std::ops::Deref for CKSyncEngineSendChangesScope {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineSendChangesScope {}
impl CKSyncEngineSendChangesScope {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineSendChangesScope").unwrap(), alloc) })
    }
}
impl PNSCopying for CKSyncEngineSendChangesScope {}
impl INSObject for CKSyncEngineSendChangesScope {}
impl PNSObject for CKSyncEngineSendChangesScope {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineSendChangesScope {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineSendChangesScope, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineSendChangesScope").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEngineSendChangesScope(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineSendChangesScope")
        }
    }
}
impl ICKSyncEngineSendChangesScope for CKSyncEngineSendChangesScope {}
pub trait ICKSyncEngineSendChangesScope: Sized + std::ops::Deref {
    unsafe fn initWithZoneIDs_(&self, zoneIDs: NSSet) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithZoneIDs : zoneIDs)
    }
    unsafe fn initWithExcludedZoneIDs_(&self, excludedZoneIDs: NSSet) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExcludedZoneIDs : excludedZoneIDs)
    }
    unsafe fn initWithRecordIDs_(&self, recordIDs: NSSet) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRecordIDs : recordIDs)
    }
    unsafe fn containsRecordID_(&self, recordID: CKRecordID) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsRecordID : recordID)
    }
    unsafe fn containsPendingRecordZoneChange_(
        &self,
        pendingRecordZoneChange: CKSyncEnginePendingRecordZoneChange,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsPendingRecordZoneChange : pendingRecordZoneChange)
    }
    unsafe fn zoneIDs(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneIDs)
    }
    unsafe fn excludedZoneIDs(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedZoneIDs)
    }
    unsafe fn recordIDs(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordIDs)
    }
}
pub type CKSyncEngineSyncReason = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFetchChangesContext(pub id);
impl std::ops::Deref for CKSyncEngineFetchChangesContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFetchChangesContext {}
impl CKSyncEngineFetchChangesContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchChangesContext").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineFetchChangesContext {}
impl PNSObject for CKSyncEngineFetchChangesContext {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineFetchChangesContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineFetchChangesContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFetchChangesContext").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineFetchChangesContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineFetchChangesContext")
        }
    }
}
impl ICKSyncEngineFetchChangesContext for CKSyncEngineFetchChangesContext {}
pub trait ICKSyncEngineFetchChangesContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn reason(&self) -> CKSyncEngineSyncReason
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reason)
    }
    unsafe fn options(&self) -> CKSyncEngineFetchChangesOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchChangesContext").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineSendChangesContext(pub id);
impl std::ops::Deref for CKSyncEngineSendChangesContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineSendChangesContext {}
impl CKSyncEngineSendChangesContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineSendChangesContext").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineSendChangesContext {}
impl PNSObject for CKSyncEngineSendChangesContext {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineSendChangesContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineSendChangesContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineSendChangesContext").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineSendChangesContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineSendChangesContext")
        }
    }
}
impl ICKSyncEngineSendChangesContext for CKSyncEngineSendChangesContext {}
pub trait ICKSyncEngineSendChangesContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn reason(&self) -> CKSyncEngineSyncReason
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reason)
    }
    unsafe fn options(&self) -> CKSyncEngineSendChangesOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineSendChangesContext").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineConfiguration(pub id);
impl std::ops::Deref for CKSyncEngineConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineConfiguration {}
impl CKSyncEngineConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineConfiguration {}
impl PNSObject for CKSyncEngineConfiguration {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineConfiguration").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEngineConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineConfiguration")
        }
    }
}
impl ICKSyncEngineConfiguration for CKSyncEngineConfiguration {}
pub trait ICKSyncEngineConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithDatabase_stateSerialization_delegate_(
        &self,
        database: CKDatabase,
        stateSerialization: CKSyncEngineStateSerialization,
        delegate: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDatabase : database, stateSerialization : stateSerialization, delegate : delegate)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn database(&self) -> CKDatabase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, database)
    }
    unsafe fn setDatabase_(&self, database: CKDatabase)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDatabase : database)
    }
    unsafe fn stateSerialization(&self) -> CKSyncEngineStateSerialization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateSerialization)
    }
    unsafe fn setStateSerialization_(&self, stateSerialization: CKSyncEngineStateSerialization)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStateSerialization : stateSerialization)
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
    unsafe fn automaticallySync(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticallySync)
    }
    unsafe fn setAutomaticallySync_(&self, automaticallySync: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticallySync : automaticallySync)
    }
    unsafe fn subscriptionID(&self) -> CKSubscriptionID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionID)
    }
    unsafe fn setSubscriptionID_(&self, subscriptionID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubscriptionID : subscriptionID)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineConfiguration").unwrap(), new)
    }
}
pub type CKSyncEngineEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineEvent(pub id);
impl std::ops::Deref for CKSyncEngineEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineEvent {}
impl CKSyncEngineEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineEvent").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineEvent {}
impl PNSObject for CKSyncEngineEvent {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineEvent").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEngineEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineEvent")
        }
    }
}
impl ICKSyncEngineEvent for CKSyncEngineEvent {}
pub trait ICKSyncEngineEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn type_(&self) -> CKSyncEngineEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn stateUpdateEvent(&self) -> CKSyncEngineStateUpdateEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateUpdateEvent)
    }
    unsafe fn accountChangeEvent(&self) -> CKSyncEngineAccountChangeEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountChangeEvent)
    }
    unsafe fn willFetchChangesEvent(&self) -> CKSyncEngineWillFetchChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willFetchChangesEvent)
    }
    unsafe fn fetchedDatabaseChangesEvent(&self) -> CKSyncEngineFetchedDatabaseChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchedDatabaseChangesEvent)
    }
    unsafe fn didFetchChangesEvent(&self) -> CKSyncEngineDidFetchChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didFetchChangesEvent)
    }
    unsafe fn willFetchRecordZoneChangesEvent(&self) -> CKSyncEngineWillFetchRecordZoneChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willFetchRecordZoneChangesEvent)
    }
    unsafe fn fetchedRecordZoneChangesEvent(&self) -> CKSyncEngineFetchedRecordZoneChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchedRecordZoneChangesEvent)
    }
    unsafe fn didFetchRecordZoneChangesEvent(&self) -> CKSyncEngineDidFetchRecordZoneChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didFetchRecordZoneChangesEvent)
    }
    unsafe fn willSendChangesEvent(&self) -> CKSyncEngineWillSendChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willSendChangesEvent)
    }
    unsafe fn sentDatabaseChangesEvent(&self) -> CKSyncEngineSentDatabaseChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sentDatabaseChangesEvent)
    }
    unsafe fn sentRecordZoneChangesEvent(&self) -> CKSyncEngineSentRecordZoneChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sentRecordZoneChangesEvent)
    }
    unsafe fn didSendChangesEvent(&self) -> CKSyncEngineDidSendChangesEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didSendChangesEvent)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineEvent").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineStateUpdateEvent(pub id);
impl std::ops::Deref for CKSyncEngineStateUpdateEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineStateUpdateEvent {}
impl CKSyncEngineStateUpdateEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineStateUpdateEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineStateUpdateEvent {}
impl From<CKSyncEngineStateUpdateEvent> for CKSyncEngineEvent {
    fn from(child: CKSyncEngineStateUpdateEvent) -> CKSyncEngineEvent {
        CKSyncEngineEvent(child.0)
    }
}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineStateUpdateEvent {
    type Error = &'static str;
    fn try_from(parent: CKSyncEngineEvent) -> Result<CKSyncEngineStateUpdateEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineStateUpdateEvent").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEngineStateUpdateEvent(parent.0))
        } else {
            Err("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineStateUpdateEvent")
        }
    }
}
impl INSObject for CKSyncEngineStateUpdateEvent {}
impl PNSObject for CKSyncEngineStateUpdateEvent {}
impl ICKSyncEngineStateUpdateEvent for CKSyncEngineStateUpdateEvent {}
pub trait ICKSyncEngineStateUpdateEvent: Sized + std::ops::Deref {
    unsafe fn stateSerialization(&self) -> CKSyncEngineStateSerialization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateSerialization)
    }
}
pub type CKSyncEngineAccountChangeType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineAccountChangeEvent(pub id);
impl std::ops::Deref for CKSyncEngineAccountChangeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineAccountChangeEvent {}
impl CKSyncEngineAccountChangeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineAccountChangeEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineAccountChangeEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineAccountChangeEvent {
    type Error = &'static str;
    fn try_from(parent: CKSyncEngineEvent) -> Result<CKSyncEngineAccountChangeEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineAccountChangeEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineAccountChangeEvent(parent.0))
        } else {
            Err("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineAccountChangeEvent")
        }
    }
}
impl INSObject for CKSyncEngineAccountChangeEvent {}
impl PNSObject for CKSyncEngineAccountChangeEvent {}
impl ICKSyncEngineAccountChangeEvent for CKSyncEngineAccountChangeEvent {}
pub trait ICKSyncEngineAccountChangeEvent: Sized + std::ops::Deref {
    unsafe fn changeType(&self) -> CKSyncEngineAccountChangeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeType)
    }
    unsafe fn previousUser(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousUser)
    }
    unsafe fn currentUser(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentUser)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFetchedDatabaseChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineFetchedDatabaseChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFetchedDatabaseChangesEvent {}
impl CKSyncEngineFetchedDatabaseChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedDatabaseChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineFetchedDatabaseChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineFetchedDatabaseChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineFetchedDatabaseChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedDatabaseChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineFetchedDatabaseChangesEvent(parent.0))
        } else {
            Err ("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineFetchedDatabaseChangesEvent" ,)
        }
    }
}
impl INSObject for CKSyncEngineFetchedDatabaseChangesEvent {}
impl PNSObject for CKSyncEngineFetchedDatabaseChangesEvent {}
impl ICKSyncEngineFetchedDatabaseChangesEvent for CKSyncEngineFetchedDatabaseChangesEvent {}
pub trait ICKSyncEngineFetchedDatabaseChangesEvent: Sized + std::ops::Deref {
    unsafe fn modifications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifications)
    }
    unsafe fn deletions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deletions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFetchedRecordZoneChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineFetchedRecordZoneChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFetchedRecordZoneChangesEvent {}
impl CKSyncEngineFetchedRecordZoneChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedRecordZoneChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineFetchedRecordZoneChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineFetchedRecordZoneChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineFetchedRecordZoneChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedRecordZoneChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineFetchedRecordZoneChangesEvent(parent.0))
        } else {
            Err ("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineFetchedRecordZoneChangesEvent" ,)
        }
    }
}
impl INSObject for CKSyncEngineFetchedRecordZoneChangesEvent {}
impl PNSObject for CKSyncEngineFetchedRecordZoneChangesEvent {}
impl ICKSyncEngineFetchedRecordZoneChangesEvent for CKSyncEngineFetchedRecordZoneChangesEvent {}
pub trait ICKSyncEngineFetchedRecordZoneChangesEvent: Sized + std::ops::Deref {
    unsafe fn modifications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifications)
    }
    unsafe fn deletions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deletions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineSentDatabaseChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineSentDatabaseChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineSentDatabaseChangesEvent {}
impl CKSyncEngineSentDatabaseChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineSentDatabaseChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineSentDatabaseChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineSentDatabaseChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineSentDatabaseChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineSentDatabaseChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineSentDatabaseChangesEvent(parent.0))
        } else {
            Err ("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineSentDatabaseChangesEvent" ,)
        }
    }
}
impl INSObject for CKSyncEngineSentDatabaseChangesEvent {}
impl PNSObject for CKSyncEngineSentDatabaseChangesEvent {}
impl ICKSyncEngineSentDatabaseChangesEvent for CKSyncEngineSentDatabaseChangesEvent {}
pub trait ICKSyncEngineSentDatabaseChangesEvent: Sized + std::ops::Deref {
    unsafe fn savedZones(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, savedZones)
    }
    unsafe fn failedZoneSaves(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, failedZoneSaves)
    }
    unsafe fn deletedZoneIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deletedZoneIDs)
    }
    unsafe fn failedZoneDeletes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, failedZoneDeletes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineSentRecordZoneChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineSentRecordZoneChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineSentRecordZoneChangesEvent {}
impl CKSyncEngineSentRecordZoneChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineSentRecordZoneChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineSentRecordZoneChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineSentRecordZoneChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineSentRecordZoneChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineSentRecordZoneChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineSentRecordZoneChangesEvent(parent.0))
        } else {
            Err ("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineSentRecordZoneChangesEvent" ,)
        }
    }
}
impl INSObject for CKSyncEngineSentRecordZoneChangesEvent {}
impl PNSObject for CKSyncEngineSentRecordZoneChangesEvent {}
impl ICKSyncEngineSentRecordZoneChangesEvent for CKSyncEngineSentRecordZoneChangesEvent {}
pub trait ICKSyncEngineSentRecordZoneChangesEvent: Sized + std::ops::Deref {
    unsafe fn savedRecords(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, savedRecords)
    }
    unsafe fn failedRecordSaves(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, failedRecordSaves)
    }
    unsafe fn deletedRecordIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deletedRecordIDs)
    }
    unsafe fn failedRecordDeletes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, failedRecordDeletes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineWillFetchChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineWillFetchChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineWillFetchChangesEvent {}
impl CKSyncEngineWillFetchChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineWillFetchChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineWillFetchChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineWillFetchChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineWillFetchChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineWillFetchChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineWillFetchChangesEvent(parent.0))
        } else {
            Err("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineWillFetchChangesEvent")
        }
    }
}
impl INSObject for CKSyncEngineWillFetchChangesEvent {}
impl PNSObject for CKSyncEngineWillFetchChangesEvent {}
impl ICKSyncEngineWillFetchChangesEvent for CKSyncEngineWillFetchChangesEvent {}
pub trait ICKSyncEngineWillFetchChangesEvent: Sized + std::ops::Deref {
    unsafe fn context(&self) -> CKSyncEngineFetchChangesContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineWillFetchRecordZoneChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineWillFetchRecordZoneChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineWillFetchRecordZoneChangesEvent {}
impl CKSyncEngineWillFetchRecordZoneChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineWillFetchRecordZoneChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineWillFetchRecordZoneChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineWillFetchRecordZoneChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineWillFetchRecordZoneChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineWillFetchRecordZoneChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineWillFetchRecordZoneChangesEvent(parent.0))
        } else {
            Err ("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineWillFetchRecordZoneChangesEvent" ,)
        }
    }
}
impl INSObject for CKSyncEngineWillFetchRecordZoneChangesEvent {}
impl PNSObject for CKSyncEngineWillFetchRecordZoneChangesEvent {}
impl ICKSyncEngineWillFetchRecordZoneChangesEvent for CKSyncEngineWillFetchRecordZoneChangesEvent {}
pub trait ICKSyncEngineWillFetchRecordZoneChangesEvent: Sized + std::ops::Deref {
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineDidFetchRecordZoneChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineDidFetchRecordZoneChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineDidFetchRecordZoneChangesEvent {}
impl CKSyncEngineDidFetchRecordZoneChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineDidFetchRecordZoneChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineDidFetchRecordZoneChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineDidFetchRecordZoneChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineDidFetchRecordZoneChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineDidFetchRecordZoneChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineDidFetchRecordZoneChangesEvent(parent.0))
        } else {
            Err ("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineDidFetchRecordZoneChangesEvent" ,)
        }
    }
}
impl INSObject for CKSyncEngineDidFetchRecordZoneChangesEvent {}
impl PNSObject for CKSyncEngineDidFetchRecordZoneChangesEvent {}
impl ICKSyncEngineDidFetchRecordZoneChangesEvent for CKSyncEngineDidFetchRecordZoneChangesEvent {}
pub trait ICKSyncEngineDidFetchRecordZoneChangesEvent: Sized + std::ops::Deref {
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineDidFetchChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineDidFetchChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineDidFetchChangesEvent {}
impl CKSyncEngineDidFetchChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineDidFetchChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineDidFetchChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineDidFetchChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineDidFetchChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineDidFetchChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineDidFetchChangesEvent(parent.0))
        } else {
            Err("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineDidFetchChangesEvent")
        }
    }
}
impl INSObject for CKSyncEngineDidFetchChangesEvent {}
impl PNSObject for CKSyncEngineDidFetchChangesEvent {}
impl ICKSyncEngineDidFetchChangesEvent for CKSyncEngineDidFetchChangesEvent {}
pub trait ICKSyncEngineDidFetchChangesEvent: Sized + std::ops::Deref {
    unsafe fn context(&self) -> CKSyncEngineFetchChangesContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineWillSendChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineWillSendChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineWillSendChangesEvent {}
impl CKSyncEngineWillSendChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineWillSendChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineWillSendChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineWillSendChangesEvent {
    type Error = &'static str;
    fn try_from(
        parent: CKSyncEngineEvent,
    ) -> Result<CKSyncEngineWillSendChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineWillSendChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineWillSendChangesEvent(parent.0))
        } else {
            Err("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineWillSendChangesEvent")
        }
    }
}
impl INSObject for CKSyncEngineWillSendChangesEvent {}
impl PNSObject for CKSyncEngineWillSendChangesEvent {}
impl ICKSyncEngineWillSendChangesEvent for CKSyncEngineWillSendChangesEvent {}
pub trait ICKSyncEngineWillSendChangesEvent: Sized + std::ops::Deref {
    unsafe fn context(&self) -> CKSyncEngineSendChangesContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineDidSendChangesEvent(pub id);
impl std::ops::Deref for CKSyncEngineDidSendChangesEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineDidSendChangesEvent {}
impl CKSyncEngineDidSendChangesEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineDidSendChangesEvent").unwrap(), alloc) })
    }
}
impl ICKSyncEngineEvent for CKSyncEngineDidSendChangesEvent {}
impl std::convert::TryFrom<CKSyncEngineEvent> for CKSyncEngineDidSendChangesEvent {
    type Error = &'static str;
    fn try_from(parent: CKSyncEngineEvent) -> Result<CKSyncEngineDidSendChangesEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineDidSendChangesEvent").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineDidSendChangesEvent(parent.0))
        } else {
            Err("This CKSyncEngineEvent cannot be downcasted to CKSyncEngineDidSendChangesEvent")
        }
    }
}
impl INSObject for CKSyncEngineDidSendChangesEvent {}
impl PNSObject for CKSyncEngineDidSendChangesEvent {}
impl ICKSyncEngineDidSendChangesEvent for CKSyncEngineDidSendChangesEvent {}
pub trait ICKSyncEngineDidSendChangesEvent: Sized + std::ops::Deref {
    unsafe fn context(&self) -> CKSyncEngineSendChangesContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFetchedRecordDeletion(pub id);
impl std::ops::Deref for CKSyncEngineFetchedRecordDeletion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFetchedRecordDeletion {}
impl CKSyncEngineFetchedRecordDeletion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedRecordDeletion").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineFetchedRecordDeletion {}
impl PNSObject for CKSyncEngineFetchedRecordDeletion {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineFetchedRecordDeletion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineFetchedRecordDeletion, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedRecordDeletion").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineFetchedRecordDeletion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineFetchedRecordDeletion")
        }
    }
}
impl ICKSyncEngineFetchedRecordDeletion for CKSyncEngineFetchedRecordDeletion {}
pub trait ICKSyncEngineFetchedRecordDeletion: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn recordID(&self) -> CKRecordID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordID)
    }
    unsafe fn recordType(&self) -> CKRecordType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordType)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedRecordDeletion").unwrap(), new)
    }
}
pub type CKSyncEngineZoneDeletionReason = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFetchedZoneDeletion(pub id);
impl std::ops::Deref for CKSyncEngineFetchedZoneDeletion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFetchedZoneDeletion {}
impl CKSyncEngineFetchedZoneDeletion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedZoneDeletion").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineFetchedZoneDeletion {}
impl PNSObject for CKSyncEngineFetchedZoneDeletion {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineFetchedZoneDeletion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineFetchedZoneDeletion, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedZoneDeletion").unwrap())
        };
        if is_kind_of {
            Ok(CKSyncEngineFetchedZoneDeletion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineFetchedZoneDeletion")
        }
    }
}
impl ICKSyncEngineFetchedZoneDeletion for CKSyncEngineFetchedZoneDeletion {}
pub trait ICKSyncEngineFetchedZoneDeletion: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn zoneID(&self) -> CKRecordZoneID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoneID)
    }
    unsafe fn reason(&self) -> CKSyncEngineZoneDeletionReason
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reason)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFetchedZoneDeletion").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFailedRecordSave(pub id);
impl std::ops::Deref for CKSyncEngineFailedRecordSave {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFailedRecordSave {}
impl CKSyncEngineFailedRecordSave {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFailedRecordSave").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineFailedRecordSave {}
impl PNSObject for CKSyncEngineFailedRecordSave {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineFailedRecordSave {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineFailedRecordSave, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFailedRecordSave").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEngineFailedRecordSave(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineFailedRecordSave")
        }
    }
}
impl ICKSyncEngineFailedRecordSave for CKSyncEngineFailedRecordSave {}
pub trait ICKSyncEngineFailedRecordSave: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn record(&self) -> CKRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, record)
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFailedRecordSave").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CKSyncEngineFailedZoneSave(pub id);
impl std::ops::Deref for CKSyncEngineFailedZoneSave {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CKSyncEngineFailedZoneSave {}
impl CKSyncEngineFailedZoneSave {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFailedZoneSave").unwrap(), alloc) })
    }
}
impl INSObject for CKSyncEngineFailedZoneSave {}
impl PNSObject for CKSyncEngineFailedZoneSave {}
impl std::convert::TryFrom<NSObject> for CKSyncEngineFailedZoneSave {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CKSyncEngineFailedZoneSave, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CKSyncEngineFailedZoneSave").unwrap()) };
        if is_kind_of {
            Ok(CKSyncEngineFailedZoneSave(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CKSyncEngineFailedZoneSave")
        }
    }
}
impl ICKSyncEngineFailedZoneSave for CKSyncEngineFailedZoneSave {}
pub trait ICKSyncEngineFailedZoneSave: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn recordZone(&self) -> CKRecordZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordZone)
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"CKSyncEngineFailedZoneSave").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static CKRecordTypeUserRecord: CKRecordType;
}
unsafe extern "C" {
    pub static CKRecordRecordIDKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKRecordCreatorUserRecordIDKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKRecordCreationDateKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKRecordLastModifiedUserRecordIDKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKRecordModificationDateKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKRecordParentKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKRecordShareKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKCurrentUserDefaultName: NSString;
}
unsafe extern "C" {
    pub static CKOwnerDefaultName: NSString;
}
unsafe extern "C" {
    pub static CKAccountChangedNotification: NSString;
}
unsafe extern "C" {
    pub static CKErrorDomain: NSString;
}
unsafe extern "C" {
    pub static CKPartialErrorsByItemIDKey: NSString;
}
unsafe extern "C" {
    pub static CKRecordChangedErrorAncestorRecordKey: NSString;
}
unsafe extern "C" {
    pub static CKRecordChangedErrorServerRecordKey: NSString;
}
unsafe extern "C" {
    pub static CKRecordChangedErrorClientRecordKey: NSString;
}
unsafe extern "C" {
    pub static CKErrorUserDidResetEncryptedDataKey: NSString;
}
unsafe extern "C" {
    pub static CKErrorRetryAfterKey: NSString;
}
unsafe extern "C" {
    pub static CKRecordZoneDefaultName: NSString;
}
unsafe extern "C" {
    pub static CKRecordTypeShare: CKRecordType;
}
unsafe extern "C" {
    pub static CKRecordNameZoneWideShare: NSString;
}
unsafe extern "C" {
    pub static CKShareTitleKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKShareThumbnailImageDataKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKShareTypeKey: CKRecordFieldKey;
}
unsafe extern "C" {
    pub static CKQueryOperationMaximumResults: NSUInteger;
}

unsafe impl objc2::encode::RefEncode for CKAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKReference {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKReference {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSubscription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSubscription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKQuerySubscription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKQuerySubscription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKRecordZoneSubscription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKRecordZoneSubscription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKDatabaseSubscription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKDatabaseSubscription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKNotificationInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKNotificationInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKDatabase {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKDatabase {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKOperationConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKOperationConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKContainer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKContainer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKLocationSortDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKLocationSortDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKNotificationID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKNotificationID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKQueryNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKQueryNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKRecordZoneNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKRecordZoneNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKDatabaseNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKDatabaseNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKRecordZone {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKRecordZone {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKRecordID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKRecordID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKRecordZoneID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKRecordZoneID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKDatabaseOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKDatabaseOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKServerChangeToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKServerChangeToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKShareParticipant {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKShareParticipant {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKShare {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKShare {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKShareAccessRequester {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKShareAccessRequester {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKShareBlockedIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKShareBlockedIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKShareMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKShareMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKUserIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKUserIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKUserIdentityLookupInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKUserIdentityLookupInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKAcceptSharesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKAcceptSharesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKDiscoverAllUserIdentitiesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKDiscoverAllUserIdentitiesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKDiscoverUserIdentitiesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKDiscoverUserIdentitiesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchDatabaseChangesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchDatabaseChangesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchNotificationChangesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchNotificationChangesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchRecordChangesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchRecordChangesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchRecordsOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchRecordsOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchRecordZoneChangesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchRecordZoneChangesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchRecordZoneChangesConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchRecordZoneChangesConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchRecordZoneChangesOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchRecordZoneChangesOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchRecordZonesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchRecordZonesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchShareMetadataOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchShareMetadataOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchShareParticipantsOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchShareParticipantsOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchSubscriptionsOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchSubscriptionsOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKFetchWebAuthTokenOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKFetchWebAuthTokenOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKMarkNotificationsReadOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKMarkNotificationsReadOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKModifyBadgeOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKModifyBadgeOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKModifyRecordsOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKModifyRecordsOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKModifyRecordZonesOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKModifyRecordZonesOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKModifySubscriptionsOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKModifySubscriptionsOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKOperationGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKOperationGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKQueryCursor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKQueryCursor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKQueryOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKQueryOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKShareRequestAccessOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKShareRequestAccessOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKAllowedSharingOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKAllowedSharingOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSystemSharingUIObserver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSystemSharingUIObserver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineStateSerialization {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineStateSerialization {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEnginePendingRecordZoneChange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEnginePendingRecordZoneChange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEnginePendingDatabaseChange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEnginePendingDatabaseChange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEnginePendingZoneSave {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEnginePendingZoneSave {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEnginePendingZoneDelete {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEnginePendingZoneDelete {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineRecordZoneChangeBatch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineRecordZoneChangeBatch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFetchChangesOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFetchChangesOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFetchChangesScope {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFetchChangesScope {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineSendChangesOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineSendChangesOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineSendChangesScope {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineSendChangesScope {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFetchChangesContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFetchChangesContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineSendChangesContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineSendChangesContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineStateUpdateEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineStateUpdateEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineAccountChangeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineAccountChangeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFetchedDatabaseChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFetchedDatabaseChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFetchedRecordZoneChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFetchedRecordZoneChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineSentDatabaseChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineSentDatabaseChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineSentRecordZoneChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineSentRecordZoneChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineWillFetchChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineWillFetchChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineWillFetchRecordZoneChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineWillFetchRecordZoneChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineDidFetchRecordZoneChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineDidFetchRecordZoneChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineDidFetchChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineDidFetchChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineWillSendChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineWillSendChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineDidSendChangesEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineDidSendChangesEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFetchedRecordDeletion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFetchedRecordDeletion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFetchedZoneDeletion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFetchedZoneDeletion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFailedRecordSave {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFailedRecordSave {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CKSyncEngineFailedZoneSave {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CKSyncEngineFailedZoneSave {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
