#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::OpenGL::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMAVControl(pub id);
impl std::ops::Deref for IMAVControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMAVControl {}
impl IMAVControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVControl").unwrap(), alloc) })
    }
}
impl INSObject for IMAVControl {}
impl PNSObject for IMAVControl {}
impl std::convert::TryFrom<NSObject> for IMAVControl {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IMAVControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMAVControl").unwrap()) };
        if is_kind_of {
            Ok(IMAVControl(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IMAVControl")
        }
    }
}
impl IIMAVControl for IMAVControl {}
pub trait IIMAVControl: Sized + std::ops::Deref {
    unsafe fn target(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, anObject: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : anObject)
    }
    unsafe fn action(&self) -> objc2::runtime::Sel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn setAction_(&self, aSelector: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAction : aSelector)
    }
    unsafe fn tag(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tag)
    }
    unsafe fn setTag_(&self, anInt: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTag : anInt)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : flag)
    }
    unsafe fn integerValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, integerValue)
    }
    unsafe fn setIntegerValue_(&self, anInteger: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntegerValue : anInteger)
    }
    unsafe fn intValue(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intValue)
    }
    unsafe fn setIntValue_(&self, anInt: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntValue : anInt)
    }
    unsafe fn floatValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatValue)
    }
    unsafe fn setFloatValue_(&self, aFloat: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatValue : aFloat)
    }
    unsafe fn doubleValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doubleValue)
    }
    unsafe fn setDoubleValue_(&self, aDouble: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleValue : aDouble)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMAVButton(pub id);
impl std::ops::Deref for IMAVButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMAVButton {}
impl IMAVButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVButton").unwrap(), alloc) })
    }
}
impl IIMAVControl for IMAVButton {}
impl From<IMAVButton> for IMAVControl {
    fn from(child: IMAVButton) -> IMAVControl {
        IMAVControl(child.0)
    }
}
impl std::convert::TryFrom<IMAVControl> for IMAVButton {
    type Error = &'static str;
    fn try_from(parent: IMAVControl) -> Result<IMAVButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMAVButton").unwrap()) };
        if is_kind_of {
            Ok(IMAVButton(parent.0))
        } else {
            Err("This IMAVControl cannot be downcasted to IMAVButton")
        }
    }
}
impl INSObject for IMAVButton {}
impl PNSObject for IMAVButton {}
impl IIMAVButton for IMAVButton {}
pub trait IIMAVButton: Sized + std::ops::Deref {
    unsafe fn state(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn setState_(&self, value: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setState : value)
    }
}
impl IMAVButton_StandardControls for IMAVButton {}
pub trait IMAVButton_StandardControls: Sized + std::ops::Deref {
    unsafe fn playPauseButton() -> IMAVButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVButton").unwrap(), playPauseButton)
    }
    unsafe fn forwardButton() -> IMAVButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVButton").unwrap(), forwardButton)
    }
    unsafe fn backwardButton() -> IMAVButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVButton").unwrap(), backwardButton)
    }
    unsafe fn muteButton() -> IMAVButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVButton").unwrap(), muteButton)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMAVSlider(pub id);
impl std::ops::Deref for IMAVSlider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMAVSlider {}
impl IMAVSlider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVSlider").unwrap(), alloc) })
    }
}
impl IIMAVControl for IMAVSlider {}
impl std::convert::TryFrom<IMAVControl> for IMAVSlider {
    type Error = &'static str;
    fn try_from(parent: IMAVControl) -> Result<IMAVSlider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMAVSlider").unwrap()) };
        if is_kind_of {
            Ok(IMAVSlider(parent.0))
        } else {
            Err("This IMAVControl cannot be downcasted to IMAVSlider")
        }
    }
}
impl INSObject for IMAVSlider {}
impl PNSObject for IMAVSlider {}
impl IIMAVSlider for IMAVSlider {}
pub trait IIMAVSlider: Sized + std::ops::Deref {
    unsafe fn minValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minValue)
    }
    unsafe fn setMinValue_(&self, aDouble: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinValue : aDouble)
    }
    unsafe fn maxValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxValue)
    }
    unsafe fn setMaxValue_(&self, aDouble: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxValue : aDouble)
    }
}
impl IMAVSlider_StandardControls for IMAVSlider {}
pub trait IMAVSlider_StandardControls: Sized + std::ops::Deref {
    unsafe fn timeSlider() -> IMAVSlider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVSlider").unwrap(), timeSlider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMAVControlBar(pub id);
impl std::ops::Deref for IMAVControlBar {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMAVControlBar {}
impl IMAVControlBar {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVControlBar").unwrap(), alloc) })
    }
}
impl INSObject for IMAVControlBar {}
impl PNSObject for IMAVControlBar {}
impl std::convert::TryFrom<NSObject> for IMAVControlBar {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IMAVControlBar, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMAVControlBar").unwrap()) };
        if is_kind_of {
            Ok(IMAVControlBar(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IMAVControlBar")
        }
    }
}
impl IIMAVControlBar for IMAVControlBar {}
pub trait IIMAVControlBar: Sized + std::ops::Deref {
    unsafe fn controls(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controls)
    }
    unsafe fn addControl_(&self, control: IMAVControl)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addControl : control)
    }
    unsafe fn removeControl_(&self, control: IMAVControl)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeControl : control)
    }
    unsafe fn removeAllControls(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllControls)
    }
}
pub type IMServiceStatus = NSUInteger;
pub type IMPersonStatus = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMService(pub id);
impl std::ops::Deref for IMService {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMService {}
impl IMService {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMService").unwrap(), alloc) })
    }
}
impl INSObject for IMService {}
impl PNSObject for IMService {}
impl std::convert::TryFrom<NSObject> for IMService {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IMService, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMService").unwrap()) };
        if is_kind_of {
            Ok(IMService(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IMService")
        }
    }
}
impl IIMService for IMService {}
pub trait IIMService: Sized + std::ops::Deref {
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn localizedShortName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedShortName)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn status(&self) -> IMServiceStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn infoForScreenName_(&self, screenName: NSString) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, infoForScreenName : screenName)
    }
    unsafe fn infoForAllScreenNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, infoForAllScreenNames)
    }
    unsafe fn infoForPreferredScreenNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, infoForPreferredScreenNames)
    }
    unsafe fn peopleWithScreenName_(&self, screenName: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peopleWithScreenName : screenName)
    }
    unsafe fn screenNamesForPerson_(&self, person: ABPerson) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, screenNamesForPerson : person)
    }
    unsafe fn imageNameForStatus_(status: IMPersonStatus) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMService").unwrap(), imageNameForStatus : status)
    }
    unsafe fn allServices() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMService").unwrap(), allServices)
    }
    unsafe fn serviceWithName_(name: NSString) -> IMService
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMService").unwrap(), serviceWithName : name)
    }
    unsafe fn notificationCenter() -> NSNotificationCenter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMService").unwrap(), notificationCenter)
    }
    unsafe fn myStatus() -> IMPersonStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMService").unwrap(), myStatus)
    }
    unsafe fn myIdleTime() -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMService").unwrap(), myIdleTime)
    }
}
impl IMService_NSDeprecatedMethods for IMService {}
pub trait IMService_NSDeprecatedMethods: Sized + std::ops::Deref {
    unsafe fn imageURLForStatus_(status: IMPersonStatus) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMService").unwrap(), imageURLForStatus : status)
    }
}
pub type IMAVManagerState = NSUInteger;
pub type IMVideoOptimizationOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMAVManager(pub id);
impl std::ops::Deref for IMAVManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMAVManager {}
impl IMAVManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVManager").unwrap(), alloc) })
    }
}
impl INSObject for IMAVManager {}
impl PNSObject for IMAVManager {}
impl std::convert::TryFrom<NSObject> for IMAVManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IMAVManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMAVManager").unwrap()) };
        if is_kind_of {
            Ok(IMAVManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IMAVManager")
        }
    }
}
impl IIMAVManager for IMAVManager {}
pub trait IIMAVManager: Sized + std::ops::Deref {
    unsafe fn state(&self) -> IMAVManagerState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn URLToShare(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLToShare)
    }
    unsafe fn setVideoDataSource_(&self, dataSource: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoDataSource : dataSource)
    }
    unsafe fn videoDataSource(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoDataSource)
    }
    unsafe fn setVideoOptimizationOptions_(&self, options: IMVideoOptimizationOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoOptimizationOptions : options)
    }
    unsafe fn videoOptimizationOptions(&self) -> IMVideoOptimizationOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoOptimizationOptions)
    }
    unsafe fn setNumberOfAudioChannels_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfAudioChannels : count)
    }
    unsafe fn numberOfAudioChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfAudioChannels)
    }
    unsafe fn audioDeviceUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioDeviceUID)
    }
    unsafe fn audioDeviceChannels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioDeviceChannels)
    }
    unsafe fn controlBar(&self) -> IMAVControlBar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlBar)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn sharedAVManager() -> IMAVManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IMAVManager").unwrap(), sharedAVManager)
    }
}
pub trait NSObject_IMVideoDataSource: Sized + std::ops::Deref {
    unsafe fn getPixelBufferPixelFormat_(&self, pixelFormatOut: *mut OSType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPixelBufferPixelFormat : pixelFormatOut)
    }
    unsafe fn renderIntoPixelBuffer_forTime_(
        &self,
        buffer: CVPixelBufferRef,
        timeStamp: *mut CVTimeStamp,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderIntoPixelBuffer : buffer, forTime : timeStamp)
    }
    unsafe fn getOpenGLBufferContext_pixelFormat_(
        &self,
        contextOut: *mut CGLContextObj,
        pixelFormatOut: *mut CGLPixelFormatObj,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getOpenGLBufferContext : contextOut, pixelFormat : pixelFormatOut)
    }
    unsafe fn renderIntoOpenGLBuffer_onScreen_forTime_(
        &self,
        buffer: CVOpenGLBufferRef,
        screenInOut: *mut ::std::os::raw::c_int,
        timeStamp: *mut CVTimeStamp,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderIntoOpenGLBuffer : buffer, onScreen : screenInOut, forTime : timeStamp)
    }
}
unsafe extern "C" {
    pub static IMServiceStatusChangedNotification: NSString;
}
unsafe extern "C" {
    pub static IMMyStatusChangedNotification: NSString;
}
unsafe extern "C" {
    pub static IMPersonStatusChangedNotification: NSString;
}
unsafe extern "C" {
    pub static IMPersonInfoChangedNotification: NSString;
}
unsafe extern "C" {
    pub static IMStatusImagesChangedAppearanceNotification: NSString;
}
unsafe extern "C" {
    pub fn IMComparePersonStatus(
        status: IMPersonStatus,
        compareTo: IMPersonStatus,
    ) -> NSComparisonResult;
}
unsafe extern "C" {
    pub static mut IMPersonServiceNameKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonScreenNameKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonStatusKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonStatusMessageKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonIdleSinceKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonFirstNameKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonLastNameKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonEmailKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonPictureDataKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonAVBusyKey: NSString;
}
unsafe extern "C" {
    pub static mut IMPersonCapabilitiesKey: NSString;
}
unsafe extern "C" {
    pub static IMCapabilityText: NSString;
}
unsafe extern "C" {
    pub static IMCapabilityDirectIM: NSString;
}
unsafe extern "C" {
    pub static IMCapabilityFileTransfer: NSString;
}
unsafe extern "C" {
    pub static IMCapabilityFileSharing: NSString;
}
unsafe extern "C" {
    pub static IMCapabilityAudioConference: NSString;
}
unsafe extern "C" {
    pub static IMCapabilityVideoConference: NSString;
}
unsafe extern "C" {
    pub static IMAVManagerStateChangedNotification: NSString;
}
unsafe extern "C" {
    pub static IMAVManagerURLToShareChangedNotification: NSString;
}

unsafe impl objc2::encode::RefEncode for IMAVControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMAVControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IMAVButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMAVButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IMAVSlider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMAVSlider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IMAVControlBar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMAVControlBar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IMService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IMAVManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMAVManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
