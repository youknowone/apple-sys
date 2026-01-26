#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use libc::{id_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SBObject(pub id);
impl std::ops::Deref for SBObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SBObject {}
impl SBObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SBObject").unwrap(), alloc) })
    }
}
impl PNSCoding for SBObject {}
impl INSObject for SBObject {}
impl PNSObject for SBObject {}
impl std::convert::TryFrom<NSObject> for SBObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SBObject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SBObject").unwrap()) };
        if is_kind_of {
            Ok(SBObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SBObject")
        }
    }
}
impl ISBObject for SBObject {}
pub trait ISBObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithProperties_(&self, properties: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProperties : properties)
    }
    unsafe fn initWithData_(&self, data: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn get(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, get)
    }
    unsafe fn lastError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastError)
    }
}
impl SBObject_SBGlueInterface for SBObject {}
pub trait SBObject_SBGlueInterface: Sized + std::ops::Deref {
    unsafe fn initWithElementCode_properties_data_(
        &self,
        code: DescType,
        properties: NSDictionary,
        data: id,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElementCode : code, properties : properties, data : data)
    }
    unsafe fn propertyWithCode_(&self, code: AEKeyword) -> SBObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertyWithCode : code)
    }
    unsafe fn propertyWithClass_code_(&self, cls: Class, code: AEKeyword) -> SBObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertyWithClass : cls, code : code)
    }
    unsafe fn elementArrayWithCode_(&self, code: DescType) -> SBElementArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementArrayWithCode : code)
    }
    unsafe fn sendEvent_id_parameters_(
        &self,
        eventClass: AEEventClass,
        eventID: AEEventID,
        firstParamCode: DescType,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendEvent : eventClass, id : eventID, parameters : firstParamCode)
    }
    unsafe fn setTo_(&self, value: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTo : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SBApplication(pub id);
impl std::ops::Deref for SBApplication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SBApplication {}
impl SBApplication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SBApplication").unwrap(), alloc) })
    }
}
impl PNSCoding for SBApplication {}
impl ISBObject for SBApplication {}
impl From<SBApplication> for SBObject {
    fn from(child: SBApplication) -> SBObject {
        SBObject(child.0)
    }
}
impl std::convert::TryFrom<SBObject> for SBApplication {
    type Error = &'static str;
    fn try_from(parent: SBObject) -> Result<SBApplication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SBApplication").unwrap()) };
        if is_kind_of {
            Ok(SBApplication(parent.0))
        } else {
            Err("This SBObject cannot be downcasted to SBApplication")
        }
    }
}
impl INSObject for SBApplication {}
impl PNSObject for SBApplication {}
impl ISBApplication for SBApplication {}
pub trait ISBApplication: Sized + std::ops::Deref {
    unsafe fn initWithBundleIdentifier_(&self, ident: NSString) -> SBApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBundleIdentifier : ident)
    }
    unsafe fn initWithURL_(&self, url: NSURL) -> SBApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn initWithProcessIdentifier_(&self, pid: pid_t) -> SBApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProcessIdentifier : pid)
    }
    unsafe fn classForScriptingClass_(&self, className: NSString) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classForScriptingClass : className)
    }
    unsafe fn activate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activate)
    }
    unsafe fn isRunning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRunning)
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
    unsafe fn launchFlags(&self) -> LSLaunchFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, launchFlags)
    }
    unsafe fn setLaunchFlags_(&self, launchFlags: LSLaunchFlags)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLaunchFlags : launchFlags)
    }
    unsafe fn sendMode(&self) -> AESendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sendMode)
    }
    unsafe fn setSendMode_(&self, sendMode: AESendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSendMode : sendMode)
    }
    unsafe fn timeout(&self) -> ::std::os::raw::c_long
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeout)
    }
    unsafe fn setTimeout_(&self, timeout: ::std::os::raw::c_long)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeout : timeout)
    }
    unsafe fn applicationWithBundleIdentifier_(ident: NSString) -> SBApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SBApplication").unwrap(), applicationWithBundleIdentifier : ident)
    }
    unsafe fn applicationWithURL_(url: NSURL) -> SBApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SBApplication").unwrap(), applicationWithURL : url)
    }
    unsafe fn applicationWithProcessIdentifier_(pid: pid_t) -> SBApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SBApplication").unwrap(), applicationWithProcessIdentifier : pid)
    }
}
pub trait PSBApplicationDelegate: Sized + std::ops::Deref {
    unsafe fn eventDidFail_withError_(&self, event: *const AppleEvent, error: NSError) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eventDidFail : event, withError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SBElementArray(pub id);
impl std::ops::Deref for SBElementArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SBElementArray {}
impl SBElementArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SBElementArray").unwrap(), alloc) })
    }
}
impl<ObjectType: 'static> INSMutableArray<ObjectType> for SBElementArray {}
impl<ObjectType: 'static> INSArray<ObjectType> for SBElementArray {}
impl PNSCopying for SBElementArray {}
impl PNSMutableCopying for SBElementArray {}
impl PNSSecureCoding for SBElementArray {}
impl PNSFastEnumeration for SBElementArray {}
impl INSObject for SBElementArray {}
impl PNSObject for SBElementArray {}
impl std::convert::TryFrom<NSObject> for SBElementArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SBElementArray, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SBElementArray").unwrap()) };
        if is_kind_of {
            Ok(SBElementArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SBElementArray")
        }
    }
}
impl<ObjectType: 'static> ISBElementArray<ObjectType> for SBElementArray {}
pub trait ISBElementArray<ObjectType: 'static>: Sized + std::ops::Deref {
    unsafe fn objectWithName_(&self, name: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectWithName : name)
    }
    unsafe fn objectWithID_(&self, identifier: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectWithID : identifier)
    }
    unsafe fn objectAtLocation_(&self, location: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtLocation : location)
    }
    unsafe fn arrayByApplyingSelector_(&self, selector: objc2::runtime::Sel) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, arrayByApplyingSelector : selector)
    }
    unsafe fn arrayByApplyingSelector_withObject_(
        &self,
        aSelector: objc2::runtime::Sel,
        argument: id,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, arrayByApplyingSelector : aSelector, withObject : argument)
    }
    unsafe fn get(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, get)
    }
}

unsafe impl objc2::encode::RefEncode for SBObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SBObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SBApplication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SBApplication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SBElementArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SBElementArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
