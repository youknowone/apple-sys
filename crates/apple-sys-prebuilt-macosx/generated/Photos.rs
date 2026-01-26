#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreImage::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use crate::ImageIO::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type PHImageContentMode = NSInteger;
pub type PHCollectionListType = NSInteger;
pub type PHCollectionListSubtype = NSInteger;
pub type PHCollectionEditOperation = NSInteger;
pub type PHAssetCollectionType = NSInteger;
pub type PHAssetCollectionSubtype = NSInteger;
pub type PHAssetEditOperation = NSInteger;
pub type PHAssetPlaybackStyle = NSInteger;
pub type PHAssetMediaType = NSInteger;
pub type PHAssetMediaSubtype = NSUInteger;
pub type PHAssetBurstSelectionType = NSUInteger;
pub type PHAssetSourceType = NSUInteger;
pub type PHAssetResourceType = NSInteger;
pub type PHAssetResourceUploadJobState = NSInteger;
pub type PHAssetResourceUploadJobAction = NSInteger;
pub type PHObjectType = NSInteger;
pub type PHAuthorizationStatus = NSInteger;
pub type PHAccessLevel = NSInteger;
pub trait PPHPhotoLibraryChangeObserver: Sized + std::ops::Deref {
    unsafe fn photoLibraryDidChange_(&self, changeInstance: PHChange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, photoLibraryDidChange : changeInstance)
    }
}
pub trait PPHPhotoLibraryAvailabilityObserver: Sized + std::ops::Deref {
    unsafe fn photoLibraryDidBecomeUnavailable_(&self, photoLibrary: PHPhotoLibrary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, photoLibraryDidBecomeUnavailable : photoLibrary)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPhotoLibrary(pub id);
impl std::ops::Deref for PHPhotoLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPhotoLibrary {}
impl PHPhotoLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPhotoLibrary").unwrap(), alloc) })
    }
}
impl INSObject for PHPhotoLibrary {}
impl PNSObject for PHPhotoLibrary {}
impl std::convert::TryFrom<NSObject> for PHPhotoLibrary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPhotoLibrary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPhotoLibrary").unwrap()) };
        if is_kind_of {
            Ok(PHPhotoLibrary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPhotoLibrary")
        }
    }
}
impl IPHPhotoLibrary for PHPhotoLibrary {}
pub trait IPHPhotoLibrary: Sized + std::ops::Deref {
    unsafe fn registerAvailabilityObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerAvailabilityObserver : observer)
    }
    unsafe fn unregisterAvailabilityObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterAvailabilityObserver : observer)
    }
    unsafe fn performChanges_completionHandler_(
        &self,
        changeBlock: dispatch_block_t,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performChanges : changeBlock, completionHandler : completionHandler)
    }
    unsafe fn performChangesAndWait_error_(
        &self,
        changeBlock: dispatch_block_t,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performChangesAndWait : changeBlock, error : error)
    }
    unsafe fn registerChangeObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerChangeObserver : observer)
    }
    unsafe fn unregisterChangeObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterChangeObserver : observer)
    }
    unsafe fn fetchPersistentChangesSinceToken_error_(
        &self,
        token: PHPersistentChangeToken,
        error: *mut NSError,
    ) -> PHPersistentChangeFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchPersistentChangesSinceToken : token, error : error)
    }
    unsafe fn unavailabilityReason(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unavailabilityReason)
    }
    unsafe fn currentChangeToken(&self) -> PHPersistentChangeToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentChangeToken)
    }
    unsafe fn sharedPhotoLibrary() -> PHPhotoLibrary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPhotoLibrary").unwrap(), sharedPhotoLibrary)
    }
    unsafe fn authorizationStatusForAccessLevel_(
        accessLevel: PHAccessLevel,
    ) -> PHAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPhotoLibrary").unwrap(), authorizationStatusForAccessLevel : accessLevel)
    }
    unsafe fn requestAuthorizationForAccessLevel_handler_(
        accessLevel: PHAccessLevel,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPhotoLibrary").unwrap(), requestAuthorizationForAccessLevel : accessLevel, handler : handler)
    }
    unsafe fn authorizationStatus() -> PHAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPhotoLibrary").unwrap(), authorizationStatus)
    }
    unsafe fn requestAuthorization_(handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPhotoLibrary").unwrap(), requestAuthorization : handler)
    }
}
pub type PHPhotosError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHObject(pub id);
impl std::ops::Deref for PHObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHObject {}
impl PHObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHObject").unwrap(), alloc) })
    }
}
impl PNSCopying for PHObject {}
impl INSObject for PHObject {}
impl PNSObject for PHObject {}
impl std::convert::TryFrom<NSObject> for PHObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHObject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHObject").unwrap()) };
        if is_kind_of {
            Ok(PHObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHObject")
        }
    }
}
impl IPHObject for PHObject {}
pub trait IPHObject: Sized + std::ops::Deref {
    unsafe fn localIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHObjectPlaceholder(pub id);
impl std::ops::Deref for PHObjectPlaceholder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHObjectPlaceholder {}
impl PHObjectPlaceholder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHObjectPlaceholder").unwrap(), alloc) })
    }
}
impl IPHObject for PHObjectPlaceholder {}
impl PNSCopying for PHObjectPlaceholder {}
impl From<PHObjectPlaceholder> for PHObject {
    fn from(child: PHObjectPlaceholder) -> PHObject {
        PHObject(child.0)
    }
}
impl std::convert::TryFrom<PHObject> for PHObjectPlaceholder {
    type Error = &'static str;
    fn try_from(parent: PHObject) -> Result<PHObjectPlaceholder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHObjectPlaceholder").unwrap()) };
        if is_kind_of {
            Ok(PHObjectPlaceholder(parent.0))
        } else {
            Err("This PHObject cannot be downcasted to PHObjectPlaceholder")
        }
    }
}
impl INSObject for PHObjectPlaceholder {}
impl PNSObject for PHObjectPlaceholder {}
impl IPHObjectPlaceholder for PHObjectPlaceholder {}
pub trait IPHObjectPlaceholder: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHFetchResult(pub id);
impl std::ops::Deref for PHFetchResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHFetchResult {}
impl PHFetchResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHFetchResult").unwrap(), alloc) })
    }
}
impl PNSCopying for PHFetchResult {}
impl PNSFastEnumeration for PHFetchResult {}
impl INSObject for PHFetchResult {}
impl PNSObject for PHFetchResult {}
impl std::convert::TryFrom<NSObject> for PHFetchResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHFetchResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHFetchResult").unwrap()) };
        if is_kind_of {
            Ok(PHFetchResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHFetchResult")
        }
    }
}
impl<ObjectType: 'static> IPHFetchResult<ObjectType> for PHFetchResult {}
pub trait IPHFetchResult<ObjectType: 'static>: Sized + std::ops::Deref {
    unsafe fn objectAtIndex_(&self, index: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndex : index)
    }
    unsafe fn objectAtIndexedSubscript_(&self, idx: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : idx)
    }
    unsafe fn containsObject_(&self, anObject: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsObject : anObject)
    }
    unsafe fn indexOfObject_(&self, anObject: id) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexOfObject : anObject)
    }
    unsafe fn indexOfObject_inRange_(&self, anObject: id, range: NSRange) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexOfObject : anObject, inRange : range)
    }
    unsafe fn objectsAtIndexes_(&self, indexes: NSIndexSet) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectsAtIndexes : indexes)
    }
    unsafe fn enumerateObjectsUsingBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateObjectsUsingBlock : block)
    }
    unsafe fn enumerateObjectsWithOptions_usingBlock_(
        &self,
        opts: NSEnumerationOptions,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateObjectsWithOptions : opts, usingBlock : block)
    }
    unsafe fn enumerateObjectsAtIndexes_options_usingBlock_(
        &self,
        s: NSIndexSet,
        opts: NSEnumerationOptions,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateObjectsAtIndexes : s, options : opts, usingBlock : block)
    }
    unsafe fn countOfAssetsWithMediaType_(&self, mediaType: PHAssetMediaType) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, countOfAssetsWithMediaType : mediaType)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn firstObject(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstObject)
    }
    unsafe fn lastObject(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastObject)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAsset(pub id);
impl std::ops::Deref for PHAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAsset {}
impl PHAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAsset").unwrap(), alloc) })
    }
}
impl IPHObject for PHAsset {}
impl PNSCopying for PHAsset {}
impl std::convert::TryFrom<PHObject> for PHAsset {
    type Error = &'static str;
    fn try_from(parent: PHObject) -> Result<PHAsset, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAsset").unwrap()) };
        if is_kind_of {
            Ok(PHAsset(parent.0))
        } else {
            Err("This PHObject cannot be downcasted to PHAsset")
        }
    }
}
impl INSObject for PHAsset {}
impl PNSObject for PHAsset {}
impl IPHAsset for PHAsset {}
pub trait IPHAsset: Sized + std::ops::Deref {
    unsafe fn canPerformEditOperation_(&self, editOperation: PHAssetEditOperation) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canPerformEditOperation : editOperation)
    }
    unsafe fn playbackStyle(&self) -> PHAssetPlaybackStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackStyle)
    }
    unsafe fn mediaType(&self) -> PHAssetMediaType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaType)
    }
    unsafe fn mediaSubtypes(&self) -> PHAssetMediaSubtype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSubtypes)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn pixelWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelWidth)
    }
    unsafe fn pixelHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelHeight)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn modificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modificationDate)
    }
    unsafe fn addedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addedDate)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn isFavorite(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFavorite)
    }
    unsafe fn isSyncFailureHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSyncFailureHidden)
    }
    unsafe fn burstIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, burstIdentifier)
    }
    unsafe fn burstSelectionTypes(&self) -> PHAssetBurstSelectionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, burstSelectionTypes)
    }
    unsafe fn representsBurst(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, representsBurst)
    }
    unsafe fn sourceType(&self) -> PHAssetSourceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceType)
    }
    unsafe fn hasAdjustments(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAdjustments)
    }
    unsafe fn adjustmentFormatIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adjustmentFormatIdentifier)
    }
    unsafe fn fetchAssetsInAssetCollection_options_(
        assetCollection: PHAssetCollection,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAsset").unwrap(), fetchAssetsInAssetCollection : assetCollection, options : options)
    }
    unsafe fn fetchAssetsWithLocalIdentifiers_options_(
        identifiers: NSArray,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAsset").unwrap(), fetchAssetsWithLocalIdentifiers : identifiers, options : options)
    }
    unsafe fn fetchKeyAssetsInAssetCollection_options_(
        assetCollection: PHAssetCollection,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAsset").unwrap(), fetchKeyAssetsInAssetCollection : assetCollection, options : options)
    }
    unsafe fn fetchAssetsWithBurstIdentifier_options_(
        burstIdentifier: NSString,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAsset").unwrap(), fetchAssetsWithBurstIdentifier : burstIdentifier, options : options)
    }
    unsafe fn fetchAssetsWithOptions_(options: PHFetchOptions) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAsset").unwrap(), fetchAssetsWithOptions : options)
    }
    unsafe fn fetchAssetsWithMediaType_options_(
        mediaType: PHAssetMediaType,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAsset").unwrap(), fetchAssetsWithMediaType : mediaType, options : options)
    }
    unsafe fn fetchAssetsWithALAssetURLs_options_(
        assetURLs: NSArray,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAsset").unwrap(), fetchAssetsWithALAssetURLs : assetURLs, options : options)
    }
}
pub type PHLivePhotoRequestID = i32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHLivePhoto(pub id);
impl std::ops::Deref for PHLivePhoto {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHLivePhoto {}
impl PHLivePhoto {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHLivePhoto").unwrap(), alloc) })
    }
}
impl PNSCopying for PHLivePhoto {}
impl PNSSecureCoding for PHLivePhoto {}
impl INSObject for PHLivePhoto {}
impl PNSObject for PHLivePhoto {}
impl std::convert::TryFrom<NSObject> for PHLivePhoto {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHLivePhoto, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHLivePhoto").unwrap()) };
        if is_kind_of {
            Ok(PHLivePhoto(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHLivePhoto")
        }
    }
}
impl IPHLivePhoto for PHLivePhoto {}
pub trait IPHLivePhoto: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn requestLivePhotoWithResourceFileURLs_placeholderImage_targetSize_contentMode_resultHandler_(
        fileURLs: NSArray,
        image: NSImage,
        targetSize: CGSize,
        contentMode: PHImageContentMode,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> PHLivePhotoRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHLivePhoto").unwrap(), requestLivePhotoWithResourceFileURLs : fileURLs, placeholderImage : image, targetSize : targetSize, contentMode : contentMode, resultHandler : resultHandler)
    }
    unsafe fn cancelLivePhotoRequestWithRequestID_(requestID: PHLivePhotoRequestID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHLivePhoto").unwrap(), cancelLivePhotoRequestWithRequestID : requestID)
    }
}
impl PHLivePhoto_NSItemProvider for PHLivePhoto {}
pub trait PHLivePhoto_NSItemProvider: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHCollection(pub id);
impl std::ops::Deref for PHCollection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHCollection {}
impl PHCollection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollection").unwrap(), alloc) })
    }
}
impl IPHObject for PHCollection {}
impl PNSCopying for PHCollection {}
impl std::convert::TryFrom<PHObject> for PHCollection {
    type Error = &'static str;
    fn try_from(parent: PHObject) -> Result<PHCollection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHCollection").unwrap()) };
        if is_kind_of {
            Ok(PHCollection(parent.0))
        } else {
            Err("This PHObject cannot be downcasted to PHCollection")
        }
    }
}
impl INSObject for PHCollection {}
impl PNSObject for PHCollection {}
impl IPHCollection for PHCollection {}
pub trait IPHCollection: Sized + std::ops::Deref {
    unsafe fn canPerformEditOperation_(&self, anOperation: PHCollectionEditOperation) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canPerformEditOperation : anOperation)
    }
    unsafe fn canContainAssets(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canContainAssets)
    }
    unsafe fn canContainCollections(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canContainCollections)
    }
    unsafe fn localizedTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedTitle)
    }
    unsafe fn fetchCollectionsInCollectionList_options_(
        collectionList: PHCollectionList,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollection").unwrap(), fetchCollectionsInCollectionList : collectionList, options : options)
    }
    unsafe fn fetchTopLevelUserCollectionsWithOptions_(options: PHFetchOptions) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollection").unwrap(), fetchTopLevelUserCollectionsWithOptions : options)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetCollection(pub id);
impl std::ops::Deref for PHAssetCollection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetCollection {}
impl PHAssetCollection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), alloc) })
    }
}
impl IPHCollection for PHAssetCollection {}
impl From<PHAssetCollection> for PHCollection {
    fn from(child: PHAssetCollection) -> PHCollection {
        PHCollection(child.0)
    }
}
impl std::convert::TryFrom<PHCollection> for PHAssetCollection {
    type Error = &'static str;
    fn try_from(parent: PHCollection) -> Result<PHAssetCollection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap()) };
        if is_kind_of {
            Ok(PHAssetCollection(parent.0))
        } else {
            Err("This PHCollection cannot be downcasted to PHAssetCollection")
        }
    }
}
impl IPHObject for PHAssetCollection {}
impl PNSCopying for PHAssetCollection {}
impl INSObject for PHAssetCollection {}
impl PNSObject for PHAssetCollection {}
impl IPHAssetCollection for PHAssetCollection {}
pub trait IPHAssetCollection: Sized + std::ops::Deref {
    unsafe fn assetCollectionType(&self) -> PHAssetCollectionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetCollectionType)
    }
    unsafe fn assetCollectionSubtype(&self) -> PHAssetCollectionSubtype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetCollectionSubtype)
    }
    unsafe fn estimatedAssetCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, estimatedAssetCount)
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
    unsafe fn approximateLocation(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, approximateLocation)
    }
    unsafe fn localizedLocationNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedLocationNames)
    }
    unsafe fn fetchAssetCollectionsWithLocalIdentifiers_options_(
        identifiers: NSArray,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), fetchAssetCollectionsWithLocalIdentifiers : identifiers, options : options)
    }
    unsafe fn fetchAssetCollectionsWithType_subtype_options_(
        type_: PHAssetCollectionType,
        subtype: PHAssetCollectionSubtype,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), fetchAssetCollectionsWithType : type_, subtype : subtype, options : options)
    }
    unsafe fn fetchAssetCollectionsContainingAsset_withType_options_(
        asset: PHAsset,
        type_: PHAssetCollectionType,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), fetchAssetCollectionsContainingAsset : asset, withType : type_, options : options)
    }
    unsafe fn fetchAssetCollectionsWithALAssetGroupURLs_options_(
        assetGroupURLs: NSArray,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), fetchAssetCollectionsWithALAssetGroupURLs : assetGroupURLs, options : options)
    }
    unsafe fn fetchMomentsInMomentList_options_(
        momentList: PHCollectionList,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), fetchMomentsInMomentList : momentList, options : options)
    }
    unsafe fn fetchMomentsWithOptions_(options: PHFetchOptions) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), fetchMomentsWithOptions : options)
    }
    unsafe fn transientAssetCollectionWithAssets_title_(
        assets: NSArray,
        title: NSString,
    ) -> PHAssetCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), transientAssetCollectionWithAssets : assets, title : title)
    }
    unsafe fn transientAssetCollectionWithAssetFetchResult_title_(
        fetchResult: PHFetchResult,
        title: NSString,
    ) -> PHAssetCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollection").unwrap(), transientAssetCollectionWithAssetFetchResult : fetchResult, title : title)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHCollectionList(pub id);
impl std::ops::Deref for PHCollectionList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHCollectionList {}
impl PHCollectionList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap(), alloc) })
    }
}
impl IPHCollection for PHCollectionList {}
impl std::convert::TryFrom<PHCollection> for PHCollectionList {
    type Error = &'static str;
    fn try_from(parent: PHCollection) -> Result<PHCollectionList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap()) };
        if is_kind_of {
            Ok(PHCollectionList(parent.0))
        } else {
            Err("This PHCollection cannot be downcasted to PHCollectionList")
        }
    }
}
impl IPHObject for PHCollectionList {}
impl PNSCopying for PHCollectionList {}
impl INSObject for PHCollectionList {}
impl PNSObject for PHCollectionList {}
impl IPHCollectionList for PHCollectionList {}
pub trait IPHCollectionList: Sized + std::ops::Deref {
    unsafe fn collectionListType(&self) -> PHCollectionListType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collectionListType)
    }
    unsafe fn collectionListSubtype(&self) -> PHCollectionListSubtype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collectionListSubtype)
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
    unsafe fn localizedLocationNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedLocationNames)
    }
    unsafe fn fetchCollectionListsContainingCollection_options_(
        collection: PHCollection,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap(), fetchCollectionListsContainingCollection : collection, options : options)
    }
    unsafe fn fetchCollectionListsWithLocalIdentifiers_options_(
        identifiers: NSArray,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap(), fetchCollectionListsWithLocalIdentifiers : identifiers, options : options)
    }
    unsafe fn fetchCollectionListsWithType_subtype_options_(
        collectionListType: PHCollectionListType,
        subtype: PHCollectionListSubtype,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap(), fetchCollectionListsWithType : collectionListType, subtype : subtype, options : options)
    }
    unsafe fn fetchMomentListsWithSubtype_containingMoment_options_(
        momentListSubtype: PHCollectionListSubtype,
        moment: PHAssetCollection,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap(), fetchMomentListsWithSubtype : momentListSubtype, containingMoment : moment, options : options)
    }
    unsafe fn fetchMomentListsWithSubtype_options_(
        momentListSubtype: PHCollectionListSubtype,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap(), fetchMomentListsWithSubtype : momentListSubtype, options : options)
    }
    unsafe fn transientCollectionListWithCollections_title_(
        collections: NSArray,
        title: NSString,
    ) -> PHCollectionList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap(), transientCollectionListWithCollections : collections, title : title)
    }
    unsafe fn transientCollectionListWithCollectionsFetchResult_title_(
        fetchResult: PHFetchResult,
        title: NSString,
    ) -> PHCollectionList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionList").unwrap(), transientCollectionListWithCollectionsFetchResult : fetchResult, title : title)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHFetchOptions(pub id);
impl std::ops::Deref for PHFetchOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHFetchOptions {}
impl PHFetchOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHFetchOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for PHFetchOptions {}
impl INSObject for PHFetchOptions {}
impl PNSObject for PHFetchOptions {}
impl std::convert::TryFrom<NSObject> for PHFetchOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHFetchOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHFetchOptions").unwrap()) };
        if is_kind_of {
            Ok(PHFetchOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHFetchOptions")
        }
    }
}
impl IPHFetchOptions for PHFetchOptions {}
pub trait IPHFetchOptions: Sized + std::ops::Deref {
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
    unsafe fn includeHiddenAssets(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeHiddenAssets)
    }
    unsafe fn setIncludeHiddenAssets_(&self, includeHiddenAssets: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeHiddenAssets : includeHiddenAssets)
    }
    unsafe fn includeAllBurstAssets(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeAllBurstAssets)
    }
    unsafe fn setIncludeAllBurstAssets_(&self, includeAllBurstAssets: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeAllBurstAssets : includeAllBurstAssets)
    }
    unsafe fn includeAssetSourceTypes(&self) -> PHAssetSourceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeAssetSourceTypes)
    }
    unsafe fn setIncludeAssetSourceTypes_(&self, includeAssetSourceTypes: PHAssetSourceType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeAssetSourceTypes : includeAssetSourceTypes)
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
    unsafe fn wantsIncrementalChangeDetails(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsIncrementalChangeDetails)
    }
    unsafe fn setWantsIncrementalChangeDetails_(&self, wantsIncrementalChangeDetails: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsIncrementalChangeDetails : wantsIncrementalChangeDetails)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHChange(pub id);
impl std::ops::Deref for PHChange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHChange {}
impl PHChange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHChange").unwrap(), alloc) })
    }
}
impl INSObject for PHChange {}
impl PNSObject for PHChange {}
impl std::convert::TryFrom<NSObject> for PHChange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHChange, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHChange").unwrap()) };
        if is_kind_of {
            Ok(PHChange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHChange")
        }
    }
}
impl IPHChange for PHChange {}
pub trait IPHChange: Sized + std::ops::Deref {
    unsafe fn changeDetailsForObject_(&self, object: PHObject) -> PHObjectChangeDetails
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeDetailsForObject : object)
    }
    unsafe fn changeDetailsForFetchResult_(
        &self,
        object: PHFetchResult,
    ) -> PHFetchResultChangeDetails
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeDetailsForFetchResult : object)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHObjectChangeDetails(pub id);
impl std::ops::Deref for PHObjectChangeDetails {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHObjectChangeDetails {}
impl PHObjectChangeDetails {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHObjectChangeDetails").unwrap(), alloc) })
    }
}
impl INSObject for PHObjectChangeDetails {}
impl PNSObject for PHObjectChangeDetails {}
impl std::convert::TryFrom<NSObject> for PHObjectChangeDetails {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHObjectChangeDetails, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHObjectChangeDetails").unwrap()) };
        if is_kind_of {
            Ok(PHObjectChangeDetails(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHObjectChangeDetails")
        }
    }
}
impl<ObjectType: 'static> IPHObjectChangeDetails<ObjectType> for PHObjectChangeDetails {}
pub trait IPHObjectChangeDetails<ObjectType: 'static>: Sized + std::ops::Deref {
    unsafe fn objectBeforeChanges(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectBeforeChanges)
    }
    unsafe fn objectAfterChanges(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectAfterChanges)
    }
    unsafe fn assetContentChanged(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetContentChanged)
    }
    unsafe fn objectWasDeleted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectWasDeleted)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHFetchResultChangeDetails(pub id);
impl std::ops::Deref for PHFetchResultChangeDetails {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHFetchResultChangeDetails {}
impl PHFetchResultChangeDetails {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHFetchResultChangeDetails").unwrap(), alloc) })
    }
}
impl INSObject for PHFetchResultChangeDetails {}
impl PNSObject for PHFetchResultChangeDetails {}
impl std::convert::TryFrom<NSObject> for PHFetchResultChangeDetails {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHFetchResultChangeDetails, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHFetchResultChangeDetails").unwrap()) };
        if is_kind_of {
            Ok(PHFetchResultChangeDetails(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHFetchResultChangeDetails")
        }
    }
}
impl<ObjectType: 'static> IPHFetchResultChangeDetails<ObjectType> for PHFetchResultChangeDetails {}
pub trait IPHFetchResultChangeDetails<ObjectType: 'static>: Sized + std::ops::Deref {
    unsafe fn enumerateMovesWithBlock_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateMovesWithBlock : handler)
    }
    unsafe fn fetchResultBeforeChanges(&self) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchResultBeforeChanges)
    }
    unsafe fn fetchResultAfterChanges(&self) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchResultAfterChanges)
    }
    unsafe fn hasIncrementalChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasIncrementalChanges)
    }
    unsafe fn removedIndexes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removedIndexes)
    }
    unsafe fn removedObjects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removedObjects)
    }
    unsafe fn insertedIndexes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insertedIndexes)
    }
    unsafe fn insertedObjects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insertedObjects)
    }
    unsafe fn changedIndexes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedIndexes)
    }
    unsafe fn changedObjects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changedObjects)
    }
    unsafe fn hasMoves(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasMoves)
    }
    unsafe fn changeDetailsFromFetchResult_toFetchResult_changedObjects_(
        fromResult: PHFetchResult,
        toResult: PHFetchResult,
        changedObjects: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHFetchResultChangeDetails").unwrap(), changeDetailsFromFetchResult : fromResult, toFetchResult : toResult, changedObjects : changedObjects)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPersistentChange(pub id);
impl std::ops::Deref for PHPersistentChange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPersistentChange {}
impl PHPersistentChange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPersistentChange").unwrap(), alloc) })
    }
}
impl INSObject for PHPersistentChange {}
impl PNSObject for PHPersistentChange {}
impl std::convert::TryFrom<NSObject> for PHPersistentChange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPersistentChange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPersistentChange").unwrap()) };
        if is_kind_of {
            Ok(PHPersistentChange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPersistentChange")
        }
    }
}
impl IPHPersistentChange for PHPersistentChange {}
pub trait IPHPersistentChange: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn changeDetailsForObjectType_error_(
        &self,
        objectType: PHObjectType,
        error: *mut NSError,
    ) -> PHPersistentObjectChangeDetails
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeDetailsForObjectType : objectType, error : error)
    }
    unsafe fn changeToken(&self) -> PHPersistentChangeToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeToken)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPersistentChange").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPersistentChangeToken(pub id);
impl std::ops::Deref for PHPersistentChangeToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPersistentChangeToken {}
impl PHPersistentChangeToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPersistentChangeToken").unwrap(), alloc) })
    }
}
impl PNSCopying for PHPersistentChangeToken {}
impl PNSSecureCoding for PHPersistentChangeToken {}
impl INSObject for PHPersistentChangeToken {}
impl PNSObject for PHPersistentChangeToken {}
impl std::convert::TryFrom<NSObject> for PHPersistentChangeToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPersistentChangeToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPersistentChangeToken").unwrap()) };
        if is_kind_of {
            Ok(PHPersistentChangeToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPersistentChangeToken")
        }
    }
}
impl IPHPersistentChangeToken for PHPersistentChangeToken {}
pub trait IPHPersistentChangeToken: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPersistentChangeToken").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPersistentChangeFetchResult(pub id);
impl std::ops::Deref for PHPersistentChangeFetchResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPersistentChangeFetchResult {}
impl PHPersistentChangeFetchResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPersistentChangeFetchResult").unwrap(), alloc) })
    }
}
impl INSObject for PHPersistentChangeFetchResult {}
impl PNSObject for PHPersistentChangeFetchResult {}
impl std::convert::TryFrom<NSObject> for PHPersistentChangeFetchResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPersistentChangeFetchResult, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPersistentChangeFetchResult").unwrap())
        };
        if is_kind_of {
            Ok(PHPersistentChangeFetchResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPersistentChangeFetchResult")
        }
    }
}
impl IPHPersistentChangeFetchResult for PHPersistentChangeFetchResult {}
pub trait IPHPersistentChangeFetchResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn enumerateChangesWithBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateChangesWithBlock : block)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPersistentChangeFetchResult").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPersistentObjectChangeDetails(pub id);
impl std::ops::Deref for PHPersistentObjectChangeDetails {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPersistentObjectChangeDetails {}
impl PHPersistentObjectChangeDetails {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPersistentObjectChangeDetails").unwrap(), alloc) })
    }
}
impl INSObject for PHPersistentObjectChangeDetails {}
impl PNSObject for PHPersistentObjectChangeDetails {}
impl std::convert::TryFrom<NSObject> for PHPersistentObjectChangeDetails {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPersistentObjectChangeDetails, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPersistentObjectChangeDetails").unwrap())
        };
        if is_kind_of {
            Ok(PHPersistentObjectChangeDetails(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPersistentObjectChangeDetails")
        }
    }
}
impl IPHPersistentObjectChangeDetails for PHPersistentObjectChangeDetails {}
pub trait IPHPersistentObjectChangeDetails: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn objectType(&self) -> PHObjectType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectType)
    }
    unsafe fn insertedLocalIdentifiers(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insertedLocalIdentifiers)
    }
    unsafe fn updatedLocalIdentifiers(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updatedLocalIdentifiers)
    }
    unsafe fn deletedLocalIdentifiers(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deletedLocalIdentifiers)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPersistentObjectChangeDetails").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHChangeRequest(pub id);
impl std::ops::Deref for PHChangeRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHChangeRequest {}
impl PHChangeRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHChangeRequest").unwrap(), alloc) })
    }
}
impl INSObject for PHChangeRequest {}
impl PNSObject for PHChangeRequest {}
impl std::convert::TryFrom<NSObject> for PHChangeRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHChangeRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHChangeRequest").unwrap()) };
        if is_kind_of {
            Ok(PHChangeRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHChangeRequest")
        }
    }
}
impl IPHChangeRequest for PHChangeRequest {}
pub trait IPHChangeRequest: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHContentEditingOutput(pub id);
impl std::ops::Deref for PHContentEditingOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHContentEditingOutput {}
impl PHContentEditingOutput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHContentEditingOutput").unwrap(), alloc) })
    }
}
impl INSObject for PHContentEditingOutput {}
impl PNSObject for PHContentEditingOutput {}
impl std::convert::TryFrom<NSObject> for PHContentEditingOutput {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHContentEditingOutput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHContentEditingOutput").unwrap()) };
        if is_kind_of {
            Ok(PHContentEditingOutput(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHContentEditingOutput")
        }
    }
}
impl IPHContentEditingOutput for PHContentEditingOutput {}
pub trait IPHContentEditingOutput: Sized + std::ops::Deref {
    unsafe fn initWithContentEditingInput_(
        &self,
        contentEditingInput: PHContentEditingInput,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentEditingInput : contentEditingInput)
    }
    unsafe fn renderedContentURLForType_error_(&self, type_: UTType, error: *mut NSError) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderedContentURLForType : type_, error : error)
    }
    unsafe fn adjustmentData(&self) -> PHAdjustmentData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adjustmentData)
    }
    unsafe fn setAdjustmentData_(&self, adjustmentData: PHAdjustmentData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdjustmentData : adjustmentData)
    }
    unsafe fn renderedContentURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderedContentURL)
    }
    unsafe fn defaultRenderedContentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultRenderedContentType)
    }
    unsafe fn supportedRenderedContentTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedRenderedContentTypes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetChangeRequest(pub id);
impl std::ops::Deref for PHAssetChangeRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetChangeRequest {}
impl PHAssetChangeRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetChangeRequest").unwrap(), alloc) })
    }
}
impl IPHChangeRequest for PHAssetChangeRequest {}
impl From<PHAssetChangeRequest> for PHChangeRequest {
    fn from(child: PHAssetChangeRequest) -> PHChangeRequest {
        PHChangeRequest(child.0)
    }
}
impl std::convert::TryFrom<PHChangeRequest> for PHAssetChangeRequest {
    type Error = &'static str;
    fn try_from(parent: PHChangeRequest) -> Result<PHAssetChangeRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetChangeRequest").unwrap()) };
        if is_kind_of {
            Ok(PHAssetChangeRequest(parent.0))
        } else {
            Err("This PHChangeRequest cannot be downcasted to PHAssetChangeRequest")
        }
    }
}
impl INSObject for PHAssetChangeRequest {}
impl PNSObject for PHAssetChangeRequest {}
impl IPHAssetChangeRequest for PHAssetChangeRequest {}
pub trait IPHAssetChangeRequest: Sized + std::ops::Deref {
    unsafe fn revertAssetContentToOriginal(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revertAssetContentToOriginal)
    }
    unsafe fn placeholderForCreatedAsset(&self) -> PHObjectPlaceholder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placeholderForCreatedAsset)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn setCreationDate_(&self, creationDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCreationDate : creationDate)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn setLocation_(&self, location: CLLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocation : location)
    }
    unsafe fn isFavorite(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFavorite)
    }
    unsafe fn setFavorite_(&self, favorite: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFavorite : favorite)
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
    unsafe fn contentEditingOutput(&self) -> PHContentEditingOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentEditingOutput)
    }
    unsafe fn setContentEditingOutput_(&self, contentEditingOutput: PHContentEditingOutput)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentEditingOutput : contentEditingOutput)
    }
    unsafe fn creationRequestForAssetFromImage_(image: NSImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetChangeRequest").unwrap(), creationRequestForAssetFromImage : image)
    }
    unsafe fn creationRequestForAssetFromImageAtFileURL_(fileURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetChangeRequest").unwrap(), creationRequestForAssetFromImageAtFileURL : fileURL)
    }
    unsafe fn creationRequestForAssetFromVideoAtFileURL_(fileURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetChangeRequest").unwrap(), creationRequestForAssetFromVideoAtFileURL : fileURL)
    }
    unsafe fn deleteAssets_(assets: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetChangeRequest").unwrap(), deleteAssets : assets)
    }
    unsafe fn changeRequestForAsset_(asset: PHAsset) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetChangeRequest").unwrap(), changeRequestForAsset : asset)
    }
}
pub type PHContentEditingInputRequestID = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHContentEditingInputRequestOptions(pub id);
impl std::ops::Deref for PHContentEditingInputRequestOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHContentEditingInputRequestOptions {}
impl PHContentEditingInputRequestOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHContentEditingInputRequestOptions").unwrap(), alloc) })
    }
}
impl INSObject for PHContentEditingInputRequestOptions {}
impl PNSObject for PHContentEditingInputRequestOptions {}
impl std::convert::TryFrom<NSObject> for PHContentEditingInputRequestOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHContentEditingInputRequestOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHContentEditingInputRequestOptions").unwrap())
        };
        if is_kind_of {
            Ok(PHContentEditingInputRequestOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHContentEditingInputRequestOptions")
        }
    }
}
impl IPHContentEditingInputRequestOptions for PHContentEditingInputRequestOptions {}
pub trait IPHContentEditingInputRequestOptions: Sized + std::ops::Deref {
    unsafe fn canHandleAdjustmentData(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canHandleAdjustmentData)
    }
    unsafe fn setCanHandleAdjustmentData_(
        &self,
        canHandleAdjustmentData: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanHandleAdjustmentData : canHandleAdjustmentData)
    }
    unsafe fn isNetworkAccessAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNetworkAccessAllowed)
    }
    unsafe fn setNetworkAccessAllowed_(&self, networkAccessAllowed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkAccessAllowed : networkAccessAllowed)
    }
    unsafe fn progressHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progressHandler)
    }
    unsafe fn setProgressHandler_(&self, progressHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgressHandler : progressHandler)
    }
}
impl PHAsset_PHContentEditingInput for PHAsset {}
pub trait PHAsset_PHContentEditingInput: Sized + std::ops::Deref {
    unsafe fn requestContentEditingInputWithOptions_completionHandler_(
        &self,
        options: PHContentEditingInputRequestOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> PHContentEditingInputRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestContentEditingInputWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn cancelContentEditingInputRequest_(&self, requestID: PHContentEditingInputRequestID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelContentEditingInputRequest : requestID)
    }
}
impl PHContentEditingOutput_PHAssetChangeRequest for PHContentEditingOutput {}
pub trait PHContentEditingOutput_PHAssetChangeRequest: Sized + std::ops::Deref {
    unsafe fn initWithPlaceholderForCreatedAsset_(
        &self,
        placeholderForCreatedAsset: PHObjectPlaceholder,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlaceholderForCreatedAsset : placeholderForCreatedAsset)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetResourceCreationOptions(pub id);
impl std::ops::Deref for PHAssetResourceCreationOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetResourceCreationOptions {}
impl PHAssetResourceCreationOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceCreationOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for PHAssetResourceCreationOptions {}
impl INSObject for PHAssetResourceCreationOptions {}
impl PNSObject for PHAssetResourceCreationOptions {}
impl std::convert::TryFrom<NSObject> for PHAssetResourceCreationOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHAssetResourceCreationOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetResourceCreationOptions").unwrap())
        };
        if is_kind_of {
            Ok(PHAssetResourceCreationOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHAssetResourceCreationOptions")
        }
    }
}
impl IPHAssetResourceCreationOptions for PHAssetResourceCreationOptions {}
pub trait IPHAssetResourceCreationOptions: Sized + std::ops::Deref {
    unsafe fn originalFilename(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalFilename)
    }
    unsafe fn setOriginalFilename_(&self, originalFilename: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOriginalFilename : originalFilename)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn setContentType_(&self, contentType: UTType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentType : contentType)
    }
    unsafe fn uniformTypeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniformTypeIdentifier)
    }
    unsafe fn setUniformTypeIdentifier_(&self, uniformTypeIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUniformTypeIdentifier : uniformTypeIdentifier)
    }
    unsafe fn shouldMoveFile(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldMoveFile)
    }
    unsafe fn setShouldMoveFile_(&self, shouldMoveFile: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldMoveFile : shouldMoveFile)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetCreationRequest(pub id);
impl std::ops::Deref for PHAssetCreationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetCreationRequest {}
impl PHAssetCreationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCreationRequest").unwrap(), alloc) })
    }
}
impl IPHAssetChangeRequest for PHAssetCreationRequest {}
impl From<PHAssetCreationRequest> for PHAssetChangeRequest {
    fn from(child: PHAssetCreationRequest) -> PHAssetChangeRequest {
        PHAssetChangeRequest(child.0)
    }
}
impl std::convert::TryFrom<PHAssetChangeRequest> for PHAssetCreationRequest {
    type Error = &'static str;
    fn try_from(parent: PHAssetChangeRequest) -> Result<PHAssetCreationRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetCreationRequest").unwrap()) };
        if is_kind_of {
            Ok(PHAssetCreationRequest(parent.0))
        } else {
            Err("This PHAssetChangeRequest cannot be downcasted to PHAssetCreationRequest")
        }
    }
}
impl IPHChangeRequest for PHAssetCreationRequest {}
impl INSObject for PHAssetCreationRequest {}
impl PNSObject for PHAssetCreationRequest {}
impl IPHAssetCreationRequest for PHAssetCreationRequest {}
pub trait IPHAssetCreationRequest: Sized + std::ops::Deref {
    unsafe fn addResourceWithType_fileURL_options_(
        &self,
        type_: PHAssetResourceType,
        fileURL: NSURL,
        options: PHAssetResourceCreationOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addResourceWithType : type_, fileURL : fileURL, options : options)
    }
    unsafe fn addResourceWithType_data_options_(
        &self,
        type_: PHAssetResourceType,
        data: NSData,
        options: PHAssetResourceCreationOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addResourceWithType : type_, data : data, options : options)
    }
    unsafe fn creationRequestForAsset() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCreationRequest").unwrap(), creationRequestForAsset)
    }
    unsafe fn supportsAssetResourceTypes_(types: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCreationRequest").unwrap(), supportsAssetResourceTypes : types)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetCollectionChangeRequest(pub id);
impl std::ops::Deref for PHAssetCollectionChangeRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetCollectionChangeRequest {}
impl PHAssetCollectionChangeRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollectionChangeRequest").unwrap(), alloc) })
    }
}
impl IPHChangeRequest for PHAssetCollectionChangeRequest {}
impl std::convert::TryFrom<PHChangeRequest> for PHAssetCollectionChangeRequest {
    type Error = &'static str;
    fn try_from(parent: PHChangeRequest) -> Result<PHAssetCollectionChangeRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetCollectionChangeRequest").unwrap())
        };
        if is_kind_of {
            Ok(PHAssetCollectionChangeRequest(parent.0))
        } else {
            Err("This PHChangeRequest cannot be downcasted to PHAssetCollectionChangeRequest")
        }
    }
}
impl INSObject for PHAssetCollectionChangeRequest {}
impl PNSObject for PHAssetCollectionChangeRequest {}
impl IPHAssetCollectionChangeRequest for PHAssetCollectionChangeRequest {}
pub trait IPHAssetCollectionChangeRequest: Sized + std::ops::Deref {
    unsafe fn addAssets_(&self, assets: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAssets : assets)
    }
    unsafe fn insertAssets_atIndexes_(&self, assets: *mut u64, indexes: NSIndexSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertAssets : assets, atIndexes : indexes)
    }
    unsafe fn removeAssets_(&self, assets: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAssets : assets)
    }
    unsafe fn removeAssetsAtIndexes_(&self, indexes: NSIndexSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAssetsAtIndexes : indexes)
    }
    unsafe fn replaceAssetsAtIndexes_withAssets_(&self, indexes: NSIndexSet, assets: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceAssetsAtIndexes : indexes, withAssets : assets)
    }
    unsafe fn moveAssetsAtIndexes_toIndex_(&self, fromIndexes: NSIndexSet, toIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveAssetsAtIndexes : fromIndexes, toIndex : toIndex)
    }
    unsafe fn placeholderForCreatedAssetCollection(&self) -> PHObjectPlaceholder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placeholderForCreatedAssetCollection)
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
    unsafe fn creationRequestForAssetCollectionWithTitle_(title: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollectionChangeRequest").unwrap(), creationRequestForAssetCollectionWithTitle : title)
    }
    unsafe fn deleteAssetCollections_(assetCollections: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollectionChangeRequest").unwrap(), deleteAssetCollections : assetCollections)
    }
    unsafe fn changeRequestForAssetCollection_(assetCollection: PHAssetCollection) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollectionChangeRequest").unwrap(), changeRequestForAssetCollection : assetCollection)
    }
    unsafe fn changeRequestForAssetCollection_assets_(
        assetCollection: PHAssetCollection,
        assets: PHFetchResult,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetCollectionChangeRequest").unwrap(), changeRequestForAssetCollection : assetCollection, assets : assets)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetResourceUploadJob(pub id);
impl std::ops::Deref for PHAssetResourceUploadJob {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetResourceUploadJob {}
impl PHAssetResourceUploadJob {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceUploadJob").unwrap(), alloc) })
    }
}
impl IPHObject for PHAssetResourceUploadJob {}
impl PNSCopying for PHAssetResourceUploadJob {}
impl std::convert::TryFrom<PHObject> for PHAssetResourceUploadJob {
    type Error = &'static str;
    fn try_from(parent: PHObject) -> Result<PHAssetResourceUploadJob, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetResourceUploadJob").unwrap()) };
        if is_kind_of {
            Ok(PHAssetResourceUploadJob(parent.0))
        } else {
            Err("This PHObject cannot be downcasted to PHAssetResourceUploadJob")
        }
    }
}
impl INSObject for PHAssetResourceUploadJob {}
impl PNSObject for PHAssetResourceUploadJob {}
impl IPHAssetResourceUploadJob for PHAssetResourceUploadJob {}
pub trait IPHAssetResourceUploadJob: Sized + std::ops::Deref {
    unsafe fn resource(&self) -> PHAssetResource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resource)
    }
    unsafe fn destination(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn state(&self) -> PHAssetResourceUploadJobState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn fetchJobsWithAction_options_(
        action: PHAssetResourceUploadJobAction,
        options: PHFetchOptions,
    ) -> PHFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceUploadJob").unwrap(), fetchJobsWithAction : action, options : options)
    }
    unsafe fn jobLimit() -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceUploadJob").unwrap(), jobLimit)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetResourceUploadJobChangeRequest(pub id);
impl std::ops::Deref for PHAssetResourceUploadJobChangeRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetResourceUploadJobChangeRequest {}
impl PHAssetResourceUploadJobChangeRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceUploadJobChangeRequest").unwrap(), alloc) })
    }
}
impl IPHChangeRequest for PHAssetResourceUploadJobChangeRequest {}
impl std::convert::TryFrom<PHChangeRequest> for PHAssetResourceUploadJobChangeRequest {
    type Error = &'static str;
    fn try_from(
        parent: PHChangeRequest,
    ) -> Result<PHAssetResourceUploadJobChangeRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetResourceUploadJobChangeRequest").unwrap())
        };
        if is_kind_of {
            Ok(PHAssetResourceUploadJobChangeRequest(parent.0))
        } else {
            Err ("This PHChangeRequest cannot be downcasted to PHAssetResourceUploadJobChangeRequest" ,)
        }
    }
}
impl INSObject for PHAssetResourceUploadJobChangeRequest {}
impl PNSObject for PHAssetResourceUploadJobChangeRequest {}
impl IPHAssetResourceUploadJobChangeRequest for PHAssetResourceUploadJobChangeRequest {}
pub trait IPHAssetResourceUploadJobChangeRequest: Sized + std::ops::Deref {
    unsafe fn acknowledge(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acknowledge)
    }
    unsafe fn retryWithDestination_(&self, destination: NSURLRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retryWithDestination : destination)
    }
    unsafe fn createJobWithDestination_resource_(
        destination: NSURLRequest,
        resource: PHAssetResource,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceUploadJobChangeRequest").unwrap(), createJobWithDestination : destination, resource : resource)
    }
    unsafe fn changeRequestForUploadJob_(job: PHAssetResourceUploadJob) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceUploadJobChangeRequest").unwrap(), changeRequestForUploadJob : job)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHCollectionListChangeRequest(pub id);
impl std::ops::Deref for PHCollectionListChangeRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHCollectionListChangeRequest {}
impl PHCollectionListChangeRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionListChangeRequest").unwrap(), alloc) })
    }
}
impl IPHChangeRequest for PHCollectionListChangeRequest {}
impl std::convert::TryFrom<PHChangeRequest> for PHCollectionListChangeRequest {
    type Error = &'static str;
    fn try_from(parent: PHChangeRequest) -> Result<PHCollectionListChangeRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHCollectionListChangeRequest").unwrap())
        };
        if is_kind_of {
            Ok(PHCollectionListChangeRequest(parent.0))
        } else {
            Err("This PHChangeRequest cannot be downcasted to PHCollectionListChangeRequest")
        }
    }
}
impl INSObject for PHCollectionListChangeRequest {}
impl PNSObject for PHCollectionListChangeRequest {}
impl IPHCollectionListChangeRequest for PHCollectionListChangeRequest {}
pub trait IPHCollectionListChangeRequest: Sized + std::ops::Deref {
    unsafe fn addChildCollections_(&self, collections: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addChildCollections : collections)
    }
    unsafe fn insertChildCollections_atIndexes_(&self, collections: *mut u64, indexes: NSIndexSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertChildCollections : collections, atIndexes : indexes)
    }
    unsafe fn removeChildCollections_(&self, collections: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeChildCollections : collections)
    }
    unsafe fn removeChildCollectionsAtIndexes_(&self, indexes: NSIndexSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeChildCollectionsAtIndexes : indexes)
    }
    unsafe fn replaceChildCollectionsAtIndexes_withChildCollections_(
        &self,
        indexes: NSIndexSet,
        collections: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceChildCollectionsAtIndexes : indexes, withChildCollections : collections)
    }
    unsafe fn moveChildCollectionsAtIndexes_toIndex_(
        &self,
        indexes: NSIndexSet,
        toIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveChildCollectionsAtIndexes : indexes, toIndex : toIndex)
    }
    unsafe fn placeholderForCreatedCollectionList(&self) -> PHObjectPlaceholder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placeholderForCreatedCollectionList)
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
    unsafe fn creationRequestForCollectionListWithTitle_(title: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionListChangeRequest").unwrap(), creationRequestForCollectionListWithTitle : title)
    }
    unsafe fn deleteCollectionLists_(collectionLists: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionListChangeRequest").unwrap(), deleteCollectionLists : collectionLists)
    }
    unsafe fn changeRequestForCollectionList_(collectionList: PHCollectionList) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionListChangeRequest").unwrap(), changeRequestForCollectionList : collectionList)
    }
    unsafe fn changeRequestForCollectionList_childCollections_(
        collectionList: PHCollectionList,
        childCollections: PHFetchResult,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionListChangeRequest").unwrap(), changeRequestForCollectionList : collectionList, childCollections : childCollections)
    }
    unsafe fn changeRequestForTopLevelCollectionListUserCollections_(
        childCollections: PHFetchResult,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCollectionListChangeRequest").unwrap(), changeRequestForTopLevelCollectionListUserCollections : childCollections)
    }
}
pub type PHLivePhotoFrameProcessingBlock = *mut ::std::os::raw::c_void;
pub type PHLivePhotoEditingOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHLivePhotoEditingContext(pub id);
impl std::ops::Deref for PHLivePhotoEditingContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHLivePhotoEditingContext {}
impl PHLivePhotoEditingContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHLivePhotoEditingContext").unwrap(), alloc) })
    }
}
impl INSObject for PHLivePhotoEditingContext {}
impl PNSObject for PHLivePhotoEditingContext {}
impl std::convert::TryFrom<NSObject> for PHLivePhotoEditingContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHLivePhotoEditingContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHLivePhotoEditingContext").unwrap()) };
        if is_kind_of {
            Ok(PHLivePhotoEditingContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHLivePhotoEditingContext")
        }
    }
}
impl IPHLivePhotoEditingContext for PHLivePhotoEditingContext {}
pub trait IPHLivePhotoEditingContext: Sized + std::ops::Deref {
    unsafe fn initWithLivePhotoEditingInput_(
        &self,
        livePhotoInput: PHContentEditingInput,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLivePhotoEditingInput : livePhotoInput)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn prepareLivePhotoForPlaybackWithTargetSize_options_completionHandler_(
        &self,
        targetSize: CGSize,
        options: NSDictionary,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareLivePhotoForPlaybackWithTargetSize : targetSize, options : options, completionHandler : handler)
    }
    unsafe fn saveLivePhotoToOutput_options_completionHandler_(
        &self,
        output: PHContentEditingOutput,
        options: NSDictionary,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveLivePhotoToOutput : output, options : options, completionHandler : handler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn fullSizeImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullSizeImage)
    }
    unsafe fn duration(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn photoTime(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, photoTime)
    }
    unsafe fn frameProcessor(&self) -> PHLivePhotoFrameProcessingBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameProcessor)
    }
    unsafe fn setFrameProcessor_(&self, frameProcessor: PHLivePhotoFrameProcessingBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameProcessor : frameProcessor)
    }
    unsafe fn audioVolume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioVolume)
    }
    unsafe fn setAudioVolume_(&self, audioVolume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioVolume : audioVolume)
    }
    unsafe fn orientation(&self) -> CGImagePropertyOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
}
pub type PHLivePhotoFrameType = NSInteger;
pub trait PPHLivePhotoFrame: Sized + std::ops::Deref {
    unsafe fn image(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn time(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn type_(&self) -> PHLivePhotoFrameType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn renderScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderScale)
    }
}
pub type PHLivePhotoEditingErrorCode = NSInteger;
pub type UIImageOrientation = NSInteger;
pub type PHImageRequestOptionsVersion = NSInteger;
pub type PHImageRequestOptionsDeliveryMode = NSInteger;
pub type PHImageRequestOptionsResizeMode = NSInteger;
pub type PHAssetImageProgressHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHImageRequestOptions(pub id);
impl std::ops::Deref for PHImageRequestOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHImageRequestOptions {}
impl PHImageRequestOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHImageRequestOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for PHImageRequestOptions {}
impl INSObject for PHImageRequestOptions {}
impl PNSObject for PHImageRequestOptions {}
impl std::convert::TryFrom<NSObject> for PHImageRequestOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHImageRequestOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHImageRequestOptions").unwrap()) };
        if is_kind_of {
            Ok(PHImageRequestOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHImageRequestOptions")
        }
    }
}
impl IPHImageRequestOptions for PHImageRequestOptions {}
pub trait IPHImageRequestOptions: Sized + std::ops::Deref {
    unsafe fn version(&self) -> PHImageRequestOptionsVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn setVersion_(&self, version: PHImageRequestOptionsVersion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersion : version)
    }
    unsafe fn deliveryMode(&self) -> PHImageRequestOptionsDeliveryMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deliveryMode)
    }
    unsafe fn setDeliveryMode_(&self, deliveryMode: PHImageRequestOptionsDeliveryMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeliveryMode : deliveryMode)
    }
    unsafe fn resizeMode(&self) -> PHImageRequestOptionsResizeMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resizeMode)
    }
    unsafe fn setResizeMode_(&self, resizeMode: PHImageRequestOptionsResizeMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResizeMode : resizeMode)
    }
    unsafe fn normalizedCropRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedCropRect)
    }
    unsafe fn setNormalizedCropRect_(&self, normalizedCropRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalizedCropRect : normalizedCropRect)
    }
    unsafe fn isNetworkAccessAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNetworkAccessAllowed)
    }
    unsafe fn setNetworkAccessAllowed_(&self, networkAccessAllowed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkAccessAllowed : networkAccessAllowed)
    }
    unsafe fn isSynchronous(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSynchronous)
    }
    unsafe fn setSynchronous_(&self, synchronous: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSynchronous : synchronous)
    }
    unsafe fn progressHandler(&self) -> PHAssetImageProgressHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progressHandler)
    }
    unsafe fn setProgressHandler_(&self, progressHandler: PHAssetImageProgressHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgressHandler : progressHandler)
    }
    unsafe fn allowSecondaryDegradedImage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowSecondaryDegradedImage)
    }
    unsafe fn setAllowSecondaryDegradedImage_(&self, allowSecondaryDegradedImage: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowSecondaryDegradedImage : allowSecondaryDegradedImage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHLivePhotoRequestOptions(pub id);
impl std::ops::Deref for PHLivePhotoRequestOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHLivePhotoRequestOptions {}
impl PHLivePhotoRequestOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHLivePhotoRequestOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for PHLivePhotoRequestOptions {}
impl INSObject for PHLivePhotoRequestOptions {}
impl PNSObject for PHLivePhotoRequestOptions {}
impl std::convert::TryFrom<NSObject> for PHLivePhotoRequestOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHLivePhotoRequestOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHLivePhotoRequestOptions").unwrap()) };
        if is_kind_of {
            Ok(PHLivePhotoRequestOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHLivePhotoRequestOptions")
        }
    }
}
impl IPHLivePhotoRequestOptions for PHLivePhotoRequestOptions {}
pub trait IPHLivePhotoRequestOptions: Sized + std::ops::Deref {
    unsafe fn version(&self) -> PHImageRequestOptionsVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn setVersion_(&self, version: PHImageRequestOptionsVersion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersion : version)
    }
    unsafe fn deliveryMode(&self) -> PHImageRequestOptionsDeliveryMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deliveryMode)
    }
    unsafe fn setDeliveryMode_(&self, deliveryMode: PHImageRequestOptionsDeliveryMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeliveryMode : deliveryMode)
    }
    unsafe fn isNetworkAccessAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNetworkAccessAllowed)
    }
    unsafe fn setNetworkAccessAllowed_(&self, networkAccessAllowed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkAccessAllowed : networkAccessAllowed)
    }
    unsafe fn progressHandler(&self) -> PHAssetImageProgressHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progressHandler)
    }
    unsafe fn setProgressHandler_(&self, progressHandler: PHAssetImageProgressHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgressHandler : progressHandler)
    }
}
pub type PHVideoRequestOptionsVersion = NSInteger;
pub type PHVideoRequestOptionsDeliveryMode = NSInteger;
pub type PHAssetVideoProgressHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHVideoRequestOptions(pub id);
impl std::ops::Deref for PHVideoRequestOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHVideoRequestOptions {}
impl PHVideoRequestOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHVideoRequestOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for PHVideoRequestOptions {}
impl INSObject for PHVideoRequestOptions {}
impl PNSObject for PHVideoRequestOptions {}
impl std::convert::TryFrom<NSObject> for PHVideoRequestOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHVideoRequestOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHVideoRequestOptions").unwrap()) };
        if is_kind_of {
            Ok(PHVideoRequestOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHVideoRequestOptions")
        }
    }
}
impl IPHVideoRequestOptions for PHVideoRequestOptions {}
pub trait IPHVideoRequestOptions: Sized + std::ops::Deref {
    unsafe fn isNetworkAccessAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNetworkAccessAllowed)
    }
    unsafe fn setNetworkAccessAllowed_(&self, networkAccessAllowed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkAccessAllowed : networkAccessAllowed)
    }
    unsafe fn version(&self) -> PHVideoRequestOptionsVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn setVersion_(&self, version: PHVideoRequestOptionsVersion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersion : version)
    }
    unsafe fn deliveryMode(&self) -> PHVideoRequestOptionsDeliveryMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deliveryMode)
    }
    unsafe fn setDeliveryMode_(&self, deliveryMode: PHVideoRequestOptionsDeliveryMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeliveryMode : deliveryMode)
    }
    unsafe fn progressHandler(&self) -> PHAssetVideoProgressHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progressHandler)
    }
    unsafe fn setProgressHandler_(&self, progressHandler: PHAssetVideoProgressHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgressHandler : progressHandler)
    }
}
pub type PHImageRequestID = i32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHImageManager(pub id);
impl std::ops::Deref for PHImageManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHImageManager {}
impl PHImageManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHImageManager").unwrap(), alloc) })
    }
}
impl INSObject for PHImageManager {}
impl PNSObject for PHImageManager {}
impl std::convert::TryFrom<NSObject> for PHImageManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHImageManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHImageManager").unwrap()) };
        if is_kind_of {
            Ok(PHImageManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHImageManager")
        }
    }
}
impl IPHImageManager for PHImageManager {}
pub trait IPHImageManager: Sized + std::ops::Deref {
    unsafe fn requestImageForAsset_targetSize_contentMode_options_resultHandler_(
        &self,
        asset: PHAsset,
        targetSize: CGSize,
        contentMode: PHImageContentMode,
        options: PHImageRequestOptions,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> PHImageRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestImageForAsset : asset, targetSize : targetSize, contentMode : contentMode, options : options, resultHandler : resultHandler)
    }
    unsafe fn requestImageDataForAsset_options_resultHandler_(
        &self,
        asset: PHAsset,
        options: PHImageRequestOptions,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> PHImageRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestImageDataForAsset : asset, options : options, resultHandler : resultHandler)
    }
    unsafe fn requestImageDataAndOrientationForAsset_options_resultHandler_(
        &self,
        asset: PHAsset,
        options: PHImageRequestOptions,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> PHImageRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestImageDataAndOrientationForAsset : asset, options : options, resultHandler : resultHandler)
    }
    unsafe fn cancelImageRequest_(&self, requestID: PHImageRequestID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelImageRequest : requestID)
    }
    unsafe fn requestLivePhotoForAsset_targetSize_contentMode_options_resultHandler_(
        &self,
        asset: PHAsset,
        targetSize: CGSize,
        contentMode: PHImageContentMode,
        options: PHLivePhotoRequestOptions,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> PHImageRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestLivePhotoForAsset : asset, targetSize : targetSize, contentMode : contentMode, options : options, resultHandler : resultHandler)
    }
    unsafe fn requestPlayerItemForVideo_options_resultHandler_(
        &self,
        asset: PHAsset,
        options: PHVideoRequestOptions,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> PHImageRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestPlayerItemForVideo : asset, options : options, resultHandler : resultHandler)
    }
    unsafe fn requestExportSessionForVideo_options_exportPreset_resultHandler_(
        &self,
        asset: PHAsset,
        options: PHVideoRequestOptions,
        exportPreset: NSString,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> PHImageRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestExportSessionForVideo : asset, options : options, exportPreset : exportPreset, resultHandler : resultHandler)
    }
    unsafe fn requestAVAssetForVideo_options_resultHandler_(
        &self,
        asset: PHAsset,
        options: PHVideoRequestOptions,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> PHImageRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAVAssetForVideo : asset, options : options, resultHandler : resultHandler)
    }
    unsafe fn defaultManager() -> PHImageManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHImageManager").unwrap(), defaultManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHCachingImageManager(pub id);
impl std::ops::Deref for PHCachingImageManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHCachingImageManager {}
impl PHCachingImageManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHCachingImageManager").unwrap(), alloc) })
    }
}
impl IPHImageManager for PHCachingImageManager {}
impl From<PHCachingImageManager> for PHImageManager {
    fn from(child: PHCachingImageManager) -> PHImageManager {
        PHImageManager(child.0)
    }
}
impl std::convert::TryFrom<PHImageManager> for PHCachingImageManager {
    type Error = &'static str;
    fn try_from(parent: PHImageManager) -> Result<PHCachingImageManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHCachingImageManager").unwrap()) };
        if is_kind_of {
            Ok(PHCachingImageManager(parent.0))
        } else {
            Err("This PHImageManager cannot be downcasted to PHCachingImageManager")
        }
    }
}
impl INSObject for PHCachingImageManager {}
impl PNSObject for PHCachingImageManager {}
impl IPHCachingImageManager for PHCachingImageManager {}
pub trait IPHCachingImageManager: Sized + std::ops::Deref {
    unsafe fn startCachingImagesForAssets_targetSize_contentMode_options_(
        &self,
        assets: NSArray,
        targetSize: CGSize,
        contentMode: PHImageContentMode,
        options: PHImageRequestOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startCachingImagesForAssets : assets, targetSize : targetSize, contentMode : contentMode, options : options)
    }
    unsafe fn stopCachingImagesForAssets_targetSize_contentMode_options_(
        &self,
        assets: NSArray,
        targetSize: CGSize,
        contentMode: PHImageContentMode,
        options: PHImageRequestOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopCachingImagesForAssets : assets, targetSize : targetSize, contentMode : contentMode, options : options)
    }
    unsafe fn stopCachingImagesForAllAssets(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopCachingImagesForAllAssets)
    }
    unsafe fn allowsCachingHighQualityImages(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCachingHighQualityImages)
    }
    unsafe fn setAllowsCachingHighQualityImages_(&self, allowsCachingHighQualityImages: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsCachingHighQualityImages : allowsCachingHighQualityImages)
    }
}
pub type PHAssetResourceDataRequestID = i32;
pub type PHAssetResourceProgressHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetResourceRequestOptions(pub id);
impl std::ops::Deref for PHAssetResourceRequestOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetResourceRequestOptions {}
impl PHAssetResourceRequestOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceRequestOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for PHAssetResourceRequestOptions {}
impl INSObject for PHAssetResourceRequestOptions {}
impl PNSObject for PHAssetResourceRequestOptions {}
impl std::convert::TryFrom<NSObject> for PHAssetResourceRequestOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHAssetResourceRequestOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetResourceRequestOptions").unwrap())
        };
        if is_kind_of {
            Ok(PHAssetResourceRequestOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHAssetResourceRequestOptions")
        }
    }
}
impl IPHAssetResourceRequestOptions for PHAssetResourceRequestOptions {}
pub trait IPHAssetResourceRequestOptions: Sized + std::ops::Deref {
    unsafe fn isNetworkAccessAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNetworkAccessAllowed)
    }
    unsafe fn setNetworkAccessAllowed_(&self, networkAccessAllowed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkAccessAllowed : networkAccessAllowed)
    }
    unsafe fn progressHandler(&self) -> PHAssetResourceProgressHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progressHandler)
    }
    unsafe fn setProgressHandler_(&self, progressHandler: PHAssetResourceProgressHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgressHandler : progressHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetResourceManager(pub id);
impl std::ops::Deref for PHAssetResourceManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetResourceManager {}
impl PHAssetResourceManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceManager").unwrap(), alloc) })
    }
}
impl INSObject for PHAssetResourceManager {}
impl PNSObject for PHAssetResourceManager {}
impl std::convert::TryFrom<NSObject> for PHAssetResourceManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHAssetResourceManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetResourceManager").unwrap()) };
        if is_kind_of {
            Ok(PHAssetResourceManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHAssetResourceManager")
        }
    }
}
impl IPHAssetResourceManager for PHAssetResourceManager {}
pub trait IPHAssetResourceManager: Sized + std::ops::Deref {
    unsafe fn requestDataForAssetResource_options_dataReceivedHandler_completionHandler_(
        &self,
        resource: PHAssetResource,
        options: PHAssetResourceRequestOptions,
        handler: *mut ::std::os::raw::c_void,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> PHAssetResourceDataRequestID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDataForAssetResource : resource, options : options, dataReceivedHandler : handler, completionHandler : completionHandler)
    }
    unsafe fn writeDataForAssetResource_toFile_options_completionHandler_(
        &self,
        resource: PHAssetResource,
        fileURL: NSURL,
        options: PHAssetResourceRequestOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeDataForAssetResource : resource, toFile : fileURL, options : options, completionHandler : completionHandler)
    }
    unsafe fn cancelDataRequest_(&self, requestID: PHAssetResourceDataRequestID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelDataRequest : requestID)
    }
    unsafe fn defaultManager() -> PHAssetResourceManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResourceManager").unwrap(), defaultManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAssetResource(pub id);
impl std::ops::Deref for PHAssetResource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAssetResource {}
impl PHAssetResource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResource").unwrap(), alloc) })
    }
}
impl INSObject for PHAssetResource {}
impl PNSObject for PHAssetResource {}
impl std::convert::TryFrom<NSObject> for PHAssetResource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHAssetResource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAssetResource").unwrap()) };
        if is_kind_of {
            Ok(PHAssetResource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHAssetResource")
        }
    }
}
impl IPHAssetResource for PHAssetResource {}
pub trait IPHAssetResource: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> PHAssetResourceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn assetLocalIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetLocalIdentifier)
    }
    unsafe fn originalFilename(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalFilename)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn uniformTypeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniformTypeIdentifier)
    }
    unsafe fn pixelWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelWidth)
    }
    unsafe fn pixelHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelHeight)
    }
    unsafe fn assetResourcesForAsset_(asset: PHAsset) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResource").unwrap(), assetResourcesForAsset : asset)
    }
    unsafe fn assetResourcesForLivePhoto_(livePhoto: PHLivePhoto) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHAssetResource").unwrap(), assetResourcesForLivePhoto : livePhoto)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHAdjustmentData(pub id);
impl std::ops::Deref for PHAdjustmentData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHAdjustmentData {}
impl PHAdjustmentData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHAdjustmentData").unwrap(), alloc) })
    }
}
impl INSObject for PHAdjustmentData {}
impl PNSObject for PHAdjustmentData {}
impl std::convert::TryFrom<NSObject> for PHAdjustmentData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHAdjustmentData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHAdjustmentData").unwrap()) };
        if is_kind_of {
            Ok(PHAdjustmentData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHAdjustmentData")
        }
    }
}
impl IPHAdjustmentData for PHAdjustmentData {}
pub trait IPHAdjustmentData: Sized + std::ops::Deref {
    unsafe fn initWithFormatIdentifier_formatVersion_data_(
        &self,
        formatIdentifier: NSString,
        formatVersion: NSString,
        data: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormatIdentifier : formatIdentifier, formatVersion : formatVersion, data : data)
    }
    unsafe fn formatIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatIdentifier)
    }
    unsafe fn formatVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatVersion)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHContentEditingInput(pub id);
impl std::ops::Deref for PHContentEditingInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHContentEditingInput {}
impl PHContentEditingInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHContentEditingInput").unwrap(), alloc) })
    }
}
impl INSObject for PHContentEditingInput {}
impl PNSObject for PHContentEditingInput {}
impl std::convert::TryFrom<NSObject> for PHContentEditingInput {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHContentEditingInput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHContentEditingInput").unwrap()) };
        if is_kind_of {
            Ok(PHContentEditingInput(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHContentEditingInput")
        }
    }
}
impl IPHContentEditingInput for PHContentEditingInput {}
pub trait IPHContentEditingInput: Sized + std::ops::Deref {
    unsafe fn mediaType(&self) -> PHAssetMediaType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaType)
    }
    unsafe fn mediaSubtypes(&self) -> PHAssetMediaSubtype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSubtypes)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn uniformTypeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniformTypeIdentifier)
    }
    unsafe fn playbackStyle(&self) -> PHAssetPlaybackStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackStyle)
    }
    unsafe fn adjustmentData(&self) -> PHAdjustmentData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adjustmentData)
    }
    unsafe fn displaySizeImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaySizeImage)
    }
    unsafe fn fullSizeImageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullSizeImageURL)
    }
    unsafe fn fullSizeImageOrientation(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullSizeImageOrientation)
    }
    unsafe fn avAsset(&self) -> AVAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, avAsset)
    }
    unsafe fn audiovisualAsset(&self) -> AVAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audiovisualAsset)
    }
    unsafe fn livePhoto(&self) -> PHLivePhoto
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, livePhoto)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProject(pub id);
impl std::ops::Deref for PHProject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProject {}
impl PHProject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProject").unwrap(), alloc) })
    }
}
impl IPHAssetCollection for PHProject {}
impl From<PHProject> for PHAssetCollection {
    fn from(child: PHProject) -> PHAssetCollection {
        PHAssetCollection(child.0)
    }
}
impl std::convert::TryFrom<PHAssetCollection> for PHProject {
    type Error = &'static str;
    fn try_from(parent: PHAssetCollection) -> Result<PHProject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProject").unwrap()) };
        if is_kind_of {
            Ok(PHProject(parent.0))
        } else {
            Err("This PHAssetCollection cannot be downcasted to PHProject")
        }
    }
}
impl IPHCollection for PHProject {}
impl IPHObject for PHProject {}
impl PNSCopying for PHProject {}
impl INSObject for PHProject {}
impl PNSObject for PHProject {}
impl IPHProject for PHProject {}
pub trait IPHProject: Sized + std::ops::Deref {
    unsafe fn projectExtensionData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectExtensionData)
    }
    unsafe fn hasProjectPreview(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasProjectPreview)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectChangeRequest(pub id);
impl std::ops::Deref for PHProjectChangeRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectChangeRequest {}
impl PHProjectChangeRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectChangeRequest").unwrap(), alloc) })
    }
}
impl IPHChangeRequest for PHProjectChangeRequest {}
impl std::convert::TryFrom<PHChangeRequest> for PHProjectChangeRequest {
    type Error = &'static str;
    fn try_from(parent: PHChangeRequest) -> Result<PHProjectChangeRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectChangeRequest").unwrap()) };
        if is_kind_of {
            Ok(PHProjectChangeRequest(parent.0))
        } else {
            Err("This PHChangeRequest cannot be downcasted to PHProjectChangeRequest")
        }
    }
}
impl INSObject for PHProjectChangeRequest {}
impl PNSObject for PHProjectChangeRequest {}
impl IPHProjectChangeRequest for PHProjectChangeRequest {}
pub trait IPHProjectChangeRequest: Sized + std::ops::Deref {
    unsafe fn initWithProject_(&self, project: PHProject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProject : project)
    }
    unsafe fn setKeyAsset_(&self, keyAsset: PHAsset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyAsset : keyAsset)
    }
    unsafe fn setProjectPreviewImage_(&self, previewImage: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProjectPreviewImage : previewImage)
    }
    unsafe fn removeAssets_(&self, assets: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAssets : assets)
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
    unsafe fn projectExtensionData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectExtensionData)
    }
    unsafe fn setProjectExtensionData_(&self, projectExtensionData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProjectExtensionData : projectExtensionData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHCloudIdentifier(pub id);
impl std::ops::Deref for PHCloudIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHCloudIdentifier {}
impl PHCloudIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHCloudIdentifier").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHCloudIdentifier {}
impl INSObject for PHCloudIdentifier {}
impl PNSObject for PHCloudIdentifier {}
impl std::convert::TryFrom<NSObject> for PHCloudIdentifier {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHCloudIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHCloudIdentifier").unwrap()) };
        if is_kind_of {
            Ok(PHCloudIdentifier(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHCloudIdentifier")
        }
    }
}
impl IPHCloudIdentifier for PHCloudIdentifier {}
pub trait IPHCloudIdentifier: Sized + std::ops::Deref {
    unsafe fn initWithStringValue_(&self, stringValue: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStringValue : stringValue)
    }
    unsafe fn stringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringValue)
    }
    unsafe fn notFoundIdentifier() -> PHCloudIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHCloudIdentifier").unwrap(), notFoundIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHCloudIdentifierMapping(pub id);
impl std::ops::Deref for PHCloudIdentifierMapping {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHCloudIdentifierMapping {}
impl PHCloudIdentifierMapping {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHCloudIdentifierMapping").unwrap(), alloc) })
    }
}
impl INSObject for PHCloudIdentifierMapping {}
impl PNSObject for PHCloudIdentifierMapping {}
impl std::convert::TryFrom<NSObject> for PHCloudIdentifierMapping {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHCloudIdentifierMapping, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHCloudIdentifierMapping").unwrap()) };
        if is_kind_of {
            Ok(PHCloudIdentifierMapping(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHCloudIdentifierMapping")
        }
    }
}
impl IPHCloudIdentifierMapping for PHCloudIdentifierMapping {}
pub trait IPHCloudIdentifierMapping: Sized + std::ops::Deref {
    unsafe fn cloudIdentifier(&self) -> PHCloudIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloudIdentifier)
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
pub struct PHLocalIdentifierMapping(pub id);
impl std::ops::Deref for PHLocalIdentifierMapping {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHLocalIdentifierMapping {}
impl PHLocalIdentifierMapping {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHLocalIdentifierMapping").unwrap(), alloc) })
    }
}
impl INSObject for PHLocalIdentifierMapping {}
impl PNSObject for PHLocalIdentifierMapping {}
impl std::convert::TryFrom<NSObject> for PHLocalIdentifierMapping {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHLocalIdentifierMapping, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHLocalIdentifierMapping").unwrap()) };
        if is_kind_of {
            Ok(PHLocalIdentifierMapping(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHLocalIdentifierMapping")
        }
    }
}
impl IPHLocalIdentifierMapping for PHLocalIdentifierMapping {}
pub trait IPHLocalIdentifierMapping: Sized + std::ops::Deref {
    unsafe fn localIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localIdentifier)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
}
impl PHCloudIdentifier_ for PHCloudIdentifier {}
pub trait PHCloudIdentifier_: Sized + std::ops::Deref {}
impl PHPhotoLibrary_CloudIdentifiers for PHPhotoLibrary {}
pub trait PHPhotoLibrary_CloudIdentifiers: Sized + std::ops::Deref {
    unsafe fn localIdentifierMappingsForCloudIdentifiers_(
        &self,
        cloudIdentifiers: NSArray,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localIdentifierMappingsForCloudIdentifiers : cloudIdentifiers)
    }
    unsafe fn cloudIdentifierMappingsForLocalIdentifiers_(
        &self,
        localIdentifiers: NSArray,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cloudIdentifierMappingsForLocalIdentifiers : localIdentifiers)
    }
    unsafe fn localIdentifiersForCloudIdentifiers_(&self, cloudIdentifiers: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localIdentifiersForCloudIdentifiers : cloudIdentifiers)
    }
    unsafe fn cloudIdentifiersForLocalIdentifiers_(&self, localIdentifiers: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cloudIdentifiersForLocalIdentifiers : localIdentifiers)
    }
}
unsafe extern "C" {
    pub static PHPhotosErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static PHLocalIdentifiersErrorKey: NSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static PHLivePhotoInfoErrorKey: NSString;
}
unsafe extern "C" {
    pub static PHLivePhotoInfoIsDegradedKey: NSString;
}
unsafe extern "C" {
    pub static PHLivePhotoInfoCancelledKey: NSString;
}
unsafe extern "C" {
    pub static PHContentEditingInputResultIsInCloudKey: NSString;
}
unsafe extern "C" {
    pub static PHContentEditingInputCancelledKey: NSString;
}
unsafe extern "C" {
    pub static PHContentEditingInputErrorKey: NSString;
}
unsafe extern "C" {
    pub static mut PHLivePhotoShouldRenderAtPlaybackTime: PHLivePhotoEditingOption;
}
unsafe extern "C" {
    pub static PHLivePhotoEditingErrorDomain: NSString;
}
unsafe extern "C" {
    pub static PHImageManagerMaximumSize: CGSize;
}
unsafe extern "C" {
    pub static PHImageResultIsInCloudKey: NSString;
}
unsafe extern "C" {
    pub static PHImageResultIsDegradedKey: NSString;
}
unsafe extern "C" {
    pub static PHImageResultRequestIDKey: NSString;
}
unsafe extern "C" {
    pub static PHImageCancelledKey: NSString;
}
unsafe extern "C" {
    pub static PHImageErrorKey: NSString;
}
unsafe extern "C" {
    pub static PHLocalIdentifierNotFound: NSString;
}

unsafe impl objc2::encode::RefEncode for PHPhotoLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPhotoLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHObjectPlaceholder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHObjectPlaceholder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHFetchResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHFetchResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHLivePhoto {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHLivePhoto {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHCollectionList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHCollectionList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHFetchOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHFetchOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHChange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHChange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHObjectChangeDetails {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHObjectChangeDetails {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHFetchResultChangeDetails {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHFetchResultChangeDetails {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPersistentChange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPersistentChange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPersistentChangeToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPersistentChangeToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPersistentChangeFetchResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPersistentChangeFetchResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPersistentObjectChangeDetails {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPersistentObjectChangeDetails {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHChangeRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHChangeRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHContentEditingOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHContentEditingOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetChangeRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetChangeRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHContentEditingInputRequestOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHContentEditingInputRequestOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetResourceCreationOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetResourceCreationOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetCreationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetCreationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetCollectionChangeRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetCollectionChangeRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetResourceUploadJob {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetResourceUploadJob {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetResourceUploadJobChangeRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetResourceUploadJobChangeRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHCollectionListChangeRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHCollectionListChangeRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHLivePhotoEditingContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHLivePhotoEditingContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHImageRequestOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHImageRequestOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHLivePhotoRequestOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHLivePhotoRequestOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHVideoRequestOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHVideoRequestOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHImageManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHImageManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHCachingImageManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHCachingImageManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetResourceRequestOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetResourceRequestOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetResourceManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetResourceManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAssetResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAssetResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHAdjustmentData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHAdjustmentData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHContentEditingInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHContentEditingInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectChangeRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectChangeRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHCloudIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHCloudIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHCloudIdentifierMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHCloudIdentifierMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHLocalIdentifierMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHLocalIdentifierMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
