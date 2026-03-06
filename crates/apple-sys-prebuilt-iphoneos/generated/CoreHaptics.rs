#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::AudioToolbox::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CHHapticEventParameterID = NSString;
pub type CHHapticDynamicParameterID = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CHHapticEventParameter(pub id);
impl std::ops::Deref for CHHapticEventParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CHHapticEventParameter {}
impl CHHapticEventParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CHHapticEventParameter").unwrap(), alloc) })
    }
}
impl INSObject for CHHapticEventParameter {}
impl PNSObject for CHHapticEventParameter {}
impl std::convert::TryFrom<NSObject> for CHHapticEventParameter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CHHapticEventParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CHHapticEventParameter").unwrap()) };
        if is_kind_of {
            Ok(CHHapticEventParameter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CHHapticEventParameter")
        }
    }
}
impl ICHHapticEventParameter for CHHapticEventParameter {}
pub trait ICHHapticEventParameter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithParameterID_value_(&self, parameterID: NSString, value: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithParameterID : parameterID, value : value)
    }
    unsafe fn parameterID(&self) -> CHHapticEventParameterID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterID)
    }
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CHHapticDynamicParameter(pub id);
impl std::ops::Deref for CHHapticDynamicParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CHHapticDynamicParameter {}
impl CHHapticDynamicParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CHHapticDynamicParameter").unwrap(), alloc) })
    }
}
impl INSObject for CHHapticDynamicParameter {}
impl PNSObject for CHHapticDynamicParameter {}
impl std::convert::TryFrom<NSObject> for CHHapticDynamicParameter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CHHapticDynamicParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CHHapticDynamicParameter").unwrap()) };
        if is_kind_of {
            Ok(CHHapticDynamicParameter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CHHapticDynamicParameter")
        }
    }
}
impl ICHHapticDynamicParameter for CHHapticDynamicParameter {}
pub trait ICHHapticDynamicParameter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithParameterID_value_relativeTime_(
        &self,
        parameterID: NSString,
        value: f32,
        time: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithParameterID : parameterID, value : value, relativeTime : time)
    }
    unsafe fn parameterID(&self) -> CHHapticDynamicParameterID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterID)
    }
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn relativeTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeTime)
    }
    unsafe fn setRelativeTime_(&self, relativeTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelativeTime : relativeTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CHHapticParameterCurveControlPoint(pub id);
impl std::ops::Deref for CHHapticParameterCurveControlPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CHHapticParameterCurveControlPoint {}
impl CHHapticParameterCurveControlPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CHHapticParameterCurveControlPoint").unwrap(), alloc) })
    }
}
impl INSObject for CHHapticParameterCurveControlPoint {}
impl PNSObject for CHHapticParameterCurveControlPoint {}
impl std::convert::TryFrom<NSObject> for CHHapticParameterCurveControlPoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CHHapticParameterCurveControlPoint, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CHHapticParameterCurveControlPoint").unwrap())
        };
        if is_kind_of {
            Ok(CHHapticParameterCurveControlPoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CHHapticParameterCurveControlPoint")
        }
    }
}
impl ICHHapticParameterCurveControlPoint for CHHapticParameterCurveControlPoint {}
pub trait ICHHapticParameterCurveControlPoint: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRelativeTime_value_(&self, time: NSTimeInterval, value: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRelativeTime : time, value : value)
    }
    unsafe fn relativeTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeTime)
    }
    unsafe fn setRelativeTime_(&self, relativeTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelativeTime : relativeTime)
    }
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CHHapticParameterCurve(pub id);
impl std::ops::Deref for CHHapticParameterCurve {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CHHapticParameterCurve {}
impl CHHapticParameterCurve {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CHHapticParameterCurve").unwrap(), alloc) })
    }
}
impl INSObject for CHHapticParameterCurve {}
impl PNSObject for CHHapticParameterCurve {}
impl std::convert::TryFrom<NSObject> for CHHapticParameterCurve {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CHHapticParameterCurve, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CHHapticParameterCurve").unwrap()) };
        if is_kind_of {
            Ok(CHHapticParameterCurve(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CHHapticParameterCurve")
        }
    }
}
impl ICHHapticParameterCurve for CHHapticParameterCurve {}
pub trait ICHHapticParameterCurve: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithParameterID_controlPoints_relativeTime_(
        &self,
        parameterID: NSString,
        controlPoints: NSArray,
        relativeTime: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithParameterID : parameterID, controlPoints : controlPoints, relativeTime : relativeTime)
    }
    unsafe fn parameterID(&self) -> CHHapticDynamicParameterID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterID)
    }
    unsafe fn relativeTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeTime)
    }
    unsafe fn setRelativeTime_(&self, relativeTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelativeTime : relativeTime)
    }
    unsafe fn controlPoints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPoints)
    }
}
pub type CHHapticEventType = NSString;
pub type CHHapticAudioResourceID = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CHHapticEvent(pub id);
impl std::ops::Deref for CHHapticEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CHHapticEvent {}
impl CHHapticEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CHHapticEvent").unwrap(), alloc) })
    }
}
impl INSObject for CHHapticEvent {}
impl PNSObject for CHHapticEvent {}
impl std::convert::TryFrom<NSObject> for CHHapticEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CHHapticEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CHHapticEvent").unwrap()) };
        if is_kind_of {
            Ok(CHHapticEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CHHapticEvent")
        }
    }
}
impl ICHHapticEvent for CHHapticEvent {}
pub trait ICHHapticEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEventType_parameters_relativeTime_(
        &self,
        type_: NSString,
        eventParams: NSArray,
        time: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEventType : type_, parameters : eventParams, relativeTime : time)
    }
    unsafe fn initWithEventType_parameters_relativeTime_duration_(
        &self,
        type_: NSString,
        eventParams: NSArray,
        time: NSTimeInterval,
        duration: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEventType : type_, parameters : eventParams, relativeTime : time, duration : duration)
    }
    unsafe fn initWithAudioResourceID_parameters_relativeTime_(
        &self,
        resID: CHHapticAudioResourceID,
        eventParams: NSArray,
        time: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioResourceID : resID, parameters : eventParams, relativeTime : time)
    }
    unsafe fn initWithAudioResourceID_parameters_relativeTime_duration_(
        &self,
        resID: CHHapticAudioResourceID,
        eventParams: NSArray,
        time: NSTimeInterval,
        duration: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioResourceID : resID, parameters : eventParams, relativeTime : time, duration : duration)
    }
    unsafe fn type_(&self) -> CHHapticEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn eventParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventParameters)
    }
    unsafe fn relativeTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeTime)
    }
    unsafe fn setRelativeTime_(&self, relativeTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelativeTime : relativeTime)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn setDuration_(&self, duration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuration : duration)
    }
}
pub trait PCHHapticParameterAttributes: Sized + std::ops::Deref {
    unsafe fn minValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minValue)
    }
    unsafe fn maxValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxValue)
    }
    unsafe fn defaultValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultValue)
    }
}
pub trait PCHHapticDeviceCapability: Sized + std::ops::Deref {
    unsafe fn attributesForEventParameter_eventType_error_(
        &self,
        inParameter: NSString,
        type_: NSString,
        outError: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributesForEventParameter : inParameter, eventType : type_, error : outError)
    }
    unsafe fn attributesForDynamicParameter_error_(
        &self,
        inParameter: NSString,
        outError: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributesForDynamicParameter : inParameter, error : outError)
    }
    unsafe fn supportsHaptics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsHaptics)
    }
    unsafe fn supportsAudio(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsAudio)
    }
}
pub type CHHapticErrorCode = NSInteger;
pub trait PCHHapticPatternPlayer: Sized + std::ops::Deref {
    unsafe fn startAtTime_error_(&self, time: NSTimeInterval, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAtTime : time, error : outError)
    }
    unsafe fn stopAtTime_error_(&self, time: NSTimeInterval, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopAtTime : time, error : outError)
    }
    unsafe fn sendParameters_atTime_error_(
        &self,
        parameters: NSArray,
        time: NSTimeInterval,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendParameters : parameters, atTime : time, error : outError)
    }
    unsafe fn scheduleParameterCurve_atTime_error_(
        &self,
        parameterCurve: CHHapticParameterCurve,
        time: NSTimeInterval,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleParameterCurve : parameterCurve, atTime : time, error : outError)
    }
    unsafe fn cancelAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelAndReturnError : outError)
    }
    unsafe fn isMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMuted)
    }
    unsafe fn setIsMuted_(&self, isMuted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsMuted : isMuted)
    }
}
pub type CHHapticAdvancedPatternPlayerCompletionHandler = *mut ::std::os::raw::c_void;
pub trait PCHHapticAdvancedPatternPlayer: Sized + std::ops::Deref {
    unsafe fn pauseAtTime_error_(&self, time: NSTimeInterval, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseAtTime : time, error : outError)
    }
    unsafe fn resumeAtTime_error_(&self, time: NSTimeInterval, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeAtTime : time, error : outError)
    }
    unsafe fn seekToOffset_error_(&self, offsetTime: NSTimeInterval, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, seekToOffset : offsetTime, error : outError)
    }
    unsafe fn loopEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loopEnabled)
    }
    unsafe fn setLoopEnabled_(&self, loopEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoopEnabled : loopEnabled)
    }
    unsafe fn loopEnd(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loopEnd)
    }
    unsafe fn setLoopEnd_(&self, loopEnd: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoopEnd : loopEnd)
    }
    unsafe fn playbackRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackRate)
    }
    unsafe fn setPlaybackRate_(&self, playbackRate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaybackRate : playbackRate)
    }
    unsafe fn isMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMuted)
    }
    unsafe fn setIsMuted_(&self, isMuted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsMuted : isMuted)
    }
    unsafe fn completionHandler(&self) -> CHHapticAdvancedPatternPlayerCompletionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionHandler)
    }
    unsafe fn setCompletionHandler_(
        &self,
        completionHandler: CHHapticAdvancedPatternPlayerCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionHandler : completionHandler)
    }
}
pub type CHHapticCompletionHandler = *mut ::std::os::raw::c_void;
pub type CHHapticEngineFinishedAction = NSInteger;
pub type CHHapticEngineFinishedHandler = *mut ::std::os::raw::c_void;
pub type CHHapticEngineStoppedReason = NSInteger;
pub type CHHapticEngineStoppedHandler = *mut ::std::os::raw::c_void;
pub type CHHapticEngineResetHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CHHapticEngine(pub id);
impl std::ops::Deref for CHHapticEngine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CHHapticEngine {}
impl CHHapticEngine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CHHapticEngine").unwrap(), alloc) })
    }
}
impl INSObject for CHHapticEngine {}
impl PNSObject for CHHapticEngine {}
impl std::convert::TryFrom<NSObject> for CHHapticEngine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CHHapticEngine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CHHapticEngine").unwrap()) };
        if is_kind_of {
            Ok(CHHapticEngine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CHHapticEngine")
        }
    }
}
impl ICHHapticEngine for CHHapticEngine {}
pub trait ICHHapticEngine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initAndReturnError_(&self, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initAndReturnError : error)
    }
    unsafe fn initWithAudioSession_error_(
        &self,
        audioSession: AVAudioSession,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioSession : audioSession, error : error)
    }
    unsafe fn startWithCompletionHandler_(&self, completionHandler: CHHapticCompletionHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithCompletionHandler : completionHandler)
    }
    unsafe fn startAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAndReturnError : outError)
    }
    unsafe fn stopWithCompletionHandler_(&self, completionHandler: CHHapticCompletionHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopWithCompletionHandler : completionHandler)
    }
    unsafe fn notifyWhenPlayersFinished_(&self, finishedHandler: CHHapticEngineFinishedHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notifyWhenPlayersFinished : finishedHandler)
    }
    unsafe fn createPlayerWithPattern_error_(
        &self,
        pattern: CHHapticPattern,
        outError: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createPlayerWithPattern : pattern, error : outError)
    }
    unsafe fn createAdvancedPlayerWithPattern_error_(
        &self,
        pattern: CHHapticPattern,
        outError: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createAdvancedPlayerWithPattern : pattern, error : outError)
    }
    unsafe fn registerAudioResource_options_error_(
        &self,
        resourceURL: NSURL,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> CHHapticAudioResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerAudioResource : resourceURL, options : options, error : outError)
    }
    unsafe fn unregisterAudioResource_error_(
        &self,
        resourceID: CHHapticAudioResourceID,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterAudioResource : resourceID, error : outError)
    }
    unsafe fn playPatternFromURL_error_(&self, fileURL: NSURL, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playPatternFromURL : fileURL, error : outError)
    }
    unsafe fn playPatternFromData_error_(&self, data: NSData, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playPatternFromData : data, error : outError)
    }
    unsafe fn currentTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentTime)
    }
    unsafe fn stoppedHandler(&self) -> CHHapticEngineStoppedHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stoppedHandler)
    }
    unsafe fn setStoppedHandler_(&self, stoppedHandler: CHHapticEngineStoppedHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStoppedHandler : stoppedHandler)
    }
    unsafe fn resetHandler(&self) -> CHHapticEngineResetHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetHandler)
    }
    unsafe fn setResetHandler_(&self, resetHandler: CHHapticEngineResetHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResetHandler : resetHandler)
    }
    unsafe fn playsHapticsOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playsHapticsOnly)
    }
    unsafe fn setPlaysHapticsOnly_(&self, playsHapticsOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaysHapticsOnly : playsHapticsOnly)
    }
    unsafe fn playsAudioOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playsAudioOnly)
    }
    unsafe fn setPlaysAudioOnly_(&self, playsAudioOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaysAudioOnly : playsAudioOnly)
    }
    unsafe fn isMutedForAudio(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMutedForAudio)
    }
    unsafe fn setIsMutedForAudio_(&self, isMutedForAudio: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsMutedForAudio : isMutedForAudio)
    }
    unsafe fn isMutedForHaptics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMutedForHaptics)
    }
    unsafe fn setIsMutedForHaptics_(&self, isMutedForHaptics: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsMutedForHaptics : isMutedForHaptics)
    }
    unsafe fn isAutoShutdownEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutoShutdownEnabled)
    }
    unsafe fn setAutoShutdownEnabled_(&self, autoShutdownEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoShutdownEnabled : autoShutdownEnabled)
    }
    unsafe fn intendedSpatialExperience(&self) -> CASpatialAudioExperience
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intendedSpatialExperience)
    }
    unsafe fn setIntendedSpatialExperience_(
        &self,
        intendedSpatialExperience: CASpatialAudioExperience,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntendedSpatialExperience : intendedSpatialExperience)
    }
    unsafe fn capabilitiesForHardware() -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CHHapticEngine").unwrap(), capabilitiesForHardware)
    }
}
pub type CHHapticAudioResourceKey = NSString;
pub type CHHapticPatternKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CHHapticPattern(pub id);
impl std::ops::Deref for CHHapticPattern {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CHHapticPattern {}
impl CHHapticPattern {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CHHapticPattern").unwrap(), alloc) })
    }
}
impl INSObject for CHHapticPattern {}
impl PNSObject for CHHapticPattern {}
impl std::convert::TryFrom<NSObject> for CHHapticPattern {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CHHapticPattern, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CHHapticPattern").unwrap()) };
        if is_kind_of {
            Ok(CHHapticPattern(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CHHapticPattern")
        }
    }
}
impl ICHHapticPattern for CHHapticPattern {}
pub trait ICHHapticPattern: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEvents_parameters_error_(
        &self,
        events: NSArray,
        parameters: NSArray,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEvents : events, parameters : parameters, error : outError)
    }
    unsafe fn initWithEvents_parameterCurves_error_(
        &self,
        events: NSArray,
        parameterCurves: NSArray,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEvents : events, parameterCurves : parameterCurves, error : outError)
    }
    unsafe fn initWithDictionary_error_(
        &self,
        patternDict: NSDictionary,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : patternDict, error : outError)
    }
    unsafe fn initWithContentsOfURL_error_(
        &self,
        ahapURL: NSURL,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : ahapURL, error : outError)
    }
    unsafe fn exportDictionaryAndReturnError_(&self, outError: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exportDictionaryAndReturnError : outError)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDHapticIntensity: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDHapticSharpness: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDAttackTime: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDDecayTime: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDReleaseTime: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDSustained: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDAudioVolume: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDAudioPitch: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDAudioPan: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventParameterIDAudioBrightness: CHHapticEventParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDHapticIntensityControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDHapticSharpnessControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDHapticAttackTimeControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDHapticDecayTimeControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDHapticReleaseTimeControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDAudioVolumeControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDAudioPanControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDAudioBrightnessControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDAudioPitchControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDAudioAttackTimeControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDAudioDecayTimeControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticDynamicParameterIDAudioReleaseTimeControl: CHHapticDynamicParameterID;
}
unsafe extern "C" {
    pub static mut CHHapticEventTypeHapticTransient: CHHapticEventType;
}
unsafe extern "C" {
    pub static mut CHHapticEventTypeHapticContinuous: CHHapticEventType;
}
unsafe extern "C" {
    pub static mut CHHapticEventTypeAudioContinuous: CHHapticEventType;
}
unsafe extern "C" {
    pub static mut CHHapticEventTypeAudioCustom: CHHapticEventType;
}
unsafe extern "C" {
    pub static mut CoreHapticsErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static mut CHHapticAudioResourceKeyUseVolumeEnvelope: CHHapticAudioResourceKey;
}
unsafe extern "C" {
    pub static mut CHHapticAudioResourceKeyLoopEnabled: CHHapticAudioResourceKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyVersion: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyPattern: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyEvent: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyEventType: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyTime: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyEventDuration: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyEventWaveformPath: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyEventParameters: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyEventWaveformUseVolumeEnvelope: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyEventWaveformLoopEnabled: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyParameter: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyParameterID: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyParameterValue: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyParameterCurve: CHHapticPatternKey;
}
unsafe extern "C" {
    pub static mut CHHapticPatternKeyParameterCurveControlPoints: CHHapticPatternKey;
}

unsafe impl objc2::encode::RefEncode for CHHapticEventParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CHHapticEventParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CHHapticDynamicParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CHHapticDynamicParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CHHapticParameterCurveControlPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CHHapticParameterCurveControlPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CHHapticParameterCurve {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CHHapticParameterCurve {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CHHapticEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CHHapticEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CHHapticEngine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CHHapticEngine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CHHapticPattern {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CHHapticPattern {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
