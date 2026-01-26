#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type LABiometryType = NSInteger;
pub type LACompanionType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LADomainStateBiometry(pub id);
impl std::ops::Deref for LADomainStateBiometry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LADomainStateBiometry {}
impl LADomainStateBiometry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LADomainStateBiometry").unwrap(), alloc) })
    }
}
impl INSObject for LADomainStateBiometry {}
impl PNSObject for LADomainStateBiometry {}
impl std::convert::TryFrom<NSObject> for LADomainStateBiometry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LADomainStateBiometry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LADomainStateBiometry").unwrap()) };
        if is_kind_of {
            Ok(LADomainStateBiometry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LADomainStateBiometry")
        }
    }
}
impl ILADomainStateBiometry for LADomainStateBiometry {}
pub trait ILADomainStateBiometry: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn biometryType(&self) -> LABiometryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biometryType)
    }
    unsafe fn stateHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateHash)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LADomainStateBiometry").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LADomainStateCompanion(pub id);
impl std::ops::Deref for LADomainStateCompanion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LADomainStateCompanion {}
impl LADomainStateCompanion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LADomainStateCompanion").unwrap(), alloc) })
    }
}
impl INSObject for LADomainStateCompanion {}
impl PNSObject for LADomainStateCompanion {}
impl std::convert::TryFrom<NSObject> for LADomainStateCompanion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LADomainStateCompanion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LADomainStateCompanion").unwrap()) };
        if is_kind_of {
            Ok(LADomainStateCompanion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LADomainStateCompanion")
        }
    }
}
impl ILADomainStateCompanion for LADomainStateCompanion {}
pub trait ILADomainStateCompanion: Sized + std::ops::Deref {
    unsafe fn stateHashForCompanionType_(&self, companionType: LACompanionType) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stateHashForCompanionType : companionType)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn availableCompanionTypes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableCompanionTypes)
    }
    unsafe fn stateHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateHash)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LADomainStateCompanion").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LADomainState(pub id);
impl std::ops::Deref for LADomainState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LADomainState {}
impl LADomainState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LADomainState").unwrap(), alloc) })
    }
}
impl INSObject for LADomainState {}
impl PNSObject for LADomainState {}
impl std::convert::TryFrom<NSObject> for LADomainState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LADomainState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LADomainState").unwrap()) };
        if is_kind_of {
            Ok(LADomainState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LADomainState")
        }
    }
}
impl ILADomainState for LADomainState {}
pub trait ILADomainState: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn biometry(&self) -> LADomainStateBiometry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biometry)
    }
    unsafe fn companion(&self) -> LADomainStateCompanion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, companion)
    }
    unsafe fn stateHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateHash)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LADomainState").unwrap(), new)
    }
}
pub type LAPolicy = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAContext(pub id);
impl std::ops::Deref for LAContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAContext {}
impl LAContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAContext").unwrap(), alloc) })
    }
}
impl INSObject for LAContext {}
impl PNSObject for LAContext {}
impl std::convert::TryFrom<NSObject> for LAContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LAContext, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAContext").unwrap()) };
        if is_kind_of {
            Ok(LAContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LAContext")
        }
    }
}
impl ILAContext for LAContext {}
pub trait ILAContext: Sized + std::ops::Deref {
    unsafe fn canEvaluatePolicy_error_(&self, policy: LAPolicy, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canEvaluatePolicy : policy, error : error)
    }
    unsafe fn evaluatePolicy_localizedReason_reply_(
        &self,
        policy: LAPolicy,
        localizedReason: NSString,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluatePolicy : policy, localizedReason : localizedReason, reply : reply)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn setCredential_type_(&self, credential: NSData, type_: LACredentialType) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCredential : credential, r#type : type_)
    }
    unsafe fn isCredentialSet_(&self, type_: LACredentialType) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isCredentialSet : type_)
    }
    unsafe fn evaluateAccessControl_operation_localizedReason_reply_(
        &self,
        accessControl: SecAccessControlRef,
        operation: LAAccessControlOperation,
        localizedReason: NSString,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateAccessControl : accessControl, operation : operation, localizedReason : localizedReason, reply : reply)
    }
    unsafe fn localizedFallbackTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedFallbackTitle)
    }
    unsafe fn setLocalizedFallbackTitle_(&self, localizedFallbackTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedFallbackTitle : localizedFallbackTitle)
    }
    unsafe fn maxBiometryFailures(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxBiometryFailures)
    }
    unsafe fn setMaxBiometryFailures_(&self, maxBiometryFailures: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxBiometryFailures : maxBiometryFailures)
    }
    unsafe fn localizedCancelTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedCancelTitle)
    }
    unsafe fn setLocalizedCancelTitle_(&self, localizedCancelTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedCancelTitle : localizedCancelTitle)
    }
    unsafe fn touchIDAuthenticationAllowableReuseDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchIDAuthenticationAllowableReuseDuration)
    }
    unsafe fn setTouchIDAuthenticationAllowableReuseDuration_(
        &self,
        touchIDAuthenticationAllowableReuseDuration: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTouchIDAuthenticationAllowableReuseDuration : touchIDAuthenticationAllowableReuseDuration)
    }
    unsafe fn localizedReason(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedReason)
    }
    unsafe fn setLocalizedReason_(&self, localizedReason: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedReason : localizedReason)
    }
    unsafe fn interactionNotAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interactionNotAllowed)
    }
    unsafe fn setInteractionNotAllowed_(&self, interactionNotAllowed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteractionNotAllowed : interactionNotAllowed)
    }
    unsafe fn biometryType(&self) -> LABiometryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biometryType)
    }
    unsafe fn evaluatedPolicyDomainState(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, evaluatedPolicyDomainState)
    }
    unsafe fn domainState(&self) -> LADomainState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainState)
    }
}
pub type LACredentialType = NSInteger;
pub type LAAccessControlOperation = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAEnvironmentMechanism(pub id);
impl std::ops::Deref for LAEnvironmentMechanism {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAEnvironmentMechanism {}
impl LAEnvironmentMechanism {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironmentMechanism").unwrap(), alloc) })
    }
}
impl INSObject for LAEnvironmentMechanism {}
impl PNSObject for LAEnvironmentMechanism {}
impl std::convert::TryFrom<NSObject> for LAEnvironmentMechanism {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LAEnvironmentMechanism, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAEnvironmentMechanism").unwrap()) };
        if is_kind_of {
            Ok(LAEnvironmentMechanism(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LAEnvironmentMechanism")
        }
    }
}
impl ILAEnvironmentMechanism for LAEnvironmentMechanism {}
pub trait ILAEnvironmentMechanism: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isUsable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUsable)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn iconSystemName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconSystemName)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironmentMechanism").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAEnvironmentMechanismBiometry(pub id);
impl std::ops::Deref for LAEnvironmentMechanismBiometry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAEnvironmentMechanismBiometry {}
impl LAEnvironmentMechanismBiometry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironmentMechanismBiometry").unwrap(), alloc) })
    }
}
impl ILAEnvironmentMechanism for LAEnvironmentMechanismBiometry {}
impl From<LAEnvironmentMechanismBiometry> for LAEnvironmentMechanism {
    fn from(child: LAEnvironmentMechanismBiometry) -> LAEnvironmentMechanism {
        LAEnvironmentMechanism(child.0)
    }
}
impl std::convert::TryFrom<LAEnvironmentMechanism> for LAEnvironmentMechanismBiometry {
    type Error = &'static str;
    fn try_from(
        parent: LAEnvironmentMechanism,
    ) -> Result<LAEnvironmentMechanismBiometry, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAEnvironmentMechanismBiometry").unwrap())
        };
        if is_kind_of {
            Ok(LAEnvironmentMechanismBiometry(parent.0))
        } else {
            Err ("This LAEnvironmentMechanism cannot be downcasted to LAEnvironmentMechanismBiometry" ,)
        }
    }
}
impl INSObject for LAEnvironmentMechanismBiometry {}
impl PNSObject for LAEnvironmentMechanismBiometry {}
impl ILAEnvironmentMechanismBiometry for LAEnvironmentMechanismBiometry {}
pub trait ILAEnvironmentMechanismBiometry: Sized + std::ops::Deref {
    unsafe fn biometryType(&self) -> LABiometryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biometryType)
    }
    unsafe fn isEnrolled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnrolled)
    }
    unsafe fn isLockedOut(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLockedOut)
    }
    unsafe fn stateHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateHash)
    }
    unsafe fn builtInSensorInaccessible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, builtInSensorInaccessible)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAEnvironmentMechanismCompanion(pub id);
impl std::ops::Deref for LAEnvironmentMechanismCompanion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAEnvironmentMechanismCompanion {}
impl LAEnvironmentMechanismCompanion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironmentMechanismCompanion").unwrap(), alloc) })
    }
}
impl ILAEnvironmentMechanism for LAEnvironmentMechanismCompanion {}
impl std::convert::TryFrom<LAEnvironmentMechanism> for LAEnvironmentMechanismCompanion {
    type Error = &'static str;
    fn try_from(
        parent: LAEnvironmentMechanism,
    ) -> Result<LAEnvironmentMechanismCompanion, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAEnvironmentMechanismCompanion").unwrap())
        };
        if is_kind_of {
            Ok(LAEnvironmentMechanismCompanion(parent.0))
        } else {
            Err ("This LAEnvironmentMechanism cannot be downcasted to LAEnvironmentMechanismCompanion" ,)
        }
    }
}
impl INSObject for LAEnvironmentMechanismCompanion {}
impl PNSObject for LAEnvironmentMechanismCompanion {}
impl ILAEnvironmentMechanismCompanion for LAEnvironmentMechanismCompanion {}
pub trait ILAEnvironmentMechanismCompanion: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> LACompanionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn stateHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateHash)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAEnvironmentMechanismUserPassword(pub id);
impl std::ops::Deref for LAEnvironmentMechanismUserPassword {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAEnvironmentMechanismUserPassword {}
impl LAEnvironmentMechanismUserPassword {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironmentMechanismUserPassword").unwrap(), alloc) })
    }
}
impl ILAEnvironmentMechanism for LAEnvironmentMechanismUserPassword {}
impl std::convert::TryFrom<LAEnvironmentMechanism> for LAEnvironmentMechanismUserPassword {
    type Error = &'static str;
    fn try_from(
        parent: LAEnvironmentMechanism,
    ) -> Result<LAEnvironmentMechanismUserPassword, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAEnvironmentMechanismUserPassword").unwrap())
        };
        if is_kind_of {
            Ok(LAEnvironmentMechanismUserPassword(parent.0))
        } else {
            Err ("This LAEnvironmentMechanism cannot be downcasted to LAEnvironmentMechanismUserPassword" ,)
        }
    }
}
impl INSObject for LAEnvironmentMechanismUserPassword {}
impl PNSObject for LAEnvironmentMechanismUserPassword {}
impl ILAEnvironmentMechanismUserPassword for LAEnvironmentMechanismUserPassword {}
pub trait ILAEnvironmentMechanismUserPassword: Sized + std::ops::Deref {
    unsafe fn isSet(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSet)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAEnvironmentState(pub id);
impl std::ops::Deref for LAEnvironmentState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAEnvironmentState {}
impl LAEnvironmentState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironmentState").unwrap(), alloc) })
    }
}
impl PNSCopying for LAEnvironmentState {}
impl INSObject for LAEnvironmentState {}
impl PNSObject for LAEnvironmentState {}
impl std::convert::TryFrom<NSObject> for LAEnvironmentState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LAEnvironmentState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAEnvironmentState").unwrap()) };
        if is_kind_of {
            Ok(LAEnvironmentState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LAEnvironmentState")
        }
    }
}
impl ILAEnvironmentState for LAEnvironmentState {}
pub trait ILAEnvironmentState: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn biometry(&self) -> LAEnvironmentMechanismBiometry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biometry)
    }
    unsafe fn userPassword(&self) -> LAEnvironmentMechanismUserPassword
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userPassword)
    }
    unsafe fn companions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, companions)
    }
    unsafe fn allMechanisms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allMechanisms)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironmentState").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAEnvironment(pub id);
impl std::ops::Deref for LAEnvironment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAEnvironment {}
impl LAEnvironment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironment").unwrap(), alloc) })
    }
}
impl INSObject for LAEnvironment {}
impl PNSObject for LAEnvironment {}
impl std::convert::TryFrom<NSObject> for LAEnvironment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LAEnvironment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAEnvironment").unwrap()) };
        if is_kind_of {
            Ok(LAEnvironment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LAEnvironment")
        }
    }
}
impl ILAEnvironment for LAEnvironment {}
pub trait ILAEnvironment: Sized + std::ops::Deref {
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
    unsafe fn state(&self) -> LAEnvironmentState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironment").unwrap(), new)
    }
    unsafe fn currentUser() -> LAEnvironment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAEnvironment").unwrap(), currentUser)
    }
}
pub trait PLAEnvironmentObserver: Sized + std::ops::Deref {
    unsafe fn environment_stateDidChangeFromOldState_(
        &self,
        environment: LAEnvironment,
        oldState: LAEnvironmentState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, environment : environment, stateDidChangeFromOldState : oldState)
    }
}
pub type LAError = NSInteger;
pub type LARightState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LARight(pub id);
impl std::ops::Deref for LARight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LARight {}
impl LARight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LARight").unwrap(), alloc) })
    }
}
impl INSObject for LARight {}
impl PNSObject for LARight {}
impl std::convert::TryFrom<NSObject> for LARight {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LARight, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LARight").unwrap()) };
        if is_kind_of {
            Ok(LARight(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LARight")
        }
    }
}
impl ILARight for LARight {}
pub trait ILARight: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRequirement_(&self, requirement: LAAuthenticationRequirement) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRequirement : requirement)
    }
    unsafe fn authorizeWithLocalizedReason_completion_(
        &self,
        localizedReason: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizeWithLocalizedReason : localizedReason, completion : handler)
    }
    unsafe fn checkCanAuthorizeWithCompletion_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkCanAuthorizeWithCompletion : handler)
    }
    unsafe fn deauthorizeWithCompletion_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deauthorizeWithCompletion : handler)
    }
    unsafe fn state(&self) -> LARightState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn tag(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tag)
    }
    unsafe fn setTag_(&self, tag: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTag : tag)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAPersistedRight(pub id);
impl std::ops::Deref for LAPersistedRight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAPersistedRight {}
impl LAPersistedRight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAPersistedRight").unwrap(), alloc) })
    }
}
impl ILARight for LAPersistedRight {}
impl From<LAPersistedRight> for LARight {
    fn from(child: LAPersistedRight) -> LARight {
        LARight(child.0)
    }
}
impl std::convert::TryFrom<LARight> for LAPersistedRight {
    type Error = &'static str;
    fn try_from(parent: LARight) -> Result<LAPersistedRight, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAPersistedRight").unwrap()) };
        if is_kind_of {
            Ok(LAPersistedRight(parent.0))
        } else {
            Err("This LARight cannot be downcasted to LAPersistedRight")
        }
    }
}
impl INSObject for LAPersistedRight {}
impl PNSObject for LAPersistedRight {}
impl ILAPersistedRight for LAPersistedRight {}
pub trait ILAPersistedRight: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn key(&self) -> LAPrivateKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, key)
    }
    unsafe fn secret(&self) -> LASecret
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secret)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAPersistedRight").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAPrivateKey(pub id);
impl std::ops::Deref for LAPrivateKey {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAPrivateKey {}
impl LAPrivateKey {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAPrivateKey").unwrap(), alloc) })
    }
}
impl INSObject for LAPrivateKey {}
impl PNSObject for LAPrivateKey {}
impl std::convert::TryFrom<NSObject> for LAPrivateKey {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LAPrivateKey, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAPrivateKey").unwrap()) };
        if is_kind_of {
            Ok(LAPrivateKey(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LAPrivateKey")
        }
    }
}
impl ILAPrivateKey for LAPrivateKey {}
pub trait ILAPrivateKey: Sized + std::ops::Deref {
    unsafe fn signData_secKeyAlgorithm_completion_(
        &self,
        data: NSData,
        algorithm: SecKeyAlgorithm,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signData : data, secKeyAlgorithm : algorithm, completion : handler)
    }
    unsafe fn canSignUsingSecKeyAlgorithm_(&self, algorithm: SecKeyAlgorithm) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canSignUsingSecKeyAlgorithm : algorithm)
    }
    unsafe fn decryptData_secKeyAlgorithm_completion_(
        &self,
        data: NSData,
        algorithm: SecKeyAlgorithm,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, decryptData : data, secKeyAlgorithm : algorithm, completion : handler)
    }
    unsafe fn canDecryptUsingSecKeyAlgorithm_(&self, algorithm: SecKeyAlgorithm) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canDecryptUsingSecKeyAlgorithm : algorithm)
    }
    unsafe fn exchangeKeysWithPublicKey_secKeyAlgorithm_secKeyParameters_completion_(
        &self,
        publicKey: NSData,
        algorithm: SecKeyAlgorithm,
        parameters: NSDictionary,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exchangeKeysWithPublicKey : publicKey, secKeyAlgorithm : algorithm, secKeyParameters : parameters, completion : handler)
    }
    unsafe fn canExchangeKeysUsingSecKeyAlgorithm_(&self, algorithm: SecKeyAlgorithm) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canExchangeKeysUsingSecKeyAlgorithm : algorithm)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn publicKey(&self) -> LAPublicKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicKey)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAPrivateKey").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAPublicKey(pub id);
impl std::ops::Deref for LAPublicKey {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAPublicKey {}
impl LAPublicKey {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAPublicKey").unwrap(), alloc) })
    }
}
impl INSObject for LAPublicKey {}
impl PNSObject for LAPublicKey {}
impl std::convert::TryFrom<NSObject> for LAPublicKey {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LAPublicKey, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAPublicKey").unwrap()) };
        if is_kind_of {
            Ok(LAPublicKey(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LAPublicKey")
        }
    }
}
impl ILAPublicKey for LAPublicKey {}
pub trait ILAPublicKey: Sized + std::ops::Deref {
    unsafe fn exportBytesWithCompletion_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exportBytesWithCompletion : handler)
    }
    unsafe fn encryptData_secKeyAlgorithm_completion_(
        &self,
        data: NSData,
        algorithm: SecKeyAlgorithm,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encryptData : data, secKeyAlgorithm : algorithm, completion : handler)
    }
    unsafe fn canEncryptUsingSecKeyAlgorithm_(&self, algorithm: SecKeyAlgorithm) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canEncryptUsingSecKeyAlgorithm : algorithm)
    }
    unsafe fn verifyData_signature_secKeyAlgorithm_completion_(
        &self,
        signedData: NSData,
        signature: NSData,
        algorithm: SecKeyAlgorithm,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, verifyData : signedData, signature : signature, secKeyAlgorithm : algorithm, completion : handler)
    }
    unsafe fn canVerifyUsingSecKeyAlgorithm_(&self, algorithm: SecKeyAlgorithm) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canVerifyUsingSecKeyAlgorithm : algorithm)
    }
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAPublicKey").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAAuthenticationRequirement(pub id);
impl std::ops::Deref for LAAuthenticationRequirement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAAuthenticationRequirement {}
impl LAAuthenticationRequirement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAAuthenticationRequirement").unwrap(), alloc) })
    }
}
impl INSObject for LAAuthenticationRequirement {}
impl PNSObject for LAAuthenticationRequirement {}
impl std::convert::TryFrom<NSObject> for LAAuthenticationRequirement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LAAuthenticationRequirement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAAuthenticationRequirement").unwrap()) };
        if is_kind_of {
            Ok(LAAuthenticationRequirement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LAAuthenticationRequirement")
        }
    }
}
impl ILAAuthenticationRequirement for LAAuthenticationRequirement {}
pub trait ILAAuthenticationRequirement: Sized + std::ops::Deref {
    unsafe fn biometryRequirementWithFallback_(
        fallback: LABiometryFallbackRequirement,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAAuthenticationRequirement").unwrap(), biometryRequirementWithFallback : fallback)
    }
    unsafe fn defaultRequirement() -> LAAuthenticationRequirement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAAuthenticationRequirement").unwrap(), defaultRequirement)
    }
    unsafe fn biometryRequirement() -> LAAuthenticationRequirement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAAuthenticationRequirement").unwrap(), biometryRequirement)
    }
    unsafe fn biometryCurrentSetRequirement() -> LAAuthenticationRequirement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LAAuthenticationRequirement").unwrap(), biometryCurrentSetRequirement)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LABiometryFallbackRequirement(pub id);
impl std::ops::Deref for LABiometryFallbackRequirement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LABiometryFallbackRequirement {}
impl LABiometryFallbackRequirement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LABiometryFallbackRequirement").unwrap(), alloc) })
    }
}
impl INSObject for LABiometryFallbackRequirement {}
impl PNSObject for LABiometryFallbackRequirement {}
impl std::convert::TryFrom<NSObject> for LABiometryFallbackRequirement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LABiometryFallbackRequirement, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LABiometryFallbackRequirement").unwrap())
        };
        if is_kind_of {
            Ok(LABiometryFallbackRequirement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LABiometryFallbackRequirement")
        }
    }
}
impl ILABiometryFallbackRequirement for LABiometryFallbackRequirement {}
pub trait ILABiometryFallbackRequirement: Sized + std::ops::Deref {
    unsafe fn defaultRequirement() -> LABiometryFallbackRequirement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LABiometryFallbackRequirement").unwrap(), defaultRequirement)
    }
    unsafe fn devicePasscodeRequirement() -> LABiometryFallbackRequirement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LABiometryFallbackRequirement").unwrap(), devicePasscodeRequirement)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LARightStore(pub id);
impl std::ops::Deref for LARightStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LARightStore {}
impl LARightStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LARightStore").unwrap(), alloc) })
    }
}
impl INSObject for LARightStore {}
impl PNSObject for LARightStore {}
impl std::convert::TryFrom<NSObject> for LARightStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LARightStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LARightStore").unwrap()) };
        if is_kind_of {
            Ok(LARightStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LARightStore")
        }
    }
}
impl ILARightStore for LARightStore {}
pub trait ILARightStore: Sized + std::ops::Deref {
    unsafe fn rightForIdentifier_completion_(
        &self,
        identifier: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rightForIdentifier : identifier, completion : handler)
    }
    unsafe fn saveRight_identifier_completion_(
        &self,
        right: LARight,
        identifier: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveRight : right, identifier : identifier, completion : handler)
    }
    unsafe fn saveRight_identifier_secret_completion_(
        &self,
        right: LARight,
        identifier: NSString,
        secret: NSData,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveRight : right, identifier : identifier, secret : secret, completion : handler)
    }
    unsafe fn removeRight_completion_(
        &self,
        right: LAPersistedRight,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRight : right, completion : handler)
    }
    unsafe fn removeRightForIdentifier_completion_(
        &self,
        identifier: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRightForIdentifier : identifier, completion : handler)
    }
    unsafe fn removeAllRightsWithCompletion_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAllRightsWithCompletion : handler)
    }
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"LARightStore").unwrap(), new)
    }
    unsafe fn sharedStore() -> LARightStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"LARightStore").unwrap(), sharedStore)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LASecret(pub id);
impl std::ops::Deref for LASecret {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LASecret {}
impl LASecret {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LASecret").unwrap(), alloc) })
    }
}
impl INSObject for LASecret {}
impl PNSObject for LASecret {}
impl std::convert::TryFrom<NSObject> for LASecret {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LASecret, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LASecret").unwrap()) };
        if is_kind_of {
            Ok(LASecret(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LASecret")
        }
    }
}
impl ILASecret for LASecret {}
pub trait ILASecret: Sized + std::ops::Deref {
    unsafe fn loadDataWithCompletion_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadDataWithCompletion : handler)
    }
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"LASecret").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static LATouchIDAuthenticationMaximumAllowableReuseDuration: NSTimeInterval;
}
unsafe extern "C" {
    pub static LAErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for LADomainStateBiometry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LADomainStateBiometry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LADomainStateCompanion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LADomainStateCompanion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LADomainState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LADomainState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAEnvironmentMechanism {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAEnvironmentMechanism {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAEnvironmentMechanismBiometry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAEnvironmentMechanismBiometry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAEnvironmentMechanismCompanion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAEnvironmentMechanismCompanion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAEnvironmentMechanismUserPassword {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAEnvironmentMechanismUserPassword {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAEnvironmentState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAEnvironmentState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAEnvironment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAEnvironment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LARight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LARight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAPersistedRight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAPersistedRight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAPrivateKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAPrivateKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAPublicKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAPublicKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LAAuthenticationRequirement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAAuthenticationRequirement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LABiometryFallbackRequirement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LABiometryFallbackRequirement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LARightStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LARightStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LASecret {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LASecret {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
