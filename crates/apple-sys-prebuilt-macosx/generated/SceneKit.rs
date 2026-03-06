#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::GLKit::*;
#[allow(unused_imports)]
use crate::JavaScriptCore::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::ModelIO::*;
#[allow(unused_imports)]
use crate::OpenGL::*;
#[allow(unused_imports)]
use crate::QuartzCore::*;
#[allow(unused_imports)]
use crate::SpriteKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SCNActionTimingMode = NSInteger;
pub type SCNColorMask = NSInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCNVector3 {
    pub x: CGFloat,
    pub y: CGFloat,
    pub z: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCNVector4 {
    pub x: CGFloat,
    pub y: CGFloat,
    pub z: CGFloat,
    pub w: CGFloat,
}
pub type SCNQuaternion = SCNVector4;
pub type SCNMatrix4 = CATransform3D;
pub trait NSValue_SceneKitAdditions: Sized + std::ops::Deref {
    unsafe fn SCNVector3Value(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SCNVector3Value)
    }
    unsafe fn SCNVector4Value(&self) -> SCNVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SCNVector4Value)
    }
    unsafe fn SCNMatrix4Value(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SCNMatrix4Value)
    }
    unsafe fn valueWithSCNVector3_(v: SCNVector3) -> NSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSValue").unwrap(), valueWithSCNVector3 : v)
    }
    unsafe fn valueWithSCNVector4_(v: SCNVector4) -> NSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSValue").unwrap(), valueWithSCNVector4 : v)
    }
    unsafe fn valueWithSCNMatrix4_(v: SCNMatrix4) -> NSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSValue").unwrap(), valueWithSCNMatrix4 : v)
    }
}
pub trait PSCNAnimation: Sized + std::ops::Deref {}
pub trait CAAnimation_SCNAnimation: Sized + std::ops::Deref {}
pub trait CAAnimation_SceneKitAdditions: Sized + std::ops::Deref {
    unsafe fn usesSceneTimeBase(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesSceneTimeBase)
    }
    unsafe fn setUsesSceneTimeBase_(&self, usesSceneTimeBase: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesSceneTimeBase : usesSceneTimeBase)
    }
    unsafe fn fadeInDuration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fadeInDuration)
    }
    unsafe fn setFadeInDuration_(&self, fadeInDuration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFadeInDuration : fadeInDuration)
    }
    unsafe fn fadeOutDuration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fadeOutDuration)
    }
    unsafe fn setFadeOutDuration_(&self, fadeOutDuration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFadeOutDuration : fadeOutDuration)
    }
    unsafe fn animationEvents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationEvents)
    }
    unsafe fn setAnimationEvents_(&self, animationEvents: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimationEvents : animationEvents)
    }
    unsafe fn animationWithSCNAnimation_(animation: SCNAnimation) -> CAAnimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAAnimation").unwrap(), animationWithSCNAnimation : animation)
    }
}
pub type SCNAnimationDidStartBlock = *mut ::std::os::raw::c_void;
pub type SCNAnimationDidStopBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNTimingFunction(pub id);
impl std::ops::Deref for SCNTimingFunction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNTimingFunction {}
impl SCNTimingFunction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTimingFunction").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SCNTimingFunction {}
impl INSObject for SCNTimingFunction {}
impl PNSObject for SCNTimingFunction {}
impl std::convert::TryFrom<NSObject> for SCNTimingFunction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNTimingFunction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNTimingFunction").unwrap()) };
        if is_kind_of {
            Ok(SCNTimingFunction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNTimingFunction")
        }
    }
}
impl ISCNTimingFunction for SCNTimingFunction {}
pub trait ISCNTimingFunction: Sized + std::ops::Deref {
    unsafe fn functionWithTimingMode_(timingMode: SCNActionTimingMode) -> SCNTimingFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTimingFunction").unwrap(), functionWithTimingMode : timingMode)
    }
    unsafe fn functionWithCAMediaTimingFunction_(
        caTimingFunction: CAMediaTimingFunction,
    ) -> SCNTimingFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTimingFunction").unwrap(), functionWithCAMediaTimingFunction : caTimingFunction)
    }
}
pub trait PSCNAnimatable: Sized + std::ops::Deref {
    unsafe fn addAnimation_forKey_(&self, animation: *mut u64, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAnimation : animation, forKey : key)
    }
    unsafe fn addAnimationPlayer_forKey_(&self, player: SCNAnimationPlayer, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAnimationPlayer : player, forKey : key)
    }
    unsafe fn removeAllAnimations(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllAnimations)
    }
    unsafe fn removeAllAnimationsWithBlendOutDuration_(&self, duration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAllAnimationsWithBlendOutDuration : duration)
    }
    unsafe fn removeAnimationForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAnimationForKey : key)
    }
    unsafe fn removeAnimationForKey_blendOutDuration_(&self, key: NSString, duration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAnimationForKey : key, blendOutDuration : duration)
    }
    unsafe fn animationPlayerForKey_(&self, key: NSString) -> SCNAnimationPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animationPlayerForKey : key)
    }
    unsafe fn removeAnimationForKey_fadeOutDuration_(&self, key: NSString, duration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAnimationForKey : key, fadeOutDuration : duration)
    }
    unsafe fn animationForKey_(&self, key: NSString) -> CAAnimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animationForKey : key)
    }
    unsafe fn pauseAnimationForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseAnimationForKey : key)
    }
    unsafe fn resumeAnimationForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeAnimationForKey : key)
    }
    unsafe fn setSpeed_forAnimationKey_(&self, speed: CGFloat, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed, forAnimationKey : key)
    }
    unsafe fn isAnimationForKeyPaused_(&self, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isAnimationForKeyPaused : key)
    }
    unsafe fn animationKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationKeys)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNAnimation(pub id);
impl std::ops::Deref for SCNAnimation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNAnimation {}
impl SCNAnimation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAnimation").unwrap(), alloc) })
    }
}
impl PSCNAnimation for SCNAnimation {}
impl PNSCopying for SCNAnimation {}
impl PNSSecureCoding for SCNAnimation {}
impl INSObject for SCNAnimation {}
impl PNSObject for SCNAnimation {}
impl std::convert::TryFrom<NSObject> for SCNAnimation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNAnimation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNAnimation").unwrap()) };
        if is_kind_of {
            Ok(SCNAnimation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNAnimation")
        }
    }
}
impl ISCNAnimation for SCNAnimation {}
pub trait ISCNAnimation: Sized + std::ops::Deref {
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
    unsafe fn keyPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyPath)
    }
    unsafe fn setKeyPath_(&self, keyPath: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyPath : keyPath)
    }
    unsafe fn timingFunction(&self) -> SCNTimingFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timingFunction)
    }
    unsafe fn setTimingFunction_(&self, timingFunction: SCNTimingFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimingFunction : timingFunction)
    }
    unsafe fn blendInDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendInDuration)
    }
    unsafe fn setBlendInDuration_(&self, blendInDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendInDuration : blendInDuration)
    }
    unsafe fn blendOutDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendOutDuration)
    }
    unsafe fn setBlendOutDuration_(&self, blendOutDuration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendOutDuration : blendOutDuration)
    }
    unsafe fn isRemovedOnCompletion(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRemovedOnCompletion)
    }
    unsafe fn setRemovedOnCompletion_(&self, removedOnCompletion: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemovedOnCompletion : removedOnCompletion)
    }
    unsafe fn isAppliedOnCompletion(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAppliedOnCompletion)
    }
    unsafe fn setAppliedOnCompletion_(&self, appliedOnCompletion: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppliedOnCompletion : appliedOnCompletion)
    }
    unsafe fn repeatCount(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, repeatCount)
    }
    unsafe fn setRepeatCount_(&self, repeatCount: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRepeatCount : repeatCount)
    }
    unsafe fn autoreverses(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoreverses)
    }
    unsafe fn setAutoreverses_(&self, autoreverses: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoreverses : autoreverses)
    }
    unsafe fn startDelay(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDelay)
    }
    unsafe fn setStartDelay_(&self, startDelay: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartDelay : startDelay)
    }
    unsafe fn timeOffset(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeOffset)
    }
    unsafe fn setTimeOffset_(&self, timeOffset: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeOffset : timeOffset)
    }
    unsafe fn fillsForward(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillsForward)
    }
    unsafe fn setFillsForward_(&self, fillsForward: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillsForward : fillsForward)
    }
    unsafe fn fillsBackward(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillsBackward)
    }
    unsafe fn setFillsBackward_(&self, fillsBackward: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillsBackward : fillsBackward)
    }
    unsafe fn usesSceneTimeBase(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesSceneTimeBase)
    }
    unsafe fn setUsesSceneTimeBase_(&self, usesSceneTimeBase: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesSceneTimeBase : usesSceneTimeBase)
    }
    unsafe fn animationDidStart(&self) -> SCNAnimationDidStartBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationDidStart)
    }
    unsafe fn setAnimationDidStart_(&self, animationDidStart: SCNAnimationDidStartBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimationDidStart : animationDidStart)
    }
    unsafe fn animationDidStop(&self) -> SCNAnimationDidStopBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationDidStop)
    }
    unsafe fn setAnimationDidStop_(&self, animationDidStop: SCNAnimationDidStopBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimationDidStop : animationDidStop)
    }
    unsafe fn animationEvents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationEvents)
    }
    unsafe fn setAnimationEvents_(&self, animationEvents: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimationEvents : animationEvents)
    }
    unsafe fn isAdditive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAdditive)
    }
    unsafe fn setAdditive_(&self, additive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditive : additive)
    }
    unsafe fn isCumulative(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCumulative)
    }
    unsafe fn setCumulative_(&self, cumulative: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCumulative : cumulative)
    }
    unsafe fn animationWithContentsOfURL_(animationUrl: NSURL) -> SCNAnimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAnimation").unwrap(), animationWithContentsOfURL : animationUrl)
    }
    unsafe fn animationNamed_(animationName: NSString) -> SCNAnimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAnimation").unwrap(), animationNamed : animationName)
    }
    unsafe fn animationWithCAAnimation_(caAnimation: CAAnimation) -> SCNAnimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAnimation").unwrap(), animationWithCAAnimation : caAnimation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNAnimationPlayer(pub id);
impl std::ops::Deref for SCNAnimationPlayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNAnimationPlayer {}
impl SCNAnimationPlayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAnimationPlayer").unwrap(), alloc) })
    }
}
impl PSCNAnimatable for SCNAnimationPlayer {}
impl PNSCopying for SCNAnimationPlayer {}
impl PNSSecureCoding for SCNAnimationPlayer {}
impl INSObject for SCNAnimationPlayer {}
impl PNSObject for SCNAnimationPlayer {}
impl std::convert::TryFrom<NSObject> for SCNAnimationPlayer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNAnimationPlayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNAnimationPlayer").unwrap()) };
        if is_kind_of {
            Ok(SCNAnimationPlayer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNAnimationPlayer")
        }
    }
}
impl ISCNAnimationPlayer for SCNAnimationPlayer {}
pub trait ISCNAnimationPlayer: Sized + std::ops::Deref {
    unsafe fn play(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, play)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn stopWithBlendOutDuration_(&self, duration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopWithBlendOutDuration : duration)
    }
    unsafe fn animation(&self) -> SCNAnimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animation)
    }
    unsafe fn speed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn setSpeed_(&self, speed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed)
    }
    unsafe fn blendFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendFactor)
    }
    unsafe fn setBlendFactor_(&self, blendFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendFactor : blendFactor)
    }
    unsafe fn paused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paused)
    }
    unsafe fn setPaused_(&self, paused: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaused : paused)
    }
    unsafe fn animationPlayerWithAnimation_(animation: SCNAnimation) -> SCNAnimationPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAnimationPlayer").unwrap(), animationPlayerWithAnimation : animation)
    }
}
pub type SCNAnimationEventBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNAnimationEvent(pub id);
impl std::ops::Deref for SCNAnimationEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNAnimationEvent {}
impl SCNAnimationEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAnimationEvent").unwrap(), alloc) })
    }
}
impl INSObject for SCNAnimationEvent {}
impl PNSObject for SCNAnimationEvent {}
impl std::convert::TryFrom<NSObject> for SCNAnimationEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNAnimationEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNAnimationEvent").unwrap()) };
        if is_kind_of {
            Ok(SCNAnimationEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNAnimationEvent")
        }
    }
}
impl ISCNAnimationEvent for SCNAnimationEvent {}
pub trait ISCNAnimationEvent: Sized + std::ops::Deref {
    unsafe fn animationEventWithKeyTime_block_(
        time: CGFloat,
        eventBlock: SCNAnimationEventBlock,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAnimationEvent").unwrap(), animationEventWithKeyTime : time, block : eventBlock)
    }
}
pub trait PSCNBoundingVolume: Sized + std::ops::Deref {
    unsafe fn getBoundingBoxMin_max_(&self, min: *mut SCNVector3, max: *mut SCNVector3) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getBoundingBoxMin : min, max : max)
    }
    unsafe fn setBoundingBoxMin_max_(&self, min: *mut SCNVector3, max: *mut SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxMin : min, max : max)
    }
    unsafe fn getBoundingSphereCenter_radius_(
        &self,
        center: *mut SCNVector3,
        radius: *mut CGFloat,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getBoundingSphereCenter : center, radius : radius)
    }
}
pub type SCNHitTestSearchMode = NSInteger;
pub type SCNHitTestOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNHitTestResult(pub id);
impl std::ops::Deref for SCNHitTestResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNHitTestResult {}
impl SCNHitTestResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNHitTestResult").unwrap(), alloc) })
    }
}
impl INSObject for SCNHitTestResult {}
impl PNSObject for SCNHitTestResult {}
impl std::convert::TryFrom<NSObject> for SCNHitTestResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNHitTestResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNHitTestResult").unwrap()) };
        if is_kind_of {
            Ok(SCNHitTestResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNHitTestResult")
        }
    }
}
impl ISCNHitTestResult for SCNHitTestResult {}
pub trait ISCNHitTestResult: Sized + std::ops::Deref {
    unsafe fn textureCoordinatesWithMappingChannel_(&self, channel: NSInteger) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureCoordinatesWithMappingChannel : channel)
    }
    unsafe fn node(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, node)
    }
    unsafe fn geometryIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometryIndex)
    }
    unsafe fn faceIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, faceIndex)
    }
    unsafe fn localCoordinates(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localCoordinates)
    }
    unsafe fn worldCoordinates(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldCoordinates)
    }
    unsafe fn localNormal(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localNormal)
    }
    unsafe fn worldNormal(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldNormal)
    }
    unsafe fn modelTransform(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelTransform)
    }
    unsafe fn boneNode(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boneNode)
    }
}
impl SCNHitTestResult_SIMD for SCNHitTestResult {}
pub trait SCNHitTestResult_SIMD: Sized + std::ops::Deref {
    unsafe fn simdLocalCoordinates(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdLocalCoordinates)
    }
    unsafe fn simdWorldCoordinates(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdWorldCoordinates)
    }
    unsafe fn simdLocalNormal(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdLocalNormal)
    }
    unsafe fn simdWorldNormal(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdWorldNormal)
    }
}
pub type SCNAntialiasingMode = NSUInteger;
pub type SCNRenderingAPI = NSUInteger;
pub type SCNDebugOptions = NSUInteger;
pub trait PSCNSceneRenderer: Sized + std::ops::Deref {
    unsafe fn presentScene_withTransition_incomingPointOfView_completionHandler_(
        &self,
        scene: SCNScene,
        transition: SKTransition,
        pointOfView: SCNNode,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentScene : scene, withTransition : transition, incomingPointOfView : pointOfView, completionHandler : completionHandler)
    }
    unsafe fn hitTest_options_(&self, point: CGPoint, options: NSDictionary) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hitTest : point, options : options)
    }
    unsafe fn isNodeInsideFrustum_withPointOfView_(
        &self,
        node: SCNNode,
        pointOfView: SCNNode,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isNodeInsideFrustum : node, withPointOfView : pointOfView)
    }
    unsafe fn nodesInsideFrustumWithPointOfView_(&self, pointOfView: SCNNode) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodesInsideFrustumWithPointOfView : pointOfView)
    }
    unsafe fn projectPoint_(&self, point: SCNVector3) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, projectPoint : point)
    }
    unsafe fn unprojectPoint_(&self, point: SCNVector3) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unprojectPoint : point)
    }
    unsafe fn prepareObject_shouldAbortBlock_(
        &self,
        object: id,
        block: *mut ::std::os::raw::c_void,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareObject : object, shouldAbortBlock : block)
    }
    unsafe fn prepareObjects_withCompletionHandler_(
        &self,
        objects: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareObjects : objects, withCompletionHandler : completionHandler)
    }
    unsafe fn scene(&self) -> SCNScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scene)
    }
    unsafe fn setScene_(&self, scene: SCNScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScene : scene)
    }
    unsafe fn sceneTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sceneTime)
    }
    unsafe fn setSceneTime_(&self, sceneTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSceneTime : sceneTime)
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
    unsafe fn isPlaying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPlaying)
    }
    unsafe fn setPlaying_(&self, playing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaying : playing)
    }
    unsafe fn loops(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loops)
    }
    unsafe fn setLoops_(&self, loops: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoops : loops)
    }
    unsafe fn pointOfView(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfView)
    }
    unsafe fn setPointOfView_(&self, pointOfView: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfView : pointOfView)
    }
    unsafe fn autoenablesDefaultLighting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoenablesDefaultLighting)
    }
    unsafe fn setAutoenablesDefaultLighting_(&self, autoenablesDefaultLighting: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoenablesDefaultLighting : autoenablesDefaultLighting)
    }
    unsafe fn isJitteringEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isJitteringEnabled)
    }
    unsafe fn setJitteringEnabled_(&self, jitteringEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJitteringEnabled : jitteringEnabled)
    }
    unsafe fn isTemporalAntialiasingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTemporalAntialiasingEnabled)
    }
    unsafe fn setTemporalAntialiasingEnabled_(&self, temporalAntialiasingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTemporalAntialiasingEnabled : temporalAntialiasingEnabled)
    }
    unsafe fn showsStatistics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsStatistics)
    }
    unsafe fn setShowsStatistics_(&self, showsStatistics: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsStatistics : showsStatistics)
    }
    unsafe fn debugOptions(&self) -> SCNDebugOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, debugOptions)
    }
    unsafe fn setDebugOptions_(&self, debugOptions: SCNDebugOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDebugOptions : debugOptions)
    }
    unsafe fn overlaySKScene(&self) -> SKScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overlaySKScene)
    }
    unsafe fn setOverlaySKScene_(&self, overlaySKScene: SKScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverlaySKScene : overlaySKScene)
    }
    unsafe fn renderingAPI(&self) -> SCNRenderingAPI
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderingAPI)
    }
    unsafe fn workingColorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workingColorSpace)
    }
    unsafe fn context(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn currentRenderCommandEncoder(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentRenderCommandEncoder)
    }
    unsafe fn currentRenderPassDescriptor(&self) -> MTLRenderPassDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentRenderPassDescriptor)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn colorPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorPixelFormat)
    }
    unsafe fn depthPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthPixelFormat)
    }
    unsafe fn stencilPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilPixelFormat)
    }
    unsafe fn commandQueue(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandQueue)
    }
    unsafe fn audioEngine(&self) -> AVAudioEngine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioEngine)
    }
    unsafe fn audioEnvironmentNode(&self) -> AVAudioEnvironmentNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioEnvironmentNode)
    }
    unsafe fn audioListener(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioListener)
    }
    unsafe fn setAudioListener_(&self, audioListener: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioListener : audioListener)
    }
    unsafe fn currentViewport(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentViewport)
    }
    unsafe fn currentTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentTime)
    }
    unsafe fn setCurrentTime_(&self, currentTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentTime : currentTime)
    }
    unsafe fn usesReverseZ(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesReverseZ)
    }
    unsafe fn setUsesReverseZ_(&self, usesReverseZ: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesReverseZ : usesReverseZ)
    }
}
pub trait PSCNSceneRendererDelegate: Sized + std::ops::Deref {
    unsafe fn renderer_updateAtTime_(&self, renderer: *mut u64, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderer : renderer, updateAtTime : time)
    }
    unsafe fn renderer_didApplyAnimationsAtTime_(&self, renderer: *mut u64, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderer : renderer, didApplyAnimationsAtTime : time)
    }
    unsafe fn renderer_didSimulatePhysicsAtTime_(&self, renderer: *mut u64, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderer : renderer, didSimulatePhysicsAtTime : time)
    }
    unsafe fn renderer_didApplyConstraintsAtTime_(&self, renderer: *mut u64, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderer : renderer, didApplyConstraintsAtTime : time)
    }
    unsafe fn renderer_willRenderScene_atTime_(
        &self,
        renderer: *mut u64,
        scene: SCNScene,
        time: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderer : renderer, willRenderScene : scene, atTime : time)
    }
    unsafe fn renderer_didRenderScene_atTime_(
        &self,
        renderer: *mut u64,
        scene: SCNScene,
        time: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderer : renderer, didRenderScene : scene, atTime : time)
    }
}
pub type SCNShaderModifierEntryPoint = NSString;
pub type SCNBufferFrequency = NSInteger;
pub trait PSCNBufferStream: Sized + std::ops::Deref {
    unsafe fn writeBytes_length_(&self, bytes: *const ::std::os::raw::c_void, length: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeBytes : bytes, length : length)
    }
}
pub type SCNBufferBindingBlock = *mut ::std::os::raw::c_void;
pub type SCNBindingBlock = *mut ::std::os::raw::c_void;
pub trait PSCNShadable: Sized + std::ops::Deref {
    unsafe fn handleBindingOfSymbol_usingBlock_(&self, symbol: NSString, block: SCNBindingBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleBindingOfSymbol : symbol, usingBlock : block)
    }
    unsafe fn handleUnbindingOfSymbol_usingBlock_(&self, symbol: NSString, block: SCNBindingBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleUnbindingOfSymbol : symbol, usingBlock : block)
    }
    unsafe fn program(&self) -> SCNProgram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, program)
    }
    unsafe fn setProgram_(&self, program: SCNProgram)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgram : program)
    }
    unsafe fn shaderModifiers(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderModifiers)
    }
    unsafe fn setShaderModifiers_(&self, shaderModifiers: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShaderModifiers : shaderModifiers)
    }
    unsafe fn minimumLanguageVersion(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumLanguageVersion)
    }
    unsafe fn setMinimumLanguageVersion_(&self, minimumLanguageVersion: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumLanguageVersion : minimumLanguageVersion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNProgram(pub id);
impl std::ops::Deref for SCNProgram {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNProgram {}
impl SCNProgram {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNProgram").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNProgram {}
impl PNSSecureCoding for SCNProgram {}
impl INSObject for SCNProgram {}
impl PNSObject for SCNProgram {}
impl std::convert::TryFrom<NSObject> for SCNProgram {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNProgram, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNProgram").unwrap()) };
        if is_kind_of {
            Ok(SCNProgram(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNProgram")
        }
    }
}
impl ISCNProgram for SCNProgram {}
pub trait ISCNProgram: Sized + std::ops::Deref {
    unsafe fn handleBindingOfBufferNamed_frequency_usingBlock_(
        &self,
        name: NSString,
        frequency: SCNBufferFrequency,
        block: SCNBufferBindingBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleBindingOfBufferNamed : name, frequency : frequency, usingBlock : block)
    }
    unsafe fn setSemantic_forSymbol_options_(
        &self,
        semantic: NSString,
        symbol: NSString,
        options: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSemantic : semantic, forSymbol : symbol, options : options)
    }
    unsafe fn semanticForSymbol_(&self, symbol: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, semanticForSymbol : symbol)
    }
    unsafe fn vertexShader(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexShader)
    }
    unsafe fn setVertexShader_(&self, vertexShader: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexShader : vertexShader)
    }
    unsafe fn fragmentShader(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentShader)
    }
    unsafe fn setFragmentShader_(&self, fragmentShader: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentShader : fragmentShader)
    }
    unsafe fn tessellationControlShader(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationControlShader)
    }
    unsafe fn setTessellationControlShader_(&self, tessellationControlShader: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationControlShader : tessellationControlShader)
    }
    unsafe fn tessellationEvaluationShader(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationEvaluationShader)
    }
    unsafe fn setTessellationEvaluationShader_(&self, tessellationEvaluationShader: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationEvaluationShader : tessellationEvaluationShader)
    }
    unsafe fn geometryShader(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometryShader)
    }
    unsafe fn setGeometryShader_(&self, geometryShader: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeometryShader : geometryShader)
    }
    unsafe fn vertexFunctionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexFunctionName)
    }
    unsafe fn setVertexFunctionName_(&self, vertexFunctionName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexFunctionName : vertexFunctionName)
    }
    unsafe fn fragmentFunctionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentFunctionName)
    }
    unsafe fn setFragmentFunctionName_(&self, fragmentFunctionName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentFunctionName : fragmentFunctionName)
    }
    unsafe fn isOpaque(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpaque)
    }
    unsafe fn setOpaque_(&self, opaque: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpaque : opaque)
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
    unsafe fn library(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, library)
    }
    unsafe fn setLibrary_(&self, library: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLibrary : library)
    }
    unsafe fn program() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNProgram").unwrap(), program)
    }
}
pub trait PSCNProgramDelegate: Sized + std::ops::Deref {
    unsafe fn program_bindValueForSymbol_atLocation_programID_renderer_(
        &self,
        program: SCNProgram,
        symbol: NSString,
        location: ::std::os::raw::c_uint,
        programID: ::std::os::raw::c_uint,
        renderer: SCNRenderer,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, program : program, bindValueForSymbol : symbol, atLocation : location, programID : programID, renderer : renderer)
    }
    unsafe fn program_unbindValueForSymbol_atLocation_programID_renderer_(
        &self,
        program: SCNProgram,
        symbol: NSString,
        location: ::std::os::raw::c_uint,
        programID: ::std::os::raw::c_uint,
        renderer: SCNRenderer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, program : program, unbindValueForSymbol : symbol, atLocation : location, programID : programID, renderer : renderer)
    }
    unsafe fn program_handleError_(&self, program: SCNProgram, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, program : program, handleError : error)
    }
    unsafe fn programIsOpaque_(&self, program: SCNProgram) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, programIsOpaque : program)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNTechnique(pub id);
impl std::ops::Deref for SCNTechnique {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNTechnique {}
impl SCNTechnique {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTechnique").unwrap(), alloc) })
    }
}
impl PSCNAnimatable for SCNTechnique {}
impl PNSCopying for SCNTechnique {}
impl PNSSecureCoding for SCNTechnique {}
impl INSObject for SCNTechnique {}
impl PNSObject for SCNTechnique {}
impl std::convert::TryFrom<NSObject> for SCNTechnique {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNTechnique, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNTechnique").unwrap()) };
        if is_kind_of {
            Ok(SCNTechnique(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNTechnique")
        }
    }
}
impl ISCNTechnique for SCNTechnique {}
pub trait ISCNTechnique: Sized + std::ops::Deref {
    unsafe fn handleBindingOfSymbol_usingBlock_(&self, symbol: NSString, block: SCNBindingBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleBindingOfSymbol : symbol, usingBlock : block)
    }
    unsafe fn objectForKeyedSubscript_(&self, key: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, obj: id, key: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : obj, forKeyedSubscript : key)
    }
    unsafe fn dictionaryRepresentation(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryRepresentation)
    }
    unsafe fn library(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, library)
    }
    unsafe fn setLibrary_(&self, library: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLibrary : library)
    }
    unsafe fn techniqueWithDictionary_(dictionary: NSDictionary) -> SCNTechnique
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTechnique").unwrap(), techniqueWithDictionary : dictionary)
    }
    unsafe fn techniqueBySequencingTechniques_(techniques: NSArray) -> SCNTechnique
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTechnique").unwrap(), techniqueBySequencingTechniques : techniques)
    }
}
pub trait PSCNTechniqueSupport: Sized + std::ops::Deref {
    unsafe fn technique(&self) -> SCNTechnique
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, technique)
    }
    unsafe fn setTechnique_(&self, technique: SCNTechnique)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTechnique : technique)
    }
}
pub type SCNViewOption = NSString;
pub trait PSCNCameraControlConfiguration: Sized + std::ops::Deref {
    unsafe fn autoSwitchToFreeCamera(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoSwitchToFreeCamera)
    }
    unsafe fn setAutoSwitchToFreeCamera_(&self, autoSwitchToFreeCamera: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoSwitchToFreeCamera : autoSwitchToFreeCamera)
    }
    unsafe fn allowsTranslation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsTranslation)
    }
    unsafe fn setAllowsTranslation_(&self, allowsTranslation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsTranslation : allowsTranslation)
    }
    unsafe fn flyModeVelocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flyModeVelocity)
    }
    unsafe fn setFlyModeVelocity_(&self, flyModeVelocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlyModeVelocity : flyModeVelocity)
    }
    unsafe fn panSensitivity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, panSensitivity)
    }
    unsafe fn setPanSensitivity_(&self, panSensitivity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPanSensitivity : panSensitivity)
    }
    unsafe fn truckSensitivity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, truckSensitivity)
    }
    unsafe fn setTruckSensitivity_(&self, truckSensitivity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTruckSensitivity : truckSensitivity)
    }
    unsafe fn rotationSensitivity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationSensitivity)
    }
    unsafe fn setRotationSensitivity_(&self, rotationSensitivity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotationSensitivity : rotationSensitivity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNView(pub id);
impl std::ops::Deref for SCNView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNView {}
impl SCNView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNView").unwrap(), alloc) })
    }
}
impl PSCNSceneRenderer for SCNView {}
impl PSCNTechniqueSupport for SCNView {}
impl INSView for SCNView {}
impl PNSAnimatablePropertyContainer for SCNView {}
impl PNSUserInterfaceItemIdentification for SCNView {}
impl PNSDraggingDestination for SCNView {}
impl PNSAppearanceCustomization for SCNView {}
impl PNSAccessibilityElement for SCNView {}
impl PNSAccessibility for SCNView {}
impl std::convert::TryFrom<NSView> for SCNView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<SCNView, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNView").unwrap()) };
        if is_kind_of {
            Ok(SCNView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to SCNView")
        }
    }
}
impl INSResponder for SCNView {}
impl PNSCoding for SCNView {}
impl INSObject for SCNView {}
impl PNSObject for SCNView {}
impl ISCNView for SCNView {}
pub trait ISCNView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_options_(&self, frame: NSRect, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame, options : options)
    }
    unsafe fn snapshot(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshot)
    }
    unsafe fn play_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, play : sender)
    }
    unsafe fn pause_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pause : sender)
    }
    unsafe fn stop_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stop : sender)
    }
    unsafe fn scene(&self) -> SCNScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scene)
    }
    unsafe fn setScene_(&self, scene: SCNScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScene : scene)
    }
    unsafe fn rendersContinuously(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rendersContinuously)
    }
    unsafe fn setRendersContinuously_(&self, rendersContinuously: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRendersContinuously : rendersContinuously)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, backgroundColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : backgroundColor)
    }
    unsafe fn allowsCameraControl(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCameraControl)
    }
    unsafe fn setAllowsCameraControl_(&self, allowsCameraControl: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsCameraControl : allowsCameraControl)
    }
    unsafe fn cameraControlConfiguration(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraControlConfiguration)
    }
    unsafe fn defaultCameraController(&self) -> SCNCameraController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultCameraController)
    }
    unsafe fn preferredFramesPerSecond(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFramesPerSecond)
    }
    unsafe fn setPreferredFramesPerSecond_(&self, preferredFramesPerSecond: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredFramesPerSecond : preferredFramesPerSecond)
    }
    unsafe fn drawableResizesAsynchronously(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawableResizesAsynchronously)
    }
    unsafe fn setDrawableResizesAsynchronously_(&self, drawableResizesAsynchronously: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawableResizesAsynchronously : drawableResizesAsynchronously)
    }
    unsafe fn openGLContext(&self) -> NSOpenGLContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openGLContext)
    }
    unsafe fn setOpenGLContext_(&self, openGLContext: NSOpenGLContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpenGLContext : openGLContext)
    }
    unsafe fn antialiasingMode(&self) -> SCNAntialiasingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, antialiasingMode)
    }
    unsafe fn setAntialiasingMode_(&self, antialiasingMode: SCNAntialiasingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAntialiasingMode : antialiasingMode)
    }
    unsafe fn pixelFormat(&self) -> NSOpenGLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn setPixelFormat_(&self, pixelFormat: NSOpenGLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelFormat : pixelFormat)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNLayer(pub id);
impl std::ops::Deref for SCNLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNLayer {}
impl SCNLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLayer").unwrap(), alloc) })
    }
}
impl PSCNSceneRenderer for SCNLayer {}
impl PSCNTechniqueSupport for SCNLayer {}
impl ICAOpenGLLayer for SCNLayer {}
impl std::convert::TryFrom<CAOpenGLLayer> for SCNLayer {
    type Error = &'static str;
    fn try_from(parent: CAOpenGLLayer) -> Result<SCNLayer, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNLayer").unwrap()) };
        if is_kind_of {
            Ok(SCNLayer(parent.0))
        } else {
            Err("This CAOpenGLLayer cannot be downcasted to SCNLayer")
        }
    }
}
impl ICALayer for SCNLayer {}
impl PNSSecureCoding for SCNLayer {}
impl PCAMediaTiming for SCNLayer {}
impl INSObject for SCNLayer {}
impl PNSObject for SCNLayer {}
impl ISCNLayer for SCNLayer {}
pub trait ISCNLayer: Sized + std::ops::Deref {
    unsafe fn scene(&self) -> SCNScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scene)
    }
    unsafe fn setScene_(&self, scene: SCNScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScene : scene)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNRenderer(pub id);
impl std::ops::Deref for SCNRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNRenderer {}
impl SCNRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNRenderer").unwrap(), alloc) })
    }
}
impl PSCNSceneRenderer for SCNRenderer {}
impl PSCNTechniqueSupport for SCNRenderer {}
impl INSObject for SCNRenderer {}
impl PNSObject for SCNRenderer {}
impl std::convert::TryFrom<NSObject> for SCNRenderer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNRenderer").unwrap()) };
        if is_kind_of {
            Ok(SCNRenderer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNRenderer")
        }
    }
}
impl ISCNRenderer for SCNRenderer {}
pub trait ISCNRenderer: Sized + std::ops::Deref {
    unsafe fn renderAtTime_viewport_commandBuffer_passDescriptor_(
        &self,
        time: CFTimeInterval,
        viewport: CGRect,
        commandBuffer: *mut u64,
        renderPassDescriptor: MTLRenderPassDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderAtTime : time, viewport : viewport, commandBuffer : commandBuffer, passDescriptor : renderPassDescriptor)
    }
    unsafe fn renderAtTime_(&self, time: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderAtTime : time)
    }
    unsafe fn updateAtTime_(&self, time: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAtTime : time)
    }
    unsafe fn renderWithViewport_commandBuffer_passDescriptor_(
        &self,
        viewport: CGRect,
        commandBuffer: *mut u64,
        renderPassDescriptor: MTLRenderPassDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderWithViewport : viewport, commandBuffer : commandBuffer, passDescriptor : renderPassDescriptor)
    }
    unsafe fn snapshotAtTime_withSize_antialiasingMode_(
        &self,
        time: CFTimeInterval,
        size: CGSize,
        antialiasingMode: SCNAntialiasingMode,
    ) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, snapshotAtTime : time, withSize : size, antialiasingMode : antialiasingMode)
    }
    unsafe fn updateProbes_atTime_(&self, lightProbes: NSArray, time: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateProbes : lightProbes, atTime : time)
    }
    unsafe fn scene(&self) -> SCNScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scene)
    }
    unsafe fn setScene_(&self, scene: SCNScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScene : scene)
    }
    unsafe fn nextFrameTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextFrameTime)
    }
    unsafe fn rendererWithContext_options_(
        context: CGLContextObj,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNRenderer").unwrap(), rendererWithContext : context, options : options)
    }
    unsafe fn rendererWithDevice_options_(device: *mut u64, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNRenderer").unwrap(), rendererWithDevice : device, options : options)
    }
}
pub type SCNSceneSourceLoadingOption = NSString;
pub type SCNSceneSourceAnimationImportPolicy = NSString;
pub type SCNSceneSourceStatus = NSInteger;
pub type SCNSceneSourceStatusHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNSceneSource(pub id);
impl std::ops::Deref for SCNSceneSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNSceneSource {}
impl SCNSceneSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSceneSource").unwrap(), alloc) })
    }
}
impl INSObject for SCNSceneSource {}
impl PNSObject for SCNSceneSource {}
impl std::convert::TryFrom<NSObject> for SCNSceneSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNSceneSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNSceneSource").unwrap()) };
        if is_kind_of {
            Ok(SCNSceneSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNSceneSource")
        }
    }
}
impl ISCNSceneSource for SCNSceneSource {}
pub trait ISCNSceneSource: Sized + std::ops::Deref {
    unsafe fn initWithURL_options_(&self, url: NSURL, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, options : options)
    }
    unsafe fn initWithData_options_(&self, data: NSData, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, options : options)
    }
    unsafe fn sceneWithOptions_statusHandler_(
        &self,
        options: NSDictionary,
        statusHandler: SCNSceneSourceStatusHandler,
    ) -> SCNScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sceneWithOptions : options, statusHandler : statusHandler)
    }
    unsafe fn sceneWithOptions_error_(&self, options: NSDictionary, error: *mut NSError) -> SCNScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sceneWithOptions : options, error : error)
    }
    unsafe fn propertyForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertyForKey : key)
    }
    unsafe fn entryWithIdentifier_withClass_(&self, uid: NSString, entryClass: Class) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, entryWithIdentifier : uid, withClass : entryClass)
    }
    unsafe fn identifiersOfEntriesWithClass_(&self, entryClass: Class) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, identifiersOfEntriesWithClass : entryClass)
    }
    unsafe fn entriesPassingTest_(&self, predicate: *mut ::std::os::raw::c_void) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, entriesPassingTest : predicate)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn sceneSourceWithURL_options_(url: NSURL, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSceneSource").unwrap(), sceneSourceWithURL : url, options : options)
    }
    unsafe fn sceneSourceWithData_options_(data: NSData, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSceneSource").unwrap(), sceneSourceWithData : data, options : options)
    }
}
pub type SCNFilterMode = NSInteger;
pub type SCNWrapMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNMaterialProperty(pub id);
impl std::ops::Deref for SCNMaterialProperty {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNMaterialProperty {}
impl SCNMaterialProperty {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMaterialProperty").unwrap(), alloc) })
    }
}
impl PSCNAnimatable for SCNMaterialProperty {}
impl PNSSecureCoding for SCNMaterialProperty {}
impl INSObject for SCNMaterialProperty {}
impl PNSObject for SCNMaterialProperty {}
impl std::convert::TryFrom<NSObject> for SCNMaterialProperty {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNMaterialProperty, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNMaterialProperty").unwrap()) };
        if is_kind_of {
            Ok(SCNMaterialProperty(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNMaterialProperty")
        }
    }
}
impl ISCNMaterialProperty for SCNMaterialProperty {}
pub trait ISCNMaterialProperty: Sized + std::ops::Deref {
    unsafe fn contents(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn intensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
    unsafe fn minificationFilter(&self) -> SCNFilterMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minificationFilter)
    }
    unsafe fn setMinificationFilter_(&self, minificationFilter: SCNFilterMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinificationFilter : minificationFilter)
    }
    unsafe fn magnificationFilter(&self) -> SCNFilterMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magnificationFilter)
    }
    unsafe fn setMagnificationFilter_(&self, magnificationFilter: SCNFilterMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagnificationFilter : magnificationFilter)
    }
    unsafe fn mipFilter(&self) -> SCNFilterMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mipFilter)
    }
    unsafe fn setMipFilter_(&self, mipFilter: SCNFilterMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMipFilter : mipFilter)
    }
    unsafe fn contentsTransform(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsTransform)
    }
    unsafe fn setContentsTransform_(&self, contentsTransform: SCNMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsTransform : contentsTransform)
    }
    unsafe fn wrapS(&self) -> SCNWrapMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wrapS)
    }
    unsafe fn setWrapS_(&self, wrapS: SCNWrapMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWrapS : wrapS)
    }
    unsafe fn wrapT(&self) -> SCNWrapMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wrapT)
    }
    unsafe fn setWrapT_(&self, wrapT: SCNWrapMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWrapT : wrapT)
    }
    unsafe fn mappingChannel(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mappingChannel)
    }
    unsafe fn setMappingChannel_(&self, mappingChannel: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMappingChannel : mappingChannel)
    }
    unsafe fn textureComponents(&self) -> SCNColorMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureComponents)
    }
    unsafe fn setTextureComponents_(&self, textureComponents: SCNColorMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureComponents : textureComponents)
    }
    unsafe fn maxAnisotropy(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxAnisotropy)
    }
    unsafe fn setMaxAnisotropy_(&self, maxAnisotropy: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxAnisotropy : maxAnisotropy)
    }
    unsafe fn materialPropertyWithContents_(contents: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMaterialProperty").unwrap(), materialPropertyWithContents : contents)
    }
    unsafe fn precomputedLightingEnvironmentContentsWithURL_error_(
        url: NSURL,
        error: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMaterialProperty").unwrap(), precomputedLightingEnvironmentContentsWithURL : url, error : error)
    }
    unsafe fn precomputedLightingEnvironmentContentsWithData_error_(
        data: NSData,
        error: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMaterialProperty").unwrap(), precomputedLightingEnvironmentContentsWithData : data, error : error)
    }
    unsafe fn precomputedLightingEnvironmentDataForContents_device_error_(
        contents: id,
        device: *mut u64,
        error: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMaterialProperty").unwrap(), precomputedLightingEnvironmentDataForContents : contents, device : device, error : error)
    }
}
pub type SCNSceneExportProgressHandler = *mut ::std::os::raw::c_void;
pub type SCNSceneAttribute = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNScene(pub id);
impl std::ops::Deref for SCNScene {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNScene {}
impl SCNScene {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNScene").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SCNScene {}
impl INSObject for SCNScene {}
impl PNSObject for SCNScene {}
impl std::convert::TryFrom<NSObject> for SCNScene {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNScene, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNScene").unwrap()) };
        if is_kind_of {
            Ok(SCNScene(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNScene")
        }
    }
}
impl ISCNScene for SCNScene {}
pub trait ISCNScene: Sized + std::ops::Deref {
    unsafe fn attributeForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributeForKey : key)
    }
    unsafe fn setAttribute_forKey_(&self, attribute: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttribute : attribute, forKey : key)
    }
    unsafe fn writeToURL_options_delegate_progressHandler_(
        &self,
        url: NSURL,
        options: NSDictionary,
        delegate: *mut u64,
        progressHandler: SCNSceneExportProgressHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : url, options : options, delegate : delegate, progressHandler : progressHandler)
    }
    unsafe fn rootNode(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootNode)
    }
    unsafe fn physicsWorld(&self) -> SCNPhysicsWorld
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicsWorld)
    }
    unsafe fn background(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, background)
    }
    unsafe fn lightingEnvironment(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightingEnvironment)
    }
    unsafe fn fogStartDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fogStartDistance)
    }
    unsafe fn setFogStartDistance_(&self, fogStartDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFogStartDistance : fogStartDistance)
    }
    unsafe fn fogEndDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fogEndDistance)
    }
    unsafe fn setFogEndDistance_(&self, fogEndDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFogEndDistance : fogEndDistance)
    }
    unsafe fn fogDensityExponent(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fogDensityExponent)
    }
    unsafe fn setFogDensityExponent_(&self, fogDensityExponent: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFogDensityExponent : fogDensityExponent)
    }
    unsafe fn fogColor(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fogColor)
    }
    unsafe fn setFogColor_(&self, fogColor: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFogColor : fogColor)
    }
    unsafe fn wantsScreenSpaceReflection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsScreenSpaceReflection)
    }
    unsafe fn setWantsScreenSpaceReflection_(&self, wantsScreenSpaceReflection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsScreenSpaceReflection : wantsScreenSpaceReflection)
    }
    unsafe fn screenSpaceReflectionSampleCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceReflectionSampleCount)
    }
    unsafe fn setScreenSpaceReflectionSampleCount_(
        &self,
        screenSpaceReflectionSampleCount: NSInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpaceReflectionSampleCount : screenSpaceReflectionSampleCount)
    }
    unsafe fn screenSpaceReflectionMaximumDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceReflectionMaximumDistance)
    }
    unsafe fn setScreenSpaceReflectionMaximumDistance_(
        &self,
        screenSpaceReflectionMaximumDistance: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpaceReflectionMaximumDistance : screenSpaceReflectionMaximumDistance)
    }
    unsafe fn screenSpaceReflectionStride(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceReflectionStride)
    }
    unsafe fn setScreenSpaceReflectionStride_(&self, screenSpaceReflectionStride: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpaceReflectionStride : screenSpaceReflectionStride)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn setPaused_(&self, paused: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaused : paused)
    }
    unsafe fn scene() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNScene").unwrap(), scene)
    }
    unsafe fn sceneNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNScene").unwrap(), sceneNamed : name)
    }
    unsafe fn sceneNamed_inDirectory_options_(
        name: NSString,
        directory: NSString,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNScene").unwrap(), sceneNamed : name, inDirectory : directory, options : options)
    }
    unsafe fn sceneWithURL_options_error_(
        url: NSURL,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNScene").unwrap(), sceneWithURL : url, options : options, error : error)
    }
}
pub trait PSCNSceneExportDelegate: Sized + std::ops::Deref {
    unsafe fn writeImage_withSceneDocumentURL_originalImageURL_(
        &self,
        image: NSImage,
        documentURL: NSURL,
        originalImageURL: NSURL,
    ) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeImage : image, withSceneDocumentURL : documentURL, originalImageURL : originalImageURL)
    }
}
pub type SCNActionTimingFunction = *mut ::std::os::raw::c_void;
pub trait PSCNActionable: Sized + std::ops::Deref {
    unsafe fn runAction_(&self, action: SCNAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAction : action)
    }
    unsafe fn runAction_completionHandler_(
        &self,
        action: SCNAction,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAction : action, completionHandler : block)
    }
    unsafe fn runAction_forKey_(&self, action: SCNAction, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAction : action, forKey : key)
    }
    unsafe fn runAction_forKey_completionHandler_(
        &self,
        action: SCNAction,
        key: NSString,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAction : action, forKey : key, completionHandler : block)
    }
    unsafe fn actionForKey_(&self, key: NSString) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, actionForKey : key)
    }
    unsafe fn removeActionForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeActionForKey : key)
    }
    unsafe fn removeAllActions(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllActions)
    }
    unsafe fn hasActions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasActions)
    }
    unsafe fn actionKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionKeys)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNAction(pub id);
impl std::ops::Deref for SCNAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNAction {}
impl SCNAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNAction {}
impl PNSSecureCoding for SCNAction {}
impl INSObject for SCNAction {}
impl PNSObject for SCNAction {}
impl std::convert::TryFrom<NSObject> for SCNAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNAction, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNAction").unwrap()) };
        if is_kind_of {
            Ok(SCNAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNAction")
        }
    }
}
impl ISCNAction for SCNAction {}
pub trait ISCNAction: Sized + std::ops::Deref {
    unsafe fn reversedAction(&self) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reversedAction)
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
    unsafe fn timingMode(&self) -> SCNActionTimingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timingMode)
    }
    unsafe fn setTimingMode_(&self, timingMode: SCNActionTimingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimingMode : timingMode)
    }
    unsafe fn timingFunction(&self) -> SCNActionTimingFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timingFunction)
    }
    unsafe fn setTimingFunction_(&self, timingFunction: SCNActionTimingFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimingFunction : timingFunction)
    }
    unsafe fn speed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn setSpeed_(&self, speed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed)
    }
    unsafe fn moveByX_y_z_duration_(
        deltaX: CGFloat,
        deltaY: CGFloat,
        deltaZ: CGFloat,
        duration: NSTimeInterval,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), moveByX : deltaX, y : deltaY, z : deltaZ, duration : duration)
    }
    unsafe fn moveBy_duration_(delta: SCNVector3, duration: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), moveBy : delta, duration : duration)
    }
    unsafe fn moveTo_duration_(location: SCNVector3, duration: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), moveTo : location, duration : duration)
    }
    unsafe fn rotateByX_y_z_duration_(
        xAngle: CGFloat,
        yAngle: CGFloat,
        zAngle: CGFloat,
        duration: NSTimeInterval,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), rotateByX : xAngle, y : yAngle, z : zAngle, duration : duration)
    }
    unsafe fn rotateToX_y_z_duration_(
        xAngle: CGFloat,
        yAngle: CGFloat,
        zAngle: CGFloat,
        duration: NSTimeInterval,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), rotateToX : xAngle, y : yAngle, z : zAngle, duration : duration)
    }
    unsafe fn rotateToX_y_z_duration_shortestUnitArc_(
        xAngle: CGFloat,
        yAngle: CGFloat,
        zAngle: CGFloat,
        duration: NSTimeInterval,
        shortestUnitArc: BOOL,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), rotateToX : xAngle, y : yAngle, z : zAngle, duration : duration, shortestUnitArc : shortestUnitArc)
    }
    unsafe fn rotateByAngle_aroundAxis_duration_(
        angle: CGFloat,
        axis: SCNVector3,
        duration: NSTimeInterval,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), rotateByAngle : angle, aroundAxis : axis, duration : duration)
    }
    unsafe fn rotateToAxisAngle_duration_(
        axisAngle: SCNVector4,
        duration: NSTimeInterval,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), rotateToAxisAngle : axisAngle, duration : duration)
    }
    unsafe fn scaleBy_duration_(scale: CGFloat, sec: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), scaleBy : scale, duration : sec)
    }
    unsafe fn scaleTo_duration_(scale: CGFloat, sec: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), scaleTo : scale, duration : sec)
    }
    unsafe fn sequence_(actions: NSArray) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), sequence : actions)
    }
    unsafe fn group_(actions: NSArray) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), group : actions)
    }
    unsafe fn repeatAction_count_(action: SCNAction, count: NSUInteger) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), repeatAction : action, count : count)
    }
    unsafe fn repeatActionForever_(action: SCNAction) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), repeatActionForever : action)
    }
    unsafe fn fadeInWithDuration_(sec: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), fadeInWithDuration : sec)
    }
    unsafe fn fadeOutWithDuration_(sec: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), fadeOutWithDuration : sec)
    }
    unsafe fn fadeOpacityBy_duration_(factor: CGFloat, sec: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), fadeOpacityBy : factor, duration : sec)
    }
    unsafe fn fadeOpacityTo_duration_(opacity: CGFloat, sec: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), fadeOpacityTo : opacity, duration : sec)
    }
    unsafe fn hide() -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), hide)
    }
    unsafe fn unhide() -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), unhide)
    }
    unsafe fn waitForDuration_(sec: NSTimeInterval) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), waitForDuration : sec)
    }
    unsafe fn waitForDuration_withRange_(
        sec: NSTimeInterval,
        durationRange: NSTimeInterval,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), waitForDuration : sec, withRange : durationRange)
    }
    unsafe fn removeFromParentNode() -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), removeFromParentNode)
    }
    unsafe fn runBlock_(block: *mut ::std::os::raw::c_void) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), runBlock : block)
    }
    unsafe fn runBlock_queue_(block: *mut ::std::os::raw::c_void, queue: NSObject) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), runBlock : block, queue : queue)
    }
    unsafe fn javaScriptActionWithScript_duration_(
        script: NSString,
        seconds: NSTimeInterval,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), javaScriptActionWithScript : script, duration : seconds)
    }
    unsafe fn customActionWithDuration_actionBlock_(
        seconds: NSTimeInterval,
        block: *mut ::std::os::raw::c_void,
    ) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), customActionWithDuration : seconds, actionBlock : block)
    }
    unsafe fn playAudioSource_waitForCompletion_(source: SCNAudioSource, wait: BOOL) -> SCNAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAction").unwrap(), playAudioSource : source, waitForCompletion : wait)
    }
}
pub type SCNMovabilityHint = NSInteger;
pub type SCNNodeFocusBehavior = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNNode(pub id);
impl std::ops::Deref for SCNNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNNode {}
impl SCNNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNNode {}
impl PNSSecureCoding for SCNNode {}
impl PSCNAnimatable for SCNNode {}
impl PSCNActionable for SCNNode {}
impl PSCNBoundingVolume for SCNNode {}
impl INSObject for SCNNode {}
impl PNSObject for SCNNode {}
impl std::convert::TryFrom<NSObject> for SCNNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNNode, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNNode").unwrap()) };
        if is_kind_of {
            Ok(SCNNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNNode")
        }
    }
}
impl ISCNNode for SCNNode {}
pub trait ISCNNode: Sized + std::ops::Deref {
    unsafe fn clone(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clone)
    }
    unsafe fn flattenedClone(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flattenedClone)
    }
    unsafe fn setWorldTransform_(&self, worldTransform: SCNMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorldTransform : worldTransform)
    }
    unsafe fn addChildNode_(&self, child: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addChildNode : child)
    }
    unsafe fn insertChildNode_atIndex_(&self, child: SCNNode, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertChildNode : child, atIndex : index)
    }
    unsafe fn removeFromParentNode(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeFromParentNode)
    }
    unsafe fn replaceChildNode_with_(&self, oldChild: SCNNode, newChild: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceChildNode : oldChild, with : newChild)
    }
    unsafe fn childNodeWithName_recursively_(&self, name: NSString, recursively: BOOL) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, childNodeWithName : name, recursively : recursively)
    }
    unsafe fn childNodesPassingTest_(&self, predicate: *mut ::std::os::raw::c_void) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, childNodesPassingTest : predicate)
    }
    unsafe fn enumerateChildNodesUsingBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateChildNodesUsingBlock : block)
    }
    unsafe fn enumerateHierarchyUsingBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateHierarchyUsingBlock : block)
    }
    unsafe fn convertPosition_toNode_(&self, position: SCNVector3, node: SCNNode) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPosition : position, toNode : node)
    }
    unsafe fn convertPosition_fromNode_(&self, position: SCNVector3, node: SCNNode) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPosition : position, fromNode : node)
    }
    unsafe fn convertVector_toNode_(&self, vector: SCNVector3, node: SCNNode) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertVector : vector, toNode : node)
    }
    unsafe fn convertVector_fromNode_(&self, vector: SCNVector3, node: SCNNode) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertVector : vector, fromNode : node)
    }
    unsafe fn convertTransform_toNode_(&self, transform: SCNMatrix4, node: SCNNode) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertTransform : transform, toNode : node)
    }
    unsafe fn convertTransform_fromNode_(&self, transform: SCNMatrix4, node: SCNNode) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertTransform : transform, fromNode : node)
    }
    unsafe fn hitTestWithSegmentFromPoint_toPoint_options_(
        &self,
        pointA: SCNVector3,
        pointB: SCNVector3,
        options: NSDictionary,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hitTestWithSegmentFromPoint : pointA, toPoint : pointB, options : options)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn light(&self) -> SCNLight
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, light)
    }
    unsafe fn setLight_(&self, light: SCNLight)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLight : light)
    }
    unsafe fn camera(&self) -> SCNCamera
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, camera)
    }
    unsafe fn setCamera_(&self, camera: SCNCamera)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCamera : camera)
    }
    unsafe fn geometry(&self) -> SCNGeometry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometry)
    }
    unsafe fn setGeometry_(&self, geometry: SCNGeometry)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeometry : geometry)
    }
    unsafe fn skinner(&self) -> SCNSkinner
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skinner)
    }
    unsafe fn setSkinner_(&self, skinner: SCNSkinner)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSkinner : skinner)
    }
    unsafe fn morpher(&self) -> SCNMorpher
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, morpher)
    }
    unsafe fn setMorpher_(&self, morpher: SCNMorpher)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMorpher : morpher)
    }
    unsafe fn transform(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: SCNMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
    unsafe fn worldTransform(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldTransform)
    }
    unsafe fn position(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn worldPosition(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldPosition)
    }
    unsafe fn setWorldPosition_(&self, worldPosition: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorldPosition : worldPosition)
    }
    unsafe fn rotation(&self) -> SCNVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: SCNVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn orientation(&self) -> SCNQuaternion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn setOrientation_(&self, orientation: SCNQuaternion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientation : orientation)
    }
    unsafe fn worldOrientation(&self) -> SCNQuaternion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldOrientation)
    }
    unsafe fn setWorldOrientation_(&self, worldOrientation: SCNQuaternion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorldOrientation : worldOrientation)
    }
    unsafe fn eulerAngles(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eulerAngles)
    }
    unsafe fn setEulerAngles_(&self, eulerAngles: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEulerAngles : eulerAngles)
    }
    unsafe fn scale(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn pivot(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pivot)
    }
    unsafe fn setPivot_(&self, pivot: SCNMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPivot : pivot)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn setHidden_(&self, hidden: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidden : hidden)
    }
    unsafe fn opacity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opacity)
    }
    unsafe fn setOpacity_(&self, opacity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpacity : opacity)
    }
    unsafe fn renderingOrder(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderingOrder)
    }
    unsafe fn setRenderingOrder_(&self, renderingOrder: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderingOrder : renderingOrder)
    }
    unsafe fn castsShadow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, castsShadow)
    }
    unsafe fn setCastsShadow_(&self, castsShadow: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCastsShadow : castsShadow)
    }
    unsafe fn movabilityHint(&self) -> SCNMovabilityHint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, movabilityHint)
    }
    unsafe fn setMovabilityHint_(&self, movabilityHint: SCNMovabilityHint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMovabilityHint : movabilityHint)
    }
    unsafe fn parentNode(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentNode)
    }
    unsafe fn childNodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childNodes)
    }
    unsafe fn physicsBody(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicsBody)
    }
    unsafe fn setPhysicsBody_(&self, physicsBody: SCNPhysicsBody)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhysicsBody : physicsBody)
    }
    unsafe fn physicsField(&self) -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicsField)
    }
    unsafe fn setPhysicsField_(&self, physicsField: SCNPhysicsField)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhysicsField : physicsField)
    }
    unsafe fn constraints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constraints)
    }
    unsafe fn setConstraints_(&self, constraints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstraints : constraints)
    }
    unsafe fn filters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filters)
    }
    unsafe fn setFilters_(&self, filters: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilters : filters)
    }
    unsafe fn presentationNode(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationNode)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn setPaused_(&self, paused: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaused : paused)
    }
    unsafe fn rendererDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rendererDelegate)
    }
    unsafe fn setRendererDelegate_(&self, rendererDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRendererDelegate : rendererDelegate)
    }
    unsafe fn categoryBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryBitMask)
    }
    unsafe fn setCategoryBitMask_(&self, categoryBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryBitMask : categoryBitMask)
    }
    unsafe fn node() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), node)
    }
    unsafe fn nodeWithGeometry_(geometry: SCNGeometry) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), nodeWithGeometry : geometry)
    }
}
impl SCNNode_Transforms for SCNNode {}
pub trait SCNNode_Transforms: Sized + std::ops::Deref {
    unsafe fn lookAt_(&self, worldTarget: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAt : worldTarget)
    }
    unsafe fn lookAt_up_localFront_(
        &self,
        worldTarget: SCNVector3,
        worldUp: SCNVector3,
        localFront: SCNVector3,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAt : worldTarget, up : worldUp, localFront : localFront)
    }
    unsafe fn localTranslateBy_(&self, translation: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localTranslateBy : translation)
    }
    unsafe fn localRotateBy_(&self, rotation: SCNQuaternion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localRotateBy : rotation)
    }
    unsafe fn rotateBy_aroundTarget_(&self, worldRotation: SCNQuaternion, worldTarget: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotateBy : worldRotation, aroundTarget : worldTarget)
    }
    unsafe fn worldUp(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldUp)
    }
    unsafe fn worldRight(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldRight)
    }
    unsafe fn worldFront(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldFront)
    }
    unsafe fn localUp() -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), localUp)
    }
    unsafe fn localRight() -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), localRight)
    }
    unsafe fn localFront() -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), localFront)
    }
}
pub trait PSCNNodeRendererDelegate: Sized + std::ops::Deref {
    unsafe fn renderNode_renderer_arguments_(
        &self,
        node: SCNNode,
        renderer: SCNRenderer,
        arguments: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderNode : node, renderer : renderer, arguments : arguments)
    }
}
impl SCNNode_SIMD for SCNNode {}
pub trait SCNNode_SIMD: Sized + std::ops::Deref {
    unsafe fn simdConvertPosition_toNode_(
        &self,
        position: simd_float3,
        node: SCNNode,
    ) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, simdConvertPosition : position, toNode : node)
    }
    unsafe fn simdConvertPosition_fromNode_(
        &self,
        position: simd_float3,
        node: SCNNode,
    ) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, simdConvertPosition : position, fromNode : node)
    }
    unsafe fn simdConvertVector_toNode_(&self, vector: simd_float3, node: SCNNode) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, simdConvertVector : vector, toNode : node)
    }
    unsafe fn simdConvertVector_fromNode_(&self, vector: simd_float3, node: SCNNode) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, simdConvertVector : vector, fromNode : node)
    }
    unsafe fn simdLookAt_(&self, worldTarget: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, simdLookAt : worldTarget)
    }
    unsafe fn simdLookAt_up_localFront_(
        &self,
        worldTarget: simd_float3,
        worldUp: simd_float3,
        localFront: simd_float3,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, simdLookAt : worldTarget, up : worldUp, localFront : localFront)
    }
    unsafe fn simdLocalTranslateBy_(&self, translation: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, simdLocalTranslateBy : translation)
    }
    unsafe fn simdPosition(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdPosition)
    }
    unsafe fn setSimdPosition_(&self, simdPosition: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSimdPosition : simdPosition)
    }
    unsafe fn simdRotation(&self) -> simd_float4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdRotation)
    }
    unsafe fn setSimdRotation_(&self, simdRotation: simd_float4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSimdRotation : simdRotation)
    }
    unsafe fn simdEulerAngles(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdEulerAngles)
    }
    unsafe fn setSimdEulerAngles_(&self, simdEulerAngles: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSimdEulerAngles : simdEulerAngles)
    }
    unsafe fn simdScale(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdScale)
    }
    unsafe fn setSimdScale_(&self, simdScale: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSimdScale : simdScale)
    }
    unsafe fn simdWorldPosition(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdWorldPosition)
    }
    unsafe fn setSimdWorldPosition_(&self, simdWorldPosition: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSimdWorldPosition : simdWorldPosition)
    }
    unsafe fn simdWorldUp(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdWorldUp)
    }
    unsafe fn simdWorldRight(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdWorldRight)
    }
    unsafe fn simdWorldFront(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simdWorldFront)
    }
    unsafe fn simdLocalUp() -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), simdLocalUp)
    }
    unsafe fn simdLocalRight() -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), simdLocalRight)
    }
    unsafe fn simdLocalFront() -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), simdLocalFront)
    }
}
impl SCNNode_Focus for SCNNode {}
pub trait SCNNode_Focus: Sized + std::ops::Deref {
    unsafe fn focusBehavior(&self) -> SCNNodeFocusBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusBehavior)
    }
    unsafe fn setFocusBehavior_(&self, focusBehavior: SCNNodeFocusBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusBehavior : focusBehavior)
    }
}
pub type SCNLightType = NSString;
pub type SCNShadowMode = NSInteger;
pub type SCNLightProbeType = NSInteger;
pub type SCNLightProbeUpdateType = NSInteger;
pub type SCNLightAreaType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNLight(pub id);
impl std::ops::Deref for SCNLight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNLight {}
impl SCNLight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLight").unwrap(), alloc) })
    }
}
impl PSCNAnimatable for SCNLight {}
impl PNSCopying for SCNLight {}
impl PNSSecureCoding for SCNLight {}
impl INSObject for SCNLight {}
impl PNSObject for SCNLight {}
impl std::convert::TryFrom<NSObject> for SCNLight {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNLight, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNLight").unwrap()) };
        if is_kind_of {
            Ok(SCNLight(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNLight")
        }
    }
}
impl ISCNLight for SCNLight {}
pub trait ISCNLight: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> SCNLightType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn color(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn temperature(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, temperature)
    }
    unsafe fn setTemperature_(&self, temperature: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTemperature : temperature)
    }
    unsafe fn intensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn castsShadow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, castsShadow)
    }
    unsafe fn setCastsShadow_(&self, castsShadow: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCastsShadow : castsShadow)
    }
    unsafe fn shadowColor(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowColor)
    }
    unsafe fn setShadowColor_(&self, shadowColor: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowColor : shadowColor)
    }
    unsafe fn shadowRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowRadius)
    }
    unsafe fn setShadowRadius_(&self, shadowRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowRadius : shadowRadius)
    }
    unsafe fn shadowMapSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowMapSize)
    }
    unsafe fn setShadowMapSize_(&self, shadowMapSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowMapSize : shadowMapSize)
    }
    unsafe fn shadowSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowSampleCount)
    }
    unsafe fn setShadowSampleCount_(&self, shadowSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowSampleCount : shadowSampleCount)
    }
    unsafe fn shadowMode(&self) -> SCNShadowMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowMode)
    }
    unsafe fn setShadowMode_(&self, shadowMode: SCNShadowMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowMode : shadowMode)
    }
    unsafe fn shadowBias(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowBias)
    }
    unsafe fn setShadowBias_(&self, shadowBias: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowBias : shadowBias)
    }
    unsafe fn automaticallyAdjustsShadowProjection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticallyAdjustsShadowProjection)
    }
    unsafe fn setAutomaticallyAdjustsShadowProjection_(
        &self,
        automaticallyAdjustsShadowProjection: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticallyAdjustsShadowProjection : automaticallyAdjustsShadowProjection)
    }
    unsafe fn maximumShadowDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumShadowDistance)
    }
    unsafe fn setMaximumShadowDistance_(&self, maximumShadowDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumShadowDistance : maximumShadowDistance)
    }
    unsafe fn forcesBackFaceCasters(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forcesBackFaceCasters)
    }
    unsafe fn setForcesBackFaceCasters_(&self, forcesBackFaceCasters: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setForcesBackFaceCasters : forcesBackFaceCasters)
    }
    unsafe fn sampleDistributedShadowMaps(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleDistributedShadowMaps)
    }
    unsafe fn setSampleDistributedShadowMaps_(&self, sampleDistributedShadowMaps: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleDistributedShadowMaps : sampleDistributedShadowMaps)
    }
    unsafe fn shadowCascadeCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowCascadeCount)
    }
    unsafe fn setShadowCascadeCount_(&self, shadowCascadeCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowCascadeCount : shadowCascadeCount)
    }
    unsafe fn shadowCascadeSplittingFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowCascadeSplittingFactor)
    }
    unsafe fn setShadowCascadeSplittingFactor_(&self, shadowCascadeSplittingFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowCascadeSplittingFactor : shadowCascadeSplittingFactor)
    }
    unsafe fn orthographicScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orthographicScale)
    }
    unsafe fn setOrthographicScale_(&self, orthographicScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrthographicScale : orthographicScale)
    }
    unsafe fn zNear(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zNear)
    }
    unsafe fn setZNear_(&self, zNear: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZNear : zNear)
    }
    unsafe fn zFar(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zFar)
    }
    unsafe fn setZFar_(&self, zFar: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZFar : zFar)
    }
    unsafe fn attenuationStartDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationStartDistance)
    }
    unsafe fn setAttenuationStartDistance_(&self, attenuationStartDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttenuationStartDistance : attenuationStartDistance)
    }
    unsafe fn attenuationEndDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationEndDistance)
    }
    unsafe fn setAttenuationEndDistance_(&self, attenuationEndDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttenuationEndDistance : attenuationEndDistance)
    }
    unsafe fn attenuationFalloffExponent(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationFalloffExponent)
    }
    unsafe fn setAttenuationFalloffExponent_(&self, attenuationFalloffExponent: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttenuationFalloffExponent : attenuationFalloffExponent)
    }
    unsafe fn spotInnerAngle(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spotInnerAngle)
    }
    unsafe fn setSpotInnerAngle_(&self, spotInnerAngle: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpotInnerAngle : spotInnerAngle)
    }
    unsafe fn spotOuterAngle(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spotOuterAngle)
    }
    unsafe fn setSpotOuterAngle_(&self, spotOuterAngle: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpotOuterAngle : spotOuterAngle)
    }
    unsafe fn IESProfileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, IESProfileURL)
    }
    unsafe fn setIESProfileURL_(&self, IESProfileURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIESProfileURL : IESProfileURL)
    }
    unsafe fn sphericalHarmonicsCoefficients(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sphericalHarmonicsCoefficients)
    }
    unsafe fn probeType(&self) -> SCNLightProbeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, probeType)
    }
    unsafe fn setProbeType_(&self, probeType: SCNLightProbeType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProbeType : probeType)
    }
    unsafe fn probeUpdateType(&self) -> SCNLightProbeUpdateType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, probeUpdateType)
    }
    unsafe fn setProbeUpdateType_(&self, probeUpdateType: SCNLightProbeUpdateType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProbeUpdateType : probeUpdateType)
    }
    unsafe fn probeExtents(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, probeExtents)
    }
    unsafe fn setProbeExtents_(&self, probeExtents: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProbeExtents : probeExtents)
    }
    unsafe fn probeOffset(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, probeOffset)
    }
    unsafe fn setProbeOffset_(&self, probeOffset: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProbeOffset : probeOffset)
    }
    unsafe fn parallaxCorrectionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parallaxCorrectionEnabled)
    }
    unsafe fn setParallaxCorrectionEnabled_(&self, parallaxCorrectionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParallaxCorrectionEnabled : parallaxCorrectionEnabled)
    }
    unsafe fn parallaxExtentsFactor(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parallaxExtentsFactor)
    }
    unsafe fn setParallaxExtentsFactor_(&self, parallaxExtentsFactor: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParallaxExtentsFactor : parallaxExtentsFactor)
    }
    unsafe fn parallaxCenterOffset(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parallaxCenterOffset)
    }
    unsafe fn setParallaxCenterOffset_(&self, parallaxCenterOffset: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParallaxCenterOffset : parallaxCenterOffset)
    }
    unsafe fn probeEnvironment(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, probeEnvironment)
    }
    unsafe fn areaType(&self) -> SCNLightAreaType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areaType)
    }
    unsafe fn setAreaType_(&self, areaType: SCNLightAreaType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAreaType : areaType)
    }
    unsafe fn areaExtents(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areaExtents)
    }
    unsafe fn setAreaExtents_(&self, areaExtents: simd_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAreaExtents : areaExtents)
    }
    unsafe fn areaPolygonVertices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areaPolygonVertices)
    }
    unsafe fn setAreaPolygonVertices_(&self, areaPolygonVertices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAreaPolygonVertices : areaPolygonVertices)
    }
    unsafe fn drawsArea(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawsArea)
    }
    unsafe fn setDrawsArea_(&self, drawsArea: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsArea : drawsArea)
    }
    unsafe fn doubleSided(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doubleSided)
    }
    unsafe fn setDoubleSided_(&self, doubleSided: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleSided : doubleSided)
    }
    unsafe fn gobo(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gobo)
    }
    unsafe fn categoryBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryBitMask)
    }
    unsafe fn setCategoryBitMask_(&self, categoryBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryBitMask : categoryBitMask)
    }
    unsafe fn light() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLight").unwrap(), light)
    }
}
pub type SCNCameraProjectionDirection = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNCamera(pub id);
impl std::ops::Deref for SCNCamera {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNCamera {}
impl SCNCamera {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCamera").unwrap(), alloc) })
    }
}
impl PSCNAnimatable for SCNCamera {}
impl PSCNTechniqueSupport for SCNCamera {}
impl PNSCopying for SCNCamera {}
impl PNSSecureCoding for SCNCamera {}
impl INSObject for SCNCamera {}
impl PNSObject for SCNCamera {}
impl std::convert::TryFrom<NSObject> for SCNCamera {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNCamera, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNCamera").unwrap()) };
        if is_kind_of {
            Ok(SCNCamera(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNCamera")
        }
    }
}
impl ISCNCamera for SCNCamera {}
pub trait ISCNCamera: Sized + std::ops::Deref {
    unsafe fn projectionTransform(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectionTransform)
    }
    unsafe fn setProjectionTransform_(&self, projectionTransform: SCNMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProjectionTransform : projectionTransform)
    }
    unsafe fn projectionTransformWithViewportSize_(&self, viewportSize: CGSize) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, projectionTransformWithViewportSize : viewportSize)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn fieldOfView(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldOfView)
    }
    unsafe fn setFieldOfView_(&self, fieldOfView: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldOfView : fieldOfView)
    }
    unsafe fn projectionDirection(&self) -> SCNCameraProjectionDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectionDirection)
    }
    unsafe fn setProjectionDirection_(&self, projectionDirection: SCNCameraProjectionDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProjectionDirection : projectionDirection)
    }
    unsafe fn focalLength(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalLength)
    }
    unsafe fn setFocalLength_(&self, focalLength: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalLength : focalLength)
    }
    unsafe fn sensorHeight(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensorHeight)
    }
    unsafe fn setSensorHeight_(&self, sensorHeight: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSensorHeight : sensorHeight)
    }
    unsafe fn zNear(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zNear)
    }
    unsafe fn setZNear_(&self, zNear: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZNear : zNear)
    }
    unsafe fn zFar(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zFar)
    }
    unsafe fn setZFar_(&self, zFar: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZFar : zFar)
    }
    unsafe fn automaticallyAdjustsZRange(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticallyAdjustsZRange)
    }
    unsafe fn setAutomaticallyAdjustsZRange_(&self, automaticallyAdjustsZRange: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticallyAdjustsZRange : automaticallyAdjustsZRange)
    }
    unsafe fn usesOrthographicProjection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesOrthographicProjection)
    }
    unsafe fn setUsesOrthographicProjection_(&self, usesOrthographicProjection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesOrthographicProjection : usesOrthographicProjection)
    }
    unsafe fn orthographicScale(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orthographicScale)
    }
    unsafe fn setOrthographicScale_(&self, orthographicScale: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrthographicScale : orthographicScale)
    }
    unsafe fn wantsDepthOfField(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsDepthOfField)
    }
    unsafe fn setWantsDepthOfField_(&self, wantsDepthOfField: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsDepthOfField : wantsDepthOfField)
    }
    unsafe fn focusDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusDistance)
    }
    unsafe fn setFocusDistance_(&self, focusDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusDistance : focusDistance)
    }
    unsafe fn focalBlurSampleCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalBlurSampleCount)
    }
    unsafe fn setFocalBlurSampleCount_(&self, focalBlurSampleCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalBlurSampleCount : focalBlurSampleCount)
    }
    unsafe fn fStop(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fStop)
    }
    unsafe fn setFStop_(&self, fStop: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFStop : fStop)
    }
    unsafe fn apertureBladeCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, apertureBladeCount)
    }
    unsafe fn setApertureBladeCount_(&self, apertureBladeCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApertureBladeCount : apertureBladeCount)
    }
    unsafe fn motionBlurIntensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionBlurIntensity)
    }
    unsafe fn setMotionBlurIntensity_(&self, motionBlurIntensity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionBlurIntensity : motionBlurIntensity)
    }
    unsafe fn screenSpaceAmbientOcclusionIntensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceAmbientOcclusionIntensity)
    }
    unsafe fn setScreenSpaceAmbientOcclusionIntensity_(
        &self,
        screenSpaceAmbientOcclusionIntensity: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpaceAmbientOcclusionIntensity : screenSpaceAmbientOcclusionIntensity)
    }
    unsafe fn screenSpaceAmbientOcclusionRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceAmbientOcclusionRadius)
    }
    unsafe fn setScreenSpaceAmbientOcclusionRadius_(
        &self,
        screenSpaceAmbientOcclusionRadius: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpaceAmbientOcclusionRadius : screenSpaceAmbientOcclusionRadius)
    }
    unsafe fn screenSpaceAmbientOcclusionBias(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceAmbientOcclusionBias)
    }
    unsafe fn setScreenSpaceAmbientOcclusionBias_(&self, screenSpaceAmbientOcclusionBias: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpaceAmbientOcclusionBias : screenSpaceAmbientOcclusionBias)
    }
    unsafe fn screenSpaceAmbientOcclusionDepthThreshold(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceAmbientOcclusionDepthThreshold)
    }
    unsafe fn setScreenSpaceAmbientOcclusionDepthThreshold_(
        &self,
        screenSpaceAmbientOcclusionDepthThreshold: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpaceAmbientOcclusionDepthThreshold : screenSpaceAmbientOcclusionDepthThreshold)
    }
    unsafe fn screenSpaceAmbientOcclusionNormalThreshold(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceAmbientOcclusionNormalThreshold)
    }
    unsafe fn setScreenSpaceAmbientOcclusionNormalThreshold_(
        &self,
        screenSpaceAmbientOcclusionNormalThreshold: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpaceAmbientOcclusionNormalThreshold : screenSpaceAmbientOcclusionNormalThreshold)
    }
    unsafe fn wantsHDR(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsHDR)
    }
    unsafe fn setWantsHDR_(&self, wantsHDR: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsHDR : wantsHDR)
    }
    unsafe fn exposureOffset(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureOffset)
    }
    unsafe fn setExposureOffset_(&self, exposureOffset: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureOffset : exposureOffset)
    }
    unsafe fn averageGray(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averageGray)
    }
    unsafe fn setAverageGray_(&self, averageGray: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAverageGray : averageGray)
    }
    unsafe fn whitePoint(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, whitePoint)
    }
    unsafe fn setWhitePoint_(&self, whitePoint: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWhitePoint : whitePoint)
    }
    unsafe fn wantsExposureAdaptation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsExposureAdaptation)
    }
    unsafe fn setWantsExposureAdaptation_(&self, wantsExposureAdaptation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsExposureAdaptation : wantsExposureAdaptation)
    }
    unsafe fn exposureAdaptationBrighteningSpeedFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureAdaptationBrighteningSpeedFactor)
    }
    unsafe fn setExposureAdaptationBrighteningSpeedFactor_(
        &self,
        exposureAdaptationBrighteningSpeedFactor: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureAdaptationBrighteningSpeedFactor : exposureAdaptationBrighteningSpeedFactor)
    }
    unsafe fn exposureAdaptationDarkeningSpeedFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureAdaptationDarkeningSpeedFactor)
    }
    unsafe fn setExposureAdaptationDarkeningSpeedFactor_(
        &self,
        exposureAdaptationDarkeningSpeedFactor: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureAdaptationDarkeningSpeedFactor : exposureAdaptationDarkeningSpeedFactor)
    }
    unsafe fn minimumExposure(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumExposure)
    }
    unsafe fn setMinimumExposure_(&self, minimumExposure: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumExposure : minimumExposure)
    }
    unsafe fn maximumExposure(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumExposure)
    }
    unsafe fn setMaximumExposure_(&self, maximumExposure: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumExposure : maximumExposure)
    }
    unsafe fn bloomThreshold(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bloomThreshold)
    }
    unsafe fn setBloomThreshold_(&self, bloomThreshold: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBloomThreshold : bloomThreshold)
    }
    unsafe fn bloomIterationCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bloomIterationCount)
    }
    unsafe fn setBloomIterationCount_(&self, bloomIterationCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBloomIterationCount : bloomIterationCount)
    }
    unsafe fn bloomIterationSpread(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bloomIterationSpread)
    }
    unsafe fn setBloomIterationSpread_(&self, bloomIterationSpread: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBloomIterationSpread : bloomIterationSpread)
    }
    unsafe fn bloomIntensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bloomIntensity)
    }
    unsafe fn setBloomIntensity_(&self, bloomIntensity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBloomIntensity : bloomIntensity)
    }
    unsafe fn bloomBlurRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bloomBlurRadius)
    }
    unsafe fn setBloomBlurRadius_(&self, bloomBlurRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBloomBlurRadius : bloomBlurRadius)
    }
    unsafe fn vignettingPower(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vignettingPower)
    }
    unsafe fn setVignettingPower_(&self, vignettingPower: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVignettingPower : vignettingPower)
    }
    unsafe fn vignettingIntensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vignettingIntensity)
    }
    unsafe fn setVignettingIntensity_(&self, vignettingIntensity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVignettingIntensity : vignettingIntensity)
    }
    unsafe fn colorFringeStrength(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorFringeStrength)
    }
    unsafe fn setColorFringeStrength_(&self, colorFringeStrength: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorFringeStrength : colorFringeStrength)
    }
    unsafe fn colorFringeIntensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorFringeIntensity)
    }
    unsafe fn setColorFringeIntensity_(&self, colorFringeIntensity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorFringeIntensity : colorFringeIntensity)
    }
    unsafe fn saturation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saturation)
    }
    unsafe fn setSaturation_(&self, saturation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSaturation : saturation)
    }
    unsafe fn contrast(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast)
    }
    unsafe fn setContrast_(&self, contrast: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast : contrast)
    }
    unsafe fn grainIntensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grainIntensity)
    }
    unsafe fn setGrainIntensity_(&self, grainIntensity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrainIntensity : grainIntensity)
    }
    unsafe fn grainScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grainScale)
    }
    unsafe fn setGrainScale_(&self, grainScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrainScale : grainScale)
    }
    unsafe fn grainIsColored(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grainIsColored)
    }
    unsafe fn setGrainIsColored_(&self, grainIsColored: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrainIsColored : grainIsColored)
    }
    unsafe fn whiteBalanceTemperature(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, whiteBalanceTemperature)
    }
    unsafe fn setWhiteBalanceTemperature_(&self, whiteBalanceTemperature: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWhiteBalanceTemperature : whiteBalanceTemperature)
    }
    unsafe fn whiteBalanceTint(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, whiteBalanceTint)
    }
    unsafe fn setWhiteBalanceTint_(&self, whiteBalanceTint: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWhiteBalanceTint : whiteBalanceTint)
    }
    unsafe fn colorGrading(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorGrading)
    }
    unsafe fn categoryBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryBitMask)
    }
    unsafe fn setCategoryBitMask_(&self, categoryBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryBitMask : categoryBitMask)
    }
    unsafe fn camera() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCamera").unwrap(), camera)
    }
}
pub type SCNLightingModel = NSString;
pub type SCNFillMode = NSUInteger;
pub type SCNCullMode = NSInteger;
pub type SCNTransparencyMode = NSInteger;
pub type SCNBlendMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNMaterial(pub id);
impl std::ops::Deref for SCNMaterial {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNMaterial {}
impl SCNMaterial {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMaterial").unwrap(), alloc) })
    }
}
impl PSCNAnimatable for SCNMaterial {}
impl PSCNShadable for SCNMaterial {}
impl PNSCopying for SCNMaterial {}
impl PNSSecureCoding for SCNMaterial {}
impl INSObject for SCNMaterial {}
impl PNSObject for SCNMaterial {}
impl std::convert::TryFrom<NSObject> for SCNMaterial {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNMaterial, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNMaterial").unwrap()) };
        if is_kind_of {
            Ok(SCNMaterial(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNMaterial")
        }
    }
}
impl ISCNMaterial for SCNMaterial {}
pub trait ISCNMaterial: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn diffuse(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diffuse)
    }
    unsafe fn ambient(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ambient)
    }
    unsafe fn specular(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specular)
    }
    unsafe fn emission(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emission)
    }
    unsafe fn transparent(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transparent)
    }
    unsafe fn reflective(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflective)
    }
    unsafe fn multiply(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiply)
    }
    unsafe fn normal(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normal)
    }
    unsafe fn displacement(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displacement)
    }
    unsafe fn ambientOcclusion(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ambientOcclusion)
    }
    unsafe fn selfIllumination(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selfIllumination)
    }
    unsafe fn metalness(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalness)
    }
    unsafe fn roughness(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roughness)
    }
    unsafe fn clearCoat(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearCoat)
    }
    unsafe fn clearCoatRoughness(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearCoatRoughness)
    }
    unsafe fn clearCoatNormal(&self) -> SCNMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearCoatNormal)
    }
    unsafe fn shininess(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shininess)
    }
    unsafe fn setShininess_(&self, shininess: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShininess : shininess)
    }
    unsafe fn transparency(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transparency)
    }
    unsafe fn setTransparency_(&self, transparency: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransparency : transparency)
    }
    unsafe fn lightingModelName(&self) -> SCNLightingModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightingModelName)
    }
    unsafe fn setLightingModelName_(&self, lightingModelName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightingModelName : lightingModelName)
    }
    unsafe fn isLitPerPixel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLitPerPixel)
    }
    unsafe fn setLitPerPixel_(&self, litPerPixel: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLitPerPixel : litPerPixel)
    }
    unsafe fn isDoubleSided(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDoubleSided)
    }
    unsafe fn setDoubleSided_(&self, doubleSided: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleSided : doubleSided)
    }
    unsafe fn fillMode(&self) -> SCNFillMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillMode)
    }
    unsafe fn setFillMode_(&self, fillMode: SCNFillMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillMode : fillMode)
    }
    unsafe fn cullMode(&self) -> SCNCullMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cullMode)
    }
    unsafe fn setCullMode_(&self, cullMode: SCNCullMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCullMode : cullMode)
    }
    unsafe fn transparencyMode(&self) -> SCNTransparencyMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transparencyMode)
    }
    unsafe fn setTransparencyMode_(&self, transparencyMode: SCNTransparencyMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransparencyMode : transparencyMode)
    }
    unsafe fn locksAmbientWithDiffuse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locksAmbientWithDiffuse)
    }
    unsafe fn setLocksAmbientWithDiffuse_(&self, locksAmbientWithDiffuse: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocksAmbientWithDiffuse : locksAmbientWithDiffuse)
    }
    unsafe fn writesToDepthBuffer(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writesToDepthBuffer)
    }
    unsafe fn setWritesToDepthBuffer_(&self, writesToDepthBuffer: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWritesToDepthBuffer : writesToDepthBuffer)
    }
    unsafe fn colorBufferWriteMask(&self) -> SCNColorMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorBufferWriteMask)
    }
    unsafe fn setColorBufferWriteMask_(&self, colorBufferWriteMask: SCNColorMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorBufferWriteMask : colorBufferWriteMask)
    }
    unsafe fn readsFromDepthBuffer(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readsFromDepthBuffer)
    }
    unsafe fn setReadsFromDepthBuffer_(&self, readsFromDepthBuffer: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadsFromDepthBuffer : readsFromDepthBuffer)
    }
    unsafe fn fresnelExponent(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fresnelExponent)
    }
    unsafe fn setFresnelExponent_(&self, fresnelExponent: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFresnelExponent : fresnelExponent)
    }
    unsafe fn blendMode(&self) -> SCNBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendMode)
    }
    unsafe fn setBlendMode_(&self, blendMode: SCNBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendMode : blendMode)
    }
    unsafe fn material() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMaterial").unwrap(), material)
    }
}
pub type SCNGeometryPrimitiveType = NSInteger;
pub type SCNGeometrySourceSemantic = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNGeometry(pub id);
impl std::ops::Deref for SCNGeometry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNGeometry {}
impl SCNGeometry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometry").unwrap(), alloc) })
    }
}
impl PSCNAnimatable for SCNGeometry {}
impl PSCNBoundingVolume for SCNGeometry {}
impl PSCNShadable for SCNGeometry {}
impl PNSCopying for SCNGeometry {}
impl PNSSecureCoding for SCNGeometry {}
impl INSObject for SCNGeometry {}
impl PNSObject for SCNGeometry {}
impl std::convert::TryFrom<NSObject> for SCNGeometry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNGeometry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNGeometry").unwrap()) };
        if is_kind_of {
            Ok(SCNGeometry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNGeometry")
        }
    }
}
impl ISCNGeometry for SCNGeometry {}
pub trait ISCNGeometry: Sized + std::ops::Deref {
    unsafe fn insertMaterial_atIndex_(&self, material: SCNMaterial, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertMaterial : material, atIndex : index)
    }
    unsafe fn removeMaterialAtIndex_(&self, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeMaterialAtIndex : index)
    }
    unsafe fn replaceMaterialAtIndex_withMaterial_(&self, index: NSUInteger, material: SCNMaterial)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceMaterialAtIndex : index, withMaterial : material)
    }
    unsafe fn materialWithName_(&self, name: NSString) -> SCNMaterial
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, materialWithName : name)
    }
    unsafe fn geometrySourcesForSemantic_(&self, semantic: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geometrySourcesForSemantic : semantic)
    }
    unsafe fn geometryElementAtIndex_(&self, elementIndex: NSInteger) -> SCNGeometryElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geometryElementAtIndex : elementIndex)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn materials(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, materials)
    }
    unsafe fn setMaterials_(&self, materials: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaterials : materials)
    }
    unsafe fn firstMaterial(&self) -> SCNMaterial
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstMaterial)
    }
    unsafe fn setFirstMaterial_(&self, firstMaterial: SCNMaterial)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFirstMaterial : firstMaterial)
    }
    unsafe fn geometrySources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometrySources)
    }
    unsafe fn geometryElements(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometryElements)
    }
    unsafe fn geometryElementCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometryElementCount)
    }
    unsafe fn geometrySourceChannels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometrySourceChannels)
    }
    unsafe fn levelsOfDetail(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, levelsOfDetail)
    }
    unsafe fn setLevelsOfDetail_(&self, levelsOfDetail: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevelsOfDetail : levelsOfDetail)
    }
    unsafe fn tessellator(&self) -> SCNGeometryTessellator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellator)
    }
    unsafe fn setTessellator_(&self, tessellator: SCNGeometryTessellator)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellator : tessellator)
    }
    unsafe fn subdivisionLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subdivisionLevel)
    }
    unsafe fn setSubdivisionLevel_(&self, subdivisionLevel: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubdivisionLevel : subdivisionLevel)
    }
    unsafe fn wantsAdaptiveSubdivision(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsAdaptiveSubdivision)
    }
    unsafe fn setWantsAdaptiveSubdivision_(&self, wantsAdaptiveSubdivision: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsAdaptiveSubdivision : wantsAdaptiveSubdivision)
    }
    unsafe fn edgeCreasesElement(&self) -> SCNGeometryElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeCreasesElement)
    }
    unsafe fn setEdgeCreasesElement_(&self, edgeCreasesElement: SCNGeometryElement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeCreasesElement : edgeCreasesElement)
    }
    unsafe fn edgeCreasesSource(&self) -> SCNGeometrySource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeCreasesSource)
    }
    unsafe fn setEdgeCreasesSource_(&self, edgeCreasesSource: SCNGeometrySource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeCreasesSource : edgeCreasesSource)
    }
    unsafe fn geometry() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometry").unwrap(), geometry)
    }
    unsafe fn geometryWithSources_elements_(sources: NSArray, elements: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometry").unwrap(), geometryWithSources : sources, elements : elements)
    }
    unsafe fn geometryWithSources_elements_sourceChannels_(
        sources: NSArray,
        elements: NSArray,
        sourceChannels: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometry").unwrap(), geometryWithSources : sources, elements : elements, sourceChannels : sourceChannels)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNGeometrySource(pub id);
impl std::ops::Deref for SCNGeometrySource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNGeometrySource {}
impl SCNGeometrySource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometrySource").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SCNGeometrySource {}
impl INSObject for SCNGeometrySource {}
impl PNSObject for SCNGeometrySource {}
impl std::convert::TryFrom<NSObject> for SCNGeometrySource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNGeometrySource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNGeometrySource").unwrap()) };
        if is_kind_of {
            Ok(SCNGeometrySource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNGeometrySource")
        }
    }
}
impl ISCNGeometrySource for SCNGeometrySource {}
pub trait ISCNGeometrySource: Sized + std::ops::Deref {
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn semantic(&self) -> SCNGeometrySourceSemantic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semantic)
    }
    unsafe fn vectorCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vectorCount)
    }
    unsafe fn floatComponents(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatComponents)
    }
    unsafe fn componentsPerVector(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, componentsPerVector)
    }
    unsafe fn bytesPerComponent(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerComponent)
    }
    unsafe fn dataOffset(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataOffset)
    }
    unsafe fn dataStride(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataStride)
    }
    unsafe fn geometrySourceWithData_semantic_vectorCount_floatComponents_componentsPerVector_bytesPerComponent_dataOffset_dataStride_(
        data: NSData,
        semantic: NSString,
        vectorCount: NSInteger,
        floatComponents: BOOL,
        componentsPerVector: NSInteger,
        bytesPerComponent: NSInteger,
        offset: NSInteger,
        stride: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometrySource").unwrap(), geometrySourceWithData : data, semantic : semantic, vectorCount : vectorCount, floatComponents : floatComponents, componentsPerVector : componentsPerVector, bytesPerComponent : bytesPerComponent, dataOffset : offset, dataStride : stride)
    }
    unsafe fn geometrySourceWithVertices_count_(
        vertices: *const SCNVector3,
        count: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometrySource").unwrap(), geometrySourceWithVertices : vertices, count : count)
    }
    unsafe fn geometrySourceWithNormals_count_(
        normals: *const SCNVector3,
        count: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometrySource").unwrap(), geometrySourceWithNormals : normals, count : count)
    }
    unsafe fn geometrySourceWithTextureCoordinates_count_(
        texcoord: *const CGPoint,
        count: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometrySource").unwrap(), geometrySourceWithTextureCoordinates : texcoord, count : count)
    }
    unsafe fn geometrySourceWithBuffer_vertexFormat_semantic_vertexCount_dataOffset_dataStride_(
        buffer: *mut u64,
        vertexFormat: MTLVertexFormat,
        semantic: NSString,
        vertexCount: NSInteger,
        offset: NSInteger,
        stride: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometrySource").unwrap(), geometrySourceWithBuffer : buffer, vertexFormat : vertexFormat, semantic : semantic, vertexCount : vertexCount, dataOffset : offset, dataStride : stride)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNGeometryElement(pub id);
impl std::ops::Deref for SCNGeometryElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNGeometryElement {}
impl SCNGeometryElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometryElement").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SCNGeometryElement {}
impl INSObject for SCNGeometryElement {}
impl PNSObject for SCNGeometryElement {}
impl std::convert::TryFrom<NSObject> for SCNGeometryElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNGeometryElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNGeometryElement").unwrap()) };
        if is_kind_of {
            Ok(SCNGeometryElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNGeometryElement")
        }
    }
}
impl ISCNGeometryElement for SCNGeometryElement {}
pub trait ISCNGeometryElement: Sized + std::ops::Deref {
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn primitiveType(&self) -> SCNGeometryPrimitiveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveType)
    }
    unsafe fn primitiveCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveCount)
    }
    unsafe fn hasInterleavedIndicesChannels(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasInterleavedIndicesChannels)
    }
    unsafe fn indicesChannelCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indicesChannelCount)
    }
    unsafe fn bytesPerIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerIndex)
    }
    unsafe fn primitiveRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveRange)
    }
    unsafe fn setPrimitiveRange_(&self, primitiveRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveRange : primitiveRange)
    }
    unsafe fn pointSize(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointSize)
    }
    unsafe fn setPointSize_(&self, pointSize: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointSize : pointSize)
    }
    unsafe fn minimumPointScreenSpaceRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumPointScreenSpaceRadius)
    }
    unsafe fn setMinimumPointScreenSpaceRadius_(&self, minimumPointScreenSpaceRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumPointScreenSpaceRadius : minimumPointScreenSpaceRadius)
    }
    unsafe fn maximumPointScreenSpaceRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumPointScreenSpaceRadius)
    }
    unsafe fn setMaximumPointScreenSpaceRadius_(&self, maximumPointScreenSpaceRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumPointScreenSpaceRadius : maximumPointScreenSpaceRadius)
    }
    unsafe fn geometryElementWithData_primitiveType_primitiveCount_bytesPerIndex_(
        data: NSData,
        primitiveType: SCNGeometryPrimitiveType,
        primitiveCount: NSInteger,
        bytesPerIndex: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometryElement").unwrap(), geometryElementWithData : data, primitiveType : primitiveType, primitiveCount : primitiveCount, bytesPerIndex : bytesPerIndex)
    }
    unsafe fn geometryElementWithData_primitiveType_primitiveCount_indicesChannelCount_interleavedIndicesChannels_bytesPerIndex_(
        data: NSData,
        primitiveType: SCNGeometryPrimitiveType,
        primitiveCount: NSInteger,
        indicesChannelCount: NSInteger,
        interleavedIndicesChannels: BOOL,
        bytesPerIndex: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometryElement").unwrap(), geometryElementWithData : data, primitiveType : primitiveType, primitiveCount : primitiveCount, indicesChannelCount : indicesChannelCount, interleavedIndicesChannels : interleavedIndicesChannels, bytesPerIndex : bytesPerIndex)
    }
    unsafe fn geometryElementWithBuffer_primitiveType_primitiveCount_bytesPerIndex_(
        buffer: *mut u64,
        primitiveType: SCNGeometryPrimitiveType,
        primitiveCount: NSInteger,
        bytesPerIndex: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometryElement").unwrap(), geometryElementWithBuffer : buffer, primitiveType : primitiveType, primitiveCount : primitiveCount, bytesPerIndex : bytesPerIndex)
    }
    unsafe fn geometryElementWithBuffer_primitiveType_primitiveCount_indicesChannelCount_interleavedIndicesChannels_bytesPerIndex_(
        buffer: *mut u64,
        primitiveType: SCNGeometryPrimitiveType,
        primitiveCount: NSInteger,
        indicesChannelCount: NSInteger,
        interleavedIndicesChannels: BOOL,
        bytesPerIndex: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometryElement").unwrap(), geometryElementWithBuffer : buffer, primitiveType : primitiveType, primitiveCount : primitiveCount, indicesChannelCount : indicesChannelCount, interleavedIndicesChannels : interleavedIndicesChannels, bytesPerIndex : bytesPerIndex)
    }
}
pub type SCNTessellationSmoothingMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNGeometryTessellator(pub id);
impl std::ops::Deref for SCNGeometryTessellator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNGeometryTessellator {}
impl SCNGeometryTessellator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometryTessellator").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNGeometryTessellator {}
impl PNSSecureCoding for SCNGeometryTessellator {}
impl INSObject for SCNGeometryTessellator {}
impl PNSObject for SCNGeometryTessellator {}
impl std::convert::TryFrom<NSObject> for SCNGeometryTessellator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNGeometryTessellator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNGeometryTessellator").unwrap()) };
        if is_kind_of {
            Ok(SCNGeometryTessellator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNGeometryTessellator")
        }
    }
}
impl ISCNGeometryTessellator for SCNGeometryTessellator {}
pub trait ISCNGeometryTessellator: Sized + std::ops::Deref {
    unsafe fn tessellationFactorScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationFactorScale)
    }
    unsafe fn setTessellationFactorScale_(&self, tessellationFactorScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationFactorScale : tessellationFactorScale)
    }
    unsafe fn tessellationPartitionMode(&self) -> MTLTessellationPartitionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationPartitionMode)
    }
    unsafe fn setTessellationPartitionMode_(
        &self,
        tessellationPartitionMode: MTLTessellationPartitionMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationPartitionMode : tessellationPartitionMode)
    }
    unsafe fn isAdaptive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAdaptive)
    }
    unsafe fn setAdaptive_(&self, adaptive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdaptive : adaptive)
    }
    unsafe fn isScreenSpace(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isScreenSpace)
    }
    unsafe fn setScreenSpace_(&self, screenSpace: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSpace : screenSpace)
    }
    unsafe fn edgeTessellationFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeTessellationFactor)
    }
    unsafe fn setEdgeTessellationFactor_(&self, edgeTessellationFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeTessellationFactor : edgeTessellationFactor)
    }
    unsafe fn insideTessellationFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insideTessellationFactor)
    }
    unsafe fn setInsideTessellationFactor_(&self, insideTessellationFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInsideTessellationFactor : insideTessellationFactor)
    }
    unsafe fn maximumEdgeLength(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumEdgeLength)
    }
    unsafe fn setMaximumEdgeLength_(&self, maximumEdgeLength: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumEdgeLength : maximumEdgeLength)
    }
    unsafe fn smoothingMode(&self) -> SCNTessellationSmoothingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smoothingMode)
    }
    unsafe fn setSmoothingMode_(&self, smoothingMode: SCNTessellationSmoothingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmoothingMode : smoothingMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPlane(pub id);
impl std::ops::Deref for SCNPlane {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPlane {}
impl SCNPlane {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPlane").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNPlane {}
impl PSCNAnimatable for SCNPlane {}
impl PSCNBoundingVolume for SCNPlane {}
impl PSCNShadable for SCNPlane {}
impl PNSCopying for SCNPlane {}
impl PNSSecureCoding for SCNPlane {}
impl From<SCNPlane> for SCNGeometry {
    fn from(child: SCNPlane) -> SCNGeometry {
        SCNGeometry(child.0)
    }
}
impl std::convert::TryFrom<SCNGeometry> for SCNPlane {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNPlane, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPlane").unwrap()) };
        if is_kind_of {
            Ok(SCNPlane(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNPlane")
        }
    }
}
impl INSObject for SCNPlane {}
impl PNSObject for SCNPlane {}
impl ISCNPlane for SCNPlane {}
pub trait ISCNPlane: Sized + std::ops::Deref {
    unsafe fn width(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn widthSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widthSegmentCount)
    }
    unsafe fn setWidthSegmentCount_(&self, widthSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidthSegmentCount : widthSegmentCount)
    }
    unsafe fn heightSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightSegmentCount)
    }
    unsafe fn setHeightSegmentCount_(&self, heightSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightSegmentCount : heightSegmentCount)
    }
    unsafe fn cornerRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cornerRadius)
    }
    unsafe fn setCornerRadius_(&self, cornerRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCornerRadius : cornerRadius)
    }
    unsafe fn cornerSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cornerSegmentCount)
    }
    unsafe fn setCornerSegmentCount_(&self, cornerSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCornerSegmentCount : cornerSegmentCount)
    }
    unsafe fn planeWithWidth_height_(width: CGFloat, height: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPlane").unwrap(), planeWithWidth : width, height : height)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNBox(pub id);
impl std::ops::Deref for SCNBox {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNBox {}
impl SCNBox {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNBox").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNBox {}
impl PSCNAnimatable for SCNBox {}
impl PSCNBoundingVolume for SCNBox {}
impl PSCNShadable for SCNBox {}
impl PNSCopying for SCNBox {}
impl PNSSecureCoding for SCNBox {}
impl std::convert::TryFrom<SCNGeometry> for SCNBox {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNBox, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNBox").unwrap()) };
        if is_kind_of {
            Ok(SCNBox(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNBox")
        }
    }
}
impl INSObject for SCNBox {}
impl PNSObject for SCNBox {}
impl ISCNBox for SCNBox {}
pub trait ISCNBox: Sized + std::ops::Deref {
    unsafe fn width(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn length(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn setLength_(&self, length: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLength : length)
    }
    unsafe fn chamferRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chamferRadius)
    }
    unsafe fn setChamferRadius_(&self, chamferRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChamferRadius : chamferRadius)
    }
    unsafe fn widthSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widthSegmentCount)
    }
    unsafe fn setWidthSegmentCount_(&self, widthSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidthSegmentCount : widthSegmentCount)
    }
    unsafe fn heightSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightSegmentCount)
    }
    unsafe fn setHeightSegmentCount_(&self, heightSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightSegmentCount : heightSegmentCount)
    }
    unsafe fn lengthSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lengthSegmentCount)
    }
    unsafe fn setLengthSegmentCount_(&self, lengthSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLengthSegmentCount : lengthSegmentCount)
    }
    unsafe fn chamferSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chamferSegmentCount)
    }
    unsafe fn setChamferSegmentCount_(&self, chamferSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChamferSegmentCount : chamferSegmentCount)
    }
    unsafe fn boxWithWidth_height_length_chamferRadius_(
        width: CGFloat,
        height: CGFloat,
        length: CGFloat,
        chamferRadius: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNBox").unwrap(), boxWithWidth : width, height : height, length : length, chamferRadius : chamferRadius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPyramid(pub id);
impl std::ops::Deref for SCNPyramid {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPyramid {}
impl SCNPyramid {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPyramid").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNPyramid {}
impl PSCNAnimatable for SCNPyramid {}
impl PSCNBoundingVolume for SCNPyramid {}
impl PSCNShadable for SCNPyramid {}
impl PNSCopying for SCNPyramid {}
impl PNSSecureCoding for SCNPyramid {}
impl std::convert::TryFrom<SCNGeometry> for SCNPyramid {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNPyramid, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPyramid").unwrap()) };
        if is_kind_of {
            Ok(SCNPyramid(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNPyramid")
        }
    }
}
impl INSObject for SCNPyramid {}
impl PNSObject for SCNPyramid {}
impl ISCNPyramid for SCNPyramid {}
pub trait ISCNPyramid: Sized + std::ops::Deref {
    unsafe fn width(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn length(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn setLength_(&self, length: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLength : length)
    }
    unsafe fn widthSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widthSegmentCount)
    }
    unsafe fn setWidthSegmentCount_(&self, widthSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidthSegmentCount : widthSegmentCount)
    }
    unsafe fn heightSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightSegmentCount)
    }
    unsafe fn setHeightSegmentCount_(&self, heightSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightSegmentCount : heightSegmentCount)
    }
    unsafe fn lengthSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lengthSegmentCount)
    }
    unsafe fn setLengthSegmentCount_(&self, lengthSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLengthSegmentCount : lengthSegmentCount)
    }
    unsafe fn pyramidWithWidth_height_length_(
        width: CGFloat,
        height: CGFloat,
        length: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPyramid").unwrap(), pyramidWithWidth : width, height : height, length : length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNSphere(pub id);
impl std::ops::Deref for SCNSphere {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNSphere {}
impl SCNSphere {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSphere").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNSphere {}
impl PSCNAnimatable for SCNSphere {}
impl PSCNBoundingVolume for SCNSphere {}
impl PSCNShadable for SCNSphere {}
impl PNSCopying for SCNSphere {}
impl PNSSecureCoding for SCNSphere {}
impl std::convert::TryFrom<SCNGeometry> for SCNSphere {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNSphere, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNSphere").unwrap()) };
        if is_kind_of {
            Ok(SCNSphere(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNSphere")
        }
    }
}
impl INSObject for SCNSphere {}
impl PNSObject for SCNSphere {}
impl ISCNSphere for SCNSphere {}
pub trait ISCNSphere: Sized + std::ops::Deref {
    unsafe fn radius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn isGeodesic(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGeodesic)
    }
    unsafe fn setGeodesic_(&self, geodesic: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeodesic : geodesic)
    }
    unsafe fn segmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentCount)
    }
    unsafe fn setSegmentCount_(&self, segmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentCount : segmentCount)
    }
    unsafe fn sphereWithRadius_(radius: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSphere").unwrap(), sphereWithRadius : radius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNCylinder(pub id);
impl std::ops::Deref for SCNCylinder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNCylinder {}
impl SCNCylinder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCylinder").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNCylinder {}
impl PSCNAnimatable for SCNCylinder {}
impl PSCNBoundingVolume for SCNCylinder {}
impl PSCNShadable for SCNCylinder {}
impl PNSCopying for SCNCylinder {}
impl PNSSecureCoding for SCNCylinder {}
impl std::convert::TryFrom<SCNGeometry> for SCNCylinder {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNCylinder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNCylinder").unwrap()) };
        if is_kind_of {
            Ok(SCNCylinder(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNCylinder")
        }
    }
}
impl INSObject for SCNCylinder {}
impl PNSObject for SCNCylinder {}
impl ISCNCylinder for SCNCylinder {}
pub trait ISCNCylinder: Sized + std::ops::Deref {
    unsafe fn radius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn height(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn radialSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radialSegmentCount)
    }
    unsafe fn setRadialSegmentCount_(&self, radialSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadialSegmentCount : radialSegmentCount)
    }
    unsafe fn heightSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightSegmentCount)
    }
    unsafe fn setHeightSegmentCount_(&self, heightSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightSegmentCount : heightSegmentCount)
    }
    unsafe fn cylinderWithRadius_height_(radius: CGFloat, height: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCylinder").unwrap(), cylinderWithRadius : radius, height : height)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNCone(pub id);
impl std::ops::Deref for SCNCone {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNCone {}
impl SCNCone {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCone").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNCone {}
impl PSCNAnimatable for SCNCone {}
impl PSCNBoundingVolume for SCNCone {}
impl PSCNShadable for SCNCone {}
impl PNSCopying for SCNCone {}
impl PNSSecureCoding for SCNCone {}
impl std::convert::TryFrom<SCNGeometry> for SCNCone {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNCone, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNCone").unwrap()) };
        if is_kind_of {
            Ok(SCNCone(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNCone")
        }
    }
}
impl INSObject for SCNCone {}
impl PNSObject for SCNCone {}
impl ISCNCone for SCNCone {}
pub trait ISCNCone: Sized + std::ops::Deref {
    unsafe fn topRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRadius)
    }
    unsafe fn setTopRadius_(&self, topRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopRadius : topRadius)
    }
    unsafe fn bottomRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRadius)
    }
    unsafe fn setBottomRadius_(&self, bottomRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomRadius : bottomRadius)
    }
    unsafe fn height(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn radialSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radialSegmentCount)
    }
    unsafe fn setRadialSegmentCount_(&self, radialSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadialSegmentCount : radialSegmentCount)
    }
    unsafe fn heightSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightSegmentCount)
    }
    unsafe fn setHeightSegmentCount_(&self, heightSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightSegmentCount : heightSegmentCount)
    }
    unsafe fn coneWithTopRadius_bottomRadius_height_(
        topRadius: CGFloat,
        bottomRadius: CGFloat,
        height: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCone").unwrap(), coneWithTopRadius : topRadius, bottomRadius : bottomRadius, height : height)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNTube(pub id);
impl std::ops::Deref for SCNTube {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNTube {}
impl SCNTube {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTube").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNTube {}
impl PSCNAnimatable for SCNTube {}
impl PSCNBoundingVolume for SCNTube {}
impl PSCNShadable for SCNTube {}
impl PNSCopying for SCNTube {}
impl PNSSecureCoding for SCNTube {}
impl std::convert::TryFrom<SCNGeometry> for SCNTube {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNTube, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNTube").unwrap()) };
        if is_kind_of {
            Ok(SCNTube(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNTube")
        }
    }
}
impl INSObject for SCNTube {}
impl PNSObject for SCNTube {}
impl ISCNTube for SCNTube {}
pub trait ISCNTube: Sized + std::ops::Deref {
    unsafe fn innerRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, innerRadius)
    }
    unsafe fn setInnerRadius_(&self, innerRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInnerRadius : innerRadius)
    }
    unsafe fn outerRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerRadius)
    }
    unsafe fn setOuterRadius_(&self, outerRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOuterRadius : outerRadius)
    }
    unsafe fn height(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn radialSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radialSegmentCount)
    }
    unsafe fn setRadialSegmentCount_(&self, radialSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadialSegmentCount : radialSegmentCount)
    }
    unsafe fn heightSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightSegmentCount)
    }
    unsafe fn setHeightSegmentCount_(&self, heightSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightSegmentCount : heightSegmentCount)
    }
    unsafe fn tubeWithInnerRadius_outerRadius_height_(
        innerRadius: CGFloat,
        outerRadius: CGFloat,
        height: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTube").unwrap(), tubeWithInnerRadius : innerRadius, outerRadius : outerRadius, height : height)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNCapsule(pub id);
impl std::ops::Deref for SCNCapsule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNCapsule {}
impl SCNCapsule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCapsule").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNCapsule {}
impl PSCNAnimatable for SCNCapsule {}
impl PSCNBoundingVolume for SCNCapsule {}
impl PSCNShadable for SCNCapsule {}
impl PNSCopying for SCNCapsule {}
impl PNSSecureCoding for SCNCapsule {}
impl std::convert::TryFrom<SCNGeometry> for SCNCapsule {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNCapsule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNCapsule").unwrap()) };
        if is_kind_of {
            Ok(SCNCapsule(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNCapsule")
        }
    }
}
impl INSObject for SCNCapsule {}
impl PNSObject for SCNCapsule {}
impl ISCNCapsule for SCNCapsule {}
pub trait ISCNCapsule: Sized + std::ops::Deref {
    unsafe fn capRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capRadius)
    }
    unsafe fn setCapRadius_(&self, capRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCapRadius : capRadius)
    }
    unsafe fn height(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn radialSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radialSegmentCount)
    }
    unsafe fn setRadialSegmentCount_(&self, radialSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadialSegmentCount : radialSegmentCount)
    }
    unsafe fn heightSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightSegmentCount)
    }
    unsafe fn setHeightSegmentCount_(&self, heightSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightSegmentCount : heightSegmentCount)
    }
    unsafe fn capSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capSegmentCount)
    }
    unsafe fn setCapSegmentCount_(&self, capSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCapSegmentCount : capSegmentCount)
    }
    unsafe fn capsuleWithCapRadius_height_(capRadius: CGFloat, height: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCapsule").unwrap(), capsuleWithCapRadius : capRadius, height : height)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNTorus(pub id);
impl std::ops::Deref for SCNTorus {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNTorus {}
impl SCNTorus {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTorus").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNTorus {}
impl PSCNAnimatable for SCNTorus {}
impl PSCNBoundingVolume for SCNTorus {}
impl PSCNShadable for SCNTorus {}
impl PNSCopying for SCNTorus {}
impl PNSSecureCoding for SCNTorus {}
impl std::convert::TryFrom<SCNGeometry> for SCNTorus {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNTorus, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNTorus").unwrap()) };
        if is_kind_of {
            Ok(SCNTorus(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNTorus")
        }
    }
}
impl INSObject for SCNTorus {}
impl PNSObject for SCNTorus {}
impl ISCNTorus for SCNTorus {}
pub trait ISCNTorus: Sized + std::ops::Deref {
    unsafe fn ringRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringRadius)
    }
    unsafe fn setRingRadius_(&self, ringRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingRadius : ringRadius)
    }
    unsafe fn pipeRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pipeRadius)
    }
    unsafe fn setPipeRadius_(&self, pipeRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPipeRadius : pipeRadius)
    }
    unsafe fn ringSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringSegmentCount)
    }
    unsafe fn setRingSegmentCount_(&self, ringSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingSegmentCount : ringSegmentCount)
    }
    unsafe fn pipeSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pipeSegmentCount)
    }
    unsafe fn setPipeSegmentCount_(&self, pipeSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPipeSegmentCount : pipeSegmentCount)
    }
    unsafe fn torusWithRingRadius_pipeRadius_(
        ringRadius: CGFloat,
        pipeRadius: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTorus").unwrap(), torusWithRingRadius : ringRadius, pipeRadius : pipeRadius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNFloor(pub id);
impl std::ops::Deref for SCNFloor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNFloor {}
impl SCNFloor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNFloor").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNFloor {}
impl PSCNAnimatable for SCNFloor {}
impl PSCNBoundingVolume for SCNFloor {}
impl PSCNShadable for SCNFloor {}
impl PNSCopying for SCNFloor {}
impl PNSSecureCoding for SCNFloor {}
impl std::convert::TryFrom<SCNGeometry> for SCNFloor {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNFloor, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNFloor").unwrap()) };
        if is_kind_of {
            Ok(SCNFloor(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNFloor")
        }
    }
}
impl INSObject for SCNFloor {}
impl PNSObject for SCNFloor {}
impl ISCNFloor for SCNFloor {}
pub trait ISCNFloor: Sized + std::ops::Deref {
    unsafe fn reflectivity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflectivity)
    }
    unsafe fn setReflectivity_(&self, reflectivity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReflectivity : reflectivity)
    }
    unsafe fn reflectionFalloffStart(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflectionFalloffStart)
    }
    unsafe fn setReflectionFalloffStart_(&self, reflectionFalloffStart: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReflectionFalloffStart : reflectionFalloffStart)
    }
    unsafe fn reflectionFalloffEnd(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflectionFalloffEnd)
    }
    unsafe fn setReflectionFalloffEnd_(&self, reflectionFalloffEnd: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReflectionFalloffEnd : reflectionFalloffEnd)
    }
    unsafe fn reflectionCategoryBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflectionCategoryBitMask)
    }
    unsafe fn setReflectionCategoryBitMask_(&self, reflectionCategoryBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReflectionCategoryBitMask : reflectionCategoryBitMask)
    }
    unsafe fn width(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn length(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn setLength_(&self, length: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLength : length)
    }
    unsafe fn reflectionResolutionScaleFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflectionResolutionScaleFactor)
    }
    unsafe fn setReflectionResolutionScaleFactor_(&self, reflectionResolutionScaleFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReflectionResolutionScaleFactor : reflectionResolutionScaleFactor)
    }
    unsafe fn floor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNFloor").unwrap(), floor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNText(pub id);
impl std::ops::Deref for SCNText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNText {}
impl SCNText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNText").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNText {}
impl PSCNAnimatable for SCNText {}
impl PSCNBoundingVolume for SCNText {}
impl PSCNShadable for SCNText {}
impl PNSCopying for SCNText {}
impl PNSSecureCoding for SCNText {}
impl std::convert::TryFrom<SCNGeometry> for SCNText {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNText, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNText").unwrap()) };
        if is_kind_of {
            Ok(SCNText(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNText")
        }
    }
}
impl INSObject for SCNText {}
impl PNSObject for SCNText {}
impl ISCNText for SCNText {}
pub trait ISCNText: Sized + std::ops::Deref {
    unsafe fn extrusionDepth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrusionDepth)
    }
    unsafe fn setExtrusionDepth_(&self, extrusionDepth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrusionDepth : extrusionDepth)
    }
    unsafe fn string(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn setString_(&self, string: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setString : string)
    }
    unsafe fn font(&self) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, font)
    }
    unsafe fn setFont_(&self, font: NSFont)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFont : font)
    }
    unsafe fn isWrapped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isWrapped)
    }
    unsafe fn setWrapped_(&self, wrapped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWrapped : wrapped)
    }
    unsafe fn containerFrame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerFrame)
    }
    unsafe fn setContainerFrame_(&self, containerFrame: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainerFrame : containerFrame)
    }
    unsafe fn textSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textSize)
    }
    unsafe fn truncationMode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, truncationMode)
    }
    unsafe fn setTruncationMode_(&self, truncationMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTruncationMode : truncationMode)
    }
    unsafe fn alignmentMode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignmentMode)
    }
    unsafe fn setAlignmentMode_(&self, alignmentMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlignmentMode : alignmentMode)
    }
    unsafe fn chamferRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chamferRadius)
    }
    unsafe fn setChamferRadius_(&self, chamferRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChamferRadius : chamferRadius)
    }
    unsafe fn chamferSegmentCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chamferSegmentCount)
    }
    unsafe fn setChamferSegmentCount_(&self, chamferSegmentCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChamferSegmentCount : chamferSegmentCount)
    }
    unsafe fn chamferProfile(&self) -> NSBezierPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chamferProfile)
    }
    unsafe fn setChamferProfile_(&self, chamferProfile: NSBezierPath)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChamferProfile : chamferProfile)
    }
    unsafe fn flatness(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flatness)
    }
    unsafe fn setFlatness_(&self, flatness: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlatness : flatness)
    }
    unsafe fn textWithString_extrusionDepth_(string: id, extrusionDepth: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNText").unwrap(), textWithString : string, extrusionDepth : extrusionDepth)
    }
}
pub type SCNChamferMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNShape(pub id);
impl std::ops::Deref for SCNShape {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNShape {}
impl SCNShape {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNShape").unwrap(), alloc) })
    }
}
impl ISCNGeometry for SCNShape {}
impl PSCNAnimatable for SCNShape {}
impl PSCNBoundingVolume for SCNShape {}
impl PSCNShadable for SCNShape {}
impl PNSCopying for SCNShape {}
impl PNSSecureCoding for SCNShape {}
impl std::convert::TryFrom<SCNGeometry> for SCNShape {
    type Error = &'static str;
    fn try_from(parent: SCNGeometry) -> Result<SCNShape, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNShape").unwrap()) };
        if is_kind_of {
            Ok(SCNShape(parent.0))
        } else {
            Err("This SCNGeometry cannot be downcasted to SCNShape")
        }
    }
}
impl INSObject for SCNShape {}
impl PNSObject for SCNShape {}
impl ISCNShape for SCNShape {}
pub trait ISCNShape: Sized + std::ops::Deref {
    unsafe fn path(&self) -> NSBezierPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn setPath_(&self, path: NSBezierPath)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPath : path)
    }
    unsafe fn extrusionDepth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrusionDepth)
    }
    unsafe fn setExtrusionDepth_(&self, extrusionDepth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrusionDepth : extrusionDepth)
    }
    unsafe fn chamferMode(&self) -> SCNChamferMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chamferMode)
    }
    unsafe fn setChamferMode_(&self, chamferMode: SCNChamferMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChamferMode : chamferMode)
    }
    unsafe fn chamferRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chamferRadius)
    }
    unsafe fn setChamferRadius_(&self, chamferRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChamferRadius : chamferRadius)
    }
    unsafe fn chamferProfile(&self) -> NSBezierPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chamferProfile)
    }
    unsafe fn setChamferProfile_(&self, chamferProfile: NSBezierPath)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChamferProfile : chamferProfile)
    }
    unsafe fn shapeWithPath_extrusionDepth_(
        path: NSBezierPath,
        extrusionDepth: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNShape").unwrap(), shapeWithPath : path, extrusionDepth : extrusionDepth)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNTransaction(pub id);
impl std::ops::Deref for SCNTransaction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNTransaction {}
impl SCNTransaction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), alloc) })
    }
}
impl INSObject for SCNTransaction {}
impl PNSObject for SCNTransaction {}
impl std::convert::TryFrom<NSObject> for SCNTransaction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNTransaction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap()) };
        if is_kind_of {
            Ok(SCNTransaction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNTransaction")
        }
    }
}
impl ISCNTransaction for SCNTransaction {}
pub trait ISCNTransaction: Sized + std::ops::Deref {
    unsafe fn begin()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), begin)
    }
    unsafe fn commit()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), commit)
    }
    unsafe fn flush()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), flush)
    }
    unsafe fn lock()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), lock)
    }
    unsafe fn unlock()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), unlock)
    }
    unsafe fn valueForKey_(key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), valueForKey : key)
    }
    unsafe fn setValue_forKey_(value: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), setValue : value, forKey : key)
    }
    unsafe fn animationDuration() -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), animationDuration)
    }
    unsafe fn setAnimationDuration_(animationDuration: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), setAnimationDuration : animationDuration)
    }
    unsafe fn animationTimingFunction() -> CAMediaTimingFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), animationTimingFunction)
    }
    unsafe fn setAnimationTimingFunction_(animationTimingFunction: CAMediaTimingFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), setAnimationTimingFunction : animationTimingFunction)
    }
    unsafe fn disableActions() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), disableActions)
    }
    unsafe fn setDisableActions_(disableActions: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), setDisableActions : disableActions)
    }
    unsafe fn completionBlock() -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), completionBlock)
    }
    unsafe fn setCompletionBlock_(completionBlock: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransaction").unwrap(), setCompletionBlock : completionBlock)
    }
}
pub type SCNMorpherCalculationMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNMorpher(pub id);
impl std::ops::Deref for SCNMorpher {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNMorpher {}
impl SCNMorpher {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMorpher").unwrap(), alloc) })
    }
}
impl PSCNAnimatable for SCNMorpher {}
impl PNSSecureCoding for SCNMorpher {}
impl INSObject for SCNMorpher {}
impl PNSObject for SCNMorpher {}
impl std::convert::TryFrom<NSObject> for SCNMorpher {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNMorpher, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNMorpher").unwrap()) };
        if is_kind_of {
            Ok(SCNMorpher(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNMorpher")
        }
    }
}
impl ISCNMorpher for SCNMorpher {}
pub trait ISCNMorpher: Sized + std::ops::Deref {
    unsafe fn setWeight_forTargetAtIndex_(&self, weight: CGFloat, targetIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeight : weight, forTargetAtIndex : targetIndex)
    }
    unsafe fn weightForTargetAtIndex_(&self, targetIndex: NSUInteger) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, weightForTargetAtIndex : targetIndex)
    }
    unsafe fn setWeight_forTargetNamed_(&self, weight: CGFloat, targetName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeight : weight, forTargetNamed : targetName)
    }
    unsafe fn weightForTargetNamed_(&self, targetName: NSString) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, weightForTargetNamed : targetName)
    }
    unsafe fn targets(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targets)
    }
    unsafe fn setTargets_(&self, targets: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargets : targets)
    }
    unsafe fn weights(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weights)
    }
    unsafe fn setWeights_(&self, weights: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeights : weights)
    }
    unsafe fn calculationMode(&self) -> SCNMorpherCalculationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calculationMode)
    }
    unsafe fn setCalculationMode_(&self, calculationMode: SCNMorpherCalculationMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalculationMode : calculationMode)
    }
    unsafe fn unifiesNormals(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unifiesNormals)
    }
    unsafe fn setUnifiesNormals_(&self, unifiesNormals: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnifiesNormals : unifiesNormals)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNSkinner(pub id);
impl std::ops::Deref for SCNSkinner {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNSkinner {}
impl SCNSkinner {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSkinner").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SCNSkinner {}
impl INSObject for SCNSkinner {}
impl PNSObject for SCNSkinner {}
impl std::convert::TryFrom<NSObject> for SCNSkinner {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNSkinner, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNSkinner").unwrap()) };
        if is_kind_of {
            Ok(SCNSkinner(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNSkinner")
        }
    }
}
impl ISCNSkinner for SCNSkinner {}
pub trait ISCNSkinner: Sized + std::ops::Deref {
    unsafe fn skeleton(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skeleton)
    }
    unsafe fn setSkeleton_(&self, skeleton: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSkeleton : skeleton)
    }
    unsafe fn baseGeometry(&self) -> SCNGeometry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseGeometry)
    }
    unsafe fn setBaseGeometry_(&self, baseGeometry: SCNGeometry)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaseGeometry : baseGeometry)
    }
    unsafe fn baseGeometryBindTransform(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseGeometryBindTransform)
    }
    unsafe fn setBaseGeometryBindTransform_(&self, baseGeometryBindTransform: SCNMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaseGeometryBindTransform : baseGeometryBindTransform)
    }
    unsafe fn boneInverseBindTransforms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boneInverseBindTransforms)
    }
    unsafe fn bones(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bones)
    }
    unsafe fn boneWeights(&self) -> SCNGeometrySource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boneWeights)
    }
    unsafe fn boneIndices(&self) -> SCNGeometrySource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boneIndices)
    }
    unsafe fn skinnerWithBaseGeometry_bones_boneInverseBindTransforms_boneWeights_boneIndices_(
        baseGeometry: SCNGeometry,
        bones: NSArray,
        boneInverseBindTransforms: NSArray,
        boneWeights: SCNGeometrySource,
        boneIndices: SCNGeometrySource,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSkinner").unwrap(), skinnerWithBaseGeometry : baseGeometry, bones : bones, boneInverseBindTransforms : boneInverseBindTransforms, boneWeights : boneWeights, boneIndices : boneIndices)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNConstraint(pub id);
impl std::ops::Deref for SCNConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNConstraint {}
impl SCNConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNConstraint").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNConstraint {}
impl PNSSecureCoding for SCNConstraint {}
impl PSCNAnimatable for SCNConstraint {}
impl INSObject for SCNConstraint {}
impl PNSObject for SCNConstraint {}
impl std::convert::TryFrom<NSObject> for SCNConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNConstraint")
        }
    }
}
impl ISCNConstraint for SCNConstraint {}
pub trait ISCNConstraint: Sized + std::ops::Deref {
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
    unsafe fn influenceFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, influenceFactor)
    }
    unsafe fn setInfluenceFactor_(&self, influenceFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInfluenceFactor : influenceFactor)
    }
    unsafe fn isIncremental(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIncremental)
    }
    unsafe fn setIncremental_(&self, incremental: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncremental : incremental)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNLookAtConstraint(pub id);
impl std::ops::Deref for SCNLookAtConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNLookAtConstraint {}
impl SCNLookAtConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLookAtConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNLookAtConstraint {}
impl PNSCopying for SCNLookAtConstraint {}
impl PNSSecureCoding for SCNLookAtConstraint {}
impl PSCNAnimatable for SCNLookAtConstraint {}
impl From<SCNLookAtConstraint> for SCNConstraint {
    fn from(child: SCNLookAtConstraint) -> SCNConstraint {
        SCNConstraint(child.0)
    }
}
impl std::convert::TryFrom<SCNConstraint> for SCNLookAtConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNLookAtConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNLookAtConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNLookAtConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNLookAtConstraint")
        }
    }
}
impl INSObject for SCNLookAtConstraint {}
impl PNSObject for SCNLookAtConstraint {}
impl ISCNLookAtConstraint for SCNLookAtConstraint {}
pub trait ISCNLookAtConstraint: Sized + std::ops::Deref {
    unsafe fn target(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn targetOffset(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetOffset)
    }
    unsafe fn setTargetOffset_(&self, targetOffset: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetOffset : targetOffset)
    }
    unsafe fn localFront(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localFront)
    }
    unsafe fn setLocalFront_(&self, localFront: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalFront : localFront)
    }
    unsafe fn worldUp(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldUp)
    }
    unsafe fn setWorldUp_(&self, worldUp: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorldUp : worldUp)
    }
    unsafe fn gimbalLockEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gimbalLockEnabled)
    }
    unsafe fn setGimbalLockEnabled_(&self, gimbalLockEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGimbalLockEnabled : gimbalLockEnabled)
    }
    unsafe fn lookAtConstraintWithTarget_(target: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLookAtConstraint").unwrap(), lookAtConstraintWithTarget : target)
    }
}
pub type SCNBillboardAxis = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNBillboardConstraint(pub id);
impl std::ops::Deref for SCNBillboardConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNBillboardConstraint {}
impl SCNBillboardConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNBillboardConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNBillboardConstraint {}
impl PNSCopying for SCNBillboardConstraint {}
impl PNSSecureCoding for SCNBillboardConstraint {}
impl PSCNAnimatable for SCNBillboardConstraint {}
impl std::convert::TryFrom<SCNConstraint> for SCNBillboardConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNBillboardConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNBillboardConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNBillboardConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNBillboardConstraint")
        }
    }
}
impl INSObject for SCNBillboardConstraint {}
impl PNSObject for SCNBillboardConstraint {}
impl ISCNBillboardConstraint for SCNBillboardConstraint {}
pub trait ISCNBillboardConstraint: Sized + std::ops::Deref {
    unsafe fn freeAxes(&self) -> SCNBillboardAxis
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, freeAxes)
    }
    unsafe fn setFreeAxes_(&self, freeAxes: SCNBillboardAxis)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFreeAxes : freeAxes)
    }
    unsafe fn billboardConstraint() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNBillboardConstraint").unwrap(), billboardConstraint)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNTransformConstraint(pub id);
impl std::ops::Deref for SCNTransformConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNTransformConstraint {}
impl SCNTransformConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransformConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNTransformConstraint {}
impl PNSCopying for SCNTransformConstraint {}
impl PNSSecureCoding for SCNTransformConstraint {}
impl PSCNAnimatable for SCNTransformConstraint {}
impl std::convert::TryFrom<SCNConstraint> for SCNTransformConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNTransformConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNTransformConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNTransformConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNTransformConstraint")
        }
    }
}
impl INSObject for SCNTransformConstraint {}
impl PNSObject for SCNTransformConstraint {}
impl ISCNTransformConstraint for SCNTransformConstraint {}
pub trait ISCNTransformConstraint: Sized + std::ops::Deref {
    unsafe fn transformConstraintInWorldSpace_withBlock_(
        world: BOOL,
        block: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransformConstraint").unwrap(), transformConstraintInWorldSpace : world, withBlock : block)
    }
    unsafe fn positionConstraintInWorldSpace_withBlock_(
        world: BOOL,
        block: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransformConstraint").unwrap(), positionConstraintInWorldSpace : world, withBlock : block)
    }
    unsafe fn orientationConstraintInWorldSpace_withBlock_(
        world: BOOL,
        block: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNTransformConstraint").unwrap(), orientationConstraintInWorldSpace : world, withBlock : block)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNIKConstraint(pub id);
impl std::ops::Deref for SCNIKConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNIKConstraint {}
impl SCNIKConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNIKConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNIKConstraint {}
impl PNSCopying for SCNIKConstraint {}
impl PNSSecureCoding for SCNIKConstraint {}
impl PSCNAnimatable for SCNIKConstraint {}
impl std::convert::TryFrom<SCNConstraint> for SCNIKConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNIKConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNIKConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNIKConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNIKConstraint")
        }
    }
}
impl INSObject for SCNIKConstraint {}
impl PNSObject for SCNIKConstraint {}
impl ISCNIKConstraint for SCNIKConstraint {}
pub trait ISCNIKConstraint: Sized + std::ops::Deref {
    unsafe fn initWithChainRootNode_(&self, chainRootNode: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChainRootNode : chainRootNode)
    }
    unsafe fn setMaxAllowedRotationAngle_forJoint_(&self, angle: CGFloat, node: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxAllowedRotationAngle : angle, forJoint : node)
    }
    unsafe fn maxAllowedRotationAngleForJoint_(&self, node: SCNNode) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxAllowedRotationAngleForJoint : node)
    }
    unsafe fn chainRootNode(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chainRootNode)
    }
    unsafe fn targetPosition(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetPosition)
    }
    unsafe fn setTargetPosition_(&self, targetPosition: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetPosition : targetPosition)
    }
    unsafe fn inverseKinematicsConstraintWithChainRootNode_(chainRootNode: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNIKConstraint").unwrap(), inverseKinematicsConstraintWithChainRootNode : chainRootNode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNDistanceConstraint(pub id);
impl std::ops::Deref for SCNDistanceConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNDistanceConstraint {}
impl SCNDistanceConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNDistanceConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNDistanceConstraint {}
impl PNSCopying for SCNDistanceConstraint {}
impl PNSSecureCoding for SCNDistanceConstraint {}
impl PSCNAnimatable for SCNDistanceConstraint {}
impl std::convert::TryFrom<SCNConstraint> for SCNDistanceConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNDistanceConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNDistanceConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNDistanceConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNDistanceConstraint")
        }
    }
}
impl INSObject for SCNDistanceConstraint {}
impl PNSObject for SCNDistanceConstraint {}
impl ISCNDistanceConstraint for SCNDistanceConstraint {}
pub trait ISCNDistanceConstraint: Sized + std::ops::Deref {
    unsafe fn target(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn minimumDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumDistance)
    }
    unsafe fn setMinimumDistance_(&self, minimumDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumDistance : minimumDistance)
    }
    unsafe fn maximumDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDistance)
    }
    unsafe fn setMaximumDistance_(&self, maximumDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumDistance : maximumDistance)
    }
    unsafe fn distanceConstraintWithTarget_(target: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNDistanceConstraint").unwrap(), distanceConstraintWithTarget : target)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNReplicatorConstraint(pub id);
impl std::ops::Deref for SCNReplicatorConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNReplicatorConstraint {}
impl SCNReplicatorConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNReplicatorConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNReplicatorConstraint {}
impl PNSCopying for SCNReplicatorConstraint {}
impl PNSSecureCoding for SCNReplicatorConstraint {}
impl PSCNAnimatable for SCNReplicatorConstraint {}
impl std::convert::TryFrom<SCNConstraint> for SCNReplicatorConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNReplicatorConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNReplicatorConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNReplicatorConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNReplicatorConstraint")
        }
    }
}
impl INSObject for SCNReplicatorConstraint {}
impl PNSObject for SCNReplicatorConstraint {}
impl ISCNReplicatorConstraint for SCNReplicatorConstraint {}
pub trait ISCNReplicatorConstraint: Sized + std::ops::Deref {
    unsafe fn target(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn replicatesOrientation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replicatesOrientation)
    }
    unsafe fn setReplicatesOrientation_(&self, replicatesOrientation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplicatesOrientation : replicatesOrientation)
    }
    unsafe fn replicatesPosition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replicatesPosition)
    }
    unsafe fn setReplicatesPosition_(&self, replicatesPosition: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplicatesPosition : replicatesPosition)
    }
    unsafe fn replicatesScale(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replicatesScale)
    }
    unsafe fn setReplicatesScale_(&self, replicatesScale: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplicatesScale : replicatesScale)
    }
    unsafe fn orientationOffset(&self) -> SCNQuaternion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientationOffset)
    }
    unsafe fn setOrientationOffset_(&self, orientationOffset: SCNQuaternion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientationOffset : orientationOffset)
    }
    unsafe fn positionOffset(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, positionOffset)
    }
    unsafe fn setPositionOffset_(&self, positionOffset: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPositionOffset : positionOffset)
    }
    unsafe fn scaleOffset(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleOffset)
    }
    unsafe fn setScaleOffset_(&self, scaleOffset: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleOffset : scaleOffset)
    }
    unsafe fn replicatorConstraintWithTarget_(target: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNReplicatorConstraint").unwrap(), replicatorConstraintWithTarget : target)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNAccelerationConstraint(pub id);
impl std::ops::Deref for SCNAccelerationConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNAccelerationConstraint {}
impl SCNAccelerationConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAccelerationConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNAccelerationConstraint {}
impl PNSCopying for SCNAccelerationConstraint {}
impl PNSSecureCoding for SCNAccelerationConstraint {}
impl PSCNAnimatable for SCNAccelerationConstraint {}
impl std::convert::TryFrom<SCNConstraint> for SCNAccelerationConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNAccelerationConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNAccelerationConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNAccelerationConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNAccelerationConstraint")
        }
    }
}
impl INSObject for SCNAccelerationConstraint {}
impl PNSObject for SCNAccelerationConstraint {}
impl ISCNAccelerationConstraint for SCNAccelerationConstraint {}
pub trait ISCNAccelerationConstraint: Sized + std::ops::Deref {
    unsafe fn maximumLinearAcceleration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumLinearAcceleration)
    }
    unsafe fn setMaximumLinearAcceleration_(&self, maximumLinearAcceleration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumLinearAcceleration : maximumLinearAcceleration)
    }
    unsafe fn maximumLinearVelocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumLinearVelocity)
    }
    unsafe fn setMaximumLinearVelocity_(&self, maximumLinearVelocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumLinearVelocity : maximumLinearVelocity)
    }
    unsafe fn decelerationDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decelerationDistance)
    }
    unsafe fn setDecelerationDistance_(&self, decelerationDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDecelerationDistance : decelerationDistance)
    }
    unsafe fn damping(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, damping)
    }
    unsafe fn setDamping_(&self, damping: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDamping : damping)
    }
    unsafe fn accelerationConstraint() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAccelerationConstraint").unwrap(), accelerationConstraint)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNSliderConstraint(pub id);
impl std::ops::Deref for SCNSliderConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNSliderConstraint {}
impl SCNSliderConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSliderConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNSliderConstraint {}
impl PNSCopying for SCNSliderConstraint {}
impl PNSSecureCoding for SCNSliderConstraint {}
impl PSCNAnimatable for SCNSliderConstraint {}
impl std::convert::TryFrom<SCNConstraint> for SCNSliderConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNSliderConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNSliderConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNSliderConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNSliderConstraint")
        }
    }
}
impl INSObject for SCNSliderConstraint {}
impl PNSObject for SCNSliderConstraint {}
impl ISCNSliderConstraint for SCNSliderConstraint {}
pub trait ISCNSliderConstraint: Sized + std::ops::Deref {
    unsafe fn collisionCategoryBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collisionCategoryBitMask)
    }
    unsafe fn setCollisionCategoryBitMask_(&self, collisionCategoryBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCollisionCategoryBitMask : collisionCategoryBitMask)
    }
    unsafe fn radius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn offset(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn sliderConstraint() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNSliderConstraint").unwrap(), sliderConstraint)
    }
}
pub trait PSCNAvoidOccluderConstraintDelegate: Sized + std::ops::Deref {
    unsafe fn avoidOccluderConstraint_shouldAvoidOccluder_forNode_(
        &self,
        constraint: SCNAvoidOccluderConstraint,
        occluder: SCNNode,
        node: SCNNode,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, avoidOccluderConstraint : constraint, shouldAvoidOccluder : occluder, forNode : node)
    }
    unsafe fn avoidOccluderConstraint_didAvoidOccluder_forNode_(
        &self,
        constraint: SCNAvoidOccluderConstraint,
        occluder: SCNNode,
        node: SCNNode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, avoidOccluderConstraint : constraint, didAvoidOccluder : occluder, forNode : node)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNAvoidOccluderConstraint(pub id);
impl std::ops::Deref for SCNAvoidOccluderConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNAvoidOccluderConstraint {}
impl SCNAvoidOccluderConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAvoidOccluderConstraint").unwrap(), alloc) })
    }
}
impl ISCNConstraint for SCNAvoidOccluderConstraint {}
impl PNSCopying for SCNAvoidOccluderConstraint {}
impl PNSSecureCoding for SCNAvoidOccluderConstraint {}
impl PSCNAnimatable for SCNAvoidOccluderConstraint {}
impl std::convert::TryFrom<SCNConstraint> for SCNAvoidOccluderConstraint {
    type Error = &'static str;
    fn try_from(parent: SCNConstraint) -> Result<SCNAvoidOccluderConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNAvoidOccluderConstraint").unwrap()) };
        if is_kind_of {
            Ok(SCNAvoidOccluderConstraint(parent.0))
        } else {
            Err("This SCNConstraint cannot be downcasted to SCNAvoidOccluderConstraint")
        }
    }
}
impl INSObject for SCNAvoidOccluderConstraint {}
impl PNSObject for SCNAvoidOccluderConstraint {}
impl ISCNAvoidOccluderConstraint for SCNAvoidOccluderConstraint {}
pub trait ISCNAvoidOccluderConstraint: Sized + std::ops::Deref {
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
    unsafe fn target(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn occluderCategoryBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, occluderCategoryBitMask)
    }
    unsafe fn setOccluderCategoryBitMask_(&self, occluderCategoryBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOccluderCategoryBitMask : occluderCategoryBitMask)
    }
    unsafe fn bias(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bias)
    }
    unsafe fn setBias_(&self, bias: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBias : bias)
    }
    unsafe fn avoidOccluderConstraintWithTarget_(target: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAvoidOccluderConstraint").unwrap(), avoidOccluderConstraintWithTarget : target)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNLevelOfDetail(pub id);
impl std::ops::Deref for SCNLevelOfDetail {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNLevelOfDetail {}
impl SCNLevelOfDetail {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLevelOfDetail").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNLevelOfDetail {}
impl PNSSecureCoding for SCNLevelOfDetail {}
impl INSObject for SCNLevelOfDetail {}
impl PNSObject for SCNLevelOfDetail {}
impl std::convert::TryFrom<NSObject> for SCNLevelOfDetail {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNLevelOfDetail, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNLevelOfDetail").unwrap()) };
        if is_kind_of {
            Ok(SCNLevelOfDetail(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNLevelOfDetail")
        }
    }
}
impl ISCNLevelOfDetail for SCNLevelOfDetail {}
pub trait ISCNLevelOfDetail: Sized + std::ops::Deref {
    unsafe fn geometry(&self) -> SCNGeometry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometry)
    }
    unsafe fn screenSpaceRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSpaceRadius)
    }
    unsafe fn worldSpaceDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldSpaceDistance)
    }
    unsafe fn levelOfDetailWithGeometry_screenSpaceRadius_(
        geometry: SCNGeometry,
        radius: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLevelOfDetail").unwrap(), levelOfDetailWithGeometry : geometry, screenSpaceRadius : radius)
    }
    unsafe fn levelOfDetailWithGeometry_worldSpaceDistance_(
        geometry: SCNGeometry,
        distance: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLevelOfDetail").unwrap(), levelOfDetailWithGeometry : geometry, worldSpaceDistance : distance)
    }
}
pub type SCNParticleProperty = NSString;
pub type SCNParticleEventBlock = *mut ::std::os::raw::c_void;
pub type SCNParticleModifierBlock = *mut ::std::os::raw::c_void;
pub type SCNParticleSortingMode = NSInteger;
pub type SCNParticleBlendMode = NSInteger;
pub type SCNParticleOrientationMode = NSInteger;
pub type SCNParticleBirthLocation = NSInteger;
pub type SCNParticleBirthDirection = NSInteger;
pub type SCNParticleImageSequenceAnimationMode = NSInteger;
pub type SCNParticleInputMode = NSInteger;
pub type SCNParticleModifierStage = NSInteger;
pub type SCNParticleEvent = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNParticlePropertyController(pub id);
impl std::ops::Deref for SCNParticlePropertyController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNParticlePropertyController {}
impl SCNParticlePropertyController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNParticlePropertyController").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SCNParticlePropertyController {}
impl PNSCopying for SCNParticlePropertyController {}
impl INSObject for SCNParticlePropertyController {}
impl PNSObject for SCNParticlePropertyController {}
impl std::convert::TryFrom<NSObject> for SCNParticlePropertyController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNParticlePropertyController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNParticlePropertyController").unwrap())
        };
        if is_kind_of {
            Ok(SCNParticlePropertyController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNParticlePropertyController")
        }
    }
}
impl ISCNParticlePropertyController for SCNParticlePropertyController {}
pub trait ISCNParticlePropertyController: Sized + std::ops::Deref {
    unsafe fn animation(&self) -> CAAnimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animation)
    }
    unsafe fn setAnimation_(&self, animation: CAAnimation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimation : animation)
    }
    unsafe fn inputMode(&self) -> SCNParticleInputMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputMode)
    }
    unsafe fn setInputMode_(&self, inputMode: SCNParticleInputMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputMode : inputMode)
    }
    unsafe fn inputScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputScale)
    }
    unsafe fn setInputScale_(&self, inputScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputScale : inputScale)
    }
    unsafe fn inputBias(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputBias)
    }
    unsafe fn setInputBias_(&self, inputBias: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputBias : inputBias)
    }
    unsafe fn inputOrigin(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputOrigin)
    }
    unsafe fn setInputOrigin_(&self, inputOrigin: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputOrigin : inputOrigin)
    }
    unsafe fn inputProperty(&self) -> SCNParticleProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputProperty)
    }
    unsafe fn setInputProperty_(&self, inputProperty: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputProperty : inputProperty)
    }
    unsafe fn controllerWithAnimation_(animation: CAAnimation) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNParticlePropertyController").unwrap(), controllerWithAnimation : animation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNParticleSystem(pub id);
impl std::ops::Deref for SCNParticleSystem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNParticleSystem {}
impl SCNParticleSystem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNParticleSystem").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNParticleSystem {}
impl PNSSecureCoding for SCNParticleSystem {}
impl PSCNAnimatable for SCNParticleSystem {}
impl INSObject for SCNParticleSystem {}
impl PNSObject for SCNParticleSystem {}
impl std::convert::TryFrom<NSObject> for SCNParticleSystem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNParticleSystem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNParticleSystem").unwrap()) };
        if is_kind_of {
            Ok(SCNParticleSystem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNParticleSystem")
        }
    }
}
impl ISCNParticleSystem for SCNParticleSystem {}
pub trait ISCNParticleSystem: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn handleEvent_forProperties_withBlock_(
        &self,
        event: SCNParticleEvent,
        properties: NSArray,
        block: SCNParticleEventBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleEvent : event, forProperties : properties, withBlock : block)
    }
    unsafe fn addModifierForProperties_atStage_withBlock_(
        &self,
        properties: NSArray,
        stage: SCNParticleModifierStage,
        block: SCNParticleModifierBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addModifierForProperties : properties, atStage : stage, withBlock : block)
    }
    unsafe fn removeModifiersOfStage_(&self, stage: SCNParticleModifierStage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeModifiersOfStage : stage)
    }
    unsafe fn removeAllModifiers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllModifiers)
    }
    unsafe fn emissionDuration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emissionDuration)
    }
    unsafe fn setEmissionDuration_(&self, emissionDuration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmissionDuration : emissionDuration)
    }
    unsafe fn emissionDurationVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emissionDurationVariation)
    }
    unsafe fn setEmissionDurationVariation_(&self, emissionDurationVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmissionDurationVariation : emissionDurationVariation)
    }
    unsafe fn idleDuration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, idleDuration)
    }
    unsafe fn setIdleDuration_(&self, idleDuration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdleDuration : idleDuration)
    }
    unsafe fn idleDurationVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, idleDurationVariation)
    }
    unsafe fn setIdleDurationVariation_(&self, idleDurationVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdleDurationVariation : idleDurationVariation)
    }
    unsafe fn loops(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loops)
    }
    unsafe fn setLoops_(&self, loops: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoops : loops)
    }
    unsafe fn birthRate(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthRate)
    }
    unsafe fn setBirthRate_(&self, birthRate: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBirthRate : birthRate)
    }
    unsafe fn birthRateVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthRateVariation)
    }
    unsafe fn setBirthRateVariation_(&self, birthRateVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBirthRateVariation : birthRateVariation)
    }
    unsafe fn warmupDuration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, warmupDuration)
    }
    unsafe fn setWarmupDuration_(&self, warmupDuration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWarmupDuration : warmupDuration)
    }
    unsafe fn emitterShape(&self) -> SCNGeometry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterShape)
    }
    unsafe fn setEmitterShape_(&self, emitterShape: SCNGeometry)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterShape : emitterShape)
    }
    unsafe fn birthLocation(&self) -> SCNParticleBirthLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthLocation)
    }
    unsafe fn setBirthLocation_(&self, birthLocation: SCNParticleBirthLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBirthLocation : birthLocation)
    }
    unsafe fn birthDirection(&self) -> SCNParticleBirthDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthDirection)
    }
    unsafe fn setBirthDirection_(&self, birthDirection: SCNParticleBirthDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBirthDirection : birthDirection)
    }
    unsafe fn spreadingAngle(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spreadingAngle)
    }
    unsafe fn setSpreadingAngle_(&self, spreadingAngle: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpreadingAngle : spreadingAngle)
    }
    unsafe fn emittingDirection(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emittingDirection)
    }
    unsafe fn setEmittingDirection_(&self, emittingDirection: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmittingDirection : emittingDirection)
    }
    unsafe fn orientationDirection(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientationDirection)
    }
    unsafe fn setOrientationDirection_(&self, orientationDirection: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientationDirection : orientationDirection)
    }
    unsafe fn acceleration(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceleration)
    }
    unsafe fn setAcceleration_(&self, acceleration: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcceleration : acceleration)
    }
    unsafe fn isLocal(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocal)
    }
    unsafe fn setLocal_(&self, local: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocal : local)
    }
    unsafe fn particleAngle(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAngle)
    }
    unsafe fn setParticleAngle_(&self, particleAngle: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAngle : particleAngle)
    }
    unsafe fn particleAngleVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAngleVariation)
    }
    unsafe fn setParticleAngleVariation_(&self, particleAngleVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAngleVariation : particleAngleVariation)
    }
    unsafe fn particleVelocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleVelocity)
    }
    unsafe fn setParticleVelocity_(&self, particleVelocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleVelocity : particleVelocity)
    }
    unsafe fn particleVelocityVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleVelocityVariation)
    }
    unsafe fn setParticleVelocityVariation_(&self, particleVelocityVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleVelocityVariation : particleVelocityVariation)
    }
    unsafe fn particleAngularVelocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAngularVelocity)
    }
    unsafe fn setParticleAngularVelocity_(&self, particleAngularVelocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAngularVelocity : particleAngularVelocity)
    }
    unsafe fn particleAngularVelocityVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAngularVelocityVariation)
    }
    unsafe fn setParticleAngularVelocityVariation_(&self, particleAngularVelocityVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAngularVelocityVariation : particleAngularVelocityVariation)
    }
    unsafe fn particleLifeSpan(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleLifeSpan)
    }
    unsafe fn setParticleLifeSpan_(&self, particleLifeSpan: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleLifeSpan : particleLifeSpan)
    }
    unsafe fn particleLifeSpanVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleLifeSpanVariation)
    }
    unsafe fn setParticleLifeSpanVariation_(&self, particleLifeSpanVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleLifeSpanVariation : particleLifeSpanVariation)
    }
    unsafe fn systemSpawnedOnDying(&self) -> SCNParticleSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemSpawnedOnDying)
    }
    unsafe fn setSystemSpawnedOnDying_(&self, systemSpawnedOnDying: SCNParticleSystem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSystemSpawnedOnDying : systemSpawnedOnDying)
    }
    unsafe fn systemSpawnedOnCollision(&self) -> SCNParticleSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemSpawnedOnCollision)
    }
    unsafe fn setSystemSpawnedOnCollision_(&self, systemSpawnedOnCollision: SCNParticleSystem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSystemSpawnedOnCollision : systemSpawnedOnCollision)
    }
    unsafe fn systemSpawnedOnLiving(&self) -> SCNParticleSystem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemSpawnedOnLiving)
    }
    unsafe fn setSystemSpawnedOnLiving_(&self, systemSpawnedOnLiving: SCNParticleSystem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSystemSpawnedOnLiving : systemSpawnedOnLiving)
    }
    unsafe fn particleImage(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleImage)
    }
    unsafe fn setParticleImage_(&self, particleImage: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleImage : particleImage)
    }
    unsafe fn imageSequenceColumnCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSequenceColumnCount)
    }
    unsafe fn setImageSequenceColumnCount_(&self, imageSequenceColumnCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageSequenceColumnCount : imageSequenceColumnCount)
    }
    unsafe fn imageSequenceRowCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSequenceRowCount)
    }
    unsafe fn setImageSequenceRowCount_(&self, imageSequenceRowCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageSequenceRowCount : imageSequenceRowCount)
    }
    unsafe fn imageSequenceInitialFrame(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSequenceInitialFrame)
    }
    unsafe fn setImageSequenceInitialFrame_(&self, imageSequenceInitialFrame: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageSequenceInitialFrame : imageSequenceInitialFrame)
    }
    unsafe fn imageSequenceInitialFrameVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSequenceInitialFrameVariation)
    }
    unsafe fn setImageSequenceInitialFrameVariation_(
        &self,
        imageSequenceInitialFrameVariation: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageSequenceInitialFrameVariation : imageSequenceInitialFrameVariation)
    }
    unsafe fn imageSequenceFrameRate(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSequenceFrameRate)
    }
    unsafe fn setImageSequenceFrameRate_(&self, imageSequenceFrameRate: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageSequenceFrameRate : imageSequenceFrameRate)
    }
    unsafe fn imageSequenceFrameRateVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSequenceFrameRateVariation)
    }
    unsafe fn setImageSequenceFrameRateVariation_(&self, imageSequenceFrameRateVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageSequenceFrameRateVariation : imageSequenceFrameRateVariation)
    }
    unsafe fn imageSequenceAnimationMode(&self) -> SCNParticleImageSequenceAnimationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSequenceAnimationMode)
    }
    unsafe fn setImageSequenceAnimationMode_(
        &self,
        imageSequenceAnimationMode: SCNParticleImageSequenceAnimationMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageSequenceAnimationMode : imageSequenceAnimationMode)
    }
    unsafe fn particleColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColor)
    }
    unsafe fn setParticleColor_(&self, particleColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColor : particleColor)
    }
    unsafe fn particleColorVariation(&self) -> SCNVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorVariation)
    }
    unsafe fn setParticleColorVariation_(&self, particleColorVariation: SCNVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorVariation : particleColorVariation)
    }
    unsafe fn particleSize(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleSize)
    }
    unsafe fn setParticleSize_(&self, particleSize: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleSize : particleSize)
    }
    unsafe fn particleSizeVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleSizeVariation)
    }
    unsafe fn setParticleSizeVariation_(&self, particleSizeVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleSizeVariation : particleSizeVariation)
    }
    unsafe fn particleIntensity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleIntensity)
    }
    unsafe fn setParticleIntensity_(&self, particleIntensity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleIntensity : particleIntensity)
    }
    unsafe fn particleIntensityVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleIntensityVariation)
    }
    unsafe fn setParticleIntensityVariation_(&self, particleIntensityVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleIntensityVariation : particleIntensityVariation)
    }
    unsafe fn blendMode(&self) -> SCNParticleBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendMode)
    }
    unsafe fn setBlendMode_(&self, blendMode: SCNParticleBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendMode : blendMode)
    }
    unsafe fn isBlackPassEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlackPassEnabled)
    }
    unsafe fn setBlackPassEnabled_(&self, blackPassEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlackPassEnabled : blackPassEnabled)
    }
    unsafe fn orientationMode(&self) -> SCNParticleOrientationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientationMode)
    }
    unsafe fn setOrientationMode_(&self, orientationMode: SCNParticleOrientationMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientationMode : orientationMode)
    }
    unsafe fn sortingMode(&self) -> SCNParticleSortingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortingMode)
    }
    unsafe fn setSortingMode_(&self, sortingMode: SCNParticleSortingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSortingMode : sortingMode)
    }
    unsafe fn isLightingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLightingEnabled)
    }
    unsafe fn setLightingEnabled_(&self, lightingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightingEnabled : lightingEnabled)
    }
    unsafe fn affectedByGravity(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, affectedByGravity)
    }
    unsafe fn setAffectedByGravity_(&self, affectedByGravity: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAffectedByGravity : affectedByGravity)
    }
    unsafe fn affectedByPhysicsFields(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, affectedByPhysicsFields)
    }
    unsafe fn setAffectedByPhysicsFields_(&self, affectedByPhysicsFields: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAffectedByPhysicsFields : affectedByPhysicsFields)
    }
    unsafe fn particleDiesOnCollision(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleDiesOnCollision)
    }
    unsafe fn setParticleDiesOnCollision_(&self, particleDiesOnCollision: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleDiesOnCollision : particleDiesOnCollision)
    }
    unsafe fn colliderNodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colliderNodes)
    }
    unsafe fn setColliderNodes_(&self, colliderNodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColliderNodes : colliderNodes)
    }
    unsafe fn particleMass(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleMass)
    }
    unsafe fn setParticleMass_(&self, particleMass: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleMass : particleMass)
    }
    unsafe fn particleMassVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleMassVariation)
    }
    unsafe fn setParticleMassVariation_(&self, particleMassVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleMassVariation : particleMassVariation)
    }
    unsafe fn particleBounce(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleBounce)
    }
    unsafe fn setParticleBounce_(&self, particleBounce: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleBounce : particleBounce)
    }
    unsafe fn particleBounceVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleBounceVariation)
    }
    unsafe fn setParticleBounceVariation_(&self, particleBounceVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleBounceVariation : particleBounceVariation)
    }
    unsafe fn particleFriction(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleFriction)
    }
    unsafe fn setParticleFriction_(&self, particleFriction: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleFriction : particleFriction)
    }
    unsafe fn particleFrictionVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleFrictionVariation)
    }
    unsafe fn setParticleFrictionVariation_(&self, particleFrictionVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleFrictionVariation : particleFrictionVariation)
    }
    unsafe fn particleCharge(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleCharge)
    }
    unsafe fn setParticleCharge_(&self, particleCharge: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleCharge : particleCharge)
    }
    unsafe fn particleChargeVariation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleChargeVariation)
    }
    unsafe fn setParticleChargeVariation_(&self, particleChargeVariation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleChargeVariation : particleChargeVariation)
    }
    unsafe fn dampingFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dampingFactor)
    }
    unsafe fn setDampingFactor_(&self, dampingFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDampingFactor : dampingFactor)
    }
    unsafe fn speedFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speedFactor)
    }
    unsafe fn setSpeedFactor_(&self, speedFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeedFactor : speedFactor)
    }
    unsafe fn stretchFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stretchFactor)
    }
    unsafe fn setStretchFactor_(&self, stretchFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStretchFactor : stretchFactor)
    }
    unsafe fn fresnelExponent(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fresnelExponent)
    }
    unsafe fn setFresnelExponent_(&self, fresnelExponent: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFresnelExponent : fresnelExponent)
    }
    unsafe fn writesToDepthBuffer(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writesToDepthBuffer)
    }
    unsafe fn setWritesToDepthBuffer_(&self, writesToDepthBuffer: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWritesToDepthBuffer : writesToDepthBuffer)
    }
    unsafe fn propertyControllers(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertyControllers)
    }
    unsafe fn setPropertyControllers_(&self, propertyControllers: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertyControllers : propertyControllers)
    }
    unsafe fn particleSystem() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNParticleSystem").unwrap(), particleSystem)
    }
    unsafe fn particleSystemNamed_inDirectory_(name: NSString, directory: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNParticleSystem").unwrap(), particleSystemNamed : name, inDirectory : directory)
    }
}
impl SCNNode_SCNParticleSystemSupport for SCNNode {}
pub trait SCNNode_SCNParticleSystemSupport: Sized + std::ops::Deref {
    unsafe fn addParticleSystem_(&self, system: SCNParticleSystem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addParticleSystem : system)
    }
    unsafe fn removeAllParticleSystems(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllParticleSystems)
    }
    unsafe fn removeParticleSystem_(&self, system: SCNParticleSystem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeParticleSystem : system)
    }
    unsafe fn particleSystems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleSystems)
    }
}
impl SCNScene_SCNParticleSystemSupport for SCNScene {}
pub trait SCNScene_SCNParticleSystemSupport: Sized + std::ops::Deref {
    unsafe fn addParticleSystem_withTransform_(
        &self,
        system: SCNParticleSystem,
        transform: SCNMatrix4,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addParticleSystem : system, withTransform : transform)
    }
    unsafe fn removeAllParticleSystems(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllParticleSystems)
    }
    unsafe fn removeParticleSystem_(&self, system: SCNParticleSystem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeParticleSystem : system)
    }
    unsafe fn particleSystems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleSystems)
    }
}
pub type SCNPhysicsBodyType = NSInteger;
pub type SCNPhysicsCollisionCategory = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsBody(pub id);
impl std::ops::Deref for SCNPhysicsBody {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsBody {}
impl SCNPhysicsBody {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBody").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNPhysicsBody {}
impl PNSSecureCoding for SCNPhysicsBody {}
impl INSObject for SCNPhysicsBody {}
impl PNSObject for SCNPhysicsBody {}
impl std::convert::TryFrom<NSObject> for SCNPhysicsBody {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNPhysicsBody, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsBody").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsBody(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNPhysicsBody")
        }
    }
}
impl ISCNPhysicsBody for SCNPhysicsBody {}
pub trait ISCNPhysicsBody: Sized + std::ops::Deref {
    unsafe fn applyForce_impulse_(&self, direction: SCNVector3, impulse: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyForce : direction, impulse : impulse)
    }
    unsafe fn applyForce_atPosition_impulse_(
        &self,
        direction: SCNVector3,
        position: SCNVector3,
        impulse: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyForce : direction, atPosition : position, impulse : impulse)
    }
    unsafe fn applyTorque_impulse_(&self, torque: SCNVector4, impulse: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyTorque : torque, impulse : impulse)
    }
    unsafe fn clearAllForces(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearAllForces)
    }
    unsafe fn resetTransform(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetTransform)
    }
    unsafe fn setResting_(&self, resting: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResting : resting)
    }
    unsafe fn type_(&self) -> SCNPhysicsBodyType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: SCNPhysicsBodyType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn mass(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mass)
    }
    unsafe fn setMass_(&self, mass: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMass : mass)
    }
    unsafe fn momentOfInertia(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, momentOfInertia)
    }
    unsafe fn setMomentOfInertia_(&self, momentOfInertia: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMomentOfInertia : momentOfInertia)
    }
    unsafe fn usesDefaultMomentOfInertia(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesDefaultMomentOfInertia)
    }
    unsafe fn setUsesDefaultMomentOfInertia_(&self, usesDefaultMomentOfInertia: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesDefaultMomentOfInertia : usesDefaultMomentOfInertia)
    }
    unsafe fn charge(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charge)
    }
    unsafe fn setCharge_(&self, charge: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharge : charge)
    }
    unsafe fn friction(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, friction)
    }
    unsafe fn setFriction_(&self, friction: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFriction : friction)
    }
    unsafe fn restitution(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restitution)
    }
    unsafe fn setRestitution_(&self, restitution: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRestitution : restitution)
    }
    unsafe fn rollingFriction(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rollingFriction)
    }
    unsafe fn setRollingFriction_(&self, rollingFriction: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRollingFriction : rollingFriction)
    }
    unsafe fn physicsShape(&self) -> SCNPhysicsShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicsShape)
    }
    unsafe fn setPhysicsShape_(&self, physicsShape: SCNPhysicsShape)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhysicsShape : physicsShape)
    }
    unsafe fn isResting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isResting)
    }
    unsafe fn allowsResting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsResting)
    }
    unsafe fn setAllowsResting_(&self, allowsResting: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsResting : allowsResting)
    }
    unsafe fn velocity(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocity)
    }
    unsafe fn setVelocity_(&self, velocity: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVelocity : velocity)
    }
    unsafe fn angularVelocity(&self) -> SCNVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angularVelocity)
    }
    unsafe fn setAngularVelocity_(&self, angularVelocity: SCNVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngularVelocity : angularVelocity)
    }
    unsafe fn damping(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, damping)
    }
    unsafe fn setDamping_(&self, damping: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDamping : damping)
    }
    unsafe fn angularDamping(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angularDamping)
    }
    unsafe fn setAngularDamping_(&self, angularDamping: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngularDamping : angularDamping)
    }
    unsafe fn velocityFactor(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocityFactor)
    }
    unsafe fn setVelocityFactor_(&self, velocityFactor: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVelocityFactor : velocityFactor)
    }
    unsafe fn angularVelocityFactor(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angularVelocityFactor)
    }
    unsafe fn setAngularVelocityFactor_(&self, angularVelocityFactor: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngularVelocityFactor : angularVelocityFactor)
    }
    unsafe fn categoryBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryBitMask)
    }
    unsafe fn setCategoryBitMask_(&self, categoryBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryBitMask : categoryBitMask)
    }
    unsafe fn collisionBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collisionBitMask)
    }
    unsafe fn setCollisionBitMask_(&self, collisionBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCollisionBitMask : collisionBitMask)
    }
    unsafe fn contactTestBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactTestBitMask)
    }
    unsafe fn setContactTestBitMask_(&self, contactTestBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactTestBitMask : contactTestBitMask)
    }
    unsafe fn isAffectedByGravity(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAffectedByGravity)
    }
    unsafe fn setAffectedByGravity_(&self, affectedByGravity: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAffectedByGravity : affectedByGravity)
    }
    unsafe fn continuousCollisionDetectionThreshold(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, continuousCollisionDetectionThreshold)
    }
    unsafe fn setContinuousCollisionDetectionThreshold_(
        &self,
        continuousCollisionDetectionThreshold: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContinuousCollisionDetectionThreshold : continuousCollisionDetectionThreshold)
    }
    unsafe fn centerOfMassOffset(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerOfMassOffset)
    }
    unsafe fn setCenterOfMassOffset_(&self, centerOfMassOffset: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterOfMassOffset : centerOfMassOffset)
    }
    unsafe fn linearRestingThreshold(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linearRestingThreshold)
    }
    unsafe fn setLinearRestingThreshold_(&self, linearRestingThreshold: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinearRestingThreshold : linearRestingThreshold)
    }
    unsafe fn angularRestingThreshold(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angularRestingThreshold)
    }
    unsafe fn setAngularRestingThreshold_(&self, angularRestingThreshold: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngularRestingThreshold : angularRestingThreshold)
    }
    unsafe fn staticBody() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBody").unwrap(), staticBody)
    }
    unsafe fn dynamicBody() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBody").unwrap(), dynamicBody)
    }
    unsafe fn kinematicBody() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBody").unwrap(), kinematicBody)
    }
    unsafe fn bodyWithType_shape_(type_: SCNPhysicsBodyType, shape: SCNPhysicsShape) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBody").unwrap(), bodyWithType : type_, shape : shape)
    }
}
pub type SCNPhysicsFieldScope = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsField(pub id);
impl std::ops::Deref for SCNPhysicsField {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsField {}
impl SCNPhysicsField {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNPhysicsField {}
impl PNSSecureCoding for SCNPhysicsField {}
impl INSObject for SCNPhysicsField {}
impl PNSObject for SCNPhysicsField {}
impl std::convert::TryFrom<NSObject> for SCNPhysicsField {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNPhysicsField, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsField(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNPhysicsField")
        }
    }
}
impl ISCNPhysicsField for SCNPhysicsField {}
pub trait ISCNPhysicsField: Sized + std::ops::Deref {
    unsafe fn strength(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strength)
    }
    unsafe fn setStrength_(&self, strength: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrength : strength)
    }
    unsafe fn falloffExponent(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, falloffExponent)
    }
    unsafe fn setFalloffExponent_(&self, falloffExponent: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFalloffExponent : falloffExponent)
    }
    unsafe fn minimumDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumDistance)
    }
    unsafe fn setMinimumDistance_(&self, minimumDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumDistance : minimumDistance)
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
    unsafe fn isExclusive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExclusive)
    }
    unsafe fn setExclusive_(&self, exclusive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExclusive : exclusive)
    }
    unsafe fn halfExtent(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, halfExtent)
    }
    unsafe fn setHalfExtent_(&self, halfExtent: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHalfExtent : halfExtent)
    }
    unsafe fn usesEllipsoidalExtent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesEllipsoidalExtent)
    }
    unsafe fn setUsesEllipsoidalExtent_(&self, usesEllipsoidalExtent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesEllipsoidalExtent : usesEllipsoidalExtent)
    }
    unsafe fn scope(&self) -> SCNPhysicsFieldScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scope)
    }
    unsafe fn setScope_(&self, scope: SCNPhysicsFieldScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScope : scope)
    }
    unsafe fn offset(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn direction(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
    unsafe fn setDirection_(&self, direction: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDirection : direction)
    }
    unsafe fn categoryBitMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryBitMask)
    }
    unsafe fn setCategoryBitMask_(&self, categoryBitMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryBitMask : categoryBitMask)
    }
    unsafe fn dragField() -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), dragField)
    }
    unsafe fn vortexField() -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), vortexField)
    }
    unsafe fn radialGravityField() -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), radialGravityField)
    }
    unsafe fn linearGravityField() -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), linearGravityField)
    }
    unsafe fn noiseFieldWithSmoothness_animationSpeed_(
        smoothness: CGFloat,
        speed: CGFloat,
    ) -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), noiseFieldWithSmoothness : smoothness, animationSpeed : speed)
    }
    unsafe fn turbulenceFieldWithSmoothness_animationSpeed_(
        smoothness: CGFloat,
        speed: CGFloat,
    ) -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), turbulenceFieldWithSmoothness : smoothness, animationSpeed : speed)
    }
    unsafe fn springField() -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), springField)
    }
    unsafe fn electricField() -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), electricField)
    }
    unsafe fn magneticField() -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), magneticField)
    }
    unsafe fn customFieldWithEvaluationBlock_(block: SCNFieldForceEvaluator) -> SCNPhysicsField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsField").unwrap(), customFieldWithEvaluationBlock : block)
    }
}
pub type SCNFieldForceEvaluator = *mut ::std::os::raw::c_void;
pub type SCNPhysicsShapeOption = NSString;
pub type SCNPhysicsShapeType = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsShape(pub id);
impl std::ops::Deref for SCNPhysicsShape {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsShape {}
impl SCNPhysicsShape {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsShape").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNPhysicsShape {}
impl PNSSecureCoding for SCNPhysicsShape {}
impl INSObject for SCNPhysicsShape {}
impl PNSObject for SCNPhysicsShape {}
impl std::convert::TryFrom<NSObject> for SCNPhysicsShape {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNPhysicsShape, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsShape").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsShape(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNPhysicsShape")
        }
    }
}
impl ISCNPhysicsShape for SCNPhysicsShape {}
pub trait ISCNPhysicsShape: Sized + std::ops::Deref {
    unsafe fn options(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn sourceObject(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceObject)
    }
    unsafe fn transforms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transforms)
    }
    unsafe fn shapeWithGeometry_options_(
        geometry: SCNGeometry,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsShape").unwrap(), shapeWithGeometry : geometry, options : options)
    }
    unsafe fn shapeWithNode_options_(node: SCNNode, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsShape").unwrap(), shapeWithNode : node, options : options)
    }
    unsafe fn shapeWithShapes_transforms_(shapes: NSArray, transforms: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsShape").unwrap(), shapeWithShapes : shapes, transforms : transforms)
    }
}
pub type SCNPhysicsTestOption = NSString;
pub type SCNPhysicsTestSearchMode = NSString;
pub trait PSCNPhysicsContactDelegate: Sized + std::ops::Deref {
    unsafe fn physicsWorld_didBeginContact_(
        &self,
        world: SCNPhysicsWorld,
        contact: SCNPhysicsContact,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, physicsWorld : world, didBeginContact : contact)
    }
    unsafe fn physicsWorld_didUpdateContact_(
        &self,
        world: SCNPhysicsWorld,
        contact: SCNPhysicsContact,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, physicsWorld : world, didUpdateContact : contact)
    }
    unsafe fn physicsWorld_didEndContact_(&self, world: SCNPhysicsWorld, contact: SCNPhysicsContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, physicsWorld : world, didEndContact : contact)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsWorld(pub id);
impl std::ops::Deref for SCNPhysicsWorld {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsWorld {}
impl SCNPhysicsWorld {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsWorld").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SCNPhysicsWorld {}
impl INSObject for SCNPhysicsWorld {}
impl PNSObject for SCNPhysicsWorld {}
impl std::convert::TryFrom<NSObject> for SCNPhysicsWorld {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNPhysicsWorld, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsWorld").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsWorld(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNPhysicsWorld")
        }
    }
}
impl ISCNPhysicsWorld for SCNPhysicsWorld {}
pub trait ISCNPhysicsWorld: Sized + std::ops::Deref {
    unsafe fn addBehavior_(&self, behavior: SCNPhysicsBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addBehavior : behavior)
    }
    unsafe fn removeBehavior_(&self, behavior: SCNPhysicsBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeBehavior : behavior)
    }
    unsafe fn removeAllBehaviors(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllBehaviors)
    }
    unsafe fn rayTestWithSegmentFromPoint_toPoint_options_(
        &self,
        origin: SCNVector3,
        dest: SCNVector3,
        options: NSDictionary,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rayTestWithSegmentFromPoint : origin, toPoint : dest, options : options)
    }
    unsafe fn contactTestBetweenBody_andBody_options_(
        &self,
        bodyA: SCNPhysicsBody,
        bodyB: SCNPhysicsBody,
        options: NSDictionary,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contactTestBetweenBody : bodyA, andBody : bodyB, options : options)
    }
    unsafe fn contactTestWithBody_options_(
        &self,
        body: SCNPhysicsBody,
        options: NSDictionary,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contactTestWithBody : body, options : options)
    }
    unsafe fn convexSweepTestWithShape_fromTransform_toTransform_options_(
        &self,
        shape: SCNPhysicsShape,
        from: SCNMatrix4,
        to: SCNMatrix4,
        options: NSDictionary,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convexSweepTestWithShape : shape, fromTransform : from, toTransform : to, options : options)
    }
    unsafe fn updateCollisionPairs(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateCollisionPairs)
    }
    unsafe fn gravity(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gravity)
    }
    unsafe fn setGravity_(&self, gravity: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGravity : gravity)
    }
    unsafe fn speed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn setSpeed_(&self, speed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed)
    }
    unsafe fn timeStep(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeStep)
    }
    unsafe fn setTimeStep_(&self, timeStep: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeStep : timeStep)
    }
    unsafe fn contactDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactDelegate)
    }
    unsafe fn setContactDelegate_(&self, contactDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactDelegate : contactDelegate)
    }
    unsafe fn allBehaviors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allBehaviors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsContact(pub id);
impl std::ops::Deref for SCNPhysicsContact {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsContact {}
impl SCNPhysicsContact {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsContact").unwrap(), alloc) })
    }
}
impl INSObject for SCNPhysicsContact {}
impl PNSObject for SCNPhysicsContact {}
impl std::convert::TryFrom<NSObject> for SCNPhysicsContact {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNPhysicsContact, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsContact").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsContact(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNPhysicsContact")
        }
    }
}
impl ISCNPhysicsContact for SCNPhysicsContact {}
pub trait ISCNPhysicsContact: Sized + std::ops::Deref {
    unsafe fn nodeA(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodeA)
    }
    unsafe fn nodeB(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodeB)
    }
    unsafe fn contactPoint(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactPoint)
    }
    unsafe fn contactNormal(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactNormal)
    }
    unsafe fn collisionImpulse(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collisionImpulse)
    }
    unsafe fn penetrationDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, penetrationDistance)
    }
    unsafe fn sweepTestFraction(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sweepTestFraction)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsBehavior(pub id);
impl std::ops::Deref for SCNPhysicsBehavior {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsBehavior {}
impl SCNPhysicsBehavior {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBehavior").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SCNPhysicsBehavior {}
impl INSObject for SCNPhysicsBehavior {}
impl PNSObject for SCNPhysicsBehavior {}
impl std::convert::TryFrom<NSObject> for SCNPhysicsBehavior {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNPhysicsBehavior, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsBehavior").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsBehavior(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNPhysicsBehavior")
        }
    }
}
impl ISCNPhysicsBehavior for SCNPhysicsBehavior {}
pub trait ISCNPhysicsBehavior: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsHingeJoint(pub id);
impl std::ops::Deref for SCNPhysicsHingeJoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsHingeJoint {}
impl SCNPhysicsHingeJoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsHingeJoint").unwrap(), alloc) })
    }
}
impl ISCNPhysicsBehavior for SCNPhysicsHingeJoint {}
impl PNSSecureCoding for SCNPhysicsHingeJoint {}
impl From<SCNPhysicsHingeJoint> for SCNPhysicsBehavior {
    fn from(child: SCNPhysicsHingeJoint) -> SCNPhysicsBehavior {
        SCNPhysicsBehavior(child.0)
    }
}
impl std::convert::TryFrom<SCNPhysicsBehavior> for SCNPhysicsHingeJoint {
    type Error = &'static str;
    fn try_from(parent: SCNPhysicsBehavior) -> Result<SCNPhysicsHingeJoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsHingeJoint").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsHingeJoint(parent.0))
        } else {
            Err("This SCNPhysicsBehavior cannot be downcasted to SCNPhysicsHingeJoint")
        }
    }
}
impl INSObject for SCNPhysicsHingeJoint {}
impl PNSObject for SCNPhysicsHingeJoint {}
impl ISCNPhysicsHingeJoint for SCNPhysicsHingeJoint {}
pub trait ISCNPhysicsHingeJoint: Sized + std::ops::Deref {
    unsafe fn bodyA(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyA)
    }
    unsafe fn axisA(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axisA)
    }
    unsafe fn setAxisA_(&self, axisA: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAxisA : axisA)
    }
    unsafe fn anchorA(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorA)
    }
    unsafe fn setAnchorA_(&self, anchorA: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorA : anchorA)
    }
    unsafe fn bodyB(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyB)
    }
    unsafe fn axisB(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axisB)
    }
    unsafe fn setAxisB_(&self, axisB: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAxisB : axisB)
    }
    unsafe fn anchorB(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorB)
    }
    unsafe fn setAnchorB_(&self, anchorB: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorB : anchorB)
    }
    unsafe fn jointWithBodyA_axisA_anchorA_bodyB_axisB_anchorB_(
        bodyA: SCNPhysicsBody,
        axisA: SCNVector3,
        anchorA: SCNVector3,
        bodyB: SCNPhysicsBody,
        axisB: SCNVector3,
        anchorB: SCNVector3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsHingeJoint").unwrap(), jointWithBodyA : bodyA, axisA : axisA, anchorA : anchorA, bodyB : bodyB, axisB : axisB, anchorB : anchorB)
    }
    unsafe fn jointWithBody_axis_anchor_(
        body: SCNPhysicsBody,
        axis: SCNVector3,
        anchor: SCNVector3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsHingeJoint").unwrap(), jointWithBody : body, axis : axis, anchor : anchor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsBallSocketJoint(pub id);
impl std::ops::Deref for SCNPhysicsBallSocketJoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsBallSocketJoint {}
impl SCNPhysicsBallSocketJoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBallSocketJoint").unwrap(), alloc) })
    }
}
impl ISCNPhysicsBehavior for SCNPhysicsBallSocketJoint {}
impl PNSSecureCoding for SCNPhysicsBallSocketJoint {}
impl std::convert::TryFrom<SCNPhysicsBehavior> for SCNPhysicsBallSocketJoint {
    type Error = &'static str;
    fn try_from(parent: SCNPhysicsBehavior) -> Result<SCNPhysicsBallSocketJoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsBallSocketJoint").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsBallSocketJoint(parent.0))
        } else {
            Err("This SCNPhysicsBehavior cannot be downcasted to SCNPhysicsBallSocketJoint")
        }
    }
}
impl INSObject for SCNPhysicsBallSocketJoint {}
impl PNSObject for SCNPhysicsBallSocketJoint {}
impl ISCNPhysicsBallSocketJoint for SCNPhysicsBallSocketJoint {}
pub trait ISCNPhysicsBallSocketJoint: Sized + std::ops::Deref {
    unsafe fn bodyA(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyA)
    }
    unsafe fn anchorA(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorA)
    }
    unsafe fn setAnchorA_(&self, anchorA: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorA : anchorA)
    }
    unsafe fn bodyB(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyB)
    }
    unsafe fn anchorB(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorB)
    }
    unsafe fn setAnchorB_(&self, anchorB: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorB : anchorB)
    }
    unsafe fn jointWithBodyA_anchorA_bodyB_anchorB_(
        bodyA: SCNPhysicsBody,
        anchorA: SCNVector3,
        bodyB: SCNPhysicsBody,
        anchorB: SCNVector3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBallSocketJoint").unwrap(), jointWithBodyA : bodyA, anchorA : anchorA, bodyB : bodyB, anchorB : anchorB)
    }
    unsafe fn jointWithBody_anchor_(body: SCNPhysicsBody, anchor: SCNVector3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsBallSocketJoint").unwrap(), jointWithBody : body, anchor : anchor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsSliderJoint(pub id);
impl std::ops::Deref for SCNPhysicsSliderJoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsSliderJoint {}
impl SCNPhysicsSliderJoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsSliderJoint").unwrap(), alloc) })
    }
}
impl ISCNPhysicsBehavior for SCNPhysicsSliderJoint {}
impl PNSSecureCoding for SCNPhysicsSliderJoint {}
impl std::convert::TryFrom<SCNPhysicsBehavior> for SCNPhysicsSliderJoint {
    type Error = &'static str;
    fn try_from(parent: SCNPhysicsBehavior) -> Result<SCNPhysicsSliderJoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsSliderJoint").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsSliderJoint(parent.0))
        } else {
            Err("This SCNPhysicsBehavior cannot be downcasted to SCNPhysicsSliderJoint")
        }
    }
}
impl INSObject for SCNPhysicsSliderJoint {}
impl PNSObject for SCNPhysicsSliderJoint {}
impl ISCNPhysicsSliderJoint for SCNPhysicsSliderJoint {}
pub trait ISCNPhysicsSliderJoint: Sized + std::ops::Deref {
    unsafe fn bodyA(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyA)
    }
    unsafe fn axisA(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axisA)
    }
    unsafe fn setAxisA_(&self, axisA: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAxisA : axisA)
    }
    unsafe fn anchorA(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorA)
    }
    unsafe fn setAnchorA_(&self, anchorA: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorA : anchorA)
    }
    unsafe fn bodyB(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyB)
    }
    unsafe fn axisB(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axisB)
    }
    unsafe fn setAxisB_(&self, axisB: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAxisB : axisB)
    }
    unsafe fn anchorB(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorB)
    }
    unsafe fn setAnchorB_(&self, anchorB: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorB : anchorB)
    }
    unsafe fn minimumLinearLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumLinearLimit)
    }
    unsafe fn setMinimumLinearLimit_(&self, minimumLinearLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumLinearLimit : minimumLinearLimit)
    }
    unsafe fn maximumLinearLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumLinearLimit)
    }
    unsafe fn setMaximumLinearLimit_(&self, maximumLinearLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumLinearLimit : maximumLinearLimit)
    }
    unsafe fn minimumAngularLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumAngularLimit)
    }
    unsafe fn setMinimumAngularLimit_(&self, minimumAngularLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumAngularLimit : minimumAngularLimit)
    }
    unsafe fn maximumAngularLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumAngularLimit)
    }
    unsafe fn setMaximumAngularLimit_(&self, maximumAngularLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumAngularLimit : maximumAngularLimit)
    }
    unsafe fn motorTargetLinearVelocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motorTargetLinearVelocity)
    }
    unsafe fn setMotorTargetLinearVelocity_(&self, motorTargetLinearVelocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotorTargetLinearVelocity : motorTargetLinearVelocity)
    }
    unsafe fn motorMaximumForce(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motorMaximumForce)
    }
    unsafe fn setMotorMaximumForce_(&self, motorMaximumForce: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotorMaximumForce : motorMaximumForce)
    }
    unsafe fn motorTargetAngularVelocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motorTargetAngularVelocity)
    }
    unsafe fn setMotorTargetAngularVelocity_(&self, motorTargetAngularVelocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotorTargetAngularVelocity : motorTargetAngularVelocity)
    }
    unsafe fn motorMaximumTorque(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motorMaximumTorque)
    }
    unsafe fn setMotorMaximumTorque_(&self, motorMaximumTorque: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotorMaximumTorque : motorMaximumTorque)
    }
    unsafe fn jointWithBodyA_axisA_anchorA_bodyB_axisB_anchorB_(
        bodyA: SCNPhysicsBody,
        axisA: SCNVector3,
        anchorA: SCNVector3,
        bodyB: SCNPhysicsBody,
        axisB: SCNVector3,
        anchorB: SCNVector3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsSliderJoint").unwrap(), jointWithBodyA : bodyA, axisA : axisA, anchorA : anchorA, bodyB : bodyB, axisB : axisB, anchorB : anchorB)
    }
    unsafe fn jointWithBody_axis_anchor_(
        body: SCNPhysicsBody,
        axis: SCNVector3,
        anchor: SCNVector3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsSliderJoint").unwrap(), jointWithBody : body, axis : axis, anchor : anchor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsConeTwistJoint(pub id);
impl std::ops::Deref for SCNPhysicsConeTwistJoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsConeTwistJoint {}
impl SCNPhysicsConeTwistJoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsConeTwistJoint").unwrap(), alloc) })
    }
}
impl ISCNPhysicsBehavior for SCNPhysicsConeTwistJoint {}
impl PNSSecureCoding for SCNPhysicsConeTwistJoint {}
impl std::convert::TryFrom<SCNPhysicsBehavior> for SCNPhysicsConeTwistJoint {
    type Error = &'static str;
    fn try_from(parent: SCNPhysicsBehavior) -> Result<SCNPhysicsConeTwistJoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsConeTwistJoint").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsConeTwistJoint(parent.0))
        } else {
            Err("This SCNPhysicsBehavior cannot be downcasted to SCNPhysicsConeTwistJoint")
        }
    }
}
impl INSObject for SCNPhysicsConeTwistJoint {}
impl PNSObject for SCNPhysicsConeTwistJoint {}
impl ISCNPhysicsConeTwistJoint for SCNPhysicsConeTwistJoint {}
pub trait ISCNPhysicsConeTwistJoint: Sized + std::ops::Deref {
    unsafe fn bodyA(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyA)
    }
    unsafe fn frameA(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameA)
    }
    unsafe fn setFrameA_(&self, frameA: SCNMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameA : frameA)
    }
    unsafe fn bodyB(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyB)
    }
    unsafe fn frameB(&self) -> SCNMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameB)
    }
    unsafe fn setFrameB_(&self, frameB: SCNMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameB : frameB)
    }
    unsafe fn maximumAngularLimit1(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumAngularLimit1)
    }
    unsafe fn setMaximumAngularLimit1_(&self, maximumAngularLimit1: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumAngularLimit1 : maximumAngularLimit1)
    }
    unsafe fn maximumAngularLimit2(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumAngularLimit2)
    }
    unsafe fn setMaximumAngularLimit2_(&self, maximumAngularLimit2: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumAngularLimit2 : maximumAngularLimit2)
    }
    unsafe fn maximumTwistAngle(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumTwistAngle)
    }
    unsafe fn setMaximumTwistAngle_(&self, maximumTwistAngle: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumTwistAngle : maximumTwistAngle)
    }
    unsafe fn jointWithBodyA_frameA_bodyB_frameB_(
        bodyA: SCNPhysicsBody,
        frameA: SCNMatrix4,
        bodyB: SCNPhysicsBody,
        frameB: SCNMatrix4,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsConeTwistJoint").unwrap(), jointWithBodyA : bodyA, frameA : frameA, bodyB : bodyB, frameB : frameB)
    }
    unsafe fn jointWithBody_frame_(body: SCNPhysicsBody, frame: SCNMatrix4) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsConeTwistJoint").unwrap(), jointWithBody : body, frame : frame)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsVehicleWheel(pub id);
impl std::ops::Deref for SCNPhysicsVehicleWheel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsVehicleWheel {}
impl SCNPhysicsVehicleWheel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsVehicleWheel").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNPhysicsVehicleWheel {}
impl PNSSecureCoding for SCNPhysicsVehicleWheel {}
impl INSObject for SCNPhysicsVehicleWheel {}
impl PNSObject for SCNPhysicsVehicleWheel {}
impl std::convert::TryFrom<NSObject> for SCNPhysicsVehicleWheel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNPhysicsVehicleWheel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsVehicleWheel").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsVehicleWheel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNPhysicsVehicleWheel")
        }
    }
}
impl ISCNPhysicsVehicleWheel for SCNPhysicsVehicleWheel {}
pub trait ISCNPhysicsVehicleWheel: Sized + std::ops::Deref {
    unsafe fn node(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, node)
    }
    unsafe fn suspensionStiffness(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suspensionStiffness)
    }
    unsafe fn setSuspensionStiffness_(&self, suspensionStiffness: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuspensionStiffness : suspensionStiffness)
    }
    unsafe fn suspensionCompression(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suspensionCompression)
    }
    unsafe fn setSuspensionCompression_(&self, suspensionCompression: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuspensionCompression : suspensionCompression)
    }
    unsafe fn suspensionDamping(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suspensionDamping)
    }
    unsafe fn setSuspensionDamping_(&self, suspensionDamping: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuspensionDamping : suspensionDamping)
    }
    unsafe fn maximumSuspensionTravel(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumSuspensionTravel)
    }
    unsafe fn setMaximumSuspensionTravel_(&self, maximumSuspensionTravel: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumSuspensionTravel : maximumSuspensionTravel)
    }
    unsafe fn frictionSlip(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frictionSlip)
    }
    unsafe fn setFrictionSlip_(&self, frictionSlip: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrictionSlip : frictionSlip)
    }
    unsafe fn maximumSuspensionForce(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumSuspensionForce)
    }
    unsafe fn setMaximumSuspensionForce_(&self, maximumSuspensionForce: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumSuspensionForce : maximumSuspensionForce)
    }
    unsafe fn connectionPosition(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionPosition)
    }
    unsafe fn setConnectionPosition_(&self, connectionPosition: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectionPosition : connectionPosition)
    }
    unsafe fn steeringAxis(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, steeringAxis)
    }
    unsafe fn setSteeringAxis_(&self, steeringAxis: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSteeringAxis : steeringAxis)
    }
    unsafe fn axle(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axle)
    }
    unsafe fn setAxle_(&self, axle: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAxle : axle)
    }
    unsafe fn radius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn suspensionRestLength(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suspensionRestLength)
    }
    unsafe fn setSuspensionRestLength_(&self, suspensionRestLength: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuspensionRestLength : suspensionRestLength)
    }
    unsafe fn wheelWithNode_(node: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsVehicleWheel").unwrap(), wheelWithNode : node)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNPhysicsVehicle(pub id);
impl std::ops::Deref for SCNPhysicsVehicle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNPhysicsVehicle {}
impl SCNPhysicsVehicle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsVehicle").unwrap(), alloc) })
    }
}
impl ISCNPhysicsBehavior for SCNPhysicsVehicle {}
impl PNSSecureCoding for SCNPhysicsVehicle {}
impl std::convert::TryFrom<SCNPhysicsBehavior> for SCNPhysicsVehicle {
    type Error = &'static str;
    fn try_from(parent: SCNPhysicsBehavior) -> Result<SCNPhysicsVehicle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNPhysicsVehicle").unwrap()) };
        if is_kind_of {
            Ok(SCNPhysicsVehicle(parent.0))
        } else {
            Err("This SCNPhysicsBehavior cannot be downcasted to SCNPhysicsVehicle")
        }
    }
}
impl INSObject for SCNPhysicsVehicle {}
impl PNSObject for SCNPhysicsVehicle {}
impl ISCNPhysicsVehicle for SCNPhysicsVehicle {}
pub trait ISCNPhysicsVehicle: Sized + std::ops::Deref {
    unsafe fn applyEngineForce_forWheelAtIndex_(&self, value: CGFloat, index: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyEngineForce : value, forWheelAtIndex : index)
    }
    unsafe fn setSteeringAngle_forWheelAtIndex_(&self, value: CGFloat, index: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSteeringAngle : value, forWheelAtIndex : index)
    }
    unsafe fn applyBrakingForce_forWheelAtIndex_(&self, value: CGFloat, index: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyBrakingForce : value, forWheelAtIndex : index)
    }
    unsafe fn speedInKilometersPerHour(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speedInKilometersPerHour)
    }
    unsafe fn wheels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wheels)
    }
    unsafe fn chassisBody(&self) -> SCNPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chassisBody)
    }
    unsafe fn vehicleWithChassisBody_wheels_(
        chassisBody: SCNPhysicsBody,
        wheels: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNPhysicsVehicle").unwrap(), vehicleWithChassisBody : chassisBody, wheels : wheels)
    }
}
pub type SCNReferenceLoadingPolicy = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNReferenceNode(pub id);
impl std::ops::Deref for SCNReferenceNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNReferenceNode {}
impl SCNReferenceNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNReferenceNode").unwrap(), alloc) })
    }
}
impl ISCNNode for SCNReferenceNode {}
impl PNSCopying for SCNReferenceNode {}
impl PNSSecureCoding for SCNReferenceNode {}
impl PSCNAnimatable for SCNReferenceNode {}
impl PSCNActionable for SCNReferenceNode {}
impl PSCNBoundingVolume for SCNReferenceNode {}
impl From<SCNReferenceNode> for SCNNode {
    fn from(child: SCNReferenceNode) -> SCNNode {
        SCNNode(child.0)
    }
}
impl std::convert::TryFrom<SCNNode> for SCNReferenceNode {
    type Error = &'static str;
    fn try_from(parent: SCNNode) -> Result<SCNReferenceNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNReferenceNode").unwrap()) };
        if is_kind_of {
            Ok(SCNReferenceNode(parent.0))
        } else {
            Err("This SCNNode cannot be downcasted to SCNReferenceNode")
        }
    }
}
impl INSObject for SCNReferenceNode {}
impl PNSObject for SCNReferenceNode {}
impl ISCNReferenceNode for SCNReferenceNode {}
pub trait ISCNReferenceNode: Sized + std::ops::Deref {
    unsafe fn initWithURL_(&self, referenceURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : referenceURL)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn load(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, load)
    }
    unsafe fn unload(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unload)
    }
    unsafe fn referenceURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, referenceURL)
    }
    unsafe fn setReferenceURL_(&self, referenceURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReferenceURL : referenceURL)
    }
    unsafe fn loadingPolicy(&self) -> SCNReferenceLoadingPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loadingPolicy)
    }
    unsafe fn setLoadingPolicy_(&self, loadingPolicy: SCNReferenceLoadingPolicy)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoadingPolicy : loadingPolicy)
    }
    unsafe fn isLoaded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoaded)
    }
    unsafe fn referenceNodeWithURL_(referenceURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNReferenceNode").unwrap(), referenceNodeWithURL : referenceURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNAudioSource(pub id);
impl std::ops::Deref for SCNAudioSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNAudioSource {}
impl SCNAudioSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAudioSource").unwrap(), alloc) })
    }
}
impl PNSCopying for SCNAudioSource {}
impl PNSSecureCoding for SCNAudioSource {}
impl INSObject for SCNAudioSource {}
impl PNSObject for SCNAudioSource {}
impl std::convert::TryFrom<NSObject> for SCNAudioSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNAudioSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNAudioSource").unwrap()) };
        if is_kind_of {
            Ok(SCNAudioSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNAudioSource")
        }
    }
}
impl ISCNAudioSource for SCNAudioSource {}
pub trait ISCNAudioSource: Sized + std::ops::Deref {
    unsafe fn initWithFileNamed_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileNamed : name)
    }
    unsafe fn initWithURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn load(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, load)
    }
    unsafe fn isPositional(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPositional)
    }
    unsafe fn setPositional_(&self, positional: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPositional : positional)
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
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
    unsafe fn reverbBlend(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reverbBlend)
    }
    unsafe fn setReverbBlend_(&self, reverbBlend: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReverbBlend : reverbBlend)
    }
    unsafe fn loops(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loops)
    }
    unsafe fn setLoops_(&self, loops: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoops : loops)
    }
    unsafe fn shouldStream(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldStream)
    }
    unsafe fn setShouldStream_(&self, shouldStream: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldStream : shouldStream)
    }
    unsafe fn audioSourceNamed_(fileName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAudioSource").unwrap(), audioSourceNamed : fileName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNAudioPlayer(pub id);
impl std::ops::Deref for SCNAudioPlayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNAudioPlayer {}
impl SCNAudioPlayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAudioPlayer").unwrap(), alloc) })
    }
}
impl INSObject for SCNAudioPlayer {}
impl PNSObject for SCNAudioPlayer {}
impl std::convert::TryFrom<NSObject> for SCNAudioPlayer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNAudioPlayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNAudioPlayer").unwrap()) };
        if is_kind_of {
            Ok(SCNAudioPlayer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNAudioPlayer")
        }
    }
}
impl ISCNAudioPlayer for SCNAudioPlayer {}
pub trait ISCNAudioPlayer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSource_(&self, source: SCNAudioSource) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source)
    }
    unsafe fn initWithAVAudioNode_(&self, audioNode: AVAudioNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAVAudioNode : audioNode)
    }
    unsafe fn willStartPlayback(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willStartPlayback)
    }
    unsafe fn setWillStartPlayback_(&self, willStartPlayback: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWillStartPlayback : willStartPlayback)
    }
    unsafe fn didFinishPlayback(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didFinishPlayback)
    }
    unsafe fn setDidFinishPlayback_(&self, didFinishPlayback: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDidFinishPlayback : didFinishPlayback)
    }
    unsafe fn audioNode(&self) -> AVAudioNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioNode)
    }
    unsafe fn audioSource(&self) -> SCNAudioSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioSource)
    }
    unsafe fn audioPlayerWithSource_(source: SCNAudioSource) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAudioPlayer").unwrap(), audioPlayerWithSource : source)
    }
    unsafe fn audioPlayerWithAVAudioNode_(audioNode: AVAudioNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNAudioPlayer").unwrap(), audioPlayerWithAVAudioNode : audioNode)
    }
}
impl SCNNode_SCNAudioSupport for SCNNode {}
pub trait SCNNode_SCNAudioSupport: Sized + std::ops::Deref {
    unsafe fn addAudioPlayer_(&self, player: SCNAudioPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAudioPlayer : player)
    }
    unsafe fn removeAllAudioPlayers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllAudioPlayers)
    }
    unsafe fn removeAudioPlayer_(&self, player: SCNAudioPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAudioPlayer : player)
    }
    unsafe fn audioPlayers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioPlayers)
    }
}
pub type SCNInteractionMode = NSInteger;
pub trait PSCNCameraControllerDelegate: Sized + std::ops::Deref {
    unsafe fn cameraInertiaWillStartForController_(&self, cameraController: SCNCameraController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraInertiaWillStartForController : cameraController)
    }
    unsafe fn cameraInertiaDidEndForController_(&self, cameraController: SCNCameraController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraInertiaDidEndForController : cameraController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCNCameraController(pub id);
impl std::ops::Deref for SCNCameraController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCNCameraController {}
impl SCNCameraController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCameraController").unwrap(), alloc) })
    }
}
impl INSObject for SCNCameraController {}
impl PNSObject for SCNCameraController {}
impl std::convert::TryFrom<NSObject> for SCNCameraController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCNCameraController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCNCameraController").unwrap()) };
        if is_kind_of {
            Ok(SCNCameraController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCNCameraController")
        }
    }
}
impl ISCNCameraController for SCNCameraController {}
pub trait ISCNCameraController: Sized + std::ops::Deref {
    unsafe fn translateInCameraSpaceByX_Y_Z_(&self, deltaX: f32, deltaY: f32, deltaZ: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, translateInCameraSpaceByX : deltaX, Y : deltaY, Z : deltaZ)
    }
    unsafe fn frameNodes_(&self, nodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, frameNodes : nodes)
    }
    unsafe fn rotateByX_Y_(&self, deltaX: f32, deltaY: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotateByX : deltaX, Y : deltaY)
    }
    unsafe fn rollBy_aroundScreenPoint_viewport_(
        &self,
        delta: f32,
        point: CGPoint,
        viewport: CGSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rollBy : delta, aroundScreenPoint : point, viewport : viewport)
    }
    unsafe fn dollyBy_onScreenPoint_viewport_(&self, delta: f32, point: CGPoint, viewport: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dollyBy : delta, onScreenPoint : point, viewport : viewport)
    }
    unsafe fn rollAroundTarget_(&self, delta: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rollAroundTarget : delta)
    }
    unsafe fn dollyToTarget_(&self, delta: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dollyToTarget : delta)
    }
    unsafe fn clearRoll(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearRoll)
    }
    unsafe fn stopInertia(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopInertia)
    }
    unsafe fn beginInteraction_withViewport_(&self, location: CGPoint, viewport: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginInteraction : location, withViewport : viewport)
    }
    unsafe fn continueInteraction_withViewport_sensitivity_(
        &self,
        location: CGPoint,
        viewport: CGSize,
        sensitivity: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, continueInteraction : location, withViewport : viewport, sensitivity : sensitivity)
    }
    unsafe fn endInteraction_withViewport_velocity_(
        &self,
        location: CGPoint,
        viewport: CGSize,
        velocity: CGPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endInteraction : location, withViewport : viewport, velocity : velocity)
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
    unsafe fn pointOfView(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfView)
    }
    unsafe fn setPointOfView_(&self, pointOfView: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfView : pointOfView)
    }
    unsafe fn interactionMode(&self) -> SCNInteractionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interactionMode)
    }
    unsafe fn setInteractionMode_(&self, interactionMode: SCNInteractionMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteractionMode : interactionMode)
    }
    unsafe fn target(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn automaticTarget(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticTarget)
    }
    unsafe fn setAutomaticTarget_(&self, automaticTarget: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticTarget : automaticTarget)
    }
    unsafe fn worldUp(&self) -> SCNVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldUp)
    }
    unsafe fn setWorldUp_(&self, worldUp: SCNVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorldUp : worldUp)
    }
    unsafe fn inertiaEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inertiaEnabled)
    }
    unsafe fn setInertiaEnabled_(&self, inertiaEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInertiaEnabled : inertiaEnabled)
    }
    unsafe fn inertiaFriction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inertiaFriction)
    }
    unsafe fn setInertiaFriction_(&self, inertiaFriction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInertiaFriction : inertiaFriction)
    }
    unsafe fn isInertiaRunning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInertiaRunning)
    }
    unsafe fn minimumVerticalAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumVerticalAngle)
    }
    unsafe fn setMinimumVerticalAngle_(&self, minimumVerticalAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumVerticalAngle : minimumVerticalAngle)
    }
    unsafe fn maximumVerticalAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumVerticalAngle)
    }
    unsafe fn setMaximumVerticalAngle_(&self, maximumVerticalAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumVerticalAngle : maximumVerticalAngle)
    }
    unsafe fn minimumHorizontalAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumHorizontalAngle)
    }
    unsafe fn setMinimumHorizontalAngle_(&self, minimumHorizontalAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumHorizontalAngle : minimumHorizontalAngle)
    }
    unsafe fn maximumHorizontalAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumHorizontalAngle)
    }
    unsafe fn setMaximumHorizontalAngle_(&self, maximumHorizontalAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumHorizontalAngle : maximumHorizontalAngle)
    }
}
impl SCNLight_SCNDeprecated for SCNLight {}
pub trait SCNLight_SCNDeprecated: Sized + std::ops::Deref {
    unsafe fn attributeForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributeForKey : key)
    }
    unsafe fn setAttribute_forKey_(&self, attribute: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttribute : attribute, forKey : key)
    }
}
impl SCNCamera_SCNDeprecated for SCNCamera {}
pub trait SCNCamera_SCNDeprecated: Sized + std::ops::Deref {
    unsafe fn focalBlurRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalBlurRadius)
    }
    unsafe fn setFocalBlurRadius_(&self, focalBlurRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalBlurRadius : focalBlurRadius)
    }
    unsafe fn xFov(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xFov)
    }
    unsafe fn setXFov_(&self, xFov: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXFov : xFov)
    }
    unsafe fn yFov(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yFov)
    }
    unsafe fn setYFov_(&self, yFov: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYFov : yFov)
    }
    unsafe fn aperture(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aperture)
    }
    unsafe fn setAperture_(&self, aperture: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAperture : aperture)
    }
    unsafe fn focalSize(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalSize)
    }
    unsafe fn setFocalSize_(&self, focalSize: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalSize : focalSize)
    }
    unsafe fn focalDistance(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalDistance)
    }
    unsafe fn setFocalDistance_(&self, focalDistance: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalDistance : focalDistance)
    }
}
impl SCNRenderer_SCNDeprecated for SCNRenderer {}
pub trait SCNRenderer_SCNDeprecated: Sized + std::ops::Deref {
    unsafe fn render(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, render)
    }
}
impl SCNMaterialProperty_SCNDeprecated for SCNMaterialProperty {}
pub trait SCNMaterialProperty_SCNDeprecated: Sized + std::ops::Deref {
    unsafe fn borderColor(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderColor)
    }
    unsafe fn setBorderColor_(&self, borderColor: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderColor : borderColor)
    }
}
impl SCNScene_SCNModelIO for SCNScene {}
pub trait SCNScene_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn sceneWithMDLAsset_(mdlAsset: MDLAsset) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNScene").unwrap(), sceneWithMDLAsset : mdlAsset)
    }
}
pub trait MDLAsset_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn assetWithSCNScene_(scnScene: SCNScene) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAsset").unwrap(), assetWithSCNScene : scnScene)
    }
    unsafe fn assetWithSCNScene_bufferAllocator_(
        scnScene: SCNScene,
        bufferAllocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAsset").unwrap(), assetWithSCNScene : scnScene, bufferAllocator : bufferAllocator)
    }
}
impl SCNNode_SCNModelIO for SCNNode {}
pub trait SCNNode_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn nodeWithMDLObject_(mdlObject: MDLObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNNode").unwrap(), nodeWithMDLObject : mdlObject)
    }
}
pub trait MDLObject_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn objectWithSCNNode_(scnNode: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLObject").unwrap(), objectWithSCNNode : scnNode)
    }
    unsafe fn objectWithSCNNode_bufferAllocator_(
        scnNode: SCNNode,
        bufferAllocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLObject").unwrap(), objectWithSCNNode : scnNode, bufferAllocator : bufferAllocator)
    }
}
impl SCNGeometry_SCNModelIO for SCNGeometry {}
pub trait SCNGeometry_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn geometryWithMDLMesh_(mdlMesh: MDLMesh) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometry").unwrap(), geometryWithMDLMesh : mdlMesh)
    }
}
pub trait MDLMesh_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn meshWithSCNGeometry_(scnGeometry: SCNGeometry) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), meshWithSCNGeometry : scnGeometry)
    }
    unsafe fn meshWithSCNGeometry_bufferAllocator_(
        scnGeometry: SCNGeometry,
        bufferAllocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), meshWithSCNGeometry : scnGeometry, bufferAllocator : bufferAllocator)
    }
}
impl SCNGeometryElement_SCNModelIO for SCNGeometryElement {}
pub trait SCNGeometryElement_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn geometryElementWithMDLSubmesh_(mdlSubMesh: MDLSubmesh) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNGeometryElement").unwrap(), geometryElementWithMDLSubmesh : mdlSubMesh)
    }
}
pub trait MDLSubmesh_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn submeshWithSCNGeometryElement_(scnGeometryElement: SCNGeometryElement) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLSubmesh").unwrap(), submeshWithSCNGeometryElement : scnGeometryElement)
    }
    unsafe fn submeshWithSCNGeometryElement_bufferAllocator_(
        scnGeometryElement: SCNGeometryElement,
        bufferAllocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLSubmesh").unwrap(), submeshWithSCNGeometryElement : scnGeometryElement, bufferAllocator : bufferAllocator)
    }
}
impl SCNMaterial_SCNModelIO for SCNMaterial {}
pub trait SCNMaterial_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn materialWithMDLMaterial_(mdlMaterial: MDLMaterial) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNMaterial").unwrap(), materialWithMDLMaterial : mdlMaterial)
    }
}
pub trait MDLMaterial_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn materialWithSCNMaterial_(scnMaterial: SCNMaterial) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMaterial").unwrap(), materialWithSCNMaterial : scnMaterial)
    }
}
impl SCNLight_SCNModelIO for SCNLight {}
pub trait SCNLight_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn lightWithMDLLight_(mdlLight: MDLLight) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNLight").unwrap(), lightWithMDLLight : mdlLight)
    }
}
pub trait MDLLight_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn lightWithSCNLight_(scnLight: SCNLight) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLLight").unwrap(), lightWithSCNLight : scnLight)
    }
}
impl SCNCamera_SCNModelIO for SCNCamera {}
pub trait SCNCamera_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn cameraWithMDLCamera_(mdlCamera: MDLCamera) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SCNCamera").unwrap(), cameraWithMDLCamera : mdlCamera)
    }
}
pub trait MDLCamera_SCNModelIO: Sized + std::ops::Deref {
    unsafe fn cameraWithSCNCamera_(scnCamera: SCNCamera) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLCamera").unwrap(), cameraWithSCNCamera : scnCamera)
    }
}
unsafe extern "C" {
    pub static SCNVector3Zero: SCNVector3;
}
unsafe extern "C" {
    pub static SCNVector4Zero: SCNVector4;
}
unsafe extern "C" {
    pub fn SCNVector3EqualToVector3(a: SCNVector3, b: SCNVector3) -> bool;
}
unsafe extern "C" {
    pub fn SCNVector4EqualToVector4(a: SCNVector4, b: SCNVector4) -> bool;
}
unsafe extern "C" {
    pub static SCNMatrix4Identity: SCNMatrix4;
}
unsafe extern "C" {
    pub fn SCNMatrix4IsIdentity(m: SCNMatrix4) -> bool;
}
unsafe extern "C" {
    pub fn SCNMatrix4EqualToMatrix4(a: SCNMatrix4, b: SCNMatrix4) -> bool;
}
unsafe extern "C" {
    pub fn SCNMatrix4MakeRotation(angle: CGFloat, x: CGFloat, y: CGFloat, z: CGFloat)
        -> SCNMatrix4;
}
unsafe extern "C" {
    pub fn SCNMatrix4Scale(m: SCNMatrix4, sx: CGFloat, sy: CGFloat, sz: CGFloat) -> SCNMatrix4;
}
unsafe extern "C" {
    pub fn SCNMatrix4Rotate(
        m: SCNMatrix4,
        angle: CGFloat,
        x: CGFloat,
        y: CGFloat,
        z: CGFloat,
    ) -> SCNMatrix4;
}
unsafe extern "C" {
    pub fn SCNMatrix4Invert(m: SCNMatrix4) -> SCNMatrix4;
}
unsafe extern "C" {
    pub fn SCNMatrix4Mult(a: SCNMatrix4, b: SCNMatrix4) -> SCNMatrix4;
}
unsafe extern "C" {
    pub fn SCNMatrix4ToGLKMatrix4(mat: SCNMatrix4) -> GLKMatrix4;
}
unsafe extern "C" {
    pub fn SCNMatrix4FromGLKMatrix4(mat: GLKMatrix4) -> SCNMatrix4;
}
unsafe extern "C" {
    pub static SCNErrorDomain: NSString;
}
unsafe extern "C" {
    pub static SCNHitTestClipToZRangeKey: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestBackFaceCullingKey: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestBoundingBoxOnlyKey: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestIgnoreChildNodesKey: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestRootNodeKey: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestIgnoreHiddenNodesKey: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestOptionCategoryBitMask: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestOptionSearchMode: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestOptionIgnoreLightArea: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestFirstFoundOnlyKey: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNHitTestSortResultsKey: SCNHitTestOption;
}
unsafe extern "C" {
    pub static SCNProgramMappingChannelKey: NSString;
}
unsafe extern "C" {
    pub static SCNShaderModifierEntryPointGeometry: SCNShaderModifierEntryPoint;
}
unsafe extern "C" {
    pub static SCNShaderModifierEntryPointSurface: SCNShaderModifierEntryPoint;
}
unsafe extern "C" {
    pub static SCNShaderModifierEntryPointLightingModel: SCNShaderModifierEntryPoint;
}
unsafe extern "C" {
    pub static SCNShaderModifierEntryPointFragment: SCNShaderModifierEntryPoint;
}
unsafe extern "C" {
    pub static SCNPreferredRenderingAPIKey: SCNViewOption;
}
unsafe extern "C" {
    pub static SCNPreferredDeviceKey: SCNViewOption;
}
unsafe extern "C" {
    pub static SCNPreferLowPowerDeviceKey: SCNViewOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetContributorsKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetCreatedDateKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetModifiedDateKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetUpAxisKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetUnitKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetAuthoringToolKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetAuthorKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetUnitNameKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetUnitMeterKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneSourceCreateNormalsIfAbsentKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceCheckConsistencyKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceFlattenSceneKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceUseSafeModeKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceAssetDirectoryURLsKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceOverrideAssetURLsKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceStrictConformanceKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceConvertUnitsToMetersKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceConvertToYUpKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceAnimationImportPolicyKey: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceLoadingOptionPreserveOriginalTopology: SCNSceneSourceLoadingOption;
}
unsafe extern "C" {
    pub static SCNSceneSourceAnimationImportPolicyPlay: SCNSceneSourceAnimationImportPolicy;
}
unsafe extern "C" {
    pub static SCNSceneSourceAnimationImportPolicyPlayRepeatedly:
        SCNSceneSourceAnimationImportPolicy;
}
unsafe extern "C" {
    pub static SCNSceneSourceAnimationImportPolicyDoNotPlay: SCNSceneSourceAnimationImportPolicy;
}
unsafe extern "C" {
    pub static SCNSceneSourceAnimationImportPolicyPlayUsingSceneTimeBase:
        SCNSceneSourceAnimationImportPolicy;
}
unsafe extern "C" {
    pub static SCNDetailedErrorsKey: NSString;
}
unsafe extern "C" {
    pub static SCNConsistencyElementIDErrorKey: NSString;
}
unsafe extern "C" {
    pub static SCNConsistencyElementTypeErrorKey: NSString;
}
unsafe extern "C" {
    pub static SCNConsistencyLineNumberErrorKey: NSString;
}
unsafe extern "C" {
    pub static SCNSceneExportDestinationURL: NSString;
}
unsafe extern "C" {
    pub static SCNSceneStartTimeAttributeKey: SCNSceneAttribute;
}
unsafe extern "C" {
    pub static SCNSceneEndTimeAttributeKey: SCNSceneAttribute;
}
unsafe extern "C" {
    pub static SCNSceneFrameRateAttributeKey: SCNSceneAttribute;
}
unsafe extern "C" {
    pub static SCNSceneUpAxisAttributeKey: SCNSceneAttribute;
}
unsafe extern "C" {
    pub static SCNModelTransform: NSString;
}
unsafe extern "C" {
    pub static SCNViewTransform: NSString;
}
unsafe extern "C" {
    pub static SCNProjectionTransform: NSString;
}
unsafe extern "C" {
    pub static SCNNormalTransform: NSString;
}
unsafe extern "C" {
    pub static SCNModelViewTransform: NSString;
}
unsafe extern "C" {
    pub static SCNModelViewProjectionTransform: NSString;
}
unsafe extern "C" {
    pub static SCNLightTypeAmbient: SCNLightType;
}
unsafe extern "C" {
    pub static SCNLightTypeOmni: SCNLightType;
}
unsafe extern "C" {
    pub static SCNLightTypeDirectional: SCNLightType;
}
unsafe extern "C" {
    pub static SCNLightTypeSpot: SCNLightType;
}
unsafe extern "C" {
    pub static SCNLightTypeIES: SCNLightType;
}
unsafe extern "C" {
    pub static SCNLightTypeProbe: SCNLightType;
}
unsafe extern "C" {
    pub static SCNLightTypeArea: SCNLightType;
}
unsafe extern "C" {
    pub static SCNLightingModelPhong: SCNLightingModel;
}
unsafe extern "C" {
    pub static SCNLightingModelBlinn: SCNLightingModel;
}
unsafe extern "C" {
    pub static SCNLightingModelLambert: SCNLightingModel;
}
unsafe extern "C" {
    pub static SCNLightingModelConstant: SCNLightingModel;
}
unsafe extern "C" {
    pub static SCNLightingModelPhysicallyBased: SCNLightingModel;
}
unsafe extern "C" {
    pub static SCNLightingModelShadowOnly: SCNLightingModel;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticVertex: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticNormal: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticColor: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticTexcoord: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticTangent: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticVertexCrease: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticEdgeCrease: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticBoneWeights: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNGeometrySourceSemanticBoneIndices: SCNGeometrySourceSemantic;
}
unsafe extern "C" {
    pub static SCNParticlePropertyPosition: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyAngle: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyRotationAxis: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyVelocity: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyAngularVelocity: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyLife: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyColor: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyOpacity: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertySize: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyFrame: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyFrameRate: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyBounce: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyCharge: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyFriction: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyContactPoint: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNParticlePropertyContactNormal: SCNParticleProperty;
}
unsafe extern "C" {
    pub static SCNPhysicsShapeTypeKey: SCNPhysicsShapeOption;
}
unsafe extern "C" {
    pub static SCNPhysicsShapeKeepAsCompoundKey: SCNPhysicsShapeOption;
}
unsafe extern "C" {
    pub static SCNPhysicsShapeScaleKey: SCNPhysicsShapeOption;
}
unsafe extern "C" {
    pub static SCNPhysicsShapeOptionCollisionMargin: SCNPhysicsShapeOption;
}
unsafe extern "C" {
    pub static SCNPhysicsShapeTypeBoundingBox: SCNPhysicsShapeType;
}
unsafe extern "C" {
    pub static SCNPhysicsShapeTypeConvexHull: SCNPhysicsShapeType;
}
unsafe extern "C" {
    pub static SCNPhysicsShapeTypeConcavePolyhedron: SCNPhysicsShapeType;
}
unsafe extern "C" {
    pub static SCNPhysicsTestCollisionBitMaskKey: SCNPhysicsTestOption;
}
unsafe extern "C" {
    pub static SCNPhysicsTestSearchModeKey: SCNPhysicsTestOption;
}
unsafe extern "C" {
    pub static SCNPhysicsTestBackfaceCullingKey: SCNPhysicsTestOption;
}
unsafe extern "C" {
    pub static SCNPhysicsTestSearchModeAny: SCNPhysicsTestSearchMode;
}
unsafe extern "C" {
    pub static SCNPhysicsTestSearchModeClosest: SCNPhysicsTestSearchMode;
}
unsafe extern "C" {
    pub static SCNPhysicsTestSearchModeAll: SCNPhysicsTestSearchMode;
}
unsafe extern "C" {
    pub fn SCNExportJavaScriptModule(context: JSContext);
}
unsafe extern "C" {
    pub static SCNLightAttenuationStartKey: NSString;
}
unsafe extern "C" {
    pub static SCNLightAttenuationEndKey: NSString;
}
unsafe extern "C" {
    pub static SCNLightAttenuationFalloffExponentKey: NSString;
}
unsafe extern "C" {
    pub static SCNLightSpotInnerAngleKey: NSString;
}
unsafe extern "C" {
    pub static SCNLightSpotOuterAngleKey: NSString;
}
unsafe extern "C" {
    pub static SCNLightShadowNearClippingKey: NSString;
}
unsafe extern "C" {
    pub static SCNLightShadowFarClippingKey: NSString;
}

unsafe impl objc2::encode::RefEncode for SCNVector3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNVector3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SCNVector3", &[]);
}
unsafe impl objc2::encode::RefEncode for SCNVector4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNVector4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SCNVector4", &[]);
}
unsafe impl objc2::encode::RefEncode for SCNTimingFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNTimingFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNAnimation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNAnimation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNAnimationPlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNAnimationPlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNAnimationEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNAnimationEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNHitTestResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNHitTestResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNProgram {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNProgram {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNTechnique {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNTechnique {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNSceneSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNSceneSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNMaterialProperty {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNMaterialProperty {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNScene {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNScene {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNLight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNLight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNCamera {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNCamera {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNMaterial {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNMaterial {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNGeometry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNGeometry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNGeometrySource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNGeometrySource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNGeometryElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNGeometryElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNGeometryTessellator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNGeometryTessellator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPlane {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPlane {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNBox {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNBox {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPyramid {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPyramid {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNSphere {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNSphere {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNCylinder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNCylinder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNCone {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNCone {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNTube {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNTube {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNCapsule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNCapsule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNTorus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNTorus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNFloor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNFloor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNShape {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNShape {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNTransaction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNTransaction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNMorpher {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNMorpher {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNSkinner {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNSkinner {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNLookAtConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNLookAtConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNBillboardConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNBillboardConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNTransformConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNTransformConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNIKConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNIKConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNDistanceConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNDistanceConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNReplicatorConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNReplicatorConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNAccelerationConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNAccelerationConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNSliderConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNSliderConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNAvoidOccluderConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNAvoidOccluderConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNLevelOfDetail {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNLevelOfDetail {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNParticlePropertyController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNParticlePropertyController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNParticleSystem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNParticleSystem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsBody {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsBody {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsField {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsField {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsShape {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsShape {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsWorld {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsWorld {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsContact {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsContact {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsBehavior {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsBehavior {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsHingeJoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsHingeJoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsBallSocketJoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsBallSocketJoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsSliderJoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsSliderJoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsConeTwistJoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsConeTwistJoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsVehicleWheel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsVehicleWheel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNPhysicsVehicle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNPhysicsVehicle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNReferenceNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNReferenceNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNAudioSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNAudioSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNAudioPlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNAudioPlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCNCameraController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNCameraController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
