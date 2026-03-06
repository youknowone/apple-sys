#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFAuthorization(pub id);
impl std::ops::Deref for SFAuthorization {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFAuthorization {}
impl SFAuthorization {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFAuthorization").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SFAuthorization {}
impl INSObject for SFAuthorization {}
impl PNSObject for SFAuthorization {}
impl std::convert::TryFrom<NSObject> for SFAuthorization {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFAuthorization, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFAuthorization").unwrap()) };
        if is_kind_of {
            Ok(SFAuthorization(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFAuthorization")
        }
    }
}
impl ISFAuthorization for SFAuthorization {}
pub trait ISFAuthorization: Sized + std::ops::Deref {
    unsafe fn authorizationRef(&self) -> AuthorizationRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRef)
    }
    unsafe fn initWithFlags_rights_environment_(
        &self,
        flags: AuthorizationFlags,
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFlags : flags, rights : rights, environment : environment)
    }
    unsafe fn init(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn invalidateCredentials(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidateCredentials)
    }
    unsafe fn obtainWithRight_flags_error_(
        &self,
        rightName: AuthorizationString,
        flags: AuthorizationFlags,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, obtainWithRight : rightName, flags : flags, error : error)
    }
    unsafe fn obtainWithRights_flags_environment_authorizedRights_error_(
        &self,
        rights: *const AuthorizationRights,
        flags: AuthorizationFlags,
        environment: *const AuthorizationEnvironment,
        authorizedRights: *mut *mut AuthorizationRights,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, obtainWithRights : rights, flags : flags, environment : environment, authorizedRights : authorizedRights, error : error)
    }
    unsafe fn authorization() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFAuthorization").unwrap(), authorization)
    }
    unsafe fn authorizationWithFlags_rights_environment_(
        flags: AuthorizationFlags,
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFAuthorization").unwrap(), authorizationWithFlags : flags, rights : rights, environment : environment)
    }
}
impl SFAuthorization_SFAuthorizationDeprecated for SFAuthorization {}
pub trait SFAuthorization_SFAuthorizationDeprecated: Sized + std::ops::Deref {
    unsafe fn permitWithRights_flags_environment_authorizedRights_(
        &self,
        rights: *const AuthorizationRights,
        flags: AuthorizationFlags,
        environment: *const AuthorizationEnvironment,
        authorizedRights: *mut AuthorizationRights,
    ) -> OSStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, permitWithRights : rights, flags : flags, environment : environment, authorizedRights : authorizedRights)
    }
    unsafe fn permitWithRight_flags_(
        &self,
        rightName: AuthorizationString,
        flags: AuthorizationFlags,
    ) -> OSStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, permitWithRight : rightName, flags : flags)
    }
}

unsafe impl objc2::encode::RefEncode for SFAuthorization {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFAuthorization {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
