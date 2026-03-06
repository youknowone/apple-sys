#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type BAContentRequest = NSInteger;
pub type BADownloadState = NSInteger;
pub type BADownloaderPriority = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BADownload(pub id);
impl std::ops::Deref for BADownload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BADownload {}
impl BADownload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BADownload").unwrap(), alloc) })
    }
}
impl PNSCoding for BADownload {}
impl PNSSecureCoding for BADownload {}
impl PNSCopying for BADownload {}
impl INSObject for BADownload {}
impl PNSObject for BADownload {}
impl std::convert::TryFrom<NSObject> for BADownload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BADownload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BADownload").unwrap()) };
        if is_kind_of {
            Ok(BADownload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BADownload")
        }
    }
}
impl IBADownload for BADownload {}
pub trait IBADownload: Sized + std::ops::Deref {
    unsafe fn copyAsNonEssential(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, copyAsNonEssential)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn state(&self) -> BADownloadState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn uniqueIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn priority(&self) -> BADownloaderPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, priority)
    }
    unsafe fn isEssential(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEssential)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BADownload").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BAAppExtensionInfo(pub id);
impl std::ops::Deref for BAAppExtensionInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BAAppExtensionInfo {}
impl BAAppExtensionInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BAAppExtensionInfo").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for BAAppExtensionInfo {}
impl INSObject for BAAppExtensionInfo {}
impl PNSObject for BAAppExtensionInfo {}
impl std::convert::TryFrom<NSObject> for BAAppExtensionInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BAAppExtensionInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BAAppExtensionInfo").unwrap()) };
        if is_kind_of {
            Ok(BAAppExtensionInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BAAppExtensionInfo")
        }
    }
}
impl IBAAppExtensionInfo for BAAppExtensionInfo {}
pub trait IBAAppExtensionInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn restrictedDownloadSizeRemaining(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restrictedDownloadSizeRemaining)
    }
    unsafe fn restrictedEssentialDownloadSizeRemaining(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restrictedEssentialDownloadSizeRemaining)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BAAppExtensionInfo").unwrap(), new)
    }
}
pub trait PBADownloaderExtension: Sized + std::ops::Deref {
    unsafe fn downloadsForRequest_manifestURL_extensionInfo_(
        &self,
        contentRequest: BAContentRequest,
        manifestURL: NSURL,
        extensionInfo: BAAppExtensionInfo,
    ) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadsForRequest : contentRequest, manifestURL : manifestURL, extensionInfo : extensionInfo)
    }
    unsafe fn backgroundDownload_didReceiveChallenge_completionHandler_(
        &self,
        download: BADownload,
        challenge: NSURLAuthenticationChallenge,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, backgroundDownload : download, didReceiveChallenge : challenge, completionHandler : completionHandler)
    }
    unsafe fn backgroundDownload_failedWithError_(&self, download: BADownload, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, backgroundDownload : download, failedWithError : error)
    }
    unsafe fn backgroundDownload_finishedWithFileURL_(&self, download: BADownload, fileURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, backgroundDownload : download, finishedWithFileURL : fileURL)
    }
    unsafe fn extensionWillTerminate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionWillTerminate)
    }
}
pub trait PBADownloadManagerDelegate: Sized + std::ops::Deref {
    unsafe fn downloadDidBegin_(&self, download: BADownload)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadDidBegin : download)
    }
    unsafe fn downloadDidPause_(&self, download: BADownload)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadDidPause : download)
    }
    unsafe fn download_didWriteBytes_totalBytesWritten_totalBytesExpectedToWrite_(
        &self,
        download: BADownload,
        bytesWritten: i64,
        totalBytesWritten: i64,
        totalExpectedBytes: i64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, didWriteBytes : bytesWritten, totalBytesWritten : totalBytesWritten, totalBytesExpectedToWrite : totalExpectedBytes)
    }
    unsafe fn download_didReceiveChallenge_completionHandler_(
        &self,
        download: BADownload,
        challenge: NSURLAuthenticationChallenge,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, didReceiveChallenge : challenge, completionHandler : completionHandler)
    }
    unsafe fn download_failedWithError_(&self, download: BADownload, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, failedWithError : error)
    }
    unsafe fn download_finishedWithFileURL_(&self, download: BADownload, fileURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, finishedWithFileURL : fileURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BADownloadManager(pub id);
impl std::ops::Deref for BADownloadManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BADownloadManager {}
impl BADownloadManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BADownloadManager").unwrap(), alloc) })
    }
}
impl INSObject for BADownloadManager {}
impl PNSObject for BADownloadManager {}
impl std::convert::TryFrom<NSObject> for BADownloadManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BADownloadManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BADownloadManager").unwrap()) };
        if is_kind_of {
            Ok(BADownloadManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BADownloadManager")
        }
    }
}
impl IBADownloadManager for BADownloadManager {}
pub trait IBADownloadManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fetchCurrentDownloads_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchCurrentDownloads : error)
    }
    unsafe fn fetchCurrentDownloadsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchCurrentDownloadsWithCompletionHandler : completionHandler)
    }
    unsafe fn scheduleDownload_error_(&self, download: BADownload, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleDownload : download, error : error)
    }
    unsafe fn performWithExclusiveControl_(&self, performHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performWithExclusiveControl : performHandler)
    }
    unsafe fn performWithExclusiveControlBeforeDate_performHandler_(
        &self,
        date: NSDate,
        performHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performWithExclusiveControlBeforeDate : date, performHandler : performHandler)
    }
    unsafe fn startForegroundDownload_error_(
        &self,
        download: BADownload,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startForegroundDownload : download, error : error)
    }
    unsafe fn cancelDownload_error_(&self, download: BADownload, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelDownload : download, error : error)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BADownloadManager").unwrap(), new)
    }
    unsafe fn sharedManager() -> BADownloadManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BADownloadManager").unwrap(), sharedManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BAURLDownload(pub id);
impl std::ops::Deref for BAURLDownload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BAURLDownload {}
impl BAURLDownload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BAURLDownload").unwrap(), alloc) })
    }
}
impl PNSCopying for BAURLDownload {}
impl IBADownload for BAURLDownload {}
impl PNSCoding for BAURLDownload {}
impl PNSSecureCoding for BAURLDownload {}
impl From<BAURLDownload> for BADownload {
    fn from(child: BAURLDownload) -> BADownload {
        BADownload(child.0)
    }
}
impl std::convert::TryFrom<BADownload> for BAURLDownload {
    type Error = &'static str;
    fn try_from(parent: BADownload) -> Result<BAURLDownload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BAURLDownload").unwrap()) };
        if is_kind_of {
            Ok(BAURLDownload(parent.0))
        } else {
            Err("This BADownload cannot be downcasted to BAURLDownload")
        }
    }
}
impl INSObject for BAURLDownload {}
impl PNSObject for BAURLDownload {}
impl IBAURLDownload for BAURLDownload {}
pub trait IBAURLDownload: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithIdentifier_request_fileSize_applicationGroupIdentifier_(
        &self,
        identifier: NSString,
        request: NSURLRequest,
        fileSize: NSUInteger,
        applicationGroupIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, request : request, fileSize : fileSize, applicationGroupIdentifier : applicationGroupIdentifier)
    }
    unsafe fn initWithIdentifier_request_essential_fileSize_applicationGroupIdentifier_priority_(
        &self,
        identifier: NSString,
        request: NSURLRequest,
        essential: BOOL,
        fileSize: NSUInteger,
        applicationGroupIdentifier: NSString,
        priority: BADownloaderPriority,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, request : request, essential : essential, fileSize : fileSize, applicationGroupIdentifier : applicationGroupIdentifier, priority : priority)
    }
    unsafe fn initWithIdentifier_request_applicationGroupIdentifier_(
        &self,
        identifier: NSString,
        request: NSURLRequest,
        applicationGroupIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, request : request, applicationGroupIdentifier : applicationGroupIdentifier)
    }
    unsafe fn initWithIdentifier_request_applicationGroupIdentifier_priority_(
        &self,
        identifier: NSString,
        request: NSURLRequest,
        applicationGroupIdentifier: NSString,
        priority: BADownloaderPriority,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, request : request, applicationGroupIdentifier : applicationGroupIdentifier, priority : priority)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BAURLDownload").unwrap(), new)
    }
}
pub type BAErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BAAssetPack(pub id);
impl std::ops::Deref for BAAssetPack {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BAAssetPack {}
impl BAAssetPack {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BAAssetPack").unwrap(), alloc) })
    }
}
impl INSObject for BAAssetPack {}
impl PNSObject for BAAssetPack {}
impl std::convert::TryFrom<NSObject> for BAAssetPack {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BAAssetPack, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BAAssetPack").unwrap()) };
        if is_kind_of {
            Ok(BAAssetPack(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BAAssetPack")
        }
    }
}
impl IBAAssetPack for BAAssetPack {}
pub trait IBAAssetPack: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn download(&self) -> BADownload
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, download)
    }
    unsafe fn downloadForContentRequest_(&self, contentRequest: BAContentRequest) -> BADownload
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadForContentRequest : contentRequest)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn downloadSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadSize)
    }
    unsafe fn version(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn userInfo(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BAAssetPack").unwrap(), new)
    }
}
pub type BAAssetPackStatus = NSUInteger;
pub trait PBAManagedAssetPackDownloadDelegate: Sized + std::ops::Deref {
    unsafe fn downloadOfAssetPackBegan_(&self, assetPack: BAAssetPack)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadOfAssetPackBegan : assetPack)
    }
    unsafe fn downloadOfAssetPackPaused_(&self, assetPack: BAAssetPack)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadOfAssetPackPaused : assetPack)
    }
    unsafe fn downloadOfAssetPack_hasProgress_(&self, assetPack: BAAssetPack, progress: NSProgress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadOfAssetPack : assetPack, hasProgress : progress)
    }
    unsafe fn downloadOfAssetPackFinished_(&self, assetPack: BAAssetPack)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadOfAssetPackFinished : assetPack)
    }
    unsafe fn downloadOfAssetPack_failedWithError_(&self, assetPack: BAAssetPack, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadOfAssetPack : assetPack, failedWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BAAssetPackManager(pub id);
impl std::ops::Deref for BAAssetPackManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BAAssetPackManager {}
impl BAAssetPackManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BAAssetPackManager").unwrap(), alloc) })
    }
}
impl INSObject for BAAssetPackManager {}
impl PNSObject for BAAssetPackManager {}
impl std::convert::TryFrom<NSObject> for BAAssetPackManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BAAssetPackManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BAAssetPackManager").unwrap()) };
        if is_kind_of {
            Ok(BAAssetPackManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BAAssetPackManager")
        }
    }
}
impl IBAAssetPackManager for BAAssetPackManager {}
pub trait IBAAssetPackManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn getAllAssetPacksWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAllAssetPacksWithCompletionHandler : completionHandler)
    }
    unsafe fn getAssetPackWithIdentifier_completionHandler_(
        &self,
        assetPackIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAssetPackWithIdentifier : assetPackIdentifier, completionHandler : completionHandler)
    }
    unsafe fn getStatusOfAssetPackWithIdentifier_completionHandler_(
        &self,
        assetPackIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getStatusOfAssetPackWithIdentifier : assetPackIdentifier, completionHandler : completionHandler)
    }
    unsafe fn ensureLocalAvailabilityOfAssetPack_completionHandler_(
        &self,
        assetPack: BAAssetPack,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ensureLocalAvailabilityOfAssetPack : assetPack, completionHandler : completionHandler)
    }
    unsafe fn checkForUpdatesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkForUpdatesWithCompletionHandler : completionHandler)
    }
    unsafe fn contentsAtPath_searchingInAssetPackWithIdentifier_options_error_(
        &self,
        path: NSString,
        assetPackIdentifier: NSString,
        options: NSDataReadingOptions,
        error: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentsAtPath : path, searchingInAssetPackWithIdentifier : assetPackIdentifier, options : options, error : error)
    }
    unsafe fn fileDescriptorForPath_searchingInAssetPackWithIdentifier_error_(
        &self,
        path: NSString,
        assetPackIdentifier: NSString,
        error: *mut NSError,
    ) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileDescriptorForPath : path, searchingInAssetPackWithIdentifier : assetPackIdentifier, error : error)
    }
    unsafe fn URLForPath_error_(&self, path: NSString, error: *mut NSError) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLForPath : path, error : error)
    }
    unsafe fn removeAssetPackWithIdentifier_completionHandler_(
        &self,
        assetPackIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAssetPackWithIdentifier : assetPackIdentifier, completionHandler : completionHandler)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BAAssetPackManager").unwrap(), new)
    }
    unsafe fn sharedManager() -> BAAssetPackManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BAAssetPackManager").unwrap(), sharedManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BAAssetPackManifest(pub id);
impl std::ops::Deref for BAAssetPackManifest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BAAssetPackManifest {}
impl BAAssetPackManifest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BAAssetPackManifest").unwrap(), alloc) })
    }
}
impl INSObject for BAAssetPackManifest {}
impl PNSObject for BAAssetPackManifest {}
impl std::convert::TryFrom<NSObject> for BAAssetPackManifest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BAAssetPackManifest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BAAssetPackManifest").unwrap()) };
        if is_kind_of {
            Ok(BAAssetPackManifest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BAAssetPackManifest")
        }
    }
}
impl IBAAssetPackManifest for BAAssetPackManifest {}
pub trait IBAAssetPackManifest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithContentsOfURL_applicationGroupIdentifier_error_(
        &self,
        URL: NSURL,
        applicationGroupIdentifier: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : URL, applicationGroupIdentifier : applicationGroupIdentifier, error : error)
    }
    unsafe fn initFromData_applicationGroupIdentifier_error_(
        &self,
        data: NSData,
        applicationGroupIdentifier: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initFromData : data, applicationGroupIdentifier : applicationGroupIdentifier, error : error)
    }
    unsafe fn allDownloads(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allDownloads)
    }
    unsafe fn allDownloadsForContentRequest_(&self, contentRequest: BAContentRequest) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, allDownloadsForContentRequest : contentRequest)
    }
    unsafe fn assetPacks(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetPacks)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BAAssetPackManifest").unwrap(), new)
    }
}
pub trait PBAManagedDownloaderExtension: Sized + std::ops::Deref {
    unsafe fn shouldDownloadAssetPack_(&self, assetPack: BAAssetPack) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldDownloadAssetPack : assetPack)
    }
}
pub type BAManagedErrorCode = NSInteger;
unsafe extern "C" {
    pub static BADownloaderPriorityMin: BADownloaderPriority;
}
unsafe extern "C" {
    pub static BADownloaderPriorityDefault: BADownloaderPriority;
}
unsafe extern "C" {
    pub static BADownloaderPriorityMax: BADownloaderPriority;
}
unsafe extern "C" {
    pub static BAErrorDomain: NSString;
}
unsafe extern "C" {
    pub static BAManagedErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static BAAssetPackIdentifierErrorKey: NSErrorUserInfoKey;
}

unsafe impl objc2::encode::RefEncode for BADownload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BADownload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BAAppExtensionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BAAppExtensionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BADownloadManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BADownloadManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BAURLDownload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BAURLDownload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BAAssetPack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BAAssetPack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BAAssetPackManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BAAssetPackManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BAAssetPackManifest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BAAssetPackManifest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
