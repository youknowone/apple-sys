#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSPerson(pub id);
impl std::ops::Deref for CSPerson {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSPerson {}
impl CSPerson {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSPerson").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CSPerson {}
impl PNSCopying for CSPerson {}
impl INSObject for CSPerson {}
impl PNSObject for CSPerson {}
impl std::convert::TryFrom<NSObject> for CSPerson {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSPerson, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSPerson").unwrap()) };
        if is_kind_of {
            Ok(CSPerson(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSPerson")
        }
    }
}
impl ICSPerson for CSPerson {}
pub trait ICSPerson: Sized + std::ops::Deref {
    unsafe fn initWithDisplayName_handles_handleIdentifier_(
        &self,
        displayName: NSString,
        handles: NSArray,
        handleIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisplayName : displayName, handles : handles, handleIdentifier : handleIdentifier)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn handles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handles)
    }
    unsafe fn handleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handleIdentifier)
    }
    unsafe fn contactIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactIdentifier)
    }
    unsafe fn setContactIdentifier_(&self, contactIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactIdentifier : contactIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSSearchableItemAttributeSet(pub id);
impl std::ops::Deref for CSSearchableItemAttributeSet {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSSearchableItemAttributeSet {}
impl CSSearchableItemAttributeSet {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSSearchableItemAttributeSet").unwrap(), alloc) })
    }
}
impl PNSCopying for CSSearchableItemAttributeSet {}
impl PNSSecureCoding for CSSearchableItemAttributeSet {}
impl INSObject for CSSearchableItemAttributeSet {}
impl PNSObject for CSSearchableItemAttributeSet {}
impl std::convert::TryFrom<NSObject> for CSSearchableItemAttributeSet {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSSearchableItemAttributeSet, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSSearchableItemAttributeSet").unwrap()) };
        if is_kind_of {
            Ok(CSSearchableItemAttributeSet(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSSearchableItemAttributeSet")
        }
    }
}
impl ICSSearchableItemAttributeSet for CSSearchableItemAttributeSet {}
pub trait ICSSearchableItemAttributeSet: Sized + std::ops::Deref {
    unsafe fn initWithItemContentType_(&self, itemContentType: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithItemContentType : itemContentType)
    }
    unsafe fn initWithContentType_(&self, contentType: UTType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentType : contentType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSLocalizedString(pub id);
impl std::ops::Deref for CSLocalizedString {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSLocalizedString {}
impl CSLocalizedString {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSLocalizedString").unwrap(), alloc) })
    }
}
impl INSString for CSLocalizedString {}
impl PNSCopying for CSLocalizedString {}
impl PNSMutableCopying for CSLocalizedString {}
impl PNSSecureCoding for CSLocalizedString {}
impl std::convert::TryFrom<NSString> for CSLocalizedString {
    type Error = &'static str;
    fn try_from(parent: NSString) -> Result<CSLocalizedString, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSLocalizedString").unwrap()) };
        if is_kind_of {
            Ok(CSLocalizedString(parent.0))
        } else {
            Err("This NSString cannot be downcasted to CSLocalizedString")
        }
    }
}
impl INSObject for CSLocalizedString {}
impl PNSObject for CSLocalizedString {}
impl ICSLocalizedString for CSLocalizedString {}
pub trait ICSLocalizedString: Sized + std::ops::Deref {
    unsafe fn initWithLocalizedStrings_(&self, localizedStrings: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocalizedStrings : localizedStrings)
    }
    unsafe fn localizedString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedString)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSCustomAttributeKey(pub id);
impl std::ops::Deref for CSCustomAttributeKey {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSCustomAttributeKey {}
impl CSCustomAttributeKey {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSCustomAttributeKey").unwrap(), alloc) })
    }
}
impl PNSCopying for CSCustomAttributeKey {}
impl PNSSecureCoding for CSCustomAttributeKey {}
impl INSObject for CSCustomAttributeKey {}
impl PNSObject for CSCustomAttributeKey {}
impl std::convert::TryFrom<NSObject> for CSCustomAttributeKey {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSCustomAttributeKey, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSCustomAttributeKey").unwrap()) };
        if is_kind_of {
            Ok(CSCustomAttributeKey(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSCustomAttributeKey")
        }
    }
}
impl ICSCustomAttributeKey for CSCustomAttributeKey {}
pub trait ICSCustomAttributeKey: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithKeyName_(&self, keyName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKeyName : keyName)
    }
    unsafe fn initWithKeyName_searchable_searchableByDefault_unique_multiValued_(
        &self,
        keyName: NSString,
        searchable: BOOL,
        searchableByDefault: BOOL,
        unique: BOOL,
        multiValued: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKeyName : keyName, searchable : searchable, searchableByDefault : searchableByDefault, unique : unique, multiValued : multiValued)
    }
    unsafe fn keyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyName)
    }
    unsafe fn isSearchable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSearchable)
    }
    unsafe fn isSearchableByDefault(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSearchableByDefault)
    }
    unsafe fn isUnique(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUnique)
    }
    unsafe fn isMultiValued(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMultiValued)
    }
}
impl CSSearchableItemAttributeSet_CSCustomAttributes for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSCustomAttributes: Sized + std::ops::Deref {
    unsafe fn setValue_forCustomKey_(&self, value: *mut u64, key: CSCustomAttributeKey)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forCustomKey : key)
    }
    unsafe fn valueForCustomKey_(&self, key: CSCustomAttributeKey) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForCustomKey : key)
    }
}
pub trait NSUserActivity_CSSearchableItemAttributeSet: Sized + std::ops::Deref {
    unsafe fn contentAttributeSet(&self) -> CSSearchableItemAttributeSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentAttributeSet)
    }
    unsafe fn setContentAttributeSet_(&self, contentAttributeSet: CSSearchableItemAttributeSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentAttributeSet : contentAttributeSet)
    }
}
impl CSSearchableItemAttributeSet_CSGeneral for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSGeneral: Sized + std::ops::Deref {
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
    unsafe fn alternateNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternateNames)
    }
    unsafe fn setAlternateNames_(&self, alternateNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlternateNames : alternateNames)
    }
    unsafe fn path(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn setPath_(&self, path: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPath : path)
    }
    unsafe fn contentURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentURL)
    }
    unsafe fn setContentURL_(&self, contentURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentURL : contentURL)
    }
    unsafe fn thumbnailURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnailURL)
    }
    unsafe fn setThumbnailURL_(&self, thumbnailURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThumbnailURL : thumbnailURL)
    }
    unsafe fn thumbnailData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnailData)
    }
    unsafe fn setThumbnailData_(&self, thumbnailData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThumbnailData : thumbnailData)
    }
    unsafe fn darkThumbnailURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, darkThumbnailURL)
    }
    unsafe fn setDarkThumbnailURL_(&self, darkThumbnailURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDarkThumbnailURL : darkThumbnailURL)
    }
    unsafe fn relatedUniqueIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relatedUniqueIdentifier)
    }
    unsafe fn setRelatedUniqueIdentifier_(&self, relatedUniqueIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelatedUniqueIdentifier : relatedUniqueIdentifier)
    }
    unsafe fn weakRelatedUniqueIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weakRelatedUniqueIdentifier)
    }
    unsafe fn setWeakRelatedUniqueIdentifier_(&self, weakRelatedUniqueIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeakRelatedUniqueIdentifier : weakRelatedUniqueIdentifier)
    }
    unsafe fn metadataModificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadataModificationDate)
    }
    unsafe fn setMetadataModificationDate_(&self, metadataModificationDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMetadataModificationDate : metadataModificationDate)
    }
    unsafe fn contentType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn setContentType_(&self, contentType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentType : contentType)
    }
    unsafe fn contentTypeTree(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentTypeTree)
    }
    unsafe fn setContentTypeTree_(&self, contentTypeTree: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentTypeTree : contentTypeTree)
    }
    unsafe fn keywords(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keywords)
    }
    unsafe fn setKeywords_(&self, keywords: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeywords : keywords)
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
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn setVersion_(&self, version: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersion : version)
    }
    unsafe fn isUserCreated(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserCreated)
    }
    unsafe fn setUserCreated_(&self, userCreated: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserCreated : userCreated)
    }
    unsafe fn isUserOwned(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserOwned)
    }
    unsafe fn setUserOwned_(&self, userOwned: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserOwned : userOwned)
    }
    unsafe fn isUserCurated(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserCurated)
    }
    unsafe fn setUserCurated_(&self, userCurated: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserCurated : userCurated)
    }
    unsafe fn rankingHint(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rankingHint)
    }
    unsafe fn setRankingHint_(&self, rankingHint: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRankingHint : rankingHint)
    }
    unsafe fn domainIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainIdentifier)
    }
    unsafe fn setDomainIdentifier_(&self, domainIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDomainIdentifier : domainIdentifier)
    }
}
impl CSSearchableItemAttributeSet_CSActionExtras for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSActionExtras: Sized + std::ops::Deref {
    unsafe fn supportsPhoneCall(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsPhoneCall)
    }
    unsafe fn setSupportsPhoneCall_(&self, supportsPhoneCall: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsPhoneCall : supportsPhoneCall)
    }
    unsafe fn supportsNavigation(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsNavigation)
    }
    unsafe fn setSupportsNavigation_(&self, supportsNavigation: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsNavigation : supportsNavigation)
    }
    unsafe fn actionIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionIdentifiers)
    }
    unsafe fn setActionIdentifiers_(&self, actionIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActionIdentifiers : actionIdentifiers)
    }
    unsafe fn sharedItemContentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharedItemContentType)
    }
    unsafe fn setSharedItemContentType_(&self, sharedItemContentType: UTType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharedItemContentType : sharedItemContentType)
    }
}
impl CSSearchableItemAttributeSet_CSContainment for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSContainment: Sized + std::ops::Deref {
    unsafe fn containerTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerTitle)
    }
    unsafe fn setContainerTitle_(&self, containerTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainerTitle : containerTitle)
    }
    unsafe fn containerDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerDisplayName)
    }
    unsafe fn setContainerDisplayName_(&self, containerDisplayName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainerDisplayName : containerDisplayName)
    }
    unsafe fn containerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerIdentifier)
    }
    unsafe fn setContainerIdentifier_(&self, containerIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainerIdentifier : containerIdentifier)
    }
    unsafe fn containerOrder(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerOrder)
    }
    unsafe fn setContainerOrder_(&self, containerOrder: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainerOrder : containerOrder)
    }
}
impl CSSearchableItemAttributeSet_CSItemProvider for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSItemProvider: Sized + std::ops::Deref {
    unsafe fn providerDataTypeIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerDataTypeIdentifiers)
    }
    unsafe fn setProviderDataTypeIdentifiers_(&self, providerDataTypeIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderDataTypeIdentifiers : providerDataTypeIdentifiers)
    }
    unsafe fn providerFileTypeIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerFileTypeIdentifiers)
    }
    unsafe fn setProviderFileTypeIdentifiers_(&self, providerFileTypeIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderFileTypeIdentifiers : providerFileTypeIdentifiers)
    }
    unsafe fn providerInPlaceFileTypeIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerInPlaceFileTypeIdentifiers)
    }
    unsafe fn setProviderInPlaceFileTypeIdentifiers_(
        &self,
        providerInPlaceFileTypeIdentifiers: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderInPlaceFileTypeIdentifiers : providerInPlaceFileTypeIdentifiers)
    }
}
impl CSSearchableItemAttributeSet_CSDocuments for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSDocuments: Sized + std::ops::Deref {
    unsafe fn moveFrom_(&self, sourceAttributeSet: CSSearchableItemAttributeSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveFrom : sourceAttributeSet)
    }
    unsafe fn subject(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subject)
    }
    unsafe fn setSubject_(&self, subject: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubject : subject)
    }
    unsafe fn theme(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, theme)
    }
    unsafe fn setTheme_(&self, theme: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTheme : theme)
    }
    unsafe fn contentDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentDescription)
    }
    unsafe fn setContentDescription_(&self, contentDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentDescription : contentDescription)
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
    unsafe fn audiences(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audiences)
    }
    unsafe fn setAudiences_(&self, audiences: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudiences : audiences)
    }
    unsafe fn fileSize(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSize)
    }
    unsafe fn setFileSize_(&self, fileSize: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFileSize : fileSize)
    }
    unsafe fn pageCount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageCount)
    }
    unsafe fn setPageCount_(&self, pageCount: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageCount : pageCount)
    }
    unsafe fn pageWidth(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageWidth)
    }
    unsafe fn setPageWidth_(&self, pageWidth: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageWidth : pageWidth)
    }
    unsafe fn pageHeight(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageHeight)
    }
    unsafe fn setPageHeight_(&self, pageHeight: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageHeight : pageHeight)
    }
    unsafe fn securityMethod(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, securityMethod)
    }
    unsafe fn setSecurityMethod_(&self, securityMethod: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSecurityMethod : securityMethod)
    }
    unsafe fn creator(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creator)
    }
    unsafe fn setCreator_(&self, creator: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCreator : creator)
    }
    unsafe fn encodingApplications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encodingApplications)
    }
    unsafe fn setEncodingApplications_(&self, encodingApplications: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEncodingApplications : encodingApplications)
    }
    unsafe fn kind(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kind)
    }
    unsafe fn setKind_(&self, kind: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKind : kind)
    }
    unsafe fn fontNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontNames)
    }
    unsafe fn setFontNames_(&self, fontNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontNames : fontNames)
    }
}
impl CSSearchableItemAttributeSet_CSEvents for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSEvents: Sized + std::ops::Deref {
    unsafe fn dueDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dueDate)
    }
    unsafe fn setDueDate_(&self, dueDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDueDate : dueDate)
    }
    unsafe fn completionDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionDate)
    }
    unsafe fn setCompletionDate_(&self, completionDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionDate : completionDate)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn setStartDate_(&self, startDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartDate : startDate)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn setEndDate_(&self, endDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndDate : endDate)
    }
    unsafe fn importantDates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, importantDates)
    }
    unsafe fn setImportantDates_(&self, importantDates: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImportantDates : importantDates)
    }
    unsafe fn allDay(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allDay)
    }
    unsafe fn setAllDay_(&self, allDay: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllDay : allDay)
    }
}
impl CSSearchableItemAttributeSet_CSMessaging for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSMessaging: Sized + std::ops::Deref {
    unsafe fn accountIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountIdentifier)
    }
    unsafe fn setAccountIdentifier_(&self, accountIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountIdentifier : accountIdentifier)
    }
    unsafe fn accountHandles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountHandles)
    }
    unsafe fn setAccountHandles_(&self, accountHandles: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountHandles : accountHandles)
    }
    unsafe fn HTMLContentData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, HTMLContentData)
    }
    unsafe fn setHTMLContentData_(&self, HTMLContentData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHTMLContentData : HTMLContentData)
    }
    unsafe fn textContent(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textContent)
    }
    unsafe fn setTextContent_(&self, textContent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextContent : textContent)
    }
    unsafe fn authors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authors)
    }
    unsafe fn setAuthors_(&self, authors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthors : authors)
    }
    unsafe fn primaryRecipients(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryRecipients)
    }
    unsafe fn setPrimaryRecipients_(&self, primaryRecipients: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimaryRecipients : primaryRecipients)
    }
    unsafe fn additionalRecipients(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalRecipients)
    }
    unsafe fn setAdditionalRecipients_(&self, additionalRecipients: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalRecipients : additionalRecipients)
    }
    unsafe fn hiddenAdditionalRecipients(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hiddenAdditionalRecipients)
    }
    unsafe fn setHiddenAdditionalRecipients_(&self, hiddenAdditionalRecipients: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHiddenAdditionalRecipients : hiddenAdditionalRecipients)
    }
    unsafe fn emailHeaders(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailHeaders)
    }
    unsafe fn setEmailHeaders_(&self, emailHeaders: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmailHeaders : emailHeaders)
    }
    unsafe fn mailboxIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mailboxIdentifiers)
    }
    unsafe fn setMailboxIdentifiers_(&self, mailboxIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMailboxIdentifiers : mailboxIdentifiers)
    }
    unsafe fn authorNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorNames)
    }
    unsafe fn setAuthorNames_(&self, authorNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorNames : authorNames)
    }
    unsafe fn recipientNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipientNames)
    }
    unsafe fn setRecipientNames_(&self, recipientNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecipientNames : recipientNames)
    }
    unsafe fn authorEmailAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorEmailAddresses)
    }
    unsafe fn setAuthorEmailAddresses_(&self, authorEmailAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorEmailAddresses : authorEmailAddresses)
    }
    unsafe fn recipientEmailAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipientEmailAddresses)
    }
    unsafe fn setRecipientEmailAddresses_(&self, recipientEmailAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecipientEmailAddresses : recipientEmailAddresses)
    }
    unsafe fn authorAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorAddresses)
    }
    unsafe fn setAuthorAddresses_(&self, authorAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorAddresses : authorAddresses)
    }
    unsafe fn recipientAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipientAddresses)
    }
    unsafe fn setRecipientAddresses_(&self, recipientAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecipientAddresses : recipientAddresses)
    }
    unsafe fn phoneNumbers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneNumbers)
    }
    unsafe fn setPhoneNumbers_(&self, phoneNumbers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneNumbers : phoneNumbers)
    }
    unsafe fn emailAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailAddresses)
    }
    unsafe fn setEmailAddresses_(&self, emailAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmailAddresses : emailAddresses)
    }
    unsafe fn instantMessageAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instantMessageAddresses)
    }
    unsafe fn setInstantMessageAddresses_(&self, instantMessageAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstantMessageAddresses : instantMessageAddresses)
    }
    unsafe fn isLikelyJunk(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLikelyJunk)
    }
    unsafe fn setLikelyJunk_(&self, likelyJunk: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLikelyJunk : likelyJunk)
    }
    unsafe fn isPriority(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPriority)
    }
    unsafe fn textContentSummary(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textContentSummary)
    }
    unsafe fn transcribedTextContent(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transcribedTextContent)
    }
    unsafe fn setTranscribedTextContent_(&self, transcribedTextContent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTranscribedTextContent : transcribedTextContent)
    }
}
impl CSSearchableItemAttributeSet_CSMedia for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSMedia: Sized + std::ops::Deref {
    unsafe fn editors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, editors)
    }
    unsafe fn setEditors_(&self, editors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditors : editors)
    }
    unsafe fn participants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participants)
    }
    unsafe fn setParticipants_(&self, participants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticipants : participants)
    }
    unsafe fn projects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projects)
    }
    unsafe fn setProjects_(&self, projects: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProjects : projects)
    }
    unsafe fn downloadedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadedDate)
    }
    unsafe fn setDownloadedDate_(&self, downloadedDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownloadedDate : downloadedDate)
    }
    unsafe fn contentSources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentSources)
    }
    unsafe fn setContentSources_(&self, contentSources: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentSources : contentSources)
    }
    unsafe fn comment(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, comment)
    }
    unsafe fn setComment_(&self, comment: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComment : comment)
    }
    unsafe fn copyright(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, copyright)
    }
    unsafe fn setCopyright_(&self, copyright: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCopyright : copyright)
    }
    unsafe fn lastUsedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastUsedDate)
    }
    unsafe fn setLastUsedDate_(&self, lastUsedDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLastUsedDate : lastUsedDate)
    }
    unsafe fn contentCreationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentCreationDate)
    }
    unsafe fn setContentCreationDate_(&self, contentCreationDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentCreationDate : contentCreationDate)
    }
    unsafe fn contentModificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentModificationDate)
    }
    unsafe fn setContentModificationDate_(&self, contentModificationDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentModificationDate : contentModificationDate)
    }
    unsafe fn addedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addedDate)
    }
    unsafe fn setAddedDate_(&self, addedDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddedDate : addedDate)
    }
    unsafe fn duration(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn setDuration_(&self, duration: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuration : duration)
    }
    unsafe fn contactKeywords(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactKeywords)
    }
    unsafe fn setContactKeywords_(&self, contactKeywords: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactKeywords : contactKeywords)
    }
    unsafe fn codecs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, codecs)
    }
    unsafe fn setCodecs_(&self, codecs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCodecs : codecs)
    }
    unsafe fn mediaTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaTypes)
    }
    unsafe fn setMediaTypes_(&self, mediaTypes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaTypes : mediaTypes)
    }
    unsafe fn isStreamable(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStreamable)
    }
    unsafe fn setStreamable_(&self, streamable: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStreamable : streamable)
    }
    unsafe fn totalBitRate(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalBitRate)
    }
    unsafe fn setTotalBitRate_(&self, totalBitRate: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTotalBitRate : totalBitRate)
    }
    unsafe fn videoBitRate(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoBitRate)
    }
    unsafe fn setVideoBitRate_(&self, videoBitRate: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoBitRate : videoBitRate)
    }
    unsafe fn audioBitRate(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioBitRate)
    }
    unsafe fn setAudioBitRate_(&self, audioBitRate: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioBitRate : audioBitRate)
    }
    unsafe fn deliveryType(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deliveryType)
    }
    unsafe fn setDeliveryType_(&self, deliveryType: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeliveryType : deliveryType)
    }
    unsafe fn organizations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, organizations)
    }
    unsafe fn setOrganizations_(&self, organizations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrganizations : organizations)
    }
    unsafe fn role(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, role)
    }
    unsafe fn setRole_(&self, role: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRole : role)
    }
    unsafe fn languages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languages)
    }
    unsafe fn setLanguages_(&self, languages: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguages : languages)
    }
    unsafe fn rights(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rights)
    }
    unsafe fn setRights_(&self, rights: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRights : rights)
    }
    unsafe fn publishers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publishers)
    }
    unsafe fn setPublishers_(&self, publishers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPublishers : publishers)
    }
    unsafe fn contributors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contributors)
    }
    unsafe fn setContributors_(&self, contributors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContributors : contributors)
    }
    unsafe fn coverage(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coverage)
    }
    unsafe fn setCoverage_(&self, coverage: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCoverage : coverage)
    }
    unsafe fn rating(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rating)
    }
    unsafe fn setRating_(&self, rating: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRating : rating)
    }
    unsafe fn ratingDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ratingDescription)
    }
    unsafe fn setRatingDescription_(&self, ratingDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRatingDescription : ratingDescription)
    }
    unsafe fn playCount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playCount)
    }
    unsafe fn setPlayCount_(&self, playCount: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayCount : playCount)
    }
    unsafe fn information(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, information)
    }
    unsafe fn setInformation_(&self, information: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInformation : information)
    }
    unsafe fn director(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, director)
    }
    unsafe fn setDirector_(&self, director: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDirector : director)
    }
    unsafe fn producer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, producer)
    }
    unsafe fn setProducer_(&self, producer: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProducer : producer)
    }
    unsafe fn genre(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, genre)
    }
    unsafe fn setGenre_(&self, genre: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGenre : genre)
    }
    unsafe fn performers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, performers)
    }
    unsafe fn setPerformers_(&self, performers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerformers : performers)
    }
    unsafe fn originalFormat(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalFormat)
    }
    unsafe fn setOriginalFormat_(&self, originalFormat: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOriginalFormat : originalFormat)
    }
    unsafe fn originalSource(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalSource)
    }
    unsafe fn setOriginalSource_(&self, originalSource: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOriginalSource : originalSource)
    }
    unsafe fn isLocal(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocal)
    }
    unsafe fn setLocal_(&self, local: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocal : local)
    }
    unsafe fn contentRating(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentRating)
    }
    unsafe fn setContentRating_(&self, contentRating: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentRating : contentRating)
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
}
impl CSSearchableItemAttributeSet_CSMusic for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSMusic: Sized + std::ops::Deref {
    unsafe fn audioSampleRate(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioSampleRate)
    }
    unsafe fn setAudioSampleRate_(&self, audioSampleRate: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioSampleRate : audioSampleRate)
    }
    unsafe fn audioChannelCount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioChannelCount)
    }
    unsafe fn setAudioChannelCount_(&self, audioChannelCount: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioChannelCount : audioChannelCount)
    }
    unsafe fn tempo(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tempo)
    }
    unsafe fn setTempo_(&self, tempo: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTempo : tempo)
    }
    unsafe fn keySignature(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keySignature)
    }
    unsafe fn setKeySignature_(&self, keySignature: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeySignature : keySignature)
    }
    unsafe fn timeSignature(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeSignature)
    }
    unsafe fn setTimeSignature_(&self, timeSignature: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeSignature : timeSignature)
    }
    unsafe fn audioEncodingApplication(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioEncodingApplication)
    }
    unsafe fn setAudioEncodingApplication_(&self, audioEncodingApplication: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioEncodingApplication : audioEncodingApplication)
    }
    unsafe fn composer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composer)
    }
    unsafe fn setComposer_(&self, composer: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComposer : composer)
    }
    unsafe fn lyricist(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lyricist)
    }
    unsafe fn setLyricist_(&self, lyricist: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLyricist : lyricist)
    }
    unsafe fn album(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, album)
    }
    unsafe fn setAlbum_(&self, album: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlbum : album)
    }
    unsafe fn artist(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artist)
    }
    unsafe fn setArtist_(&self, artist: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArtist : artist)
    }
    unsafe fn audioTrackNumber(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioTrackNumber)
    }
    unsafe fn setAudioTrackNumber_(&self, audioTrackNumber: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioTrackNumber : audioTrackNumber)
    }
    unsafe fn recordingDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordingDate)
    }
    unsafe fn setRecordingDate_(&self, recordingDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordingDate : recordingDate)
    }
    unsafe fn musicalGenre(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, musicalGenre)
    }
    unsafe fn setMusicalGenre_(&self, musicalGenre: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMusicalGenre : musicalGenre)
    }
    unsafe fn isGeneralMIDISequence(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGeneralMIDISequence)
    }
    unsafe fn setGeneralMIDISequence_(&self, generalMIDISequence: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeneralMIDISequence : generalMIDISequence)
    }
    unsafe fn musicalInstrumentCategory(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, musicalInstrumentCategory)
    }
    unsafe fn setMusicalInstrumentCategory_(&self, musicalInstrumentCategory: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMusicalInstrumentCategory : musicalInstrumentCategory)
    }
    unsafe fn musicalInstrumentName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, musicalInstrumentName)
    }
    unsafe fn setMusicalInstrumentName_(&self, musicalInstrumentName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMusicalInstrumentName : musicalInstrumentName)
    }
}
impl CSSearchableItemAttributeSet_CSImages for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSImages: Sized + std::ops::Deref {
    unsafe fn pixelHeight(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelHeight)
    }
    unsafe fn setPixelHeight_(&self, pixelHeight: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelHeight : pixelHeight)
    }
    unsafe fn pixelWidth(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelWidth)
    }
    unsafe fn setPixelWidth_(&self, pixelWidth: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelWidth : pixelWidth)
    }
    unsafe fn pixelCount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelCount)
    }
    unsafe fn setPixelCount_(&self, pixelCount: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelCount : pixelCount)
    }
    unsafe fn colorSpace(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
    unsafe fn bitsPerSample(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bitsPerSample)
    }
    unsafe fn setBitsPerSample_(&self, bitsPerSample: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBitsPerSample : bitsPerSample)
    }
    unsafe fn isFlashOn(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFlashOn)
    }
    unsafe fn setFlashOn_(&self, flashOn: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlashOn : flashOn)
    }
    unsafe fn focalLength(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalLength)
    }
    unsafe fn setFocalLength_(&self, focalLength: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalLength : focalLength)
    }
    unsafe fn isFocalLength35mm(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFocalLength35mm)
    }
    unsafe fn setFocalLength35mm_(&self, focalLength35mm: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalLength35mm : focalLength35mm)
    }
    unsafe fn acquisitionMake(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acquisitionMake)
    }
    unsafe fn setAcquisitionMake_(&self, acquisitionMake: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcquisitionMake : acquisitionMake)
    }
    unsafe fn acquisitionModel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acquisitionModel)
    }
    unsafe fn setAcquisitionModel_(&self, acquisitionModel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcquisitionModel : acquisitionModel)
    }
    unsafe fn cameraOwner(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraOwner)
    }
    unsafe fn setCameraOwner_(&self, cameraOwner: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraOwner : cameraOwner)
    }
    unsafe fn lensModel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lensModel)
    }
    unsafe fn setLensModel_(&self, lensModel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLensModel : lensModel)
    }
    unsafe fn ISOSpeed(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ISOSpeed)
    }
    unsafe fn setISOSpeed_(&self, ISOSpeed: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setISOSpeed : ISOSpeed)
    }
    unsafe fn orientation(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn setOrientation_(&self, orientation: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientation : orientation)
    }
    unsafe fn layerNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerNames)
    }
    unsafe fn setLayerNames_(&self, layerNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayerNames : layerNames)
    }
    unsafe fn whiteBalance(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, whiteBalance)
    }
    unsafe fn setWhiteBalance_(&self, whiteBalance: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWhiteBalance : whiteBalance)
    }
    unsafe fn aperture(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aperture)
    }
    unsafe fn setAperture_(&self, aperture: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAperture : aperture)
    }
    unsafe fn profileName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileName)
    }
    unsafe fn setProfileName_(&self, profileName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileName : profileName)
    }
    unsafe fn resolutionWidthDPI(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolutionWidthDPI)
    }
    unsafe fn setResolutionWidthDPI_(&self, resolutionWidthDPI: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResolutionWidthDPI : resolutionWidthDPI)
    }
    unsafe fn resolutionHeightDPI(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolutionHeightDPI)
    }
    unsafe fn setResolutionHeightDPI_(&self, resolutionHeightDPI: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResolutionHeightDPI : resolutionHeightDPI)
    }
    unsafe fn exposureMode(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureMode)
    }
    unsafe fn setExposureMode_(&self, exposureMode: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureMode : exposureMode)
    }
    unsafe fn exposureTime(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureTime)
    }
    unsafe fn setExposureTime_(&self, exposureTime: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureTime : exposureTime)
    }
    unsafe fn EXIFVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, EXIFVersion)
    }
    unsafe fn setEXIFVersion_(&self, EXIFVersion: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEXIFVersion : EXIFVersion)
    }
    unsafe fn EXIFGPSVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, EXIFGPSVersion)
    }
    unsafe fn setEXIFGPSVersion_(&self, EXIFGPSVersion: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEXIFGPSVersion : EXIFGPSVersion)
    }
    unsafe fn hasAlphaChannel(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAlphaChannel)
    }
    unsafe fn setHasAlphaChannel_(&self, hasAlphaChannel: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasAlphaChannel : hasAlphaChannel)
    }
    unsafe fn isRedEyeOn(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRedEyeOn)
    }
    unsafe fn setRedEyeOn_(&self, redEyeOn: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRedEyeOn : redEyeOn)
    }
    unsafe fn meteringMode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meteringMode)
    }
    unsafe fn setMeteringMode_(&self, meteringMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeteringMode : meteringMode)
    }
    unsafe fn maxAperture(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxAperture)
    }
    unsafe fn setMaxAperture_(&self, maxAperture: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxAperture : maxAperture)
    }
    unsafe fn fNumber(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fNumber)
    }
    unsafe fn setFNumber_(&self, fNumber: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFNumber : fNumber)
    }
    unsafe fn exposureProgram(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureProgram)
    }
    unsafe fn setExposureProgram_(&self, exposureProgram: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureProgram : exposureProgram)
    }
    unsafe fn exposureTimeString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureTimeString)
    }
    unsafe fn setExposureTimeString_(&self, exposureTimeString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureTimeString : exposureTimeString)
    }
}
impl CSSearchableItemAttributeSet_CSPlaces for CSSearchableItemAttributeSet {}
pub trait CSSearchableItemAttributeSet_CSPlaces: Sized + std::ops::Deref {
    unsafe fn headline(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headline)
    }
    unsafe fn setHeadline_(&self, headline: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeadline : headline)
    }
    unsafe fn instructions(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instructions)
    }
    unsafe fn setInstructions_(&self, instructions: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstructions : instructions)
    }
    unsafe fn thoroughfare(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thoroughfare)
    }
    unsafe fn setThoroughfare_(&self, thoroughfare: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThoroughfare : thoroughfare)
    }
    unsafe fn subThoroughfare(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subThoroughfare)
    }
    unsafe fn setSubThoroughfare_(&self, subThoroughfare: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubThoroughfare : subThoroughfare)
    }
    unsafe fn postalCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postalCode)
    }
    unsafe fn setPostalCode_(&self, postalCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPostalCode : postalCode)
    }
    unsafe fn city(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, city)
    }
    unsafe fn setCity_(&self, city: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCity : city)
    }
    unsafe fn stateOrProvince(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateOrProvince)
    }
    unsafe fn setStateOrProvince_(&self, stateOrProvince: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStateOrProvince : stateOrProvince)
    }
    unsafe fn country(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, country)
    }
    unsafe fn setCountry_(&self, country: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCountry : country)
    }
    unsafe fn fullyFormattedAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullyFormattedAddress)
    }
    unsafe fn setFullyFormattedAddress_(&self, fullyFormattedAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFullyFormattedAddress : fullyFormattedAddress)
    }
    unsafe fn altitude(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altitude)
    }
    unsafe fn setAltitude_(&self, altitude: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAltitude : altitude)
    }
    unsafe fn latitude(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, latitude)
    }
    unsafe fn setLatitude_(&self, latitude: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLatitude : latitude)
    }
    unsafe fn longitude(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longitude)
    }
    unsafe fn setLongitude_(&self, longitude: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLongitude : longitude)
    }
    unsafe fn speed(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn setSpeed_(&self, speed: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed)
    }
    unsafe fn timestamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn setTimestamp_(&self, timestamp: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimestamp : timestamp)
    }
    unsafe fn imageDirection(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageDirection)
    }
    unsafe fn setImageDirection_(&self, imageDirection: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageDirection : imageDirection)
    }
    unsafe fn namedLocation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, namedLocation)
    }
    unsafe fn setNamedLocation_(&self, namedLocation: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNamedLocation : namedLocation)
    }
    unsafe fn GPSTrack(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSTrack)
    }
    unsafe fn setGPSTrack_(&self, GPSTrack: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSTrack : GPSTrack)
    }
    unsafe fn GPSStatus(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSStatus)
    }
    unsafe fn setGPSStatus_(&self, GPSStatus: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSStatus : GPSStatus)
    }
    unsafe fn GPSMeasureMode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSMeasureMode)
    }
    unsafe fn setGPSMeasureMode_(&self, GPSMeasureMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSMeasureMode : GPSMeasureMode)
    }
    unsafe fn GPSDOP(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSDOP)
    }
    unsafe fn setGPSDOP_(&self, GPSDOP: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSDOP : GPSDOP)
    }
    unsafe fn GPSMapDatum(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSMapDatum)
    }
    unsafe fn setGPSMapDatum_(&self, GPSMapDatum: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSMapDatum : GPSMapDatum)
    }
    unsafe fn GPSDestLatitude(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSDestLatitude)
    }
    unsafe fn setGPSDestLatitude_(&self, GPSDestLatitude: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSDestLatitude : GPSDestLatitude)
    }
    unsafe fn GPSDestLongitude(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSDestLongitude)
    }
    unsafe fn setGPSDestLongitude_(&self, GPSDestLongitude: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSDestLongitude : GPSDestLongitude)
    }
    unsafe fn GPSDestBearing(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSDestBearing)
    }
    unsafe fn setGPSDestBearing_(&self, GPSDestBearing: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSDestBearing : GPSDestBearing)
    }
    unsafe fn GPSDestDistance(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSDestDistance)
    }
    unsafe fn setGPSDestDistance_(&self, GPSDestDistance: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSDestDistance : GPSDestDistance)
    }
    unsafe fn GPSProcessingMethod(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSProcessingMethod)
    }
    unsafe fn setGPSProcessingMethod_(&self, GPSProcessingMethod: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSProcessingMethod : GPSProcessingMethod)
    }
    unsafe fn GPSAreaInformation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSAreaInformation)
    }
    unsafe fn setGPSAreaInformation_(&self, GPSAreaInformation: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSAreaInformation : GPSAreaInformation)
    }
    unsafe fn GPSDateStamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSDateStamp)
    }
    unsafe fn setGPSDateStamp_(&self, GPSDateStamp: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSDateStamp : GPSDateStamp)
    }
    unsafe fn GPSDifferental(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPSDifferental)
    }
    unsafe fn setGPSDifferental_(&self, GPSDifferental: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPSDifferental : GPSDifferental)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSImportExtension(pub id);
impl std::ops::Deref for CSImportExtension {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSImportExtension {}
impl CSImportExtension {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSImportExtension").unwrap(), alloc) })
    }
}
impl PNSExtensionRequestHandling for CSImportExtension {}
impl INSObject for CSImportExtension {}
impl PNSObject for CSImportExtension {}
impl std::convert::TryFrom<NSObject> for CSImportExtension {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSImportExtension, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSImportExtension").unwrap()) };
        if is_kind_of {
            Ok(CSImportExtension(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSImportExtension")
        }
    }
}
impl ICSImportExtension for CSImportExtension {}
pub trait ICSImportExtension: Sized + std::ops::Deref {
    unsafe fn updateAttributes_forFileAtURL_error_(
        &self,
        attributes: CSSearchableItemAttributeSet,
        contentURL: NSURL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAttributes : attributes, forFileAtURL : contentURL, error : error)
    }
}
pub type CSSearchableItemUpdateListenerOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSSearchableItem(pub id);
impl std::ops::Deref for CSSearchableItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSSearchableItem {}
impl CSSearchableItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSSearchableItem").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CSSearchableItem {}
impl PNSCopying for CSSearchableItem {}
impl INSObject for CSSearchableItem {}
impl PNSObject for CSSearchableItem {}
impl std::convert::TryFrom<NSObject> for CSSearchableItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSSearchableItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSSearchableItem").unwrap()) };
        if is_kind_of {
            Ok(CSSearchableItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSSearchableItem")
        }
    }
}
impl ICSSearchableItem for CSSearchableItem {}
pub trait ICSSearchableItem: Sized + std::ops::Deref {
    unsafe fn initWithUniqueIdentifier_domainIdentifier_attributeSet_(
        &self,
        uniqueIdentifier: NSString,
        domainIdentifier: NSString,
        attributeSet: CSSearchableItemAttributeSet,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUniqueIdentifier : uniqueIdentifier, domainIdentifier : domainIdentifier, attributeSet : attributeSet)
    }
    unsafe fn compareByRank_(&self, other: CSSearchableItem) -> NSComparisonResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareByRank : other)
    }
    unsafe fn uniqueIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn setUniqueIdentifier_(&self, uniqueIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUniqueIdentifier : uniqueIdentifier)
    }
    unsafe fn domainIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainIdentifier)
    }
    unsafe fn setDomainIdentifier_(&self, domainIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDomainIdentifier : domainIdentifier)
    }
    unsafe fn expirationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expirationDate)
    }
    unsafe fn setExpirationDate_(&self, expirationDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExpirationDate : expirationDate)
    }
    unsafe fn attributeSet(&self) -> CSSearchableItemAttributeSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeSet)
    }
    unsafe fn setAttributeSet_(&self, attributeSet: CSSearchableItemAttributeSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeSet : attributeSet)
    }
    unsafe fn isUpdate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUpdate)
    }
    unsafe fn setIsUpdate_(&self, isUpdate: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsUpdate : isUpdate)
    }
    unsafe fn updateListenerOptions(&self) -> CSSearchableItemUpdateListenerOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateListenerOptions)
    }
    unsafe fn setUpdateListenerOptions_(
        &self,
        updateListenerOptions: CSSearchableItemUpdateListenerOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpdateListenerOptions : updateListenerOptions)
    }
}
pub type CSIndexErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSSearchableIndex(pub id);
impl std::ops::Deref for CSSearchableIndex {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSSearchableIndex {}
impl CSSearchableIndex {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSSearchableIndex").unwrap(), alloc) })
    }
}
impl INSObject for CSSearchableIndex {}
impl PNSObject for CSSearchableIndex {}
impl std::convert::TryFrom<NSObject> for CSSearchableIndex {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSSearchableIndex, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSSearchableIndex").unwrap()) };
        if is_kind_of {
            Ok(CSSearchableIndex(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSSearchableIndex")
        }
    }
}
impl ICSSearchableIndex for CSSearchableIndex {}
pub trait ICSSearchableIndex: Sized + std::ops::Deref {
    unsafe fn initWithName_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name)
    }
    unsafe fn initWithName_protectionClass_(
        &self,
        name: NSString,
        protectionClass: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, protectionClass : protectionClass)
    }
    unsafe fn indexSearchableItems_completionHandler_(
        &self,
        items: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexSearchableItems : items, completionHandler : completionHandler)
    }
    unsafe fn deleteSearchableItemsWithIdentifiers_completionHandler_(
        &self,
        identifiers: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteSearchableItemsWithIdentifiers : identifiers, completionHandler : completionHandler)
    }
    unsafe fn deleteSearchableItemsWithDomainIdentifiers_completionHandler_(
        &self,
        domainIdentifiers: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteSearchableItemsWithDomainIdentifiers : domainIdentifiers, completionHandler : completionHandler)
    }
    unsafe fn deleteAllSearchableItemsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteAllSearchableItemsWithCompletionHandler : completionHandler)
    }
    unsafe fn indexDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexDelegate)
    }
    unsafe fn setIndexDelegate_(&self, indexDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexDelegate : indexDelegate)
    }
    unsafe fn isIndexingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CSSearchableIndex").unwrap(), isIndexingAvailable)
    }
    unsafe fn defaultSearchableIndex() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CSSearchableIndex").unwrap(), defaultSearchableIndex)
    }
}
impl CSSearchableIndex_CSOptionalBatching for CSSearchableIndex {}
pub trait CSSearchableIndex_CSOptionalBatching: Sized + std::ops::Deref {
    unsafe fn beginIndexBatch(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginIndexBatch)
    }
    unsafe fn endIndexBatchWithExpectedClientState_newClientState_completionHandler_(
        &self,
        expectedClientState: NSData,
        newClientState: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endIndexBatchWithExpectedClientState : expectedClientState, newClientState : newClientState, completionHandler : completionHandler)
    }
    unsafe fn endIndexBatchWithClientState_completionHandler_(
        &self,
        clientState: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endIndexBatchWithClientState : clientState, completionHandler : completionHandler)
    }
    unsafe fn fetchLastClientStateWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchLastClientStateWithCompletionHandler : completionHandler)
    }
}
impl CSSearchableIndex_CSExternalProvider for CSSearchableIndex {}
pub trait CSSearchableIndex_CSExternalProvider: Sized + std::ops::Deref {
    unsafe fn fetchDataForBundleIdentifier_itemIdentifier_contentType_completionHandler_(
        &self,
        bundleIdentifier: NSString,
        itemIdentifier: NSString,
        contentType: UTType,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchDataForBundleIdentifier : bundleIdentifier, itemIdentifier : itemIdentifier, contentType : contentType, completionHandler : completionHandler)
    }
}
impl CSSearchableIndex_CSOptionalBatchingWithExpectedState for CSSearchableIndex {}
pub trait CSSearchableIndex_CSOptionalBatchingWithExpectedState: Sized + std::ops::Deref {}
pub trait PCSSearchableIndexDelegate: Sized + std::ops::Deref {
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
    unsafe fn searchableIndexDidThrottle_(&self, searchableIndex: CSSearchableIndex)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchableIndexDidThrottle : searchableIndex)
    }
    unsafe fn searchableIndexDidFinishThrottle_(&self, searchableIndex: CSSearchableIndex)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchableIndexDidFinishThrottle : searchableIndex)
    }
    unsafe fn dataForSearchableIndex_itemIdentifier_typeIdentifier_error_(
        &self,
        searchableIndex: CSSearchableIndex,
        itemIdentifier: NSString,
        typeIdentifier: NSString,
        outError: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataForSearchableIndex : searchableIndex, itemIdentifier : itemIdentifier, typeIdentifier : typeIdentifier, error : outError)
    }
    unsafe fn fileURLForSearchableIndex_itemIdentifier_typeIdentifier_inPlace_error_(
        &self,
        searchableIndex: CSSearchableIndex,
        itemIdentifier: NSString,
        typeIdentifier: NSString,
        inPlace: BOOL,
        outError: *mut NSError,
    ) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileURLForSearchableIndex : searchableIndex, itemIdentifier : itemIdentifier, typeIdentifier : typeIdentifier, inPlace : inPlace, error : outError)
    }
    unsafe fn searchableItemsForIdentifiers_searchableItemsHandler_(
        &self,
        identifiers: NSArray,
        searchableItemsHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchableItemsForIdentifiers : identifiers, searchableItemsHandler : searchableItemsHandler)
    }
    unsafe fn searchableItemsDidUpdate_(&self, items: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchableItemsDidUpdate : items)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSIndexExtensionRequestHandler(pub id);
impl std::ops::Deref for CSIndexExtensionRequestHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSIndexExtensionRequestHandler {}
impl CSIndexExtensionRequestHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSIndexExtensionRequestHandler").unwrap(), alloc) })
    }
}
impl PNSExtensionRequestHandling for CSIndexExtensionRequestHandler {}
impl PCSSearchableIndexDelegate for CSIndexExtensionRequestHandler {}
impl INSObject for CSIndexExtensionRequestHandler {}
impl PNSObject for CSIndexExtensionRequestHandler {}
impl std::convert::TryFrom<NSObject> for CSIndexExtensionRequestHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSIndexExtensionRequestHandler, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSIndexExtensionRequestHandler").unwrap())
        };
        if is_kind_of {
            Ok(CSIndexExtensionRequestHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSIndexExtensionRequestHandler")
        }
    }
}
impl ICSIndexExtensionRequestHandler for CSIndexExtensionRequestHandler {}
pub trait ICSIndexExtensionRequestHandler: Sized + std::ops::Deref {}
pub type CSSearchQueryErrorCode = NSInteger;
pub type CSSearchQuerySourceOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSSearchQueryContext(pub id);
impl std::ops::Deref for CSSearchQueryContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSSearchQueryContext {}
impl CSSearchQueryContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSSearchQueryContext").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CSSearchQueryContext {}
impl PNSCopying for CSSearchQueryContext {}
impl INSObject for CSSearchQueryContext {}
impl PNSObject for CSSearchQueryContext {}
impl std::convert::TryFrom<NSObject> for CSSearchQueryContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSSearchQueryContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSSearchQueryContext").unwrap()) };
        if is_kind_of {
            Ok(CSSearchQueryContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSSearchQueryContext")
        }
    }
}
impl ICSSearchQueryContext for CSSearchQueryContext {}
pub trait ICSSearchQueryContext: Sized + std::ops::Deref {
    unsafe fn fetchAttributes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchAttributes)
    }
    unsafe fn setFetchAttributes_(&self, fetchAttributes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchAttributes : fetchAttributes)
    }
    unsafe fn filterQueries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterQueries)
    }
    unsafe fn setFilterQueries_(&self, filterQueries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterQueries : filterQueries)
    }
    unsafe fn keyboardLanguage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyboardLanguage)
    }
    unsafe fn setKeyboardLanguage_(&self, keyboardLanguage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyboardLanguage : keyboardLanguage)
    }
    unsafe fn sourceOptions(&self) -> CSSearchQuerySourceOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceOptions)
    }
    unsafe fn setSourceOptions_(&self, sourceOptions: CSSearchQuerySourceOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceOptions : sourceOptions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSSearchQuery(pub id);
impl std::ops::Deref for CSSearchQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSSearchQuery {}
impl CSSearchQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSSearchQuery").unwrap(), alloc) })
    }
}
impl INSObject for CSSearchQuery {}
impl PNSObject for CSSearchQuery {}
impl std::convert::TryFrom<NSObject> for CSSearchQuery {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSSearchQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSSearchQuery").unwrap()) };
        if is_kind_of {
            Ok(CSSearchQuery(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSSearchQuery")
        }
    }
}
impl ICSSearchQuery for CSSearchQuery {}
pub trait ICSSearchQuery: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithQueryString_queryContext_(
        &self,
        queryString: NSString,
        queryContext: CSSearchQueryContext,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQueryString : queryString, queryContext : queryContext)
    }
    unsafe fn initWithQueryString_attributes_(
        &self,
        queryString: NSString,
        attributes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQueryString : queryString, attributes : attributes)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isCancelled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCancelled)
    }
    unsafe fn foundItemCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foundItemCount)
    }
    unsafe fn foundItemsHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foundItemsHandler)
    }
    unsafe fn setFoundItemsHandler_(&self, foundItemsHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFoundItemsHandler : foundItemsHandler)
    }
    unsafe fn completionHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionHandler)
    }
    unsafe fn setCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionHandler : completionHandler)
    }
    unsafe fn protectionClasses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protectionClasses)
    }
    unsafe fn setProtectionClasses_(&self, protectionClasses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProtectionClasses : protectionClasses)
    }
}
pub type CSSuggestionKind = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSSuggestion(pub id);
impl std::ops::Deref for CSSuggestion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSSuggestion {}
impl CSSuggestion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSSuggestion").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CSSuggestion {}
impl PNSCopying for CSSuggestion {}
impl INSObject for CSSuggestion {}
impl PNSObject for CSSuggestion {}
impl std::convert::TryFrom<NSObject> for CSSuggestion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CSSuggestion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSSuggestion").unwrap()) };
        if is_kind_of {
            Ok(CSSuggestion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CSSuggestion")
        }
    }
}
impl ICSSuggestion for CSSuggestion {}
pub trait ICSSuggestion: Sized + std::ops::Deref {
    unsafe fn compareByRank_(&self, other: CSSuggestion) -> NSComparisonResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareByRank : other)
    }
    unsafe fn compare_(&self, other: CSSuggestion) -> NSComparisonResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compare : other)
    }
    unsafe fn localizedAttributedSuggestion(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedAttributedSuggestion)
    }
    unsafe fn suggestionKind(&self) -> CSSuggestionKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suggestionKind)
    }
}
pub type CSUserInteraction = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSUserQueryContext(pub id);
impl std::ops::Deref for CSUserQueryContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSUserQueryContext {}
impl CSUserQueryContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSUserQueryContext").unwrap(), alloc) })
    }
}
impl ICSSearchQueryContext for CSUserQueryContext {}
impl PNSSecureCoding for CSUserQueryContext {}
impl PNSCopying for CSUserQueryContext {}
impl From<CSUserQueryContext> for CSSearchQueryContext {
    fn from(child: CSUserQueryContext) -> CSSearchQueryContext {
        CSSearchQueryContext(child.0)
    }
}
impl std::convert::TryFrom<CSSearchQueryContext> for CSUserQueryContext {
    type Error = &'static str;
    fn try_from(parent: CSSearchQueryContext) -> Result<CSUserQueryContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSUserQueryContext").unwrap()) };
        if is_kind_of {
            Ok(CSUserQueryContext(parent.0))
        } else {
            Err("This CSSearchQueryContext cannot be downcasted to CSUserQueryContext")
        }
    }
}
impl INSObject for CSUserQueryContext {}
impl PNSObject for CSUserQueryContext {}
impl ICSUserQueryContext for CSUserQueryContext {}
pub trait ICSUserQueryContext: Sized + std::ops::Deref {
    unsafe fn enableRankedResults(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableRankedResults)
    }
    unsafe fn setEnableRankedResults_(&self, enableRankedResults: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableRankedResults : enableRankedResults)
    }
    unsafe fn disableSemanticSearch(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableSemanticSearch)
    }
    unsafe fn setDisableSemanticSearch_(&self, disableSemanticSearch: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisableSemanticSearch : disableSemanticSearch)
    }
    unsafe fn maxResultCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxResultCount)
    }
    unsafe fn setMaxResultCount_(&self, maxResultCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxResultCount : maxResultCount)
    }
    unsafe fn maxSuggestionCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxSuggestionCount)
    }
    unsafe fn setMaxSuggestionCount_(&self, maxSuggestionCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxSuggestionCount : maxSuggestionCount)
    }
    unsafe fn maxRankedResultCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxRankedResultCount)
    }
    unsafe fn setMaxRankedResultCount_(&self, maxRankedResultCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxRankedResultCount : maxRankedResultCount)
    }
    unsafe fn userQueryContext() -> CSUserQueryContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CSUserQueryContext").unwrap(), userQueryContext)
    }
    unsafe fn userQueryContextWithCurrentSuggestion_(
        currentSuggestion: CSSuggestion,
    ) -> CSUserQueryContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CSUserQueryContext").unwrap(), userQueryContextWithCurrentSuggestion : currentSuggestion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CSUserQuery(pub id);
impl std::ops::Deref for CSUserQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CSUserQuery {}
impl CSUserQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CSUserQuery").unwrap(), alloc) })
    }
}
impl ICSSearchQuery for CSUserQuery {}
impl From<CSUserQuery> for CSSearchQuery {
    fn from(child: CSUserQuery) -> CSSearchQuery {
        CSSearchQuery(child.0)
    }
}
impl std::convert::TryFrom<CSSearchQuery> for CSUserQuery {
    type Error = &'static str;
    fn try_from(parent: CSSearchQuery) -> Result<CSUserQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CSUserQuery").unwrap()) };
        if is_kind_of {
            Ok(CSUserQuery(parent.0))
        } else {
            Err("This CSSearchQuery cannot be downcasted to CSUserQuery")
        }
    }
}
impl INSObject for CSUserQuery {}
impl PNSObject for CSUserQuery {}
impl ICSUserQuery for CSUserQuery {}
pub trait ICSUserQuery: Sized + std::ops::Deref {
    unsafe fn initWithUserQueryString_userQueryContext_(
        &self,
        userQueryString: NSString,
        userQueryContext: CSUserQueryContext,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUserQueryString : userQueryString, userQueryContext : userQueryContext)
    }
    unsafe fn userEngagedWithItem_visibleItems_userInteractionType_(
        &self,
        item: CSSearchableItem,
        visibleItems: NSArray,
        userInteractionType: CSUserInteraction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userEngagedWithItem : item, visibleItems : visibleItems, userInteractionType : userInteractionType)
    }
    unsafe fn userEngagedWithSuggestion_visibleSuggestions_userInteractionType_(
        &self,
        suggestion: CSSuggestion,
        visibleSuggestions: NSArray,
        userInteractionType: CSUserInteraction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userEngagedWithSuggestion : suggestion, visibleSuggestions : visibleSuggestions, userInteractionType : userInteractionType)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn foundSuggestionCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foundSuggestionCount)
    }
    unsafe fn foundSuggestionsHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foundSuggestionsHandler)
    }
    unsafe fn setFoundSuggestionsHandler_(
        &self,
        foundSuggestionsHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFoundSuggestionsHandler : foundSuggestionsHandler)
    }
    unsafe fn prepare()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CSUserQuery").unwrap(), prepare)
    }
    unsafe fn prepareProtectionClasses_(protectionClasses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CSUserQuery").unwrap(), prepareProtectionClasses : protectionClasses)
    }
}
unsafe extern "C" {
    pub static mut CoreSpotlightVersionNumber: f64;
}
unsafe extern "C" {
    pub static CoreSpotlightVersionString: [::std::os::raw::c_uchar; 0usize];
}
unsafe extern "C" {
    pub static CSMailboxInbox: NSString;
}
unsafe extern "C" {
    pub static CSMailboxDrafts: NSString;
}
unsafe extern "C" {
    pub static CSMailboxSent: NSString;
}
unsafe extern "C" {
    pub static CSMailboxJunk: NSString;
}
unsafe extern "C" {
    pub static CSMailboxTrash: NSString;
}
unsafe extern "C" {
    pub static CSMailboxArchive: NSString;
}
unsafe extern "C" {
    pub static CSSearchableItemActionType: NSString;
}
unsafe extern "C" {
    pub static CSSearchableItemActivityIdentifier: NSString;
}
unsafe extern "C" {
    pub static CSActionIdentifier: NSString;
}
unsafe extern "C" {
    pub static CSQueryContinuationActionType: NSString;
}
unsafe extern "C" {
    pub static CSSearchQueryString: NSString;
}
unsafe extern "C" {
    pub static CSIndexErrorDomain: NSString;
}
unsafe extern "C" {
    pub static CSSearchQueryErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static CSSuggestionHighlightAttributeName: NSAttributedStringKey;
}

unsafe impl objc2::encode::RefEncode for CSPerson {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSPerson {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSSearchableItemAttributeSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSearchableItemAttributeSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSLocalizedString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSLocalizedString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSCustomAttributeKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSCustomAttributeKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSImportExtension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSImportExtension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSSearchableItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSearchableItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSSearchableIndex {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSearchableIndex {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSIndexExtensionRequestHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSIndexExtensionRequestHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSSearchQueryContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSearchQueryContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSSearchQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSearchQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSSuggestion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSuggestion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSUserQueryContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSUserQueryContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CSUserQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSUserQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
