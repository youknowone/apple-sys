#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::AVRouting::*;
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
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVPlaybackSpeed(pub id);
impl std::ops::Deref for AVPlaybackSpeed {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVPlaybackSpeed {}
impl AVPlaybackSpeed {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVPlaybackSpeed").unwrap(), alloc) })
    }
}
impl INSObject for AVPlaybackSpeed {}
impl PNSObject for AVPlaybackSpeed {}
impl std::convert::TryFrom<NSObject> for AVPlaybackSpeed {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVPlaybackSpeed, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVPlaybackSpeed").unwrap()) };
        if is_kind_of {
            Ok(AVPlaybackSpeed(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVPlaybackSpeed")
        }
    }
}
impl IAVPlaybackSpeed for AVPlaybackSpeed {}
pub trait IAVPlaybackSpeed: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRate_localizedName_(&self, rate: f32, localizedName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRate : rate, localizedName : localizedName)
    }
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn localizedNumericName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedNumericName)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVPlaybackSpeed").unwrap(), new)
    }
    unsafe fn systemDefaultSpeeds() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVPlaybackSpeed").unwrap(), systemDefaultSpeeds)
    }
}
pub type AVVideoFrameAnalysisType = NSUInteger;
pub type AVDisplayDynamicRange = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVCaptureView(pub id);
impl std::ops::Deref for AVCaptureView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVCaptureView {}
impl AVCaptureView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVCaptureView").unwrap(), alloc) })
    }
}
impl INSView for AVCaptureView {}
impl PNSAnimatablePropertyContainer for AVCaptureView {}
impl PNSUserInterfaceItemIdentification for AVCaptureView {}
impl PNSDraggingDestination for AVCaptureView {}
impl PNSAppearanceCustomization for AVCaptureView {}
impl PNSAccessibilityElement for AVCaptureView {}
impl PNSAccessibility for AVCaptureView {}
impl std::convert::TryFrom<NSView> for AVCaptureView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<AVCaptureView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVCaptureView").unwrap()) };
        if is_kind_of {
            Ok(AVCaptureView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to AVCaptureView")
        }
    }
}
impl INSResponder for AVCaptureView {}
impl PNSCoding for AVCaptureView {}
impl INSObject for AVCaptureView {}
impl PNSObject for AVCaptureView {}
impl IAVCaptureView for AVCaptureView {}
pub trait IAVCaptureView: Sized + std::ops::Deref {
    unsafe fn setSession_showVideoPreview_showAudioPreview_(
        &self,
        session: AVCaptureSession,
        showVideoPreview: BOOL,
        showAudioPreview: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSession : session, showVideoPreview : showVideoPreview, showAudioPreview : showAudioPreview)
    }
    unsafe fn session(&self) -> AVCaptureSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, session)
    }
    unsafe fn fileOutput(&self) -> AVCaptureFileOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileOutput)
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
    unsafe fn controlsStyle(&self) -> AVCaptureViewControlsStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlsStyle)
    }
    unsafe fn setControlsStyle_(&self, controlsStyle: AVCaptureViewControlsStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlsStyle : controlsStyle)
    }
    unsafe fn videoGravity(&self) -> AVLayerVideoGravity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoGravity)
    }
    unsafe fn setVideoGravity_(&self, videoGravity: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoGravity : videoGravity)
    }
}
pub type AVCaptureViewControlsStyle = NSInteger;
pub trait PAVCaptureViewDelegate: Sized + std::ops::Deref {
    unsafe fn captureView_startRecordingToFileOutput_(
        &self,
        captureView: AVCaptureView,
        fileOutput: AVCaptureFileOutput,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, captureView : captureView, startRecordingToFileOutput : fileOutput)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVPictureInPictureController(pub id);
impl std::ops::Deref for AVPictureInPictureController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVPictureInPictureController {}
impl AVPictureInPictureController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVPictureInPictureController").unwrap(), alloc) })
    }
}
impl INSObject for AVPictureInPictureController {}
impl PNSObject for AVPictureInPictureController {}
impl std::convert::TryFrom<NSObject> for AVPictureInPictureController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVPictureInPictureController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVPictureInPictureController").unwrap()) };
        if is_kind_of {
            Ok(AVPictureInPictureController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVPictureInPictureController")
        }
    }
}
impl IAVPictureInPictureController for AVPictureInPictureController {}
pub trait IAVPictureInPictureController: Sized + std::ops::Deref {
    unsafe fn initWithContentSource_(
        &self,
        contentSource: AVPictureInPictureControllerContentSource,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentSource : contentSource)
    }
    unsafe fn initWithPlayerLayer_(&self, playerLayer: AVPlayerLayer) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlayerLayer : playerLayer)
    }
    unsafe fn startPictureInPicture(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startPictureInPicture)
    }
    unsafe fn stopPictureInPicture(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopPictureInPicture)
    }
    unsafe fn contentSource(&self) -> AVPictureInPictureControllerContentSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentSource)
    }
    unsafe fn setContentSource_(&self, contentSource: AVPictureInPictureControllerContentSource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentSource : contentSource)
    }
    unsafe fn playerLayer(&self) -> AVPlayerLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerLayer)
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
    unsafe fn isPictureInPicturePossible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPictureInPicturePossible)
    }
    unsafe fn isPictureInPictureActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPictureInPictureActive)
    }
    unsafe fn isPictureInPictureSuspended(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPictureInPictureSuspended)
    }
    unsafe fn canStopPictureInPicture(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canStopPictureInPicture)
    }
    unsafe fn requiresLinearPlayback(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresLinearPlayback)
    }
    unsafe fn setRequiresLinearPlayback_(&self, requiresLinearPlayback: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresLinearPlayback : requiresLinearPlayback)
    }
    unsafe fn canStartPictureInPictureAutomaticallyFromInline(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canStartPictureInPictureAutomaticallyFromInline)
    }
    unsafe fn setCanStartPictureInPictureAutomaticallyFromInline_(
        &self,
        canStartPictureInPictureAutomaticallyFromInline: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanStartPictureInPictureAutomaticallyFromInline : canStartPictureInPictureAutomaticallyFromInline)
    }
    unsafe fn isPictureInPictureSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVPictureInPictureController").unwrap(), isPictureInPictureSupported)
    }
    unsafe fn pictureInPictureButtonStartImage() -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVPictureInPictureController").unwrap(), pictureInPictureButtonStartImage)
    }
    unsafe fn pictureInPictureButtonStopImage() -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVPictureInPictureController").unwrap(), pictureInPictureButtonStopImage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVPictureInPictureControllerContentSource(pub id);
impl std::ops::Deref for AVPictureInPictureControllerContentSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVPictureInPictureControllerContentSource {}
impl AVPictureInPictureControllerContentSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVPictureInPictureControllerContentSource").unwrap(), alloc) })
    }
}
impl INSObject for AVPictureInPictureControllerContentSource {}
impl PNSObject for AVPictureInPictureControllerContentSource {}
impl std::convert::TryFrom<NSObject> for AVPictureInPictureControllerContentSource {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<AVPictureInPictureControllerContentSource, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVPictureInPictureControllerContentSource").unwrap())
        };
        if is_kind_of {
            Ok(AVPictureInPictureControllerContentSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVPictureInPictureControllerContentSource")
        }
    }
}
impl IAVPictureInPictureControllerContentSource for AVPictureInPictureControllerContentSource {}
pub trait IAVPictureInPictureControllerContentSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPlayerLayer_(&self, playerLayer: AVPlayerLayer) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlayerLayer : playerLayer)
    }
    unsafe fn playerLayer(&self) -> AVPlayerLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerLayer)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVPictureInPictureControllerContentSource").unwrap(), new)
    }
}
pub trait PAVPictureInPictureControllerDelegate: Sized + std::ops::Deref {
    unsafe fn pictureInPictureControllerWillStartPictureInPicture_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureControllerWillStartPictureInPicture : pictureInPictureController)
    }
    unsafe fn pictureInPictureControllerDidStartPictureInPicture_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureControllerDidStartPictureInPicture : pictureInPictureController)
    }
    unsafe fn pictureInPictureController_failedToStartPictureInPictureWithError_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureController : pictureInPictureController, failedToStartPictureInPictureWithError : error)
    }
    unsafe fn pictureInPictureControllerWillStopPictureInPicture_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureControllerWillStopPictureInPicture : pictureInPictureController)
    }
    unsafe fn pictureInPictureControllerDidStopPictureInPicture_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureControllerDidStopPictureInPicture : pictureInPictureController)
    }
    unsafe fn pictureInPictureController_restoreUserInterfaceForPictureInPictureStopWithCompletionHandler_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureController : pictureInPictureController, restoreUserInterfaceForPictureInPictureStopWithCompletionHandler : completionHandler)
    }
}
impl AVPictureInPictureController_ for AVPictureInPictureController {}
pub trait AVPictureInPictureController_: Sized + std::ops::Deref {
    unsafe fn invalidatePlaybackState(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidatePlaybackState)
    }
}
pub trait PAVPictureInPictureSampleBufferPlaybackDelegate: Sized + std::ops::Deref {
    unsafe fn pictureInPictureController_setPlaying_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
        playing: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureController : pictureInPictureController, setPlaying : playing)
    }
    unsafe fn pictureInPictureControllerTimeRangeForPlayback_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
    ) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureControllerTimeRangeForPlayback : pictureInPictureController)
    }
    unsafe fn pictureInPictureControllerIsPlaybackPaused_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureControllerIsPlaybackPaused : pictureInPictureController)
    }
    unsafe fn pictureInPictureController_didTransitionToRenderSize_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
        newRenderSize: CMVideoDimensions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureController : pictureInPictureController, didTransitionToRenderSize : newRenderSize)
    }
    unsafe fn pictureInPictureController_skipByInterval_completionHandler_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
        skipInterval: CMTime,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureController : pictureInPictureController, skipByInterval : skipInterval, completionHandler : completionHandler)
    }
    unsafe fn pictureInPictureControllerShouldProhibitBackgroundAudioPlayback_(
        &self,
        pictureInPictureController: AVPictureInPictureController,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pictureInPictureControllerShouldProhibitBackgroundAudioPlayback : pictureInPictureController)
    }
}
impl AVPictureInPictureControllerContentSource_AVSampleBufferDisplayLayerSupport
    for AVPictureInPictureControllerContentSource
{
}
pub trait AVPictureInPictureControllerContentSource_AVSampleBufferDisplayLayerSupport:
    Sized + std::ops::Deref
{
    unsafe fn initWithSampleBufferDisplayLayer_playbackDelegate_(
        &self,
        sampleBufferDisplayLayer: AVSampleBufferDisplayLayer,
        playbackDelegate: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleBufferDisplayLayer : sampleBufferDisplayLayer, playbackDelegate : playbackDelegate)
    }
    unsafe fn sampleBufferDisplayLayer(&self) -> AVSampleBufferDisplayLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBufferDisplayLayer)
    }
    unsafe fn sampleBufferPlaybackDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBufferPlaybackDelegate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVPlayerView(pub id);
impl std::ops::Deref for AVPlayerView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVPlayerView {}
impl AVPlayerView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVPlayerView").unwrap(), alloc) })
    }
}
impl INSView for AVPlayerView {}
impl PNSAnimatablePropertyContainer for AVPlayerView {}
impl PNSUserInterfaceItemIdentification for AVPlayerView {}
impl PNSDraggingDestination for AVPlayerView {}
impl PNSAppearanceCustomization for AVPlayerView {}
impl PNSAccessibilityElement for AVPlayerView {}
impl PNSAccessibility for AVPlayerView {}
impl std::convert::TryFrom<NSView> for AVPlayerView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<AVPlayerView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVPlayerView").unwrap()) };
        if is_kind_of {
            Ok(AVPlayerView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to AVPlayerView")
        }
    }
}
impl INSResponder for AVPlayerView {}
impl PNSCoding for AVPlayerView {}
impl INSObject for AVPlayerView {}
impl PNSObject for AVPlayerView {}
impl IAVPlayerView for AVPlayerView {}
pub trait IAVPlayerView: Sized + std::ops::Deref {
    unsafe fn selectSpeed_(&self, speed: AVPlaybackSpeed)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectSpeed : speed)
    }
    unsafe fn setMagnification_centeredAtPoint_(&self, magnification: CGFloat, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagnification : magnification, centeredAtPoint : point)
    }
    unsafe fn player(&self) -> AVPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, player)
    }
    unsafe fn setPlayer_(&self, player: AVPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayer : player)
    }
    unsafe fn controlsStyle(&self) -> AVPlayerViewControlsStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlsStyle)
    }
    unsafe fn setControlsStyle_(&self, controlsStyle: AVPlayerViewControlsStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlsStyle : controlsStyle)
    }
    unsafe fn videoGravity(&self) -> AVLayerVideoGravity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoGravity)
    }
    unsafe fn setVideoGravity_(&self, videoGravity: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoGravity : videoGravity)
    }
    unsafe fn isReadyForDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadyForDisplay)
    }
    unsafe fn videoBounds(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoBounds)
    }
    unsafe fn contentOverlayView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentOverlayView)
    }
    unsafe fn updatesNowPlayingInfoCenter(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updatesNowPlayingInfoCenter)
    }
    unsafe fn setUpdatesNowPlayingInfoCenter_(&self, updatesNowPlayingInfoCenter: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpdatesNowPlayingInfoCenter : updatesNowPlayingInfoCenter)
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
    unsafe fn speeds(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speeds)
    }
    unsafe fn setSpeeds_(&self, speeds: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeeds : speeds)
    }
    unsafe fn selectedSpeed(&self) -> AVPlaybackSpeed
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedSpeed)
    }
    unsafe fn allowsVideoFrameAnalysis(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsVideoFrameAnalysis)
    }
    unsafe fn setAllowsVideoFrameAnalysis_(&self, allowsVideoFrameAnalysis: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsVideoFrameAnalysis : allowsVideoFrameAnalysis)
    }
    unsafe fn videoFrameAnalysisTypes(&self) -> AVVideoFrameAnalysisType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoFrameAnalysisTypes)
    }
    unsafe fn setVideoFrameAnalysisTypes_(&self, videoFrameAnalysisTypes: AVVideoFrameAnalysisType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoFrameAnalysisTypes : videoFrameAnalysisTypes)
    }
    unsafe fn allowsMagnification(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsMagnification)
    }
    unsafe fn setAllowsMagnification_(&self, allowsMagnification: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsMagnification : allowsMagnification)
    }
    unsafe fn magnification(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magnification)
    }
    unsafe fn setMagnification_(&self, magnification: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagnification : magnification)
    }
    unsafe fn preferredDisplayDynamicRange(&self) -> AVDisplayDynamicRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDisplayDynamicRange)
    }
    unsafe fn setPreferredDisplayDynamicRange_(
        &self,
        preferredDisplayDynamicRange: AVDisplayDynamicRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredDisplayDynamicRange : preferredDisplayDynamicRange)
    }
}
pub type AVPlayerViewControlsStyle = NSInteger;
impl AVPlayerView_AVPlayerViewCustomization for AVPlayerView {}
pub trait AVPlayerView_AVPlayerViewCustomization: Sized + std::ops::Deref {
    unsafe fn showsFrameSteppingButtons(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsFrameSteppingButtons)
    }
    unsafe fn setShowsFrameSteppingButtons_(&self, showsFrameSteppingButtons: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsFrameSteppingButtons : showsFrameSteppingButtons)
    }
    unsafe fn showsSharingServiceButton(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsSharingServiceButton)
    }
    unsafe fn setShowsSharingServiceButton_(&self, showsSharingServiceButton: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsSharingServiceButton : showsSharingServiceButton)
    }
    unsafe fn actionPopUpButtonMenu(&self) -> NSMenu
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionPopUpButtonMenu)
    }
    unsafe fn setActionPopUpButtonMenu_(&self, actionPopUpButtonMenu: NSMenu)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActionPopUpButtonMenu : actionPopUpButtonMenu)
    }
    unsafe fn showsFullScreenToggleButton(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsFullScreenToggleButton)
    }
    unsafe fn setShowsFullScreenToggleButton_(&self, showsFullScreenToggleButton: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsFullScreenToggleButton : showsFullScreenToggleButton)
    }
    unsafe fn showsTimecodes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsTimecodes)
    }
    unsafe fn setShowsTimecodes_(&self, showsTimecodes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsTimecodes : showsTimecodes)
    }
}
impl AVPlayerView_AVPlayerViewTrimming for AVPlayerView {}
pub trait AVPlayerView_AVPlayerViewTrimming: Sized + std::ops::Deref {
    unsafe fn beginTrimmingWithCompletionHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginTrimmingWithCompletionHandler : handler)
    }
    unsafe fn canBeginTrimming(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canBeginTrimming)
    }
}
pub type AVPlayerViewTrimResult = NSInteger;
impl AVPlayerView_AVPlayerViewChapterIndicator for AVPlayerView {}
pub trait AVPlayerView_AVPlayerViewChapterIndicator: Sized + std::ops::Deref {
    unsafe fn flashChapterNumber_chapterTitle_(
        &self,
        chapterNumber: NSUInteger,
        chapterTitle: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, flashChapterNumber : chapterNumber, chapterTitle : chapterTitle)
    }
}
impl AVPlayerView_AVPlayerViewPictureInPictureSupport for AVPlayerView {}
pub trait AVPlayerView_AVPlayerViewPictureInPictureSupport: Sized + std::ops::Deref {
    unsafe fn allowsPictureInPicturePlayback(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsPictureInPicturePlayback)
    }
    unsafe fn setAllowsPictureInPicturePlayback_(&self, allowsPictureInPicturePlayback: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsPictureInPicturePlayback : allowsPictureInPicturePlayback)
    }
    unsafe fn pictureInPictureDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pictureInPictureDelegate)
    }
    unsafe fn setPictureInPictureDelegate_(&self, pictureInPictureDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPictureInPictureDelegate : pictureInPictureDelegate)
    }
}
pub trait PAVPlayerViewDelegate: Sized + std::ops::Deref {
    unsafe fn playerViewWillEnterFullScreen_(&self, playerView: AVPlayerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewWillEnterFullScreen : playerView)
    }
    unsafe fn playerViewDidEnterFullScreen_(&self, playerView: AVPlayerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewDidEnterFullScreen : playerView)
    }
    unsafe fn playerViewWillExitFullScreen_(&self, playerView: AVPlayerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewWillExitFullScreen : playerView)
    }
    unsafe fn playerViewDidExitFullScreen_(&self, playerView: AVPlayerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewDidExitFullScreen : playerView)
    }
    unsafe fn playerView_restoreUserInterfaceForFullScreenExitWithCompletionHandler_(
        &self,
        playerView: AVPlayerView,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerView : playerView, restoreUserInterfaceForFullScreenExitWithCompletionHandler : completionHandler)
    }
}
pub trait PAVPlayerViewPictureInPictureDelegate: Sized + std::ops::Deref {
    unsafe fn playerViewWillStartPictureInPicture_(&self, playerView: AVPlayerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewWillStartPictureInPicture : playerView)
    }
    unsafe fn playerViewDidStartPictureInPicture_(&self, playerView: AVPlayerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewDidStartPictureInPicture : playerView)
    }
    unsafe fn playerView_failedToStartPictureInPictureWithError_(
        &self,
        playerView: AVPlayerView,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerView : playerView, failedToStartPictureInPictureWithError : error)
    }
    unsafe fn playerViewWillStopPictureInPicture_(&self, playerView: AVPlayerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewWillStopPictureInPicture : playerView)
    }
    unsafe fn playerViewDidStopPictureInPicture_(&self, playerView: AVPlayerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewDidStopPictureInPicture : playerView)
    }
    unsafe fn playerView_restoreUserInterfaceForPictureInPictureStopWithCompletionHandler_(
        &self,
        playerView: AVPlayerView,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerView : playerView, restoreUserInterfaceForPictureInPictureStopWithCompletionHandler : completionHandler)
    }
    unsafe fn playerViewShouldAutomaticallyDismissAtPictureInPictureStart_(
        &self,
        playerView: AVPlayerView,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerViewShouldAutomaticallyDismissAtPictureInPictureStart : playerView)
    }
}
pub type AVRoutePickerViewButtonState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVRoutePickerView(pub id);
impl std::ops::Deref for AVRoutePickerView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVRoutePickerView {}
impl AVRoutePickerView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVRoutePickerView").unwrap(), alloc) })
    }
}
impl INSView for AVRoutePickerView {}
impl PNSAnimatablePropertyContainer for AVRoutePickerView {}
impl PNSUserInterfaceItemIdentification for AVRoutePickerView {}
impl PNSDraggingDestination for AVRoutePickerView {}
impl PNSAppearanceCustomization for AVRoutePickerView {}
impl PNSAccessibilityElement for AVRoutePickerView {}
impl PNSAccessibility for AVRoutePickerView {}
impl std::convert::TryFrom<NSView> for AVRoutePickerView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<AVRoutePickerView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVRoutePickerView").unwrap()) };
        if is_kind_of {
            Ok(AVRoutePickerView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to AVRoutePickerView")
        }
    }
}
impl INSResponder for AVRoutePickerView {}
impl PNSCoding for AVRoutePickerView {}
impl INSObject for AVRoutePickerView {}
impl PNSObject for AVRoutePickerView {}
impl IAVRoutePickerView for AVRoutePickerView {}
pub trait IAVRoutePickerView: Sized + std::ops::Deref {
    unsafe fn routePickerButtonColorForState_(&self, state: AVRoutePickerViewButtonState) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, routePickerButtonColorForState : state)
    }
    unsafe fn setRoutePickerButtonColor_forState_(
        &self,
        color: NSColor,
        state: AVRoutePickerViewButtonState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoutePickerButtonColor : color, forState : state)
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
    unsafe fn player(&self) -> AVPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, player)
    }
    unsafe fn setPlayer_(&self, player: AVPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayer : player)
    }
    unsafe fn isRoutePickerButtonBordered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRoutePickerButtonBordered)
    }
    unsafe fn setRoutePickerButtonBordered_(&self, routePickerButtonBordered: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoutePickerButtonBordered : routePickerButtonBordered)
    }
    unsafe fn activeTintColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeTintColor)
    }
    unsafe fn setActiveTintColor_(&self, activeTintColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActiveTintColor : activeTintColor)
    }
    unsafe fn routePickerButtonStyle(&self) -> AVRoutePickerViewButtonStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, routePickerButtonStyle)
    }
    unsafe fn setRoutePickerButtonStyle_(
        &self,
        routePickerButtonStyle: AVRoutePickerViewButtonStyle,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoutePickerButtonStyle : routePickerButtonStyle)
    }
    unsafe fn prioritizesVideoDevices(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prioritizesVideoDevices)
    }
    unsafe fn setPrioritizesVideoDevices_(&self, prioritizesVideoDevices: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrioritizesVideoDevices : prioritizesVideoDevices)
    }
    unsafe fn customRoutingController(&self) -> AVCustomRoutingController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customRoutingController)
    }
    unsafe fn setCustomRoutingController_(&self, customRoutingController: AVCustomRoutingController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomRoutingController : customRoutingController)
    }
}
pub type AVRoutePickerViewButtonStyle = NSInteger;
pub trait PAVRoutePickerViewDelegate: Sized + std::ops::Deref {
    unsafe fn routePickerViewWillBeginPresentingRoutes_(&self, routePickerView: AVRoutePickerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, routePickerViewWillBeginPresentingRoutes : routePickerView)
    }
    unsafe fn routePickerViewDidEndPresentingRoutes_(&self, routePickerView: AVRoutePickerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, routePickerViewDidEndPresentingRoutes : routePickerView)
    }
}

unsafe impl objc2::encode::RefEncode for AVPlaybackSpeed {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVPlaybackSpeed {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVCaptureView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVCaptureView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVPictureInPictureController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVPictureInPictureController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVPictureInPictureControllerContentSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVPictureInPictureControllerContentSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVPlayerView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVPlayerView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVRoutePickerView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVRoutePickerView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
