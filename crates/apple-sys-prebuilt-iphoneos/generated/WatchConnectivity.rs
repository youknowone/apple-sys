#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type WCErrorCode = NSInteger;
pub type WCSessionActivationState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WCSession(pub id);
impl std::ops::Deref for WCSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WCSession {}
impl WCSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WCSession").unwrap(), alloc) })
    }
}
impl INSObject for WCSession {}
impl PNSObject for WCSession {}
impl std::convert::TryFrom<NSObject> for WCSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WCSession, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WCSession").unwrap()) };
        if is_kind_of {
            Ok(WCSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WCSession")
        }
    }
}
impl IWCSession for WCSession {}
pub trait IWCSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn activateSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activateSession)
    }
    unsafe fn sendMessage_replyHandler_errorHandler_(
        &self,
        message: NSDictionary,
        replyHandler: *mut ::std::os::raw::c_void,
        errorHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMessage : message, replyHandler : replyHandler, errorHandler : errorHandler)
    }
    unsafe fn sendMessageData_replyHandler_errorHandler_(
        &self,
        data: NSData,
        replyHandler: *mut ::std::os::raw::c_void,
        errorHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMessageData : data, replyHandler : replyHandler, errorHandler : errorHandler)
    }
    unsafe fn updateApplicationContext_error_(
        &self,
        applicationContext: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateApplicationContext : applicationContext, error : error)
    }
    unsafe fn transferUserInfo_(&self, userInfo: NSDictionary) -> WCSessionUserInfoTransfer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transferUserInfo : userInfo)
    }
    unsafe fn transferCurrentComplicationUserInfo_(
        &self,
        userInfo: NSDictionary,
    ) -> WCSessionUserInfoTransfer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transferCurrentComplicationUserInfo : userInfo)
    }
    unsafe fn transferFile_metadata_(
        &self,
        file: NSURL,
        metadata: NSDictionary,
    ) -> WCSessionFileTransfer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transferFile : file, metadata : metadata)
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
    unsafe fn activationState(&self) -> WCSessionActivationState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activationState)
    }
    unsafe fn hasContentPending(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasContentPending)
    }
    unsafe fn isPaired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaired)
    }
    unsafe fn isWatchAppInstalled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isWatchAppInstalled)
    }
    unsafe fn isComplicationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isComplicationEnabled)
    }
    unsafe fn remainingComplicationUserInfoTransfers(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remainingComplicationUserInfoTransfers)
    }
    unsafe fn watchDirectoryURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, watchDirectoryURL)
    }
    unsafe fn isCompanionAppInstalled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompanionAppInstalled)
    }
    unsafe fn isReachable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReachable)
    }
    unsafe fn iOSDeviceNeedsUnlockAfterRebootForReachability(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iOSDeviceNeedsUnlockAfterRebootForReachability)
    }
    unsafe fn applicationContext(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationContext)
    }
    unsafe fn receivedApplicationContext(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, receivedApplicationContext)
    }
    unsafe fn outstandingUserInfoTransfers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outstandingUserInfoTransfers)
    }
    unsafe fn outstandingFileTransfers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outstandingFileTransfers)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WCSession").unwrap(), isSupported)
    }
    unsafe fn defaultSession() -> WCSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WCSession").unwrap(), defaultSession)
    }
}
pub trait PWCSessionDelegate: Sized + std::ops::Deref {
    unsafe fn session_activationDidCompleteWithState_error_(
        &self,
        session: WCSession,
        activationState: WCSessionActivationState,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, activationDidCompleteWithState : activationState, error : error)
    }
    unsafe fn sessionDidBecomeInactive_(&self, session: WCSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDidBecomeInactive : session)
    }
    unsafe fn sessionDidDeactivate_(&self, session: WCSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDidDeactivate : session)
    }
    unsafe fn sessionWatchStateDidChange_(&self, session: WCSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionWatchStateDidChange : session)
    }
    unsafe fn sessionCompanionAppInstalledDidChange_(&self, session: WCSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionCompanionAppInstalledDidChange : session)
    }
    unsafe fn sessionReachabilityDidChange_(&self, session: WCSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionReachabilityDidChange : session)
    }
    unsafe fn session_didReceiveMessage_(&self, session: WCSession, message: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveMessage : message)
    }
    unsafe fn session_didReceiveMessage_replyHandler_(
        &self,
        session: WCSession,
        message: NSDictionary,
        replyHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveMessage : message, replyHandler : replyHandler)
    }
    unsafe fn session_didReceiveMessageData_(&self, session: WCSession, messageData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveMessageData : messageData)
    }
    unsafe fn session_didReceiveMessageData_replyHandler_(
        &self,
        session: WCSession,
        messageData: NSData,
        replyHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveMessageData : messageData, replyHandler : replyHandler)
    }
    unsafe fn session_didReceiveApplicationContext_(
        &self,
        session: WCSession,
        applicationContext: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveApplicationContext : applicationContext)
    }
    unsafe fn session_didFinishUserInfoTransfer_error_(
        &self,
        session: WCSession,
        userInfoTransfer: WCSessionUserInfoTransfer,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didFinishUserInfoTransfer : userInfoTransfer, error : error)
    }
    unsafe fn session_didReceiveUserInfo_(&self, session: WCSession, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveUserInfo : userInfo)
    }
    unsafe fn session_didFinishFileTransfer_error_(
        &self,
        session: WCSession,
        fileTransfer: WCSessionFileTransfer,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didFinishFileTransfer : fileTransfer, error : error)
    }
    unsafe fn session_didReceiveFile_(&self, session: WCSession, file: WCSessionFile)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveFile : file)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WCSessionFile(pub id);
impl std::ops::Deref for WCSessionFile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WCSessionFile {}
impl WCSessionFile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WCSessionFile").unwrap(), alloc) })
    }
}
impl INSObject for WCSessionFile {}
impl PNSObject for WCSessionFile {}
impl std::convert::TryFrom<NSObject> for WCSessionFile {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WCSessionFile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WCSessionFile").unwrap()) };
        if is_kind_of {
            Ok(WCSessionFile(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WCSessionFile")
        }
    }
}
impl IWCSessionFile for WCSessionFile {}
pub trait IWCSessionFile: Sized + std::ops::Deref {
    unsafe fn fileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileURL)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WCSessionFileTransfer(pub id);
impl std::ops::Deref for WCSessionFileTransfer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WCSessionFileTransfer {}
impl WCSessionFileTransfer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WCSessionFileTransfer").unwrap(), alloc) })
    }
}
impl INSObject for WCSessionFileTransfer {}
impl PNSObject for WCSessionFileTransfer {}
impl std::convert::TryFrom<NSObject> for WCSessionFileTransfer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WCSessionFileTransfer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WCSessionFileTransfer").unwrap()) };
        if is_kind_of {
            Ok(WCSessionFileTransfer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WCSessionFileTransfer")
        }
    }
}
impl IWCSessionFileTransfer for WCSessionFileTransfer {}
pub trait IWCSessionFileTransfer: Sized + std::ops::Deref {
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn file(&self) -> WCSessionFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, file)
    }
    unsafe fn progress(&self) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progress)
    }
    unsafe fn isTransferring(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTransferring)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WCSessionUserInfoTransfer(pub id);
impl std::ops::Deref for WCSessionUserInfoTransfer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WCSessionUserInfoTransfer {}
impl WCSessionUserInfoTransfer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WCSessionUserInfoTransfer").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for WCSessionUserInfoTransfer {}
impl INSObject for WCSessionUserInfoTransfer {}
impl PNSObject for WCSessionUserInfoTransfer {}
impl std::convert::TryFrom<NSObject> for WCSessionUserInfoTransfer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WCSessionUserInfoTransfer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WCSessionUserInfoTransfer").unwrap()) };
        if is_kind_of {
            Ok(WCSessionUserInfoTransfer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WCSessionUserInfoTransfer")
        }
    }
}
impl IWCSessionUserInfoTransfer for WCSessionUserInfoTransfer {}
pub trait IWCSessionUserInfoTransfer: Sized + std::ops::Deref {
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isCurrentComplicationInfo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCurrentComplicationInfo)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn isTransferring(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTransferring)
    }
}
unsafe extern "C" {
    pub static WCErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for WCSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WCSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WCSessionFile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WCSessionFile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WCSessionFileTransfer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WCSessionFileTransfer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WCSessionUserInfoTransfer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WCSessionUserInfoTransfer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
