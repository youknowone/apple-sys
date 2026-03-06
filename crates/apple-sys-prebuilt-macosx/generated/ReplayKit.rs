#[allow(unused_imports)]
use crate::objc::*;
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
pub struct RPPreviewViewController(pub id);
impl std::ops::Deref for RPPreviewViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for RPPreviewViewController {}
impl RPPreviewViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"RPPreviewViewController").unwrap(), alloc) })
    }
}
impl INSViewController for RPPreviewViewController {}
impl PNSEditor for RPPreviewViewController {}
impl PNSSeguePerforming for RPPreviewViewController {}
impl PNSUserInterfaceItemIdentification for RPPreviewViewController {}
impl std::convert::TryFrom<NSViewController> for RPPreviewViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<RPPreviewViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"RPPreviewViewController").unwrap()) };
        if is_kind_of {
            Ok(RPPreviewViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to RPPreviewViewController")
        }
    }
}
impl INSResponder for RPPreviewViewController {}
impl PNSCoding for RPPreviewViewController {}
impl INSObject for RPPreviewViewController {}
impl PNSObject for RPPreviewViewController {}
impl IRPPreviewViewController for RPPreviewViewController {}
pub trait IRPPreviewViewController: Sized + std::ops::Deref {
    unsafe fn previewControllerDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previewControllerDelegate)
    }
    unsafe fn setPreviewControllerDelegate_(&self, previewControllerDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviewControllerDelegate : previewControllerDelegate)
    }
}
pub trait PRPPreviewViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn previewControllerDidFinish_(&self, previewController: RPPreviewViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, previewControllerDidFinish : previewController)
    }
    unsafe fn previewController_didFinishWithActivityTypes_(
        &self,
        previewController: RPPreviewViewController,
        activityTypes: NSSet,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, previewController : previewController, didFinishWithActivityTypes : activityTypes)
    }
}
pub trait NSExtensionContext_RPBroadcastExtension: Sized + std::ops::Deref {
    unsafe fn loadBroadcastingApplicationInfoWithCompletion_(
        &self,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadBroadcastingApplicationInfoWithCompletion : handler)
    }
    unsafe fn completeRequestWithBroadcastURL_setupInfo_(
        &self,
        broadcastURL: NSURL,
        setupInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeRequestWithBroadcastURL : broadcastURL, setupInfo : setupInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RPBroadcastHandler(pub id);
impl std::ops::Deref for RPBroadcastHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for RPBroadcastHandler {}
impl RPBroadcastHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"RPBroadcastHandler").unwrap(), alloc) })
    }
}
impl PNSExtensionRequestHandling for RPBroadcastHandler {}
impl INSObject for RPBroadcastHandler {}
impl PNSObject for RPBroadcastHandler {}
impl std::convert::TryFrom<NSObject> for RPBroadcastHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<RPBroadcastHandler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"RPBroadcastHandler").unwrap()) };
        if is_kind_of {
            Ok(RPBroadcastHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to RPBroadcastHandler")
        }
    }
}
impl IRPBroadcastHandler for RPBroadcastHandler {}
pub trait IRPBroadcastHandler: Sized + std::ops::Deref {
    unsafe fn updateServiceInfo_(&self, serviceInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateServiceInfo : serviceInfo)
    }
    unsafe fn updateBroadcastURL_(&self, broadcastURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateBroadcastURL : broadcastURL)
    }
}
pub type RPSampleBufferType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RPBroadcastSampleHandler(pub id);
impl std::ops::Deref for RPBroadcastSampleHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for RPBroadcastSampleHandler {}
impl RPBroadcastSampleHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"RPBroadcastSampleHandler").unwrap(), alloc) })
    }
}
impl IRPBroadcastHandler for RPBroadcastSampleHandler {}
impl PNSExtensionRequestHandling for RPBroadcastSampleHandler {}
impl From<RPBroadcastSampleHandler> for RPBroadcastHandler {
    fn from(child: RPBroadcastSampleHandler) -> RPBroadcastHandler {
        RPBroadcastHandler(child.0)
    }
}
impl std::convert::TryFrom<RPBroadcastHandler> for RPBroadcastSampleHandler {
    type Error = &'static str;
    fn try_from(parent: RPBroadcastHandler) -> Result<RPBroadcastSampleHandler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"RPBroadcastSampleHandler").unwrap()) };
        if is_kind_of {
            Ok(RPBroadcastSampleHandler(parent.0))
        } else {
            Err("This RPBroadcastHandler cannot be downcasted to RPBroadcastSampleHandler")
        }
    }
}
impl INSObject for RPBroadcastSampleHandler {}
impl PNSObject for RPBroadcastSampleHandler {}
impl IRPBroadcastSampleHandler for RPBroadcastSampleHandler {}
pub trait IRPBroadcastSampleHandler: Sized + std::ops::Deref {
    unsafe fn broadcastStartedWithSetupInfo_(&self, setupInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, broadcastStartedWithSetupInfo : setupInfo)
    }
    unsafe fn broadcastPaused(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, broadcastPaused)
    }
    unsafe fn broadcastResumed(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, broadcastResumed)
    }
    unsafe fn broadcastFinished(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, broadcastFinished)
    }
    unsafe fn broadcastAnnotatedWithApplicationInfo_(&self, applicationInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, broadcastAnnotatedWithApplicationInfo : applicationInfo)
    }
    unsafe fn processSampleBuffer_withType_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        sampleBufferType: RPSampleBufferType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processSampleBuffer : sampleBuffer, withType : sampleBufferType)
    }
    unsafe fn finishBroadcastWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishBroadcastWithError : error)
    }
}
pub type RPCameraPosition = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RPBroadcastActivityViewController(pub id);
impl std::ops::Deref for RPBroadcastActivityViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for RPBroadcastActivityViewController {}
impl RPBroadcastActivityViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"RPBroadcastActivityViewController").unwrap(), alloc) })
    }
}
impl IRPBroadcastActivityViewController for RPBroadcastActivityViewController {}
pub trait IRPBroadcastActivityViewController: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RPScreenRecorder(pub id);
impl std::ops::Deref for RPScreenRecorder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for RPScreenRecorder {}
impl RPScreenRecorder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"RPScreenRecorder").unwrap(), alloc) })
    }
}
impl INSObject for RPScreenRecorder {}
impl PNSObject for RPScreenRecorder {}
impl std::convert::TryFrom<NSObject> for RPScreenRecorder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<RPScreenRecorder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"RPScreenRecorder").unwrap()) };
        if is_kind_of {
            Ok(RPScreenRecorder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to RPScreenRecorder")
        }
    }
}
impl IRPScreenRecorder for RPScreenRecorder {}
pub trait IRPScreenRecorder: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn startRecordingWithMicrophoneEnabled_handler_(
        &self,
        microphoneEnabled: BOOL,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startRecordingWithMicrophoneEnabled : microphoneEnabled, handler : handler)
    }
    unsafe fn startRecordingWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startRecordingWithHandler : handler)
    }
    unsafe fn stopRecordingWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopRecordingWithHandler : handler)
    }
    unsafe fn stopRecordingWithOutputURL_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopRecordingWithOutputURL : url, completionHandler : completionHandler)
    }
    unsafe fn discardRecordingWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discardRecordingWithHandler : handler)
    }
    unsafe fn startCaptureWithHandler_completionHandler_(
        &self,
        captureHandler: *mut ::std::os::raw::c_void,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startCaptureWithHandler : captureHandler, completionHandler : completionHandler)
    }
    unsafe fn stopCaptureWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopCaptureWithHandler : handler)
    }
    unsafe fn startClipBufferingWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startClipBufferingWithCompletionHandler : completionHandler)
    }
    unsafe fn stopClipBufferingWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopClipBufferingWithCompletionHandler : completionHandler)
    }
    unsafe fn exportClipToURL_duration_completionHandler_(
        &self,
        url: NSURL,
        duration: NSTimeInterval,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exportClipToURL : url, duration : duration, completionHandler : completionHandler)
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
    unsafe fn isAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAvailable)
    }
    unsafe fn isRecording(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRecording)
    }
    unsafe fn isMicrophoneEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMicrophoneEnabled)
    }
    unsafe fn setMicrophoneEnabled_(&self, microphoneEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMicrophoneEnabled : microphoneEnabled)
    }
    unsafe fn isCameraEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCameraEnabled)
    }
    unsafe fn setCameraEnabled_(&self, cameraEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraEnabled : cameraEnabled)
    }
    unsafe fn cameraPosition(&self) -> RPCameraPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraPosition)
    }
    unsafe fn setCameraPosition_(&self, cameraPosition: RPCameraPosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraPosition : cameraPosition)
    }
    unsafe fn cameraPreviewView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraPreviewView)
    }
    unsafe fn sharedRecorder() -> RPScreenRecorder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"RPScreenRecorder").unwrap(), sharedRecorder)
    }
}
pub trait PRPScreenRecorderDelegate: Sized + std::ops::Deref {
    unsafe fn screenRecorder_didStopRecordingWithError_previewViewController_(
        &self,
        screenRecorder: RPScreenRecorder,
        error: NSError,
        previewViewController: RPPreviewViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, screenRecorder : screenRecorder, didStopRecordingWithError : error, previewViewController : previewViewController)
    }
    unsafe fn screenRecorder_didStopRecordingWithPreviewViewController_error_(
        &self,
        screenRecorder: RPScreenRecorder,
        previewViewController: RPPreviewViewController,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, screenRecorder : screenRecorder, didStopRecordingWithPreviewViewController : previewViewController, error : error)
    }
    unsafe fn screenRecorderDidChangeAvailability_(&self, screenRecorder: RPScreenRecorder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, screenRecorderDidChangeAvailability : screenRecorder)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RPBroadcastActivityController(pub id);
impl std::ops::Deref for RPBroadcastActivityController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for RPBroadcastActivityController {}
impl RPBroadcastActivityController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"RPBroadcastActivityController").unwrap(), alloc) })
    }
}
impl INSObject for RPBroadcastActivityController {}
impl PNSObject for RPBroadcastActivityController {}
impl std::convert::TryFrom<NSObject> for RPBroadcastActivityController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<RPBroadcastActivityController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"RPBroadcastActivityController").unwrap())
        };
        if is_kind_of {
            Ok(RPBroadcastActivityController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to RPBroadcastActivityController")
        }
    }
}
impl IRPBroadcastActivityController for RPBroadcastActivityController {}
pub trait IRPBroadcastActivityController: Sized + std::ops::Deref {
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
    unsafe fn showBroadcastPickerAtPoint_fromWindow_preferredExtensionIdentifier_completionHandler_(
        point: CGPoint,
        window: NSWindow,
        preferredExtension: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"RPBroadcastActivityController").unwrap(), showBroadcastPickerAtPoint : point, fromWindow : window, preferredExtensionIdentifier : preferredExtension, completionHandler : handler)
    }
}
pub trait PRPBroadcastActivityControllerDelegate: Sized + std::ops::Deref {
    unsafe fn broadcastActivityController_didFinishWithBroadcastController_error_(
        &self,
        broadcastActivityController: RPBroadcastActivityController,
        broadcastController: RPBroadcastController,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, broadcastActivityController : broadcastActivityController, didFinishWithBroadcastController : broadcastController, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RPBroadcastController(pub id);
impl std::ops::Deref for RPBroadcastController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for RPBroadcastController {}
impl RPBroadcastController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"RPBroadcastController").unwrap(), alloc) })
    }
}
impl INSObject for RPBroadcastController {}
impl PNSObject for RPBroadcastController {}
impl std::convert::TryFrom<NSObject> for RPBroadcastController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<RPBroadcastController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"RPBroadcastController").unwrap()) };
        if is_kind_of {
            Ok(RPBroadcastController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to RPBroadcastController")
        }
    }
}
impl IRPBroadcastController for RPBroadcastController {}
pub trait IRPBroadcastController: Sized + std::ops::Deref {
    unsafe fn startBroadcastWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startBroadcastWithHandler : handler)
    }
    unsafe fn pauseBroadcast(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pauseBroadcast)
    }
    unsafe fn resumeBroadcast(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resumeBroadcast)
    }
    unsafe fn finishBroadcastWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishBroadcastWithHandler : handler)
    }
    unsafe fn isBroadcasting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBroadcasting)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn broadcastURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, broadcastURL)
    }
    unsafe fn serviceInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceInfo)
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
    unsafe fn broadcastExtensionBundleID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, broadcastExtensionBundleID)
    }
}
pub trait PRPBroadcastControllerDelegate: Sized + std::ops::Deref {
    unsafe fn broadcastController_didFinishWithError_(
        &self,
        broadcastController: RPBroadcastController,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, broadcastController : broadcastController, didFinishWithError : error)
    }
    unsafe fn broadcastController_didUpdateServiceInfo_(
        &self,
        broadcastController: RPBroadcastController,
        serviceInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, broadcastController : broadcastController, didUpdateServiceInfo : serviceInfo)
    }
    unsafe fn broadcastController_didUpdateBroadcastURL_(
        &self,
        broadcastController: RPBroadcastController,
        broadcastURL: NSURL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, broadcastController : broadcastController, didUpdateBroadcastURL : broadcastURL)
    }
}
pub type RPRecordingErrorCode = NSInteger;
unsafe extern "C" {
    pub static RPVideoSampleOrientationKey: NSString;
}
unsafe extern "C" {
    pub static RPApplicationInfoBundleIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static RPRecordingErrorDomain: NSString;
}
unsafe extern "C" {
    pub static SCStreamErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for RPPreviewViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RPPreviewViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for RPBroadcastHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RPBroadcastHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for RPBroadcastSampleHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RPBroadcastSampleHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for RPBroadcastActivityViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RPBroadcastActivityViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for RPScreenRecorder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RPScreenRecorder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for RPBroadcastActivityController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RPBroadcastActivityController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for RPBroadcastController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for RPBroadcastController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
