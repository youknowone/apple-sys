#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolEffectOptionsRepeatBehavior(pub id);
impl std::ops::Deref for NSSymbolEffectOptionsRepeatBehavior {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolEffectOptionsRepeatBehavior {}
impl NSSymbolEffectOptionsRepeatBehavior {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptionsRepeatBehavior").unwrap(), alloc) })
    }
}
impl PNSCopying for NSSymbolEffectOptionsRepeatBehavior {}
impl PNSSecureCoding for NSSymbolEffectOptionsRepeatBehavior {}
impl INSObject for NSSymbolEffectOptionsRepeatBehavior {}
impl PNSObject for NSSymbolEffectOptionsRepeatBehavior {}
impl std::convert::TryFrom<NSObject> for NSSymbolEffectOptionsRepeatBehavior {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSSymbolEffectOptionsRepeatBehavior, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolEffectOptionsRepeatBehavior").unwrap())
        };
        if is_kind_of {
            Ok(NSSymbolEffectOptionsRepeatBehavior(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSSymbolEffectOptionsRepeatBehavior")
        }
    }
}
impl INSSymbolEffectOptionsRepeatBehavior for NSSymbolEffectOptionsRepeatBehavior {}
pub trait INSSymbolEffectOptionsRepeatBehavior: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptionsRepeatBehavior").unwrap(), new)
    }
    unsafe fn behaviorPeriodic() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptionsRepeatBehavior").unwrap(), behaviorPeriodic)
    }
    unsafe fn behaviorPeriodicWithCount_(count: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptionsRepeatBehavior").unwrap(), behaviorPeriodicWithCount : count)
    }
    unsafe fn behaviorPeriodicWithDelay_(delay: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptionsRepeatBehavior").unwrap(), behaviorPeriodicWithDelay : delay)
    }
    unsafe fn behaviorPeriodicWithCount_delay_(count: NSInteger, delay: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptionsRepeatBehavior").unwrap(), behaviorPeriodicWithCount : count, delay : delay)
    }
    unsafe fn behaviorContinuous() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptionsRepeatBehavior").unwrap(), behaviorContinuous)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolEffectOptions(pub id);
impl std::ops::Deref for NSSymbolEffectOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolEffectOptions {}
impl NSSymbolEffectOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for NSSymbolEffectOptions {}
impl PNSSecureCoding for NSSymbolEffectOptions {}
impl INSObject for NSSymbolEffectOptions {}
impl PNSObject for NSSymbolEffectOptions {}
impl std::convert::TryFrom<NSObject> for NSSymbolEffectOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSSymbolEffectOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolEffectOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSSymbolEffectOptions")
        }
    }
}
impl INSSymbolEffectOptions for NSSymbolEffectOptions {}
pub trait INSSymbolEffectOptions: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn optionsWithRepeating(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optionsWithRepeating)
    }
    unsafe fn optionsWithNonRepeating(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optionsWithNonRepeating)
    }
    unsafe fn optionsWithRepeatCount_(&self, count: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optionsWithRepeatCount : count)
    }
    unsafe fn optionsWithSpeed_(&self, speed: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optionsWithSpeed : speed)
    }
    unsafe fn optionsWithRepeatBehavior_(
        &self,
        behavior: NSSymbolEffectOptionsRepeatBehavior,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optionsWithRepeatBehavior : behavior)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap(), new)
    }
    unsafe fn options() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap(), options)
    }
    unsafe fn class_optionsWithRepeating() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap(), optionsWithRepeating)
    }
    unsafe fn class_optionsWithNonRepeating() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap(), optionsWithNonRepeating)
    }
    unsafe fn class_optionsWithRepeatCount_(count: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap(), optionsWithRepeatCount : count)
    }
    unsafe fn class_optionsWithSpeed_(speed: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap(), optionsWithSpeed : speed)
    }
    unsafe fn class_optionsWithRepeatBehavior_(
        behavior: NSSymbolEffectOptionsRepeatBehavior,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffectOptions").unwrap(), optionsWithRepeatBehavior : behavior)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolEffect(pub id);
impl std::ops::Deref for NSSymbolEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolEffect {}
impl NSSymbolEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffect").unwrap(), alloc) })
    }
}
impl PNSCopying for NSSymbolEffect {}
impl PNSSecureCoding for NSSymbolEffect {}
impl INSObject for NSSymbolEffect {}
impl PNSObject for NSSymbolEffect {}
impl std::convert::TryFrom<NSObject> for NSSymbolEffect {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSSymbolEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolEffect(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSSymbolEffect")
        }
    }
}
impl INSSymbolEffect for NSSymbolEffect {}
pub trait INSSymbolEffect: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolEffect").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolPulseEffect(pub id);
impl std::ops::Deref for NSSymbolPulseEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolPulseEffect {}
impl NSSymbolPulseEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolPulseEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolPulseEffect {}
impl PNSCopying for NSSymbolPulseEffect {}
impl PNSSecureCoding for NSSymbolPulseEffect {}
impl From<NSSymbolPulseEffect> for NSSymbolEffect {
    fn from(child: NSSymbolPulseEffect) -> NSSymbolEffect {
        NSSymbolEffect(child.0)
    }
}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolPulseEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolPulseEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolPulseEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolPulseEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolPulseEffect")
        }
    }
}
impl INSObject for NSSymbolPulseEffect {}
impl PNSObject for NSSymbolPulseEffect {}
impl INSSymbolPulseEffect for NSSymbolPulseEffect {}
pub trait INSSymbolPulseEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolPulseEffect").unwrap(), effect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolBounceEffect(pub id);
impl std::ops::Deref for NSSymbolBounceEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolBounceEffect {}
impl NSSymbolBounceEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolBounceEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolBounceEffect {}
impl PNSCopying for NSSymbolBounceEffect {}
impl PNSSecureCoding for NSSymbolBounceEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolBounceEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolBounceEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolBounceEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolBounceEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolBounceEffect")
        }
    }
}
impl INSObject for NSSymbolBounceEffect {}
impl PNSObject for NSSymbolBounceEffect {}
impl INSSymbolBounceEffect for NSSymbolBounceEffect {}
pub trait INSSymbolBounceEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolBounceEffect").unwrap(), effect)
    }
    unsafe fn bounceUpEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolBounceEffect").unwrap(), bounceUpEffect)
    }
    unsafe fn bounceDownEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolBounceEffect").unwrap(), bounceDownEffect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolVariableColorEffect(pub id);
impl std::ops::Deref for NSSymbolVariableColorEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolVariableColorEffect {}
impl NSSymbolVariableColorEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolVariableColorEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolVariableColorEffect {}
impl PNSCopying for NSSymbolVariableColorEffect {}
impl PNSSecureCoding for NSSymbolVariableColorEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolVariableColorEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolVariableColorEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolVariableColorEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolVariableColorEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolVariableColorEffect")
        }
    }
}
impl INSObject for NSSymbolVariableColorEffect {}
impl PNSObject for NSSymbolVariableColorEffect {}
impl INSSymbolVariableColorEffect for NSSymbolVariableColorEffect {}
pub trait INSSymbolVariableColorEffect: Sized + std::ops::Deref {
    unsafe fn effectWithIterative(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithIterative)
    }
    unsafe fn effectWithCumulative(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithCumulative)
    }
    unsafe fn effectWithReversing(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithReversing)
    }
    unsafe fn effectWithNonReversing(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithNonReversing)
    }
    unsafe fn effectWithHideInactiveLayers(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithHideInactiveLayers)
    }
    unsafe fn effectWithDimInactiveLayers(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithDimInactiveLayers)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolVariableColorEffect").unwrap(), effect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolScaleEffect(pub id);
impl std::ops::Deref for NSSymbolScaleEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolScaleEffect {}
impl NSSymbolScaleEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolScaleEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolScaleEffect {}
impl PNSCopying for NSSymbolScaleEffect {}
impl PNSSecureCoding for NSSymbolScaleEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolScaleEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolScaleEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolScaleEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolScaleEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolScaleEffect")
        }
    }
}
impl INSObject for NSSymbolScaleEffect {}
impl PNSObject for NSSymbolScaleEffect {}
impl INSSymbolScaleEffect for NSSymbolScaleEffect {}
pub trait INSSymbolScaleEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolScaleEffect").unwrap(), effect)
    }
    unsafe fn scaleUpEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolScaleEffect").unwrap(), scaleUpEffect)
    }
    unsafe fn scaleDownEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolScaleEffect").unwrap(), scaleDownEffect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolAppearEffect(pub id);
impl std::ops::Deref for NSSymbolAppearEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolAppearEffect {}
impl NSSymbolAppearEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolAppearEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolAppearEffect {}
impl PNSCopying for NSSymbolAppearEffect {}
impl PNSSecureCoding for NSSymbolAppearEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolAppearEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolAppearEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolAppearEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolAppearEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolAppearEffect")
        }
    }
}
impl INSObject for NSSymbolAppearEffect {}
impl PNSObject for NSSymbolAppearEffect {}
impl INSSymbolAppearEffect for NSSymbolAppearEffect {}
pub trait INSSymbolAppearEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolAppearEffect").unwrap(), effect)
    }
    unsafe fn appearUpEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolAppearEffect").unwrap(), appearUpEffect)
    }
    unsafe fn appearDownEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolAppearEffect").unwrap(), appearDownEffect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolDisappearEffect(pub id);
impl std::ops::Deref for NSSymbolDisappearEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolDisappearEffect {}
impl NSSymbolDisappearEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolDisappearEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolDisappearEffect {}
impl PNSCopying for NSSymbolDisappearEffect {}
impl PNSSecureCoding for NSSymbolDisappearEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolDisappearEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolDisappearEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolDisappearEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolDisappearEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolDisappearEffect")
        }
    }
}
impl INSObject for NSSymbolDisappearEffect {}
impl PNSObject for NSSymbolDisappearEffect {}
impl INSSymbolDisappearEffect for NSSymbolDisappearEffect {}
pub trait INSSymbolDisappearEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolDisappearEffect").unwrap(), effect)
    }
    unsafe fn disappearUpEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolDisappearEffect").unwrap(), disappearUpEffect)
    }
    unsafe fn disappearDownEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolDisappearEffect").unwrap(), disappearDownEffect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolWiggleEffect(pub id);
impl std::ops::Deref for NSSymbolWiggleEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolWiggleEffect {}
impl NSSymbolWiggleEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolWiggleEffect {}
impl PNSCopying for NSSymbolWiggleEffect {}
impl PNSSecureCoding for NSSymbolWiggleEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolWiggleEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolWiggleEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolWiggleEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolWiggleEffect")
        }
    }
}
impl INSObject for NSSymbolWiggleEffect {}
impl PNSObject for NSSymbolWiggleEffect {}
impl INSSymbolWiggleEffect for NSSymbolWiggleEffect {}
pub trait INSSymbolWiggleEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), effect)
    }
    unsafe fn wiggleClockwiseEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleClockwiseEffect)
    }
    unsafe fn wiggleCounterClockwiseEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleCounterClockwiseEffect)
    }
    unsafe fn wiggleLeftEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleLeftEffect)
    }
    unsafe fn wiggleRightEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleRightEffect)
    }
    unsafe fn wiggleUpEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleUpEffect)
    }
    unsafe fn wiggleDownEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleDownEffect)
    }
    unsafe fn wiggleForwardEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleForwardEffect)
    }
    unsafe fn wiggleBackwardEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleBackwardEffect)
    }
    unsafe fn wiggleCustomAngleEffect_(angle: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolWiggleEffect").unwrap(), wiggleCustomAngleEffect : angle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolRotateEffect(pub id);
impl std::ops::Deref for NSSymbolRotateEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolRotateEffect {}
impl NSSymbolRotateEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolRotateEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolRotateEffect {}
impl PNSCopying for NSSymbolRotateEffect {}
impl PNSSecureCoding for NSSymbolRotateEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolRotateEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolRotateEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolRotateEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolRotateEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolRotateEffect")
        }
    }
}
impl INSObject for NSSymbolRotateEffect {}
impl PNSObject for NSSymbolRotateEffect {}
impl INSSymbolRotateEffect for NSSymbolRotateEffect {}
pub trait INSSymbolRotateEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolRotateEffect").unwrap(), effect)
    }
    unsafe fn rotateClockwiseEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolRotateEffect").unwrap(), rotateClockwiseEffect)
    }
    unsafe fn rotateCounterClockwiseEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolRotateEffect").unwrap(), rotateCounterClockwiseEffect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolBreatheEffect(pub id);
impl std::ops::Deref for NSSymbolBreatheEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolBreatheEffect {}
impl NSSymbolBreatheEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolBreatheEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolBreatheEffect {}
impl PNSCopying for NSSymbolBreatheEffect {}
impl PNSSecureCoding for NSSymbolBreatheEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolBreatheEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolBreatheEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolBreatheEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolBreatheEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolBreatheEffect")
        }
    }
}
impl INSObject for NSSymbolBreatheEffect {}
impl PNSObject for NSSymbolBreatheEffect {}
impl INSSymbolBreatheEffect for NSSymbolBreatheEffect {}
pub trait INSSymbolBreatheEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolBreatheEffect").unwrap(), effect)
    }
    unsafe fn breathePulseEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolBreatheEffect").unwrap(), breathePulseEffect)
    }
    unsafe fn breathePlainEffect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolBreatheEffect").unwrap(), breathePlainEffect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolDrawOnEffect(pub id);
impl std::ops::Deref for NSSymbolDrawOnEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolDrawOnEffect {}
impl NSSymbolDrawOnEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolDrawOnEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolDrawOnEffect {}
impl PNSCopying for NSSymbolDrawOnEffect {}
impl PNSSecureCoding for NSSymbolDrawOnEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolDrawOnEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolDrawOnEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolDrawOnEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolDrawOnEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolDrawOnEffect")
        }
    }
}
impl INSObject for NSSymbolDrawOnEffect {}
impl PNSObject for NSSymbolDrawOnEffect {}
impl INSSymbolDrawOnEffect for NSSymbolDrawOnEffect {}
pub trait INSSymbolDrawOnEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effectWithIndividually(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithIndividually)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolDrawOnEffect").unwrap(), effect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolDrawOffEffect(pub id);
impl std::ops::Deref for NSSymbolDrawOffEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolDrawOffEffect {}
impl NSSymbolDrawOffEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolDrawOffEffect").unwrap(), alloc) })
    }
}
impl INSSymbolEffect for NSSymbolDrawOffEffect {}
impl PNSCopying for NSSymbolDrawOffEffect {}
impl PNSSecureCoding for NSSymbolDrawOffEffect {}
impl std::convert::TryFrom<NSSymbolEffect> for NSSymbolDrawOffEffect {
    type Error = &'static str;
    fn try_from(parent: NSSymbolEffect) -> Result<NSSymbolDrawOffEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolDrawOffEffect").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolDrawOffEffect(parent.0))
        } else {
            Err("This NSSymbolEffect cannot be downcasted to NSSymbolDrawOffEffect")
        }
    }
}
impl INSObject for NSSymbolDrawOffEffect {}
impl PNSObject for NSSymbolDrawOffEffect {}
impl INSSymbolDrawOffEffect for NSSymbolDrawOffEffect {}
pub trait INSSymbolDrawOffEffect: Sized + std::ops::Deref {
    unsafe fn effectWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithByLayer)
    }
    unsafe fn effectWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithWholeSymbol)
    }
    unsafe fn effectWithIndividually(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithIndividually)
    }
    unsafe fn effectWithReversed(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithReversed)
    }
    unsafe fn effectWithNonReversed(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectWithNonReversed)
    }
    unsafe fn effect() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolDrawOffEffect").unwrap(), effect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolContentTransition(pub id);
impl std::ops::Deref for NSSymbolContentTransition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolContentTransition {}
impl NSSymbolContentTransition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolContentTransition").unwrap(), alloc) })
    }
}
impl PNSCopying for NSSymbolContentTransition {}
impl PNSSecureCoding for NSSymbolContentTransition {}
impl INSObject for NSSymbolContentTransition {}
impl PNSObject for NSSymbolContentTransition {}
impl std::convert::TryFrom<NSObject> for NSSymbolContentTransition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSSymbolContentTransition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolContentTransition").unwrap()) };
        if is_kind_of {
            Ok(NSSymbolContentTransition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSSymbolContentTransition")
        }
    }
}
impl INSSymbolContentTransition for NSSymbolContentTransition {}
pub trait INSSymbolContentTransition: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolContentTransition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolMagicReplaceContentTransition(pub id);
impl std::ops::Deref for NSSymbolMagicReplaceContentTransition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolMagicReplaceContentTransition {}
impl NSSymbolMagicReplaceContentTransition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolMagicReplaceContentTransition").unwrap(), alloc) })
    }
}
impl INSSymbolContentTransition for NSSymbolMagicReplaceContentTransition {}
impl PNSCopying for NSSymbolMagicReplaceContentTransition {}
impl PNSSecureCoding for NSSymbolMagicReplaceContentTransition {}
impl From<NSSymbolMagicReplaceContentTransition> for NSSymbolContentTransition {
    fn from(child: NSSymbolMagicReplaceContentTransition) -> NSSymbolContentTransition {
        NSSymbolContentTransition(child.0)
    }
}
impl std::convert::TryFrom<NSSymbolContentTransition> for NSSymbolMagicReplaceContentTransition {
    type Error = &'static str;
    fn try_from(
        parent: NSSymbolContentTransition,
    ) -> Result<NSSymbolMagicReplaceContentTransition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolMagicReplaceContentTransition").unwrap())
        };
        if is_kind_of {
            Ok(NSSymbolMagicReplaceContentTransition(parent.0))
        } else {
            Err ("This NSSymbolContentTransition cannot be downcasted to NSSymbolMagicReplaceContentTransition" ,)
        }
    }
}
impl INSObject for NSSymbolMagicReplaceContentTransition {}
impl PNSObject for NSSymbolMagicReplaceContentTransition {}
impl INSSymbolMagicReplaceContentTransition for NSSymbolMagicReplaceContentTransition {}
pub trait INSSymbolMagicReplaceContentTransition: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolReplaceContentTransition(pub id);
impl std::ops::Deref for NSSymbolReplaceContentTransition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolReplaceContentTransition {}
impl NSSymbolReplaceContentTransition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolReplaceContentTransition").unwrap(), alloc) })
    }
}
impl INSSymbolContentTransition for NSSymbolReplaceContentTransition {}
impl PNSCopying for NSSymbolReplaceContentTransition {}
impl PNSSecureCoding for NSSymbolReplaceContentTransition {}
impl std::convert::TryFrom<NSSymbolContentTransition> for NSSymbolReplaceContentTransition {
    type Error = &'static str;
    fn try_from(
        parent: NSSymbolContentTransition,
    ) -> Result<NSSymbolReplaceContentTransition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolReplaceContentTransition").unwrap())
        };
        if is_kind_of {
            Ok(NSSymbolReplaceContentTransition(parent.0))
        } else {
            Err ("This NSSymbolContentTransition cannot be downcasted to NSSymbolReplaceContentTransition" ,)
        }
    }
}
impl INSObject for NSSymbolReplaceContentTransition {}
impl PNSObject for NSSymbolReplaceContentTransition {}
impl INSSymbolReplaceContentTransition for NSSymbolReplaceContentTransition {}
pub trait INSSymbolReplaceContentTransition: Sized + std::ops::Deref {
    unsafe fn transitionWithByLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transitionWithByLayer)
    }
    unsafe fn transitionWithWholeSymbol(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transitionWithWholeSymbol)
    }
    unsafe fn transition() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolReplaceContentTransition").unwrap(), transition)
    }
    unsafe fn replaceDownUpTransition() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolReplaceContentTransition").unwrap(), replaceDownUpTransition)
    }
    unsafe fn replaceUpUpTransition() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolReplaceContentTransition").unwrap(), replaceUpUpTransition)
    }
    unsafe fn replaceOffUpTransition() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolReplaceContentTransition").unwrap(), replaceOffUpTransition)
    }
    unsafe fn magicTransitionWithFallback_(
        fallback: NSSymbolReplaceContentTransition,
    ) -> NSSymbolMagicReplaceContentTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolReplaceContentTransition").unwrap(), magicTransitionWithFallback : fallback)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSSymbolAutomaticContentTransition(pub id);
impl std::ops::Deref for NSSymbolAutomaticContentTransition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSSymbolAutomaticContentTransition {}
impl NSSymbolAutomaticContentTransition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolAutomaticContentTransition").unwrap(), alloc) })
    }
}
impl INSSymbolContentTransition for NSSymbolAutomaticContentTransition {}
impl PNSCopying for NSSymbolAutomaticContentTransition {}
impl PNSSecureCoding for NSSymbolAutomaticContentTransition {}
impl std::convert::TryFrom<NSSymbolContentTransition> for NSSymbolAutomaticContentTransition {
    type Error = &'static str;
    fn try_from(
        parent: NSSymbolContentTransition,
    ) -> Result<NSSymbolAutomaticContentTransition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSSymbolAutomaticContentTransition").unwrap())
        };
        if is_kind_of {
            Ok(NSSymbolAutomaticContentTransition(parent.0))
        } else {
            Err ("This NSSymbolContentTransition cannot be downcasted to NSSymbolAutomaticContentTransition" ,)
        }
    }
}
impl INSObject for NSSymbolAutomaticContentTransition {}
impl PNSObject for NSSymbolAutomaticContentTransition {}
impl INSSymbolAutomaticContentTransition for NSSymbolAutomaticContentTransition {}
pub trait INSSymbolAutomaticContentTransition: Sized + std::ops::Deref {
    unsafe fn transition() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSymbolAutomaticContentTransition").unwrap(), transition)
    }
}

unsafe impl objc2::encode::RefEncode for NSSymbolEffectOptionsRepeatBehavior {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolEffectOptionsRepeatBehavior {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolEffectOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolEffectOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolPulseEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolPulseEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolBounceEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolBounceEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolVariableColorEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolVariableColorEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolScaleEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolScaleEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolAppearEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolAppearEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolDisappearEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolDisappearEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolWiggleEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolWiggleEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolRotateEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolRotateEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolBreatheEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolBreatheEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolDrawOnEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolDrawOnEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolDrawOffEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolDrawOffEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolContentTransition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolContentTransition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolMagicReplaceContentTransition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolMagicReplaceContentTransition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolReplaceContentTransition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolReplaceContentTransition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSSymbolAutomaticContentTransition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSSymbolAutomaticContentTransition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
