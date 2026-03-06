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
pub struct NSExceptionHandler(pub id);
impl std::ops::Deref for NSExceptionHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSExceptionHandler {}
impl NSExceptionHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSExceptionHandler").unwrap(), alloc) })
    }
}
impl INSObject for NSExceptionHandler {}
impl PNSObject for NSExceptionHandler {}
impl std::convert::TryFrom<NSObject> for NSExceptionHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSExceptionHandler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSExceptionHandler").unwrap()) };
        if is_kind_of {
            Ok(NSExceptionHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSExceptionHandler")
        }
    }
}
impl INSExceptionHandler for NSExceptionHandler {}
pub trait INSExceptionHandler: Sized + std::ops::Deref {
    unsafe fn setExceptionHandlingMask_(&self, aMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExceptionHandlingMask : aMask)
    }
    unsafe fn exceptionHandlingMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionHandlingMask)
    }
    unsafe fn setExceptionHangingMask_(&self, aMask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExceptionHangingMask : aMask)
    }
    unsafe fn exceptionHangingMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionHangingMask)
    }
    unsafe fn setDelegate_(&self, anObject: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : anObject)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn defaultExceptionHandler() -> NSExceptionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSExceptionHandler").unwrap(), defaultExceptionHandler)
    }
}
pub trait NSObject_NSExceptionHandlerDelegate: Sized + std::ops::Deref {
    unsafe fn exceptionHandler_shouldLogException_mask_(
        &self,
        sender: NSExceptionHandler,
        exception: NSException,
        aMask: NSUInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exceptionHandler : sender, shouldLogException : exception, mask : aMask)
    }
    unsafe fn exceptionHandler_shouldHandleException_mask_(
        &self,
        sender: NSExceptionHandler,
        exception: NSException,
        aMask: NSUInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exceptionHandler : sender, shouldHandleException : exception, mask : aMask)
    }
}
unsafe extern "C" {
    pub static mut NSUncaughtSystemExceptionException: NSString;
}
unsafe extern "C" {
    pub static mut NSUncaughtRuntimeErrorException: NSString;
}
unsafe extern "C" {
    pub static mut NSStackTraceKey: NSString;
}
unsafe extern "C" {
    pub fn NSExceptionHandlerResume();
}

unsafe impl objc2::encode::RefEncode for NSExceptionHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSExceptionHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
