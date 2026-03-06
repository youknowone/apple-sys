#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type PKPushType = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPushCredentials(pub id);
impl std::ops::Deref for PKPushCredentials {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPushCredentials {}
impl PKPushCredentials {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPushCredentials").unwrap(), alloc) })
    }
}
impl INSObject for PKPushCredentials {}
impl PNSObject for PKPushCredentials {}
impl std::convert::TryFrom<NSObject> for PKPushCredentials {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPushCredentials, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPushCredentials").unwrap()) };
        if is_kind_of {
            Ok(PKPushCredentials(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPushCredentials")
        }
    }
}
impl IPKPushCredentials for PKPushCredentials {}
pub trait IPKPushCredentials: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> PKPushType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn token(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, token)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPushPayload(pub id);
impl std::ops::Deref for PKPushPayload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPushPayload {}
impl PKPushPayload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPushPayload").unwrap(), alloc) })
    }
}
impl INSObject for PKPushPayload {}
impl PNSObject for PKPushPayload {}
impl std::convert::TryFrom<NSObject> for PKPushPayload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPushPayload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPushPayload").unwrap()) };
        if is_kind_of {
            Ok(PKPushPayload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPushPayload")
        }
    }
}
impl IPKPushPayload for PKPushPayload {}
pub trait IPKPushPayload: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> PKPushType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn dictionaryPayload(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryPayload)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPushRegistry(pub id);
impl std::ops::Deref for PKPushRegistry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPushRegistry {}
impl PKPushRegistry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPushRegistry").unwrap(), alloc) })
    }
}
impl INSObject for PKPushRegistry {}
impl PNSObject for PKPushRegistry {}
impl std::convert::TryFrom<NSObject> for PKPushRegistry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPushRegistry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPushRegistry").unwrap()) };
        if is_kind_of {
            Ok(PKPushRegistry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPushRegistry")
        }
    }
}
impl IPKPushRegistry for PKPushRegistry {}
pub trait IPKPushRegistry: Sized + std::ops::Deref {
    unsafe fn pushTokenForType_(&self, type_: NSString) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushTokenForType : type_)
    }
    unsafe fn initWithQueue_(&self, queue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQueue : queue)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn desiredPushTypes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, desiredPushTypes)
    }
    unsafe fn setDesiredPushTypes_(&self, desiredPushTypes: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredPushTypes : desiredPushTypes)
    }
}
pub trait PPKPushRegistryDelegate: Sized + std::ops::Deref {
    unsafe fn pushRegistry_didUpdatePushCredentials_forType_(
        &self,
        registry: PKPushRegistry,
        pushCredentials: PKPushCredentials,
        type_: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushRegistry : registry, didUpdatePushCredentials : pushCredentials, forType : type_)
    }
    unsafe fn pushRegistry_didReceiveIncomingPushWithPayload_forType_(
        &self,
        registry: PKPushRegistry,
        payload: PKPushPayload,
        type_: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushRegistry : registry, didReceiveIncomingPushWithPayload : payload, forType : type_)
    }
    unsafe fn pushRegistry_didReceiveIncomingPushWithPayload_forType_withCompletionHandler_(
        &self,
        registry: PKPushRegistry,
        payload: PKPushPayload,
        type_: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushRegistry : registry, didReceiveIncomingPushWithPayload : payload, forType : type_, withCompletionHandler : completion)
    }
    unsafe fn pushRegistry_didInvalidatePushTokenForType_(
        &self,
        registry: PKPushRegistry,
        type_: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushRegistry : registry, didInvalidatePushTokenForType : type_)
    }
}
unsafe extern "C" {
    pub static PKPushTypeVoIP: PKPushType;
}
unsafe extern "C" {
    pub static PKPushTypeComplication: PKPushType;
}
unsafe extern "C" {
    pub static PKPushTypeFileProvider: PKPushType;
}

unsafe impl objc2::encode::RefEncode for PKPushCredentials {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPushCredentials {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPushPayload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPushPayload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPushRegistry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPushRegistry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
