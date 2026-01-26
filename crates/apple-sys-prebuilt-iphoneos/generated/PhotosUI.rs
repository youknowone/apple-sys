#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::MapKit::*;
#[allow(unused_imports)]
use crate::Photos::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type PHLivePhotoViewPlaybackStyle = NSInteger;
pub type PHLivePhotoViewContentMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHLivePhotoView(pub id);
impl std::ops::Deref for PHLivePhotoView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHLivePhotoView {}
impl PHLivePhotoView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHLivePhotoView").unwrap(), alloc) })
    }
}
impl INSView for PHLivePhotoView {}
impl PNSAnimatablePropertyContainer for PHLivePhotoView {}
impl PNSUserInterfaceItemIdentification for PHLivePhotoView {}
impl PNSDraggingDestination for PHLivePhotoView {}
impl PNSAppearanceCustomization for PHLivePhotoView {}
impl PNSAccessibilityElement for PHLivePhotoView {}
impl PNSAccessibility for PHLivePhotoView {}
impl std::convert::TryFrom<NSView> for PHLivePhotoView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<PHLivePhotoView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHLivePhotoView").unwrap()) };
        if is_kind_of {
            Ok(PHLivePhotoView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to PHLivePhotoView")
        }
    }
}
impl INSResponder for PHLivePhotoView {}
impl PNSCoding for PHLivePhotoView {}
impl INSObject for PHLivePhotoView {}
impl PNSObject for PHLivePhotoView {}
impl IPHLivePhotoView for PHLivePhotoView {}
pub trait IPHLivePhotoView: Sized + std::ops::Deref {
    unsafe fn startPlaybackWithStyle_(&self, playbackStyle: PHLivePhotoViewPlaybackStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startPlaybackWithStyle : playbackStyle)
    }
    unsafe fn stopPlayback(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopPlayback)
    }
    unsafe fn stopPlaybackAnimated_(&self, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopPlaybackAnimated : animated)
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
    unsafe fn livePhoto(&self) -> PHLivePhoto
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, livePhoto)
    }
    unsafe fn setLivePhoto_(&self, livePhoto: PHLivePhoto)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLivePhoto : livePhoto)
    }
    unsafe fn contentMode(&self) -> PHLivePhotoViewContentMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentMode)
    }
    unsafe fn setContentMode_(&self, contentMode: PHLivePhotoViewContentMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentMode : contentMode)
    }
    unsafe fn contentsRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsRect)
    }
    unsafe fn setContentsRect_(&self, contentsRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsRect : contentsRect)
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
    unsafe fn isMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMuted)
    }
    unsafe fn setMuted_(&self, muted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMuted : muted)
    }
    unsafe fn livePhotoBadgeView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, livePhotoBadgeView)
    }
}
pub trait PPHLivePhotoViewDelegate: Sized + std::ops::Deref {
    unsafe fn livePhotoView_canBeginPlaybackWithStyle_(
        &self,
        livePhotoView: PHLivePhotoView,
        playbackStyle: PHLivePhotoViewPlaybackStyle,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, livePhotoView : livePhotoView, canBeginPlaybackWithStyle : playbackStyle)
    }
    unsafe fn livePhotoView_willBeginPlaybackWithStyle_(
        &self,
        livePhotoView: PHLivePhotoView,
        playbackStyle: PHLivePhotoViewPlaybackStyle,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, livePhotoView : livePhotoView, willBeginPlaybackWithStyle : playbackStyle)
    }
    unsafe fn livePhotoView_didEndPlaybackWithStyle_(
        &self,
        livePhotoView: PHLivePhotoView,
        playbackStyle: PHLivePhotoViewPlaybackStyle,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, livePhotoView : livePhotoView, didEndPlaybackWithStyle : playbackStyle)
    }
}
pub type PHPickerConfigurationAssetRepresentationMode = NSInteger;
pub type PHPickerConfigurationSelection = NSInteger;
pub type PHPickerMode = NSInteger;
pub type PHPickerCapabilities = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPickerFilter(pub id);
impl std::ops::Deref for PHPickerFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPickerFilter {}
impl PHPickerFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), alloc) })
    }
}
impl PNSCopying for PHPickerFilter {}
impl INSObject for PHPickerFilter {}
impl PNSObject for PHPickerFilter {}
impl std::convert::TryFrom<NSObject> for PHPickerFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPickerFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap()) };
        if is_kind_of {
            Ok(PHPickerFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPickerFilter")
        }
    }
}
impl IPHPickerFilter for PHPickerFilter {}
pub trait IPHPickerFilter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn playbackStyleFilter_(playbackStyle: PHAssetPlaybackStyle) -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), playbackStyleFilter : playbackStyle)
    }
    unsafe fn anyFilterMatchingSubfilters_(subfilters: NSArray) -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), anyFilterMatchingSubfilters : subfilters)
    }
    unsafe fn allFilterMatchingSubfilters_(subfilters: NSArray) -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), allFilterMatchingSubfilters : subfilters)
    }
    unsafe fn notFilterOfSubfilter_(subfilter: PHPickerFilter) -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), notFilterOfSubfilter : subfilter)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), new)
    }
    unsafe fn imagesFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), imagesFilter)
    }
    unsafe fn videosFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), videosFilter)
    }
    unsafe fn livePhotosFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), livePhotosFilter)
    }
    unsafe fn depthEffectPhotosFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), depthEffectPhotosFilter)
    }
    unsafe fn burstsFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), burstsFilter)
    }
    unsafe fn panoramasFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), panoramasFilter)
    }
    unsafe fn screenshotsFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), screenshotsFilter)
    }
    unsafe fn screenRecordingsFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), screenRecordingsFilter)
    }
    unsafe fn cinematicVideosFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), cinematicVideosFilter)
    }
    unsafe fn slomoVideosFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), slomoVideosFilter)
    }
    unsafe fn timelapseVideosFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), timelapseVideosFilter)
    }
    unsafe fn spatialMediaFilter() -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerFilter").unwrap(), spatialMediaFilter)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPickerUpdateConfiguration(pub id);
impl std::ops::Deref for PHPickerUpdateConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPickerUpdateConfiguration {}
impl PHPickerUpdateConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerUpdateConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for PHPickerUpdateConfiguration {}
impl INSObject for PHPickerUpdateConfiguration {}
impl PNSObject for PHPickerUpdateConfiguration {}
impl std::convert::TryFrom<NSObject> for PHPickerUpdateConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPickerUpdateConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPickerUpdateConfiguration").unwrap()) };
        if is_kind_of {
            Ok(PHPickerUpdateConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPickerUpdateConfiguration")
        }
    }
}
impl IPHPickerUpdateConfiguration for PHPickerUpdateConfiguration {}
pub trait IPHPickerUpdateConfiguration: Sized + std::ops::Deref {
    unsafe fn selectionLimit(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionLimit)
    }
    unsafe fn setSelectionLimit_(&self, selectionLimit: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionLimit : selectionLimit)
    }
    unsafe fn edgesWithoutContentMargins(&self) -> NSDirectionalRectEdge
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgesWithoutContentMargins)
    }
    unsafe fn setEdgesWithoutContentMargins_(
        &self,
        edgesWithoutContentMargins: NSDirectionalRectEdge,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgesWithoutContentMargins : edgesWithoutContentMargins)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPickerConfiguration(pub id);
impl std::ops::Deref for PHPickerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPickerConfiguration {}
impl PHPickerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for PHPickerConfiguration {}
impl INSObject for PHPickerConfiguration {}
impl PNSObject for PHPickerConfiguration {}
impl std::convert::TryFrom<NSObject> for PHPickerConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPickerConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPickerConfiguration").unwrap()) };
        if is_kind_of {
            Ok(PHPickerConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPickerConfiguration")
        }
    }
}
impl IPHPickerConfiguration for PHPickerConfiguration {}
pub trait IPHPickerConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithPhotoLibrary_(&self, photoLibrary: PHPhotoLibrary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPhotoLibrary : photoLibrary)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn preferredAssetRepresentationMode(
        &self,
    ) -> PHPickerConfigurationAssetRepresentationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredAssetRepresentationMode)
    }
    unsafe fn setPreferredAssetRepresentationMode_(
        &self,
        preferredAssetRepresentationMode: PHPickerConfigurationAssetRepresentationMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredAssetRepresentationMode : preferredAssetRepresentationMode)
    }
    unsafe fn selection(&self) -> PHPickerConfigurationSelection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selection)
    }
    unsafe fn setSelection_(&self, selection: PHPickerConfigurationSelection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelection : selection)
    }
    unsafe fn selectionLimit(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionLimit)
    }
    unsafe fn setSelectionLimit_(&self, selectionLimit: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionLimit : selectionLimit)
    }
    unsafe fn filter(&self) -> PHPickerFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filter)
    }
    unsafe fn setFilter_(&self, filter: PHPickerFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilter : filter)
    }
    unsafe fn preselectedAssetIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preselectedAssetIdentifiers)
    }
    unsafe fn setPreselectedAssetIdentifiers_(&self, preselectedAssetIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreselectedAssetIdentifiers : preselectedAssetIdentifiers)
    }
    unsafe fn mode(&self) -> PHPickerMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn setMode_(&self, mode: PHPickerMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode)
    }
    unsafe fn edgesWithoutContentMargins(&self) -> NSDirectionalRectEdge
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgesWithoutContentMargins)
    }
    unsafe fn setEdgesWithoutContentMargins_(
        &self,
        edgesWithoutContentMargins: NSDirectionalRectEdge,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgesWithoutContentMargins : edgesWithoutContentMargins)
    }
    unsafe fn disabledCapabilities(&self) -> PHPickerCapabilities
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabledCapabilities)
    }
    unsafe fn setDisabledCapabilities_(&self, disabledCapabilities: PHPickerCapabilities)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabledCapabilities : disabledCapabilities)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPickerResult(pub id);
impl std::ops::Deref for PHPickerResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPickerResult {}
impl PHPickerResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerResult").unwrap(), alloc) })
    }
}
impl INSObject for PHPickerResult {}
impl PNSObject for PHPickerResult {}
impl std::convert::TryFrom<NSObject> for PHPickerResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHPickerResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPickerResult").unwrap()) };
        if is_kind_of {
            Ok(PHPickerResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHPickerResult")
        }
    }
}
impl IPHPickerResult for PHPickerResult {}
pub trait IPHPickerResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn itemProvider(&self) -> NSItemProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemProvider)
    }
    unsafe fn assetIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerResult").unwrap(), new)
    }
}
pub trait PPHPickerViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn picker_didFinishPicking_(&self, picker: PHPickerViewController, results: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, picker : picker, didFinishPicking : results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHPickerViewController(pub id);
impl std::ops::Deref for PHPickerViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHPickerViewController {}
impl PHPickerViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerViewController").unwrap(), alloc) })
    }
}
impl INSViewController for PHPickerViewController {}
impl PNSEditor for PHPickerViewController {}
impl PNSSeguePerforming for PHPickerViewController {}
impl PNSUserInterfaceItemIdentification for PHPickerViewController {}
impl std::convert::TryFrom<NSViewController> for PHPickerViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<PHPickerViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHPickerViewController").unwrap()) };
        if is_kind_of {
            Ok(PHPickerViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to PHPickerViewController")
        }
    }
}
impl INSResponder for PHPickerViewController {}
impl PNSCoding for PHPickerViewController {}
impl INSObject for PHPickerViewController {}
impl PNSObject for PHPickerViewController {}
impl IPHPickerViewController for PHPickerViewController {}
pub trait IPHPickerViewController: Sized + std::ops::Deref {
    unsafe fn initWithConfiguration_(&self, configuration: PHPickerConfiguration) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
    unsafe fn updatePickerUsingConfiguration_(&self, configuration: PHPickerUpdateConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updatePickerUsingConfiguration : configuration)
    }
    unsafe fn deselectAssetsWithIdentifiers_(&self, identifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deselectAssetsWithIdentifiers : identifiers)
    }
    unsafe fn moveAssetWithIdentifier_afterAssetWithIdentifier_(
        &self,
        identifier: NSString,
        afterIdentifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveAssetWithIdentifier : identifier, afterAssetWithIdentifier : afterIdentifier)
    }
    unsafe fn scrollToInitialPosition(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrollToInitialPosition)
    }
    unsafe fn zoomIn(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoomIn)
    }
    unsafe fn zoomOut(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoomOut)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithNibName_bundle_(
        &self,
        nibNameOrNil: NSString,
        nibBundleOrNil: NSBundle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNibName : nibNameOrNil, bundle : nibBundleOrNil)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn configuration(&self) -> PHPickerConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHPickerViewController").unwrap(), new)
    }
}
pub trait PPHContentEditingController: Sized + std::ops::Deref {
    unsafe fn canHandleAdjustmentData_(&self, adjustmentData: PHAdjustmentData) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canHandleAdjustmentData : adjustmentData)
    }
    unsafe fn startContentEditingWithInput_placeholderImage_(
        &self,
        contentEditingInput: PHContentEditingInput,
        placeholderImage: NSImage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startContentEditingWithInput : contentEditingInput, placeholderImage : placeholderImage)
    }
    unsafe fn finishContentEditingWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishContentEditingWithCompletionHandler : completionHandler)
    }
    unsafe fn cancelContentEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelContentEditing)
    }
    unsafe fn shouldShowCancelConfirmation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldShowCancelConfirmation)
    }
}
pub type PHProjectType = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectExtensionContext(pub id);
impl std::ops::Deref for PHProjectExtensionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectExtensionContext {}
impl PHProjectExtensionContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectExtensionContext").unwrap(), alloc) })
    }
}
impl INSExtensionContext for PHProjectExtensionContext {}
impl std::convert::TryFrom<NSExtensionContext> for PHProjectExtensionContext {
    type Error = &'static str;
    fn try_from(parent: NSExtensionContext) -> Result<PHProjectExtensionContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectExtensionContext").unwrap()) };
        if is_kind_of {
            Ok(PHProjectExtensionContext(parent.0))
        } else {
            Err("This NSExtensionContext cannot be downcasted to PHProjectExtensionContext")
        }
    }
}
impl INSObject for PHProjectExtensionContext {}
impl PNSObject for PHProjectExtensionContext {}
impl IPHProjectExtensionContext for PHProjectExtensionContext {}
pub trait IPHProjectExtensionContext: Sized + std::ops::Deref {
    unsafe fn showEditorForAsset_(&self, asset: PHAsset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showEditorForAsset : asset)
    }
    unsafe fn updatedProjectInfoFromProjectInfo_completion_(
        &self,
        existingProjectInfo: PHProjectInfo,
        completion: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updatedProjectInfoFromProjectInfo : existingProjectInfo, completion : completion)
    }
    unsafe fn photoLibrary(&self) -> PHPhotoLibrary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, photoLibrary)
    }
    unsafe fn project(&self) -> PHProject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, project)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectTypeDescription(pub id);
impl std::ops::Deref for PHProjectTypeDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectTypeDescription {}
impl PHProjectTypeDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectTypeDescription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectTypeDescription {}
impl INSObject for PHProjectTypeDescription {}
impl PNSObject for PHProjectTypeDescription {}
impl std::convert::TryFrom<NSObject> for PHProjectTypeDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHProjectTypeDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectTypeDescription").unwrap()) };
        if is_kind_of {
            Ok(PHProjectTypeDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHProjectTypeDescription")
        }
    }
}
impl IPHProjectTypeDescription for PHProjectTypeDescription {}
pub trait IPHProjectTypeDescription: Sized + std::ops::Deref {
    unsafe fn initWithProjectType_title_description_image_subtypeDescriptions_(
        &self,
        projectType: NSString,
        localizedTitle: NSString,
        localizedDescription: NSString,
        image: NSImage,
        subtypeDescriptions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProjectType : projectType, title : localizedTitle, description : localizedDescription, image : image, subtypeDescriptions : subtypeDescriptions)
    }
    unsafe fn initWithProjectType_title_attributedDescription_image_subtypeDescriptions_(
        &self,
        projectType: NSString,
        localizedTitle: NSString,
        localizedAttributedDescription: NSAttributedString,
        image: NSImage,
        subtypeDescriptions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProjectType : projectType, title : localizedTitle, attributedDescription : localizedAttributedDescription, image : image, subtypeDescriptions : subtypeDescriptions)
    }
    unsafe fn initWithProjectType_title_description_image_(
        &self,
        projectType: NSString,
        localizedTitle: NSString,
        localizedDescription: NSString,
        image: NSImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProjectType : projectType, title : localizedTitle, description : localizedDescription, image : image)
    }
    unsafe fn initWithProjectType_title_description_image_canProvideSubtypes_(
        &self,
        projectType: NSString,
        localizedTitle: NSString,
        localizedDescription: NSString,
        image: NSImage,
        canProvideSubtypes: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProjectType : projectType, title : localizedTitle, description : localizedDescription, image : image, canProvideSubtypes : canProvideSubtypes)
    }
    unsafe fn initWithProjectType_title_attributedDescription_image_canProvideSubtypes_(
        &self,
        projectType: NSString,
        localizedTitle: NSString,
        localizedAttributedDescription: NSAttributedString,
        image: NSImage,
        canProvideSubtypes: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProjectType : projectType, title : localizedTitle, attributedDescription : localizedAttributedDescription, image : image, canProvideSubtypes : canProvideSubtypes)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn projectType(&self) -> PHProjectType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectType)
    }
    unsafe fn localizedTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedTitle)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn localizedAttributedDescription(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedAttributedDescription)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn subtypeDescriptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtypeDescriptions)
    }
    unsafe fn canProvideSubtypes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canProvideSubtypes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectTypeDescription").unwrap(), new)
    }
}
pub type PHProjectCreationSource = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectInfo(pub id);
impl std::ops::Deref for PHProjectInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectInfo {}
impl PHProjectInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectInfo").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectInfo {}
impl INSObject for PHProjectInfo {}
impl PNSObject for PHProjectInfo {}
impl std::convert::TryFrom<NSObject> for PHProjectInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHProjectInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectInfo").unwrap()) };
        if is_kind_of {
            Ok(PHProjectInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHProjectInfo")
        }
    }
}
impl IPHProjectInfo for PHProjectInfo {}
pub trait IPHProjectInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn creationSource(&self) -> PHProjectCreationSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationSource)
    }
    unsafe fn projectType(&self) -> PHProjectType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectType)
    }
    unsafe fn sections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sections)
    }
    unsafe fn brandingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, brandingEnabled)
    }
    unsafe fn pageNumbersEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageNumbersEnabled)
    }
    unsafe fn productIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productIdentifier)
    }
    unsafe fn themeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, themeIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectInfo").unwrap(), new)
    }
}
pub type PHProjectSectionType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectSection(pub id);
impl std::ops::Deref for PHProjectSection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectSection {}
impl PHProjectSection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectSection").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectSection {}
impl INSObject for PHProjectSection {}
impl PNSObject for PHProjectSection {}
impl std::convert::TryFrom<NSObject> for PHProjectSection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHProjectSection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectSection").unwrap()) };
        if is_kind_of {
            Ok(PHProjectSection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHProjectSection")
        }
    }
}
impl IPHProjectSection for PHProjectSection {}
pub trait IPHProjectSection: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sectionContents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sectionContents)
    }
    unsafe fn sectionType(&self) -> PHProjectSectionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sectionType)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectSection").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectSectionContent(pub id);
impl std::ops::Deref for PHProjectSectionContent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectSectionContent {}
impl PHProjectSectionContent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectSectionContent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectSectionContent {}
impl INSObject for PHProjectSectionContent {}
impl PNSObject for PHProjectSectionContent {}
impl std::convert::TryFrom<NSObject> for PHProjectSectionContent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHProjectSectionContent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectSectionContent").unwrap()) };
        if is_kind_of {
            Ok(PHProjectSectionContent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHProjectSectionContent")
        }
    }
}
impl IPHProjectSectionContent for PHProjectSectionContent {}
pub trait IPHProjectSectionContent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn elements(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn numberOfColumns(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfColumns)
    }
    unsafe fn aspectRatio(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
    unsafe fn cloudAssetIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloudAssetIdentifiers)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectSectionContent").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectElement(pub id);
impl std::ops::Deref for PHProjectElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectElement {}
impl PHProjectElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectElement").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectElement {}
impl INSObject for PHProjectElement {}
impl PNSObject for PHProjectElement {}
impl std::convert::TryFrom<NSObject> for PHProjectElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHProjectElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectElement").unwrap()) };
        if is_kind_of {
            Ok(PHProjectElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHProjectElement")
        }
    }
}
impl IPHProjectElement for PHProjectElement {}
pub trait IPHProjectElement: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn weight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weight)
    }
    unsafe fn placement(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placement)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectElement").unwrap(), new)
    }
}
pub type PHProjectRegionOfInterestIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectRegionOfInterest(pub id);
impl std::ops::Deref for PHProjectRegionOfInterest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectRegionOfInterest {}
impl PHProjectRegionOfInterest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectRegionOfInterest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectRegionOfInterest {}
impl INSObject for PHProjectRegionOfInterest {}
impl PNSObject for PHProjectRegionOfInterest {}
impl std::convert::TryFrom<NSObject> for PHProjectRegionOfInterest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHProjectRegionOfInterest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectRegionOfInterest").unwrap()) };
        if is_kind_of {
            Ok(PHProjectRegionOfInterest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHProjectRegionOfInterest")
        }
    }
}
impl IPHProjectRegionOfInterest for PHProjectRegionOfInterest {}
pub trait IPHProjectRegionOfInterest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn rect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rect)
    }
    unsafe fn weight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weight)
    }
    unsafe fn quality(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quality)
    }
    unsafe fn identifier(&self) -> PHProjectRegionOfInterestIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectRegionOfInterest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectAssetElement(pub id);
impl std::ops::Deref for PHProjectAssetElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectAssetElement {}
impl PHProjectAssetElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectAssetElement").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectAssetElement {}
impl IPHProjectElement for PHProjectAssetElement {}
impl From<PHProjectAssetElement> for PHProjectElement {
    fn from(child: PHProjectAssetElement) -> PHProjectElement {
        PHProjectElement(child.0)
    }
}
impl std::convert::TryFrom<PHProjectElement> for PHProjectAssetElement {
    type Error = &'static str;
    fn try_from(parent: PHProjectElement) -> Result<PHProjectAssetElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectAssetElement").unwrap()) };
        if is_kind_of {
            Ok(PHProjectAssetElement(parent.0))
        } else {
            Err("This PHProjectElement cannot be downcasted to PHProjectAssetElement")
        }
    }
}
impl INSObject for PHProjectAssetElement {}
impl PNSObject for PHProjectAssetElement {}
impl IPHProjectAssetElement for PHProjectAssetElement {}
pub trait IPHProjectAssetElement: Sized + std::ops::Deref {
    unsafe fn cloudAssetIdentifier(&self) -> PHCloudIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloudAssetIdentifier)
    }
    unsafe fn annotation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, annotation)
    }
    unsafe fn cropRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cropRect)
    }
    unsafe fn regionsOfInterest(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionsOfInterest)
    }
    unsafe fn horizontallyFlipped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontallyFlipped)
    }
    unsafe fn verticallyFlipped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticallyFlipped)
    }
}
pub type PHProjectTextElementType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectTextElement(pub id);
impl std::ops::Deref for PHProjectTextElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectTextElement {}
impl PHProjectTextElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectTextElement").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectTextElement {}
impl IPHProjectElement for PHProjectTextElement {}
impl std::convert::TryFrom<PHProjectElement> for PHProjectTextElement {
    type Error = &'static str;
    fn try_from(parent: PHProjectElement) -> Result<PHProjectTextElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectTextElement").unwrap()) };
        if is_kind_of {
            Ok(PHProjectTextElement(parent.0))
        } else {
            Err("This PHProjectElement cannot be downcasted to PHProjectTextElement")
        }
    }
}
impl INSObject for PHProjectTextElement {}
impl PNSObject for PHProjectTextElement {}
impl IPHProjectTextElement for PHProjectTextElement {}
pub trait IPHProjectTextElement: Sized + std::ops::Deref {
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn attributedText(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedText)
    }
    unsafe fn textElementType(&self) -> PHProjectTextElementType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textElementType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectJournalEntryElement(pub id);
impl std::ops::Deref for PHProjectJournalEntryElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectJournalEntryElement {}
impl PHProjectJournalEntryElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectJournalEntryElement").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectJournalEntryElement {}
impl IPHProjectElement for PHProjectJournalEntryElement {}
impl std::convert::TryFrom<PHProjectElement> for PHProjectJournalEntryElement {
    type Error = &'static str;
    fn try_from(parent: PHProjectElement) -> Result<PHProjectJournalEntryElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectJournalEntryElement").unwrap()) };
        if is_kind_of {
            Ok(PHProjectJournalEntryElement(parent.0))
        } else {
            Err("This PHProjectElement cannot be downcasted to PHProjectJournalEntryElement")
        }
    }
}
impl INSObject for PHProjectJournalEntryElement {}
impl PNSObject for PHProjectJournalEntryElement {}
impl IPHProjectJournalEntryElement for PHProjectJournalEntryElement {}
pub trait IPHProjectJournalEntryElement: Sized + std::ops::Deref {
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn assetElement(&self) -> PHProjectAssetElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetElement)
    }
    unsafe fn textElement(&self) -> PHProjectTextElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textElement)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHProjectMapElement(pub id);
impl std::ops::Deref for PHProjectMapElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHProjectMapElement {}
impl PHProjectMapElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHProjectMapElement").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for PHProjectMapElement {}
impl IPHProjectElement for PHProjectMapElement {}
impl std::convert::TryFrom<PHProjectElement> for PHProjectMapElement {
    type Error = &'static str;
    fn try_from(parent: PHProjectElement) -> Result<PHProjectMapElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHProjectMapElement").unwrap()) };
        if is_kind_of {
            Ok(PHProjectMapElement(parent.0))
        } else {
            Err("This PHProjectElement cannot be downcasted to PHProjectMapElement")
        }
    }
}
impl INSObject for PHProjectMapElement {}
impl PNSObject for PHProjectMapElement {}
impl IPHProjectMapElement for PHProjectMapElement {}
pub trait IPHProjectMapElement: Sized + std::ops::Deref {
    unsafe fn mapType(&self) -> MKMapType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapType)
    }
    unsafe fn centerCoordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerCoordinate)
    }
    unsafe fn heading(&self) -> CLLocationDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heading)
    }
    unsafe fn pitch(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn altitude(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altitude)
    }
    unsafe fn annotations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, annotations)
    }
}

unsafe impl objc2::encode::RefEncode for PHLivePhotoView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHLivePhotoView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPickerFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPickerFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPickerUpdateConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPickerUpdateConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPickerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPickerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPickerResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPickerResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHPickerViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHPickerViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectExtensionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectExtensionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectTypeDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectTypeDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectSection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectSection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectSectionContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectSectionContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectRegionOfInterest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectRegionOfInterest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectAssetElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectAssetElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectTextElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectTextElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectJournalEntryElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectJournalEntryElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHProjectMapElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHProjectMapElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
