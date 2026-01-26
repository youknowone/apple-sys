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
pub type NSFileProviderItemIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderItemVersion(pub id);
impl std::ops::Deref for NSFileProviderItemVersion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderItemVersion {}
impl NSFileProviderItemVersion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderItemVersion").unwrap(), alloc) })
    }
}
impl INSObject for NSFileProviderItemVersion {}
impl PNSObject for NSFileProviderItemVersion {}
impl std::convert::TryFrom<NSObject> for NSFileProviderItemVersion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderItemVersion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderItemVersion").unwrap()) };
        if is_kind_of {
            Ok(NSFileProviderItemVersion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderItemVersion")
        }
    }
}
impl INSFileProviderItemVersion for NSFileProviderItemVersion {}
pub trait INSFileProviderItemVersion: Sized + std::ops::Deref {
    unsafe fn initWithContentVersion_metadataVersion_(
        &self,
        contentVersion: NSData,
        metadataVersion: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentVersion : contentVersion, metadataVersion : metadataVersion)
    }
    unsafe fn contentVersion(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentVersion)
    }
    unsafe fn metadataVersion(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadataVersion)
    }
    unsafe fn beforeFirstSyncComponent() -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderItemVersion").unwrap(), beforeFirstSyncComponent)
    }
}
pub type NSFileProviderItemCapabilities = NSUInteger;
pub type NSFileProviderItemFields = NSUInteger;
pub type NSFileProviderFileSystemFlags = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderTypeAndCreator {
    pub type_: OSType,
    pub creator: OSType,
}
pub type NSFileProviderContentPolicy = NSInteger;
pub trait PNSFileProviderItem: Sized + std::ops::Deref {
    unsafe fn itemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemIdentifier)
    }
    unsafe fn parentItemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentItemIdentifier)
    }
    unsafe fn filename(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filename)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn typeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typeIdentifier)
    }
    unsafe fn typeAndCreator(&self) -> NSFileProviderTypeAndCreator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typeAndCreator)
    }
    unsafe fn capabilities(&self) -> NSFileProviderItemCapabilities
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capabilities)
    }
    unsafe fn fileSystemFlags(&self) -> NSFileProviderFileSystemFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSystemFlags)
    }
    unsafe fn documentSize(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentSize)
    }
    unsafe fn childItemCount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childItemCount)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn contentModificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentModificationDate)
    }
    unsafe fn extendedAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extendedAttributes)
    }
    unsafe fn lastUsedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastUsedDate)
    }
    unsafe fn tagData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tagData)
    }
    unsafe fn favoriteRank(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, favoriteRank)
    }
    unsafe fn isTrashed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTrashed)
    }
    unsafe fn isUploaded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUploaded)
    }
    unsafe fn isUploading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUploading)
    }
    unsafe fn uploadingError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uploadingError)
    }
    unsafe fn isDownloaded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDownloaded)
    }
    unsafe fn isDownloading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDownloading)
    }
    unsafe fn downloadingError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadingError)
    }
    unsafe fn isMostRecentVersionDownloaded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMostRecentVersionDownloaded)
    }
    unsafe fn isShared(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isShared)
    }
    unsafe fn isSharedByCurrentUser(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSharedByCurrentUser)
    }
    unsafe fn ownerNameComponents(&self) -> NSPersonNameComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerNameComponents)
    }
    unsafe fn mostRecentEditorNameComponents(&self) -> NSPersonNameComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mostRecentEditorNameComponents)
    }
    unsafe fn versionIdentifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionIdentifier)
    }
    unsafe fn itemVersion(&self) -> NSFileProviderItemVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemVersion)
    }
    unsafe fn symlinkTargetPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symlinkTargetPath)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn contentPolicy(&self) -> NSFileProviderContentPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentPolicy)
    }
}
pub type NSFileProviderItem = *mut u64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderExtension(pub id);
impl std::ops::Deref for NSFileProviderExtension {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderExtension {}
impl NSFileProviderExtension {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderExtension").unwrap(), alloc) })
    }
}
impl INSObject for NSFileProviderExtension {}
impl PNSObject for NSFileProviderExtension {}
impl std::convert::TryFrom<NSObject> for NSFileProviderExtension {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderExtension, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderExtension").unwrap()) };
        if is_kind_of {
            Ok(NSFileProviderExtension(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderExtension")
        }
    }
}
impl INSFileProviderExtension for NSFileProviderExtension {}
pub trait INSFileProviderExtension: Sized + std::ops::Deref {
    unsafe fn itemForIdentifier_error_(
        &self,
        identifier: NSString,
        error: *mut NSError,
    ) -> NSFileProviderItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemForIdentifier : identifier, error : error)
    }
    unsafe fn URLForItemWithPersistentIdentifier_(&self, identifier: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLForItemWithPersistentIdentifier : identifier)
    }
    unsafe fn persistentIdentifierForItemAtURL_(&self, url: NSURL) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, persistentIdentifierForItemAtURL : url)
    }
    unsafe fn providePlaceholderAtURL_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, providePlaceholderAtURL : url, completionHandler : completionHandler)
    }
    unsafe fn startProvidingItemAtURL_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startProvidingItemAtURL : url, completionHandler : completionHandler)
    }
    unsafe fn stopProvidingItemAtURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopProvidingItemAtURL : url)
    }
    unsafe fn itemChangedAtURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemChangedAtURL : url)
    }
}
impl NSFileProviderExtension_Deprecated for NSFileProviderExtension {}
pub trait NSFileProviderExtension_Deprecated: Sized + std::ops::Deref {
    unsafe fn providerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerIdentifier)
    }
    unsafe fn documentStorageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentStorageURL)
    }
    unsafe fn writePlaceholderAtURL_withMetadata_error_(
        placeholderURL: NSURL,
        metadata: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderExtension").unwrap(), writePlaceholderAtURL : placeholderURL, withMetadata : metadata, error : error)
    }
    unsafe fn placeholderURLForURL_(url: NSURL) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderExtension").unwrap(), placeholderURLForURL : url)
    }
}
pub type NSFileProviderDomainIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderDomainVersion(pub id);
impl std::ops::Deref for NSFileProviderDomainVersion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderDomainVersion {}
impl NSFileProviderDomainVersion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderDomainVersion").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NSFileProviderDomainVersion {}
impl INSObject for NSFileProviderDomainVersion {}
impl PNSObject for NSFileProviderDomainVersion {}
impl std::convert::TryFrom<NSObject> for NSFileProviderDomainVersion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderDomainVersion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderDomainVersion").unwrap()) };
        if is_kind_of {
            Ok(NSFileProviderDomainVersion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderDomainVersion")
        }
    }
}
impl INSFileProviderDomainVersion for NSFileProviderDomainVersion {}
pub trait INSFileProviderDomainVersion: Sized + std::ops::Deref {
    unsafe fn next(&self) -> NSFileProviderDomainVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, next)
    }
    unsafe fn compare_(&self, otherVersion: NSFileProviderDomainVersion) -> NSComparisonResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compare : otherVersion)
    }
}
pub type NSFileProviderDomainTestingModes = NSUInteger;
pub type NSFileProviderKnownFolders = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderDomain(pub id);
impl std::ops::Deref for NSFileProviderDomain {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderDomain {}
impl NSFileProviderDomain {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderDomain").unwrap(), alloc) })
    }
}
impl INSObject for NSFileProviderDomain {}
impl PNSObject for NSFileProviderDomain {}
impl std::convert::TryFrom<NSObject> for NSFileProviderDomain {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderDomain, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderDomain").unwrap()) };
        if is_kind_of {
            Ok(NSFileProviderDomain(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderDomain")
        }
    }
}
impl INSFileProviderDomain for NSFileProviderDomain {}
pub trait INSFileProviderDomain: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_displayName_pathRelativeToDocumentStorage_(
        &self,
        identifier: NSString,
        displayName: NSString,
        pathRelativeToDocumentStorage: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, displayName : displayName, pathRelativeToDocumentStorage : pathRelativeToDocumentStorage)
    }
    unsafe fn initWithIdentifier_displayName_(
        &self,
        identifier: NSString,
        displayName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, displayName : displayName)
    }
    unsafe fn initWithDisplayName_userInfo_volumeURL_(
        &self,
        displayName: NSString,
        userInfo: NSDictionary,
        volumeURL: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisplayName : displayName, userInfo : userInfo, volumeURL : volumeURL)
    }
    unsafe fn identifier(&self) -> NSFileProviderDomainIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn pathRelativeToDocumentStorage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathRelativeToDocumentStorage)
    }
    unsafe fn isDisconnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDisconnected)
    }
    unsafe fn userEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userEnabled)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn setHidden_(&self, hidden: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidden : hidden)
    }
    unsafe fn isReplicated(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReplicated)
    }
    unsafe fn testingModes(&self) -> NSFileProviderDomainTestingModes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, testingModes)
    }
    unsafe fn setTestingModes_(&self, testingModes: NSFileProviderDomainTestingModes)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTestingModes : testingModes)
    }
    unsafe fn backingStoreIdentity(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backingStoreIdentity)
    }
    unsafe fn supportsSyncingTrash(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsSyncingTrash)
    }
    unsafe fn setSupportsSyncingTrash_(&self, supportsSyncingTrash: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsSyncingTrash : supportsSyncingTrash)
    }
    unsafe fn volumeUUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volumeUUID)
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
    unsafe fn replicatedKnownFolders(&self) -> NSFileProviderKnownFolders
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replicatedKnownFolders)
    }
    unsafe fn supportedKnownFolders(&self) -> NSFileProviderKnownFolders
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedKnownFolders)
    }
    unsafe fn setSupportedKnownFolders_(&self, supportedKnownFolders: NSFileProviderKnownFolders)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedKnownFolders : supportedKnownFolders)
    }
    unsafe fn supportsStringSearchRequest(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsStringSearchRequest)
    }
    unsafe fn setSupportsStringSearchRequest_(&self, supportsStringSearchRequest: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsStringSearchRequest : supportsStringSearchRequest)
    }
}
impl NSFileProviderExtension_NSFileProviderDomain for NSFileProviderExtension {}
pub trait NSFileProviderExtension_NSFileProviderDomain: Sized + std::ops::Deref {
    unsafe fn domain(&self) -> NSFileProviderDomain
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domain)
    }
}
pub type NSFileProviderUserInfoKey = NSString;
pub type NSFileProviderPage = NSData;
pub trait PNSFileProviderEnumerationObserver: Sized + std::ops::Deref {
    unsafe fn didEnumerateItems_(&self, updatedItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didEnumerateItems : updatedItems)
    }
    unsafe fn finishEnumeratingUpToPage_(&self, nextPage: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishEnumeratingUpToPage : nextPage)
    }
    unsafe fn finishEnumeratingWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishEnumeratingWithError : error)
    }
    unsafe fn suggestedPageSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suggestedPageSize)
    }
}
pub trait PNSFileProviderChangeObserver: Sized + std::ops::Deref {
    unsafe fn didUpdateItems_(&self, updatedItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didUpdateItems : updatedItems)
    }
    unsafe fn didDeleteItemsWithIdentifiers_(&self, deletedItemIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didDeleteItemsWithIdentifiers : deletedItemIdentifiers)
    }
    unsafe fn finishEnumeratingChangesUpToSyncAnchor_moreComing_(
        &self,
        anchor: NSData,
        moreComing: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishEnumeratingChangesUpToSyncAnchor : anchor, moreComing : moreComing)
    }
    unsafe fn finishEnumeratingWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishEnumeratingWithError : error)
    }
    unsafe fn suggestedBatchSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suggestedBatchSize)
    }
}
pub trait PNSFileProviderEnumerator: Sized + std::ops::Deref {
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn enumerateItemsForObserver_startingAtPage_(&self, observer: *mut u64, page: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateItemsForObserver : observer, startingAtPage : page)
    }
    unsafe fn enumerateChangesForObserver_fromSyncAnchor_(
        &self,
        observer: *mut u64,
        syncAnchor: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateChangesForObserver : observer, fromSyncAnchor : syncAnchor)
    }
    unsafe fn currentSyncAnchorWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, currentSyncAnchorWithCompletionHandler : completionHandler)
    }
}
impl NSFileProviderExtension_NSFileProviderEnumeration for NSFileProviderExtension {}
pub trait NSFileProviderExtension_NSFileProviderEnumeration: Sized + std::ops::Deref {
    unsafe fn enumeratorForContainerItemIdentifier_error_(
        &self,
        containerItemIdentifier: NSString,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumeratorForContainerItemIdentifier : containerItemIdentifier, error : error)
    }
}
pub type NSFileProviderErrorCode = NSInteger;
pub trait NSError_NSFileProviderError: Sized + std::ops::Deref {
    unsafe fn fileProviderErrorForCollisionWithItem_(
        existingItem: NSFileProviderItem,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSError").unwrap(), fileProviderErrorForCollisionWithItem : existingItem)
    }
    unsafe fn fileProviderErrorForNonExistentItemWithIdentifier_(
        itemIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSError").unwrap(), fileProviderErrorForNonExistentItemWithIdentifier : itemIdentifier)
    }
    unsafe fn fileProviderErrorForRejectedDeletionOfItem_(
        updatedVersion: NSFileProviderItem,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSError").unwrap(), fileProviderErrorForRejectedDeletionOfItem : updatedVersion)
    }
}
pub type NSFileProviderModifyItemOptions = NSUInteger;
pub type NSFileProviderDomainRemovalMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderManager(pub id);
impl std::ops::Deref for NSFileProviderManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderManager {}
impl NSFileProviderManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), alloc) })
    }
}
impl INSObject for NSFileProviderManager {}
impl PNSObject for NSFileProviderManager {}
impl std::convert::TryFrom<NSObject> for NSFileProviderManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap()) };
        if is_kind_of {
            Ok(NSFileProviderManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderManager")
        }
    }
}
impl INSFileProviderManager for NSFileProviderManager {}
pub trait INSFileProviderManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn signalEnumeratorForContainerItemIdentifier_completionHandler_(
        &self,
        containerItemIdentifier: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signalEnumeratorForContainerItemIdentifier : containerItemIdentifier, completionHandler : completion)
    }
    unsafe fn getUserVisibleURLForItemIdentifier_completionHandler_(
        &self,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getUserVisibleURLForItemIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn registerURLSessionTask_forItemWithIdentifier_completionHandler_(
        &self,
        task: NSURLSessionTask,
        identifier: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerURLSessionTask : task, forItemWithIdentifier : identifier, completionHandler : completion)
    }
    unsafe fn temporaryDirectoryURLWithError_(&self, error: *mut NSError) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, temporaryDirectoryURLWithError : error)
    }
    unsafe fn signalErrorResolved_completionHandler_(
        &self,
        error: NSError,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signalErrorResolved : error, completionHandler : completionHandler)
    }
    unsafe fn globalProgressForKind_(&self, kind: NSString) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, globalProgressForKind : kind)
    }
    unsafe fn providerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerIdentifier)
    }
    unsafe fn documentStorageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentStorageURL)
    }
    unsafe fn managerForDomain_(domain: NSFileProviderDomain) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), managerForDomain : domain)
    }
    unsafe fn getIdentifierForUserVisibleFileAtURL_completionHandler_(
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), getIdentifierForUserVisibleFileAtURL : url, completionHandler : completionHandler)
    }
    unsafe fn writePlaceholderAtURL_withMetadata_error_(
        placeholderURL: NSURL,
        metadata: NSFileProviderItem,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), writePlaceholderAtURL : placeholderURL, withMetadata : metadata, error : error)
    }
    unsafe fn placeholderURLForURL_(url: NSURL) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), placeholderURLForURL : url)
    }
    unsafe fn addDomain_completionHandler_(
        domain: NSFileProviderDomain,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), addDomain : domain, completionHandler : completionHandler)
    }
    unsafe fn removeDomain_completionHandler_(
        domain: NSFileProviderDomain,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), removeDomain : domain, completionHandler : completionHandler)
    }
    unsafe fn removeDomain_mode_completionHandler_(
        domain: NSFileProviderDomain,
        mode: NSFileProviderDomainRemovalMode,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), removeDomain : domain, mode : mode, completionHandler : completionHandler)
    }
    unsafe fn getDomainsWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), getDomainsWithCompletionHandler : completionHandler)
    }
    unsafe fn removeAllDomainsWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), removeAllDomainsWithCompletionHandler : completionHandler)
    }
    unsafe fn defaultManager() -> NSFileProviderManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), defaultManager)
    }
}
impl NSFileProviderManager_MaterializedSet for NSFileProviderManager {}
pub trait NSFileProviderManager_MaterializedSet: Sized + std::ops::Deref {
    unsafe fn enumeratorForMaterializedItems(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enumeratorForMaterializedItems)
    }
}
pub trait PNSFileProviderPendingSetEnumerator: Sized + std::ops::Deref {
    unsafe fn domainVersion(&self) -> NSFileProviderDomainVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainVersion)
    }
    unsafe fn refreshInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refreshInterval)
    }
    unsafe fn isMaximumSizeReached(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMaximumSizeReached)
    }
}
impl NSFileProviderManager_PendingSet for NSFileProviderManager {}
pub trait NSFileProviderManager_PendingSet: Sized + std::ops::Deref {
    unsafe fn enumeratorForPendingItems(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enumeratorForPendingItems)
    }
}
impl NSFileProviderManager_Import for NSFileProviderManager {}
pub trait NSFileProviderManager_Import: Sized + std::ops::Deref {
    unsafe fn reimportItemsBelowItemWithIdentifier_completionHandler_(
        &self,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reimportItemsBelowItemWithIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn requestModificationOfFields_forItemWithIdentifier_options_completionHandler_(
        &self,
        fields: NSFileProviderItemFields,
        itemIdentifier: NSString,
        options: NSFileProviderModifyItemOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestModificationOfFields : fields, forItemWithIdentifier : itemIdentifier, options : options, completionHandler : completionHandler)
    }
    unsafe fn importDomain_fromDirectoryAtURL_completionHandler_(
        domain: NSFileProviderDomain,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), importDomain : domain, fromDirectoryAtURL : url, completionHandler : completionHandler)
    }
}
impl NSFileProviderManager_Eviction for NSFileProviderManager {}
pub trait NSFileProviderManager_Eviction: Sized + std::ops::Deref {
    unsafe fn evictItemWithIdentifier_completionHandler_(
        &self,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evictItemWithIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
}
impl NSFileProviderManager_Barrier for NSFileProviderManager {}
pub trait NSFileProviderManager_Barrier: Sized + std::ops::Deref {
    unsafe fn waitForChangesOnItemsBelowItemWithIdentifier_completionHandler_(
        &self,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForChangesOnItemsBelowItemWithIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
}
impl NSFileProviderManager_Stabilization for NSFileProviderManager {}
pub trait NSFileProviderManager_Stabilization: Sized + std::ops::Deref {
    unsafe fn waitForStabilizationWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForStabilizationWithCompletionHandler : completionHandler)
    }
}
pub type NSFileProviderManagerDisconnectionOptions = NSUInteger;
impl NSFileProviderManager_Disconnection for NSFileProviderManager {}
pub trait NSFileProviderManager_Disconnection: Sized + std::ops::Deref {
    unsafe fn disconnectWithReason_options_completionHandler_(
        &self,
        localizedReason: NSString,
        options: NSFileProviderManagerDisconnectionOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectWithReason : localizedReason, options : options, completionHandler : completionHandler)
    }
    unsafe fn reconnectWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reconnectWithCompletionHandler : completionHandler)
    }
}
impl NSFileProviderManager_Materialize for NSFileProviderManager {}
pub trait NSFileProviderManager_Materialize: Sized + std::ops::Deref {
    unsafe fn requestDownloadForItemWithIdentifier_requestedRange_completionHandler_(
        &self,
        itemIdentifier: NSString,
        rangeToMaterialize: NSRange,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDownloadForItemWithIdentifier : itemIdentifier, requestedRange : rangeToMaterialize, completionHandler : completionHandler)
    }
}
impl NSFileProviderManager_StateDirectory for NSFileProviderManager {}
pub trait NSFileProviderManager_StateDirectory: Sized + std::ops::Deref {
    unsafe fn stateDirectoryURLWithError_(&self, error: *mut NSError) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stateDirectoryURLWithError : error)
    }
}
pub type NSFileProviderVolumeUnsupportedReason = NSUInteger;
impl NSFileProviderManager_ExternalDomain for NSFileProviderManager {}
pub trait NSFileProviderManager_ExternalDomain: Sized + std::ops::Deref {
    unsafe fn checkDomainsCanBeStored_onVolumeAtURL_unsupportedReason_error_(
        eligible: *mut BOOL,
        url: NSURL,
        unsupportedReason: *mut NSFileProviderVolumeUnsupportedReason,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderManager").unwrap(), checkDomainsCanBeStored : eligible, onVolumeAtURL : url, unsupportedReason : unsupportedReason, error : error)
    }
}
impl NSFileProviderManager_Diagnostics for NSFileProviderManager {}
pub trait NSFileProviderManager_Diagnostics: Sized + std::ops::Deref {
    unsafe fn requestDiagnosticCollectionForItemWithIdentifier_errorReason_completionHandler_(
        &self,
        itemIdentifier: NSString,
        errorReason: NSError,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDiagnosticCollectionForItemWithIdentifier : itemIdentifier, errorReason : errorReason, completionHandler : completionHandler)
    }
}
impl NSFileProviderExtension_NSFileProviderActions for NSFileProviderExtension {}
pub trait NSFileProviderExtension_NSFileProviderActions: Sized + std::ops::Deref {
    unsafe fn importDocumentAtURL_toParentItemIdentifier_completionHandler_(
        &self,
        fileURL: NSURL,
        parentItemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, importDocumentAtURL : fileURL, toParentItemIdentifier : parentItemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn createDirectoryWithName_inParentItemIdentifier_completionHandler_(
        &self,
        directoryName: NSString,
        parentItemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createDirectoryWithName : directoryName, inParentItemIdentifier : parentItemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn renameItemWithIdentifier_toName_completionHandler_(
        &self,
        itemIdentifier: NSString,
        itemName: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renameItemWithIdentifier : itemIdentifier, toName : itemName, completionHandler : completionHandler)
    }
    unsafe fn reparentItemWithIdentifier_toParentItemWithIdentifier_newName_completionHandler_(
        &self,
        itemIdentifier: NSString,
        parentItemIdentifier: NSString,
        newName: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reparentItemWithIdentifier : itemIdentifier, toParentItemWithIdentifier : parentItemIdentifier, newName : newName, completionHandler : completionHandler)
    }
    unsafe fn trashItemWithIdentifier_completionHandler_(
        &self,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, trashItemWithIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn untrashItemWithIdentifier_toParentItemIdentifier_completionHandler_(
        &self,
        itemIdentifier: NSString,
        parentItemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, untrashItemWithIdentifier : itemIdentifier, toParentItemIdentifier : parentItemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn deleteItemWithIdentifier_completionHandler_(
        &self,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteItemWithIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn setLastUsedDate_forItemIdentifier_completionHandler_(
        &self,
        lastUsedDate: NSDate,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLastUsedDate : lastUsedDate, forItemIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn setTagData_forItemIdentifier_completionHandler_(
        &self,
        tagData: NSData,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTagData : tagData, forItemIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
    unsafe fn setFavoriteRank_forItemIdentifier_completionHandler_(
        &self,
        favoriteRank: NSNumber,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFavoriteRank : favoriteRank, forItemIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
}
pub trait PNSFileProviderServiceSource: Sized + std::ops::Deref {
    unsafe fn makeListenerEndpointAndReturnError_(
        &self,
        error: *mut NSError,
    ) -> NSXPCListenerEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeListenerEndpointAndReturnError : error)
    }
    unsafe fn serviceName(&self) -> NSFileProviderServiceName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceName)
    }
    unsafe fn isRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRestricted)
    }
}
impl NSFileProviderExtension_NSFileProviderService for NSFileProviderExtension {}
pub trait NSFileProviderExtension_NSFileProviderService: Sized + std::ops::Deref {
    unsafe fn supportedServiceSourcesForItemIdentifier_error_(
        &self,
        itemIdentifier: NSString,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedServiceSourcesForItemIdentifier : itemIdentifier, error : error)
    }
}
impl NSFileProviderManager_NSFileProviderService for NSFileProviderManager {}
pub trait NSFileProviderManager_NSFileProviderService: Sized + std::ops::Deref {
    unsafe fn getServiceWithName_itemIdentifier_completionHandler_(
        &self,
        serviceName: NSString,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getServiceWithName : serviceName, itemIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
}
impl NSFileProviderExtension_NSFileProviderThumbnailing for NSFileProviderExtension {}
pub trait NSFileProviderExtension_NSFileProviderThumbnailing: Sized + std::ops::Deref {
    unsafe fn fetchThumbnailsForItemIdentifiers_requestedSize_perThumbnailCompletionHandler_completionHandler_(
        &self,
        itemIdentifiers: NSArray,
        size: CGSize,
        perThumbnailCompletionHandler: *mut ::std::os::raw::c_void,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchThumbnailsForItemIdentifiers : itemIdentifiers, requestedSize : size, perThumbnailCompletionHandler : perThumbnailCompletionHandler, completionHandler : completionHandler)
    }
}
pub trait PNSFileProviderItemDecorating: Sized + std::ops::Deref {
    unsafe fn decorations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decorations)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderRequest(pub id);
impl std::ops::Deref for NSFileProviderRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderRequest {}
impl NSFileProviderRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderRequest").unwrap(), alloc) })
    }
}
impl INSObject for NSFileProviderRequest {}
impl PNSObject for NSFileProviderRequest {}
impl std::convert::TryFrom<NSObject> for NSFileProviderRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderRequest").unwrap()) };
        if is_kind_of {
            Ok(NSFileProviderRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderRequest")
        }
    }
}
impl INSFileProviderRequest for NSFileProviderRequest {}
pub trait INSFileProviderRequest: Sized + std::ops::Deref {
    unsafe fn isSystemRequest(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSystemRequest)
    }
    unsafe fn isFileViewerRequest(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFileViewerRequest)
    }
    unsafe fn requestingExecutable(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestingExecutable)
    }
    unsafe fn domainVersion(&self) -> NSFileProviderDomainVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainVersion)
    }
}
pub type NSFileProviderCreateItemOptions = NSUInteger;
pub type NSFileProviderDeleteItemOptions = NSUInteger;
pub type NSFileProviderMaterializationFlags = NSUInteger;
pub type NSFileProviderFetchContentsOptions = NSUInteger;
pub trait PNSFileProviderEnumerating: Sized + std::ops::Deref {
    unsafe fn enumeratorForContainerItemIdentifier_request_error_(
        &self,
        containerItemIdentifier: NSString,
        request: NSFileProviderRequest,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumeratorForContainerItemIdentifier : containerItemIdentifier, request : request, error : error)
    }
}
pub trait PNSFileProviderReplicatedExtension: Sized + std::ops::Deref {
    unsafe fn initWithDomain_(&self, domain: NSFileProviderDomain) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDomain : domain)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn itemForIdentifier_request_completionHandler_(
        &self,
        identifier: NSString,
        request: NSFileProviderRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemForIdentifier : identifier, request : request, completionHandler : completionHandler)
    }
    unsafe fn fetchContentsForItemWithIdentifier_version_request_completionHandler_(
        &self,
        itemIdentifier: NSString,
        requestedVersion: NSFileProviderItemVersion,
        request: NSFileProviderRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchContentsForItemWithIdentifier : itemIdentifier, version : requestedVersion, request : request, completionHandler : completionHandler)
    }
    unsafe fn createItemBasedOnTemplate_fields_contents_options_request_completionHandler_(
        &self,
        itemTemplate: NSFileProviderItem,
        fields: NSFileProviderItemFields,
        url: NSURL,
        options: NSFileProviderCreateItemOptions,
        request: NSFileProviderRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createItemBasedOnTemplate : itemTemplate, fields : fields, contents : url, options : options, request : request, completionHandler : completionHandler)
    }
    unsafe fn modifyItem_baseVersion_changedFields_contents_options_request_completionHandler_(
        &self,
        item: NSFileProviderItem,
        version: NSFileProviderItemVersion,
        changedFields: NSFileProviderItemFields,
        newContents: NSURL,
        options: NSFileProviderModifyItemOptions,
        request: NSFileProviderRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modifyItem : item, baseVersion : version, changedFields : changedFields, contents : newContents, options : options, request : request, completionHandler : completionHandler)
    }
    unsafe fn deleteItemWithIdentifier_baseVersion_options_request_completionHandler_(
        &self,
        identifier: NSString,
        version: NSFileProviderItemVersion,
        options: NSFileProviderDeleteItemOptions,
        request: NSFileProviderRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteItemWithIdentifier : identifier, baseVersion : version, options : options, request : request, completionHandler : completionHandler)
    }
    unsafe fn importDidFinishWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, importDidFinishWithCompletionHandler : completionHandler)
    }
    unsafe fn materializedItemsDidChangeWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, materializedItemsDidChangeWithCompletionHandler : completionHandler)
    }
    unsafe fn pendingItemsDidChangeWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pendingItemsDidChangeWithCompletionHandler : completionHandler)
    }
}
pub trait PNSFileProviderIncrementalContentFetching: Sized + std::ops::Deref {
    unsafe fn fetchContentsForItemWithIdentifier_version_usingExistingContentsAtURL_existingVersion_request_completionHandler_(
        &self,
        itemIdentifier: NSString,
        requestedVersion: NSFileProviderItemVersion,
        existingContents: NSURL,
        existingVersion: NSFileProviderItemVersion,
        request: NSFileProviderRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchContentsForItemWithIdentifier : itemIdentifier, version : requestedVersion, usingExistingContentsAtURL : existingContents, existingVersion : existingVersion, request : request, completionHandler : completionHandler)
    }
}
pub trait PNSFileProviderServicing: Sized + std::ops::Deref {
    unsafe fn supportedServiceSourcesForItemIdentifier_completionHandler_(
        &self,
        itemIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedServiceSourcesForItemIdentifier : itemIdentifier, completionHandler : completionHandler)
    }
}
pub trait PNSFileProviderThumbnailing: Sized + std::ops::Deref {
    unsafe fn fetchThumbnailsForItemIdentifiers_requestedSize_perThumbnailCompletionHandler_completionHandler_(
        &self,
        itemIdentifiers: NSArray,
        size: CGSize,
        perThumbnailCompletionHandler: *mut ::std::os::raw::c_void,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchThumbnailsForItemIdentifiers : itemIdentifiers, requestedSize : size, perThumbnailCompletionHandler : perThumbnailCompletionHandler, completionHandler : completionHandler)
    }
}
pub trait PNSFileProviderCustomAction: Sized + std::ops::Deref {
    unsafe fn performActionWithIdentifier_onItemsWithIdentifiers_completionHandler_(
        &self,
        actionIdentifier: NSString,
        itemIdentifiers: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performActionWithIdentifier : actionIdentifier, onItemsWithIdentifiers : itemIdentifiers, completionHandler : completionHandler)
    }
}
pub trait PNSFileProviderUserInteractionSuppressing: Sized + std::ops::Deref {
    unsafe fn setInteractionSuppressed_forIdentifier_(
        &self,
        suppression: BOOL,
        suppressionIdentifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteractionSuppressed : suppression, forIdentifier : suppressionIdentifier)
    }
    unsafe fn isInteractionSuppressedForIdentifier_(&self, suppressionIdentifier: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isInteractionSuppressedForIdentifier : suppressionIdentifier)
    }
}
pub trait PNSFileProviderDomainState: Sized + std::ops::Deref {
    unsafe fn domainVersion(&self) -> NSFileProviderDomainVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainVersion)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
}
pub trait PNSFileProviderPartialContentFetching: Sized + std::ops::Deref {
    unsafe fn fetchPartialContentsForItemWithIdentifier_version_request_minimalRange_aligningTo_options_completionHandler_(
        &self,
        itemIdentifier: NSString,
        requestedVersion: NSFileProviderItemVersion,
        request: NSFileProviderRequest,
        requestedRange: NSRange,
        alignment: NSUInteger,
        options: NSFileProviderFetchContentsOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchPartialContentsForItemWithIdentifier : itemIdentifier, version : requestedVersion, request : request, minimalRange : requestedRange, aligningTo : alignment, options : options, completionHandler : completionHandler)
    }
}
pub trait PNSFileProviderExternalVolumeHandling: Sized + std::ops::Deref {
    unsafe fn shouldConnectExternalDomainWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldConnectExternalDomainWithCompletionHandler : completionHandler)
    }
}
pub trait PNSFileProviderSearchResult: Sized + std::ops::Deref {
    unsafe fn itemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemIdentifier)
    }
    unsafe fn filename(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filename)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn contentModificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentModificationDate)
    }
    unsafe fn lastUsedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastUsedDate)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn documentSize(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentSize)
    }
}
pub trait PNSFileProviderSearchEnumerationObserver: Sized + std::ops::Deref {
    unsafe fn didEnumerateSearchResults_(&self, searchResults: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didEnumerateSearchResults : searchResults)
    }
    unsafe fn finishEnumeratingUpToPage_(&self, nextPage: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishEnumeratingUpToPage : nextPage)
    }
    unsafe fn finishEnumeratingWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishEnumeratingWithError : error)
    }
    unsafe fn maximumNumberOfResultsPerPage(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumNumberOfResultsPerPage)
    }
}
pub trait PNSFileProviderSearchEnumerator: Sized + std::ops::Deref {
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn enumerateSearchResultsForObserver_startingAtPage_(
        &self,
        observer: *mut u64,
        page: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateSearchResultsForObserver : observer, startingAtPage : page)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderStringSearchRequest(pub id);
impl std::ops::Deref for NSFileProviderStringSearchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderStringSearchRequest {}
impl NSFileProviderStringSearchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderStringSearchRequest").unwrap(), alloc) })
    }
}
impl INSObject for NSFileProviderStringSearchRequest {}
impl PNSObject for NSFileProviderStringSearchRequest {}
impl std::convert::TryFrom<NSObject> for NSFileProviderStringSearchRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderStringSearchRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderStringSearchRequest").unwrap())
        };
        if is_kind_of {
            Ok(NSFileProviderStringSearchRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderStringSearchRequest")
        }
    }
}
impl INSFileProviderStringSearchRequest for NSFileProviderStringSearchRequest {}
pub trait INSFileProviderStringSearchRequest: Sized + std::ops::Deref {
    unsafe fn query(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, query)
    }
    unsafe fn desiredNumberOfResults(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredNumberOfResults)
    }
}
pub trait PNSFileProviderSearching: Sized + std::ops::Deref {
    unsafe fn searchEnumeratorForStringSearchRequest_(
        &self,
        request: NSFileProviderStringSearchRequest,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchEnumeratorForStringSearchRequest : request)
    }
}
pub type NSFileProviderTestingOperationType = NSInteger;
pub trait PNSFileProviderTestingOperation: Sized + std::ops::Deref {
    unsafe fn asIngestion(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asIngestion)
    }
    unsafe fn asLookup(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asLookup)
    }
    unsafe fn asCreation(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asCreation)
    }
    unsafe fn asModification(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asModification)
    }
    unsafe fn asDeletion(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asDeletion)
    }
    unsafe fn asContentFetch(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asContentFetch)
    }
    unsafe fn asChildrenEnumeration(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asChildrenEnumeration)
    }
    unsafe fn asCollisionResolution(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asCollisionResolution)
    }
    unsafe fn type_(&self) -> NSFileProviderTestingOperationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
impl NSFileProviderManager_TestingModeInteractive for NSFileProviderManager {}
pub trait NSFileProviderManager_TestingModeInteractive: Sized + std::ops::Deref {
    unsafe fn listAvailableTestingOperationsWithError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, listAvailableTestingOperationsWithError : error)
    }
    unsafe fn runTestingOperations_error_(
        &self,
        operations: NSArray,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runTestingOperations : operations, error : error)
    }
}
pub type NSFileProviderTestingOperationSide = NSUInteger;
pub trait PNSFileProviderTestingIngestion: Sized + std::ops::Deref {
    unsafe fn side(&self) -> NSFileProviderTestingOperationSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, side)
    }
    unsafe fn itemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemIdentifier)
    }
    unsafe fn item(&self) -> NSFileProviderItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, item)
    }
}
pub trait PNSFileProviderTestingLookup: Sized + std::ops::Deref {
    unsafe fn side(&self) -> NSFileProviderTestingOperationSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, side)
    }
    unsafe fn itemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemIdentifier)
    }
}
pub trait PNSFileProviderTestingCreation: Sized + std::ops::Deref {
    unsafe fn targetSide(&self) -> NSFileProviderTestingOperationSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetSide)
    }
    unsafe fn sourceItem(&self) -> NSFileProviderItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceItem)
    }
    unsafe fn domainVersion(&self) -> NSFileProviderDomainVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainVersion)
    }
}
pub trait PNSFileProviderTestingModification: Sized + std::ops::Deref {
    unsafe fn targetSide(&self) -> NSFileProviderTestingOperationSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetSide)
    }
    unsafe fn sourceItem(&self) -> NSFileProviderItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceItem)
    }
    unsafe fn targetItemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetItemIdentifier)
    }
    unsafe fn targetItemBaseVersion(&self) -> NSFileProviderItemVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetItemBaseVersion)
    }
    unsafe fn changedFields(&self) -> NSFileProviderItemFields
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedFields)
    }
    unsafe fn domainVersion(&self) -> NSFileProviderDomainVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainVersion)
    }
}
pub trait PNSFileProviderTestingDeletion: Sized + std::ops::Deref {
    unsafe fn targetSide(&self) -> NSFileProviderTestingOperationSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetSide)
    }
    unsafe fn sourceItemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceItemIdentifier)
    }
    unsafe fn targetItemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetItemIdentifier)
    }
    unsafe fn targetItemBaseVersion(&self) -> NSFileProviderItemVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetItemBaseVersion)
    }
    unsafe fn domainVersion(&self) -> NSFileProviderDomainVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainVersion)
    }
}
pub trait PNSFileProviderTestingContentFetch: Sized + std::ops::Deref {
    unsafe fn side(&self) -> NSFileProviderTestingOperationSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, side)
    }
    unsafe fn itemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemIdentifier)
    }
}
pub trait PNSFileProviderTestingChildrenEnumeration: Sized + std::ops::Deref {
    unsafe fn side(&self) -> NSFileProviderTestingOperationSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, side)
    }
    unsafe fn itemIdentifier(&self) -> NSFileProviderItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemIdentifier)
    }
}
pub trait PNSFileProviderTestingCollisionResolution: Sized + std::ops::Deref {
    unsafe fn side(&self) -> NSFileProviderTestingOperationSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, side)
    }
    unsafe fn renamedItem(&self) -> NSFileProviderItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renamedItem)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderKnownFolderLocation(pub id);
impl std::ops::Deref for NSFileProviderKnownFolderLocation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderKnownFolderLocation {}
impl NSFileProviderKnownFolderLocation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderKnownFolderLocation").unwrap(), alloc) })
    }
}
impl INSObject for NSFileProviderKnownFolderLocation {}
impl PNSObject for NSFileProviderKnownFolderLocation {}
impl std::convert::TryFrom<NSObject> for NSFileProviderKnownFolderLocation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderKnownFolderLocation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderKnownFolderLocation").unwrap())
        };
        if is_kind_of {
            Ok(NSFileProviderKnownFolderLocation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderKnownFolderLocation")
        }
    }
}
impl INSFileProviderKnownFolderLocation for NSFileProviderKnownFolderLocation {}
pub trait INSFileProviderKnownFolderLocation: Sized + std::ops::Deref {
    unsafe fn initWithParentItemIdentifier_filename_(
        &self,
        parentItemIdentifier: NSString,
        filename: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithParentItemIdentifier : parentItemIdentifier, filename : filename)
    }
    unsafe fn initWithExistingItemIdentifier_(
        &self,
        existingItemIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExistingItemIdentifier : existingItemIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSFileProviderKnownFolderLocations(pub id);
impl std::ops::Deref for NSFileProviderKnownFolderLocations {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSFileProviderKnownFolderLocations {}
impl NSFileProviderKnownFolderLocations {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileProviderKnownFolderLocations").unwrap(), alloc) })
    }
}
impl INSObject for NSFileProviderKnownFolderLocations {}
impl PNSObject for NSFileProviderKnownFolderLocations {}
impl std::convert::TryFrom<NSObject> for NSFileProviderKnownFolderLocations {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSFileProviderKnownFolderLocations, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSFileProviderKnownFolderLocations").unwrap())
        };
        if is_kind_of {
            Ok(NSFileProviderKnownFolderLocations(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSFileProviderKnownFolderLocations")
        }
    }
}
impl INSFileProviderKnownFolderLocations for NSFileProviderKnownFolderLocations {}
pub trait INSFileProviderKnownFolderLocations: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn shouldCreateBinaryCompatibilitySymlink(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCreateBinaryCompatibilitySymlink)
    }
    unsafe fn setShouldCreateBinaryCompatibilitySymlink_(
        &self,
        shouldCreateBinaryCompatibilitySymlink: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldCreateBinaryCompatibilitySymlink : shouldCreateBinaryCompatibilitySymlink)
    }
    unsafe fn desktopLocation(&self) -> NSFileProviderKnownFolderLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desktopLocation)
    }
    unsafe fn setDesktopLocation_(&self, desktopLocation: NSFileProviderKnownFolderLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesktopLocation : desktopLocation)
    }
    unsafe fn documentsLocation(&self) -> NSFileProviderKnownFolderLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentsLocation)
    }
    unsafe fn setDocumentsLocation_(&self, documentsLocation: NSFileProviderKnownFolderLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentsLocation : documentsLocation)
    }
}
impl NSFileProviderManager_KnownFolders for NSFileProviderManager {}
pub trait NSFileProviderManager_KnownFolders: Sized + std::ops::Deref {
    unsafe fn claimKnownFolders_localizedReason_completionHandler_(
        &self,
        knownFolders: NSFileProviderKnownFolderLocations,
        localizedReason: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, claimKnownFolders : knownFolders, localizedReason : localizedReason, completionHandler : completionHandler)
    }
    unsafe fn releaseKnownFolders_localizedReason_completionHandler_(
        &self,
        knownFolders: NSFileProviderKnownFolders,
        localizedReason: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, releaseKnownFolders : knownFolders, localizedReason : localizedReason, completionHandler : completionHandler)
    }
}
pub trait PNSFileProviderKnownFolderSupporting: Sized + std::ops::Deref {
    unsafe fn getKnownFolderLocations_completionHandler_(
        &self,
        knownFolders: NSFileProviderKnownFolders,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getKnownFolderLocations : knownFolders, completionHandler : completionHandler)
    }
}
unsafe extern "C" {
    pub static NSFileProviderRootContainerItemIdentifier: NSFileProviderItemIdentifier;
}
unsafe extern "C" {
    pub static NSFileProviderWorkingSetContainerItemIdentifier: NSFileProviderItemIdentifier;
}
unsafe extern "C" {
    pub static NSFileProviderTrashContainerItemIdentifier: NSFileProviderItemIdentifier;
}
unsafe extern "C" {
    pub static NSFileProviderFavoriteRankUnranked: ::std::os::raw::c_ulonglong;
}
unsafe extern "C" {
    pub static NSFileProviderDomainDidChange: NSNotificationName;
}
unsafe extern "C" {
    pub static NSFileProviderUserInfoExperimentIDKey: NSFileProviderUserInfoKey;
}
unsafe extern "C" {
    pub static NSFileProviderInitialPageSortedByDate: NSFileProviderPage;
}
unsafe extern "C" {
    pub static NSFileProviderInitialPageSortedByName: NSFileProviderPage;
}
unsafe extern "C" {
    pub static NSFileProviderErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static NSFileProviderErrorCollidingItemKey: NSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static NSFileProviderErrorItemKey: NSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static NSFileProviderErrorNonExistentItemIdentifierKey: NSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static NSFileProviderMaterializedSetDidChange: NSNotificationName;
}
unsafe extern "C" {
    pub static NSFileProviderPendingSetDidChange: NSNotificationName;
}

unsafe impl objc2::encode::RefEncode for NSFileProviderItemVersion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderItemVersion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFileProviderTypeAndCreator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderTypeAndCreator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NSFileProviderTypeAndCreator", &[]);
}
unsafe impl objc2::encode::RefEncode for NSFileProviderExtension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderExtension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFileProviderDomainVersion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderDomainVersion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFileProviderDomain {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderDomain {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFileProviderManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFileProviderRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFileProviderStringSearchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderStringSearchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFileProviderKnownFolderLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderKnownFolderLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSFileProviderKnownFolderLocations {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSFileProviderKnownFolderLocations {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
