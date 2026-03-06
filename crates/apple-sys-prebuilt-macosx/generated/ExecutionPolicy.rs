#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type EPDeveloperToolStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EPDeveloperTool(pub id);
impl std::ops::Deref for EPDeveloperTool {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EPDeveloperTool {}
impl EPDeveloperTool {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EPDeveloperTool").unwrap(), alloc) })
    }
}
impl INSObject for EPDeveloperTool {}
impl PNSObject for EPDeveloperTool {}
impl std::convert::TryFrom<NSObject> for EPDeveloperTool {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EPDeveloperTool, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EPDeveloperTool").unwrap()) };
        if is_kind_of {
            Ok(EPDeveloperTool(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EPDeveloperTool")
        }
    }
}
impl IEPDeveloperTool for EPDeveloperTool {}
pub trait IEPDeveloperTool: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn requestDeveloperToolAccessWithCompletionHandler_(
        &self,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDeveloperToolAccessWithCompletionHandler : handler)
    }
    unsafe fn authorizationStatus(&self) -> EPDeveloperToolStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationStatus)
    }
}
pub type EPError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EPExecutionPolicy(pub id);
impl std::ops::Deref for EPExecutionPolicy {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EPExecutionPolicy {}
impl EPExecutionPolicy {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EPExecutionPolicy").unwrap(), alloc) })
    }
}
impl INSObject for EPExecutionPolicy {}
impl PNSObject for EPExecutionPolicy {}
impl std::convert::TryFrom<NSObject> for EPExecutionPolicy {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EPExecutionPolicy, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EPExecutionPolicy").unwrap()) };
        if is_kind_of {
            Ok(EPExecutionPolicy(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EPExecutionPolicy")
        }
    }
}
impl IEPExecutionPolicy for EPExecutionPolicy {}
pub trait IEPExecutionPolicy: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addPolicyExceptionForURL_error_(&self, url: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPolicyExceptionForURL : url, error : error)
    }
}
unsafe extern "C" {
    pub static EPErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for EPDeveloperTool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EPDeveloperTool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EPExecutionPolicy {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EPExecutionPolicy {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
