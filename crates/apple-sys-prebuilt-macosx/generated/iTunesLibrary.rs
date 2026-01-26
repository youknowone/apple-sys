#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ITLibAlbum(pub id);
impl std::ops::Deref for ITLibAlbum {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ITLibAlbum {}
impl ITLibAlbum {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibAlbum").unwrap(), alloc) })
    }
}
impl INSObject for ITLibAlbum {}
impl PNSObject for ITLibAlbum {}
impl std::convert::TryFrom<NSObject> for ITLibAlbum {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ITLibAlbum, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ITLibAlbum").unwrap()) };
        if is_kind_of {
            Ok(ITLibAlbum(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ITLibAlbum")
        }
    }
}
impl IITLibAlbum for ITLibAlbum {}
pub trait IITLibAlbum: Sized + std::ops::Deref {
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn sortTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortTitle)
    }
    unsafe fn isCompilation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompilation)
    }
    unsafe fn artist(&self) -> ITLibArtist
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artist)
    }
    unsafe fn discCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discCount)
    }
    unsafe fn discNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discNumber)
    }
    unsafe fn rating(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rating)
    }
    unsafe fn isRatingComputed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRatingComputed)
    }
    unsafe fn isGapless(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGapless)
    }
    unsafe fn trackCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackCount)
    }
    unsafe fn albumArtist(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, albumArtist)
    }
    unsafe fn sortAlbumArtist(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortAlbumArtist)
    }
    unsafe fn persistentID(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ITLibArtist(pub id);
impl std::ops::Deref for ITLibArtist {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ITLibArtist {}
impl ITLibArtist {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibArtist").unwrap(), alloc) })
    }
}
impl INSObject for ITLibArtist {}
impl PNSObject for ITLibArtist {}
impl std::convert::TryFrom<NSObject> for ITLibArtist {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ITLibArtist, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ITLibArtist").unwrap()) };
        if is_kind_of {
            Ok(ITLibArtist(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ITLibArtist")
        }
    }
}
impl IITLibArtist for ITLibArtist {}
pub trait IITLibArtist: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn sortName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortName)
    }
    unsafe fn persistentID(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentID)
    }
}
pub type ITLibArtworkFormat = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ITLibArtwork(pub id);
impl std::ops::Deref for ITLibArtwork {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ITLibArtwork {}
impl ITLibArtwork {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibArtwork").unwrap(), alloc) })
    }
}
impl INSObject for ITLibArtwork {}
impl PNSObject for ITLibArtwork {}
impl std::convert::TryFrom<NSObject> for ITLibArtwork {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ITLibArtwork, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ITLibArtwork").unwrap()) };
        if is_kind_of {
            Ok(ITLibArtwork(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ITLibArtwork")
        }
    }
}
impl IITLibArtwork for ITLibArtwork {}
pub trait IITLibArtwork: Sized + std::ops::Deref {
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn imageData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageData)
    }
    unsafe fn imageDataFormat(&self) -> ITLibArtworkFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageDataFormat)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ITLibMediaEntity(pub id);
impl std::ops::Deref for ITLibMediaEntity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ITLibMediaEntity {}
impl ITLibMediaEntity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibMediaEntity").unwrap(), alloc) })
    }
}
impl INSObject for ITLibMediaEntity {}
impl PNSObject for ITLibMediaEntity {}
impl std::convert::TryFrom<NSObject> for ITLibMediaEntity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ITLibMediaEntity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ITLibMediaEntity").unwrap()) };
        if is_kind_of {
            Ok(ITLibMediaEntity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ITLibMediaEntity")
        }
    }
}
impl IITLibMediaEntity for ITLibMediaEntity {}
pub trait IITLibMediaEntity: Sized + std::ops::Deref {
    unsafe fn valueForProperty_(&self, property: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForProperty : property)
    }
    unsafe fn enumerateValuesForProperties_usingBlock_(
        &self,
        properties: NSSet,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateValuesForProperties : properties, usingBlock : block)
    }
    unsafe fn enumerateValuesExceptForProperties_usingBlock_(
        &self,
        properties: NSSet,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateValuesExceptForProperties : properties, usingBlock : block)
    }
    unsafe fn persistentID(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentID)
    }
}
pub type ITLibMediaItemMediaKind = NSUInteger;
pub type ITLibMediaItemLyricsContentRating = NSUInteger;
pub type ITLibMediaItemLocationType = NSUInteger;
pub type ITLibMediaItemPlayStatus = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ITLibMediaItem(pub id);
impl std::ops::Deref for ITLibMediaItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ITLibMediaItem {}
impl ITLibMediaItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibMediaItem").unwrap(), alloc) })
    }
}
impl IITLibMediaEntity for ITLibMediaItem {}
impl From<ITLibMediaItem> for ITLibMediaEntity {
    fn from(child: ITLibMediaItem) -> ITLibMediaEntity {
        ITLibMediaEntity(child.0)
    }
}
impl std::convert::TryFrom<ITLibMediaEntity> for ITLibMediaItem {
    type Error = &'static str;
    fn try_from(parent: ITLibMediaEntity) -> Result<ITLibMediaItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ITLibMediaItem").unwrap()) };
        if is_kind_of {
            Ok(ITLibMediaItem(parent.0))
        } else {
            Err("This ITLibMediaEntity cannot be downcasted to ITLibMediaItem")
        }
    }
}
impl INSObject for ITLibMediaItem {}
impl PNSObject for ITLibMediaItem {}
impl IITLibMediaItem for ITLibMediaItem {}
pub trait IITLibMediaItem: Sized + std::ops::Deref {
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn sortTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortTitle)
    }
    unsafe fn artist(&self) -> ITLibArtist
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artist)
    }
    unsafe fn composer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composer)
    }
    unsafe fn sortComposer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortComposer)
    }
    unsafe fn rating(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rating)
    }
    unsafe fn isRatingComputed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRatingComputed)
    }
    unsafe fn startTime(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startTime)
    }
    unsafe fn stopTime(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopTime)
    }
    unsafe fn album(&self) -> ITLibAlbum
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, album)
    }
    unsafe fn genre(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, genre)
    }
    unsafe fn kind(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kind)
    }
    unsafe fn mediaKind(&self) -> ITLibMediaItemMediaKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaKind)
    }
    unsafe fn fileSize(&self) -> ::std::os::raw::c_ulonglong
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSize)
    }
    unsafe fn size(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn totalTime(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalTime)
    }
    unsafe fn trackNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackNumber)
    }
    unsafe fn category(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
    unsafe fn description(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, description)
    }
    unsafe fn lyricsContentRating(&self) -> ITLibMediaItemLyricsContentRating
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lyricsContentRating)
    }
    unsafe fn contentRating(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentRating)
    }
    unsafe fn modifiedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifiedDate)
    }
    unsafe fn addedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addedDate)
    }
    unsafe fn bitrate(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bitrate)
    }
    unsafe fn sampleRate(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleRate)
    }
    unsafe fn beatsPerMinute(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beatsPerMinute)
    }
    unsafe fn playCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playCount)
    }
    unsafe fn lastPlayedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastPlayedDate)
    }
    unsafe fn playStatus(&self) -> ITLibMediaItemPlayStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playStatus)
    }
    unsafe fn location(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn hasArtworkAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasArtworkAvailable)
    }
    unsafe fn artwork(&self) -> ITLibArtwork
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artwork)
    }
    unsafe fn comments(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, comments)
    }
    unsafe fn isPurchased(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPurchased)
    }
    unsafe fn isCloud(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCloud)
    }
    unsafe fn isDRMProtected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDRMProtected)
    }
    unsafe fn isVideo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVideo)
    }
    unsafe fn videoInfo(&self) -> ITLibMediaItemVideoInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoInfo)
    }
    unsafe fn releaseDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseDate)
    }
    unsafe fn year(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, year)
    }
    unsafe fn fileType(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileType)
    }
    unsafe fn skipCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skipCount)
    }
    unsafe fn skipDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skipDate)
    }
    unsafe fn voiceOverLanguage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voiceOverLanguage)
    }
    unsafe fn volumeAdjustment(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volumeAdjustment)
    }
    unsafe fn volumeNormalizationEnergy(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volumeNormalizationEnergy)
    }
    unsafe fn isUserDisabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserDisabled)
    }
    unsafe fn grouping(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grouping)
    }
    unsafe fn locationType(&self) -> ITLibMediaItemLocationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ITLibMediaItemVideoInfo(pub id);
impl std::ops::Deref for ITLibMediaItemVideoInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ITLibMediaItemVideoInfo {}
impl ITLibMediaItemVideoInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibMediaItemVideoInfo").unwrap(), alloc) })
    }
}
impl INSObject for ITLibMediaItemVideoInfo {}
impl PNSObject for ITLibMediaItemVideoInfo {}
impl std::convert::TryFrom<NSObject> for ITLibMediaItemVideoInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ITLibMediaItemVideoInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ITLibMediaItemVideoInfo").unwrap()) };
        if is_kind_of {
            Ok(ITLibMediaItemVideoInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ITLibMediaItemVideoInfo")
        }
    }
}
impl IITLibMediaItemVideoInfo for ITLibMediaItemVideoInfo {}
pub trait IITLibMediaItemVideoInfo: Sized + std::ops::Deref {
    unsafe fn series(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, series)
    }
    unsafe fn sortSeries(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortSeries)
    }
    unsafe fn season(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, season)
    }
    unsafe fn episode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, episode)
    }
    unsafe fn episodeOrder(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, episodeOrder)
    }
    unsafe fn isHD(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHD)
    }
    unsafe fn videoWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoWidth)
    }
    unsafe fn videoHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoHeight)
    }
}
pub type ITLibDistinguishedPlaylistKind = NSUInteger;
pub type ITLibPlaylistKind = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ITLibPlaylist(pub id);
impl std::ops::Deref for ITLibPlaylist {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ITLibPlaylist {}
impl ITLibPlaylist {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibPlaylist").unwrap(), alloc) })
    }
}
impl IITLibMediaEntity for ITLibPlaylist {}
impl std::convert::TryFrom<ITLibMediaEntity> for ITLibPlaylist {
    type Error = &'static str;
    fn try_from(parent: ITLibMediaEntity) -> Result<ITLibPlaylist, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ITLibPlaylist").unwrap()) };
        if is_kind_of {
            Ok(ITLibPlaylist(parent.0))
        } else {
            Err("This ITLibMediaEntity cannot be downcasted to ITLibPlaylist")
        }
    }
}
impl INSObject for ITLibPlaylist {}
impl PNSObject for ITLibPlaylist {}
impl IITLibPlaylist for ITLibPlaylist {}
pub trait IITLibPlaylist: Sized + std::ops::Deref {
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
    unsafe fn parentID(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentID)
    }
    unsafe fn isVisible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVisible)
    }
    unsafe fn isAllItemsPlaylist(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAllItemsPlaylist)
    }
    unsafe fn items(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, items)
    }
    unsafe fn distinguishedKind(&self) -> ITLibDistinguishedPlaylistKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distinguishedKind)
    }
    unsafe fn kind(&self) -> ITLibPlaylistKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kind)
    }
    unsafe fn isMaster(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMaster)
    }
}
pub type ITLibExportFeature = NSUInteger;
pub type ITLibInitOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ITLibrary(pub id);
impl std::ops::Deref for ITLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ITLibrary {}
impl ITLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibrary").unwrap(), alloc) })
    }
}
impl INSObject for ITLibrary {}
impl PNSObject for ITLibrary {}
impl std::convert::TryFrom<NSObject> for ITLibrary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ITLibrary, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ITLibrary").unwrap()) };
        if is_kind_of {
            Ok(ITLibrary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ITLibrary")
        }
    }
}
impl IITLibrary for ITLibrary {}
pub trait IITLibrary: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAPIVersion_error_(
        &self,
        requestedAPIVersion: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAPIVersion : requestedAPIVersion, error : error)
    }
    unsafe fn initWithAPIVersion_options_error_(
        &self,
        requestedAPIVersion: NSString,
        options: ITLibInitOptions,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAPIVersion : requestedAPIVersion, options : options, error : error)
    }
    unsafe fn artworkForMediaFile_(&self, mediaFileURL: NSURL) -> ITLibArtwork
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, artworkForMediaFile : mediaFileURL)
    }
    unsafe fn reloadData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadData)
    }
    unsafe fn unloadData(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unloadData)
    }
    unsafe fn applicationVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationVersion)
    }
    unsafe fn features(&self) -> ITLibExportFeature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, features)
    }
    unsafe fn apiMajorVersion(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, apiMajorVersion)
    }
    unsafe fn apiMinorVersion(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, apiMinorVersion)
    }
    unsafe fn mediaFolderLocation(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaFolderLocation)
    }
    unsafe fn musicFolderLocation(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, musicFolderLocation)
    }
    unsafe fn shouldShowContentRating(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldShowContentRating)
    }
    unsafe fn allMediaItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allMediaItems)
    }
    unsafe fn allPlaylists(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allPlaylists)
    }
    unsafe fn libraryWithAPIVersion_error_(
        requestedAPIVersion: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibrary").unwrap(), libraryWithAPIVersion : requestedAPIVersion, error : error)
    }
    unsafe fn libraryWithAPIVersion_options_error_(
        requestedAPIVersion: NSString,
        options: ITLibInitOptions,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ITLibrary").unwrap(), libraryWithAPIVersion : requestedAPIVersion, options : options, error : error)
    }
}
unsafe extern "C" {
    pub static ITLibMediaEntityPropertyPersistentID: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumTitle: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertySortAlbumTitle: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumArtist: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumRating: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumRatingComputed: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertySortAlbumArtist: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumIsGapless: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumIsCompilation: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumDiscCount: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumDiscNumber: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAlbumTrackCount: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyArtistName: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertySortArtistName: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVideoIsHD: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVideoWidth: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVideoHeight: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVideoSeries: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVideoSortSeries: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVideoSeason: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVideoEpisode: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVideoEpisodeOrder: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyHasArtwork: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyBitRate: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyBeatsPerMinute: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyCategory: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyComments: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyComposer: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertySortComposer: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyContentRating: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyLyricsContentRating: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyAddedDate: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyModifiedDate: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyDescription: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyIsUserDisabled: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyFileType: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyGenre: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyGrouping: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyIsVideo: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyKind: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyTitle: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertySortTitle: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVolumeNormalizationEnergy: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyPlayCount: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyLastPlayDate: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyPlayStatus: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyIsDRMProtected: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyIsPurchased: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyMovementCount: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyMovementName: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyMovementNumber: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyRating: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyRatingComputed: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyReleaseDate: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertySampleRate: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertySize: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyFileSize: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyUserSkipCount: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertySkipDate: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyStartTime: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyStopTime: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyTotalTime: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyTrackNumber: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyLocationType: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVoiceOverLanguage: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyVolumeAdjustment: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyWork: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyYear: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyMediaKind: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyLocation: NSString;
}
unsafe extern "C" {
    pub static ITLibMediaItemPropertyArtwork: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyName: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyAllItemsPlaylist: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyDistinguisedKind: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyPrimary: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyParentPersistentID: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyVisible: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyItems: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyKind: NSString;
}
unsafe extern "C" {
    pub static ITLibPlaylistPropertyMaster: NSString;
}
unsafe extern "C" {
    pub static ITLibraryDidChangeNotification: NSNotificationName;
}

unsafe impl objc2::encode::RefEncode for ITLibAlbum {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ITLibAlbum {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ITLibArtist {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ITLibArtist {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ITLibArtwork {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ITLibArtwork {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ITLibMediaEntity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ITLibMediaEntity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ITLibMediaItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ITLibMediaItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ITLibMediaItemVideoInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ITLibMediaItemVideoInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ITLibPlaylist {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ITLibPlaylist {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ITLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ITLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
