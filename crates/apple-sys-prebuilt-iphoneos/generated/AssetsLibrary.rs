#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ALAssetOrientation = NSInteger;
pub type ALAssetsGroupType = NSUInteger;
pub type ALAuthorizationStatus = NSInteger;
pub type ALAssetsLibraryGroupsEnumerationResultsBlock = *mut ::std::os::raw::c_void;
pub type ALAssetsLibraryAssetForURLResultBlock = *mut ::std::os::raw::c_void;
pub type ALAssetsLibraryGroupResultBlock = *mut ::std::os::raw::c_void;
pub type ALAssetsLibraryAccessFailureBlock = *mut ::std::os::raw::c_void;
pub type ALAssetsLibraryWriteImageCompletionBlock = *mut ::std::os::raw::c_void;
pub type ALAssetsLibraryWriteVideoCompletionBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ALAssetsLibrary(pub id);
impl std::ops::Deref for ALAssetsLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ALAssetsLibrary {}
impl ALAssetsLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetsLibrary").unwrap(), alloc) })
    }
}
impl INSObject for ALAssetsLibrary {}
impl PNSObject for ALAssetsLibrary {}
impl std::convert::TryFrom<NSObject> for ALAssetsLibrary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ALAssetsLibrary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ALAssetsLibrary").unwrap()) };
        if is_kind_of {
            Ok(ALAssetsLibrary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ALAssetsLibrary")
        }
    }
}
impl IALAssetsLibrary for ALAssetsLibrary {}
pub trait IALAssetsLibrary: Sized + std::ops::Deref {
    unsafe fn enumerateGroupsWithTypes_usingBlock_failureBlock_(
        &self,
        types: ALAssetsGroupType,
        enumerationBlock: ALAssetsLibraryGroupsEnumerationResultsBlock,
        failureBlock: ALAssetsLibraryAccessFailureBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateGroupsWithTypes : types, usingBlock : enumerationBlock, failureBlock : failureBlock)
    }
    unsafe fn assetForURL_resultBlock_failureBlock_(
        &self,
        assetURL: NSURL,
        resultBlock: ALAssetsLibraryAssetForURLResultBlock,
        failureBlock: ALAssetsLibraryAccessFailureBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assetForURL : assetURL, resultBlock : resultBlock, failureBlock : failureBlock)
    }
    unsafe fn groupForURL_resultBlock_failureBlock_(
        &self,
        groupURL: NSURL,
        resultBlock: ALAssetsLibraryGroupResultBlock,
        failureBlock: ALAssetsLibraryAccessFailureBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, groupForURL : groupURL, resultBlock : resultBlock, failureBlock : failureBlock)
    }
    unsafe fn addAssetsGroupAlbumWithName_resultBlock_failureBlock_(
        &self,
        name: NSString,
        resultBlock: ALAssetsLibraryGroupResultBlock,
        failureBlock: ALAssetsLibraryAccessFailureBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAssetsGroupAlbumWithName : name, resultBlock : resultBlock, failureBlock : failureBlock)
    }
    unsafe fn writeImageToSavedPhotosAlbum_orientation_completionBlock_(
        &self,
        imageRef: CGImageRef,
        orientation: ALAssetOrientation,
        completionBlock: ALAssetsLibraryWriteImageCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeImageToSavedPhotosAlbum : imageRef, orientation : orientation, completionBlock : completionBlock)
    }
    unsafe fn writeImageToSavedPhotosAlbum_metadata_completionBlock_(
        &self,
        imageRef: CGImageRef,
        metadata: NSDictionary,
        completionBlock: ALAssetsLibraryWriteImageCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeImageToSavedPhotosAlbum : imageRef, metadata : metadata, completionBlock : completionBlock)
    }
    unsafe fn writeImageDataToSavedPhotosAlbum_metadata_completionBlock_(
        &self,
        imageData: NSData,
        metadata: NSDictionary,
        completionBlock: ALAssetsLibraryWriteImageCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeImageDataToSavedPhotosAlbum : imageData, metadata : metadata, completionBlock : completionBlock)
    }
    unsafe fn writeVideoAtPathToSavedPhotosAlbum_completionBlock_(
        &self,
        videoPathURL: NSURL,
        completionBlock: ALAssetsLibraryWriteVideoCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeVideoAtPathToSavedPhotosAlbum : videoPathURL, completionBlock : completionBlock)
    }
    unsafe fn videoAtPathIsCompatibleWithSavedPhotosAlbum_(&self, videoPathURL: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, videoAtPathIsCompatibleWithSavedPhotosAlbum : videoPathURL)
    }
    unsafe fn authorizationStatus() -> ALAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetsLibrary").unwrap(), authorizationStatus)
    }
    unsafe fn disableSharedPhotoStreamsSupport()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetsLibrary").unwrap(), disableSharedPhotoStreamsSupport)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ALAsset(pub id);
impl std::ops::Deref for ALAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ALAsset {}
impl ALAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ALAsset").unwrap(), alloc) })
    }
}
impl INSObject for ALAsset {}
impl PNSObject for ALAsset {}
impl std::convert::TryFrom<NSObject> for ALAsset {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ALAsset, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ALAsset").unwrap()) };
        if is_kind_of {
            Ok(ALAsset(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ALAsset")
        }
    }
}
impl IALAsset for ALAsset {}
pub trait IALAsset: Sized + std::ops::Deref {
    unsafe fn valueForProperty_(&self, property: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForProperty : property)
    }
    unsafe fn defaultRepresentation(&self) -> ALAssetRepresentation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultRepresentation)
    }
    unsafe fn representationForUTI_(&self, representationUTI: NSString) -> ALAssetRepresentation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, representationForUTI : representationUTI)
    }
    unsafe fn thumbnail(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnail)
    }
    unsafe fn aspectRatioThumbnail(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatioThumbnail)
    }
    unsafe fn writeModifiedImageDataToSavedPhotosAlbum_metadata_completionBlock_(
        &self,
        imageData: NSData,
        metadata: NSDictionary,
        completionBlock: ALAssetsLibraryWriteImageCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeModifiedImageDataToSavedPhotosAlbum : imageData, metadata : metadata, completionBlock : completionBlock)
    }
    unsafe fn writeModifiedVideoAtPathToSavedPhotosAlbum_completionBlock_(
        &self,
        videoPathURL: NSURL,
        completionBlock: ALAssetsLibraryWriteVideoCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeModifiedVideoAtPathToSavedPhotosAlbum : videoPathURL, completionBlock : completionBlock)
    }
    unsafe fn setImageData_metadata_completionBlock_(
        &self,
        imageData: NSData,
        metadata: NSDictionary,
        completionBlock: ALAssetsLibraryWriteImageCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageData : imageData, metadata : metadata, completionBlock : completionBlock)
    }
    unsafe fn setVideoAtPath_completionBlock_(
        &self,
        videoPathURL: NSURL,
        completionBlock: ALAssetsLibraryWriteVideoCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoAtPath : videoPathURL, completionBlock : completionBlock)
    }
    unsafe fn originalAsset(&self) -> ALAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalAsset)
    }
    unsafe fn isEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEditable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ALAssetsFilter(pub id);
impl std::ops::Deref for ALAssetsFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ALAssetsFilter {}
impl ALAssetsFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetsFilter").unwrap(), alloc) })
    }
}
impl INSObject for ALAssetsFilter {}
impl PNSObject for ALAssetsFilter {}
impl std::convert::TryFrom<NSObject> for ALAssetsFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ALAssetsFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ALAssetsFilter").unwrap()) };
        if is_kind_of {
            Ok(ALAssetsFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ALAssetsFilter")
        }
    }
}
impl IALAssetsFilter for ALAssetsFilter {}
pub trait IALAssetsFilter: Sized + std::ops::Deref {
    unsafe fn allPhotos() -> ALAssetsFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetsFilter").unwrap(), allPhotos)
    }
    unsafe fn allVideos() -> ALAssetsFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetsFilter").unwrap(), allVideos)
    }
    unsafe fn allAssets() -> ALAssetsFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetsFilter").unwrap(), allAssets)
    }
}
pub type ALAssetsGroupEnumerationResultsBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ALAssetsGroup(pub id);
impl std::ops::Deref for ALAssetsGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ALAssetsGroup {}
impl ALAssetsGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetsGroup").unwrap(), alloc) })
    }
}
impl INSObject for ALAssetsGroup {}
impl PNSObject for ALAssetsGroup {}
impl std::convert::TryFrom<NSObject> for ALAssetsGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ALAssetsGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ALAssetsGroup").unwrap()) };
        if is_kind_of {
            Ok(ALAssetsGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ALAssetsGroup")
        }
    }
}
impl IALAssetsGroup for ALAssetsGroup {}
pub trait IALAssetsGroup: Sized + std::ops::Deref {
    unsafe fn valueForProperty_(&self, property: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForProperty : property)
    }
    unsafe fn posterImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, posterImage)
    }
    unsafe fn setAssetsFilter_(&self, filter: ALAssetsFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAssetsFilter : filter)
    }
    unsafe fn numberOfAssets(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfAssets)
    }
    unsafe fn enumerateAssetsUsingBlock_(
        &self,
        enumerationBlock: ALAssetsGroupEnumerationResultsBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateAssetsUsingBlock : enumerationBlock)
    }
    unsafe fn enumerateAssetsWithOptions_usingBlock_(
        &self,
        options: NSEnumerationOptions,
        enumerationBlock: ALAssetsGroupEnumerationResultsBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateAssetsWithOptions : options, usingBlock : enumerationBlock)
    }
    unsafe fn enumerateAssetsAtIndexes_options_usingBlock_(
        &self,
        indexSet: NSIndexSet,
        options: NSEnumerationOptions,
        enumerationBlock: ALAssetsGroupEnumerationResultsBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateAssetsAtIndexes : indexSet, options : options, usingBlock : enumerationBlock)
    }
    unsafe fn addAsset_(&self, asset: ALAsset) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAsset : asset)
    }
    unsafe fn isEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEditable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ALAssetRepresentation(pub id);
impl std::ops::Deref for ALAssetRepresentation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ALAssetRepresentation {}
impl ALAssetRepresentation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ALAssetRepresentation").unwrap(), alloc) })
    }
}
impl INSObject for ALAssetRepresentation {}
impl PNSObject for ALAssetRepresentation {}
impl std::convert::TryFrom<NSObject> for ALAssetRepresentation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ALAssetRepresentation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ALAssetRepresentation").unwrap()) };
        if is_kind_of {
            Ok(ALAssetRepresentation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ALAssetRepresentation")
        }
    }
}
impl IALAssetRepresentation for ALAssetRepresentation {}
pub trait IALAssetRepresentation: Sized + std::ops::Deref {
    unsafe fn UTI(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UTI)
    }
    unsafe fn dimensions(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
    unsafe fn size(&self) -> ::std::os::raw::c_longlong
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn getBytes_fromOffset_length_error_(
        &self,
        buffer: *mut u8,
        offset: ::std::os::raw::c_longlong,
        length: NSUInteger,
        error: *mut NSError,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getBytes : buffer, fromOffset : offset, length : length, error : error)
    }
    unsafe fn fullResolutionImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullResolutionImage)
    }
    unsafe fn CGImageWithOptions_(&self, options: NSDictionary) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, CGImageWithOptions : options)
    }
    unsafe fn fullScreenImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullScreenImage)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn orientation(&self) -> ALAssetOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn filename(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filename)
    }
}
unsafe extern "C" {
    pub static ALAssetsLibraryChangedNotification: NSString;
}
unsafe extern "C" {
    pub static ALAssetLibraryUpdatedAssetsKey: NSString;
}
unsafe extern "C" {
    pub static ALAssetLibraryInsertedAssetGroupsKey: NSString;
}
unsafe extern "C" {
    pub static ALAssetLibraryUpdatedAssetGroupsKey: NSString;
}
unsafe extern "C" {
    pub static ALAssetLibraryDeletedAssetGroupsKey: NSString;
}
unsafe extern "C" {
    pub static ALAssetsLibraryErrorDomain: NSString;
}
unsafe extern "C" {
    pub static ALErrorInvalidProperty: NSString;
}
unsafe extern "C" {
    pub static ALAssetPropertyType: NSString;
}
unsafe extern "C" {
    pub static ALAssetPropertyLocation: NSString;
}
unsafe extern "C" {
    pub static ALAssetPropertyDuration: NSString;
}
unsafe extern "C" {
    pub static ALAssetPropertyOrientation: NSString;
}
unsafe extern "C" {
    pub static ALAssetPropertyDate: NSString;
}
unsafe extern "C" {
    pub static ALAssetPropertyRepresentations: NSString;
}
unsafe extern "C" {
    pub static ALAssetPropertyURLs: NSString;
}
unsafe extern "C" {
    pub static ALAssetPropertyAssetURL: NSString;
}
unsafe extern "C" {
    pub static ALAssetTypePhoto: NSString;
}
unsafe extern "C" {
    pub static ALAssetTypeVideo: NSString;
}
unsafe extern "C" {
    pub static ALAssetTypeUnknown: NSString;
}
unsafe extern "C" {
    pub static ALAssetsGroupPropertyName: NSString;
}
unsafe extern "C" {
    pub static ALAssetsGroupPropertyType: NSString;
}
unsafe extern "C" {
    pub static ALAssetsGroupPropertyPersistentID: NSString;
}
unsafe extern "C" {
    pub static ALAssetsGroupPropertyURL: NSString;
}

unsafe impl objc2::encode::RefEncode for ALAssetsLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALAssetsLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ALAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ALAssetsFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALAssetsFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ALAssetsGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALAssetsGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ALAssetRepresentation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALAssetRepresentation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
