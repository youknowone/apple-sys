#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type NFCReaderError = NSInteger;
pub trait PNFCReaderSession: Sized + std::ops::Deref {
    unsafe fn beginSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginSession)
    }
    unsafe fn invalidateSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidateSession)
    }
    unsafe fn invalidateSessionWithErrorMessage_(&self, errorMessage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateSessionWithErrorMessage : errorMessage)
    }
    unsafe fn isReady(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReady)
    }
    unsafe fn alertMessage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertMessage)
    }
    unsafe fn setAlertMessage_(&self, alertMessage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlertMessage : alertMessage)
    }
}
pub trait PNFCReaderSessionDelegate: Sized + std::ops::Deref {
    unsafe fn readerSessionDidBecomeActive_(&self, session: NFCReaderSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSessionDidBecomeActive : session)
    }
    unsafe fn readerSession_didInvalidateWithError_(
        &self,
        session: NFCReaderSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSession : session, didInvalidateWithError : error)
    }
    unsafe fn readerSession_didDetectTags_(&self, session: NFCReaderSession, tags: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSession : session, didDetectTags : tags)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCReaderSession(pub id);
impl std::ops::Deref for NFCReaderSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCReaderSession {}
impl NFCReaderSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCReaderSession").unwrap(), alloc) })
    }
}
impl PNFCReaderSession for NFCReaderSession {}
impl INSObject for NFCReaderSession {}
impl PNSObject for NFCReaderSession {}
impl std::convert::TryFrom<NSObject> for NFCReaderSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NFCReaderSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCReaderSession").unwrap()) };
        if is_kind_of {
            Ok(NFCReaderSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NFCReaderSession")
        }
    }
}
impl INFCReaderSession for NFCReaderSession {}
pub trait INFCReaderSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn sessionQueue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionQueue)
    }
    unsafe fn readingAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NFCReaderSession").unwrap(), readingAvailable)
    }
}
pub type NFCTagType = NSUInteger;
pub trait PNFCTag: Sized + std::ops::Deref {
    unsafe fn asNFCISO15693Tag(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asNFCISO15693Tag)
    }
    unsafe fn asNFCISO7816Tag(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asNFCISO7816Tag)
    }
    unsafe fn asNFCFeliCaTag(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asNFCFeliCaTag)
    }
    unsafe fn asNFCMiFareTag(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asNFCMiFareTag)
    }
    unsafe fn type_(&self) -> NFCTagType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn session(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, session)
    }
    unsafe fn isAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAvailable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCTagCommandConfiguration(pub id);
impl std::ops::Deref for NFCTagCommandConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCTagCommandConfiguration {}
impl NFCTagCommandConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCTagCommandConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for NFCTagCommandConfiguration {}
impl INSObject for NFCTagCommandConfiguration {}
impl PNSObject for NFCTagCommandConfiguration {}
impl std::convert::TryFrom<NSObject> for NFCTagCommandConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NFCTagCommandConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCTagCommandConfiguration").unwrap()) };
        if is_kind_of {
            Ok(NFCTagCommandConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NFCTagCommandConfiguration")
        }
    }
}
impl INFCTagCommandConfiguration for NFCTagCommandConfiguration {}
pub trait INFCTagCommandConfiguration: Sized + std::ops::Deref {
    unsafe fn maximumRetries(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumRetries)
    }
    unsafe fn setMaximumRetries_(&self, maximumRetries: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumRetries : maximumRetries)
    }
    unsafe fn retryInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retryInterval)
    }
    unsafe fn setRetryInterval_(&self, retryInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRetryInterval : retryInterval)
    }
}
pub trait PNFCTagReaderSessionDelegate: Sized + std::ops::Deref {
    unsafe fn tagReaderSession_didInvalidateWithError_(
        &self,
        session: NFCTagReaderSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tagReaderSession : session, didInvalidateWithError : error)
    }
    unsafe fn tagReaderSessionDidBecomeActive_(&self, session: NFCTagReaderSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tagReaderSessionDidBecomeActive : session)
    }
    unsafe fn tagReaderSession_didDetectTags_(&self, session: NFCTagReaderSession, tags: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tagReaderSession : session, didDetectTags : tags)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCTagReaderSession(pub id);
impl std::ops::Deref for NFCTagReaderSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCTagReaderSession {}
impl NFCTagReaderSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCTagReaderSession").unwrap(), alloc) })
    }
}
impl INFCReaderSession for NFCTagReaderSession {}
impl PNFCReaderSession for NFCTagReaderSession {}
impl From<NFCTagReaderSession> for NFCReaderSession {
    fn from(child: NFCTagReaderSession) -> NFCReaderSession {
        NFCReaderSession(child.0)
    }
}
impl std::convert::TryFrom<NFCReaderSession> for NFCTagReaderSession {
    type Error = &'static str;
    fn try_from(parent: NFCReaderSession) -> Result<NFCTagReaderSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCTagReaderSession").unwrap()) };
        if is_kind_of {
            Ok(NFCTagReaderSession(parent.0))
        } else {
            Err("This NFCReaderSession cannot be downcasted to NFCTagReaderSession")
        }
    }
}
impl INSObject for NFCTagReaderSession {}
impl PNSObject for NFCTagReaderSession {}
impl INFCTagReaderSession for NFCTagReaderSession {}
pub trait INFCTagReaderSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPollingOption_delegate_queue_(
        &self,
        pollingOption: NFCPollingOption,
        delegate: *mut u64,
        queue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPollingOption : pollingOption, delegate : delegate, queue : queue)
    }
    unsafe fn restartPolling(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restartPolling)
    }
    unsafe fn connectToTag_completionHandler_(
        &self,
        tag: *mut u64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectToTag : tag, completionHandler : completionHandler)
    }
    unsafe fn connectedTag(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedTag)
    }
}
pub type NFCPollingOption = NSInteger;
pub trait PNFCNDEFReaderSessionDelegate: Sized + std::ops::Deref {
    unsafe fn readerSession_didInvalidateWithError_(
        &self,
        session: NFCNDEFReaderSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSession : session, didInvalidateWithError : error)
    }
    unsafe fn readerSession_didDetectNDEFs_(&self, session: NFCNDEFReaderSession, messages: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSession : session, didDetectNDEFs : messages)
    }
    unsafe fn readerSession_didDetectTags_(&self, session: NFCNDEFReaderSession, tags: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSession : session, didDetectTags : tags)
    }
    unsafe fn readerSessionDidBecomeActive_(&self, session: NFCNDEFReaderSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSessionDidBecomeActive : session)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCNDEFReaderSession(pub id);
impl std::ops::Deref for NFCNDEFReaderSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCNDEFReaderSession {}
impl NFCNDEFReaderSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCNDEFReaderSession").unwrap(), alloc) })
    }
}
impl INFCReaderSession for NFCNDEFReaderSession {}
impl PNFCReaderSession for NFCNDEFReaderSession {}
impl std::convert::TryFrom<NFCReaderSession> for NFCNDEFReaderSession {
    type Error = &'static str;
    fn try_from(parent: NFCReaderSession) -> Result<NFCNDEFReaderSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCNDEFReaderSession").unwrap()) };
        if is_kind_of {
            Ok(NFCNDEFReaderSession(parent.0))
        } else {
            Err("This NFCReaderSession cannot be downcasted to NFCNDEFReaderSession")
        }
    }
}
impl INSObject for NFCNDEFReaderSession {}
impl PNSObject for NFCNDEFReaderSession {}
impl INFCNDEFReaderSession for NFCNDEFReaderSession {}
pub trait INFCNDEFReaderSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDelegate_queue_invalidateAfterFirstRead_(
        &self,
        delegate: *mut u64,
        queue: NSObject,
        invalidateAfterFirstRead: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate, queue : queue, invalidateAfterFirstRead : invalidateAfterFirstRead)
    }
    unsafe fn restartPolling(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restartPolling)
    }
    unsafe fn connectToTag_completionHandler_(
        &self,
        tag: *mut u64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectToTag : tag, completionHandler : completionHandler)
    }
}
pub trait NSUserActivity_CoreNFC: Sized + std::ops::Deref {
    unsafe fn ndefMessagePayload(&self) -> NFCNDEFMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ndefMessagePayload)
    }
}
pub type NFCISO15693RequestFlag = u8;
pub use self::NFCISO15693RequestFlag as RequestFlag;
pub type NFCISO15693ResponseFlag = u8;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCISO15693CustomCommandConfiguration(pub id);
impl std::ops::Deref for NFCISO15693CustomCommandConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCISO15693CustomCommandConfiguration {}
impl NFCISO15693CustomCommandConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCISO15693CustomCommandConfiguration").unwrap(), alloc) })
    }
}
impl INFCTagCommandConfiguration for NFCISO15693CustomCommandConfiguration {}
impl PNSCopying for NFCISO15693CustomCommandConfiguration {}
impl From<NFCISO15693CustomCommandConfiguration> for NFCTagCommandConfiguration {
    fn from(child: NFCISO15693CustomCommandConfiguration) -> NFCTagCommandConfiguration {
        NFCTagCommandConfiguration(child.0)
    }
}
impl std::convert::TryFrom<NFCTagCommandConfiguration> for NFCISO15693CustomCommandConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: NFCTagCommandConfiguration,
    ) -> Result<NFCISO15693CustomCommandConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCISO15693CustomCommandConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(NFCISO15693CustomCommandConfiguration(parent.0))
        } else {
            Err ("This NFCTagCommandConfiguration cannot be downcasted to NFCISO15693CustomCommandConfiguration" ,)
        }
    }
}
impl INSObject for NFCISO15693CustomCommandConfiguration {}
impl PNSObject for NFCISO15693CustomCommandConfiguration {}
impl INFCISO15693CustomCommandConfiguration for NFCISO15693CustomCommandConfiguration {}
pub trait INFCISO15693CustomCommandConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithManufacturerCode_customCommandCode_requestParameters_(
        &self,
        manufacturerCode: NSUInteger,
        customCommandCode: NSUInteger,
        requestParameters: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithManufacturerCode : manufacturerCode, customCommandCode : customCommandCode, requestParameters : requestParameters)
    }
    unsafe fn initWithManufacturerCode_customCommandCode_requestParameters_maximumRetries_retryInterval_(
        &self,
        manufacturerCode: NSUInteger,
        customCommandCode: NSUInteger,
        requestParameters: NSData,
        maximumRetries: NSUInteger,
        retryInterval: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithManufacturerCode : manufacturerCode, customCommandCode : customCommandCode, requestParameters : requestParameters, maximumRetries : maximumRetries, retryInterval : retryInterval)
    }
    unsafe fn manufacturerCode(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturerCode)
    }
    unsafe fn setManufacturerCode_(&self, manufacturerCode: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManufacturerCode : manufacturerCode)
    }
    unsafe fn customCommandCode(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customCommandCode)
    }
    unsafe fn setCustomCommandCode_(&self, customCommandCode: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomCommandCode : customCommandCode)
    }
    unsafe fn requestParameters(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestParameters)
    }
    unsafe fn setRequestParameters_(&self, requestParameters: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequestParameters : requestParameters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCISO15693ReadMultipleBlocksConfiguration(pub id);
impl std::ops::Deref for NFCISO15693ReadMultipleBlocksConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCISO15693ReadMultipleBlocksConfiguration {}
impl NFCISO15693ReadMultipleBlocksConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCISO15693ReadMultipleBlocksConfiguration").unwrap(), alloc) })
    }
}
impl INFCTagCommandConfiguration for NFCISO15693ReadMultipleBlocksConfiguration {}
impl PNSCopying for NFCISO15693ReadMultipleBlocksConfiguration {}
impl std::convert::TryFrom<NFCTagCommandConfiguration>
    for NFCISO15693ReadMultipleBlocksConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: NFCTagCommandConfiguration,
    ) -> Result<NFCISO15693ReadMultipleBlocksConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCISO15693ReadMultipleBlocksConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(NFCISO15693ReadMultipleBlocksConfiguration(parent.0))
        } else {
            Err ("This NFCTagCommandConfiguration cannot be downcasted to NFCISO15693ReadMultipleBlocksConfiguration" ,)
        }
    }
}
impl INSObject for NFCISO15693ReadMultipleBlocksConfiguration {}
impl PNSObject for NFCISO15693ReadMultipleBlocksConfiguration {}
impl INFCISO15693ReadMultipleBlocksConfiguration for NFCISO15693ReadMultipleBlocksConfiguration {}
pub trait INFCISO15693ReadMultipleBlocksConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithRange_chunkSize_(&self, range: NSRange, chunkSize: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRange : range, chunkSize : chunkSize)
    }
    unsafe fn initWithRange_chunkSize_maximumRetries_retryInterval_(
        &self,
        range: NSRange,
        chunkSize: NSUInteger,
        maximumRetries: NSUInteger,
        retryInterval: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRange : range, chunkSize : chunkSize, maximumRetries : maximumRetries, retryInterval : retryInterval)
    }
    unsafe fn range(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, range)
    }
    unsafe fn setRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRange : range)
    }
    unsafe fn chunkSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chunkSize)
    }
    unsafe fn setChunkSize_(&self, chunkSize: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChunkSize : chunkSize)
    }
}
pub trait PNFCISO15693Tag: Sized + std::ops::Deref {
    unsafe fn sendCustomCommandWithConfiguration_completionHandler_(
        &self,
        commandConfiguration: NFCISO15693CustomCommandConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendCustomCommandWithConfiguration : commandConfiguration, completionHandler : completionHandler)
    }
    unsafe fn readMultipleBlocksWithConfiguration_completionHandler_(
        &self,
        readConfiguration: NFCISO15693ReadMultipleBlocksConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readMultipleBlocksWithConfiguration : readConfiguration, completionHandler : completionHandler)
    }
    unsafe fn stayQuietWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stayQuietWithCompletionHandler : completionHandler)
    }
    unsafe fn readSingleBlockWithRequestFlags_blockNumber_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockNumber: u8,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readSingleBlockWithRequestFlags : flags, blockNumber : blockNumber, completionHandler : completionHandler)
    }
    unsafe fn writeSingleBlockWithRequestFlags_blockNumber_dataBlock_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockNumber: u8,
        dataBlock: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeSingleBlockWithRequestFlags : flags, blockNumber : blockNumber, dataBlock : dataBlock, completionHandler : completionHandler)
    }
    unsafe fn lockBlockWithRequestFlags_blockNumber_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockNumber: u8,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockBlockWithRequestFlags : flags, blockNumber : blockNumber, completionHandler : completionHandler)
    }
    unsafe fn readMultipleBlocksWithRequestFlags_blockRange_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockRange: NSRange,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readMultipleBlocksWithRequestFlags : flags, blockRange : blockRange, completionHandler : completionHandler)
    }
    unsafe fn writeMultipleBlocksWithRequestFlags_blockRange_dataBlocks_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockRange: NSRange,
        dataBlocks: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeMultipleBlocksWithRequestFlags : flags, blockRange : blockRange, dataBlocks : dataBlocks, completionHandler : completionHandler)
    }
    unsafe fn selectWithRequestFlags_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectWithRequestFlags : flags, completionHandler : completionHandler)
    }
    unsafe fn resetToReadyWithRequestFlags_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetToReadyWithRequestFlags : flags, completionHandler : completionHandler)
    }
    unsafe fn writeAFIWithRequestFlag_afi_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        afi: u8,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeAFIWithRequestFlag : flags, afi : afi, completionHandler : completionHandler)
    }
    unsafe fn lockAFIWithRequestFlag_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockAFIWithRequestFlag : flags, completionHandler : completionHandler)
    }
    unsafe fn writeDSFIDWithRequestFlag_dsfid_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        dsfid: u8,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeDSFIDWithRequestFlag : flags, dsfid : dsfid, completionHandler : completionHandler)
    }
    unsafe fn lockDFSIDWithRequestFlag_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockDFSIDWithRequestFlag : flags, completionHandler : completionHandler)
    }
    unsafe fn lockDSFIDWithRequestFlag_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockDSFIDWithRequestFlag : flags, completionHandler : completionHandler)
    }
    unsafe fn getSystemInfoWithRequestFlag_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSystemInfoWithRequestFlag : flags, completionHandler : completionHandler)
    }
    unsafe fn getSystemInfoAndUIDWithRequestFlag_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSystemInfoAndUIDWithRequestFlag : flags, completionHandler : completionHandler)
    }
    unsafe fn getMultipleBlockSecurityStatusWithRequestFlag_blockRange_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockRange: NSRange,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getMultipleBlockSecurityStatusWithRequestFlag : flags, blockRange : blockRange, completionHandler : completionHandler)
    }
    unsafe fn fastReadMultipleBlocksWithRequestFlag_blockRange_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockRange: NSRange,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fastReadMultipleBlocksWithRequestFlag : flags, blockRange : blockRange, completionHandler : completionHandler)
    }
    unsafe fn customCommandWithRequestFlag_customCommandCode_customRequestParameters_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        customCommandCode: NSInteger,
        customRequestParameters: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, customCommandWithRequestFlag : flags, customCommandCode : customCommandCode, customRequestParameters : customRequestParameters, completionHandler : completionHandler)
    }
    unsafe fn extendedReadSingleBlockWithRequestFlags_blockNumber_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockNumber: NSInteger,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendedReadSingleBlockWithRequestFlags : flags, blockNumber : blockNumber, completionHandler : completionHandler)
    }
    unsafe fn extendedWriteSingleBlockWithRequestFlags_blockNumber_dataBlock_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockNumber: NSInteger,
        dataBlock: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendedWriteSingleBlockWithRequestFlags : flags, blockNumber : blockNumber, dataBlock : dataBlock, completionHandler : completionHandler)
    }
    unsafe fn extendedLockBlockWithRequestFlags_blockNumber_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockNumber: NSInteger,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendedLockBlockWithRequestFlags : flags, blockNumber : blockNumber, completionHandler : completionHandler)
    }
    unsafe fn extendedReadMultipleBlocksWithRequestFlags_blockRange_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockRange: NSRange,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendedReadMultipleBlocksWithRequestFlags : flags, blockRange : blockRange, completionHandler : completionHandler)
    }
    unsafe fn extendedWriteMultipleBlocksWithRequestFlags_blockRange_dataBlocks_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockRange: NSRange,
        dataBlocks: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendedWriteMultipleBlocksWithRequestFlags : flags, blockRange : blockRange, dataBlocks : dataBlocks, completionHandler : completionHandler)
    }
    unsafe fn authenticateWithRequestFlags_cryptoSuiteIdentifier_message_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        cryptoSuiteIdentifier: NSInteger,
        message: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authenticateWithRequestFlags : flags, cryptoSuiteIdentifier : cryptoSuiteIdentifier, message : message, completionHandler : completionHandler)
    }
    unsafe fn keyUpdateWithRequestFlags_keyIdentifier_message_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        keyIdentifier: NSInteger,
        message: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, keyUpdateWithRequestFlags : flags, keyIdentifier : keyIdentifier, message : message, completionHandler : completionHandler)
    }
    unsafe fn challengeWithRequestFlags_cryptoSuiteIdentifier_message_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        cryptoSuiteIdentifier: NSInteger,
        message: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeWithRequestFlags : flags, cryptoSuiteIdentifier : cryptoSuiteIdentifier, message : message, completionHandler : completionHandler)
    }
    unsafe fn readBufferWithRequestFlags_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readBufferWithRequestFlags : flags, completionHandler : completionHandler)
    }
    unsafe fn extendedGetMultipleBlockSecurityStatusWithRequestFlag_blockRange_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockRange: NSRange,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendedGetMultipleBlockSecurityStatusWithRequestFlag : flags, blockRange : blockRange, completionHandler : completionHandler)
    }
    unsafe fn extendedFastReadMultipleBlocksWithRequestFlag_blockRange_completionHandler_(
        &self,
        flags: NFCISO15693RequestFlag,
        blockRange: NSRange,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendedFastReadMultipleBlocksWithRequestFlag : flags, blockRange : blockRange, completionHandler : completionHandler)
    }
    unsafe fn sendRequestWithFlag_commandCode_data_completionHandler_(
        &self,
        flags: NSInteger,
        commandCode: NSInteger,
        data: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendRequestWithFlag : flags, commandCode : commandCode, data : data, completionHandler : completionHandler)
    }
    unsafe fn identifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn icManufacturerCode(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, icManufacturerCode)
    }
    unsafe fn icSerialNumber(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, icSerialNumber)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCISO15693ReaderSession(pub id);
impl std::ops::Deref for NFCISO15693ReaderSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCISO15693ReaderSession {}
impl NFCISO15693ReaderSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCISO15693ReaderSession").unwrap(), alloc) })
    }
}
impl INFCReaderSession for NFCISO15693ReaderSession {}
impl PNFCReaderSession for NFCISO15693ReaderSession {}
impl std::convert::TryFrom<NFCReaderSession> for NFCISO15693ReaderSession {
    type Error = &'static str;
    fn try_from(parent: NFCReaderSession) -> Result<NFCISO15693ReaderSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCISO15693ReaderSession").unwrap()) };
        if is_kind_of {
            Ok(NFCISO15693ReaderSession(parent.0))
        } else {
            Err("This NFCReaderSession cannot be downcasted to NFCISO15693ReaderSession")
        }
    }
}
impl INSObject for NFCISO15693ReaderSession {}
impl PNSObject for NFCISO15693ReaderSession {}
impl INFCISO15693ReaderSession for NFCISO15693ReaderSession {}
pub trait INFCISO15693ReaderSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDelegate_queue_(&self, delegate: *mut u64, queue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate, queue : queue)
    }
    unsafe fn restartPolling(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restartPolling)
    }
}
pub type NFCNDEFStatus = NSUInteger;
pub trait PNFCNDEFTag: Sized + std::ops::Deref {
    unsafe fn queryNDEFStatusWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryNDEFStatusWithCompletionHandler : completionHandler)
    }
    unsafe fn readNDEFWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readNDEFWithCompletionHandler : completionHandler)
    }
    unsafe fn writeNDEF_completionHandler_(
        &self,
        ndefMessage: NFCNDEFMessage,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeNDEF : ndefMessage, completionHandler : completionHandler)
    }
    unsafe fn writeLockWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeLockWithCompletionHandler : completionHandler)
    }
    unsafe fn isAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAvailable)
    }
}
pub type NFCFeliCaPollingRequestCode = NSInteger;
pub use self::NFCFeliCaPollingRequestCode as PollingRequestCode;
pub type NFCFeliCaPollingTimeSlot = NSInteger;
pub use self::NFCFeliCaPollingTimeSlot as PollingTimeSlot;
pub type NFCFeliCaEncryptionId = NSInteger;
pub use self::NFCFeliCaEncryptionId as EncryptionId;
pub trait PNFCFeliCaTag: Sized + std::ops::Deref {
    unsafe fn pollingWithSystemCode_requestCode_timeSlot_completionHandler_(
        &self,
        systemCode: NSData,
        requestCode: NFCFeliCaPollingRequestCode,
        timeSlot: NFCFeliCaPollingTimeSlot,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pollingWithSystemCode : systemCode, requestCode : requestCode, timeSlot : timeSlot, completionHandler : completionHandler)
    }
    unsafe fn requestServiceWithNodeCodeList_completionHandler_(
        &self,
        nodeCodeList: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestServiceWithNodeCodeList : nodeCodeList, completionHandler : completionHandler)
    }
    unsafe fn requestResponseWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestResponseWithCompletionHandler : completionHandler)
    }
    unsafe fn readWithoutEncryptionWithServiceCodeList_blockList_completionHandler_(
        &self,
        serviceCodeList: NSArray,
        blockList: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readWithoutEncryptionWithServiceCodeList : serviceCodeList, blockList : blockList, completionHandler : completionHandler)
    }
    unsafe fn writeWithoutEncryptionWithServiceCodeList_blockList_blockData_completionHandler_(
        &self,
        serviceCodeList: NSArray,
        blockList: NSArray,
        blockData: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeWithoutEncryptionWithServiceCodeList : serviceCodeList, blockList : blockList, blockData : blockData, completionHandler : completionHandler)
    }
    unsafe fn requestSystemCodeWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestSystemCodeWithCompletionHandler : completionHandler)
    }
    unsafe fn requestServiceV2WithNodeCodeList_completionHandler_(
        &self,
        nodeCodeList: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestServiceV2WithNodeCodeList : nodeCodeList, completionHandler : completionHandler)
    }
    unsafe fn requestSpecificationVersionWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestSpecificationVersionWithCompletionHandler : completionHandler)
    }
    unsafe fn resetModeWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetModeWithCompletionHandler : completionHandler)
    }
    unsafe fn sendFeliCaCommandPacket_completionHandler_(
        &self,
        commandPacket: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendFeliCaCommandPacket : commandPacket, completionHandler : completionHandler)
    }
    unsafe fn currentSystemCode(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentSystemCode)
    }
    unsafe fn currentIDm(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentIDm)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCISO7816APDU(pub id);
impl std::ops::Deref for NFCISO7816APDU {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCISO7816APDU {}
impl NFCISO7816APDU {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCISO7816APDU").unwrap(), alloc) })
    }
}
impl PNSCopying for NFCISO7816APDU {}
impl INSObject for NFCISO7816APDU {}
impl PNSObject for NFCISO7816APDU {}
impl std::convert::TryFrom<NSObject> for NFCISO7816APDU {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NFCISO7816APDU, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCISO7816APDU").unwrap()) };
        if is_kind_of {
            Ok(NFCISO7816APDU(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NFCISO7816APDU")
        }
    }
}
impl INFCISO7816APDU for NFCISO7816APDU {}
pub trait INFCISO7816APDU: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInstructionClass_instructionCode_p1Parameter_p2Parameter_data_expectedResponseLength_(
        &self,
        instructionClass: u8,
        instructionCode: u8,
        p1Parameter: u8,
        p2Parameter: u8,
        data: NSData,
        expectedResponseLength: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInstructionClass : instructionClass, instructionCode : instructionCode, p1Parameter : p1Parameter, p2Parameter : p2Parameter, data : data, expectedResponseLength : expectedResponseLength)
    }
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn instructionClass(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instructionClass)
    }
    unsafe fn instructionCode(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instructionCode)
    }
    unsafe fn p1Parameter(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, p1Parameter)
    }
    unsafe fn p2Parameter(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, p2Parameter)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn expectedResponseLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedResponseLength)
    }
}
pub trait PNFCISO7816Tag: Sized + std::ops::Deref {
    unsafe fn sendCommandAPDU_completionHandler_(
        &self,
        apdu: NFCISO7816APDU,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendCommandAPDU : apdu, completionHandler : completionHandler)
    }
    unsafe fn initialSelectedAID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialSelectedAID)
    }
    unsafe fn identifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn historicalBytes(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, historicalBytes)
    }
    unsafe fn applicationData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationData)
    }
    unsafe fn proprietaryApplicationDataCoding(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proprietaryApplicationDataCoding)
    }
}
pub type NFCMiFareFamily = NSUInteger;
pub trait PNFCMiFareTag: Sized + std::ops::Deref {
    unsafe fn sendMiFareCommand_completionHandler_(
        &self,
        command: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMiFareCommand : command, completionHandler : completionHandler)
    }
    unsafe fn sendMiFareISO7816Command_completionHandler_(
        &self,
        apdu: NFCISO7816APDU,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMiFareISO7816Command : apdu, completionHandler : completionHandler)
    }
    unsafe fn mifareFamily(&self) -> NFCMiFareFamily
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mifareFamily)
    }
    unsafe fn identifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn historicalBytes(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, historicalBytes)
    }
}
pub type NFCTypeNameFormat = u8;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCNDEFPayload(pub id);
impl std::ops::Deref for NFCNDEFPayload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCNDEFPayload {}
impl NFCNDEFPayload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCNDEFPayload").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NFCNDEFPayload {}
impl INSObject for NFCNDEFPayload {}
impl PNSObject for NFCNDEFPayload {}
impl std::convert::TryFrom<NSObject> for NFCNDEFPayload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NFCNDEFPayload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCNDEFPayload").unwrap()) };
        if is_kind_of {
            Ok(NFCNDEFPayload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NFCNDEFPayload")
        }
    }
}
impl INFCNDEFPayload for NFCNDEFPayload {}
pub trait INFCNDEFPayload: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFormat_type_identifier_payload_(
        &self,
        format: NFCTypeNameFormat,
        type_: NSData,
        identifier: NSData,
        payload: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormat : format, r#type : type_, identifier : identifier, payload : payload)
    }
    unsafe fn initWithFormat_type_identifier_payload_chunkSize_(
        &self,
        format: NFCTypeNameFormat,
        type_: NSData,
        identifier: NSData,
        payload: NSData,
        chunkSize: usize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormat : format, r#type : type_, identifier : identifier, payload : payload, chunkSize : chunkSize)
    }
    unsafe fn typeNameFormat(&self) -> NFCTypeNameFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typeNameFormat)
    }
    unsafe fn setTypeNameFormat_(&self, typeNameFormat: NFCTypeNameFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTypeNameFormat : typeNameFormat)
    }
    unsafe fn type_(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn identifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
    unsafe fn payload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, payload)
    }
    unsafe fn setPayload_(&self, payload: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPayload : payload)
    }
}
impl NFCNDEFPayload_ConvenienceHelpers for NFCNDEFPayload {}
pub trait NFCNDEFPayload_ConvenienceHelpers: Sized + std::ops::Deref {
    unsafe fn wellKnownTypeURIPayload(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wellKnownTypeURIPayload)
    }
    unsafe fn wellKnownTypeTextPayloadWithLocale_(&self, locale: *mut NSLocale) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, wellKnownTypeTextPayloadWithLocale : locale)
    }
    unsafe fn wellKnownTypeURIPayloadWithString_(uri: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NFCNDEFPayload").unwrap(), wellKnownTypeURIPayloadWithString : uri)
    }
    unsafe fn wellKnownTypeURIPayloadWithURL_(url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NFCNDEFPayload").unwrap(), wellKnownTypeURIPayloadWithURL : url)
    }
    unsafe fn wellKnownTypeTextPayloadWithString_locale_(
        text: NSString,
        locale: NSLocale,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NFCNDEFPayload").unwrap(), wellKnownTypeTextPayloadWithString : text, locale : locale)
    }
    unsafe fn wellKnowTypeTextPayloadWithString_locale_(
        text: NSString,
        locale: NSLocale,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NFCNDEFPayload").unwrap(), wellKnowTypeTextPayloadWithString : text, locale : locale)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCNDEFMessage(pub id);
impl std::ops::Deref for NFCNDEFMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCNDEFMessage {}
impl NFCNDEFMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCNDEFMessage").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NFCNDEFMessage {}
impl INSObject for NFCNDEFMessage {}
impl PNSObject for NFCNDEFMessage {}
impl std::convert::TryFrom<NSObject> for NFCNDEFMessage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NFCNDEFMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCNDEFMessage").unwrap()) };
        if is_kind_of {
            Ok(NFCNDEFMessage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NFCNDEFMessage")
        }
    }
}
impl INFCNDEFMessage for NFCNDEFMessage {}
pub trait INFCNDEFMessage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithNDEFRecords_(&self, records: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNDEFRecords : records)
    }
    unsafe fn records(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, records)
    }
    unsafe fn setRecords_(&self, records: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecords : records)
    }
    unsafe fn length(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn ndefMessageWithData_(data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NFCNDEFMessage").unwrap(), ndefMessageWithData : data)
    }
}
pub type NFCVASMode = NSInteger;
pub use self::NFCVASMode as VASMode;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCVASCommandConfiguration(pub id);
impl std::ops::Deref for NFCVASCommandConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCVASCommandConfiguration {}
impl NFCVASCommandConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCVASCommandConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for NFCVASCommandConfiguration {}
impl INSObject for NFCVASCommandConfiguration {}
impl PNSObject for NFCVASCommandConfiguration {}
impl std::convert::TryFrom<NSObject> for NFCVASCommandConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NFCVASCommandConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCVASCommandConfiguration").unwrap()) };
        if is_kind_of {
            Ok(NFCVASCommandConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NFCVASCommandConfiguration")
        }
    }
}
impl INFCVASCommandConfiguration for NFCVASCommandConfiguration {}
pub trait INFCVASCommandConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithVASMode_passTypeIdentifier_url_(
        &self,
        mode: NFCVASMode,
        passTypeIdentifier: NSString,
        url: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVASMode : mode, passTypeIdentifier : passTypeIdentifier, url : url)
    }
    unsafe fn mode(&self) -> NFCVASMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn setMode_(&self, mode: NFCVASMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode)
    }
    unsafe fn passTypeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passTypeIdentifier)
    }
    unsafe fn setPassTypeIdentifier_(&self, passTypeIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPassTypeIdentifier : passTypeIdentifier)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn setUrl_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUrl : url)
    }
}
pub type NFCVASErrorCode = NSInteger;
pub use self::NFCVASErrorCode as VASErrorCode;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCVASResponse(pub id);
impl std::ops::Deref for NFCVASResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCVASResponse {}
impl NFCVASResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCVASResponse").unwrap(), alloc) })
    }
}
impl PNSCopying for NFCVASResponse {}
impl INSObject for NFCVASResponse {}
impl PNSObject for NFCVASResponse {}
impl std::convert::TryFrom<NSObject> for NFCVASResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NFCVASResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCVASResponse").unwrap()) };
        if is_kind_of {
            Ok(NFCVASResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NFCVASResponse")
        }
    }
}
impl INFCVASResponse for NFCVASResponse {}
pub trait INFCVASResponse: Sized + std::ops::Deref {
    unsafe fn status(&self) -> NFCVASErrorCode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn vasData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vasData)
    }
    unsafe fn mobileToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mobileToken)
    }
}
pub trait PNFCVASReaderSessionDelegate: Sized + std::ops::Deref {
    unsafe fn readerSessionDidBecomeActive_(&self, session: NFCVASReaderSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSessionDidBecomeActive : session)
    }
    unsafe fn readerSession_didInvalidateWithError_(
        &self,
        session: NFCVASReaderSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSession : session, didInvalidateWithError : error)
    }
    unsafe fn readerSession_didReceiveVASResponses_(
        &self,
        session: NFCVASReaderSession,
        responses: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readerSession : session, didReceiveVASResponses : responses)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCVASReaderSession(pub id);
impl std::ops::Deref for NFCVASReaderSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCVASReaderSession {}
impl NFCVASReaderSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCVASReaderSession").unwrap(), alloc) })
    }
}
impl INFCReaderSession for NFCVASReaderSession {}
impl PNFCReaderSession for NFCVASReaderSession {}
impl std::convert::TryFrom<NFCReaderSession> for NFCVASReaderSession {
    type Error = &'static str;
    fn try_from(parent: NFCReaderSession) -> Result<NFCVASReaderSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCVASReaderSession").unwrap()) };
        if is_kind_of {
            Ok(NFCVASReaderSession(parent.0))
        } else {
            Err("This NFCReaderSession cannot be downcasted to NFCVASReaderSession")
        }
    }
}
impl INSObject for NFCVASReaderSession {}
impl PNSObject for NFCVASReaderSession {}
impl INFCVASReaderSession for NFCVASReaderSession {}
pub trait INFCVASReaderSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithVASCommandConfigurations_delegate_queue_(
        &self,
        commandConfigurations: NSArray,
        delegate: *mut u64,
        queue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVASCommandConfigurations : commandConfigurations, delegate : delegate, queue : queue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NFCPaymentTagReaderSession(pub id);
impl std::ops::Deref for NFCPaymentTagReaderSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NFCPaymentTagReaderSession {}
impl NFCPaymentTagReaderSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NFCPaymentTagReaderSession").unwrap(), alloc) })
    }
}
impl INFCTagReaderSession for NFCPaymentTagReaderSession {}
impl From<NFCPaymentTagReaderSession> for NFCTagReaderSession {
    fn from(child: NFCPaymentTagReaderSession) -> NFCTagReaderSession {
        NFCTagReaderSession(child.0)
    }
}
impl std::convert::TryFrom<NFCTagReaderSession> for NFCPaymentTagReaderSession {
    type Error = &'static str;
    fn try_from(parent: NFCTagReaderSession) -> Result<NFCPaymentTagReaderSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NFCPaymentTagReaderSession").unwrap()) };
        if is_kind_of {
            Ok(NFCPaymentTagReaderSession(parent.0))
        } else {
            Err("This NFCTagReaderSession cannot be downcasted to NFCPaymentTagReaderSession")
        }
    }
}
impl INFCReaderSession for NFCPaymentTagReaderSession {}
impl PNFCReaderSession for NFCPaymentTagReaderSession {}
impl INSObject for NFCPaymentTagReaderSession {}
impl PNSObject for NFCPaymentTagReaderSession {}
impl INFCPaymentTagReaderSession for NFCPaymentTagReaderSession {}
pub trait INFCPaymentTagReaderSession: Sized + std::ops::Deref {
    unsafe fn initWithDelegate_queue_(&self, delegate: *mut u64, queue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate, queue : queue)
    }
}
unsafe extern "C" {
    pub static NFCErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static NFCISO15693TagResponseErrorKey: NSString;
}
unsafe extern "C" {
    pub static NFCTagResponseUnexpectedLengthErrorKey: NSString;
}

unsafe impl objc2::encode::RefEncode for NFCReaderSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCReaderSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCTagCommandConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCTagCommandConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCTagReaderSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCTagReaderSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCNDEFReaderSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCNDEFReaderSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCISO15693CustomCommandConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCISO15693CustomCommandConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCISO15693ReadMultipleBlocksConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCISO15693ReadMultipleBlocksConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCISO15693ReaderSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCISO15693ReaderSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCISO7816APDU {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCISO7816APDU {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCNDEFPayload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCNDEFPayload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCNDEFMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCNDEFMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCVASCommandConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCVASCommandConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCVASResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCVASResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCVASReaderSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCVASReaderSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NFCPaymentTagReaderSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NFCPaymentTagReaderSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
