#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;
#[allow(unused_imports)]
use libc::{id_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type SCShareableContentStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCRunningApplication(pub id);
impl std::ops::Deref for SCRunningApplication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCRunningApplication {}
impl SCRunningApplication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCRunningApplication").unwrap(), alloc) })
    }
}
impl INSObject for SCRunningApplication {}
impl PNSObject for SCRunningApplication {}
impl std::convert::TryFrom<NSObject> for SCRunningApplication {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCRunningApplication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCRunningApplication").unwrap()) };
        if is_kind_of {
            Ok(SCRunningApplication(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCRunningApplication")
        }
    }
}
impl ISCRunningApplication for SCRunningApplication {}
pub trait ISCRunningApplication: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn applicationName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationName)
    }
    unsafe fn processID(&self) -> pid_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processID)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCRunningApplication").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCWindow(pub id);
impl std::ops::Deref for SCWindow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCWindow {}
impl SCWindow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCWindow").unwrap(), alloc) })
    }
}
impl INSObject for SCWindow {}
impl PNSObject for SCWindow {}
impl std::convert::TryFrom<NSObject> for SCWindow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCWindow, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCWindow").unwrap()) };
        if is_kind_of {
            Ok(SCWindow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCWindow")
        }
    }
}
impl ISCWindow for SCWindow {}
pub trait ISCWindow: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn windowID(&self) -> CGWindowID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowID)
    }
    unsafe fn frame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frame)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn windowLayer(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowLayer)
    }
    unsafe fn owningApplication(&self) -> SCRunningApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, owningApplication)
    }
    unsafe fn isOnScreen(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOnScreen)
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCWindow").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCDisplay(pub id);
impl std::ops::Deref for SCDisplay {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCDisplay {}
impl SCDisplay {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCDisplay").unwrap(), alloc) })
    }
}
impl INSObject for SCDisplay {}
impl PNSObject for SCDisplay {}
impl std::convert::TryFrom<NSObject> for SCDisplay {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCDisplay, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCDisplay").unwrap()) };
        if is_kind_of {
            Ok(SCDisplay(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCDisplay")
        }
    }
}
impl ISCDisplay for SCDisplay {}
pub trait ISCDisplay: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn displayID(&self) -> CGDirectDisplayID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayID)
    }
    unsafe fn width(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn frame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frame)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCDisplay").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCShareableContentInfo(pub id);
impl std::ops::Deref for SCShareableContentInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCShareableContentInfo {}
impl SCShareableContentInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContentInfo").unwrap(), alloc) })
    }
}
impl INSObject for SCShareableContentInfo {}
impl PNSObject for SCShareableContentInfo {}
impl std::convert::TryFrom<NSObject> for SCShareableContentInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCShareableContentInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCShareableContentInfo").unwrap()) };
        if is_kind_of {
            Ok(SCShareableContentInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCShareableContentInfo")
        }
    }
}
impl ISCShareableContentInfo for SCShareableContentInfo {}
pub trait ISCShareableContentInfo: Sized + std::ops::Deref {
    unsafe fn style(&self) -> SCShareableContentStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn pointPixelScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointPixelScale)
    }
    unsafe fn contentRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentRect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCShareableContent(pub id);
impl std::ops::Deref for SCShareableContent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCShareableContent {}
impl SCShareableContent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap(), alloc) })
    }
}
impl INSObject for SCShareableContent {}
impl PNSObject for SCShareableContent {}
impl std::convert::TryFrom<NSObject> for SCShareableContent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCShareableContent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap()) };
        if is_kind_of {
            Ok(SCShareableContent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCShareableContent")
        }
    }
}
impl ISCShareableContent for SCShareableContent {}
pub trait ISCShareableContent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn windows(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windows)
    }
    unsafe fn displays(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displays)
    }
    unsafe fn applications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applications)
    }
    unsafe fn getShareableContentWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap(), getShareableContentWithCompletionHandler : completionHandler)
    }
    unsafe fn getCurrentProcessShareableContentWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap(), getCurrentProcessShareableContentWithCompletionHandler : completionHandler)
    }
    unsafe fn getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler_(
        excludeDesktopWindows: BOOL,
        onScreenWindowsOnly: BOOL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap(), getShareableContentExcludingDesktopWindows : excludeDesktopWindows, onScreenWindowsOnly : onScreenWindowsOnly, completionHandler : completionHandler)
    }
    unsafe fn getShareableContentExcludingDesktopWindows_onScreenWindowsOnlyBelowWindow_completionHandler_(
        excludeDesktopWindows: BOOL,
        window: SCWindow,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap(), getShareableContentExcludingDesktopWindows : excludeDesktopWindows, onScreenWindowsOnlyBelowWindow : window, completionHandler : completionHandler)
    }
    unsafe fn getShareableContentExcludingDesktopWindows_onScreenWindowsOnlyAboveWindow_completionHandler_(
        excludeDesktopWindows: BOOL,
        window: SCWindow,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap(), getShareableContentExcludingDesktopWindows : excludeDesktopWindows, onScreenWindowsOnlyAboveWindow : window, completionHandler : completionHandler)
    }
    unsafe fn infoForFilter_(filter: SCContentFilter) -> SCShareableContentInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap(), infoForFilter : filter)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCShareableContent").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCRecordingOutputConfiguration(pub id);
impl std::ops::Deref for SCRecordingOutputConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCRecordingOutputConfiguration {}
impl SCRecordingOutputConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCRecordingOutputConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for SCRecordingOutputConfiguration {}
impl PNSObject for SCRecordingOutputConfiguration {}
impl std::convert::TryFrom<NSObject> for SCRecordingOutputConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCRecordingOutputConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCRecordingOutputConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(SCRecordingOutputConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCRecordingOutputConfiguration")
        }
    }
}
impl ISCRecordingOutputConfiguration for SCRecordingOutputConfiguration {}
pub trait ISCRecordingOutputConfiguration: Sized + std::ops::Deref {
    unsafe fn outputURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputURL)
    }
    unsafe fn setOutputURL_(&self, outputURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputURL : outputURL)
    }
    unsafe fn videoCodecType(&self) -> AVVideoCodecType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoCodecType)
    }
    unsafe fn setVideoCodecType_(&self, videoCodecType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoCodecType : videoCodecType)
    }
    unsafe fn outputFileType(&self) -> AVFileType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputFileType)
    }
    unsafe fn setOutputFileType_(&self, outputFileType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputFileType : outputFileType)
    }
    unsafe fn availableVideoCodecTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableVideoCodecTypes)
    }
    unsafe fn availableOutputFileTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableOutputFileTypes)
    }
}
pub trait PSCRecordingOutputDelegate: Sized + std::ops::Deref {
    unsafe fn recordingOutputDidStartRecording_(&self, recordingOutput: SCRecordingOutput)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordingOutputDidStartRecording : recordingOutput)
    }
    unsafe fn recordingOutput_didFailWithError_(
        &self,
        recordingOutput: SCRecordingOutput,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordingOutput : recordingOutput, didFailWithError : error)
    }
    unsafe fn recordingOutputDidFinishRecording_(&self, recordingOutput: SCRecordingOutput)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordingOutputDidFinishRecording : recordingOutput)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCRecordingOutput(pub id);
impl std::ops::Deref for SCRecordingOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCRecordingOutput {}
impl SCRecordingOutput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCRecordingOutput").unwrap(), alloc) })
    }
}
impl INSObject for SCRecordingOutput {}
impl PNSObject for SCRecordingOutput {}
impl std::convert::TryFrom<NSObject> for SCRecordingOutput {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCRecordingOutput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCRecordingOutput").unwrap()) };
        if is_kind_of {
            Ok(SCRecordingOutput(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCRecordingOutput")
        }
    }
}
impl ISCRecordingOutput for SCRecordingOutput {}
pub trait ISCRecordingOutput: Sized + std::ops::Deref {
    unsafe fn initWithConfiguration_delegate_(
        &self,
        recordingOutputConfiguration: SCRecordingOutputConfiguration,
        delegate: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : recordingOutputConfiguration, delegate : delegate)
    }
    unsafe fn recordedDuration(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordedDuration)
    }
    unsafe fn recordedFileSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordedFileSize)
    }
}
pub type SCStreamOutputType = NSInteger;
pub type SCFrameStatus = NSInteger;
pub type SCPresenterOverlayAlertSetting = NSInteger;
pub type SCStreamType = NSInteger;
pub type SCCaptureResolutionType = NSInteger;
pub type SCCaptureDynamicRange = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCContentFilter(pub id);
impl std::ops::Deref for SCContentFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCContentFilter {}
impl SCContentFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCContentFilter").unwrap(), alloc) })
    }
}
impl INSObject for SCContentFilter {}
impl PNSObject for SCContentFilter {}
impl std::convert::TryFrom<NSObject> for SCContentFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCContentFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCContentFilter").unwrap()) };
        if is_kind_of {
            Ok(SCContentFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCContentFilter")
        }
    }
}
impl ISCContentFilter for SCContentFilter {}
pub trait ISCContentFilter: Sized + std::ops::Deref {
    unsafe fn initWithDesktopIndependentWindow_(&self, window: SCWindow) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDesktopIndependentWindow : window)
    }
    unsafe fn initWithDisplay_excludingWindows_(
        &self,
        display: SCDisplay,
        excluded: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisplay : display, excludingWindows : excluded)
    }
    unsafe fn initWithDisplay_includingWindows_(
        &self,
        display: SCDisplay,
        includedWindows: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisplay : display, includingWindows : includedWindows)
    }
    unsafe fn initWithDisplay_includingApplications_exceptingWindows_(
        &self,
        display: SCDisplay,
        applications: NSArray,
        exceptingWindows: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisplay : display, includingApplications : applications, exceptingWindows : exceptingWindows)
    }
    unsafe fn initWithDisplay_excludingApplications_exceptingWindows_(
        &self,
        display: SCDisplay,
        applications: NSArray,
        exceptingWindows: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisplay : display, excludingApplications : applications, exceptingWindows : exceptingWindows)
    }
    unsafe fn streamType(&self) -> SCStreamType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamType)
    }
    unsafe fn style(&self) -> SCShareableContentStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn pointPixelScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointPixelScale)
    }
    unsafe fn contentRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentRect)
    }
    unsafe fn includeMenuBar(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeMenuBar)
    }
    unsafe fn setIncludeMenuBar_(&self, includeMenuBar: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeMenuBar : includeMenuBar)
    }
    unsafe fn includedDisplays(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includedDisplays)
    }
    unsafe fn includedApplications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includedApplications)
    }
    unsafe fn includedWindows(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includedWindows)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCStreamConfiguration(pub id);
impl std::ops::Deref for SCStreamConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCStreamConfiguration {}
impl SCStreamConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCStreamConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for SCStreamConfiguration {}
impl PNSObject for SCStreamConfiguration {}
impl std::convert::TryFrom<NSObject> for SCStreamConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCStreamConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCStreamConfiguration").unwrap()) };
        if is_kind_of {
            Ok(SCStreamConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCStreamConfiguration")
        }
    }
}
impl ISCStreamConfiguration for SCStreamConfiguration {}
pub trait ISCStreamConfiguration: Sized + std::ops::Deref {
    unsafe fn width(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: usize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: usize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn minimumFrameInterval(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumFrameInterval)
    }
    unsafe fn setMinimumFrameInterval_(&self, minimumFrameInterval: CMTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumFrameInterval : minimumFrameInterval)
    }
    unsafe fn pixelFormat(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn setPixelFormat_(&self, pixelFormat: OSType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelFormat : pixelFormat)
    }
    unsafe fn scalesToFit(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scalesToFit)
    }
    unsafe fn setScalesToFit_(&self, scalesToFit: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScalesToFit : scalesToFit)
    }
    unsafe fn preservesAspectRatio(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preservesAspectRatio)
    }
    unsafe fn setPreservesAspectRatio_(&self, preservesAspectRatio: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreservesAspectRatio : preservesAspectRatio)
    }
    unsafe fn streamName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamName)
    }
    unsafe fn setStreamName_(&self, streamName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStreamName : streamName)
    }
    unsafe fn showsCursor(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsCursor)
    }
    unsafe fn setShowsCursor_(&self, showsCursor: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsCursor : showsCursor)
    }
    unsafe fn showMouseClicks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showMouseClicks)
    }
    unsafe fn setShowMouseClicks_(&self, showMouseClicks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowMouseClicks : showMouseClicks)
    }
    unsafe fn backgroundColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, backgroundColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : backgroundColor)
    }
    unsafe fn sourceRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceRect)
    }
    unsafe fn setSourceRect_(&self, sourceRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceRect : sourceRect)
    }
    unsafe fn destinationRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationRect)
    }
    unsafe fn setDestinationRect_(&self, destinationRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationRect : destinationRect)
    }
    unsafe fn queueDepth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queueDepth)
    }
    unsafe fn setQueueDepth_(&self, queueDepth: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueueDepth : queueDepth)
    }
    unsafe fn colorMatrix(&self) -> CFStringRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorMatrix)
    }
    unsafe fn setColorMatrix_(&self, colorMatrix: CFStringRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorMatrix : colorMatrix)
    }
    unsafe fn colorSpaceName(&self) -> CFStringRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpaceName)
    }
    unsafe fn setColorSpaceName_(&self, colorSpaceName: CFStringRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpaceName : colorSpaceName)
    }
    unsafe fn capturesAudio(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capturesAudio)
    }
    unsafe fn setCapturesAudio_(&self, capturesAudio: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCapturesAudio : capturesAudio)
    }
    unsafe fn sampleRate(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleRate)
    }
    unsafe fn setSampleRate_(&self, sampleRate: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleRate : sampleRate)
    }
    unsafe fn channelCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelCount)
    }
    unsafe fn setChannelCount_(&self, channelCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannelCount : channelCount)
    }
    unsafe fn excludesCurrentProcessAudio(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludesCurrentProcessAudio)
    }
    unsafe fn setExcludesCurrentProcessAudio_(&self, excludesCurrentProcessAudio: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludesCurrentProcessAudio : excludesCurrentProcessAudio)
    }
    unsafe fn ignoreShadowsDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoreShadowsDisplay)
    }
    unsafe fn setIgnoreShadowsDisplay_(&self, ignoreShadowsDisplay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoreShadowsDisplay : ignoreShadowsDisplay)
    }
    unsafe fn ignoreShadowsSingleWindow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoreShadowsSingleWindow)
    }
    unsafe fn setIgnoreShadowsSingleWindow_(&self, ignoreShadowsSingleWindow: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoreShadowsSingleWindow : ignoreShadowsSingleWindow)
    }
    unsafe fn captureResolution(&self) -> SCCaptureResolutionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captureResolution)
    }
    unsafe fn setCaptureResolution_(&self, captureResolution: SCCaptureResolutionType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaptureResolution : captureResolution)
    }
    unsafe fn capturesShadowsOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capturesShadowsOnly)
    }
    unsafe fn setCapturesShadowsOnly_(&self, capturesShadowsOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCapturesShadowsOnly : capturesShadowsOnly)
    }
    unsafe fn shouldBeOpaque(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBeOpaque)
    }
    unsafe fn setShouldBeOpaque_(&self, shouldBeOpaque: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldBeOpaque : shouldBeOpaque)
    }
    unsafe fn ignoreGlobalClipDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoreGlobalClipDisplay)
    }
    unsafe fn setIgnoreGlobalClipDisplay_(&self, ignoreGlobalClipDisplay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoreGlobalClipDisplay : ignoreGlobalClipDisplay)
    }
    unsafe fn ignoreGlobalClipSingleWindow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoreGlobalClipSingleWindow)
    }
    unsafe fn setIgnoreGlobalClipSingleWindow_(&self, ignoreGlobalClipSingleWindow: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoreGlobalClipSingleWindow : ignoreGlobalClipSingleWindow)
    }
    unsafe fn presenterOverlayPrivacyAlertSetting(&self) -> SCPresenterOverlayAlertSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presenterOverlayPrivacyAlertSetting)
    }
    unsafe fn setPresenterOverlayPrivacyAlertSetting_(
        &self,
        presenterOverlayPrivacyAlertSetting: SCPresenterOverlayAlertSetting,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresenterOverlayPrivacyAlertSetting : presenterOverlayPrivacyAlertSetting)
    }
    unsafe fn includeChildWindows(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeChildWindows)
    }
    unsafe fn setIncludeChildWindows_(&self, includeChildWindows: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeChildWindows : includeChildWindows)
    }
    unsafe fn captureMicrophone(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captureMicrophone)
    }
    unsafe fn setCaptureMicrophone_(&self, captureMicrophone: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaptureMicrophone : captureMicrophone)
    }
    unsafe fn microphoneCaptureDeviceID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, microphoneCaptureDeviceID)
    }
    unsafe fn setMicrophoneCaptureDeviceID_(&self, microphoneCaptureDeviceID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMicrophoneCaptureDeviceID : microphoneCaptureDeviceID)
    }
    unsafe fn captureDynamicRange(&self) -> SCCaptureDynamicRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captureDynamicRange)
    }
    unsafe fn setCaptureDynamicRange_(&self, captureDynamicRange: SCCaptureDynamicRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaptureDynamicRange : captureDynamicRange)
    }
    unsafe fn streamConfigurationWithPreset_(preset: SCStreamConfigurationPreset) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCStreamConfiguration").unwrap(), streamConfigurationWithPreset : preset)
    }
}
pub type SCStreamConfigurationPreset = NSInteger;
pub type SCStreamFrameInfo = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCStream(pub id);
impl std::ops::Deref for SCStream {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCStream {}
impl SCStream {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCStream").unwrap(), alloc) })
    }
}
impl INSObject for SCStream {}
impl PNSObject for SCStream {}
impl std::convert::TryFrom<NSObject> for SCStream {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCStream, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCStream").unwrap()) };
        if is_kind_of {
            Ok(SCStream(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCStream")
        }
    }
}
impl ISCStream for SCStream {}
pub trait ISCStream: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFilter_configuration_delegate_(
        &self,
        contentFilter: SCContentFilter,
        streamConfig: SCStreamConfiguration,
        delegate: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFilter : contentFilter, configuration : streamConfig, delegate : delegate)
    }
    unsafe fn addStreamOutput_type_sampleHandlerQueue_error_(
        &self,
        output: *mut u64,
        type_: SCStreamOutputType,
        sampleHandlerQueue: NSObject,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addStreamOutput : output, r#type : type_, sampleHandlerQueue : sampleHandlerQueue, error : error)
    }
    unsafe fn removeStreamOutput_type_error_(
        &self,
        output: *mut u64,
        type_: SCStreamOutputType,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeStreamOutput : output, r#type : type_, error : error)
    }
    unsafe fn updateContentFilter_completionHandler_(
        &self,
        contentFilter: SCContentFilter,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateContentFilter : contentFilter, completionHandler : completionHandler)
    }
    unsafe fn updateConfiguration_completionHandler_(
        &self,
        streamConfig: SCStreamConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateConfiguration : streamConfig, completionHandler : completionHandler)
    }
    unsafe fn startCaptureWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startCaptureWithCompletionHandler : completionHandler)
    }
    unsafe fn stopCaptureWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopCaptureWithCompletionHandler : completionHandler)
    }
    unsafe fn addRecordingOutput_error_(
        &self,
        recordingOutput: SCRecordingOutput,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRecordingOutput : recordingOutput, error : error)
    }
    unsafe fn removeRecordingOutput_error_(
        &self,
        recordingOutput: SCRecordingOutput,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRecordingOutput : recordingOutput, error : error)
    }
    unsafe fn synchronizationClock(&self) -> CMClockRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, synchronizationClock)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCStream").unwrap(), new)
    }
}
pub trait PSCStreamOutput: Sized + std::ops::Deref {
    unsafe fn stream_didOutputSampleBuffer_ofType_(
        &self,
        stream: SCStream,
        sampleBuffer: CMSampleBufferRef,
        type_: SCStreamOutputType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stream : stream, didOutputSampleBuffer : sampleBuffer, ofType : type_)
    }
}
pub trait PSCStreamDelegate: Sized + std::ops::Deref {
    unsafe fn stream_didStopWithError_(&self, stream: SCStream, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stream : stream, didStopWithError : error)
    }
    unsafe fn outputVideoEffectDidStartForStream_(&self, stream: SCStream)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, outputVideoEffectDidStartForStream : stream)
    }
    unsafe fn outputVideoEffectDidStopForStream_(&self, stream: SCStream)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, outputVideoEffectDidStopForStream : stream)
    }
    unsafe fn streamDidBecomeActive_(&self, stream: SCStream)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, streamDidBecomeActive : stream)
    }
    unsafe fn streamDidBecomeInactive_(&self, stream: SCStream)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, streamDidBecomeInactive : stream)
    }
}
pub type SCStreamErrorCode = NSInteger;
pub type SCContentSharingPickerMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCContentSharingPickerConfiguration(pub id);
impl std::ops::Deref for SCContentSharingPickerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCContentSharingPickerConfiguration {}
impl SCContentSharingPickerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCContentSharingPickerConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for SCContentSharingPickerConfiguration {}
impl PNSObject for SCContentSharingPickerConfiguration {}
impl std::convert::TryFrom<NSObject> for SCContentSharingPickerConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCContentSharingPickerConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCContentSharingPickerConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(SCContentSharingPickerConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCContentSharingPickerConfiguration")
        }
    }
}
impl<NSCopying: 'static> ISCContentSharingPickerConfiguration<NSCopying>
    for SCContentSharingPickerConfiguration
{
}
pub trait ISCContentSharingPickerConfiguration<NSCopying: 'static>:
    Sized + std::ops::Deref
{
    unsafe fn allowedPickerModes(&self) -> SCContentSharingPickerMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedPickerModes)
    }
    unsafe fn setAllowedPickerModes_(&self, allowedPickerModes: SCContentSharingPickerMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedPickerModes : allowedPickerModes)
    }
    unsafe fn excludedWindowIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedWindowIDs)
    }
    unsafe fn setExcludedWindowIDs_(&self, excludedWindowIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedWindowIDs : excludedWindowIDs)
    }
    unsafe fn excludedBundleIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedBundleIDs)
    }
    unsafe fn setExcludedBundleIDs_(&self, excludedBundleIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedBundleIDs : excludedBundleIDs)
    }
    unsafe fn allowsChangingSelectedContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsChangingSelectedContent)
    }
    unsafe fn setAllowsChangingSelectedContent_(&self, allowsChangingSelectedContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsChangingSelectedContent : allowsChangingSelectedContent)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCContentSharingPicker(pub id);
impl std::ops::Deref for SCContentSharingPicker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCContentSharingPicker {}
impl SCContentSharingPicker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCContentSharingPicker").unwrap(), alloc) })
    }
}
impl INSObject for SCContentSharingPicker {}
impl PNSObject for SCContentSharingPicker {}
impl std::convert::TryFrom<NSObject> for SCContentSharingPicker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCContentSharingPicker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCContentSharingPicker").unwrap()) };
        if is_kind_of {
            Ok(SCContentSharingPicker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCContentSharingPicker")
        }
    }
}
impl ISCContentSharingPicker for SCContentSharingPicker {}
pub trait ISCContentSharingPicker: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObserver : observer)
    }
    unsafe fn removeObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObserver : observer)
    }
    unsafe fn setConfiguration_forStream_(
        &self,
        pickerConfig: SCContentSharingPickerConfiguration,
        stream: SCStream,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfiguration : pickerConfig, forStream : stream)
    }
    unsafe fn present(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, present)
    }
    unsafe fn presentPickerUsingContentStyle_(&self, contentStyle: SCShareableContentStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentPickerUsingContentStyle : contentStyle)
    }
    unsafe fn presentPickerForStream_(&self, stream: SCStream)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentPickerForStream : stream)
    }
    unsafe fn presentPickerForStream_usingContentStyle_(
        &self,
        stream: SCStream,
        contentStyle: SCShareableContentStyle,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentPickerForStream : stream, usingContentStyle : contentStyle)
    }
    unsafe fn defaultConfiguration(&self) -> SCContentSharingPickerConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultConfiguration)
    }
    unsafe fn setDefaultConfiguration_(
        &self,
        defaultConfiguration: SCContentSharingPickerConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultConfiguration : defaultConfiguration)
    }
    unsafe fn maximumStreamCount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumStreamCount)
    }
    unsafe fn setMaximumStreamCount_(&self, maximumStreamCount: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumStreamCount : maximumStreamCount)
    }
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCContentSharingPicker").unwrap(), new)
    }
    unsafe fn sharedPicker() -> SCContentSharingPicker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCContentSharingPicker").unwrap(), sharedPicker)
    }
}
pub trait PSCContentSharingPickerObserver: Sized + std::ops::Deref {
    unsafe fn contentSharingPicker_didCancelForStream_(
        &self,
        picker: SCContentSharingPicker,
        stream: SCStream,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentSharingPicker : picker, didCancelForStream : stream)
    }
    unsafe fn contentSharingPicker_didUpdateWithFilter_forStream_(
        &self,
        picker: SCContentSharingPicker,
        filter: SCContentFilter,
        stream: SCStream,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentSharingPicker : picker, didUpdateWithFilter : filter, forStream : stream)
    }
    unsafe fn contentSharingPickerStartDidFailWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentSharingPickerStartDidFailWithError : error)
    }
}
pub type SCScreenshotDisplayIntent = NSInteger;
pub type SCScreenshotDynamicRange = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCScreenshotConfiguration(pub id);
impl std::ops::Deref for SCScreenshotConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCScreenshotConfiguration {}
impl SCScreenshotConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for SCScreenshotConfiguration {}
impl PNSObject for SCScreenshotConfiguration {}
impl std::convert::TryFrom<NSObject> for SCScreenshotConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCScreenshotConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCScreenshotConfiguration").unwrap()) };
        if is_kind_of {
            Ok(SCScreenshotConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCScreenshotConfiguration")
        }
    }
}
impl ISCScreenshotConfiguration for SCScreenshotConfiguration {}
pub trait ISCScreenshotConfiguration: Sized + std::ops::Deref {
    unsafe fn width(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn showsCursor(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsCursor)
    }
    unsafe fn setShowsCursor_(&self, showsCursor: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsCursor : showsCursor)
    }
    unsafe fn sourceRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceRect)
    }
    unsafe fn setSourceRect_(&self, sourceRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceRect : sourceRect)
    }
    unsafe fn destinationRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationRect)
    }
    unsafe fn setDestinationRect_(&self, destinationRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationRect : destinationRect)
    }
    unsafe fn ignoreShadows(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoreShadows)
    }
    unsafe fn setIgnoreShadows_(&self, ignoreShadows: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoreShadows : ignoreShadows)
    }
    unsafe fn ignoreClipping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoreClipping)
    }
    unsafe fn setIgnoreClipping_(&self, ignoreClipping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoreClipping : ignoreClipping)
    }
    unsafe fn includeChildWindows(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeChildWindows)
    }
    unsafe fn setIncludeChildWindows_(&self, includeChildWindows: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeChildWindows : includeChildWindows)
    }
    unsafe fn displayIntent(&self) -> SCScreenshotDisplayIntent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayIntent)
    }
    unsafe fn setDisplayIntent_(&self, displayIntent: SCScreenshotDisplayIntent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayIntent : displayIntent)
    }
    unsafe fn dynamicRange(&self) -> SCScreenshotDynamicRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dynamicRange)
    }
    unsafe fn setDynamicRange_(&self, dynamicRange: SCScreenshotDynamicRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDynamicRange : dynamicRange)
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
    unsafe fn fileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileURL)
    }
    unsafe fn setFileURL_(&self, fileURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFileURL : fileURL)
    }
    unsafe fn supportedContentTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotConfiguration").unwrap(), supportedContentTypes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCScreenshotOutput(pub id);
impl std::ops::Deref for SCScreenshotOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCScreenshotOutput {}
impl SCScreenshotOutput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotOutput").unwrap(), alloc) })
    }
}
impl INSObject for SCScreenshotOutput {}
impl PNSObject for SCScreenshotOutput {}
impl std::convert::TryFrom<NSObject> for SCScreenshotOutput {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCScreenshotOutput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCScreenshotOutput").unwrap()) };
        if is_kind_of {
            Ok(SCScreenshotOutput(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCScreenshotOutput")
        }
    }
}
impl ISCScreenshotOutput for SCScreenshotOutput {}
pub trait ISCScreenshotOutput: Sized + std::ops::Deref {
    unsafe fn sdrImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sdrImage)
    }
    unsafe fn setSdrImage_(&self, sdrImage: CGImageRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSdrImage : sdrImage)
    }
    unsafe fn hdrImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hdrImage)
    }
    unsafe fn setHdrImage_(&self, hdrImage: CGImageRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHdrImage : hdrImage)
    }
    unsafe fn fileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileURL)
    }
    unsafe fn setFileURL_(&self, fileURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFileURL : fileURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCScreenshotManager(pub id);
impl std::ops::Deref for SCScreenshotManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCScreenshotManager {}
impl SCScreenshotManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotManager").unwrap(), alloc) })
    }
}
impl INSObject for SCScreenshotManager {}
impl PNSObject for SCScreenshotManager {}
impl std::convert::TryFrom<NSObject> for SCScreenshotManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCScreenshotManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCScreenshotManager").unwrap()) };
        if is_kind_of {
            Ok(SCScreenshotManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCScreenshotManager")
        }
    }
}
impl ISCScreenshotManager for SCScreenshotManager {}
pub trait ISCScreenshotManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn captureSampleBufferWithFilter_configuration_completionHandler_(
        contentFilter: SCContentFilter,
        config: SCStreamConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotManager").unwrap(), captureSampleBufferWithFilter : contentFilter, configuration : config, completionHandler : completionHandler)
    }
    unsafe fn captureImageWithFilter_configuration_completionHandler_(
        contentFilter: SCContentFilter,
        config: SCStreamConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotManager").unwrap(), captureImageWithFilter : contentFilter, configuration : config, completionHandler : completionHandler)
    }
    unsafe fn captureImageInRect_completionHandler_(
        rect: CGRect,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotManager").unwrap(), captureImageInRect : rect, completionHandler : completionHandler)
    }
    unsafe fn captureScreenshotWithFilter_configuration_completionHandler_(
        contentFilter: SCContentFilter,
        config: SCScreenshotConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotManager").unwrap(), captureScreenshotWithFilter : contentFilter, configuration : config, completionHandler : completionHandler)
    }
    unsafe fn captureScreenshotWithRect_configuration_completionHandler_(
        rect: CGRect,
        config: SCScreenshotConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCScreenshotManager").unwrap(), captureScreenshotWithRect : rect, configuration : config, completionHandler : completionHandler)
    }
}
unsafe extern "C" {
    pub static SCStreamFrameInfoStatus: SCStreamFrameInfo;
}
unsafe extern "C" {
    pub static SCStreamFrameInfoDisplayTime: SCStreamFrameInfo;
}
unsafe extern "C" {
    pub static SCStreamFrameInfoScaleFactor: SCStreamFrameInfo;
}
unsafe extern "C" {
    pub static SCStreamFrameInfoContentScale: SCStreamFrameInfo;
}
unsafe extern "C" {
    pub static SCStreamFrameInfoContentRect: SCStreamFrameInfo;
}
unsafe extern "C" {
    pub static SCStreamFrameInfoDirtyRects: SCStreamFrameInfo;
}
unsafe extern "C" {
    pub static SCStreamFrameInfoScreenRect: SCStreamFrameInfo;
}
unsafe extern "C" {
    pub static SCStreamFrameInfoBoundingRect: SCStreamFrameInfo;
}
unsafe extern "C" {
    pub static SCStreamFrameInfoPresenterOverlayContentRect: SCStreamFrameInfo;
}

unsafe impl objc2::encode::RefEncode for SCRunningApplication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCRunningApplication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCWindow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCWindow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCDisplay {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCDisplay {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCShareableContentInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCShareableContentInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCShareableContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCShareableContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCRecordingOutputConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCRecordingOutputConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCRecordingOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCRecordingOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCContentFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCContentFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCStreamConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCStreamConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCContentSharingPickerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCContentSharingPickerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCContentSharingPicker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCContentSharingPicker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCScreenshotConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCScreenshotConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCScreenshotOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCScreenshotOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCScreenshotManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCScreenshotManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
