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
pub type MLMediaSourceType = NSUInteger;
pub type MLMediaType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLMediaLibrary(pub id);
impl std::ops::Deref for MLMediaLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLMediaLibrary {}
impl MLMediaLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLMediaLibrary").unwrap(), alloc) })
    }
}
impl INSObject for MLMediaLibrary {}
impl PNSObject for MLMediaLibrary {}
impl std::convert::TryFrom<NSObject> for MLMediaLibrary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLMediaLibrary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLMediaLibrary").unwrap()) };
        if is_kind_of {
            Ok(MLMediaLibrary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLMediaLibrary")
        }
    }
}
impl IMLMediaLibrary for MLMediaLibrary {}
pub trait IMLMediaLibrary: Sized + std::ops::Deref {
    unsafe fn initWithOptions_(&self, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOptions : options)
    }
    unsafe fn mediaSources(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSources)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLMediaSource(pub id);
impl std::ops::Deref for MLMediaSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLMediaSource {}
impl MLMediaSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLMediaSource").unwrap(), alloc) })
    }
}
impl INSObject for MLMediaSource {}
impl PNSObject for MLMediaSource {}
impl std::convert::TryFrom<NSObject> for MLMediaSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLMediaSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLMediaSource").unwrap()) };
        if is_kind_of {
            Ok(MLMediaSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLMediaSource")
        }
    }
}
impl IMLMediaSource for MLMediaSource {}
pub trait IMLMediaSource: Sized + std::ops::Deref {
    unsafe fn mediaGroupForIdentifier_(&self, mediaGroupIdentifier: NSString) -> MLMediaGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mediaGroupForIdentifier : mediaGroupIdentifier)
    }
    unsafe fn mediaGroupsForIdentifiers_(&self, mediaGroupIdentifiers: NSArray) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mediaGroupsForIdentifiers : mediaGroupIdentifiers)
    }
    unsafe fn mediaObjectForIdentifier_(&self, mediaObjectIdentifier: NSString) -> MLMediaObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mediaObjectForIdentifier : mediaObjectIdentifier)
    }
    unsafe fn mediaObjectsForIdentifiers_(&self, mediaObjectIdentifiers: NSArray) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mediaObjectsForIdentifiers : mediaObjectIdentifiers)
    }
    unsafe fn mediaLibrary(&self) -> MLMediaLibrary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaLibrary)
    }
    unsafe fn mediaSourceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSourceIdentifier)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn rootMediaGroup(&self) -> MLMediaGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootMediaGroup)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLMediaGroup(pub id);
impl std::ops::Deref for MLMediaGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLMediaGroup {}
impl MLMediaGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLMediaGroup").unwrap(), alloc) })
    }
}
impl INSObject for MLMediaGroup {}
impl PNSObject for MLMediaGroup {}
impl std::convert::TryFrom<NSObject> for MLMediaGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLMediaGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLMediaGroup").unwrap()) };
        if is_kind_of {
            Ok(MLMediaGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLMediaGroup")
        }
    }
}
impl IMLMediaGroup for MLMediaGroup {}
pub trait IMLMediaGroup: Sized + std::ops::Deref {
    unsafe fn mediaLibrary(&self) -> MLMediaLibrary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaLibrary)
    }
    unsafe fn parent(&self) -> MLMediaGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parent)
    }
    unsafe fn mediaSourceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSourceIdentifier)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn typeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typeIdentifier)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn childGroups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childGroups)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn modificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modificationDate)
    }
    unsafe fn iconImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconImage)
    }
    unsafe fn mediaObjects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaObjects)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLMediaObject(pub id);
impl std::ops::Deref for MLMediaObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLMediaObject {}
impl MLMediaObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLMediaObject").unwrap(), alloc) })
    }
}
impl INSObject for MLMediaObject {}
impl PNSObject for MLMediaObject {}
impl std::convert::TryFrom<NSObject> for MLMediaObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLMediaObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLMediaObject").unwrap()) };
        if is_kind_of {
            Ok(MLMediaObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLMediaObject")
        }
    }
}
impl IMLMediaObject for MLMediaObject {}
pub trait IMLMediaObject: Sized + std::ops::Deref {
    unsafe fn mediaLibrary(&self) -> MLMediaLibrary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaLibrary)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn mediaSourceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSourceIdentifier)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn mediaType(&self) -> MLMediaType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaType)
    }
    unsafe fn contentType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn originalURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalURL)
    }
    unsafe fn fileSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSize)
    }
    unsafe fn modificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modificationDate)
    }
    unsafe fn thumbnailURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnailURL)
    }
    unsafe fn artworkImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artworkImage)
    }
}
unsafe extern "C" {
    pub static MLFolderRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLFolderGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesPurchasedPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesPodcastPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesVideoPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesSmartPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesFolderPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesMoviesPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesTVShowsPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesAudioBooksPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesMusicPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesMusicVideosPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesGeniusPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesSavedGeniusPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiTunesiTunesUPlaylistTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosSharedGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosAlbumsGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosFolderTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosSmartAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosPublishedAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosAllMomentsGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosMomentGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosAllCollectionsGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosCollectionGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosAllYearsGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosYearGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosLastImportGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosMyPhotoStreamTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosSharedPhotoStreamTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosFavoritesGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosFrontCameraGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosLivePhotosGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosLongExposureGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosAnimatedGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosPanoramasGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosVideosGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosSloMoGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosDepthEffectGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosTimelapseGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosBurstGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosScreenshotGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosFacesAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLPhotosAllPhotosAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoLibraryAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoEventsFolderTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoSmartAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoEventAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoLastImportAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoLastNMonthsAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoFlaggedAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoFolderAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoSubscribedAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoFacesAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoPlacesAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoPlacesCountryAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoPlacesProvinceAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoPlacesCityAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoPlacesPointOfInterestAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoFacebookAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoFlickrAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoFacebookGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoFlickrGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoSlideShowAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoLastViewedEventAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiPhotoPhotoStreamAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureUserAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureUserSmartAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureProjectAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureFolderAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureProjectFolderAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureLightTableTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureFlickrGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureFlickrAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureFacebookGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureFacebookAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureSmugMugGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureSmugMugAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureSlideShowTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureAllPhotosTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureFlaggedTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureAllProjectsTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureFacesAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLAperturePlacesAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLAperturePlacesCountryAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLAperturePlacesProvinceAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLAperturePlacesCityAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLAperturePlacesPointOfInterestAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureLastImportAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureLastNMonthsAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLApertureLastViewedEventAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLAperturePhotoStreamAlbumTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLGarageBandRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLGarageBandFolderGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLLogicRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLLogicBouncesGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLLogicProjectsGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLLogicProjectTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiMovieRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiMovieEventGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiMovieProjectGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiMovieEventLibraryGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiMovieEventCalendarGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLiMovieFolderGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLFinalCutRootGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLFinalCutEventGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLFinalCutProjectGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLFinalCutEventLibraryGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLFinalCutEventCalendarGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLFinalCutFolderGroupTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectDurationKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectArtistKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectAlbumKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectGenreKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectKindKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectTrackNumberKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectBitRateKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectSampleRateKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectChannelCountKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectResolutionStringKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectCommentsKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectKeywordsKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaObjectProtectedKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourcePhotosIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceiPhotoIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceiTunesIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceApertureIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceiMovieIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceFinalCutIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceGarageBandIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceLogicIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourcePhotoBoothIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceCustomFoldersIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceMoviesFolderIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaSourceAppDefinedFoldersIdentifier: NSString;
}
unsafe extern "C" {
    pub static MLMediaLoadSourceTypesKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaLoadIncludeSourcesKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaLoadExcludeSourcesKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaLoadFoldersKey: NSString;
}
unsafe extern "C" {
    pub static MLMediaLoadAppleLoops: NSString;
}
unsafe extern "C" {
    pub static MLMediaLoadMoviesFolder: NSString;
}
unsafe extern "C" {
    pub static MLMediaLoadAppFoldersKey: NSString;
}

unsafe impl objc2::encode::RefEncode for MLMediaLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLMediaLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLMediaSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLMediaSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLMediaGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLMediaGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLMediaObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLMediaObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
