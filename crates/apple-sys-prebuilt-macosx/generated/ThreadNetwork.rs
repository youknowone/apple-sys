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
pub struct THCredentials(pub id);
impl std::ops::Deref for THCredentials {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for THCredentials {}
impl THCredentials {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"THCredentials").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for THCredentials {}
impl INSObject for THCredentials {}
impl PNSObject for THCredentials {}
impl std::convert::TryFrom<NSObject> for THCredentials {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<THCredentials, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"THCredentials").unwrap()) };
        if is_kind_of {
            Ok(THCredentials(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to THCredentials")
        }
    }
}
impl ITHCredentials for THCredentials {}
pub trait ITHCredentials: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn networkName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkName)
    }
    unsafe fn extendedPANID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extendedPANID)
    }
    unsafe fn borderAgentID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderAgentID)
    }
    unsafe fn activeOperationalDataSet(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeOperationalDataSet)
    }
    unsafe fn networkKey(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkKey)
    }
    unsafe fn PSKC(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PSKC)
    }
    unsafe fn channel(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channel)
    }
    unsafe fn setChannel_(&self, channel: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannel : channel)
    }
    unsafe fn panID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, panID)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn lastModificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastModificationDate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"THCredentials").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct THClient(pub id);
impl std::ops::Deref for THClient {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for THClient {}
impl THClient {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"THClient").unwrap(), alloc) })
    }
}
impl INSObject for THClient {}
impl PNSObject for THClient {}
impl std::convert::TryFrom<NSObject> for THClient {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<THClient, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"THClient").unwrap()) };
        if is_kind_of {
            Ok(THClient(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to THClient")
        }
    }
}
impl ITHClient for THClient {}
pub trait ITHClient: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn retrieveAllCredentials_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retrieveAllCredentials : completion)
    }
    unsafe fn retrieveAllActiveCredentials_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retrieveAllActiveCredentials : completion)
    }
    unsafe fn deleteCredentialsForBorderAgent_completion_(
        &self,
        borderAgentID: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteCredentialsForBorderAgent : borderAgentID, completion : completion)
    }
    unsafe fn retrieveCredentialsForBorderAgent_completion_(
        &self,
        borderAgentID: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retrieveCredentialsForBorderAgent : borderAgentID, completion : completion)
    }
    unsafe fn storeCredentialsForBorderAgent_activeOperationalDataSet_completion_(
        &self,
        borderAgentID: NSData,
        activeOperationalDataSet: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, storeCredentialsForBorderAgent : borderAgentID, activeOperationalDataSet : activeOperationalDataSet, completion : completion)
    }
    unsafe fn retrievePreferredCredentials_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retrievePreferredCredentials : completion)
    }
    unsafe fn retrieveCredentialsForExtendedPANID_completion_(
        &self,
        extendedPANID: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retrieveCredentialsForExtendedPANID : extendedPANID, completion : completion)
    }
    unsafe fn checkPreferredNetworkForActiveOperationalDataset_completion_(
        &self,
        activeOperationalDataSet: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkPreferredNetworkForActiveOperationalDataset : activeOperationalDataSet, completion : completion)
    }
    unsafe fn isPreferredNetworkAvailableWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isPreferredNetworkAvailableWithCompletion : completion)
    }
}

unsafe impl objc2::encode::RefEncode for THCredentials {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for THCredentials {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for THClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for THClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
