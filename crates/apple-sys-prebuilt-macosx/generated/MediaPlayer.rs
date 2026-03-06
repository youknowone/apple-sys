#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub trait AVMediaSelectionOption_MPNowPlayingInfoLanguageOptionAdditions:
    Sized + std::ops::Deref
{
    unsafe fn makeNowPlayingInfoLanguageOption(&self) -> MPNowPlayingInfoLanguageOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeNowPlayingInfoLanguageOption)
    }
}
pub trait AVMediaSelectionGroup_MPNowPlayingInfoLanguageOptionAdditions:
    Sized + std::ops::Deref
{
    unsafe fn makeNowPlayingInfoLanguageOptionGroup(&self) -> MPNowPlayingInfoLanguageOptionGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeNowPlayingInfoLanguageOptionGroup)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPContentItem(pub id);
impl std::ops::Deref for MPContentItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPContentItem {}
impl MPContentItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPContentItem").unwrap(), alloc) })
    }
}
impl INSObject for MPContentItem {}
impl PNSObject for MPContentItem {}
impl std::convert::TryFrom<NSObject> for MPContentItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPContentItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPContentItem").unwrap()) };
        if is_kind_of {
            Ok(MPContentItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPContentItem")
        }
    }
}
impl IMPContentItem for MPContentItem {}
pub trait IMPContentItem: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
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
    unsafe fn artwork(&self) -> MPMediaItemArtwork
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artwork)
    }
    unsafe fn setArtwork_(&self, artwork: MPMediaItemArtwork)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArtwork : artwork)
    }
    unsafe fn playbackProgress(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackProgress)
    }
    unsafe fn setPlaybackProgress_(&self, playbackProgress: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaybackProgress : playbackProgress)
    }
    unsafe fn isStreamingContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStreamingContent)
    }
    unsafe fn setStreamingContent_(&self, streamingContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStreamingContent : streamingContent)
    }
    unsafe fn isExplicitContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExplicitContent)
    }
    unsafe fn setExplicitContent_(&self, explicitContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitContent : explicitContent)
    }
    unsafe fn isContainer(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContainer)
    }
    unsafe fn setContainer_(&self, container: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainer : container)
    }
    unsafe fn isPlayable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPlayable)
    }
    unsafe fn setPlayable_(&self, playable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayable : playable)
    }
}
pub type MPErrorCode = NSInteger;
pub type MPMediaEntityPersistentID = u64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaEntity(pub id);
impl std::ops::Deref for MPMediaEntity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaEntity {}
impl MPMediaEntity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaEntity").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MPMediaEntity {}
impl INSObject for MPMediaEntity {}
impl PNSObject for MPMediaEntity {}
impl std::convert::TryFrom<NSObject> for MPMediaEntity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMediaEntity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaEntity").unwrap()) };
        if is_kind_of {
            Ok(MPMediaEntity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMediaEntity")
        }
    }
}
impl IMPMediaEntity for MPMediaEntity {}
pub trait IMPMediaEntity: Sized + std::ops::Deref {
    unsafe fn enumerateValuesForProperties_usingBlock_(
        &self,
        properties: NSSet,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateValuesForProperties : properties, usingBlock : block)
    }
    unsafe fn objectForKeyedSubscript_(&self, key: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn valueForProperty_(&self, property: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForProperty : property)
    }
    unsafe fn persistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentID)
    }
    unsafe fn canFilterByProperty_(property: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaEntity").unwrap(), canFilterByProperty : property)
    }
}
pub type MPMediaType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaItem(pub id);
impl std::ops::Deref for MPMediaItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaItem {}
impl MPMediaItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItem").unwrap(), alloc) })
    }
}
impl IMPMediaEntity for MPMediaItem {}
impl PNSSecureCoding for MPMediaItem {}
impl From<MPMediaItem> for MPMediaEntity {
    fn from(child: MPMediaItem) -> MPMediaEntity {
        MPMediaEntity(child.0)
    }
}
impl std::convert::TryFrom<MPMediaEntity> for MPMediaItem {
    type Error = &'static str;
    fn try_from(parent: MPMediaEntity) -> Result<MPMediaItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaItem").unwrap()) };
        if is_kind_of {
            Ok(MPMediaItem(parent.0))
        } else {
            Err("This MPMediaEntity cannot be downcasted to MPMediaItem")
        }
    }
}
impl INSObject for MPMediaItem {}
impl PNSObject for MPMediaItem {}
impl IMPMediaItem for MPMediaItem {}
pub trait IMPMediaItem: Sized + std::ops::Deref {
    unsafe fn persistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentID)
    }
    unsafe fn mediaType(&self) -> MPMediaType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaType)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn albumTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, albumTitle)
    }
    unsafe fn albumPersistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, albumPersistentID)
    }
    unsafe fn artist(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artist)
    }
    unsafe fn artistPersistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artistPersistentID)
    }
    unsafe fn albumArtist(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, albumArtist)
    }
    unsafe fn albumArtistPersistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, albumArtistPersistentID)
    }
    unsafe fn genre(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, genre)
    }
    unsafe fn genrePersistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, genrePersistentID)
    }
    unsafe fn composer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composer)
    }
    unsafe fn composerPersistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composerPersistentID)
    }
    unsafe fn playbackDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackDuration)
    }
    unsafe fn albumTrackNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, albumTrackNumber)
    }
    unsafe fn albumTrackCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, albumTrackCount)
    }
    unsafe fn discNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discNumber)
    }
    unsafe fn discCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discCount)
    }
    unsafe fn artwork(&self) -> MPMediaItemArtwork
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artwork)
    }
    unsafe fn isExplicitItem(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExplicitItem)
    }
    unsafe fn lyrics(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lyrics)
    }
    unsafe fn isCompilation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompilation)
    }
    unsafe fn releaseDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseDate)
    }
    unsafe fn beatsPerMinute(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beatsPerMinute)
    }
    unsafe fn comments(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, comments)
    }
    unsafe fn assetURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetURL)
    }
    unsafe fn isCloudItem(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCloudItem)
    }
    unsafe fn hasProtectedAsset(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasProtectedAsset)
    }
    unsafe fn podcastTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, podcastTitle)
    }
    unsafe fn podcastPersistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, podcastPersistentID)
    }
    unsafe fn playCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playCount)
    }
    unsafe fn skipCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skipCount)
    }
    unsafe fn rating(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rating)
    }
    unsafe fn lastPlayedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastPlayedDate)
    }
    unsafe fn userGrouping(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userGrouping)
    }
    unsafe fn bookmarkTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bookmarkTime)
    }
    unsafe fn dateAdded(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateAdded)
    }
    unsafe fn playbackStoreID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackStoreID)
    }
    unsafe fn isPreorder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPreorder)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaItemArtwork(pub id);
impl std::ops::Deref for MPMediaItemArtwork {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaItemArtwork {}
impl MPMediaItemArtwork {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItemArtwork").unwrap(), alloc) })
    }
}
impl INSObject for MPMediaItemArtwork {}
impl PNSObject for MPMediaItemArtwork {}
impl std::convert::TryFrom<NSObject> for MPMediaItemArtwork {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMediaItemArtwork, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaItemArtwork").unwrap()) };
        if is_kind_of {
            Ok(MPMediaItemArtwork(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMediaItemArtwork")
        }
    }
}
impl IMPMediaItemArtwork for MPMediaItemArtwork {}
pub trait IMPMediaItemArtwork: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithBoundsSize_requestHandler_(
        &self,
        boundsSize: CGSize,
        requestHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBoundsSize : boundsSize, requestHandler : requestHandler)
    }
    unsafe fn imageWithSize_(&self, size: CGSize) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageWithSize : size)
    }
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn imageCropRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageCropRect)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItemArtwork").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaItemAnimatedArtwork(pub id);
impl std::ops::Deref for MPMediaItemAnimatedArtwork {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaItemAnimatedArtwork {}
impl MPMediaItemAnimatedArtwork {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItemAnimatedArtwork").unwrap(), alloc) })
    }
}
impl INSObject for MPMediaItemAnimatedArtwork {}
impl PNSObject for MPMediaItemAnimatedArtwork {}
impl std::convert::TryFrom<NSObject> for MPMediaItemAnimatedArtwork {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMediaItemAnimatedArtwork, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaItemAnimatedArtwork").unwrap()) };
        if is_kind_of {
            Ok(MPMediaItemAnimatedArtwork(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMediaItemAnimatedArtwork")
        }
    }
}
impl IMPMediaItemAnimatedArtwork for MPMediaItemAnimatedArtwork {}
pub trait IMPMediaItemAnimatedArtwork: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithArtworkID_previewImageRequestHandler_videoAssetFileURLRequestHandler_(
        &self,
        artworkID: NSString,
        previewImageRequestHandler: *mut ::std::os::raw::c_void,
        videoAssetFileURLRequestHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithArtworkID : artworkID, previewImageRequestHandler : previewImageRequestHandler, videoAssetFileURLRequestHandler : videoAssetFileURLRequestHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItemAnimatedArtwork").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaItemCollection(pub id);
impl std::ops::Deref for MPMediaItemCollection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaItemCollection {}
impl MPMediaItemCollection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItemCollection").unwrap(), alloc) })
    }
}
impl IMPMediaEntity for MPMediaItemCollection {}
impl PNSSecureCoding for MPMediaItemCollection {}
impl std::convert::TryFrom<MPMediaEntity> for MPMediaItemCollection {
    type Error = &'static str;
    fn try_from(parent: MPMediaEntity) -> Result<MPMediaItemCollection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaItemCollection").unwrap()) };
        if is_kind_of {
            Ok(MPMediaItemCollection(parent.0))
        } else {
            Err("This MPMediaEntity cannot be downcasted to MPMediaItemCollection")
        }
    }
}
impl INSObject for MPMediaItemCollection {}
impl PNSObject for MPMediaItemCollection {}
impl IMPMediaItemCollection for MPMediaItemCollection {}
pub trait IMPMediaItemCollection: Sized + std::ops::Deref {
    unsafe fn initWithItems_(&self, items: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithItems : items)
    }
    unsafe fn items(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, items)
    }
    unsafe fn representativeItem(&self) -> MPMediaItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, representativeItem)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn mediaTypes(&self) -> MPMediaType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaTypes)
    }
    unsafe fn collectionWithItems_(items: NSArray) -> MPMediaItemCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItemCollection").unwrap(), collectionWithItems : items)
    }
}
pub type MPMediaLibraryAuthorizationStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaLibrary(pub id);
impl std::ops::Deref for MPMediaLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaLibrary {}
impl MPMediaLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaLibrary").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MPMediaLibrary {}
impl INSObject for MPMediaLibrary {}
impl PNSObject for MPMediaLibrary {}
impl std::convert::TryFrom<NSObject> for MPMediaLibrary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMediaLibrary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaLibrary").unwrap()) };
        if is_kind_of {
            Ok(MPMediaLibrary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMediaLibrary")
        }
    }
}
impl IMPMediaLibrary for MPMediaLibrary {}
pub trait IMPMediaLibrary: Sized + std::ops::Deref {
    unsafe fn beginGeneratingLibraryChangeNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginGeneratingLibraryChangeNotifications)
    }
    unsafe fn endGeneratingLibraryChangeNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endGeneratingLibraryChangeNotifications)
    }
    unsafe fn addItemWithProductID_completionHandler_(
        &self,
        productID: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addItemWithProductID : productID, completionHandler : completionHandler)
    }
    unsafe fn getPlaylistWithUUID_creationMetadata_completionHandler_(
        &self,
        uuid: NSUUID,
        creationMetadata: MPMediaPlaylistCreationMetadata,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPlaylistWithUUID : uuid, creationMetadata : creationMetadata, completionHandler : completionHandler)
    }
    unsafe fn lastModifiedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastModifiedDate)
    }
    unsafe fn defaultMediaLibrary() -> MPMediaLibrary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaLibrary").unwrap(), defaultMediaLibrary)
    }
    unsafe fn authorizationStatus() -> MPMediaLibraryAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaLibrary").unwrap(), authorizationStatus)
    }
    unsafe fn requestAuthorization_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaLibrary").unwrap(), requestAuthorization : completionHandler)
    }
}
pub type MPMediaPlaylistAttribute = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaPlaylist(pub id);
impl std::ops::Deref for MPMediaPlaylist {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaPlaylist {}
impl MPMediaPlaylist {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaPlaylist").unwrap(), alloc) })
    }
}
impl IMPMediaItemCollection for MPMediaPlaylist {}
impl From<MPMediaPlaylist> for MPMediaItemCollection {
    fn from(child: MPMediaPlaylist) -> MPMediaItemCollection {
        MPMediaItemCollection(child.0)
    }
}
impl std::convert::TryFrom<MPMediaItemCollection> for MPMediaPlaylist {
    type Error = &'static str;
    fn try_from(parent: MPMediaItemCollection) -> Result<MPMediaPlaylist, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaPlaylist").unwrap()) };
        if is_kind_of {
            Ok(MPMediaPlaylist(parent.0))
        } else {
            Err("This MPMediaItemCollection cannot be downcasted to MPMediaPlaylist")
        }
    }
}
impl IMPMediaEntity for MPMediaPlaylist {}
impl PNSSecureCoding for MPMediaPlaylist {}
impl INSObject for MPMediaPlaylist {}
impl PNSObject for MPMediaPlaylist {}
impl IMPMediaPlaylist for MPMediaPlaylist {}
pub trait IMPMediaPlaylist: Sized + std::ops::Deref {
    unsafe fn addItemWithProductID_completionHandler_(
        &self,
        productID: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addItemWithProductID : productID, completionHandler : completionHandler)
    }
    unsafe fn addMediaItems_completionHandler_(
        &self,
        mediaItems: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMediaItems : mediaItems, completionHandler : completionHandler)
    }
    unsafe fn persistentID(&self) -> MPMediaEntityPersistentID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentID)
    }
    unsafe fn cloudGlobalID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloudGlobalID)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn playlistAttributes(&self) -> MPMediaPlaylistAttribute
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playlistAttributes)
    }
    unsafe fn seedItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seedItems)
    }
    unsafe fn descriptionText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptionText)
    }
    unsafe fn authorDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorDisplayName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaPlaylistCreationMetadata(pub id);
impl std::ops::Deref for MPMediaPlaylistCreationMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaPlaylistCreationMetadata {}
impl MPMediaPlaylistCreationMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaPlaylistCreationMetadata").unwrap(), alloc) })
    }
}
impl INSObject for MPMediaPlaylistCreationMetadata {}
impl PNSObject for MPMediaPlaylistCreationMetadata {}
impl std::convert::TryFrom<NSObject> for MPMediaPlaylistCreationMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMediaPlaylistCreationMetadata, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaPlaylistCreationMetadata").unwrap())
        };
        if is_kind_of {
            Ok(MPMediaPlaylistCreationMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMediaPlaylistCreationMetadata")
        }
    }
}
impl IMPMediaPlaylistCreationMetadata for MPMediaPlaylistCreationMetadata {}
pub trait IMPMediaPlaylistCreationMetadata: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithName_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn authorDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorDisplayName)
    }
    unsafe fn setAuthorDisplayName_(&self, authorDisplayName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorDisplayName : authorDisplayName)
    }
    unsafe fn descriptionText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptionText)
    }
    unsafe fn setDescriptionText_(&self, descriptionText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDescriptionText : descriptionText)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaPlaylistCreationMetadata").unwrap(), new)
    }
}
pub type MPMediaGrouping = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaQuery(pub id);
impl std::ops::Deref for MPMediaQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaQuery {}
impl MPMediaQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MPMediaQuery {}
impl PNSCopying for MPMediaQuery {}
impl INSObject for MPMediaQuery {}
impl PNSObject for MPMediaQuery {}
impl std::convert::TryFrom<NSObject> for MPMediaQuery {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMediaQuery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap()) };
        if is_kind_of {
            Ok(MPMediaQuery(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMediaQuery")
        }
    }
}
impl IMPMediaQuery for MPMediaQuery {}
pub trait IMPMediaQuery: Sized + std::ops::Deref {
    unsafe fn initWithFilterPredicates_(&self, filterPredicates: NSSet) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFilterPredicates : filterPredicates)
    }
    unsafe fn addFilterPredicate_(&self, predicate: MPMediaPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addFilterPredicate : predicate)
    }
    unsafe fn removeFilterPredicate_(&self, predicate: MPMediaPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFilterPredicate : predicate)
    }
    unsafe fn filterPredicates(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterPredicates)
    }
    unsafe fn setFilterPredicates_(&self, filterPredicates: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterPredicates : filterPredicates)
    }
    unsafe fn items(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, items)
    }
    unsafe fn collections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collections)
    }
    unsafe fn groupingType(&self) -> MPMediaGrouping
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupingType)
    }
    unsafe fn setGroupingType_(&self, groupingType: MPMediaGrouping)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroupingType : groupingType)
    }
    unsafe fn itemSections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemSections)
    }
    unsafe fn collectionSections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collectionSections)
    }
    unsafe fn albumsQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), albumsQuery)
    }
    unsafe fn artistsQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), artistsQuery)
    }
    unsafe fn songsQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), songsQuery)
    }
    unsafe fn playlistsQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), playlistsQuery)
    }
    unsafe fn podcastsQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), podcastsQuery)
    }
    unsafe fn audiobooksQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), audiobooksQuery)
    }
    unsafe fn compilationsQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), compilationsQuery)
    }
    unsafe fn composersQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), composersQuery)
    }
    unsafe fn genresQuery() -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuery").unwrap(), genresQuery)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaPredicate(pub id);
impl std::ops::Deref for MPMediaPredicate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaPredicate {}
impl MPMediaPredicate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaPredicate").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MPMediaPredicate {}
impl INSObject for MPMediaPredicate {}
impl PNSObject for MPMediaPredicate {}
impl std::convert::TryFrom<NSObject> for MPMediaPredicate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMediaPredicate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaPredicate").unwrap()) };
        if is_kind_of {
            Ok(MPMediaPredicate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMediaPredicate")
        }
    }
}
impl IMPMediaPredicate for MPMediaPredicate {}
pub trait IMPMediaPredicate: Sized + std::ops::Deref {}
pub type MPMediaPredicateComparison = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaPropertyPredicate(pub id);
impl std::ops::Deref for MPMediaPropertyPredicate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaPropertyPredicate {}
impl MPMediaPropertyPredicate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaPropertyPredicate").unwrap(), alloc) })
    }
}
impl IMPMediaPredicate for MPMediaPropertyPredicate {}
impl PNSSecureCoding for MPMediaPropertyPredicate {}
impl From<MPMediaPropertyPredicate> for MPMediaPredicate {
    fn from(child: MPMediaPropertyPredicate) -> MPMediaPredicate {
        MPMediaPredicate(child.0)
    }
}
impl std::convert::TryFrom<MPMediaPredicate> for MPMediaPropertyPredicate {
    type Error = &'static str;
    fn try_from(parent: MPMediaPredicate) -> Result<MPMediaPropertyPredicate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaPropertyPredicate").unwrap()) };
        if is_kind_of {
            Ok(MPMediaPropertyPredicate(parent.0))
        } else {
            Err("This MPMediaPredicate cannot be downcasted to MPMediaPropertyPredicate")
        }
    }
}
impl INSObject for MPMediaPropertyPredicate {}
impl PNSObject for MPMediaPropertyPredicate {}
impl IMPMediaPropertyPredicate for MPMediaPropertyPredicate {}
pub trait IMPMediaPropertyPredicate: Sized + std::ops::Deref {
    unsafe fn property(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, property)
    }
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn comparisonType(&self) -> MPMediaPredicateComparison
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, comparisonType)
    }
    unsafe fn predicateWithValue_forProperty_(
        value: id,
        property: NSString,
    ) -> MPMediaPropertyPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaPropertyPredicate").unwrap(), predicateWithValue : value, forProperty : property)
    }
    unsafe fn predicateWithValue_forProperty_comparisonType_(
        value: id,
        property: NSString,
        comparisonType: MPMediaPredicateComparison,
    ) -> MPMediaPropertyPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaPropertyPredicate").unwrap(), predicateWithValue : value, forProperty : property, comparisonType : comparisonType)
    }
}
impl MPMediaItem_MPMediaQueryAdditions for MPMediaItem {}
pub trait MPMediaItem_MPMediaQueryAdditions: Sized + std::ops::Deref {
    unsafe fn persistentIDPropertyForGroupingType_(groupingType: MPMediaGrouping) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItem").unwrap(), persistentIDPropertyForGroupingType : groupingType)
    }
    unsafe fn titlePropertyForGroupingType_(groupingType: MPMediaGrouping) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaItem").unwrap(), titlePropertyForGroupingType : groupingType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMediaQuerySection(pub id);
impl std::ops::Deref for MPMediaQuerySection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMediaQuerySection {}
impl MPMediaQuerySection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMediaQuerySection").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MPMediaQuerySection {}
impl PNSCopying for MPMediaQuerySection {}
impl INSObject for MPMediaQuerySection {}
impl PNSObject for MPMediaQuerySection {}
impl std::convert::TryFrom<NSObject> for MPMediaQuerySection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMediaQuerySection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMediaQuerySection").unwrap()) };
        if is_kind_of {
            Ok(MPMediaQuerySection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMediaQuerySection")
        }
    }
}
impl IMPMediaQuerySection for MPMediaQuerySection {}
pub trait IMPMediaQuerySection: Sized + std::ops::Deref {
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn range(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, range)
    }
}
pub trait PMPMediaPlayback: Sized + std::ops::Deref {
    unsafe fn prepareToPlay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToPlay)
    }
    unsafe fn play(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, play)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn beginSeekingForward(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginSeekingForward)
    }
    unsafe fn beginSeekingBackward(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginSeekingBackward)
    }
    unsafe fn endSeeking(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endSeeking)
    }
    unsafe fn isPreparedToPlay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPreparedToPlay)
    }
    unsafe fn currentPlaybackTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPlaybackTime)
    }
    unsafe fn setCurrentPlaybackTime_(&self, currentPlaybackTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentPlaybackTime : currentPlaybackTime)
    }
    unsafe fn currentPlaybackRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPlaybackRate)
    }
    unsafe fn setCurrentPlaybackRate_(&self, currentPlaybackRate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentPlaybackRate : currentPlaybackRate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerQueueDescriptor(pub id);
impl std::ops::Deref for MPMusicPlayerQueueDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerQueueDescriptor {}
impl MPMusicPlayerQueueDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerQueueDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for MPMusicPlayerQueueDescriptor {}
impl PNSObject for MPMusicPlayerQueueDescriptor {}
impl std::convert::TryFrom<NSObject> for MPMusicPlayerQueueDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMusicPlayerQueueDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerQueueDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MPMusicPlayerQueueDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMusicPlayerQueueDescriptor")
        }
    }
}
impl IMPMusicPlayerQueueDescriptor for MPMusicPlayerQueueDescriptor {}
pub trait IMPMusicPlayerQueueDescriptor: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerQueueDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerMediaItemQueueDescriptor(pub id);
impl std::ops::Deref for MPMusicPlayerMediaItemQueueDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerMediaItemQueueDescriptor {}
impl MPMusicPlayerMediaItemQueueDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerMediaItemQueueDescriptor").unwrap(), alloc) })
    }
}
impl IMPMusicPlayerQueueDescriptor for MPMusicPlayerMediaItemQueueDescriptor {}
impl From<MPMusicPlayerMediaItemQueueDescriptor> for MPMusicPlayerQueueDescriptor {
    fn from(child: MPMusicPlayerMediaItemQueueDescriptor) -> MPMusicPlayerQueueDescriptor {
        MPMusicPlayerQueueDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MPMusicPlayerQueueDescriptor> for MPMusicPlayerMediaItemQueueDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MPMusicPlayerQueueDescriptor,
    ) -> Result<MPMusicPlayerMediaItemQueueDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerMediaItemQueueDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPMusicPlayerMediaItemQueueDescriptor(parent.0))
        } else {
            Err ("This MPMusicPlayerQueueDescriptor cannot be downcasted to MPMusicPlayerMediaItemQueueDescriptor" ,)
        }
    }
}
impl INSObject for MPMusicPlayerMediaItemQueueDescriptor {}
impl PNSObject for MPMusicPlayerMediaItemQueueDescriptor {}
impl IMPMusicPlayerMediaItemQueueDescriptor for MPMusicPlayerMediaItemQueueDescriptor {}
pub trait IMPMusicPlayerMediaItemQueueDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithQuery_(&self, query: MPMediaQuery) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQuery : query)
    }
    unsafe fn initWithItemCollection_(&self, itemCollection: MPMediaItemCollection) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithItemCollection : itemCollection)
    }
    unsafe fn setStartTime_forItem_(&self, startTime: NSTimeInterval, mediaItem: MPMediaItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartTime : startTime, forItem : mediaItem)
    }
    unsafe fn setEndTime_forItem_(&self, endTime: NSTimeInterval, mediaItem: MPMediaItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndTime : endTime, forItem : mediaItem)
    }
    unsafe fn query(&self) -> MPMediaQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, query)
    }
    unsafe fn itemCollection(&self) -> MPMediaItemCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemCollection)
    }
    unsafe fn startItem(&self) -> MPMediaItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startItem)
    }
    unsafe fn setStartItem_(&self, startItem: MPMediaItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartItem : startItem)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerStoreQueueDescriptor(pub id);
impl std::ops::Deref for MPMusicPlayerStoreQueueDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerStoreQueueDescriptor {}
impl MPMusicPlayerStoreQueueDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerStoreQueueDescriptor").unwrap(), alloc) })
    }
}
impl IMPMusicPlayerQueueDescriptor for MPMusicPlayerStoreQueueDescriptor {}
impl std::convert::TryFrom<MPMusicPlayerQueueDescriptor> for MPMusicPlayerStoreQueueDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MPMusicPlayerQueueDescriptor,
    ) -> Result<MPMusicPlayerStoreQueueDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerStoreQueueDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPMusicPlayerStoreQueueDescriptor(parent.0))
        } else {
            Err ("This MPMusicPlayerQueueDescriptor cannot be downcasted to MPMusicPlayerStoreQueueDescriptor" ,)
        }
    }
}
impl INSObject for MPMusicPlayerStoreQueueDescriptor {}
impl PNSObject for MPMusicPlayerStoreQueueDescriptor {}
impl IMPMusicPlayerStoreQueueDescriptor for MPMusicPlayerStoreQueueDescriptor {}
pub trait IMPMusicPlayerStoreQueueDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithStoreIDs_(&self, storeIDs: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStoreIDs : storeIDs)
    }
    unsafe fn setStartTime_forItemWithStoreID_(&self, startTime: NSTimeInterval, storeID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartTime : startTime, forItemWithStoreID : storeID)
    }
    unsafe fn setEndTime_forItemWithStoreID_(&self, endTime: NSTimeInterval, storeID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndTime : endTime, forItemWithStoreID : storeID)
    }
    unsafe fn storeIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storeIDs)
    }
    unsafe fn setStoreIDs_(&self, storeIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStoreIDs : storeIDs)
    }
    unsafe fn startItemID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startItemID)
    }
    unsafe fn setStartItemID_(&self, startItemID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartItemID : startItemID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerPlayParameters(pub id);
impl std::ops::Deref for MPMusicPlayerPlayParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerPlayParameters {}
impl MPMusicPlayerPlayParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerPlayParameters").unwrap(), alloc) })
    }
}
impl INSObject for MPMusicPlayerPlayParameters {}
impl PNSObject for MPMusicPlayerPlayParameters {}
impl std::convert::TryFrom<NSObject> for MPMusicPlayerPlayParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMusicPlayerPlayParameters, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerPlayParameters").unwrap()) };
        if is_kind_of {
            Ok(MPMusicPlayerPlayParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMusicPlayerPlayParameters")
        }
    }
}
impl IMPMusicPlayerPlayParameters for MPMusicPlayerPlayParameters {}
pub trait IMPMusicPlayerPlayParameters: Sized + std::ops::Deref {
    unsafe fn initWithDictionary_(&self, dictionary: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : dictionary)
    }
    unsafe fn dictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionary)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerPlayParametersQueueDescriptor(pub id);
impl std::ops::Deref for MPMusicPlayerPlayParametersQueueDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerPlayParametersQueueDescriptor {}
impl MPMusicPlayerPlayParametersQueueDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerPlayParametersQueueDescriptor").unwrap(), alloc) })
    }
}
impl IMPMusicPlayerQueueDescriptor for MPMusicPlayerPlayParametersQueueDescriptor {}
impl std::convert::TryFrom<MPMusicPlayerQueueDescriptor>
    for MPMusicPlayerPlayParametersQueueDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MPMusicPlayerQueueDescriptor,
    ) -> Result<MPMusicPlayerPlayParametersQueueDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerPlayParametersQueueDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPMusicPlayerPlayParametersQueueDescriptor(parent.0))
        } else {
            Err ("This MPMusicPlayerQueueDescriptor cannot be downcasted to MPMusicPlayerPlayParametersQueueDescriptor" ,)
        }
    }
}
impl INSObject for MPMusicPlayerPlayParametersQueueDescriptor {}
impl PNSObject for MPMusicPlayerPlayParametersQueueDescriptor {}
impl IMPMusicPlayerPlayParametersQueueDescriptor for MPMusicPlayerPlayParametersQueueDescriptor {}
pub trait IMPMusicPlayerPlayParametersQueueDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPlayParametersQueue_(&self, playParametersQueue: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlayParametersQueue : playParametersQueue)
    }
    unsafe fn setStartTime_forItemWithPlayParameters_(
        &self,
        startTime: NSTimeInterval,
        playParameters: MPMusicPlayerPlayParameters,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartTime : startTime, forItemWithPlayParameters : playParameters)
    }
    unsafe fn setEndTime_forItemWithPlayParameters_(
        &self,
        endTime: NSTimeInterval,
        playParameters: MPMusicPlayerPlayParameters,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndTime : endTime, forItemWithPlayParameters : playParameters)
    }
    unsafe fn playParametersQueue(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playParametersQueue)
    }
    unsafe fn setPlayParametersQueue_(&self, playParametersQueue: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayParametersQueue : playParametersQueue)
    }
    unsafe fn startItemPlayParameters(&self) -> MPMusicPlayerPlayParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startItemPlayParameters)
    }
    unsafe fn setStartItemPlayParameters_(
        &self,
        startItemPlayParameters: MPMusicPlayerPlayParameters,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartItemPlayParameters : startItemPlayParameters)
    }
}
pub type MPMusicPlaybackState = NSInteger;
pub type MPMusicRepeatMode = NSInteger;
pub type MPMusicShuffleMode = NSInteger;
pub trait PMPSystemMusicPlayerController: Sized + std::ops::Deref {
    unsafe fn openToPlayQueueDescriptor_(&self, queueDescriptor: MPMusicPlayerQueueDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openToPlayQueueDescriptor : queueDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerController(pub id);
impl std::ops::Deref for MPMusicPlayerController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerController {}
impl MPMusicPlayerController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerController").unwrap(), alloc) })
    }
}
impl PMPMediaPlayback for MPMusicPlayerController {}
impl INSObject for MPMusicPlayerController {}
impl PNSObject for MPMusicPlayerController {}
impl std::convert::TryFrom<NSObject> for MPMusicPlayerController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMusicPlayerController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerController").unwrap()) };
        if is_kind_of {
            Ok(MPMusicPlayerController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMusicPlayerController")
        }
    }
}
impl IMPMusicPlayerController for MPMusicPlayerController {}
pub trait IMPMusicPlayerController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setQueueWithQuery_(&self, query: MPMediaQuery)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueueWithQuery : query)
    }
    unsafe fn setQueueWithItemCollection_(&self, itemCollection: MPMediaItemCollection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueueWithItemCollection : itemCollection)
    }
    unsafe fn setQueueWithStoreIDs_(&self, storeIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueueWithStoreIDs : storeIDs)
    }
    unsafe fn setQueueWithDescriptor_(&self, descriptor: MPMusicPlayerQueueDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueueWithDescriptor : descriptor)
    }
    unsafe fn prependQueueDescriptor_(&self, descriptor: MPMusicPlayerQueueDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prependQueueDescriptor : descriptor)
    }
    unsafe fn appendQueueDescriptor_(&self, descriptor: MPMusicPlayerQueueDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendQueueDescriptor : descriptor)
    }
    unsafe fn prepareToPlayWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareToPlayWithCompletionHandler : completionHandler)
    }
    unsafe fn skipToNextItem(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skipToNextItem)
    }
    unsafe fn skipToBeginning(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skipToBeginning)
    }
    unsafe fn skipToPreviousItem(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skipToPreviousItem)
    }
    unsafe fn beginGeneratingPlaybackNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginGeneratingPlaybackNotifications)
    }
    unsafe fn endGeneratingPlaybackNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endGeneratingPlaybackNotifications)
    }
    unsafe fn playbackState(&self) -> MPMusicPlaybackState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackState)
    }
    unsafe fn repeatMode(&self) -> MPMusicRepeatMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, repeatMode)
    }
    unsafe fn setRepeatMode_(&self, repeatMode: MPMusicRepeatMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRepeatMode : repeatMode)
    }
    unsafe fn shuffleMode(&self) -> MPMusicShuffleMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shuffleMode)
    }
    unsafe fn setShuffleMode_(&self, shuffleMode: MPMusicShuffleMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShuffleMode : shuffleMode)
    }
    unsafe fn volume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volume)
    }
    unsafe fn setVolume_(&self, volume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolume : volume)
    }
    unsafe fn nowPlayingItem(&self) -> MPMediaItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nowPlayingItem)
    }
    unsafe fn setNowPlayingItem_(&self, nowPlayingItem: MPMediaItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNowPlayingItem : nowPlayingItem)
    }
    unsafe fn indexOfNowPlayingItem(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexOfNowPlayingItem)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerController").unwrap(), new)
    }
    unsafe fn applicationMusicPlayer() -> MPMusicPlayerController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerController").unwrap(), applicationMusicPlayer)
    }
    unsafe fn applicationQueuePlayer() -> MPMusicPlayerApplicationController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerController").unwrap(), applicationQueuePlayer)
    }
    unsafe fn systemMusicPlayer() -> MPMusicPlayerController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerController").unwrap(), systemMusicPlayer)
    }
    unsafe fn iPodMusicPlayer() -> MPMusicPlayerController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerController").unwrap(), iPodMusicPlayer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerControllerQueue(pub id);
impl std::ops::Deref for MPMusicPlayerControllerQueue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerControllerQueue {}
impl MPMusicPlayerControllerQueue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerControllerQueue").unwrap(), alloc) })
    }
}
impl INSObject for MPMusicPlayerControllerQueue {}
impl PNSObject for MPMusicPlayerControllerQueue {}
impl std::convert::TryFrom<NSObject> for MPMusicPlayerControllerQueue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPMusicPlayerControllerQueue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerControllerQueue").unwrap()) };
        if is_kind_of {
            Ok(MPMusicPlayerControllerQueue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPMusicPlayerControllerQueue")
        }
    }
}
impl IMPMusicPlayerControllerQueue for MPMusicPlayerControllerQueue {}
pub trait IMPMusicPlayerControllerQueue: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn items(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, items)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerControllerQueue").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerControllerMutableQueue(pub id);
impl std::ops::Deref for MPMusicPlayerControllerMutableQueue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerControllerMutableQueue {}
impl MPMusicPlayerControllerMutableQueue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerControllerMutableQueue").unwrap(), alloc) })
    }
}
impl IMPMusicPlayerControllerQueue for MPMusicPlayerControllerMutableQueue {}
impl From<MPMusicPlayerControllerMutableQueue> for MPMusicPlayerControllerQueue {
    fn from(child: MPMusicPlayerControllerMutableQueue) -> MPMusicPlayerControllerQueue {
        MPMusicPlayerControllerQueue(child.0)
    }
}
impl std::convert::TryFrom<MPMusicPlayerControllerQueue> for MPMusicPlayerControllerMutableQueue {
    type Error = &'static str;
    fn try_from(
        parent: MPMusicPlayerControllerQueue,
    ) -> Result<MPMusicPlayerControllerMutableQueue, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerControllerMutableQueue").unwrap())
        };
        if is_kind_of {
            Ok(MPMusicPlayerControllerMutableQueue(parent.0))
        } else {
            Err ("This MPMusicPlayerControllerQueue cannot be downcasted to MPMusicPlayerControllerMutableQueue" ,)
        }
    }
}
impl INSObject for MPMusicPlayerControllerMutableQueue {}
impl PNSObject for MPMusicPlayerControllerMutableQueue {}
impl IMPMusicPlayerControllerMutableQueue for MPMusicPlayerControllerMutableQueue {}
pub trait IMPMusicPlayerControllerMutableQueue: Sized + std::ops::Deref {
    unsafe fn insertQueueDescriptor_afterItem_(
        &self,
        queueDescriptor: MPMusicPlayerQueueDescriptor,
        afterItem: MPMediaItem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertQueueDescriptor : queueDescriptor, afterItem : afterItem)
    }
    unsafe fn removeItem_(&self, item: MPMediaItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeItem : item)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPMusicPlayerApplicationController(pub id);
impl std::ops::Deref for MPMusicPlayerApplicationController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPMusicPlayerApplicationController {}
impl MPMusicPlayerApplicationController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPMusicPlayerApplicationController").unwrap(), alloc) })
    }
}
impl IMPMusicPlayerController for MPMusicPlayerApplicationController {}
impl PMPMediaPlayback for MPMusicPlayerApplicationController {}
impl From<MPMusicPlayerApplicationController> for MPMusicPlayerController {
    fn from(child: MPMusicPlayerApplicationController) -> MPMusicPlayerController {
        MPMusicPlayerController(child.0)
    }
}
impl std::convert::TryFrom<MPMusicPlayerController> for MPMusicPlayerApplicationController {
    type Error = &'static str;
    fn try_from(
        parent: MPMusicPlayerController,
    ) -> Result<MPMusicPlayerApplicationController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPMusicPlayerApplicationController").unwrap())
        };
        if is_kind_of {
            Ok(MPMusicPlayerApplicationController(parent.0))
        } else {
            Err ("This MPMusicPlayerController cannot be downcasted to MPMusicPlayerApplicationController" ,)
        }
    }
}
impl INSObject for MPMusicPlayerApplicationController {}
impl PNSObject for MPMusicPlayerApplicationController {}
impl IMPMusicPlayerApplicationController for MPMusicPlayerApplicationController {}
pub trait IMPMusicPlayerApplicationController: Sized + std::ops::Deref {
    unsafe fn performQueueTransaction_completionHandler_(
        &self,
        queueTransaction: *mut ::std::os::raw::c_void,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performQueueTransaction : queueTransaction, completionHandler : completionHandler)
    }
}
pub type MPNowPlayingInfoMediaType = NSUInteger;
pub type MPNowPlayingPlaybackState = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPNowPlayingInfoCenter(pub id);
impl std::ops::Deref for MPNowPlayingInfoCenter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPNowPlayingInfoCenter {}
impl MPNowPlayingInfoCenter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPNowPlayingInfoCenter").unwrap(), alloc) })
    }
}
impl INSObject for MPNowPlayingInfoCenter {}
impl PNSObject for MPNowPlayingInfoCenter {}
impl std::convert::TryFrom<NSObject> for MPNowPlayingInfoCenter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPNowPlayingInfoCenter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPNowPlayingInfoCenter").unwrap()) };
        if is_kind_of {
            Ok(MPNowPlayingInfoCenter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPNowPlayingInfoCenter")
        }
    }
}
impl IMPNowPlayingInfoCenter for MPNowPlayingInfoCenter {}
pub trait IMPNowPlayingInfoCenter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn nowPlayingInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nowPlayingInfo)
    }
    unsafe fn setNowPlayingInfo_(&self, nowPlayingInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNowPlayingInfo : nowPlayingInfo)
    }
    unsafe fn playbackState(&self) -> MPNowPlayingPlaybackState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackState)
    }
    unsafe fn setPlaybackState_(&self, playbackState: MPNowPlayingPlaybackState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaybackState : playbackState)
    }
    unsafe fn defaultCenter() -> MPNowPlayingInfoCenter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPNowPlayingInfoCenter").unwrap(), defaultCenter)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPNowPlayingInfoCenter").unwrap(), new)
    }
    unsafe fn supportedAnimatedArtworkKeys() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPNowPlayingInfoCenter").unwrap(), supportedAnimatedArtworkKeys)
    }
}
pub type MPNowPlayingInfoLanguageOptionType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPNowPlayingInfoLanguageOption(pub id);
impl std::ops::Deref for MPNowPlayingInfoLanguageOption {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPNowPlayingInfoLanguageOption {}
impl MPNowPlayingInfoLanguageOption {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPNowPlayingInfoLanguageOption").unwrap(), alloc) })
    }
}
impl INSObject for MPNowPlayingInfoLanguageOption {}
impl PNSObject for MPNowPlayingInfoLanguageOption {}
impl std::convert::TryFrom<NSObject> for MPNowPlayingInfoLanguageOption {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPNowPlayingInfoLanguageOption, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPNowPlayingInfoLanguageOption").unwrap())
        };
        if is_kind_of {
            Ok(MPNowPlayingInfoLanguageOption(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPNowPlayingInfoLanguageOption")
        }
    }
}
impl IMPNowPlayingInfoLanguageOption for MPNowPlayingInfoLanguageOption {}
pub trait IMPNowPlayingInfoLanguageOption: Sized + std::ops::Deref {
    unsafe fn initWithType_languageTag_characteristics_displayName_identifier_(
        &self,
        languageOptionType: MPNowPlayingInfoLanguageOptionType,
        languageTag: NSString,
        languageOptionCharacteristics: NSArray,
        displayName: NSString,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : languageOptionType, languageTag : languageTag, characteristics : languageOptionCharacteristics, displayName : displayName, identifier : identifier)
    }
    unsafe fn isAutomaticLegibleLanguageOption(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticLegibleLanguageOption)
    }
    unsafe fn isAutomaticAudibleLanguageOption(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticAudibleLanguageOption)
    }
    unsafe fn languageOptionType(&self) -> MPNowPlayingInfoLanguageOptionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageOptionType)
    }
    unsafe fn languageTag(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageTag)
    }
    unsafe fn languageOptionCharacteristics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageOptionCharacteristics)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPNowPlayingInfoLanguageOptionGroup(pub id);
impl std::ops::Deref for MPNowPlayingInfoLanguageOptionGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPNowPlayingInfoLanguageOptionGroup {}
impl MPNowPlayingInfoLanguageOptionGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPNowPlayingInfoLanguageOptionGroup").unwrap(), alloc) })
    }
}
impl INSObject for MPNowPlayingInfoLanguageOptionGroup {}
impl PNSObject for MPNowPlayingInfoLanguageOptionGroup {}
impl std::convert::TryFrom<NSObject> for MPNowPlayingInfoLanguageOptionGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPNowPlayingInfoLanguageOptionGroup, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPNowPlayingInfoLanguageOptionGroup").unwrap())
        };
        if is_kind_of {
            Ok(MPNowPlayingInfoLanguageOptionGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPNowPlayingInfoLanguageOptionGroup")
        }
    }
}
impl IMPNowPlayingInfoLanguageOptionGroup for MPNowPlayingInfoLanguageOptionGroup {}
pub trait IMPNowPlayingInfoLanguageOptionGroup: Sized + std::ops::Deref {
    unsafe fn initWithLanguageOptions_defaultLanguageOption_allowEmptySelection_(
        &self,
        languageOptions: NSArray,
        defaultLanguageOption: MPNowPlayingInfoLanguageOption,
        allowEmptySelection: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLanguageOptions : languageOptions, defaultLanguageOption : defaultLanguageOption, allowEmptySelection : allowEmptySelection)
    }
    unsafe fn languageOptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageOptions)
    }
    unsafe fn defaultLanguageOption(&self) -> MPNowPlayingInfoLanguageOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultLanguageOption)
    }
    unsafe fn allowEmptySelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowEmptySelection)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPAdTimeRange(pub id);
impl std::ops::Deref for MPAdTimeRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPAdTimeRange {}
impl MPAdTimeRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPAdTimeRange").unwrap(), alloc) })
    }
}
impl PNSCopying for MPAdTimeRange {}
impl INSObject for MPAdTimeRange {}
impl PNSObject for MPAdTimeRange {}
impl std::convert::TryFrom<NSObject> for MPAdTimeRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPAdTimeRange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPAdTimeRange").unwrap()) };
        if is_kind_of {
            Ok(MPAdTimeRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPAdTimeRange")
        }
    }
}
impl IMPAdTimeRange for MPAdTimeRange {}
pub trait IMPAdTimeRange: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTimeRange_(&self, timeRange: CMTimeRange) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTimeRange : timeRange)
    }
    unsafe fn timeRange(&self) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRange)
    }
    unsafe fn setTimeRange_(&self, timeRange: CMTimeRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeRange : timeRange)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPAdTimeRange").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPNowPlayingSession(pub id);
impl std::ops::Deref for MPNowPlayingSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPNowPlayingSession {}
impl MPNowPlayingSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPNowPlayingSession").unwrap(), alloc) })
    }
}
impl INSObject for MPNowPlayingSession {}
impl PNSObject for MPNowPlayingSession {}
impl std::convert::TryFrom<NSObject> for MPNowPlayingSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPNowPlayingSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPNowPlayingSession").unwrap()) };
        if is_kind_of {
            Ok(MPNowPlayingSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPNowPlayingSession")
        }
    }
}
impl IMPNowPlayingSession for MPNowPlayingSession {}
pub trait IMPNowPlayingSession: Sized + std::ops::Deref {
    unsafe fn initWithPlayers_(&self, players: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlayers : players)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn becomeActiveIfPossibleWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, becomeActiveIfPossibleWithCompletion : completion)
    }
    unsafe fn addPlayer_(&self, player: AVPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPlayer : player)
    }
    unsafe fn removePlayer_(&self, player: AVPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePlayer : player)
    }
    unsafe fn players(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, players)
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
    unsafe fn automaticallyPublishesNowPlayingInfo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticallyPublishesNowPlayingInfo)
    }
    unsafe fn setAutomaticallyPublishesNowPlayingInfo_(
        &self,
        automaticallyPublishesNowPlayingInfo: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticallyPublishesNowPlayingInfo : automaticallyPublishesNowPlayingInfo)
    }
    unsafe fn nowPlayingInfoCenter(&self) -> MPNowPlayingInfoCenter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nowPlayingInfoCenter)
    }
    unsafe fn remoteCommandCenter(&self) -> MPRemoteCommandCenter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteCommandCenter)
    }
    unsafe fn canBecomeActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canBecomeActive)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPNowPlayingSession").unwrap(), new)
    }
}
pub trait PMPNowPlayingSessionDelegate: Sized + std::ops::Deref {
    unsafe fn nowPlayingSessionDidChangeActive_(&self, nowPlayingSession: MPNowPlayingSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nowPlayingSessionDidChangeActive : nowPlayingSession)
    }
    unsafe fn nowPlayingSessionDidChangeCanBecomeActive_(
        &self,
        nowPlayingSession: MPNowPlayingSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nowPlayingSessionDidChangeCanBecomeActive : nowPlayingSession)
    }
}
pub trait PMPPlayableContentDataSource: Sized + std::ops::Deref {
    unsafe fn beginLoadingChildItemsAtIndexPath_completionHandler_(
        &self,
        indexPath: NSIndexPath,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginLoadingChildItemsAtIndexPath : indexPath, completionHandler : completionHandler)
    }
    unsafe fn childItemsDisplayPlaybackProgressAtIndexPath_(&self, indexPath: NSIndexPath) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, childItemsDisplayPlaybackProgressAtIndexPath : indexPath)
    }
    unsafe fn contentItemForIdentifier_completionHandler_(
        &self,
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentItemForIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn numberOfChildItemsAtIndexPath_(&self, indexPath: NSIndexPath) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, numberOfChildItemsAtIndexPath : indexPath)
    }
    unsafe fn contentItemAtIndexPath_(&self, indexPath: NSIndexPath) -> MPContentItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentItemAtIndexPath : indexPath)
    }
}
pub trait PMPPlayableContentDelegate: Sized + std::ops::Deref {
    unsafe fn playableContentManager_initiatePlaybackOfContentItemAtIndexPath_completionHandler_(
        &self,
        contentManager: MPPlayableContentManager,
        indexPath: NSIndexPath,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playableContentManager : contentManager, initiatePlaybackOfContentItemAtIndexPath : indexPath, completionHandler : completionHandler)
    }
    unsafe fn playableContentManager_initializePlaybackQueueWithCompletionHandler_(
        &self,
        contentManager: MPPlayableContentManager,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playableContentManager : contentManager, initializePlaybackQueueWithCompletionHandler : completionHandler)
    }
    unsafe fn playableContentManager_initializePlaybackQueueWithContentItems_completionHandler_(
        &self,
        contentManager: MPPlayableContentManager,
        contentItems: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playableContentManager : contentManager, initializePlaybackQueueWithContentItems : contentItems, completionHandler : completionHandler)
    }
    unsafe fn playableContentManager_didUpdateContext_(
        &self,
        contentManager: MPPlayableContentManager,
        context: MPPlayableContentManagerContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playableContentManager : contentManager, didUpdateContext : context)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPPlayableContentManager(pub id);
impl std::ops::Deref for MPPlayableContentManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPPlayableContentManager {}
impl MPPlayableContentManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPPlayableContentManager").unwrap(), alloc) })
    }
}
impl INSObject for MPPlayableContentManager {}
impl PNSObject for MPPlayableContentManager {}
impl std::convert::TryFrom<NSObject> for MPPlayableContentManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPPlayableContentManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPPlayableContentManager").unwrap()) };
        if is_kind_of {
            Ok(MPPlayableContentManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPPlayableContentManager")
        }
    }
}
impl IMPPlayableContentManager for MPPlayableContentManager {}
pub trait IMPPlayableContentManager: Sized + std::ops::Deref {
    unsafe fn reloadData(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadData)
    }
    unsafe fn beginUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginUpdates)
    }
    unsafe fn endUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endUpdates)
    }
    unsafe fn dataSource(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSource)
    }
    unsafe fn setDataSource_(&self, dataSource: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataSource : dataSource)
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
    unsafe fn context(&self) -> MPPlayableContentManagerContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn nowPlayingIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nowPlayingIdentifiers)
    }
    unsafe fn setNowPlayingIdentifiers_(&self, nowPlayingIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNowPlayingIdentifiers : nowPlayingIdentifiers)
    }
    unsafe fn sharedContentManager() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPPlayableContentManager").unwrap(), sharedContentManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPPlayableContentManagerContext(pub id);
impl std::ops::Deref for MPPlayableContentManagerContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPPlayableContentManagerContext {}
impl MPPlayableContentManagerContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPPlayableContentManagerContext").unwrap(), alloc) })
    }
}
impl INSObject for MPPlayableContentManagerContext {}
impl PNSObject for MPPlayableContentManagerContext {}
impl std::convert::TryFrom<NSObject> for MPPlayableContentManagerContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPPlayableContentManagerContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPPlayableContentManagerContext").unwrap())
        };
        if is_kind_of {
            Ok(MPPlayableContentManagerContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPPlayableContentManagerContext")
        }
    }
}
impl IMPPlayableContentManagerContext for MPPlayableContentManagerContext {}
pub trait IMPPlayableContentManagerContext: Sized + std::ops::Deref {
    unsafe fn enforcedContentItemsCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enforcedContentItemsCount)
    }
    unsafe fn enforcedContentTreeDepth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enforcedContentTreeDepth)
    }
    unsafe fn contentLimitsEnforced(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentLimitsEnforced)
    }
    unsafe fn contentLimitsEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentLimitsEnabled)
    }
    unsafe fn endpointAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endpointAvailable)
    }
}
pub type MPShuffleType = NSInteger;
pub type MPRepeatType = NSInteger;
pub type MPChangeLanguageOptionSetting = NSInteger;
pub type MPRemoteCommandHandlerStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPRemoteCommand(pub id);
impl std::ops::Deref for MPRemoteCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPRemoteCommand {}
impl MPRemoteCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPRemoteCommand").unwrap(), alloc) })
    }
}
impl INSObject for MPRemoteCommand {}
impl PNSObject for MPRemoteCommand {}
impl std::convert::TryFrom<NSObject> for MPRemoteCommand {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPRemoteCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPRemoteCommand").unwrap()) };
        if is_kind_of {
            Ok(MPRemoteCommand(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPRemoteCommand")
        }
    }
}
impl IMPRemoteCommand for MPRemoteCommand {}
pub trait IMPRemoteCommand: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addTarget_action_(&self, target: id, action: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTarget : target, action : action)
    }
    unsafe fn removeTarget_action_(&self, target: id, action: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTarget : target, action : action)
    }
    unsafe fn removeTarget_(&self, target: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTarget : target)
    }
    unsafe fn addTargetWithHandler_(&self, handler: *mut ::std::os::raw::c_void) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTargetWithHandler : handler)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPRemoteCommand").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSkipIntervalCommand(pub id);
impl std::ops::Deref for MPSkipIntervalCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSkipIntervalCommand {}
impl MPSkipIntervalCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSkipIntervalCommand").unwrap(), alloc) })
    }
}
impl IMPRemoteCommand for MPSkipIntervalCommand {}
impl From<MPSkipIntervalCommand> for MPRemoteCommand {
    fn from(child: MPSkipIntervalCommand) -> MPRemoteCommand {
        MPRemoteCommand(child.0)
    }
}
impl std::convert::TryFrom<MPRemoteCommand> for MPSkipIntervalCommand {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommand) -> Result<MPSkipIntervalCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSkipIntervalCommand").unwrap()) };
        if is_kind_of {
            Ok(MPSkipIntervalCommand(parent.0))
        } else {
            Err("This MPRemoteCommand cannot be downcasted to MPSkipIntervalCommand")
        }
    }
}
impl INSObject for MPSkipIntervalCommand {}
impl PNSObject for MPSkipIntervalCommand {}
impl IMPSkipIntervalCommand for MPSkipIntervalCommand {}
pub trait IMPSkipIntervalCommand: Sized + std::ops::Deref {
    unsafe fn preferredIntervals(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredIntervals)
    }
    unsafe fn setPreferredIntervals_(&self, preferredIntervals: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredIntervals : preferredIntervals)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPFeedbackCommand(pub id);
impl std::ops::Deref for MPFeedbackCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPFeedbackCommand {}
impl MPFeedbackCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPFeedbackCommand").unwrap(), alloc) })
    }
}
impl IMPRemoteCommand for MPFeedbackCommand {}
impl std::convert::TryFrom<MPRemoteCommand> for MPFeedbackCommand {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommand) -> Result<MPFeedbackCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPFeedbackCommand").unwrap()) };
        if is_kind_of {
            Ok(MPFeedbackCommand(parent.0))
        } else {
            Err("This MPRemoteCommand cannot be downcasted to MPFeedbackCommand")
        }
    }
}
impl INSObject for MPFeedbackCommand {}
impl PNSObject for MPFeedbackCommand {}
impl IMPFeedbackCommand for MPFeedbackCommand {}
pub trait IMPFeedbackCommand: Sized + std::ops::Deref {
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn setActive_(&self, active: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActive : active)
    }
    unsafe fn localizedTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedTitle)
    }
    unsafe fn setLocalizedTitle_(&self, localizedTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedTitle : localizedTitle)
    }
    unsafe fn localizedShortTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedShortTitle)
    }
    unsafe fn setLocalizedShortTitle_(&self, localizedShortTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedShortTitle : localizedShortTitle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPRatingCommand(pub id);
impl std::ops::Deref for MPRatingCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPRatingCommand {}
impl MPRatingCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPRatingCommand").unwrap(), alloc) })
    }
}
impl IMPRemoteCommand for MPRatingCommand {}
impl std::convert::TryFrom<MPRemoteCommand> for MPRatingCommand {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommand) -> Result<MPRatingCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPRatingCommand").unwrap()) };
        if is_kind_of {
            Ok(MPRatingCommand(parent.0))
        } else {
            Err("This MPRemoteCommand cannot be downcasted to MPRatingCommand")
        }
    }
}
impl INSObject for MPRatingCommand {}
impl PNSObject for MPRatingCommand {}
impl IMPRatingCommand for MPRatingCommand {}
pub trait IMPRatingCommand: Sized + std::ops::Deref {
    unsafe fn minimumRating(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumRating)
    }
    unsafe fn setMinimumRating_(&self, minimumRating: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumRating : minimumRating)
    }
    unsafe fn maximumRating(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumRating)
    }
    unsafe fn setMaximumRating_(&self, maximumRating: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumRating : maximumRating)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangePlaybackRateCommand(pub id);
impl std::ops::Deref for MPChangePlaybackRateCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangePlaybackRateCommand {}
impl MPChangePlaybackRateCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangePlaybackRateCommand").unwrap(), alloc) })
    }
}
impl IMPRemoteCommand for MPChangePlaybackRateCommand {}
impl std::convert::TryFrom<MPRemoteCommand> for MPChangePlaybackRateCommand {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommand) -> Result<MPChangePlaybackRateCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangePlaybackRateCommand").unwrap()) };
        if is_kind_of {
            Ok(MPChangePlaybackRateCommand(parent.0))
        } else {
            Err("This MPRemoteCommand cannot be downcasted to MPChangePlaybackRateCommand")
        }
    }
}
impl INSObject for MPChangePlaybackRateCommand {}
impl PNSObject for MPChangePlaybackRateCommand {}
impl IMPChangePlaybackRateCommand for MPChangePlaybackRateCommand {}
pub trait IMPChangePlaybackRateCommand: Sized + std::ops::Deref {
    unsafe fn supportedPlaybackRates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedPlaybackRates)
    }
    unsafe fn setSupportedPlaybackRates_(&self, supportedPlaybackRates: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedPlaybackRates : supportedPlaybackRates)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangePlaybackPositionCommand(pub id);
impl std::ops::Deref for MPChangePlaybackPositionCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangePlaybackPositionCommand {}
impl MPChangePlaybackPositionCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangePlaybackPositionCommand").unwrap(), alloc) })
    }
}
impl IMPRemoteCommand for MPChangePlaybackPositionCommand {}
impl std::convert::TryFrom<MPRemoteCommand> for MPChangePlaybackPositionCommand {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommand) -> Result<MPChangePlaybackPositionCommand, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangePlaybackPositionCommand").unwrap())
        };
        if is_kind_of {
            Ok(MPChangePlaybackPositionCommand(parent.0))
        } else {
            Err("This MPRemoteCommand cannot be downcasted to MPChangePlaybackPositionCommand")
        }
    }
}
impl INSObject for MPChangePlaybackPositionCommand {}
impl PNSObject for MPChangePlaybackPositionCommand {}
impl IMPChangePlaybackPositionCommand for MPChangePlaybackPositionCommand {}
pub trait IMPChangePlaybackPositionCommand: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangeShuffleModeCommand(pub id);
impl std::ops::Deref for MPChangeShuffleModeCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangeShuffleModeCommand {}
impl MPChangeShuffleModeCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangeShuffleModeCommand").unwrap(), alloc) })
    }
}
impl IMPRemoteCommand for MPChangeShuffleModeCommand {}
impl std::convert::TryFrom<MPRemoteCommand> for MPChangeShuffleModeCommand {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommand) -> Result<MPChangeShuffleModeCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangeShuffleModeCommand").unwrap()) };
        if is_kind_of {
            Ok(MPChangeShuffleModeCommand(parent.0))
        } else {
            Err("This MPRemoteCommand cannot be downcasted to MPChangeShuffleModeCommand")
        }
    }
}
impl INSObject for MPChangeShuffleModeCommand {}
impl PNSObject for MPChangeShuffleModeCommand {}
impl IMPChangeShuffleModeCommand for MPChangeShuffleModeCommand {}
pub trait IMPChangeShuffleModeCommand: Sized + std::ops::Deref {
    unsafe fn currentShuffleType(&self) -> MPShuffleType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentShuffleType)
    }
    unsafe fn setCurrentShuffleType_(&self, currentShuffleType: MPShuffleType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentShuffleType : currentShuffleType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangeRepeatModeCommand(pub id);
impl std::ops::Deref for MPChangeRepeatModeCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangeRepeatModeCommand {}
impl MPChangeRepeatModeCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangeRepeatModeCommand").unwrap(), alloc) })
    }
}
impl IMPRemoteCommand for MPChangeRepeatModeCommand {}
impl std::convert::TryFrom<MPRemoteCommand> for MPChangeRepeatModeCommand {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommand) -> Result<MPChangeRepeatModeCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangeRepeatModeCommand").unwrap()) };
        if is_kind_of {
            Ok(MPChangeRepeatModeCommand(parent.0))
        } else {
            Err("This MPRemoteCommand cannot be downcasted to MPChangeRepeatModeCommand")
        }
    }
}
impl INSObject for MPChangeRepeatModeCommand {}
impl PNSObject for MPChangeRepeatModeCommand {}
impl IMPChangeRepeatModeCommand for MPChangeRepeatModeCommand {}
pub trait IMPChangeRepeatModeCommand: Sized + std::ops::Deref {
    unsafe fn currentRepeatType(&self) -> MPRepeatType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentRepeatType)
    }
    unsafe fn setCurrentRepeatType_(&self, currentRepeatType: MPRepeatType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentRepeatType : currentRepeatType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPRemoteCommandCenter(pub id);
impl std::ops::Deref for MPRemoteCommandCenter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPRemoteCommandCenter {}
impl MPRemoteCommandCenter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPRemoteCommandCenter").unwrap(), alloc) })
    }
}
impl INSObject for MPRemoteCommandCenter {}
impl PNSObject for MPRemoteCommandCenter {}
impl std::convert::TryFrom<NSObject> for MPRemoteCommandCenter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPRemoteCommandCenter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPRemoteCommandCenter").unwrap()) };
        if is_kind_of {
            Ok(MPRemoteCommandCenter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPRemoteCommandCenter")
        }
    }
}
impl IMPRemoteCommandCenter for MPRemoteCommandCenter {}
pub trait IMPRemoteCommandCenter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn pauseCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pauseCommand)
    }
    unsafe fn playCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playCommand)
    }
    unsafe fn stopCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopCommand)
    }
    unsafe fn togglePlayPauseCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, togglePlayPauseCommand)
    }
    unsafe fn enableLanguageOptionCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableLanguageOptionCommand)
    }
    unsafe fn disableLanguageOptionCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableLanguageOptionCommand)
    }
    unsafe fn changePlaybackRateCommand(&self) -> MPChangePlaybackRateCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changePlaybackRateCommand)
    }
    unsafe fn changeRepeatModeCommand(&self) -> MPChangeRepeatModeCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeRepeatModeCommand)
    }
    unsafe fn changeShuffleModeCommand(&self) -> MPChangeShuffleModeCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeShuffleModeCommand)
    }
    unsafe fn nextTrackCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextTrackCommand)
    }
    unsafe fn previousTrackCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousTrackCommand)
    }
    unsafe fn skipForwardCommand(&self) -> MPSkipIntervalCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skipForwardCommand)
    }
    unsafe fn skipBackwardCommand(&self) -> MPSkipIntervalCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skipBackwardCommand)
    }
    unsafe fn seekForwardCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seekForwardCommand)
    }
    unsafe fn seekBackwardCommand(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seekBackwardCommand)
    }
    unsafe fn changePlaybackPositionCommand(&self) -> MPChangePlaybackPositionCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changePlaybackPositionCommand)
    }
    unsafe fn ratingCommand(&self) -> MPRatingCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ratingCommand)
    }
    unsafe fn likeCommand(&self) -> MPFeedbackCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, likeCommand)
    }
    unsafe fn dislikeCommand(&self) -> MPFeedbackCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dislikeCommand)
    }
    unsafe fn bookmarkCommand(&self) -> MPFeedbackCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bookmarkCommand)
    }
    unsafe fn sharedCommandCenter() -> MPRemoteCommandCenter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPRemoteCommandCenter").unwrap(), sharedCommandCenter)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPRemoteCommandCenter").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPRemoteCommandEvent(pub id);
impl std::ops::Deref for MPRemoteCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPRemoteCommandEvent {}
impl MPRemoteCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPRemoteCommandEvent").unwrap(), alloc) })
    }
}
impl INSObject for MPRemoteCommandEvent {}
impl PNSObject for MPRemoteCommandEvent {}
impl std::convert::TryFrom<NSObject> for MPRemoteCommandEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPRemoteCommandEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPRemoteCommandEvent").unwrap()) };
        if is_kind_of {
            Ok(MPRemoteCommandEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPRemoteCommandEvent")
        }
    }
}
impl IMPRemoteCommandEvent for MPRemoteCommandEvent {}
pub trait IMPRemoteCommandEvent: Sized + std::ops::Deref {
    unsafe fn command(&self) -> MPRemoteCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, command)
    }
    unsafe fn timestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSkipIntervalCommandEvent(pub id);
impl std::ops::Deref for MPSkipIntervalCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSkipIntervalCommandEvent {}
impl MPSkipIntervalCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSkipIntervalCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPSkipIntervalCommandEvent {}
impl From<MPSkipIntervalCommandEvent> for MPRemoteCommandEvent {
    fn from(child: MPSkipIntervalCommandEvent) -> MPRemoteCommandEvent {
        MPRemoteCommandEvent(child.0)
    }
}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPSkipIntervalCommandEvent {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommandEvent) -> Result<MPSkipIntervalCommandEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSkipIntervalCommandEvent").unwrap()) };
        if is_kind_of {
            Ok(MPSkipIntervalCommandEvent(parent.0))
        } else {
            Err("This MPRemoteCommandEvent cannot be downcasted to MPSkipIntervalCommandEvent")
        }
    }
}
impl INSObject for MPSkipIntervalCommandEvent {}
impl PNSObject for MPSkipIntervalCommandEvent {}
impl IMPSkipIntervalCommandEvent for MPSkipIntervalCommandEvent {}
pub trait IMPSkipIntervalCommandEvent: Sized + std::ops::Deref {
    unsafe fn interval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interval)
    }
}
pub type MPSeekCommandEventType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSeekCommandEvent(pub id);
impl std::ops::Deref for MPSeekCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSeekCommandEvent {}
impl MPSeekCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSeekCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPSeekCommandEvent {}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPSeekCommandEvent {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommandEvent) -> Result<MPSeekCommandEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSeekCommandEvent").unwrap()) };
        if is_kind_of {
            Ok(MPSeekCommandEvent(parent.0))
        } else {
            Err("This MPRemoteCommandEvent cannot be downcasted to MPSeekCommandEvent")
        }
    }
}
impl INSObject for MPSeekCommandEvent {}
impl PNSObject for MPSeekCommandEvent {}
impl IMPSeekCommandEvent for MPSeekCommandEvent {}
pub trait IMPSeekCommandEvent: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> MPSeekCommandEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPRatingCommandEvent(pub id);
impl std::ops::Deref for MPRatingCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPRatingCommandEvent {}
impl MPRatingCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPRatingCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPRatingCommandEvent {}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPRatingCommandEvent {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommandEvent) -> Result<MPRatingCommandEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPRatingCommandEvent").unwrap()) };
        if is_kind_of {
            Ok(MPRatingCommandEvent(parent.0))
        } else {
            Err("This MPRemoteCommandEvent cannot be downcasted to MPRatingCommandEvent")
        }
    }
}
impl INSObject for MPRatingCommandEvent {}
impl PNSObject for MPRatingCommandEvent {}
impl IMPRatingCommandEvent for MPRatingCommandEvent {}
pub trait IMPRatingCommandEvent: Sized + std::ops::Deref {
    unsafe fn rating(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rating)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangePlaybackRateCommandEvent(pub id);
impl std::ops::Deref for MPChangePlaybackRateCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangePlaybackRateCommandEvent {}
impl MPChangePlaybackRateCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangePlaybackRateCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPChangePlaybackRateCommandEvent {}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPChangePlaybackRateCommandEvent {
    type Error = &'static str;
    fn try_from(
        parent: MPRemoteCommandEvent,
    ) -> Result<MPChangePlaybackRateCommandEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangePlaybackRateCommandEvent").unwrap())
        };
        if is_kind_of {
            Ok(MPChangePlaybackRateCommandEvent(parent.0))
        } else {
            Err ("This MPRemoteCommandEvent cannot be downcasted to MPChangePlaybackRateCommandEvent" ,)
        }
    }
}
impl INSObject for MPChangePlaybackRateCommandEvent {}
impl PNSObject for MPChangePlaybackRateCommandEvent {}
impl IMPChangePlaybackRateCommandEvent for MPChangePlaybackRateCommandEvent {}
pub trait IMPChangePlaybackRateCommandEvent: Sized + std::ops::Deref {
    unsafe fn playbackRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackRate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPFeedbackCommandEvent(pub id);
impl std::ops::Deref for MPFeedbackCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPFeedbackCommandEvent {}
impl MPFeedbackCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPFeedbackCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPFeedbackCommandEvent {}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPFeedbackCommandEvent {
    type Error = &'static str;
    fn try_from(parent: MPRemoteCommandEvent) -> Result<MPFeedbackCommandEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPFeedbackCommandEvent").unwrap()) };
        if is_kind_of {
            Ok(MPFeedbackCommandEvent(parent.0))
        } else {
            Err("This MPRemoteCommandEvent cannot be downcasted to MPFeedbackCommandEvent")
        }
    }
}
impl INSObject for MPFeedbackCommandEvent {}
impl PNSObject for MPFeedbackCommandEvent {}
impl IMPFeedbackCommandEvent for MPFeedbackCommandEvent {}
pub trait IMPFeedbackCommandEvent: Sized + std::ops::Deref {
    unsafe fn isNegative(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNegative)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangeLanguageOptionCommandEvent(pub id);
impl std::ops::Deref for MPChangeLanguageOptionCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangeLanguageOptionCommandEvent {}
impl MPChangeLanguageOptionCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangeLanguageOptionCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPChangeLanguageOptionCommandEvent {}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPChangeLanguageOptionCommandEvent {
    type Error = &'static str;
    fn try_from(
        parent: MPRemoteCommandEvent,
    ) -> Result<MPChangeLanguageOptionCommandEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangeLanguageOptionCommandEvent").unwrap())
        };
        if is_kind_of {
            Ok(MPChangeLanguageOptionCommandEvent(parent.0))
        } else {
            Err ("This MPRemoteCommandEvent cannot be downcasted to MPChangeLanguageOptionCommandEvent" ,)
        }
    }
}
impl INSObject for MPChangeLanguageOptionCommandEvent {}
impl PNSObject for MPChangeLanguageOptionCommandEvent {}
impl IMPChangeLanguageOptionCommandEvent for MPChangeLanguageOptionCommandEvent {}
pub trait IMPChangeLanguageOptionCommandEvent: Sized + std::ops::Deref {
    unsafe fn languageOption(&self) -> MPNowPlayingInfoLanguageOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageOption)
    }
    unsafe fn setting(&self) -> MPChangeLanguageOptionSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setting)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangePlaybackPositionCommandEvent(pub id);
impl std::ops::Deref for MPChangePlaybackPositionCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangePlaybackPositionCommandEvent {}
impl MPChangePlaybackPositionCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangePlaybackPositionCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPChangePlaybackPositionCommandEvent {}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPChangePlaybackPositionCommandEvent {
    type Error = &'static str;
    fn try_from(
        parent: MPRemoteCommandEvent,
    ) -> Result<MPChangePlaybackPositionCommandEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangePlaybackPositionCommandEvent").unwrap())
        };
        if is_kind_of {
            Ok(MPChangePlaybackPositionCommandEvent(parent.0))
        } else {
            Err ("This MPRemoteCommandEvent cannot be downcasted to MPChangePlaybackPositionCommandEvent" ,)
        }
    }
}
impl INSObject for MPChangePlaybackPositionCommandEvent {}
impl PNSObject for MPChangePlaybackPositionCommandEvent {}
impl IMPChangePlaybackPositionCommandEvent for MPChangePlaybackPositionCommandEvent {}
pub trait IMPChangePlaybackPositionCommandEvent: Sized + std::ops::Deref {
    unsafe fn positionTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, positionTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangeShuffleModeCommandEvent(pub id);
impl std::ops::Deref for MPChangeShuffleModeCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangeShuffleModeCommandEvent {}
impl MPChangeShuffleModeCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangeShuffleModeCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPChangeShuffleModeCommandEvent {}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPChangeShuffleModeCommandEvent {
    type Error = &'static str;
    fn try_from(
        parent: MPRemoteCommandEvent,
    ) -> Result<MPChangeShuffleModeCommandEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangeShuffleModeCommandEvent").unwrap())
        };
        if is_kind_of {
            Ok(MPChangeShuffleModeCommandEvent(parent.0))
        } else {
            Err("This MPRemoteCommandEvent cannot be downcasted to MPChangeShuffleModeCommandEvent")
        }
    }
}
impl INSObject for MPChangeShuffleModeCommandEvent {}
impl PNSObject for MPChangeShuffleModeCommandEvent {}
impl IMPChangeShuffleModeCommandEvent for MPChangeShuffleModeCommandEvent {}
pub trait IMPChangeShuffleModeCommandEvent: Sized + std::ops::Deref {
    unsafe fn shuffleType(&self) -> MPShuffleType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shuffleType)
    }
    unsafe fn preservesShuffleMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preservesShuffleMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPChangeRepeatModeCommandEvent(pub id);
impl std::ops::Deref for MPChangeRepeatModeCommandEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPChangeRepeatModeCommandEvent {}
impl MPChangeRepeatModeCommandEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPChangeRepeatModeCommandEvent").unwrap(), alloc) })
    }
}
impl IMPRemoteCommandEvent for MPChangeRepeatModeCommandEvent {}
impl std::convert::TryFrom<MPRemoteCommandEvent> for MPChangeRepeatModeCommandEvent {
    type Error = &'static str;
    fn try_from(
        parent: MPRemoteCommandEvent,
    ) -> Result<MPChangeRepeatModeCommandEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPChangeRepeatModeCommandEvent").unwrap())
        };
        if is_kind_of {
            Ok(MPChangeRepeatModeCommandEvent(parent.0))
        } else {
            Err("This MPRemoteCommandEvent cannot be downcasted to MPChangeRepeatModeCommandEvent")
        }
    }
}
impl INSObject for MPChangeRepeatModeCommandEvent {}
impl PNSObject for MPChangeRepeatModeCommandEvent {}
impl IMPChangeRepeatModeCommandEvent for MPChangeRepeatModeCommandEvent {}
pub trait IMPChangeRepeatModeCommandEvent: Sized + std::ops::Deref {
    unsafe fn repeatType(&self) -> MPRepeatType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, repeatType)
    }
    unsafe fn preservesRepeatMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preservesRepeatMode)
    }
}
pub trait NSUserActivity_MediaPlayerAdditions: Sized + std::ops::Deref {
    unsafe fn externalMediaContentIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, externalMediaContentIdentifier)
    }
    unsafe fn setExternalMediaContentIdentifier_(&self, externalMediaContentIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExternalMediaContentIdentifier : externalMediaContentIdentifier)
    }
}
pub trait AVPlayerItem_MPAdditions: Sized + std::ops::Deref {
    unsafe fn nowPlayingInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nowPlayingInfo)
    }
    unsafe fn setNowPlayingInfo_(&self, nowPlayingInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNowPlayingInfo : nowPlayingInfo)
    }
}
unsafe extern "C" {
    pub static MPErrorDomain: NSString;
}
unsafe extern "C" {
    pub static MPMediaEntityPropertyPersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyPersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyMediaType: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyTitle: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyAlbumTitle: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyAlbumPersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyArtist: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyArtistPersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyAlbumArtist: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyAlbumArtistPersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyGenre: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyGenrePersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyComposer: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyComposerPersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyPlaybackDuration: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyAlbumTrackNumber: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyAlbumTrackCount: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyDiscNumber: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyDiscCount: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyArtwork: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyIsExplicit: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyLyrics: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyIsCompilation: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyReleaseDate: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyBeatsPerMinute: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyComments: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyAssetURL: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyIsCloudItem: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyHasProtectedAsset: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyPodcastTitle: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyPodcastPersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyPlayCount: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertySkipCount: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyRating: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyLastPlayedDate: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyUserGrouping: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyBookmarkTime: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyDateAdded: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyPlaybackStoreID: NSString;
}
unsafe extern "C" {
    pub static MPMediaItemPropertyIsPreorder: NSString;
}
unsafe extern "C" {
    pub static MPMediaLibraryDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static MPMediaPlaylistPropertyPersistentID: NSString;
}
unsafe extern "C" {
    pub static MPMediaPlaylistPropertyCloudGlobalID: NSString;
}
unsafe extern "C" {
    pub static MPMediaPlaylistPropertyName: NSString;
}
unsafe extern "C" {
    pub static MPMediaPlaylistPropertyPlaylistAttributes: NSString;
}
unsafe extern "C" {
    pub static MPMediaPlaylistPropertySeedItems: NSString;
}
unsafe extern "C" {
    pub static MPMediaPlaylistPropertyDescriptionText: NSString;
}
unsafe extern "C" {
    pub static MPMediaPlaylistPropertyAuthorDisplayName: NSString;
}
unsafe extern "C" {
    pub static MPMediaPlaybackIsPreparedToPlayDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static MPMusicPlayerControllerPlaybackStateDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MPMusicPlayerControllerNowPlayingItemDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MPMusicPlayerControllerVolumeDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MPMusicPlayerControllerQueueDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyElapsedPlaybackTime: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyPlaybackRate: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyDefaultPlaybackRate: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyPlaybackQueueIndex: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyPlaybackQueueCount: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyChapterNumber: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyChapterCount: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyIsLiveStream: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyAvailableLanguageOptions: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyCurrentLanguageOptions: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoCollectionIdentifier: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyExternalContentIdentifier: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyExternalUserProfileIdentifier: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyServiceIdentifier: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyPlaybackProgress: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyMediaType: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyAssetURL: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyCurrentPlaybackDate: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyAdTimeRanges: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyCreditsStartTime: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyInternationalStandardRecordingCode: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoPropertyExcludeFromSuggestions: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoProperty1x1AnimatedArtwork: NSString;
}
unsafe extern "C" {
    pub static MPNowPlayingInfoProperty3x4AnimatedArtwork: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicIsMainProgramContent: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicIsAuxiliaryContent: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicTranscribesSpokenDialog: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicDescribesMusicAndSound: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicEasyToRead: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicDescribesVideo: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicLanguageTranslation: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicDubbedTranslation: NSString;
}
unsafe extern "C" {
    pub static MPLanguageOptionCharacteristicVoiceOverTranslation: NSString;
}

unsafe impl objc2::encode::RefEncode for MPContentItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPContentItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaEntity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaEntity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaItemArtwork {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaItemArtwork {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaItemAnimatedArtwork {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaItemAnimatedArtwork {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaItemCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaItemCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaPlaylist {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaPlaylist {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaPlaylistCreationMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaPlaylistCreationMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaPredicate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaPredicate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaPropertyPredicate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaPropertyPredicate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMediaQuerySection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMediaQuerySection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerQueueDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerQueueDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerMediaItemQueueDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerMediaItemQueueDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerStoreQueueDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerStoreQueueDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerPlayParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerPlayParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerPlayParametersQueueDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerPlayParametersQueueDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerControllerQueue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerControllerQueue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerControllerMutableQueue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerControllerMutableQueue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPMusicPlayerApplicationController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPMusicPlayerApplicationController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPNowPlayingInfoCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPNowPlayingInfoCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPNowPlayingInfoLanguageOption {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPNowPlayingInfoLanguageOption {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPNowPlayingInfoLanguageOptionGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPNowPlayingInfoLanguageOptionGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPAdTimeRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPAdTimeRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPNowPlayingSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPNowPlayingSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPPlayableContentManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPPlayableContentManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPPlayableContentManagerContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPPlayableContentManagerContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPRemoteCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPRemoteCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSkipIntervalCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSkipIntervalCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPFeedbackCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPFeedbackCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPRatingCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPRatingCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangePlaybackRateCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangePlaybackRateCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangePlaybackPositionCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangePlaybackPositionCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangeShuffleModeCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangeShuffleModeCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangeRepeatModeCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangeRepeatModeCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPRemoteCommandCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPRemoteCommandCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPRemoteCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPRemoteCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSkipIntervalCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSkipIntervalCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSeekCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSeekCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPRatingCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPRatingCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangePlaybackRateCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangePlaybackRateCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPFeedbackCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPFeedbackCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangeLanguageOptionCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangeLanguageOptionCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangePlaybackPositionCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangePlaybackPositionCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangeShuffleModeCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangeShuffleModeCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPChangeRepeatModeCommandEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPChangeRepeatModeCommandEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
