#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::JavaScriptCore::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type NSAttributedStringDocumentReadingOptionKey = NSString;
pub type NSAttributedStringCompletionHandler = *mut ::std::os::raw::c_void;
pub trait NSAttributedString_NSAttributedStringWebKitAdditions: Sized + std::ops::Deref {
    unsafe fn loadFromHTMLWithRequest_options_completionHandler_(
        request: NSURLRequest,
        options: NSDictionary,
        completionHandler: NSAttributedStringCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), loadFromHTMLWithRequest : request, options : options, completionHandler : completionHandler)
    }
    unsafe fn loadFromHTMLWithFileURL_options_completionHandler_(
        fileURL: NSURL,
        options: NSDictionary,
        completionHandler: NSAttributedStringCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), loadFromHTMLWithFileURL : fileURL, options : options, completionHandler : completionHandler)
    }
    unsafe fn loadFromHTMLWithString_options_completionHandler_(
        string: NSString,
        options: NSDictionary,
        completionHandler: NSAttributedStringCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), loadFromHTMLWithString : string, options : options, completionHandler : completionHandler)
    }
    unsafe fn loadFromHTMLWithData_options_completionHandler_(
        data: NSData,
        options: NSDictionary,
        completionHandler: NSAttributedStringCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), loadFromHTMLWithData : data, options : options, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKBackForwardListItem(pub id);
impl std::ops::Deref for WKBackForwardListItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKBackForwardListItem {}
impl WKBackForwardListItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKBackForwardListItem").unwrap(), alloc) })
    }
}
impl INSObject for WKBackForwardListItem {}
impl PNSObject for WKBackForwardListItem {}
impl std::convert::TryFrom<NSObject> for WKBackForwardListItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKBackForwardListItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKBackForwardListItem").unwrap()) };
        if is_kind_of {
            Ok(WKBackForwardListItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKBackForwardListItem")
        }
    }
}
impl IWKBackForwardListItem for WKBackForwardListItem {}
pub trait IWKBackForwardListItem: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn initialURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKBackForwardList(pub id);
impl std::ops::Deref for WKBackForwardList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKBackForwardList {}
impl WKBackForwardList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKBackForwardList").unwrap(), alloc) })
    }
}
impl INSObject for WKBackForwardList {}
impl PNSObject for WKBackForwardList {}
impl std::convert::TryFrom<NSObject> for WKBackForwardList {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKBackForwardList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKBackForwardList").unwrap()) };
        if is_kind_of {
            Ok(WKBackForwardList(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKBackForwardList")
        }
    }
}
impl IWKBackForwardList for WKBackForwardList {}
pub trait IWKBackForwardList: Sized + std::ops::Deref {
    unsafe fn itemAtIndex_(&self, index: NSInteger) -> WKBackForwardListItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemAtIndex : index)
    }
    unsafe fn currentItem(&self) -> WKBackForwardListItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentItem)
    }
    unsafe fn backItem(&self) -> WKBackForwardListItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backItem)
    }
    unsafe fn forwardItem(&self) -> WKBackForwardListItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forwardItem)
    }
    unsafe fn backList(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backList)
    }
    unsafe fn forwardList(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forwardList)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKContentRuleList(pub id);
impl std::ops::Deref for WKContentRuleList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKContentRuleList {}
impl WKContentRuleList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentRuleList").unwrap(), alloc) })
    }
}
impl INSObject for WKContentRuleList {}
impl PNSObject for WKContentRuleList {}
impl std::convert::TryFrom<NSObject> for WKContentRuleList {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKContentRuleList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKContentRuleList").unwrap()) };
        if is_kind_of {
            Ok(WKContentRuleList(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKContentRuleList")
        }
    }
}
impl IWKContentRuleList for WKContentRuleList {}
pub trait IWKContentRuleList: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKContentRuleListStore(pub id);
impl std::ops::Deref for WKContentRuleListStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKContentRuleListStore {}
impl WKContentRuleListStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentRuleListStore").unwrap(), alloc) })
    }
}
impl INSObject for WKContentRuleListStore {}
impl PNSObject for WKContentRuleListStore {}
impl std::convert::TryFrom<NSObject> for WKContentRuleListStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKContentRuleListStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKContentRuleListStore").unwrap()) };
        if is_kind_of {
            Ok(WKContentRuleListStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKContentRuleListStore")
        }
    }
}
impl IWKContentRuleListStore for WKContentRuleListStore {}
pub trait IWKContentRuleListStore: Sized + std::ops::Deref {
    unsafe fn compileContentRuleListForIdentifier_encodedContentRuleList_completionHandler_(
        &self,
        identifier: NSString,
        encodedContentRuleList: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileContentRuleListForIdentifier : identifier, encodedContentRuleList : encodedContentRuleList, completionHandler : completionHandler)
    }
    unsafe fn lookUpContentRuleListForIdentifier_completionHandler_(
        &self,
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookUpContentRuleListForIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn removeContentRuleListForIdentifier_completionHandler_(
        &self,
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeContentRuleListForIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn getAvailableContentRuleListIdentifiers_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAvailableContentRuleListIdentifiers : completionHandler)
    }
    unsafe fn defaultStore() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentRuleListStore").unwrap(), defaultStore)
    }
    unsafe fn storeWithURL_(url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentRuleListStore").unwrap(), storeWithURL : url)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKContentWorld(pub id);
impl std::ops::Deref for WKContentWorld {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKContentWorld {}
impl WKContentWorld {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentWorld").unwrap(), alloc) })
    }
}
impl INSObject for WKContentWorld {}
impl PNSObject for WKContentWorld {}
impl std::convert::TryFrom<NSObject> for WKContentWorld {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKContentWorld, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKContentWorld").unwrap()) };
        if is_kind_of {
            Ok(WKContentWorld(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKContentWorld")
        }
    }
}
impl IWKContentWorld for WKContentWorld {}
pub trait IWKContentWorld: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentWorld").unwrap(), new)
    }
    unsafe fn worldWithName_(name: NSString) -> WKContentWorld
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentWorld").unwrap(), worldWithName : name)
    }
    unsafe fn pageWorld() -> WKContentWorld
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentWorld").unwrap(), pageWorld)
    }
    unsafe fn defaultClientWorld() -> WKContentWorld
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKContentWorld").unwrap(), defaultClientWorld)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKDownload(pub id);
impl std::ops::Deref for WKDownload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKDownload {}
impl WKDownload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKDownload").unwrap(), alloc) })
    }
}
impl PNSProgressReporting for WKDownload {}
impl INSObject for WKDownload {}
impl PNSObject for WKDownload {}
impl std::convert::TryFrom<NSObject> for WKDownload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKDownload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKDownload").unwrap()) };
        if is_kind_of {
            Ok(WKDownload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKDownload")
        }
    }
}
impl IWKDownload for WKDownload {}
pub trait IWKDownload: Sized + std::ops::Deref {
    unsafe fn cancel_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancel : completionHandler)
    }
    unsafe fn originalRequest(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalRequest)
    }
    unsafe fn webView(&self) -> WKWebView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webView)
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
    unsafe fn isUserInitiated(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserInitiated)
    }
    unsafe fn originatingFrame(&self) -> WKFrameInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originatingFrame)
    }
}
pub type WKDownloadRedirectPolicy = NSInteger;
pub type WKDownloadPlaceholderPolicy = NSInteger;
pub trait PWKDownloadDelegate: Sized + std::ops::Deref {
    unsafe fn download_decideDestinationUsingResponse_suggestedFilename_completionHandler_(
        &self,
        download: WKDownload,
        response: NSURLResponse,
        suggestedFilename: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, decideDestinationUsingResponse : response, suggestedFilename : suggestedFilename, completionHandler : completionHandler)
    }
    unsafe fn download_willPerformHTTPRedirection_newRequest_decisionHandler_(
        &self,
        download: WKDownload,
        response: NSHTTPURLResponse,
        request: NSURLRequest,
        decisionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, willPerformHTTPRedirection : response, newRequest : request, decisionHandler : decisionHandler)
    }
    unsafe fn download_didReceiveAuthenticationChallenge_completionHandler_(
        &self,
        download: WKDownload,
        challenge: NSURLAuthenticationChallenge,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, didReceiveAuthenticationChallenge : challenge, completionHandler : completionHandler)
    }
    unsafe fn downloadDidFinish_(&self, download: WKDownload)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadDidFinish : download)
    }
    unsafe fn download_didFailWithError_resumeData_(
        &self,
        download: WKDownload,
        error: NSError,
        resumeData: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, didFailWithError : error, resumeData : resumeData)
    }
    unsafe fn download_decidePlaceholderPolicy_(
        &self,
        download: WKDownload,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, decidePlaceholderPolicy : completionHandler)
    }
    unsafe fn download_didReceivePlaceholderURL_completionHandler_(
        &self,
        download: WKDownload,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, didReceivePlaceholderURL : url, completionHandler : completionHandler)
    }
    unsafe fn download_didReceiveFinalURL_(&self, download: WKDownload, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, download : download, didReceiveFinalURL : url)
    }
}
pub type WKErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKFindConfiguration(pub id);
impl std::ops::Deref for WKFindConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKFindConfiguration {}
impl WKFindConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKFindConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for WKFindConfiguration {}
impl INSObject for WKFindConfiguration {}
impl PNSObject for WKFindConfiguration {}
impl std::convert::TryFrom<NSObject> for WKFindConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKFindConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKFindConfiguration").unwrap()) };
        if is_kind_of {
            Ok(WKFindConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKFindConfiguration")
        }
    }
}
impl IWKFindConfiguration for WKFindConfiguration {}
pub trait IWKFindConfiguration: Sized + std::ops::Deref {
    unsafe fn backwards(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backwards)
    }
    unsafe fn setBackwards_(&self, backwards: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackwards : backwards)
    }
    unsafe fn caseSensitive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, caseSensitive)
    }
    unsafe fn setCaseSensitive_(&self, caseSensitive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaseSensitive : caseSensitive)
    }
    unsafe fn wraps(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wraps)
    }
    unsafe fn setWraps_(&self, wraps: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWraps : wraps)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKFindResult(pub id);
impl std::ops::Deref for WKFindResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKFindResult {}
impl WKFindResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKFindResult").unwrap(), alloc) })
    }
}
impl PNSCopying for WKFindResult {}
impl INSObject for WKFindResult {}
impl PNSObject for WKFindResult {}
impl std::convert::TryFrom<NSObject> for WKFindResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKFindResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKFindResult").unwrap()) };
        if is_kind_of {
            Ok(WKFindResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKFindResult")
        }
    }
}
impl IWKFindResult for WKFindResult {}
pub trait IWKFindResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn matchFound(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchFound)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKSecurityOrigin(pub id);
impl std::ops::Deref for WKSecurityOrigin {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKSecurityOrigin {}
impl WKSecurityOrigin {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKSecurityOrigin").unwrap(), alloc) })
    }
}
impl INSObject for WKSecurityOrigin {}
impl PNSObject for WKSecurityOrigin {}
impl std::convert::TryFrom<NSObject> for WKSecurityOrigin {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKSecurityOrigin, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKSecurityOrigin").unwrap()) };
        if is_kind_of {
            Ok(WKSecurityOrigin(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKSecurityOrigin")
        }
    }
}
impl IWKSecurityOrigin for WKSecurityOrigin {}
pub trait IWKSecurityOrigin: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn protocol(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocol)
    }
    unsafe fn host(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, host)
    }
    unsafe fn port(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, port)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebView(pub id);
impl std::ops::Deref for WKWebView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebView {}
impl WKWebView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebView").unwrap(), alloc) })
    }
}
impl INSView for WKWebView {}
impl PNSAnimatablePropertyContainer for WKWebView {}
impl PNSUserInterfaceItemIdentification for WKWebView {}
impl PNSDraggingDestination for WKWebView {}
impl PNSAppearanceCustomization for WKWebView {}
impl PNSAccessibilityElement for WKWebView {}
impl PNSAccessibility for WKWebView {}
impl std::convert::TryFrom<NSView> for WKWebView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<WKWebView, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebView").unwrap()) };
        if is_kind_of {
            Ok(WKWebView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to WKWebView")
        }
    }
}
impl INSResponder for WKWebView {}
impl PNSCoding for WKWebView {}
impl INSObject for WKWebView {}
impl PNSObject for WKWebView {}
impl IWKWebView for WKWebView {}
pub trait IWKWebView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_configuration_(
        &self,
        frame: CGRect,
        configuration: WKWebViewConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame, configuration : configuration)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn loadRequest_(&self, request: NSURLRequest) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadRequest : request)
    }
    unsafe fn loadFileURL_allowingReadAccessToURL_(
        &self,
        URL: NSURL,
        readAccessURL: NSURL,
    ) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFileURL : URL, allowingReadAccessToURL : readAccessURL)
    }
    unsafe fn loadHTMLString_baseURL_(&self, string: NSString, baseURL: NSURL) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadHTMLString : string, baseURL : baseURL)
    }
    unsafe fn loadData_MIMEType_characterEncodingName_baseURL_(
        &self,
        data: NSData,
        MIMEType: NSString,
        characterEncodingName: NSString,
        baseURL: NSURL,
    ) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadData : data, MIMEType : MIMEType, characterEncodingName : characterEncodingName, baseURL : baseURL)
    }
    unsafe fn goToBackForwardListItem_(&self, item: WKBackForwardListItem) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToBackForwardListItem : item)
    }
    unsafe fn goBack(&self) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, goBack)
    }
    unsafe fn goForward(&self) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, goForward)
    }
    unsafe fn reload(&self) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reload)
    }
    unsafe fn reloadFromOrigin(&self) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadFromOrigin)
    }
    unsafe fn stopLoading(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopLoading)
    }
    unsafe fn evaluateJavaScript_completionHandler_(
        &self,
        javaScriptString: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateJavaScript : javaScriptString, completionHandler : completionHandler)
    }
    unsafe fn evaluateJavaScript_inFrame_inContentWorld_completionHandler_(
        &self,
        javaScriptString: NSString,
        frame: WKFrameInfo,
        contentWorld: WKContentWorld,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateJavaScript : javaScriptString, inFrame : frame, inContentWorld : contentWorld, completionHandler : completionHandler)
    }
    unsafe fn callAsyncJavaScript_arguments_inFrame_inContentWorld_completionHandler_(
        &self,
        functionBody: NSString,
        arguments: NSDictionary,
        frame: WKFrameInfo,
        contentWorld: WKContentWorld,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, callAsyncJavaScript : functionBody, arguments : arguments, inFrame : frame, inContentWorld : contentWorld, completionHandler : completionHandler)
    }
    unsafe fn closeAllMediaPresentationsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, closeAllMediaPresentationsWithCompletionHandler : completionHandler)
    }
    unsafe fn closeAllMediaPresentations(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeAllMediaPresentations)
    }
    unsafe fn pauseAllMediaPlaybackWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseAllMediaPlaybackWithCompletionHandler : completionHandler)
    }
    unsafe fn pauseAllMediaPlayback_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseAllMediaPlayback : completionHandler)
    }
    unsafe fn setAllMediaPlaybackSuspended_completionHandler_(
        &self,
        suspended: BOOL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllMediaPlaybackSuspended : suspended, completionHandler : completionHandler)
    }
    unsafe fn resumeAllMediaPlayback_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeAllMediaPlayback : completionHandler)
    }
    unsafe fn suspendAllMediaPlayback_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, suspendAllMediaPlayback : completionHandler)
    }
    unsafe fn requestMediaPlaybackStateWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestMediaPlaybackStateWithCompletionHandler : completionHandler)
    }
    unsafe fn requestMediaPlaybackState_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestMediaPlaybackState : completionHandler)
    }
    unsafe fn setCameraCaptureState_completionHandler_(
        &self,
        state: WKMediaCaptureState,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraCaptureState : state, completionHandler : completionHandler)
    }
    unsafe fn setMicrophoneCaptureState_completionHandler_(
        &self,
        state: WKMediaCaptureState,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMicrophoneCaptureState : state, completionHandler : completionHandler)
    }
    unsafe fn takeSnapshotWithConfiguration_completionHandler_(
        &self,
        snapshotConfiguration: WKSnapshotConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, takeSnapshotWithConfiguration : snapshotConfiguration, completionHandler : completionHandler)
    }
    unsafe fn createPDFWithConfiguration_completionHandler_(
        &self,
        pdfConfiguration: WKPDFConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createPDFWithConfiguration : pdfConfiguration, completionHandler : completionHandler)
    }
    unsafe fn createWebArchiveDataWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createWebArchiveDataWithCompletionHandler : completionHandler)
    }
    unsafe fn setMagnification_centeredAtPoint_(&self, magnification: CGFloat, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagnification : magnification, centeredAtPoint : point)
    }
    unsafe fn findString_withConfiguration_completionHandler_(
        &self,
        string: NSString,
        configuration: WKFindConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findString : string, withConfiguration : configuration, completionHandler : completionHandler)
    }
    unsafe fn startDownloadUsingRequest_completionHandler_(
        &self,
        request: NSURLRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startDownloadUsingRequest : request, completionHandler : completionHandler)
    }
    unsafe fn resumeDownloadFromResumeData_completionHandler_(
        &self,
        resumeData: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeDownloadFromResumeData : resumeData, completionHandler : completionHandler)
    }
    unsafe fn loadSimulatedRequest_response_responseData_(
        &self,
        request: NSURLRequest,
        response: NSURLResponse,
        data: NSData,
    ) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadSimulatedRequest : request, response : response, responseData : data)
    }
    unsafe fn loadSimulatedRequest_withResponse_responseData_(
        &self,
        request: NSURLRequest,
        response: NSURLResponse,
        data: NSData,
    ) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadSimulatedRequest : request, withResponse : response, responseData : data)
    }
    unsafe fn loadFileRequest_allowingReadAccessToURL_(
        &self,
        request: NSURLRequest,
        readAccessURL: NSURL,
    ) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFileRequest : request, allowingReadAccessToURL : readAccessURL)
    }
    unsafe fn loadSimulatedRequest_responseHTMLString_(
        &self,
        request: NSURLRequest,
        string: NSString,
    ) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadSimulatedRequest : request, responseHTMLString : string)
    }
    unsafe fn loadSimulatedRequest_withResponseHTMLString_(
        &self,
        request: NSURLRequest,
        string: NSString,
    ) -> WKNavigation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadSimulatedRequest : request, withResponseHTMLString : string)
    }
    unsafe fn printOperationWithPrintInfo_(&self, printInfo: NSPrintInfo) -> NSPrintOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, printOperationWithPrintInfo : printInfo)
    }
    unsafe fn setMinimumViewportInset_maximumViewportInset_(
        &self,
        minimumViewportInset: NSEdgeInsets,
        maximumViewportInset: NSEdgeInsets,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumViewportInset : minimumViewportInset, maximumViewportInset : maximumViewportInset)
    }
    unsafe fn fetchDataOfTypes_completionHandler_(
        &self,
        dataTypes: WKWebViewDataType,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchDataOfTypes : dataTypes, completionHandler : completionHandler)
    }
    unsafe fn restoreData_completionHandler_(
        &self,
        data: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreData : data, completionHandler : completionHandler)
    }
    unsafe fn configuration(&self) -> WKWebViewConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn navigationDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, navigationDelegate)
    }
    unsafe fn setNavigationDelegate_(&self, navigationDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNavigationDelegate : navigationDelegate)
    }
    unsafe fn UIDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UIDelegate)
    }
    unsafe fn setUIDelegate_(&self, UIDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUIDelegate : UIDelegate)
    }
    unsafe fn backForwardList(&self) -> WKBackForwardList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backForwardList)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn estimatedProgress(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, estimatedProgress)
    }
    unsafe fn hasOnlySecureContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasOnlySecureContent)
    }
    unsafe fn serverTrust(&self) -> SecTrustRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverTrust)
    }
    unsafe fn canGoBack(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoBack)
    }
    unsafe fn canGoForward(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoForward)
    }
    unsafe fn cameraCaptureState(&self) -> WKMediaCaptureState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraCaptureState)
    }
    unsafe fn microphoneCaptureState(&self) -> WKMediaCaptureState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, microphoneCaptureState)
    }
    unsafe fn allowsBackForwardNavigationGestures(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsBackForwardNavigationGestures)
    }
    unsafe fn setAllowsBackForwardNavigationGestures_(
        &self,
        allowsBackForwardNavigationGestures: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsBackForwardNavigationGestures : allowsBackForwardNavigationGestures)
    }
    unsafe fn customUserAgent(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customUserAgent)
    }
    unsafe fn setCustomUserAgent_(&self, customUserAgent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomUserAgent : customUserAgent)
    }
    unsafe fn allowsLinkPreview(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsLinkPreview)
    }
    unsafe fn setAllowsLinkPreview_(&self, allowsLinkPreview: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsLinkPreview : allowsLinkPreview)
    }
    unsafe fn allowsMagnification(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsMagnification)
    }
    unsafe fn setAllowsMagnification_(&self, allowsMagnification: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsMagnification : allowsMagnification)
    }
    unsafe fn magnification(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magnification)
    }
    unsafe fn setMagnification_(&self, magnification: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagnification : magnification)
    }
    unsafe fn pageZoom(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageZoom)
    }
    unsafe fn setPageZoom_(&self, pageZoom: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageZoom : pageZoom)
    }
    unsafe fn mediaType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaType)
    }
    unsafe fn setMediaType_(&self, mediaType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaType : mediaType)
    }
    unsafe fn interactionState(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interactionState)
    }
    unsafe fn setInteractionState_(&self, interactionState: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteractionState : interactionState)
    }
    unsafe fn isBlockedByScreenTime(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlockedByScreenTime)
    }
    unsafe fn themeColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, themeColor)
    }
    unsafe fn underPageBackgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, underPageBackgroundColor)
    }
    unsafe fn setUnderPageBackgroundColor_(&self, underPageBackgroundColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnderPageBackgroundColor : underPageBackgroundColor)
    }
    unsafe fn fullscreenState(&self) -> WKFullscreenState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullscreenState)
    }
    unsafe fn minimumViewportInset(&self) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumViewportInset)
    }
    unsafe fn maximumViewportInset(&self) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumViewportInset)
    }
    unsafe fn isInspectable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInspectable)
    }
    unsafe fn setInspectable_(&self, inspectable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInspectable : inspectable)
    }
    unsafe fn isWritingToolsActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isWritingToolsActive)
    }
    unsafe fn obscuredContentInsets(&self) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, obscuredContentInsets)
    }
    unsafe fn setObscuredContentInsets_(&self, obscuredContentInsets: NSEdgeInsets)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObscuredContentInsets : obscuredContentInsets)
    }
    unsafe fn handlesURLScheme_(urlScheme: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebView").unwrap(), handlesURLScheme : urlScheme)
    }
}
pub type WKMediaPlaybackState = NSInteger;
pub type WKMediaCaptureState = NSInteger;
pub type WKFullscreenState = NSInteger;
pub type WKWebViewDataType = NSUInteger;
impl WKWebView_WKIBActions for WKWebView {}
pub trait WKWebView_WKIBActions: Sized + std::ops::Deref {
    unsafe fn goBack_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goBack : sender)
    }
    unsafe fn goForward_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goForward : sender)
    }
    unsafe fn reload_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reload : sender)
    }
    unsafe fn reloadFromOrigin_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reloadFromOrigin : sender)
    }
    unsafe fn stopLoading_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopLoading : sender)
    }
}
impl WKWebView_WKNSTextFinderClient for WKWebView {}
pub trait WKWebView_WKNSTextFinderClient: Sized + std::ops::Deref {}
impl WKWebView_WKDeprecated for WKWebView {}
pub trait WKWebView_WKDeprecated: Sized + std::ops::Deref {
    unsafe fn certificateChain(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, certificateChain)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKFrameInfo(pub id);
impl std::ops::Deref for WKFrameInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKFrameInfo {}
impl WKFrameInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKFrameInfo").unwrap(), alloc) })
    }
}
impl PNSCopying for WKFrameInfo {}
impl INSObject for WKFrameInfo {}
impl PNSObject for WKFrameInfo {}
impl std::convert::TryFrom<NSObject> for WKFrameInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKFrameInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKFrameInfo").unwrap()) };
        if is_kind_of {
            Ok(WKFrameInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKFrameInfo")
        }
    }
}
impl IWKFrameInfo for WKFrameInfo {}
pub trait IWKFrameInfo: Sized + std::ops::Deref {
    unsafe fn isMainFrame(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMainFrame)
    }
    unsafe fn request(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, request)
    }
    unsafe fn securityOrigin(&self) -> WKSecurityOrigin
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, securityOrigin)
    }
    unsafe fn webView(&self) -> WKWebView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webView)
    }
}
pub type WKCookiePolicy = NSInteger;
pub trait PWKHTTPCookieStoreObserver: Sized + std::ops::Deref {
    unsafe fn cookiesDidChangeInCookieStore_(&self, cookieStore: WKHTTPCookieStore)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cookiesDidChangeInCookieStore : cookieStore)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKHTTPCookieStore(pub id);
impl std::ops::Deref for WKHTTPCookieStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKHTTPCookieStore {}
impl WKHTTPCookieStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKHTTPCookieStore").unwrap(), alloc) })
    }
}
impl INSObject for WKHTTPCookieStore {}
impl PNSObject for WKHTTPCookieStore {}
impl std::convert::TryFrom<NSObject> for WKHTTPCookieStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKHTTPCookieStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKHTTPCookieStore").unwrap()) };
        if is_kind_of {
            Ok(WKHTTPCookieStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKHTTPCookieStore")
        }
    }
}
impl IWKHTTPCookieStore for WKHTTPCookieStore {}
pub trait IWKHTTPCookieStore: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn getAllCookies_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAllCookies : completionHandler)
    }
    unsafe fn setCookie_completionHandler_(
        &self,
        cookie: NSHTTPCookie,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCookie : cookie, completionHandler : completionHandler)
    }
    unsafe fn setCookies_completionHandler_(
        &self,
        cookies: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCookies : cookies, completionHandler : completionHandler)
    }
    unsafe fn deleteCookie_completionHandler_(
        &self,
        cookie: NSHTTPCookie,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteCookie : cookie, completionHandler : completionHandler)
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
    unsafe fn setCookiePolicy_completionHandler_(
        &self,
        policy: WKCookiePolicy,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCookiePolicy : policy, completionHandler : completionHandler)
    }
    unsafe fn getCookiePolicy_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getCookiePolicy : completionHandler)
    }
}
pub type WKContentMode = NSInteger;
pub type WKWebpagePreferencesUpgradeToHTTPSPolicy = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebpagePreferences(pub id);
impl std::ops::Deref for WKWebpagePreferences {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebpagePreferences {}
impl WKWebpagePreferences {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebpagePreferences").unwrap(), alloc) })
    }
}
impl INSObject for WKWebpagePreferences {}
impl PNSObject for WKWebpagePreferences {}
impl std::convert::TryFrom<NSObject> for WKWebpagePreferences {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebpagePreferences, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebpagePreferences").unwrap()) };
        if is_kind_of {
            Ok(WKWebpagePreferences(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebpagePreferences")
        }
    }
}
impl IWKWebpagePreferences for WKWebpagePreferences {}
pub trait IWKWebpagePreferences: Sized + std::ops::Deref {
    unsafe fn preferredContentMode(&self) -> WKContentMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredContentMode)
    }
    unsafe fn setPreferredContentMode_(&self, preferredContentMode: WKContentMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredContentMode : preferredContentMode)
    }
    unsafe fn allowsContentJavaScript(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsContentJavaScript)
    }
    unsafe fn setAllowsContentJavaScript_(&self, allowsContentJavaScript: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsContentJavaScript : allowsContentJavaScript)
    }
    unsafe fn isLockdownModeEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLockdownModeEnabled)
    }
    unsafe fn setLockdownModeEnabled_(&self, lockdownModeEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLockdownModeEnabled : lockdownModeEnabled)
    }
    unsafe fn preferredHTTPSNavigationPolicy(&self) -> WKWebpagePreferencesUpgradeToHTTPSPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredHTTPSNavigationPolicy)
    }
    unsafe fn setPreferredHTTPSNavigationPolicy_(
        &self,
        preferredHTTPSNavigationPolicy: WKWebpagePreferencesUpgradeToHTTPSPolicy,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredHTTPSNavigationPolicy : preferredHTTPSNavigationPolicy)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKNavigation(pub id);
impl std::ops::Deref for WKNavigation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKNavigation {}
impl WKNavigation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKNavigation").unwrap(), alloc) })
    }
}
impl INSObject for WKNavigation {}
impl PNSObject for WKNavigation {}
impl std::convert::TryFrom<NSObject> for WKNavigation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKNavigation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKNavigation").unwrap()) };
        if is_kind_of {
            Ok(WKNavigation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKNavigation")
        }
    }
}
impl IWKNavigation for WKNavigation {}
pub trait IWKNavigation: Sized + std::ops::Deref {
    unsafe fn effectiveContentMode(&self) -> WKContentMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectiveContentMode)
    }
}
pub type WKNavigationType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKNavigationAction(pub id);
impl std::ops::Deref for WKNavigationAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKNavigationAction {}
impl WKNavigationAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKNavigationAction").unwrap(), alloc) })
    }
}
impl INSObject for WKNavigationAction {}
impl PNSObject for WKNavigationAction {}
impl std::convert::TryFrom<NSObject> for WKNavigationAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKNavigationAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKNavigationAction").unwrap()) };
        if is_kind_of {
            Ok(WKNavigationAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKNavigationAction")
        }
    }
}
impl IWKNavigationAction for WKNavigationAction {}
pub trait IWKNavigationAction: Sized + std::ops::Deref {
    unsafe fn sourceFrame(&self) -> WKFrameInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn targetFrame(&self) -> WKFrameInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetFrame)
    }
    unsafe fn navigationType(&self) -> WKNavigationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, navigationType)
    }
    unsafe fn request(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, request)
    }
    unsafe fn shouldPerformDownload(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldPerformDownload)
    }
    unsafe fn isContentRuleListRedirect(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContentRuleListRedirect)
    }
    unsafe fn modifierFlags(&self) -> NSEventModifierFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifierFlags)
    }
    unsafe fn buttonNumber(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonNumber)
    }
}
pub type WKNavigationActionPolicy = NSInteger;
pub type WKNavigationResponsePolicy = NSInteger;
pub trait PWKNavigationDelegate: Sized + std::ops::Deref {
    unsafe fn webView_decidePolicyForNavigationAction_decisionHandler_(
        &self,
        webView: WKWebView,
        navigationAction: WKNavigationAction,
        decisionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, decidePolicyForNavigationAction : navigationAction, decisionHandler : decisionHandler)
    }
    unsafe fn webView_decidePolicyForNavigationAction_preferences_decisionHandler_(
        &self,
        webView: WKWebView,
        navigationAction: WKNavigationAction,
        preferences: WKWebpagePreferences,
        decisionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, decidePolicyForNavigationAction : navigationAction, preferences : preferences, decisionHandler : decisionHandler)
    }
    unsafe fn webView_decidePolicyForNavigationResponse_decisionHandler_(
        &self,
        webView: WKWebView,
        navigationResponse: WKNavigationResponse,
        decisionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, decidePolicyForNavigationResponse : navigationResponse, decisionHandler : decisionHandler)
    }
    unsafe fn webView_didStartProvisionalNavigation_(
        &self,
        webView: WKWebView,
        navigation: WKNavigation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, didStartProvisionalNavigation : navigation)
    }
    unsafe fn webView_didReceiveServerRedirectForProvisionalNavigation_(
        &self,
        webView: WKWebView,
        navigation: WKNavigation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, didReceiveServerRedirectForProvisionalNavigation : navigation)
    }
    unsafe fn webView_didFailProvisionalNavigation_withError_(
        &self,
        webView: WKWebView,
        navigation: WKNavigation,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, didFailProvisionalNavigation : navigation, withError : error)
    }
    unsafe fn webView_didCommitNavigation_(&self, webView: WKWebView, navigation: WKNavigation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, didCommitNavigation : navigation)
    }
    unsafe fn webView_didFinishNavigation_(&self, webView: WKWebView, navigation: WKNavigation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, didFinishNavigation : navigation)
    }
    unsafe fn webView_didFailNavigation_withError_(
        &self,
        webView: WKWebView,
        navigation: WKNavigation,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, didFailNavigation : navigation, withError : error)
    }
    unsafe fn webView_didReceiveAuthenticationChallenge_completionHandler_(
        &self,
        webView: WKWebView,
        challenge: NSURLAuthenticationChallenge,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, didReceiveAuthenticationChallenge : challenge, completionHandler : completionHandler)
    }
    unsafe fn webViewWebContentProcessDidTerminate_(&self, webView: WKWebView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webViewWebContentProcessDidTerminate : webView)
    }
    unsafe fn webView_authenticationChallenge_shouldAllowDeprecatedTLS_(
        &self,
        webView: WKWebView,
        challenge: NSURLAuthenticationChallenge,
        decisionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, authenticationChallenge : challenge, shouldAllowDeprecatedTLS : decisionHandler)
    }
    unsafe fn webView_navigationAction_didBecomeDownload_(
        &self,
        webView: WKWebView,
        navigationAction: WKNavigationAction,
        download: WKDownload,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, navigationAction : navigationAction, didBecomeDownload : download)
    }
    unsafe fn webView_navigationResponse_didBecomeDownload_(
        &self,
        webView: WKWebView,
        navigationResponse: WKNavigationResponse,
        download: WKDownload,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, navigationResponse : navigationResponse, didBecomeDownload : download)
    }
    unsafe fn webView_shouldGoToBackForwardListItem_willUseInstantBack_completionHandler_(
        &self,
        webView: WKWebView,
        backForwardListItem: WKBackForwardListItem,
        willUseInstantBack: BOOL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, shouldGoToBackForwardListItem : backForwardListItem, willUseInstantBack : willUseInstantBack, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKNavigationResponse(pub id);
impl std::ops::Deref for WKNavigationResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKNavigationResponse {}
impl WKNavigationResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKNavigationResponse").unwrap(), alloc) })
    }
}
impl INSObject for WKNavigationResponse {}
impl PNSObject for WKNavigationResponse {}
impl std::convert::TryFrom<NSObject> for WKNavigationResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKNavigationResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKNavigationResponse").unwrap()) };
        if is_kind_of {
            Ok(WKNavigationResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKNavigationResponse")
        }
    }
}
impl IWKNavigationResponse for WKNavigationResponse {}
pub trait IWKNavigationResponse: Sized + std::ops::Deref {
    unsafe fn isForMainFrame(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isForMainFrame)
    }
    unsafe fn response(&self) -> NSURLResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, response)
    }
    unsafe fn canShowMIMEType(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canShowMIMEType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKOpenPanelParameters(pub id);
impl std::ops::Deref for WKOpenPanelParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKOpenPanelParameters {}
impl WKOpenPanelParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKOpenPanelParameters").unwrap(), alloc) })
    }
}
impl INSObject for WKOpenPanelParameters {}
impl PNSObject for WKOpenPanelParameters {}
impl std::convert::TryFrom<NSObject> for WKOpenPanelParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKOpenPanelParameters, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKOpenPanelParameters").unwrap()) };
        if is_kind_of {
            Ok(WKOpenPanelParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKOpenPanelParameters")
        }
    }
}
impl IWKOpenPanelParameters for WKOpenPanelParameters {}
pub trait IWKOpenPanelParameters: Sized + std::ops::Deref {
    unsafe fn allowsMultipleSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsMultipleSelection)
    }
    unsafe fn allowsDirectories(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsDirectories)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKPDFConfiguration(pub id);
impl std::ops::Deref for WKPDFConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKPDFConfiguration {}
impl WKPDFConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKPDFConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for WKPDFConfiguration {}
impl INSObject for WKPDFConfiguration {}
impl PNSObject for WKPDFConfiguration {}
impl std::convert::TryFrom<NSObject> for WKPDFConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKPDFConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKPDFConfiguration").unwrap()) };
        if is_kind_of {
            Ok(WKPDFConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKPDFConfiguration")
        }
    }
}
impl IWKPDFConfiguration for WKPDFConfiguration {}
pub trait IWKPDFConfiguration: Sized + std::ops::Deref {
    unsafe fn rect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rect)
    }
    unsafe fn setRect_(&self, rect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRect : rect)
    }
    unsafe fn allowTransparentBackground(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowTransparentBackground)
    }
    unsafe fn setAllowTransparentBackground_(&self, allowTransparentBackground: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowTransparentBackground : allowTransparentBackground)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKPreferences(pub id);
impl std::ops::Deref for WKPreferences {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKPreferences {}
impl WKPreferences {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKPreferences").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for WKPreferences {}
impl INSObject for WKPreferences {}
impl PNSObject for WKPreferences {}
impl std::convert::TryFrom<NSObject> for WKPreferences {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKPreferences, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKPreferences").unwrap()) };
        if is_kind_of {
            Ok(WKPreferences(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKPreferences")
        }
    }
}
impl IWKPreferences for WKPreferences {}
pub trait IWKPreferences: Sized + std::ops::Deref {
    unsafe fn minimumFontSize(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumFontSize)
    }
    unsafe fn setMinimumFontSize_(&self, minimumFontSize: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumFontSize : minimumFontSize)
    }
    unsafe fn javaScriptCanOpenWindowsAutomatically(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, javaScriptCanOpenWindowsAutomatically)
    }
    unsafe fn setJavaScriptCanOpenWindowsAutomatically_(
        &self,
        javaScriptCanOpenWindowsAutomatically: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJavaScriptCanOpenWindowsAutomatically : javaScriptCanOpenWindowsAutomatically)
    }
    unsafe fn isFraudulentWebsiteWarningEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFraudulentWebsiteWarningEnabled)
    }
    unsafe fn setFraudulentWebsiteWarningEnabled_(&self, fraudulentWebsiteWarningEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFraudulentWebsiteWarningEnabled : fraudulentWebsiteWarningEnabled)
    }
    unsafe fn shouldPrintBackgrounds(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldPrintBackgrounds)
    }
    unsafe fn setShouldPrintBackgrounds_(&self, shouldPrintBackgrounds: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldPrintBackgrounds : shouldPrintBackgrounds)
    }
    unsafe fn tabFocusesLinks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabFocusesLinks)
    }
    unsafe fn setTabFocusesLinks_(&self, tabFocusesLinks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTabFocusesLinks : tabFocusesLinks)
    }
    unsafe fn isTextInteractionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTextInteractionEnabled)
    }
    unsafe fn setTextInteractionEnabled_(&self, textInteractionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextInteractionEnabled : textInteractionEnabled)
    }
    unsafe fn isSiteSpecificQuirksModeEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSiteSpecificQuirksModeEnabled)
    }
    unsafe fn setSiteSpecificQuirksModeEnabled_(&self, siteSpecificQuirksModeEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSiteSpecificQuirksModeEnabled : siteSpecificQuirksModeEnabled)
    }
    unsafe fn isElementFullscreenEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isElementFullscreenEnabled)
    }
    unsafe fn setElementFullscreenEnabled_(&self, elementFullscreenEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElementFullscreenEnabled : elementFullscreenEnabled)
    }
    unsafe fn inactiveSchedulingPolicy(&self) -> WKInactiveSchedulingPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inactiveSchedulingPolicy)
    }
    unsafe fn setInactiveSchedulingPolicy_(
        &self,
        inactiveSchedulingPolicy: WKInactiveSchedulingPolicy,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInactiveSchedulingPolicy : inactiveSchedulingPolicy)
    }
}
pub type WKInactiveSchedulingPolicy = NSInteger;
impl WKPreferences_WKDeprecated for WKPreferences {}
pub trait WKPreferences_WKDeprecated: Sized + std::ops::Deref {
    unsafe fn javaEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, javaEnabled)
    }
    unsafe fn setJavaEnabled_(&self, javaEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJavaEnabled : javaEnabled)
    }
    unsafe fn plugInsEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, plugInsEnabled)
    }
    unsafe fn setPlugInsEnabled_(&self, plugInsEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlugInsEnabled : plugInsEnabled)
    }
    unsafe fn javaScriptEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, javaScriptEnabled)
    }
    unsafe fn setJavaScriptEnabled_(&self, javaScriptEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJavaScriptEnabled : javaScriptEnabled)
    }
    unsafe fn isLookToScrollEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLookToScrollEnabled)
    }
    unsafe fn setIsLookToScrollEnabled_(&self, isLookToScrollEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsLookToScrollEnabled : isLookToScrollEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKProcessPool(pub id);
impl std::ops::Deref for WKProcessPool {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKProcessPool {}
impl WKProcessPool {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKProcessPool").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for WKProcessPool {}
impl INSObject for WKProcessPool {}
impl PNSObject for WKProcessPool {}
impl std::convert::TryFrom<NSObject> for WKProcessPool {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKProcessPool, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKProcessPool").unwrap()) };
        if is_kind_of {
            Ok(WKProcessPool(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKProcessPool")
        }
    }
}
impl IWKProcessPool for WKProcessPool {}
pub trait IWKProcessPool: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKScriptMessage(pub id);
impl std::ops::Deref for WKScriptMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKScriptMessage {}
impl WKScriptMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKScriptMessage").unwrap(), alloc) })
    }
}
impl INSObject for WKScriptMessage {}
impl PNSObject for WKScriptMessage {}
impl std::convert::TryFrom<NSObject> for WKScriptMessage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKScriptMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKScriptMessage").unwrap()) };
        if is_kind_of {
            Ok(WKScriptMessage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKScriptMessage")
        }
    }
}
impl IWKScriptMessage for WKScriptMessage {}
pub trait IWKScriptMessage: Sized + std::ops::Deref {
    unsafe fn body(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body)
    }
    unsafe fn webView(&self) -> WKWebView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webView)
    }
    unsafe fn frameInfo(&self) -> WKFrameInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameInfo)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn world(&self) -> WKContentWorld
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, world)
    }
}
pub trait PWKScriptMessageHandler: Sized + std::ops::Deref {
    unsafe fn userContentController_didReceiveScriptMessage_(
        &self,
        userContentController: WKUserContentController,
        message: WKScriptMessage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userContentController : userContentController, didReceiveScriptMessage : message)
    }
}
pub trait PWKScriptMessageHandlerWithReply: Sized + std::ops::Deref {
    unsafe fn userContentController_didReceiveScriptMessage_replyHandler_(
        &self,
        userContentController: WKUserContentController,
        message: WKScriptMessage,
        replyHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userContentController : userContentController, didReceiveScriptMessage : message, replyHandler : replyHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKSnapshotConfiguration(pub id);
impl std::ops::Deref for WKSnapshotConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKSnapshotConfiguration {}
impl WKSnapshotConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKSnapshotConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for WKSnapshotConfiguration {}
impl INSObject for WKSnapshotConfiguration {}
impl PNSObject for WKSnapshotConfiguration {}
impl std::convert::TryFrom<NSObject> for WKSnapshotConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKSnapshotConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKSnapshotConfiguration").unwrap()) };
        if is_kind_of {
            Ok(WKSnapshotConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKSnapshotConfiguration")
        }
    }
}
impl IWKSnapshotConfiguration for WKSnapshotConfiguration {}
pub trait IWKSnapshotConfiguration: Sized + std::ops::Deref {
    unsafe fn rect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rect)
    }
    unsafe fn setRect_(&self, rect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRect : rect)
    }
    unsafe fn snapshotWidth(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotWidth)
    }
    unsafe fn setSnapshotWidth_(&self, snapshotWidth: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSnapshotWidth : snapshotWidth)
    }
    unsafe fn afterScreenUpdates(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, afterScreenUpdates)
    }
    unsafe fn setAfterScreenUpdates_(&self, afterScreenUpdates: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAfterScreenUpdates : afterScreenUpdates)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKPreviewElementInfo(pub id);
impl std::ops::Deref for WKPreviewElementInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKPreviewElementInfo {}
impl WKPreviewElementInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKPreviewElementInfo").unwrap(), alloc) })
    }
}
impl IWKPreviewElementInfo for WKPreviewElementInfo {}
pub trait IWKPreviewElementInfo: Sized + std::ops::Deref {}
pub type WKPermissionDecision = NSInteger;
pub type WKMediaCaptureType = NSInteger;
pub type WKDialogResult = NSInteger;
pub trait PWKUIDelegate: Sized + std::ops::Deref {
    unsafe fn webView_createWebViewWithConfiguration_forNavigationAction_windowFeatures_(
        &self,
        webView: WKWebView,
        configuration: WKWebViewConfiguration,
        navigationAction: WKNavigationAction,
        windowFeatures: WKWindowFeatures,
    ) -> WKWebView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, createWebViewWithConfiguration : configuration, forNavigationAction : navigationAction, windowFeatures : windowFeatures)
    }
    unsafe fn webViewDidClose_(&self, webView: WKWebView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webViewDidClose : webView)
    }
    unsafe fn webView_runJavaScriptAlertPanelWithMessage_initiatedByFrame_completionHandler_(
        &self,
        webView: WKWebView,
        message: NSString,
        frame: WKFrameInfo,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, runJavaScriptAlertPanelWithMessage : message, initiatedByFrame : frame, completionHandler : completionHandler)
    }
    unsafe fn webView_runJavaScriptConfirmPanelWithMessage_initiatedByFrame_completionHandler_(
        &self,
        webView: WKWebView,
        message: NSString,
        frame: WKFrameInfo,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, runJavaScriptConfirmPanelWithMessage : message, initiatedByFrame : frame, completionHandler : completionHandler)
    }
    unsafe fn webView_runJavaScriptTextInputPanelWithPrompt_defaultText_initiatedByFrame_completionHandler_(
        &self,
        webView: WKWebView,
        prompt: NSString,
        defaultText: NSString,
        frame: WKFrameInfo,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, runJavaScriptTextInputPanelWithPrompt : prompt, defaultText : defaultText, initiatedByFrame : frame, completionHandler : completionHandler)
    }
    unsafe fn webView_requestMediaCapturePermissionForOrigin_initiatedByFrame_type_decisionHandler_(
        &self,
        webView: WKWebView,
        origin: WKSecurityOrigin,
        frame: WKFrameInfo,
        type_: WKMediaCaptureType,
        decisionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, requestMediaCapturePermissionForOrigin : origin, initiatedByFrame : frame, r#type : type_, decisionHandler : decisionHandler)
    }
    unsafe fn webView_requestDeviceOrientationAndMotionPermissionForOrigin_initiatedByFrame_decisionHandler_(
        &self,
        webView: WKWebView,
        origin: WKSecurityOrigin,
        frame: WKFrameInfo,
        decisionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, requestDeviceOrientationAndMotionPermissionForOrigin : origin, initiatedByFrame : frame, decisionHandler : decisionHandler)
    }
    unsafe fn webView_runOpenPanelWithParameters_initiatedByFrame_completionHandler_(
        &self,
        webView: WKWebView,
        parameters: WKOpenPanelParameters,
        frame: WKFrameInfo,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, runOpenPanelWithParameters : parameters, initiatedByFrame : frame, completionHandler : completionHandler)
    }
}
pub trait PWKURLSchemeHandler: Sized + std::ops::Deref {
    unsafe fn webView_startURLSchemeTask_(&self, webView: WKWebView, urlSchemeTask: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, startURLSchemeTask : urlSchemeTask)
    }
    unsafe fn webView_stopURLSchemeTask_(&self, webView: WKWebView, urlSchemeTask: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webView : webView, stopURLSchemeTask : urlSchemeTask)
    }
}
pub trait PWKURLSchemeTask: Sized + std::ops::Deref {
    unsafe fn didReceiveResponse_(&self, response: NSURLResponse)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveResponse : response)
    }
    unsafe fn didReceiveData_(&self, data: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveData : data)
    }
    unsafe fn didFinish(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didFinish)
    }
    unsafe fn didFailWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didFailWithError : error)
    }
    unsafe fn request(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, request)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKUserContentController(pub id);
impl std::ops::Deref for WKUserContentController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKUserContentController {}
impl WKUserContentController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKUserContentController").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for WKUserContentController {}
impl INSObject for WKUserContentController {}
impl PNSObject for WKUserContentController {}
impl std::convert::TryFrom<NSObject> for WKUserContentController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKUserContentController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKUserContentController").unwrap()) };
        if is_kind_of {
            Ok(WKUserContentController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKUserContentController")
        }
    }
}
impl IWKUserContentController for WKUserContentController {}
pub trait IWKUserContentController: Sized + std::ops::Deref {
    unsafe fn addUserScript_(&self, userScript: WKUserScript)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addUserScript : userScript)
    }
    unsafe fn removeAllUserScripts(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllUserScripts)
    }
    unsafe fn addScriptMessageHandler_contentWorld_name_(
        &self,
        scriptMessageHandler: *mut u64,
        world: WKContentWorld,
        name: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addScriptMessageHandler : scriptMessageHandler, contentWorld : world, name : name)
    }
    unsafe fn addScriptMessageHandlerWithReply_contentWorld_name_(
        &self,
        scriptMessageHandlerWithReply: *mut u64,
        contentWorld: WKContentWorld,
        name: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addScriptMessageHandlerWithReply : scriptMessageHandlerWithReply, contentWorld : contentWorld, name : name)
    }
    unsafe fn addScriptMessageHandler_name_(&self, scriptMessageHandler: *mut u64, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addScriptMessageHandler : scriptMessageHandler, name : name)
    }
    unsafe fn removeScriptMessageHandlerForName_contentWorld_(
        &self,
        name: NSString,
        contentWorld: WKContentWorld,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeScriptMessageHandlerForName : name, contentWorld : contentWorld)
    }
    unsafe fn removeScriptMessageHandlerForName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeScriptMessageHandlerForName : name)
    }
    unsafe fn removeAllScriptMessageHandlersFromContentWorld_(&self, contentWorld: WKContentWorld)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAllScriptMessageHandlersFromContentWorld : contentWorld)
    }
    unsafe fn removeAllScriptMessageHandlers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllScriptMessageHandlers)
    }
    unsafe fn addContentRuleList_(&self, contentRuleList: WKContentRuleList)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addContentRuleList : contentRuleList)
    }
    unsafe fn removeContentRuleList_(&self, contentRuleList: WKContentRuleList)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeContentRuleList : contentRuleList)
    }
    unsafe fn removeAllContentRuleLists(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllContentRuleLists)
    }
    unsafe fn userScripts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userScripts)
    }
}
pub type WKUserScriptInjectionTime = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKUserScript(pub id);
impl std::ops::Deref for WKUserScript {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKUserScript {}
impl WKUserScript {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKUserScript").unwrap(), alloc) })
    }
}
impl PNSCopying for WKUserScript {}
impl INSObject for WKUserScript {}
impl PNSObject for WKUserScript {}
impl std::convert::TryFrom<NSObject> for WKUserScript {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKUserScript, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKUserScript").unwrap()) };
        if is_kind_of {
            Ok(WKUserScript(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKUserScript")
        }
    }
}
impl IWKUserScript for WKUserScript {}
pub trait IWKUserScript: Sized + std::ops::Deref {
    unsafe fn initWithSource_injectionTime_forMainFrameOnly_(
        &self,
        source: NSString,
        injectionTime: WKUserScriptInjectionTime,
        forMainFrameOnly: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source, injectionTime : injectionTime, forMainFrameOnly : forMainFrameOnly)
    }
    unsafe fn initWithSource_injectionTime_forMainFrameOnly_inContentWorld_(
        &self,
        source: NSString,
        injectionTime: WKUserScriptInjectionTime,
        forMainFrameOnly: BOOL,
        contentWorld: WKContentWorld,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source, injectionTime : injectionTime, forMainFrameOnly : forMainFrameOnly, inContentWorld : contentWorld)
    }
    unsafe fn source(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn injectionTime(&self) -> WKUserScriptInjectionTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, injectionTime)
    }
    unsafe fn isForMainFrameOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isForMainFrameOnly)
    }
}
pub type WKWebExtensionMatchPatternError = NSInteger;
pub type WKWebExtensionMatchPatternOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionMatchPattern(pub id);
impl std::ops::Deref for WKWebExtensionMatchPattern {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionMatchPattern {}
impl WKWebExtensionMatchPattern {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMatchPattern").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for WKWebExtensionMatchPattern {}
impl PNSCopying for WKWebExtensionMatchPattern {}
impl INSObject for WKWebExtensionMatchPattern {}
impl PNSObject for WKWebExtensionMatchPattern {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionMatchPattern {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionMatchPattern, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionMatchPattern").unwrap()) };
        if is_kind_of {
            Ok(WKWebExtensionMatchPattern(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionMatchPattern")
        }
    }
}
impl IWKWebExtensionMatchPattern for WKWebExtensionMatchPattern {}
pub trait IWKWebExtensionMatchPattern: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithString_error_(&self, string: NSString, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithString : string, error : error)
    }
    unsafe fn initWithScheme_host_path_error_(
        &self,
        scheme: NSString,
        host: NSString,
        path: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithScheme : scheme, host : host, path : path, error : error)
    }
    unsafe fn matchesURL_(&self, url: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesURL : url)
    }
    unsafe fn matchesURL_options_(
        &self,
        url: NSURL,
        options: WKWebExtensionMatchPatternOptions,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesURL : url, options : options)
    }
    unsafe fn matchesPattern_(&self, pattern: WKWebExtensionMatchPattern) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesPattern : pattern)
    }
    unsafe fn matchesPattern_options_(
        &self,
        pattern: WKWebExtensionMatchPattern,
        options: WKWebExtensionMatchPatternOptions,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesPattern : pattern, options : options)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn scheme(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheme)
    }
    unsafe fn host(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, host)
    }
    unsafe fn path(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn matchesAllURLs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchesAllURLs)
    }
    unsafe fn matchesAllHosts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchesAllHosts)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMatchPattern").unwrap(), new)
    }
    unsafe fn registerCustomURLScheme_(urlScheme: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMatchPattern").unwrap(), registerCustomURLScheme : urlScheme)
    }
    unsafe fn allURLsMatchPattern() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMatchPattern").unwrap(), allURLsMatchPattern)
    }
    unsafe fn allHostsAndSchemesMatchPattern() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMatchPattern").unwrap(), allHostsAndSchemesMatchPattern)
    }
    unsafe fn matchPatternWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMatchPattern").unwrap(), matchPatternWithString : string)
    }
    unsafe fn matchPatternWithScheme_host_path_(
        scheme: NSString,
        host: NSString,
        path: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMatchPattern").unwrap(), matchPatternWithScheme : scheme, host : host, path : path)
    }
}
pub type WKWebExtensionPermission = NSString;
pub type WKWebExtensionError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtension(pub id);
impl std::ops::Deref for WKWebExtension {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtension {}
impl WKWebExtension {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtension").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtension {}
impl PNSObject for WKWebExtension {}
impl std::convert::TryFrom<NSObject> for WKWebExtension {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtension, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtension").unwrap()) };
        if is_kind_of {
            Ok(WKWebExtension(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtension")
        }
    }
}
impl IWKWebExtension for WKWebExtension {}
pub trait IWKWebExtension: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn supportsManifestVersion_(&self, manifestVersion: f64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsManifestVersion : manifestVersion)
    }
    unsafe fn iconForSize_(&self, size: CGSize) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, iconForSize : size)
    }
    unsafe fn actionIconForSize_(&self, size: CGSize) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, actionIconForSize : size)
    }
    unsafe fn errors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errors)
    }
    unsafe fn manifest(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manifest)
    }
    unsafe fn manifestVersion(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manifestVersion)
    }
    unsafe fn defaultLocale(&self) -> NSLocale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultLocale)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn displayShortName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayShortName)
    }
    unsafe fn displayVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayVersion)
    }
    unsafe fn displayDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayDescription)
    }
    unsafe fn displayActionLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayActionLabel)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn requestedPermissions(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedPermissions)
    }
    unsafe fn optionalPermissions(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optionalPermissions)
    }
    unsafe fn requestedPermissionMatchPatterns(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedPermissionMatchPatterns)
    }
    unsafe fn optionalPermissionMatchPatterns(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optionalPermissionMatchPatterns)
    }
    unsafe fn allRequestedMatchPatterns(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allRequestedMatchPatterns)
    }
    unsafe fn hasBackgroundContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasBackgroundContent)
    }
    unsafe fn hasPersistentBackgroundContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasPersistentBackgroundContent)
    }
    unsafe fn hasInjectedContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasInjectedContent)
    }
    unsafe fn hasOptionsPage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasOptionsPage)
    }
    unsafe fn hasOverrideNewTabPage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasOverrideNewTabPage)
    }
    unsafe fn hasCommands(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasCommands)
    }
    unsafe fn hasContentModificationRules(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasContentModificationRules)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtension").unwrap(), new)
    }
    unsafe fn extensionWithAppExtensionBundle_completionHandler_(
        appExtensionBundle: NSBundle,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtension").unwrap(), extensionWithAppExtensionBundle : appExtensionBundle, completionHandler : completionHandler)
    }
    unsafe fn extensionWithResourceBaseURL_completionHandler_(
        resourceBaseURL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtension").unwrap(), extensionWithResourceBaseURL : resourceBaseURL, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionAction(pub id);
impl std::ops::Deref for WKWebExtensionAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionAction {}
impl WKWebExtensionAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionAction").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtensionAction {}
impl PNSObject for WKWebExtensionAction {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionAction").unwrap()) };
        if is_kind_of {
            Ok(WKWebExtensionAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionAction")
        }
    }
}
impl IWKWebExtensionAction for WKWebExtensionAction {}
pub trait IWKWebExtensionAction: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn iconForSize_(&self, size: CGSize) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, iconForSize : size)
    }
    unsafe fn closePopup(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closePopup)
    }
    unsafe fn webExtensionContext(&self) -> WKWebExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webExtensionContext)
    }
    unsafe fn associatedTab(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, associatedTab)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn badgeText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, badgeText)
    }
    unsafe fn hasUnreadBadgeText(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasUnreadBadgeText)
    }
    unsafe fn setHasUnreadBadgeText_(&self, hasUnreadBadgeText: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasUnreadBadgeText : hasUnreadBadgeText)
    }
    unsafe fn inspectionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inspectionName)
    }
    unsafe fn setInspectionName_(&self, inspectionName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInspectionName : inspectionName)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn menuItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuItems)
    }
    unsafe fn presentsPopup(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentsPopup)
    }
    unsafe fn popupPopover(&self) -> NSPopover
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popupPopover)
    }
    unsafe fn popupWebView(&self) -> WKWebView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popupWebView)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionCommand(pub id);
impl std::ops::Deref for WKWebExtensionCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionCommand {}
impl WKWebExtensionCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionCommand").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtensionCommand {}
impl PNSObject for WKWebExtensionCommand {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionCommand {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionCommand").unwrap()) };
        if is_kind_of {
            Ok(WKWebExtensionCommand(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionCommand")
        }
    }
}
impl IWKWebExtensionCommand for WKWebExtensionCommand {}
pub trait IWKWebExtensionCommand: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn webExtensionContext(&self) -> WKWebExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webExtensionContext)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn activationKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activationKey)
    }
    unsafe fn setActivationKey_(&self, activationKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivationKey : activationKey)
    }
    unsafe fn modifierFlags(&self) -> NSEventModifierFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifierFlags)
    }
    unsafe fn setModifierFlags_(&self, modifierFlags: NSEventModifierFlags)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModifierFlags : modifierFlags)
    }
    unsafe fn menuItem(&self) -> NSMenuItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuItem)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionCommand").unwrap(), new)
    }
}
pub type WKWebExtensionTabChangedProperties = NSUInteger;
pub trait PWKWebExtensionTab: Sized + std::ops::Deref {
    unsafe fn windowForWebExtensionContext_(&self, context: WKWebExtensionContext) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, windowForWebExtensionContext : context)
    }
    unsafe fn indexInWindowForWebExtensionContext_(
        &self,
        context: WKWebExtensionContext,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexInWindowForWebExtensionContext : context)
    }
    unsafe fn parentTabForWebExtensionContext_(&self, context: WKWebExtensionContext) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parentTabForWebExtensionContext : context)
    }
    unsafe fn setParentTab_forWebExtensionContext_completionHandler_(
        &self,
        parentTab: *mut u64,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParentTab : parentTab, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn webViewForWebExtensionContext_(&self, context: WKWebExtensionContext) -> WKWebView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webViewForWebExtensionContext : context)
    }
    unsafe fn titleForWebExtensionContext_(&self, context: WKWebExtensionContext) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, titleForWebExtensionContext : context)
    }
    unsafe fn isPinnedForWebExtensionContext_(&self, context: WKWebExtensionContext) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isPinnedForWebExtensionContext : context)
    }
    unsafe fn setPinned_forWebExtensionContext_completionHandler_(
        &self,
        pinned: BOOL,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPinned : pinned, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn isReaderModeAvailableForWebExtensionContext_(
        &self,
        context: WKWebExtensionContext,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isReaderModeAvailableForWebExtensionContext : context)
    }
    unsafe fn isReaderModeActiveForWebExtensionContext_(
        &self,
        context: WKWebExtensionContext,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isReaderModeActiveForWebExtensionContext : context)
    }
    unsafe fn setReaderModeActive_forWebExtensionContext_completionHandler_(
        &self,
        active: BOOL,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReaderModeActive : active, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn isPlayingAudioForWebExtensionContext_(&self, context: WKWebExtensionContext) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isPlayingAudioForWebExtensionContext : context)
    }
    unsafe fn isMutedForWebExtensionContext_(&self, context: WKWebExtensionContext) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isMutedForWebExtensionContext : context)
    }
    unsafe fn setMuted_forWebExtensionContext_completionHandler_(
        &self,
        muted: BOOL,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMuted : muted, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn sizeForWebExtensionContext_(&self, context: WKWebExtensionContext) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sizeForWebExtensionContext : context)
    }
    unsafe fn zoomFactorForWebExtensionContext_(&self, context: WKWebExtensionContext) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, zoomFactorForWebExtensionContext : context)
    }
    unsafe fn setZoomFactor_forWebExtensionContext_completionHandler_(
        &self,
        zoomFactor: f64,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoomFactor : zoomFactor, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn urlForWebExtensionContext_(&self, context: WKWebExtensionContext) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, urlForWebExtensionContext : context)
    }
    unsafe fn pendingURLForWebExtensionContext_(&self, context: WKWebExtensionContext) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pendingURLForWebExtensionContext : context)
    }
    unsafe fn isLoadingCompleteForWebExtensionContext_(
        &self,
        context: WKWebExtensionContext,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isLoadingCompleteForWebExtensionContext : context)
    }
    unsafe fn detectWebpageLocaleForWebExtensionContext_completionHandler_(
        &self,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectWebpageLocaleForWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn takeSnapshotUsingConfiguration_forWebExtensionContext_completionHandler_(
        &self,
        configuration: WKSnapshotConfiguration,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, takeSnapshotUsingConfiguration : configuration, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn loadURL_forWebExtensionContext_completionHandler_(
        &self,
        url: NSURL,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadURL : url, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn reloadFromOrigin_forWebExtensionContext_completionHandler_(
        &self,
        fromOrigin: BOOL,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reloadFromOrigin : fromOrigin, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn goBackForWebExtensionContext_completionHandler_(
        &self,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goBackForWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn goForwardForWebExtensionContext_completionHandler_(
        &self,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goForwardForWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn activateForWebExtensionContext_completionHandler_(
        &self,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateForWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn isSelectedForWebExtensionContext_(&self, context: WKWebExtensionContext) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isSelectedForWebExtensionContext : context)
    }
    unsafe fn setSelected_forWebExtensionContext_completionHandler_(
        &self,
        selected: BOOL,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelected : selected, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn duplicateUsingConfiguration_forWebExtensionContext_completionHandler_(
        &self,
        configuration: WKWebExtensionTabConfiguration,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, duplicateUsingConfiguration : configuration, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn closeForWebExtensionContext_completionHandler_(
        &self,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, closeForWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn shouldGrantPermissionsOnUserGestureForWebExtensionContext_(
        &self,
        context: WKWebExtensionContext,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldGrantPermissionsOnUserGestureForWebExtensionContext : context)
    }
    unsafe fn shouldBypassPermissionsForWebExtensionContext_(
        &self,
        context: WKWebExtensionContext,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldBypassPermissionsForWebExtensionContext : context)
    }
}
pub type WKWebExtensionContextError = NSInteger;
pub type WKWebExtensionContextPermissionStatus = NSInteger;
pub type WKWebExtensionContextNotificationUserInfoKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionContext(pub id);
impl std::ops::Deref for WKWebExtensionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionContext {}
impl WKWebExtensionContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionContext").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtensionContext {}
impl PNSObject for WKWebExtensionContext {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionContext").unwrap()) };
        if is_kind_of {
            Ok(WKWebExtensionContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionContext")
        }
    }
}
impl IWKWebExtensionContext for WKWebExtensionContext {}
pub trait IWKWebExtensionContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initForExtension_(&self, extension: WKWebExtension) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForExtension : extension)
    }
    unsafe fn hasPermission_(&self, permission: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasPermission : permission)
    }
    unsafe fn hasPermission_inTab_(&self, permission: NSString, tab: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasPermission : permission, inTab : tab)
    }
    unsafe fn hasAccessToURL_(&self, url: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasAccessToURL : url)
    }
    unsafe fn hasAccessToURL_inTab_(&self, url: NSURL, tab: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasAccessToURL : url, inTab : tab)
    }
    unsafe fn hasInjectedContentForURL_(&self, url: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasInjectedContentForURL : url)
    }
    unsafe fn permissionStatusForPermission_(
        &self,
        permission: NSString,
    ) -> WKWebExtensionContextPermissionStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, permissionStatusForPermission : permission)
    }
    unsafe fn permissionStatusForPermission_inTab_(
        &self,
        permission: NSString,
        tab: *mut u64,
    ) -> WKWebExtensionContextPermissionStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, permissionStatusForPermission : permission, inTab : tab)
    }
    unsafe fn setPermissionStatus_forPermission_(
        &self,
        status: WKWebExtensionContextPermissionStatus,
        permission: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPermissionStatus : status, forPermission : permission)
    }
    unsafe fn setPermissionStatus_forPermission_expirationDate_(
        &self,
        status: WKWebExtensionContextPermissionStatus,
        permission: NSString,
        expirationDate: NSDate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPermissionStatus : status, forPermission : permission, expirationDate : expirationDate)
    }
    unsafe fn permissionStatusForURL_(&self, url: NSURL) -> WKWebExtensionContextPermissionStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, permissionStatusForURL : url)
    }
    unsafe fn permissionStatusForURL_inTab_(
        &self,
        url: NSURL,
        tab: *mut u64,
    ) -> WKWebExtensionContextPermissionStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, permissionStatusForURL : url, inTab : tab)
    }
    unsafe fn setPermissionStatus_forURL_(
        &self,
        status: WKWebExtensionContextPermissionStatus,
        url: NSURL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPermissionStatus : status, forURL : url)
    }
    unsafe fn setPermissionStatus_forURL_expirationDate_(
        &self,
        status: WKWebExtensionContextPermissionStatus,
        url: NSURL,
        expirationDate: NSDate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPermissionStatus : status, forURL : url, expirationDate : expirationDate)
    }
    unsafe fn permissionStatusForMatchPattern_(
        &self,
        pattern: WKWebExtensionMatchPattern,
    ) -> WKWebExtensionContextPermissionStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, permissionStatusForMatchPattern : pattern)
    }
    unsafe fn permissionStatusForMatchPattern_inTab_(
        &self,
        pattern: WKWebExtensionMatchPattern,
        tab: *mut u64,
    ) -> WKWebExtensionContextPermissionStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, permissionStatusForMatchPattern : pattern, inTab : tab)
    }
    unsafe fn setPermissionStatus_forMatchPattern_(
        &self,
        status: WKWebExtensionContextPermissionStatus,
        pattern: WKWebExtensionMatchPattern,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPermissionStatus : status, forMatchPattern : pattern)
    }
    unsafe fn setPermissionStatus_forMatchPattern_expirationDate_(
        &self,
        status: WKWebExtensionContextPermissionStatus,
        pattern: WKWebExtensionMatchPattern,
        expirationDate: NSDate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPermissionStatus : status, forMatchPattern : pattern, expirationDate : expirationDate)
    }
    unsafe fn loadBackgroundContentWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadBackgroundContentWithCompletionHandler : completionHandler)
    }
    unsafe fn actionForTab_(&self, tab: *mut u64) -> WKWebExtensionAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, actionForTab : tab)
    }
    unsafe fn performActionForTab_(&self, tab: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performActionForTab : tab)
    }
    unsafe fn performCommand_(&self, command: WKWebExtensionCommand)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performCommand : command)
    }
    unsafe fn performCommandForEvent_(&self, event: NSEvent) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performCommandForEvent : event)
    }
    unsafe fn commandForEvent_(&self, event: NSEvent) -> WKWebExtensionCommand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commandForEvent : event)
    }
    unsafe fn menuItemsForTab_(&self, tab: *mut u64) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, menuItemsForTab : tab)
    }
    unsafe fn userGesturePerformedInTab_(&self, tab: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userGesturePerformedInTab : tab)
    }
    unsafe fn hasActiveUserGestureInTab_(&self, tab: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasActiveUserGestureInTab : tab)
    }
    unsafe fn clearUserGestureInTab_(&self, tab: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clearUserGestureInTab : tab)
    }
    unsafe fn didOpenWindow_(&self, newWindow: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didOpenWindow : newWindow)
    }
    unsafe fn didCloseWindow_(&self, closedWindow: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didCloseWindow : closedWindow)
    }
    unsafe fn didFocusWindow_(&self, focusedWindow: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didFocusWindow : focusedWindow)
    }
    unsafe fn didOpenTab_(&self, newTab: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didOpenTab : newTab)
    }
    unsafe fn didCloseTab_windowIsClosing_(&self, closedTab: *mut u64, windowIsClosing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didCloseTab : closedTab, windowIsClosing : windowIsClosing)
    }
    unsafe fn didActivateTab_previousActiveTab_(
        &self,
        activatedTab: *mut u64,
        previousTab: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didActivateTab : activatedTab, previousActiveTab : previousTab)
    }
    unsafe fn didSelectTabs_(&self, selectedTabs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didSelectTabs : selectedTabs)
    }
    unsafe fn didDeselectTabs_(&self, deselectedTabs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didDeselectTabs : deselectedTabs)
    }
    unsafe fn didMoveTab_fromIndex_inWindow_(
        &self,
        movedTab: *mut u64,
        index: NSUInteger,
        oldWindow: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didMoveTab : movedTab, fromIndex : index, inWindow : oldWindow)
    }
    unsafe fn didReplaceTab_withTab_(&self, oldTab: *mut u64, newTab: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReplaceTab : oldTab, withTab : newTab)
    }
    unsafe fn didChangeTabProperties_forTab_(
        &self,
        properties: WKWebExtensionTabChangedProperties,
        changedTab: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didChangeTabProperties : properties, forTab : changedTab)
    }
    unsafe fn webExtension(&self) -> WKWebExtension
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webExtension)
    }
    unsafe fn webExtensionController(&self) -> WKWebExtensionController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webExtensionController)
    }
    unsafe fn isLoaded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoaded)
    }
    unsafe fn errors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errors)
    }
    unsafe fn baseURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseURL)
    }
    unsafe fn setBaseURL_(&self, baseURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaseURL : baseURL)
    }
    unsafe fn uniqueIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn setUniqueIdentifier_(&self, uniqueIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUniqueIdentifier : uniqueIdentifier)
    }
    unsafe fn isInspectable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInspectable)
    }
    unsafe fn setInspectable_(&self, inspectable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInspectable : inspectable)
    }
    unsafe fn inspectionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inspectionName)
    }
    unsafe fn setInspectionName_(&self, inspectionName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInspectionName : inspectionName)
    }
    unsafe fn unsupportedAPIs(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unsupportedAPIs)
    }
    unsafe fn setUnsupportedAPIs_(&self, unsupportedAPIs: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnsupportedAPIs : unsupportedAPIs)
    }
    unsafe fn webViewConfiguration(&self) -> WKWebViewConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webViewConfiguration)
    }
    unsafe fn optionsPageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optionsPageURL)
    }
    unsafe fn overrideNewTabPageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overrideNewTabPageURL)
    }
    unsafe fn grantedPermissions(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grantedPermissions)
    }
    unsafe fn setGrantedPermissions_(&self, grantedPermissions: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrantedPermissions : grantedPermissions)
    }
    unsafe fn grantedPermissionMatchPatterns(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grantedPermissionMatchPatterns)
    }
    unsafe fn setGrantedPermissionMatchPatterns_(
        &self,
        grantedPermissionMatchPatterns: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrantedPermissionMatchPatterns : grantedPermissionMatchPatterns)
    }
    unsafe fn deniedPermissions(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deniedPermissions)
    }
    unsafe fn setDeniedPermissions_(&self, deniedPermissions: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeniedPermissions : deniedPermissions)
    }
    unsafe fn deniedPermissionMatchPatterns(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deniedPermissionMatchPatterns)
    }
    unsafe fn setDeniedPermissionMatchPatterns_(&self, deniedPermissionMatchPatterns: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeniedPermissionMatchPatterns : deniedPermissionMatchPatterns)
    }
    unsafe fn hasRequestedOptionalAccessToAllHosts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasRequestedOptionalAccessToAllHosts)
    }
    unsafe fn setHasRequestedOptionalAccessToAllHosts_(
        &self,
        hasRequestedOptionalAccessToAllHosts: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasRequestedOptionalAccessToAllHosts : hasRequestedOptionalAccessToAllHosts)
    }
    unsafe fn hasAccessToPrivateData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAccessToPrivateData)
    }
    unsafe fn setHasAccessToPrivateData_(&self, hasAccessToPrivateData: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasAccessToPrivateData : hasAccessToPrivateData)
    }
    unsafe fn currentPermissions(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPermissions)
    }
    unsafe fn currentPermissionMatchPatterns(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPermissionMatchPatterns)
    }
    unsafe fn hasAccessToAllURLs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAccessToAllURLs)
    }
    unsafe fn hasAccessToAllHosts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAccessToAllHosts)
    }
    unsafe fn hasInjectedContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasInjectedContent)
    }
    unsafe fn hasContentModificationRules(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasContentModificationRules)
    }
    unsafe fn commands(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commands)
    }
    unsafe fn openWindows(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openWindows)
    }
    unsafe fn focusedWindow(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusedWindow)
    }
    unsafe fn openTabs(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openTabs)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionContext").unwrap(), new)
    }
    unsafe fn contextForExtension_(extension: WKWebExtension) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionContext").unwrap(), contextForExtension : extension)
    }
}
pub trait PWKWebExtensionControllerDelegate: Sized + std::ops::Deref {
    unsafe fn webExtensionController_openWindowsForExtensionContext_(
        &self,
        controller: WKWebExtensionController,
        extensionContext: WKWebExtensionContext,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, openWindowsForExtensionContext : extensionContext)
    }
    unsafe fn webExtensionController_focusedWindowForExtensionContext_(
        &self,
        controller: WKWebExtensionController,
        extensionContext: WKWebExtensionContext,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, focusedWindowForExtensionContext : extensionContext)
    }
    unsafe fn webExtensionController_openNewWindowUsingConfiguration_forExtensionContext_completionHandler_(
        &self,
        controller: WKWebExtensionController,
        configuration: WKWebExtensionWindowConfiguration,
        extensionContext: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, openNewWindowUsingConfiguration : configuration, forExtensionContext : extensionContext, completionHandler : completionHandler)
    }
    unsafe fn webExtensionController_openNewTabUsingConfiguration_forExtensionContext_completionHandler_(
        &self,
        controller: WKWebExtensionController,
        configuration: WKWebExtensionTabConfiguration,
        extensionContext: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, openNewTabUsingConfiguration : configuration, forExtensionContext : extensionContext, completionHandler : completionHandler)
    }
    unsafe fn webExtensionController_openOptionsPageForExtensionContext_completionHandler_(
        &self,
        controller: WKWebExtensionController,
        extensionContext: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, openOptionsPageForExtensionContext : extensionContext, completionHandler : completionHandler)
    }
    unsafe fn webExtensionController_promptForPermissions_inTab_forExtensionContext_completionHandler_(
        &self,
        controller: WKWebExtensionController,
        permissions: NSSet,
        tab: *mut u64,
        extensionContext: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, promptForPermissions : permissions, inTab : tab, forExtensionContext : extensionContext, completionHandler : completionHandler)
    }
    unsafe fn webExtensionController_promptForPermissionToAccessURLs_inTab_forExtensionContext_completionHandler_(
        &self,
        controller: WKWebExtensionController,
        urls: NSSet,
        tab: *mut u64,
        extensionContext: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, promptForPermissionToAccessURLs : urls, inTab : tab, forExtensionContext : extensionContext, completionHandler : completionHandler)
    }
    unsafe fn webExtensionController_promptForPermissionMatchPatterns_inTab_forExtensionContext_completionHandler_(
        &self,
        controller: WKWebExtensionController,
        matchPatterns: NSSet,
        tab: *mut u64,
        extensionContext: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, promptForPermissionMatchPatterns : matchPatterns, inTab : tab, forExtensionContext : extensionContext, completionHandler : completionHandler)
    }
    unsafe fn webExtensionController_didUpdateAction_forExtensionContext_(
        &self,
        controller: WKWebExtensionController,
        action: WKWebExtensionAction,
        context: WKWebExtensionContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, didUpdateAction : action, forExtensionContext : context)
    }
    unsafe fn webExtensionController_presentPopupForAction_forExtensionContext_completionHandler_(
        &self,
        controller: WKWebExtensionController,
        action: WKWebExtensionAction,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, presentPopupForAction : action, forExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn webExtensionController_sendMessage_toApplicationWithIdentifier_forExtensionContext_replyHandler_(
        &self,
        controller: WKWebExtensionController,
        message: id,
        applicationIdentifier: NSString,
        extensionContext: WKWebExtensionContext,
        replyHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, sendMessage : message, toApplicationWithIdentifier : applicationIdentifier, forExtensionContext : extensionContext, replyHandler : replyHandler)
    }
    unsafe fn webExtensionController_connectUsingMessagePort_forExtensionContext_completionHandler_(
        &self,
        controller: WKWebExtensionController,
        port: WKWebExtensionMessagePort,
        extensionContext: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webExtensionController : controller, connectUsingMessagePort : port, forExtensionContext : extensionContext, completionHandler : completionHandler)
    }
}
pub type WKWebExtensionDataType = NSString;
pub type WKWebExtensionWindowType = NSInteger;
pub type WKWebExtensionWindowState = NSInteger;
pub trait PWKWebExtensionWindow: Sized + std::ops::Deref {
    unsafe fn tabsForWebExtensionContext_(&self, context: WKWebExtensionContext) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tabsForWebExtensionContext : context)
    }
    unsafe fn activeTabForWebExtensionContext_(&self, context: WKWebExtensionContext) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activeTabForWebExtensionContext : context)
    }
    unsafe fn windowTypeForWebExtensionContext_(
        &self,
        context: WKWebExtensionContext,
    ) -> WKWebExtensionWindowType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, windowTypeForWebExtensionContext : context)
    }
    unsafe fn windowStateForWebExtensionContext_(
        &self,
        context: WKWebExtensionContext,
    ) -> WKWebExtensionWindowState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, windowStateForWebExtensionContext : context)
    }
    unsafe fn setWindowState_forWebExtensionContext_completionHandler_(
        &self,
        state: WKWebExtensionWindowState,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWindowState : state, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn isPrivateForWebExtensionContext_(&self, context: WKWebExtensionContext) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isPrivateForWebExtensionContext : context)
    }
    unsafe fn screenFrameForWebExtensionContext_(&self, context: WKWebExtensionContext) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, screenFrameForWebExtensionContext : context)
    }
    unsafe fn frameForWebExtensionContext_(&self, context: WKWebExtensionContext) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, frameForWebExtensionContext : context)
    }
    unsafe fn setFrame_forWebExtensionContext_completionHandler_(
        &self,
        frame: CGRect,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrame : frame, forWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn focusForWebExtensionContext_completionHandler_(
        &self,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, focusForWebExtensionContext : context, completionHandler : completionHandler)
    }
    unsafe fn closeForWebExtensionContext_completionHandler_(
        &self,
        context: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, closeForWebExtensionContext : context, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionController(pub id);
impl std::ops::Deref for WKWebExtensionController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionController {}
impl WKWebExtensionController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionController").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtensionController {}
impl PNSObject for WKWebExtensionController {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionController").unwrap()) };
        if is_kind_of {
            Ok(WKWebExtensionController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionController")
        }
    }
}
impl IWKWebExtensionController for WKWebExtensionController {}
pub trait IWKWebExtensionController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithConfiguration_(
        &self,
        configuration: WKWebExtensionControllerConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
    unsafe fn loadExtensionContext_error_(
        &self,
        extensionContext: WKWebExtensionContext,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadExtensionContext : extensionContext, error : error)
    }
    unsafe fn unloadExtensionContext_error_(
        &self,
        extensionContext: WKWebExtensionContext,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unloadExtensionContext : extensionContext, error : error)
    }
    unsafe fn extensionContextForExtension_(
        &self,
        extension: WKWebExtension,
    ) -> WKWebExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extensionContextForExtension : extension)
    }
    unsafe fn extensionContextForURL_(&self, URL: NSURL) -> WKWebExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extensionContextForURL : URL)
    }
    unsafe fn fetchDataRecordsOfTypes_completionHandler_(
        &self,
        dataTypes: NSSet,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchDataRecordsOfTypes : dataTypes, completionHandler : completionHandler)
    }
    unsafe fn fetchDataRecordOfTypes_forExtensionContext_completionHandler_(
        &self,
        dataTypes: NSSet,
        extensionContext: WKWebExtensionContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchDataRecordOfTypes : dataTypes, forExtensionContext : extensionContext, completionHandler : completionHandler)
    }
    unsafe fn removeDataOfTypes_fromDataRecords_completionHandler_(
        &self,
        dataTypes: NSSet,
        dataRecords: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeDataOfTypes : dataTypes, fromDataRecords : dataRecords, completionHandler : completionHandler)
    }
    unsafe fn didOpenWindow_(&self, newWindow: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didOpenWindow : newWindow)
    }
    unsafe fn didCloseWindow_(&self, closedWindow: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didCloseWindow : closedWindow)
    }
    unsafe fn didFocusWindow_(&self, focusedWindow: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didFocusWindow : focusedWindow)
    }
    unsafe fn didOpenTab_(&self, newTab: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didOpenTab : newTab)
    }
    unsafe fn didCloseTab_windowIsClosing_(&self, closedTab: *mut u64, windowIsClosing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didCloseTab : closedTab, windowIsClosing : windowIsClosing)
    }
    unsafe fn didActivateTab_previousActiveTab_(
        &self,
        activatedTab: *mut u64,
        previousTab: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didActivateTab : activatedTab, previousActiveTab : previousTab)
    }
    unsafe fn didSelectTabs_(&self, selectedTabs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didSelectTabs : selectedTabs)
    }
    unsafe fn didDeselectTabs_(&self, deselectedTabs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didDeselectTabs : deselectedTabs)
    }
    unsafe fn didMoveTab_fromIndex_inWindow_(
        &self,
        movedTab: *mut u64,
        index: NSUInteger,
        oldWindow: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didMoveTab : movedTab, fromIndex : index, inWindow : oldWindow)
    }
    unsafe fn didReplaceTab_withTab_(&self, oldTab: *mut u64, newTab: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReplaceTab : oldTab, withTab : newTab)
    }
    unsafe fn didChangeTabProperties_forTab_(
        &self,
        properties: WKWebExtensionTabChangedProperties,
        changedTab: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didChangeTabProperties : properties, forTab : changedTab)
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
    unsafe fn configuration(&self) -> WKWebExtensionControllerConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn extensions(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensions)
    }
    unsafe fn extensionContexts(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionContexts)
    }
    unsafe fn allExtensionDataTypes() -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionController").unwrap(), allExtensionDataTypes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionControllerConfiguration(pub id);
impl std::ops::Deref for WKWebExtensionControllerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionControllerConfiguration {}
impl WKWebExtensionControllerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionControllerConfiguration").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for WKWebExtensionControllerConfiguration {}
impl PNSCopying for WKWebExtensionControllerConfiguration {}
impl INSObject for WKWebExtensionControllerConfiguration {}
impl PNSObject for WKWebExtensionControllerConfiguration {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionControllerConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionControllerConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionControllerConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(WKWebExtensionControllerConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionControllerConfiguration")
        }
    }
}
impl IWKWebExtensionControllerConfiguration for WKWebExtensionControllerConfiguration {}
pub trait IWKWebExtensionControllerConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isPersistent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPersistent)
    }
    unsafe fn identifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn webViewConfiguration(&self) -> WKWebViewConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webViewConfiguration)
    }
    unsafe fn setWebViewConfiguration_(&self, webViewConfiguration: WKWebViewConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWebViewConfiguration : webViewConfiguration)
    }
    unsafe fn defaultWebsiteDataStore(&self) -> WKWebsiteDataStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultWebsiteDataStore)
    }
    unsafe fn setDefaultWebsiteDataStore_(&self, defaultWebsiteDataStore: WKWebsiteDataStore)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultWebsiteDataStore : defaultWebsiteDataStore)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionControllerConfiguration").unwrap(), new)
    }
    unsafe fn defaultConfiguration() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionControllerConfiguration").unwrap(), defaultConfiguration)
    }
    unsafe fn nonPersistentConfiguration() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionControllerConfiguration").unwrap(), nonPersistentConfiguration)
    }
    unsafe fn configurationWithIdentifier_(identifier: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionControllerConfiguration").unwrap(), configurationWithIdentifier : identifier)
    }
}
pub type WKWebExtensionDataRecordError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionDataRecord(pub id);
impl std::ops::Deref for WKWebExtensionDataRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionDataRecord {}
impl WKWebExtensionDataRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionDataRecord").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtensionDataRecord {}
impl PNSObject for WKWebExtensionDataRecord {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionDataRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionDataRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionDataRecord").unwrap()) };
        if is_kind_of {
            Ok(WKWebExtensionDataRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionDataRecord")
        }
    }
}
impl IWKWebExtensionDataRecord for WKWebExtensionDataRecord {}
pub trait IWKWebExtensionDataRecord: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sizeInBytesOfTypes_(&self, dataTypes: NSSet) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sizeInBytesOfTypes : dataTypes)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn uniqueIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn containedDataTypes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containedDataTypes)
    }
    unsafe fn errors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errors)
    }
    unsafe fn totalSizeInBytes(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalSizeInBytes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionDataRecord").unwrap(), new)
    }
}
pub type WKWebExtensionMessagePortError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionMessagePort(pub id);
impl std::ops::Deref for WKWebExtensionMessagePort {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionMessagePort {}
impl WKWebExtensionMessagePort {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMessagePort").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtensionMessagePort {}
impl PNSObject for WKWebExtensionMessagePort {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionMessagePort {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionMessagePort, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionMessagePort").unwrap()) };
        if is_kind_of {
            Ok(WKWebExtensionMessagePort(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionMessagePort")
        }
    }
}
impl IWKWebExtensionMessagePort for WKWebExtensionMessagePort {}
pub trait IWKWebExtensionMessagePort: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sendMessage_completionHandler_(
        &self,
        message: id,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMessage : message, completionHandler : completionHandler)
    }
    unsafe fn disconnect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnect)
    }
    unsafe fn disconnectWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectWithError : error)
    }
    unsafe fn applicationIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationIdentifier)
    }
    unsafe fn messageHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageHandler)
    }
    unsafe fn setMessageHandler_(&self, messageHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessageHandler : messageHandler)
    }
    unsafe fn disconnectHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnectHandler)
    }
    unsafe fn setDisconnectHandler_(&self, disconnectHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisconnectHandler : disconnectHandler)
    }
    unsafe fn isDisconnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDisconnected)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionMessagePort").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionTabConfiguration(pub id);
impl std::ops::Deref for WKWebExtensionTabConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionTabConfiguration {}
impl WKWebExtensionTabConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionTabConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtensionTabConfiguration {}
impl PNSObject for WKWebExtensionTabConfiguration {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionTabConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionTabConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionTabConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(WKWebExtensionTabConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionTabConfiguration")
        }
    }
}
impl IWKWebExtensionTabConfiguration for WKWebExtensionTabConfiguration {}
pub trait IWKWebExtensionTabConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn window(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, window)
    }
    unsafe fn index(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, index)
    }
    unsafe fn parentTab(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentTab)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn shouldBeActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBeActive)
    }
    unsafe fn shouldAddToSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldAddToSelection)
    }
    unsafe fn shouldBePinned(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBePinned)
    }
    unsafe fn shouldBeMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBeMuted)
    }
    unsafe fn shouldReaderModeBeActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldReaderModeBeActive)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionTabConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebExtensionWindowConfiguration(pub id);
impl std::ops::Deref for WKWebExtensionWindowConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebExtensionWindowConfiguration {}
impl WKWebExtensionWindowConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionWindowConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for WKWebExtensionWindowConfiguration {}
impl PNSObject for WKWebExtensionWindowConfiguration {}
impl std::convert::TryFrom<NSObject> for WKWebExtensionWindowConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebExtensionWindowConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebExtensionWindowConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(WKWebExtensionWindowConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebExtensionWindowConfiguration")
        }
    }
}
impl IWKWebExtensionWindowConfiguration for WKWebExtensionWindowConfiguration {}
pub trait IWKWebExtensionWindowConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn windowType(&self) -> WKWebExtensionWindowType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowType)
    }
    unsafe fn windowState(&self) -> WKWebExtensionWindowState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowState)
    }
    unsafe fn frame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frame)
    }
    unsafe fn tabURLs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabURLs)
    }
    unsafe fn tabs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabs)
    }
    unsafe fn shouldBeFocused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBeFocused)
    }
    unsafe fn shouldBePrivate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBePrivate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebExtensionWindowConfiguration").unwrap(), new)
    }
}
pub type WKUserInterfaceDirectionPolicy = NSInteger;
pub type WKAudiovisualMediaTypes = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebViewConfiguration(pub id);
impl std::ops::Deref for WKWebViewConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebViewConfiguration {}
impl WKWebViewConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebViewConfiguration").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for WKWebViewConfiguration {}
impl PNSCopying for WKWebViewConfiguration {}
impl INSObject for WKWebViewConfiguration {}
impl PNSObject for WKWebViewConfiguration {}
impl std::convert::TryFrom<NSObject> for WKWebViewConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebViewConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebViewConfiguration").unwrap()) };
        if is_kind_of {
            Ok(WKWebViewConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebViewConfiguration")
        }
    }
}
impl IWKWebViewConfiguration for WKWebViewConfiguration {}
pub trait IWKWebViewConfiguration: Sized + std::ops::Deref {
    unsafe fn setURLSchemeHandler_forURLScheme_(
        &self,
        urlSchemeHandler: *mut u64,
        urlScheme: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURLSchemeHandler : urlSchemeHandler, forURLScheme : urlScheme)
    }
    unsafe fn urlSchemeHandlerForURLScheme_(&self, urlScheme: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, urlSchemeHandlerForURLScheme : urlScheme)
    }
    unsafe fn processPool(&self) -> WKProcessPool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processPool)
    }
    unsafe fn setProcessPool_(&self, processPool: WKProcessPool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProcessPool : processPool)
    }
    unsafe fn preferences(&self) -> WKPreferences
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferences)
    }
    unsafe fn setPreferences_(&self, preferences: WKPreferences)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferences : preferences)
    }
    unsafe fn userContentController(&self) -> WKUserContentController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userContentController)
    }
    unsafe fn setUserContentController_(&self, userContentController: WKUserContentController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserContentController : userContentController)
    }
    unsafe fn webExtensionController(&self) -> WKWebExtensionController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webExtensionController)
    }
    unsafe fn setWebExtensionController_(&self, webExtensionController: WKWebExtensionController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWebExtensionController : webExtensionController)
    }
    unsafe fn websiteDataStore(&self) -> WKWebsiteDataStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, websiteDataStore)
    }
    unsafe fn setWebsiteDataStore_(&self, websiteDataStore: WKWebsiteDataStore)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWebsiteDataStore : websiteDataStore)
    }
    unsafe fn suppressesIncrementalRendering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suppressesIncrementalRendering)
    }
    unsafe fn setSuppressesIncrementalRendering_(&self, suppressesIncrementalRendering: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuppressesIncrementalRendering : suppressesIncrementalRendering)
    }
    unsafe fn applicationNameForUserAgent(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationNameForUserAgent)
    }
    unsafe fn setApplicationNameForUserAgent_(&self, applicationNameForUserAgent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApplicationNameForUserAgent : applicationNameForUserAgent)
    }
    unsafe fn allowsAirPlayForMediaPlayback(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAirPlayForMediaPlayback)
    }
    unsafe fn setAllowsAirPlayForMediaPlayback_(&self, allowsAirPlayForMediaPlayback: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAirPlayForMediaPlayback : allowsAirPlayForMediaPlayback)
    }
    unsafe fn showsSystemScreenTimeBlockingView(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsSystemScreenTimeBlockingView)
    }
    unsafe fn setShowsSystemScreenTimeBlockingView_(&self, showsSystemScreenTimeBlockingView: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsSystemScreenTimeBlockingView : showsSystemScreenTimeBlockingView)
    }
    unsafe fn upgradeKnownHostsToHTTPS(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upgradeKnownHostsToHTTPS)
    }
    unsafe fn setUpgradeKnownHostsToHTTPS_(&self, upgradeKnownHostsToHTTPS: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpgradeKnownHostsToHTTPS : upgradeKnownHostsToHTTPS)
    }
    unsafe fn mediaTypesRequiringUserActionForPlayback(&self) -> WKAudiovisualMediaTypes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaTypesRequiringUserActionForPlayback)
    }
    unsafe fn setMediaTypesRequiringUserActionForPlayback_(
        &self,
        mediaTypesRequiringUserActionForPlayback: WKAudiovisualMediaTypes,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaTypesRequiringUserActionForPlayback : mediaTypesRequiringUserActionForPlayback)
    }
    unsafe fn defaultWebpagePreferences(&self) -> WKWebpagePreferences
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultWebpagePreferences)
    }
    unsafe fn setDefaultWebpagePreferences_(&self, defaultWebpagePreferences: WKWebpagePreferences)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultWebpagePreferences : defaultWebpagePreferences)
    }
    unsafe fn limitsNavigationsToAppBoundDomains(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, limitsNavigationsToAppBoundDomains)
    }
    unsafe fn setLimitsNavigationsToAppBoundDomains_(
        &self,
        limitsNavigationsToAppBoundDomains: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLimitsNavigationsToAppBoundDomains : limitsNavigationsToAppBoundDomains)
    }
    unsafe fn allowsInlinePredictions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsInlinePredictions)
    }
    unsafe fn setAllowsInlinePredictions_(&self, allowsInlinePredictions: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsInlinePredictions : allowsInlinePredictions)
    }
    unsafe fn userInterfaceDirectionPolicy(&self) -> WKUserInterfaceDirectionPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInterfaceDirectionPolicy)
    }
    unsafe fn setUserInterfaceDirectionPolicy_(
        &self,
        userInterfaceDirectionPolicy: WKUserInterfaceDirectionPolicy,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInterfaceDirectionPolicy : userInterfaceDirectionPolicy)
    }
    unsafe fn supportsAdaptiveImageGlyph(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsAdaptiveImageGlyph)
    }
    unsafe fn setSupportsAdaptiveImageGlyph_(&self, supportsAdaptiveImageGlyph: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsAdaptiveImageGlyph : supportsAdaptiveImageGlyph)
    }
    unsafe fn writingToolsBehavior(&self) -> NSWritingToolsBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writingToolsBehavior)
    }
    unsafe fn setWritingToolsBehavior_(&self, writingToolsBehavior: NSWritingToolsBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWritingToolsBehavior : writingToolsBehavior)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebsiteDataRecord(pub id);
impl std::ops::Deref for WKWebsiteDataRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebsiteDataRecord {}
impl WKWebsiteDataRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebsiteDataRecord").unwrap(), alloc) })
    }
}
impl INSObject for WKWebsiteDataRecord {}
impl PNSObject for WKWebsiteDataRecord {}
impl std::convert::TryFrom<NSObject> for WKWebsiteDataRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebsiteDataRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebsiteDataRecord").unwrap()) };
        if is_kind_of {
            Ok(WKWebsiteDataRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebsiteDataRecord")
        }
    }
}
impl IWKWebsiteDataRecord for WKWebsiteDataRecord {}
pub trait IWKWebsiteDataRecord: Sized + std::ops::Deref {
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn dataTypes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataTypes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWebsiteDataStore(pub id);
impl std::ops::Deref for WKWebsiteDataStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWebsiteDataStore {}
impl WKWebsiteDataStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebsiteDataStore").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for WKWebsiteDataStore {}
impl INSObject for WKWebsiteDataStore {}
impl PNSObject for WKWebsiteDataStore {}
impl std::convert::TryFrom<NSObject> for WKWebsiteDataStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWebsiteDataStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWebsiteDataStore").unwrap()) };
        if is_kind_of {
            Ok(WKWebsiteDataStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWebsiteDataStore")
        }
    }
}
impl IWKWebsiteDataStore for WKWebsiteDataStore {}
pub trait IWKWebsiteDataStore: Sized + std::ops::Deref {
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fetchDataRecordsOfTypes_completionHandler_(
        &self,
        dataTypes: NSSet,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchDataRecordsOfTypes : dataTypes, completionHandler : completionHandler)
    }
    unsafe fn removeDataOfTypes_forDataRecords_completionHandler_(
        &self,
        dataTypes: NSSet,
        dataRecords: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeDataOfTypes : dataTypes, forDataRecords : dataRecords, completionHandler : completionHandler)
    }
    unsafe fn removeDataOfTypes_modifiedSince_completionHandler_(
        &self,
        dataTypes: NSSet,
        date: NSDate,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeDataOfTypes : dataTypes, modifiedSince : date, completionHandler : completionHandler)
    }
    unsafe fn fetchDataOfTypes_completionHandler_(
        &self,
        dataTypes: NSSet,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchDataOfTypes : dataTypes, completionHandler : completionHandler)
    }
    unsafe fn restoreData_completionHandler_(
        &self,
        data: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreData : data, completionHandler : completionHandler)
    }
    unsafe fn isPersistent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPersistent)
    }
    unsafe fn httpCookieStore(&self) -> WKHTTPCookieStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, httpCookieStore)
    }
    unsafe fn identifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn proxyConfigurations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proxyConfigurations)
    }
    unsafe fn setProxyConfigurations_(&self, proxyConfigurations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProxyConfigurations : proxyConfigurations)
    }
    unsafe fn defaultDataStore() -> WKWebsiteDataStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebsiteDataStore").unwrap(), defaultDataStore)
    }
    unsafe fn nonPersistentDataStore() -> WKWebsiteDataStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebsiteDataStore").unwrap(), nonPersistentDataStore)
    }
    unsafe fn allWebsiteDataTypes() -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebsiteDataStore").unwrap(), allWebsiteDataTypes)
    }
    unsafe fn dataStoreForIdentifier_(identifier: NSUUID) -> WKWebsiteDataStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebsiteDataStore").unwrap(), dataStoreForIdentifier : identifier)
    }
    unsafe fn removeDataStoreForIdentifier_completionHandler_(
        identifier: NSUUID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebsiteDataStore").unwrap(), removeDataStoreForIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn fetchAllDataStoreIdentifiers_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WKWebsiteDataStore").unwrap(), fetchAllDataStoreIdentifiers : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WKWindowFeatures(pub id);
impl std::ops::Deref for WKWindowFeatures {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WKWindowFeatures {}
impl WKWindowFeatures {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WKWindowFeatures").unwrap(), alloc) })
    }
}
impl INSObject for WKWindowFeatures {}
impl PNSObject for WKWindowFeatures {}
impl std::convert::TryFrom<NSObject> for WKWindowFeatures {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WKWindowFeatures, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WKWindowFeatures").unwrap()) };
        if is_kind_of {
            Ok(WKWindowFeatures(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WKWindowFeatures")
        }
    }
}
impl IWKWindowFeatures for WKWindowFeatures {}
pub trait IWKWindowFeatures: Sized + std::ops::Deref {
    unsafe fn menuBarVisibility(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuBarVisibility)
    }
    unsafe fn statusBarVisibility(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, statusBarVisibility)
    }
    unsafe fn toolbarsVisibility(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toolbarsVisibility)
    }
    unsafe fn allowsResizing(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsResizing)
    }
    unsafe fn x(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, x)
    }
    unsafe fn y(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, y)
    }
    unsafe fn width(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
}
pub trait NSObject_WebScripting: Sized + std::ops::Deref {
    unsafe fn invokeUndefinedMethodFromWebScript_withArguments_(
        &self,
        name: NSString,
        arguments: NSArray,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invokeUndefinedMethodFromWebScript : name, withArguments : arguments)
    }
    unsafe fn invokeDefaultMethodWithArguments_(&self, arguments: NSArray) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invokeDefaultMethodWithArguments : arguments)
    }
    unsafe fn finalizeForWebScript(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finalizeForWebScript)
    }
    unsafe fn webScriptNameForSelector_(selector: objc2::runtime::Sel) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), webScriptNameForSelector : selector)
    }
    unsafe fn isSelectorExcludedFromWebScript_(selector: objc2::runtime::Sel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), isSelectorExcludedFromWebScript : selector)
    }
    unsafe fn webScriptNameForKey_(name: *const ::std::os::raw::c_char) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), webScriptNameForKey : name)
    }
    unsafe fn isKeyExcludedFromWebScript_(name: *const ::std::os::raw::c_char) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), isKeyExcludedFromWebScript : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebScriptObject(pub id);
impl std::ops::Deref for WebScriptObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebScriptObject {}
impl WebScriptObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebScriptObject").unwrap(), alloc) })
    }
}
impl INSObject for WebScriptObject {}
impl PNSObject for WebScriptObject {}
impl std::convert::TryFrom<NSObject> for WebScriptObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebScriptObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebScriptObject").unwrap()) };
        if is_kind_of {
            Ok(WebScriptObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebScriptObject")
        }
    }
}
impl IWebScriptObject for WebScriptObject {}
pub trait IWebScriptObject: Sized + std::ops::Deref {
    unsafe fn JSObject(&self) -> JSObjectRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSObject)
    }
    unsafe fn callWebScriptMethod_withArguments_(&self, name: NSString, arguments: NSArray) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, callWebScriptMethod : name, withArguments : arguments)
    }
    unsafe fn evaluateWebScript_(&self, script: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateWebScript : script)
    }
    unsafe fn removeWebScriptKey_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeWebScriptKey : name)
    }
    unsafe fn stringRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringRepresentation)
    }
    unsafe fn webScriptValueAtIndex_(&self, index: ::std::os::raw::c_uint) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webScriptValueAtIndex : index)
    }
    unsafe fn setWebScriptValueAtIndex_value_(&self, index: ::std::os::raw::c_uint, value: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWebScriptValueAtIndex : index, value : value)
    }
    unsafe fn setException_(&self, description: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setException : description)
    }
    unsafe fn JSValue(&self) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSValue)
    }
    unsafe fn throwException_(exceptionMessage: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebScriptObject").unwrap(), throwException : exceptionMessage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebUndefined(pub id);
impl std::ops::Deref for WebUndefined {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebUndefined {}
impl WebUndefined {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebUndefined").unwrap(), alloc) })
    }
}
impl PNSCoding for WebUndefined {}
impl PNSCopying for WebUndefined {}
impl INSObject for WebUndefined {}
impl PNSObject for WebUndefined {}
impl std::convert::TryFrom<NSObject> for WebUndefined {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebUndefined, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebUndefined").unwrap()) };
        if is_kind_of {
            Ok(WebUndefined(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebUndefined")
        }
    }
}
impl IWebUndefined for WebUndefined {}
pub trait IWebUndefined: Sized + std::ops::Deref {
    unsafe fn undefined() -> WebUndefined
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebUndefined").unwrap(), undefined)
    }
}
pub type DOMTimeStamp = ::std::os::raw::c_ulonglong;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMObject(pub id);
impl std::ops::Deref for DOMObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMObject {}
impl DOMObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMObject").unwrap(), alloc) })
    }
}
impl PNSCopying for DOMObject {}
impl IWebScriptObject for DOMObject {}
impl From<DOMObject> for WebScriptObject {
    fn from(child: DOMObject) -> WebScriptObject {
        WebScriptObject(child.0)
    }
}
impl std::convert::TryFrom<WebScriptObject> for DOMObject {
    type Error = &'static str;
    fn try_from(parent: WebScriptObject) -> Result<DOMObject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMObject").unwrap()) };
        if is_kind_of {
            Ok(DOMObject(parent.0))
        } else {
            Err("This WebScriptObject cannot be downcasted to DOMObject")
        }
    }
}
impl INSObject for DOMObject {}
impl PNSObject for DOMObject {}
impl IDOMObject for DOMObject {}
pub trait IDOMObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
impl DOMObject_DOMLinkStyle for DOMObject {}
pub trait DOMObject_DOMLinkStyle: Sized + std::ops::Deref {
    unsafe fn sheet(&self) -> DOMStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sheet)
    }
}
pub trait PDOMEventTarget: Sized + std::ops::Deref {
    unsafe fn addEventListener_listener_useCapture_(
        &self,
        type_: NSString,
        listener: *mut u64,
        useCapture: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addEventListener : type_, listener : listener, useCapture : useCapture)
    }
    unsafe fn removeEventListener_listener_useCapture_(
        &self,
        type_: NSString,
        listener: *mut u64,
        useCapture: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeEventListener : type_, listener : listener, useCapture : useCapture)
    }
    unsafe fn dispatchEvent_(&self, event: DOMEvent) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchEvent : event)
    }
    unsafe fn addEventListener___(&self, type_: NSString, listener: *mut u64, useCapture: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addEventListener : type_, listener : listener, useCapture : useCapture)
    }
    unsafe fn removeEventListener___(&self, type_: NSString, listener: *mut u64, useCapture: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeEventListener : type_, listener : listener, useCapture : useCapture)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMNode(pub id);
impl std::ops::Deref for DOMNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMNode {}
impl DOMNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMNode").unwrap(), alloc) })
    }
}
impl PDOMEventTarget for DOMNode {}
impl IDOMObject for DOMNode {}
impl PNSCopying for DOMNode {}
impl From<DOMNode> for DOMObject {
    fn from(child: DOMNode) -> DOMObject {
        DOMObject(child.0)
    }
}
impl std::convert::TryFrom<DOMObject> for DOMNode {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMNode, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMNode").unwrap()) };
        if is_kind_of {
            Ok(DOMNode(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMNode")
        }
    }
}
impl IWebScriptObject for DOMNode {}
impl INSObject for DOMNode {}
impl PNSObject for DOMNode {}
impl IDOMNode for DOMNode {}
pub trait IDOMNode: Sized + std::ops::Deref {
    unsafe fn insertBefore_refChild_(&self, newChild: DOMNode, refChild: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertBefore : newChild, refChild : refChild)
    }
    unsafe fn replaceChild_oldChild_(&self, newChild: DOMNode, oldChild: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceChild : newChild, oldChild : oldChild)
    }
    unsafe fn removeChild_(&self, oldChild: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeChild : oldChild)
    }
    unsafe fn appendChild_(&self, newChild: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendChild : newChild)
    }
    unsafe fn hasChildNodes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasChildNodes)
    }
    unsafe fn cloneNode_(&self, deep: BOOL) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cloneNode : deep)
    }
    unsafe fn normalize(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalize)
    }
    unsafe fn isSupported_version_(&self, feature: NSString, version: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isSupported : feature, version : version)
    }
    unsafe fn hasAttributes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAttributes)
    }
    unsafe fn isSameNode_(&self, other: DOMNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isSameNode : other)
    }
    unsafe fn isEqualNode_(&self, other: DOMNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualNode : other)
    }
    unsafe fn lookupPrefix_(&self, namespaceURI: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookupPrefix : namespaceURI)
    }
    unsafe fn lookupNamespaceURI_(&self, prefix: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookupNamespaceURI : prefix)
    }
    unsafe fn isDefaultNamespace_(&self, namespaceURI: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isDefaultNamespace : namespaceURI)
    }
    unsafe fn compareDocumentPosition_(&self, other: DOMNode) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareDocumentPosition : other)
    }
    unsafe fn contains_(&self, other: DOMNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contains : other)
    }
    unsafe fn nodeName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodeName)
    }
    unsafe fn nodeValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodeValue)
    }
    unsafe fn setNodeValue_(&self, nodeValue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNodeValue : nodeValue)
    }
    unsafe fn nodeType(&self) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodeType)
    }
    unsafe fn parentNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentNode)
    }
    unsafe fn childNodes(&self) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childNodes)
    }
    unsafe fn firstChild(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstChild)
    }
    unsafe fn lastChild(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastChild)
    }
    unsafe fn previousSibling(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousSibling)
    }
    unsafe fn nextSibling(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextSibling)
    }
    unsafe fn ownerDocument(&self) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerDocument)
    }
    unsafe fn namespaceURI(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, namespaceURI)
    }
    unsafe fn prefix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prefix)
    }
    unsafe fn setPrefix_(&self, prefix: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrefix : prefix)
    }
    unsafe fn localName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localName)
    }
    unsafe fn attributes(&self) -> DOMNamedNodeMap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn baseURI(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseURI)
    }
    unsafe fn textContent(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textContent)
    }
    unsafe fn setTextContent_(&self, textContent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextContent : textContent)
    }
    unsafe fn parentElement(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentElement)
    }
    unsafe fn isContentEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContentEditable)
    }
}
impl DOMNode_DOMNodeDeprecated for DOMNode {}
pub trait DOMNode_DOMNodeDeprecated: Sized + std::ops::Deref {
    unsafe fn insertBefore__(&self, newChild: DOMNode, refChild: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertBefore : newChild, refChild : refChild)
    }
    unsafe fn replaceChild__(&self, newChild: DOMNode, oldChild: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceChild : newChild, oldChild : oldChild)
    }
    unsafe fn isSupported__(&self, feature: NSString, version: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isSupported : feature, version : version)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMAttr(pub id);
impl std::ops::Deref for DOMAttr {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMAttr {}
impl DOMAttr {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMAttr").unwrap(), alloc) })
    }
}
impl IDOMNode for DOMAttr {}
impl PDOMEventTarget for DOMAttr {}
impl From<DOMAttr> for DOMNode {
    fn from(child: DOMAttr) -> DOMNode {
        DOMNode(child.0)
    }
}
impl std::convert::TryFrom<DOMNode> for DOMAttr {
    type Error = &'static str;
    fn try_from(parent: DOMNode) -> Result<DOMAttr, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMAttr").unwrap()) };
        if is_kind_of {
            Ok(DOMAttr(parent.0))
        } else {
            Err("This DOMNode cannot be downcasted to DOMAttr")
        }
    }
}
impl IDOMObject for DOMAttr {}
impl PNSCopying for DOMAttr {}
impl IWebScriptObject for DOMAttr {}
impl INSObject for DOMAttr {}
impl PNSObject for DOMAttr {}
impl IDOMAttr for DOMAttr {}
pub trait IDOMAttr: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn specified(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specified)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn ownerElement(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerElement)
    }
    unsafe fn style(&self) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCharacterData(pub id);
impl std::ops::Deref for DOMCharacterData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCharacterData {}
impl DOMCharacterData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCharacterData").unwrap(), alloc) })
    }
}
impl IDOMNode for DOMCharacterData {}
impl PDOMEventTarget for DOMCharacterData {}
impl std::convert::TryFrom<DOMNode> for DOMCharacterData {
    type Error = &'static str;
    fn try_from(parent: DOMNode) -> Result<DOMCharacterData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCharacterData").unwrap()) };
        if is_kind_of {
            Ok(DOMCharacterData(parent.0))
        } else {
            Err("This DOMNode cannot be downcasted to DOMCharacterData")
        }
    }
}
impl IDOMObject for DOMCharacterData {}
impl PNSCopying for DOMCharacterData {}
impl IWebScriptObject for DOMCharacterData {}
impl INSObject for DOMCharacterData {}
impl PNSObject for DOMCharacterData {}
impl IDOMCharacterData for DOMCharacterData {}
pub trait IDOMCharacterData: Sized + std::ops::Deref {
    unsafe fn substringData_length_(
        &self,
        offset: ::std::os::raw::c_uint,
        length: ::std::os::raw::c_uint,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, substringData : offset, length : length)
    }
    unsafe fn appendData_(&self, data: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendData : data)
    }
    unsafe fn insertData_data_(&self, offset: ::std::os::raw::c_uint, data: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertData : offset, data : data)
    }
    unsafe fn deleteData_length_(
        &self,
        offset: ::std::os::raw::c_uint,
        length: ::std::os::raw::c_uint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteData : offset, length : length)
    }
    unsafe fn replaceData_length_data_(
        &self,
        offset: ::std::os::raw::c_uint,
        length: ::std::os::raw::c_uint,
        data: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceData : offset, length : length, data : data)
    }
    unsafe fn data(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn setData_(&self, data: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setData : data)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
impl DOMCharacterData_DOMCharacterDataDeprecated for DOMCharacterData {}
pub trait DOMCharacterData_DOMCharacterDataDeprecated: Sized + std::ops::Deref {
    unsafe fn substringData__(
        &self,
        offset: ::std::os::raw::c_uint,
        length: ::std::os::raw::c_uint,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, substringData : offset, length : length)
    }
    unsafe fn insertData__(&self, offset: ::std::os::raw::c_uint, data: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertData : offset, data : data)
    }
    unsafe fn deleteData__(&self, offset: ::std::os::raw::c_uint, length: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteData : offset, length : length)
    }
    unsafe fn replaceData___(
        &self,
        offset: ::std::os::raw::c_uint,
        length: ::std::os::raw::c_uint,
        data: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceData : offset, length : length, data : data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMText(pub id);
impl std::ops::Deref for DOMText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMText {}
impl DOMText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMText").unwrap(), alloc) })
    }
}
impl IDOMCharacterData for DOMText {}
impl From<DOMText> for DOMCharacterData {
    fn from(child: DOMText) -> DOMCharacterData {
        DOMCharacterData(child.0)
    }
}
impl std::convert::TryFrom<DOMCharacterData> for DOMText {
    type Error = &'static str;
    fn try_from(parent: DOMCharacterData) -> Result<DOMText, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMText").unwrap()) };
        if is_kind_of {
            Ok(DOMText(parent.0))
        } else {
            Err("This DOMCharacterData cannot be downcasted to DOMText")
        }
    }
}
impl IDOMNode for DOMText {}
impl PDOMEventTarget for DOMText {}
impl IDOMObject for DOMText {}
impl PNSCopying for DOMText {}
impl IWebScriptObject for DOMText {}
impl INSObject for DOMText {}
impl PNSObject for DOMText {}
impl IDOMText for DOMText {}
pub trait IDOMText: Sized + std::ops::Deref {
    unsafe fn splitText_(&self, offset: ::std::os::raw::c_uint) -> DOMText
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, splitText : offset)
    }
    unsafe fn replaceWholeText_(&self, content: NSString) -> DOMText
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceWholeText : content)
    }
    unsafe fn wholeText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wholeText)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCDATASection(pub id);
impl std::ops::Deref for DOMCDATASection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCDATASection {}
impl DOMCDATASection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCDATASection").unwrap(), alloc) })
    }
}
impl IDOMText for DOMCDATASection {}
impl From<DOMCDATASection> for DOMText {
    fn from(child: DOMCDATASection) -> DOMText {
        DOMText(child.0)
    }
}
impl std::convert::TryFrom<DOMText> for DOMCDATASection {
    type Error = &'static str;
    fn try_from(parent: DOMText) -> Result<DOMCDATASection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCDATASection").unwrap()) };
        if is_kind_of {
            Ok(DOMCDATASection(parent.0))
        } else {
            Err("This DOMText cannot be downcasted to DOMCDATASection")
        }
    }
}
impl IDOMCharacterData for DOMCDATASection {}
impl IDOMNode for DOMCDATASection {}
impl PDOMEventTarget for DOMCDATASection {}
impl IDOMObject for DOMCDATASection {}
impl PNSCopying for DOMCDATASection {}
impl IWebScriptObject for DOMCDATASection {}
impl INSObject for DOMCDATASection {}
impl PNSObject for DOMCDATASection {}
impl IDOMCDATASection for DOMCDATASection {}
pub trait IDOMCDATASection: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMComment(pub id);
impl std::ops::Deref for DOMComment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMComment {}
impl DOMComment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMComment").unwrap(), alloc) })
    }
}
impl IDOMCharacterData for DOMComment {}
impl std::convert::TryFrom<DOMCharacterData> for DOMComment {
    type Error = &'static str;
    fn try_from(parent: DOMCharacterData) -> Result<DOMComment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMComment").unwrap()) };
        if is_kind_of {
            Ok(DOMComment(parent.0))
        } else {
            Err("This DOMCharacterData cannot be downcasted to DOMComment")
        }
    }
}
impl IDOMNode for DOMComment {}
impl PDOMEventTarget for DOMComment {}
impl IDOMObject for DOMComment {}
impl PNSCopying for DOMComment {}
impl IWebScriptObject for DOMComment {}
impl INSObject for DOMComment {}
impl PNSObject for DOMComment {}
impl IDOMComment for DOMComment {}
pub trait IDOMComment: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMDocument(pub id);
impl std::ops::Deref for DOMDocument {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMDocument {}
impl DOMDocument {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMDocument").unwrap(), alloc) })
    }
}
impl IDOMNode for DOMDocument {}
impl PDOMEventTarget for DOMDocument {}
impl std::convert::TryFrom<DOMNode> for DOMDocument {
    type Error = &'static str;
    fn try_from(parent: DOMNode) -> Result<DOMDocument, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMDocument").unwrap()) };
        if is_kind_of {
            Ok(DOMDocument(parent.0))
        } else {
            Err("This DOMNode cannot be downcasted to DOMDocument")
        }
    }
}
impl IDOMObject for DOMDocument {}
impl PNSCopying for DOMDocument {}
impl IWebScriptObject for DOMDocument {}
impl INSObject for DOMDocument {}
impl PNSObject for DOMDocument {}
impl IDOMDocument for DOMDocument {}
pub trait IDOMDocument: Sized + std::ops::Deref {
    unsafe fn createElement_(&self, tagName: NSString) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createElement : tagName)
    }
    unsafe fn createDocumentFragment(&self) -> DOMDocumentFragment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createDocumentFragment)
    }
    unsafe fn createTextNode_(&self, data: NSString) -> DOMText
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createTextNode : data)
    }
    unsafe fn createComment_(&self, data: NSString) -> DOMComment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createComment : data)
    }
    unsafe fn createCDATASection_(&self, data: NSString) -> DOMCDATASection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCDATASection : data)
    }
    unsafe fn createProcessingInstruction_data_(
        &self,
        target: NSString,
        data: NSString,
    ) -> DOMProcessingInstruction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createProcessingInstruction : target, data : data)
    }
    unsafe fn createAttribute_(&self, name: NSString) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createAttribute : name)
    }
    unsafe fn createEntityReference_(&self, name: NSString) -> DOMEntityReference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createEntityReference : name)
    }
    unsafe fn getElementsByTagName_(&self, tagname: NSString) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByTagName : tagname)
    }
    unsafe fn importNode_deep_(&self, importedNode: DOMNode, deep: BOOL) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, importNode : importedNode, deep : deep)
    }
    unsafe fn createElementNS_qualifiedName_(
        &self,
        namespaceURI: NSString,
        qualifiedName: NSString,
    ) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createElementNS : namespaceURI, qualifiedName : qualifiedName)
    }
    unsafe fn createAttributeNS_qualifiedName_(
        &self,
        namespaceURI: NSString,
        qualifiedName: NSString,
    ) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createAttributeNS : namespaceURI, qualifiedName : qualifiedName)
    }
    unsafe fn getElementsByTagNameNS_localName_(
        &self,
        namespaceURI: NSString,
        localName: NSString,
    ) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByTagNameNS : namespaceURI, localName : localName)
    }
    unsafe fn adoptNode_(&self, source: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, adoptNode : source)
    }
    unsafe fn createEvent_(&self, eventType: NSString) -> DOMEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createEvent : eventType)
    }
    unsafe fn createRange(&self) -> DOMRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createRange)
    }
    unsafe fn createNodeIterator_whatToShow_filter_expandEntityReferences_(
        &self,
        root: DOMNode,
        whatToShow: ::std::os::raw::c_uint,
        filter: *mut u64,
        expandEntityReferences: BOOL,
    ) -> DOMNodeIterator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createNodeIterator : root, whatToShow : whatToShow, filter : filter, expandEntityReferences : expandEntityReferences)
    }
    unsafe fn createTreeWalker_whatToShow_filter_expandEntityReferences_(
        &self,
        root: DOMNode,
        whatToShow: ::std::os::raw::c_uint,
        filter: *mut u64,
        expandEntityReferences: BOOL,
    ) -> DOMTreeWalker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createTreeWalker : root, whatToShow : whatToShow, filter : filter, expandEntityReferences : expandEntityReferences)
    }
    unsafe fn getOverrideStyle_pseudoElement_(
        &self,
        element: DOMElement,
        pseudoElement: NSString,
    ) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getOverrideStyle : element, pseudoElement : pseudoElement)
    }
    unsafe fn createExpression_resolver_(
        &self,
        expression: NSString,
        resolver: *mut u64,
    ) -> DOMXPathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createExpression : expression, resolver : resolver)
    }
    unsafe fn createNSResolver_(&self, nodeResolver: DOMNode) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createNSResolver : nodeResolver)
    }
    unsafe fn evaluate_contextNode_resolver_type_inResult_(
        &self,
        expression: NSString,
        contextNode: DOMNode,
        resolver: *mut u64,
        type_: ::std::os::raw::c_ushort,
        inResult: DOMXPathResult,
    ) -> DOMXPathResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluate : expression, contextNode : contextNode, resolver : resolver, r#type : type_, inResult : inResult)
    }
    unsafe fn execCommand_userInterface_value_(
        &self,
        command: NSString,
        userInterface: BOOL,
        value: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, execCommand : command, userInterface : userInterface, value : value)
    }
    unsafe fn execCommand_userInterface_(&self, command: NSString, userInterface: BOOL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, execCommand : command, userInterface : userInterface)
    }
    unsafe fn execCommand_(&self, command: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, execCommand : command)
    }
    unsafe fn queryCommandEnabled_(&self, command: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryCommandEnabled : command)
    }
    unsafe fn queryCommandIndeterm_(&self, command: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryCommandIndeterm : command)
    }
    unsafe fn queryCommandState_(&self, command: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryCommandState : command)
    }
    unsafe fn queryCommandSupported_(&self, command: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryCommandSupported : command)
    }
    unsafe fn queryCommandValue_(&self, command: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryCommandValue : command)
    }
    unsafe fn getElementsByName_(&self, elementName: NSString) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByName : elementName)
    }
    unsafe fn elementFromPoint_y_(
        &self,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementFromPoint : x, y : y)
    }
    unsafe fn createCSSStyleDeclaration(&self) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createCSSStyleDeclaration)
    }
    unsafe fn getComputedStyle_pseudoElement_(
        &self,
        element: DOMElement,
        pseudoElement: NSString,
    ) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getComputedStyle : element, pseudoElement : pseudoElement)
    }
    unsafe fn getMatchedCSSRules_pseudoElement_(
        &self,
        element: DOMElement,
        pseudoElement: NSString,
    ) -> DOMCSSRuleList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getMatchedCSSRules : element, pseudoElement : pseudoElement)
    }
    unsafe fn getMatchedCSSRules_pseudoElement_authorOnly_(
        &self,
        element: DOMElement,
        pseudoElement: NSString,
        authorOnly: BOOL,
    ) -> DOMCSSRuleList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getMatchedCSSRules : element, pseudoElement : pseudoElement, authorOnly : authorOnly)
    }
    unsafe fn getElementsByClassName_(&self, classNames: NSString) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByClassName : classNames)
    }
    unsafe fn hasFocus(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasFocus)
    }
    unsafe fn webkitCancelFullScreen(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webkitCancelFullScreen)
    }
    unsafe fn getElementById_(&self, elementId: NSString) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementById : elementId)
    }
    unsafe fn querySelector_(&self, selectors: NSString) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, querySelector : selectors)
    }
    unsafe fn querySelectorAll_(&self, selectors: NSString) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, querySelectorAll : selectors)
    }
    unsafe fn doctype(&self) -> DOMDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doctype)
    }
    unsafe fn implementation(&self) -> DOMImplementation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, implementation)
    }
    unsafe fn documentElement(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentElement)
    }
    unsafe fn inputEncoding(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputEncoding)
    }
    unsafe fn xmlEncoding(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xmlEncoding)
    }
    unsafe fn xmlVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xmlVersion)
    }
    unsafe fn setXmlVersion_(&self, xmlVersion: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXmlVersion : xmlVersion)
    }
    unsafe fn xmlStandalone(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xmlStandalone)
    }
    unsafe fn setXmlStandalone_(&self, xmlStandalone: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXmlStandalone : xmlStandalone)
    }
    unsafe fn documentURI(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentURI)
    }
    unsafe fn setDocumentURI_(&self, documentURI: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentURI : documentURI)
    }
    unsafe fn defaultView(&self) -> DOMAbstractView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultView)
    }
    unsafe fn styleSheets(&self) -> DOMStyleSheetList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, styleSheets)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn referrer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, referrer)
    }
    unsafe fn domain(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domain)
    }
    unsafe fn URL(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn cookie(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cookie)
    }
    unsafe fn setCookie_(&self, cookie: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCookie : cookie)
    }
    unsafe fn body(&self) -> DOMHTMLElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body)
    }
    unsafe fn setBody_(&self, body: DOMHTMLElement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBody : body)
    }
    unsafe fn images(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, images)
    }
    unsafe fn applets(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applets)
    }
    unsafe fn links(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, links)
    }
    unsafe fn forms(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forms)
    }
    unsafe fn anchors(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchors)
    }
    unsafe fn lastModified(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastModified)
    }
    unsafe fn charset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charset)
    }
    unsafe fn setCharset_(&self, charset: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharset : charset)
    }
    unsafe fn defaultCharset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultCharset)
    }
    unsafe fn readyState(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readyState)
    }
    unsafe fn characterSet(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characterSet)
    }
    unsafe fn preferredStylesheetSet(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredStylesheetSet)
    }
    unsafe fn selectedStylesheetSet(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedStylesheetSet)
    }
    unsafe fn setSelectedStylesheetSet_(&self, selectedStylesheetSet: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedStylesheetSet : selectedStylesheetSet)
    }
    unsafe fn activeElement(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeElement)
    }
}
impl DOMDocument_DOMDocumentDeprecated for DOMDocument {}
pub trait DOMDocument_DOMDocumentDeprecated: Sized + std::ops::Deref {
    unsafe fn createProcessingInstruction__(
        &self,
        target: NSString,
        data: NSString,
    ) -> DOMProcessingInstruction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createProcessingInstruction : target, data : data)
    }
    unsafe fn importNode__(&self, importedNode: DOMNode, deep: BOOL) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, importNode : importedNode, deep : deep)
    }
    unsafe fn createElementNS__(
        &self,
        namespaceURI: NSString,
        qualifiedName: NSString,
    ) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createElementNS : namespaceURI, qualifiedName : qualifiedName)
    }
    unsafe fn createAttributeNS__(&self, namespaceURI: NSString, qualifiedName: NSString) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createAttributeNS : namespaceURI, qualifiedName : qualifiedName)
    }
    unsafe fn getElementsByTagNameNS__(
        &self,
        namespaceURI: NSString,
        localName: NSString,
    ) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByTagNameNS : namespaceURI, localName : localName)
    }
    unsafe fn createNodeIterator____(
        &self,
        root: DOMNode,
        whatToShow: ::std::os::raw::c_uint,
        filter: *mut u64,
        expandEntityReferences: BOOL,
    ) -> DOMNodeIterator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createNodeIterator : root, whatToShow : whatToShow, filter : filter, expandEntityReferences : expandEntityReferences)
    }
    unsafe fn createTreeWalker____(
        &self,
        root: DOMNode,
        whatToShow: ::std::os::raw::c_uint,
        filter: *mut u64,
        expandEntityReferences: BOOL,
    ) -> DOMTreeWalker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createTreeWalker : root, whatToShow : whatToShow, filter : filter, expandEntityReferences : expandEntityReferences)
    }
    unsafe fn getOverrideStyle__(
        &self,
        element: DOMElement,
        pseudoElement: NSString,
    ) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getOverrideStyle : element, pseudoElement : pseudoElement)
    }
    unsafe fn createExpression__(
        &self,
        expression: NSString,
        resolver: *mut u64,
    ) -> DOMXPathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createExpression : expression, resolver : resolver)
    }
    unsafe fn evaluate_____(
        &self,
        expression: NSString,
        contextNode: DOMNode,
        resolver: *mut u64,
        type_: ::std::os::raw::c_ushort,
        inResult: DOMXPathResult,
    ) -> DOMXPathResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluate : expression, contextNode : contextNode, resolver : resolver, type_ : type_, inResult : inResult)
    }
    unsafe fn getComputedStyle__(
        &self,
        element: DOMElement,
        pseudoElement: NSString,
    ) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getComputedStyle : element, pseudoElement : pseudoElement)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMDocumentFragment(pub id);
impl std::ops::Deref for DOMDocumentFragment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMDocumentFragment {}
impl DOMDocumentFragment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMDocumentFragment").unwrap(), alloc) })
    }
}
impl IDOMNode for DOMDocumentFragment {}
impl PDOMEventTarget for DOMDocumentFragment {}
impl std::convert::TryFrom<DOMNode> for DOMDocumentFragment {
    type Error = &'static str;
    fn try_from(parent: DOMNode) -> Result<DOMDocumentFragment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMDocumentFragment").unwrap()) };
        if is_kind_of {
            Ok(DOMDocumentFragment(parent.0))
        } else {
            Err("This DOMNode cannot be downcasted to DOMDocumentFragment")
        }
    }
}
impl IDOMObject for DOMDocumentFragment {}
impl PNSCopying for DOMDocumentFragment {}
impl IWebScriptObject for DOMDocumentFragment {}
impl INSObject for DOMDocumentFragment {}
impl PNSObject for DOMDocumentFragment {}
impl IDOMDocumentFragment for DOMDocumentFragment {}
pub trait IDOMDocumentFragment: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMDocumentType(pub id);
impl std::ops::Deref for DOMDocumentType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMDocumentType {}
impl DOMDocumentType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMDocumentType").unwrap(), alloc) })
    }
}
impl IDOMNode for DOMDocumentType {}
impl PDOMEventTarget for DOMDocumentType {}
impl std::convert::TryFrom<DOMNode> for DOMDocumentType {
    type Error = &'static str;
    fn try_from(parent: DOMNode) -> Result<DOMDocumentType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMDocumentType").unwrap()) };
        if is_kind_of {
            Ok(DOMDocumentType(parent.0))
        } else {
            Err("This DOMNode cannot be downcasted to DOMDocumentType")
        }
    }
}
impl IDOMObject for DOMDocumentType {}
impl PNSCopying for DOMDocumentType {}
impl IWebScriptObject for DOMDocumentType {}
impl INSObject for DOMDocumentType {}
impl PNSObject for DOMDocumentType {}
impl IDOMDocumentType for DOMDocumentType {}
pub trait IDOMDocumentType: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn entities(&self) -> DOMNamedNodeMap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entities)
    }
    unsafe fn notations(&self) -> DOMNamedNodeMap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notations)
    }
    unsafe fn publicId(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicId)
    }
    unsafe fn systemId(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemId)
    }
    unsafe fn internalSubset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, internalSubset)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMElement(pub id);
impl std::ops::Deref for DOMElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMElement {}
impl DOMElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMElement").unwrap(), alloc) })
    }
}
impl IDOMNode for DOMElement {}
impl PDOMEventTarget for DOMElement {}
impl std::convert::TryFrom<DOMNode> for DOMElement {
    type Error = &'static str;
    fn try_from(parent: DOMNode) -> Result<DOMElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMElement").unwrap()) };
        if is_kind_of {
            Ok(DOMElement(parent.0))
        } else {
            Err("This DOMNode cannot be downcasted to DOMElement")
        }
    }
}
impl IDOMObject for DOMElement {}
impl PNSCopying for DOMElement {}
impl IWebScriptObject for DOMElement {}
impl INSObject for DOMElement {}
impl PNSObject for DOMElement {}
impl IDOMElement for DOMElement {}
pub trait IDOMElement: Sized + std::ops::Deref {
    unsafe fn getAttribute_(&self, name: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttribute : name)
    }
    unsafe fn setAttribute_value_(&self, name: NSString, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttribute : name, value : value)
    }
    unsafe fn removeAttribute_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAttribute : name)
    }
    unsafe fn getAttributeNode_(&self, name: NSString) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttributeNode : name)
    }
    unsafe fn setAttributeNode_(&self, newAttr: DOMAttr) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeNode : newAttr)
    }
    unsafe fn removeAttributeNode_(&self, oldAttr: DOMAttr) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAttributeNode : oldAttr)
    }
    unsafe fn getElementsByTagName_(&self, name: NSString) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByTagName : name)
    }
    unsafe fn getAttributeNS_localName_(
        &self,
        namespaceURI: NSString,
        localName: NSString,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttributeNS : namespaceURI, localName : localName)
    }
    unsafe fn setAttributeNS_qualifiedName_value_(
        &self,
        namespaceURI: NSString,
        qualifiedName: NSString,
        value: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeNS : namespaceURI, qualifiedName : qualifiedName, value : value)
    }
    unsafe fn removeAttributeNS_localName_(&self, namespaceURI: NSString, localName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAttributeNS : namespaceURI, localName : localName)
    }
    unsafe fn getElementsByTagNameNS_localName_(
        &self,
        namespaceURI: NSString,
        localName: NSString,
    ) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByTagNameNS : namespaceURI, localName : localName)
    }
    unsafe fn getAttributeNodeNS_localName_(
        &self,
        namespaceURI: NSString,
        localName: NSString,
    ) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttributeNodeNS : namespaceURI, localName : localName)
    }
    unsafe fn setAttributeNodeNS_(&self, newAttr: DOMAttr) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeNodeNS : newAttr)
    }
    unsafe fn hasAttribute_(&self, name: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasAttribute : name)
    }
    unsafe fn hasAttributeNS_localName_(&self, namespaceURI: NSString, localName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasAttributeNS : namespaceURI, localName : localName)
    }
    unsafe fn focus(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focus)
    }
    unsafe fn blur(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blur)
    }
    unsafe fn scrollIntoView_(&self, alignWithTop: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollIntoView : alignWithTop)
    }
    unsafe fn scrollIntoViewIfNeeded_(&self, centerIfNeeded: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollIntoViewIfNeeded : centerIfNeeded)
    }
    unsafe fn getElementsByClassName_(&self, name: NSString) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByClassName : name)
    }
    unsafe fn webkitRequestFullScreen_(&self, flags: ::std::os::raw::c_ushort)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webkitRequestFullScreen : flags)
    }
    unsafe fn querySelector_(&self, selectors: NSString) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, querySelector : selectors)
    }
    unsafe fn querySelectorAll_(&self, selectors: NSString) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, querySelectorAll : selectors)
    }
    unsafe fn tagName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tagName)
    }
    unsafe fn style(&self) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn offsetLeft(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsetLeft)
    }
    unsafe fn offsetTop(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsetTop)
    }
    unsafe fn offsetWidth(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsetWidth)
    }
    unsafe fn offsetHeight(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsetHeight)
    }
    unsafe fn clientLeft(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientLeft)
    }
    unsafe fn clientTop(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientTop)
    }
    unsafe fn clientWidth(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientWidth)
    }
    unsafe fn clientHeight(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientHeight)
    }
    unsafe fn scrollLeft(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrollLeft)
    }
    unsafe fn setScrollLeft_(&self, scrollLeft: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScrollLeft : scrollLeft)
    }
    unsafe fn scrollTop(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrollTop)
    }
    unsafe fn setScrollTop_(&self, scrollTop: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScrollTop : scrollTop)
    }
    unsafe fn scrollWidth(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrollWidth)
    }
    unsafe fn scrollHeight(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrollHeight)
    }
    unsafe fn offsetParent(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsetParent)
    }
    unsafe fn innerHTML(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, innerHTML)
    }
    unsafe fn setInnerHTML_(&self, innerHTML: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInnerHTML : innerHTML)
    }
    unsafe fn outerHTML(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerHTML)
    }
    unsafe fn setOuterHTML_(&self, outerHTML: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOuterHTML : outerHTML)
    }
    unsafe fn className(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, className)
    }
    unsafe fn setClassName_(&self, className: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClassName : className)
    }
    unsafe fn innerText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, innerText)
    }
    unsafe fn previousElementSibling(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousElementSibling)
    }
    unsafe fn nextElementSibling(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextElementSibling)
    }
    unsafe fn firstElementChild(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstElementChild)
    }
    unsafe fn lastElementChild(&self) -> DOMElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastElementChild)
    }
    unsafe fn childElementCount(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childElementCount)
    }
}
impl DOMElement_DOMElementDeprecated for DOMElement {}
pub trait DOMElement_DOMElementDeprecated: Sized + std::ops::Deref {
    unsafe fn setAttribute__(&self, name: NSString, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttribute : name, value : value)
    }
    unsafe fn getAttributeNS__(&self, namespaceURI: NSString, localName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttributeNS : namespaceURI, localName : localName)
    }
    unsafe fn setAttributeNS___(
        &self,
        namespaceURI: NSString,
        qualifiedName: NSString,
        value: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeNS : namespaceURI, qualifiedName : qualifiedName, value : value)
    }
    unsafe fn removeAttributeNS__(&self, namespaceURI: NSString, localName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAttributeNS : namespaceURI, localName : localName)
    }
    unsafe fn getElementsByTagNameNS__(
        &self,
        namespaceURI: NSString,
        localName: NSString,
    ) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getElementsByTagNameNS : namespaceURI, localName : localName)
    }
    unsafe fn getAttributeNodeNS__(&self, namespaceURI: NSString, localName: NSString) -> DOMAttr
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttributeNodeNS : namespaceURI, localName : localName)
    }
    unsafe fn hasAttributeNS__(&self, namespaceURI: NSString, localName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasAttributeNS : namespaceURI, localName : localName)
    }
    unsafe fn scrollByLines_(&self, lines: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollByLines : lines)
    }
    unsafe fn scrollByPages_(&self, pages: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollByPages : pages)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMEntity(pub id);
impl std::ops::Deref for DOMEntity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMEntity {}
impl DOMEntity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMEntity").unwrap(), alloc) })
    }
}
impl IDOMNode for DOMEntity {}
impl PDOMEventTarget for DOMEntity {}
impl std::convert::TryFrom<DOMNode> for DOMEntity {
    type Error = &'static str;
    fn try_from(parent: DOMNode) -> Result<DOMEntity, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMEntity").unwrap()) };
        if is_kind_of {
            Ok(DOMEntity(parent.0))
        } else {
            Err("This DOMNode cannot be downcasted to DOMEntity")
        }
    }
}
impl IDOMObject for DOMEntity {}
impl PNSCopying for DOMEntity {}
impl IWebScriptObject for DOMEntity {}
impl INSObject for DOMEntity {}
impl PNSObject for DOMEntity {}
impl IDOMEntity for DOMEntity {}
pub trait IDOMEntity: Sized + std::ops::Deref {
    unsafe fn publicId(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicId)
    }
    unsafe fn systemId(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemId)
    }
    unsafe fn notationName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notationName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMEntityReference(pub id);
impl std::ops::Deref for DOMEntityReference {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMEntityReference {}
impl DOMEntityReference {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMEntityReference").unwrap(), alloc) })
    }
}
impl IDOMNode for DOMEntityReference {}
impl PDOMEventTarget for DOMEntityReference {}
impl std::convert::TryFrom<DOMNode> for DOMEntityReference {
    type Error = &'static str;
    fn try_from(parent: DOMNode) -> Result<DOMEntityReference, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMEntityReference").unwrap()) };
        if is_kind_of {
            Ok(DOMEntityReference(parent.0))
        } else {
            Err("This DOMNode cannot be downcasted to DOMEntityReference")
        }
    }
}
impl IDOMObject for DOMEntityReference {}
impl PNSCopying for DOMEntityReference {}
impl IWebScriptObject for DOMEntityReference {}
impl INSObject for DOMEntityReference {}
impl PNSObject for DOMEntityReference {}
impl IDOMEntityReference for DOMEntityReference {}
pub trait IDOMEntityReference: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMImplementation(pub id);
impl std::ops::Deref for DOMImplementation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMImplementation {}
impl DOMImplementation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMImplementation").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMImplementation {}
impl PNSCopying for DOMImplementation {}
impl std::convert::TryFrom<DOMObject> for DOMImplementation {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMImplementation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMImplementation").unwrap()) };
        if is_kind_of {
            Ok(DOMImplementation(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMImplementation")
        }
    }
}
impl IWebScriptObject for DOMImplementation {}
impl INSObject for DOMImplementation {}
impl PNSObject for DOMImplementation {}
impl IDOMImplementation for DOMImplementation {}
pub trait IDOMImplementation: Sized + std::ops::Deref {
    unsafe fn hasFeature_version_(&self, feature: NSString, version: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasFeature : feature, version : version)
    }
    unsafe fn createDocumentType_publicId_systemId_(
        &self,
        qualifiedName: NSString,
        publicId: NSString,
        systemId: NSString,
    ) -> DOMDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createDocumentType : qualifiedName, publicId : publicId, systemId : systemId)
    }
    unsafe fn createDocument_qualifiedName_doctype_(
        &self,
        namespaceURI: NSString,
        qualifiedName: NSString,
        doctype: DOMDocumentType,
    ) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createDocument : namespaceURI, qualifiedName : qualifiedName, doctype : doctype)
    }
    unsafe fn createCSSStyleSheet_media_(
        &self,
        title: NSString,
        media: NSString,
    ) -> DOMCSSStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCSSStyleSheet : title, media : media)
    }
    unsafe fn createHTMLDocument_(&self, title: NSString) -> DOMHTMLDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createHTMLDocument : title)
    }
}
impl DOMImplementation_DOMImplementationDeprecated for DOMImplementation {}
pub trait DOMImplementation_DOMImplementationDeprecated: Sized + std::ops::Deref {
    unsafe fn hasFeature__(&self, feature: NSString, version: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasFeature : feature, version : version)
    }
    unsafe fn createDocumentType___(
        &self,
        qualifiedName: NSString,
        publicId: NSString,
        systemId: NSString,
    ) -> DOMDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createDocumentType : qualifiedName, publicId : publicId, systemId : systemId)
    }
    unsafe fn createDocument___(
        &self,
        namespaceURI: NSString,
        qualifiedName: NSString,
        doctype: DOMDocumentType,
    ) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createDocument : namespaceURI, qualifiedName : qualifiedName, doctype : doctype)
    }
    unsafe fn createCSSStyleSheet__(&self, title: NSString, media: NSString) -> DOMCSSStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCSSStyleSheet : title, media : media)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMNamedNodeMap(pub id);
impl std::ops::Deref for DOMNamedNodeMap {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMNamedNodeMap {}
impl DOMNamedNodeMap {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMNamedNodeMap").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMNamedNodeMap {}
impl PNSCopying for DOMNamedNodeMap {}
impl std::convert::TryFrom<DOMObject> for DOMNamedNodeMap {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMNamedNodeMap, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMNamedNodeMap").unwrap()) };
        if is_kind_of {
            Ok(DOMNamedNodeMap(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMNamedNodeMap")
        }
    }
}
impl IWebScriptObject for DOMNamedNodeMap {}
impl INSObject for DOMNamedNodeMap {}
impl PNSObject for DOMNamedNodeMap {}
impl IDOMNamedNodeMap for DOMNamedNodeMap {}
pub trait IDOMNamedNodeMap: Sized + std::ops::Deref {
    unsafe fn getNamedItem_(&self, name: NSString) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getNamedItem : name)
    }
    unsafe fn setNamedItem_(&self, node: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNamedItem : node)
    }
    unsafe fn removeNamedItem_(&self, name: NSString) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeNamedItem : name)
    }
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn getNamedItemNS_localName_(
        &self,
        namespaceURI: NSString,
        localName: NSString,
    ) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getNamedItemNS : namespaceURI, localName : localName)
    }
    unsafe fn setNamedItemNS_(&self, node: DOMNode) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNamedItemNS : node)
    }
    unsafe fn removeNamedItemNS_localName_(
        &self,
        namespaceURI: NSString,
        localName: NSString,
    ) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeNamedItemNS : namespaceURI, localName : localName)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
impl DOMNamedNodeMap_DOMNamedNodeMapDeprecated for DOMNamedNodeMap {}
pub trait DOMNamedNodeMap_DOMNamedNodeMapDeprecated: Sized + std::ops::Deref {
    unsafe fn getNamedItemNS__(&self, namespaceURI: NSString, localName: NSString) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getNamedItemNS : namespaceURI, localName : localName)
    }
    unsafe fn removeNamedItemNS__(&self, namespaceURI: NSString, localName: NSString) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeNamedItemNS : namespaceURI, localName : localName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMNodeList(pub id);
impl std::ops::Deref for DOMNodeList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMNodeList {}
impl DOMNodeList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMNodeList").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMNodeList {}
impl PNSCopying for DOMNodeList {}
impl std::convert::TryFrom<DOMObject> for DOMNodeList {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMNodeList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMNodeList").unwrap()) };
        if is_kind_of {
            Ok(DOMNodeList(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMNodeList")
        }
    }
}
impl IWebScriptObject for DOMNodeList {}
impl INSObject for DOMNodeList {}
impl PNSObject for DOMNodeList {}
impl IDOMNodeList for DOMNodeList {}
pub trait IDOMNodeList: Sized + std::ops::Deref {
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMProcessingInstruction(pub id);
impl std::ops::Deref for DOMProcessingInstruction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMProcessingInstruction {}
impl DOMProcessingInstruction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMProcessingInstruction").unwrap(), alloc) })
    }
}
impl IDOMCharacterData for DOMProcessingInstruction {}
impl std::convert::TryFrom<DOMCharacterData> for DOMProcessingInstruction {
    type Error = &'static str;
    fn try_from(parent: DOMCharacterData) -> Result<DOMProcessingInstruction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMProcessingInstruction").unwrap()) };
        if is_kind_of {
            Ok(DOMProcessingInstruction(parent.0))
        } else {
            Err("This DOMCharacterData cannot be downcasted to DOMProcessingInstruction")
        }
    }
}
impl IDOMNode for DOMProcessingInstruction {}
impl PDOMEventTarget for DOMProcessingInstruction {}
impl IDOMObject for DOMProcessingInstruction {}
impl PNSCopying for DOMProcessingInstruction {}
impl IWebScriptObject for DOMProcessingInstruction {}
impl INSObject for DOMProcessingInstruction {}
impl PNSObject for DOMProcessingInstruction {}
impl IDOMProcessingInstruction for DOMProcessingInstruction {}
pub trait IDOMProcessingInstruction: Sized + std::ops::Deref {
    unsafe fn target(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn sheet(&self) -> DOMStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sheet)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMStyleSheet(pub id);
impl std::ops::Deref for DOMStyleSheet {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMStyleSheet {}
impl DOMStyleSheet {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMStyleSheet").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMStyleSheet {}
impl PNSCopying for DOMStyleSheet {}
impl std::convert::TryFrom<DOMObject> for DOMStyleSheet {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMStyleSheet, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMStyleSheet").unwrap()) };
        if is_kind_of {
            Ok(DOMStyleSheet(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMStyleSheet")
        }
    }
}
impl IWebScriptObject for DOMStyleSheet {}
impl INSObject for DOMStyleSheet {}
impl PNSObject for DOMStyleSheet {}
impl IDOMStyleSheet for DOMStyleSheet {}
pub trait IDOMStyleSheet: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn ownerNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerNode)
    }
    unsafe fn parentStyleSheet(&self) -> DOMStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentStyleSheet)
    }
    unsafe fn href(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, href)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn media(&self) -> DOMMediaList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, media)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMStyleSheetList(pub id);
impl std::ops::Deref for DOMStyleSheetList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMStyleSheetList {}
impl DOMStyleSheetList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMStyleSheetList").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMStyleSheetList {}
impl PNSCopying for DOMStyleSheetList {}
impl std::convert::TryFrom<DOMObject> for DOMStyleSheetList {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMStyleSheetList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMStyleSheetList").unwrap()) };
        if is_kind_of {
            Ok(DOMStyleSheetList(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMStyleSheetList")
        }
    }
}
impl IWebScriptObject for DOMStyleSheetList {}
impl INSObject for DOMStyleSheetList {}
impl PNSObject for DOMStyleSheetList {}
impl IDOMStyleSheetList for DOMStyleSheetList {}
pub trait IDOMStyleSheetList: Sized + std::ops::Deref {
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMMediaList(pub id);
impl std::ops::Deref for DOMMediaList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMMediaList {}
impl DOMMediaList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMMediaList").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMMediaList {}
impl PNSCopying for DOMMediaList {}
impl std::convert::TryFrom<DOMObject> for DOMMediaList {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMMediaList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMMediaList").unwrap()) };
        if is_kind_of {
            Ok(DOMMediaList(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMMediaList")
        }
    }
}
impl IWebScriptObject for DOMMediaList {}
impl INSObject for DOMMediaList {}
impl PNSObject for DOMMediaList {}
impl IDOMMediaList for DOMMediaList {}
pub trait IDOMMediaList: Sized + std::ops::Deref {
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn deleteMedium_(&self, oldMedium: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteMedium : oldMedium)
    }
    unsafe fn appendMedium_(&self, newMedium: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendMedium : newMedium)
    }
    unsafe fn mediaText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaText)
    }
    unsafe fn setMediaText_(&self, mediaText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaText : mediaText)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSRule(pub id);
impl std::ops::Deref for DOMCSSRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSRule {}
impl DOMCSSRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSRule").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMCSSRule {}
impl PNSCopying for DOMCSSRule {}
impl std::convert::TryFrom<DOMObject> for DOMCSSRule {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMCSSRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSRule").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSRule(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMCSSRule")
        }
    }
}
impl IWebScriptObject for DOMCSSRule {}
impl INSObject for DOMCSSRule {}
impl PNSObject for DOMCSSRule {}
impl IDOMCSSRule for DOMCSSRule {}
pub trait IDOMCSSRule: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn cssText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cssText)
    }
    unsafe fn setCssText_(&self, cssText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCssText : cssText)
    }
    unsafe fn parentStyleSheet(&self) -> DOMCSSStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentStyleSheet)
    }
    unsafe fn parentRule(&self) -> DOMCSSRule
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentRule)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSCharsetRule(pub id);
impl std::ops::Deref for DOMCSSCharsetRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSCharsetRule {}
impl DOMCSSCharsetRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSCharsetRule").unwrap(), alloc) })
    }
}
impl IDOMCSSRule for DOMCSSCharsetRule {}
impl From<DOMCSSCharsetRule> for DOMCSSRule {
    fn from(child: DOMCSSCharsetRule) -> DOMCSSRule {
        DOMCSSRule(child.0)
    }
}
impl std::convert::TryFrom<DOMCSSRule> for DOMCSSCharsetRule {
    type Error = &'static str;
    fn try_from(parent: DOMCSSRule) -> Result<DOMCSSCharsetRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSCharsetRule").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSCharsetRule(parent.0))
        } else {
            Err("This DOMCSSRule cannot be downcasted to DOMCSSCharsetRule")
        }
    }
}
impl IDOMObject for DOMCSSCharsetRule {}
impl PNSCopying for DOMCSSCharsetRule {}
impl IWebScriptObject for DOMCSSCharsetRule {}
impl INSObject for DOMCSSCharsetRule {}
impl PNSObject for DOMCSSCharsetRule {}
impl IDOMCSSCharsetRule for DOMCSSCharsetRule {}
pub trait IDOMCSSCharsetRule: Sized + std::ops::Deref {
    unsafe fn encoding(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encoding)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSFontFaceRule(pub id);
impl std::ops::Deref for DOMCSSFontFaceRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSFontFaceRule {}
impl DOMCSSFontFaceRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSFontFaceRule").unwrap(), alloc) })
    }
}
impl IDOMCSSRule for DOMCSSFontFaceRule {}
impl std::convert::TryFrom<DOMCSSRule> for DOMCSSFontFaceRule {
    type Error = &'static str;
    fn try_from(parent: DOMCSSRule) -> Result<DOMCSSFontFaceRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSFontFaceRule").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSFontFaceRule(parent.0))
        } else {
            Err("This DOMCSSRule cannot be downcasted to DOMCSSFontFaceRule")
        }
    }
}
impl IDOMObject for DOMCSSFontFaceRule {}
impl PNSCopying for DOMCSSFontFaceRule {}
impl IWebScriptObject for DOMCSSFontFaceRule {}
impl INSObject for DOMCSSFontFaceRule {}
impl PNSObject for DOMCSSFontFaceRule {}
impl IDOMCSSFontFaceRule for DOMCSSFontFaceRule {}
pub trait IDOMCSSFontFaceRule: Sized + std::ops::Deref {
    unsafe fn style(&self) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSImportRule(pub id);
impl std::ops::Deref for DOMCSSImportRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSImportRule {}
impl DOMCSSImportRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSImportRule").unwrap(), alloc) })
    }
}
impl IDOMCSSRule for DOMCSSImportRule {}
impl std::convert::TryFrom<DOMCSSRule> for DOMCSSImportRule {
    type Error = &'static str;
    fn try_from(parent: DOMCSSRule) -> Result<DOMCSSImportRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSImportRule").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSImportRule(parent.0))
        } else {
            Err("This DOMCSSRule cannot be downcasted to DOMCSSImportRule")
        }
    }
}
impl IDOMObject for DOMCSSImportRule {}
impl PNSCopying for DOMCSSImportRule {}
impl IWebScriptObject for DOMCSSImportRule {}
impl INSObject for DOMCSSImportRule {}
impl PNSObject for DOMCSSImportRule {}
impl IDOMCSSImportRule for DOMCSSImportRule {}
pub trait IDOMCSSImportRule: Sized + std::ops::Deref {
    unsafe fn href(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, href)
    }
    unsafe fn media(&self) -> DOMMediaList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, media)
    }
    unsafe fn styleSheet(&self) -> DOMCSSStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, styleSheet)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSMediaRule(pub id);
impl std::ops::Deref for DOMCSSMediaRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSMediaRule {}
impl DOMCSSMediaRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSMediaRule").unwrap(), alloc) })
    }
}
impl IDOMCSSRule for DOMCSSMediaRule {}
impl std::convert::TryFrom<DOMCSSRule> for DOMCSSMediaRule {
    type Error = &'static str;
    fn try_from(parent: DOMCSSRule) -> Result<DOMCSSMediaRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSMediaRule").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSMediaRule(parent.0))
        } else {
            Err("This DOMCSSRule cannot be downcasted to DOMCSSMediaRule")
        }
    }
}
impl IDOMObject for DOMCSSMediaRule {}
impl PNSCopying for DOMCSSMediaRule {}
impl IWebScriptObject for DOMCSSMediaRule {}
impl INSObject for DOMCSSMediaRule {}
impl PNSObject for DOMCSSMediaRule {}
impl IDOMCSSMediaRule for DOMCSSMediaRule {}
pub trait IDOMCSSMediaRule: Sized + std::ops::Deref {
    unsafe fn insertRule_index_(
        &self,
        rule: NSString,
        index: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertRule : rule, index : index)
    }
    unsafe fn deleteRule_(&self, index: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteRule : index)
    }
    unsafe fn media(&self) -> DOMMediaList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, media)
    }
    unsafe fn cssRules(&self) -> DOMCSSRuleList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cssRules)
    }
}
impl DOMCSSMediaRule_DOMCSSMediaRuleDeprecated for DOMCSSMediaRule {}
pub trait DOMCSSMediaRule_DOMCSSMediaRuleDeprecated: Sized + std::ops::Deref {
    unsafe fn insertRule__(
        &self,
        rule: NSString,
        index: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertRule : rule, index : index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSPageRule(pub id);
impl std::ops::Deref for DOMCSSPageRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSPageRule {}
impl DOMCSSPageRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSPageRule").unwrap(), alloc) })
    }
}
impl IDOMCSSRule for DOMCSSPageRule {}
impl std::convert::TryFrom<DOMCSSRule> for DOMCSSPageRule {
    type Error = &'static str;
    fn try_from(parent: DOMCSSRule) -> Result<DOMCSSPageRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSPageRule").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSPageRule(parent.0))
        } else {
            Err("This DOMCSSRule cannot be downcasted to DOMCSSPageRule")
        }
    }
}
impl IDOMObject for DOMCSSPageRule {}
impl PNSCopying for DOMCSSPageRule {}
impl IWebScriptObject for DOMCSSPageRule {}
impl INSObject for DOMCSSPageRule {}
impl PNSObject for DOMCSSPageRule {}
impl IDOMCSSPageRule for DOMCSSPageRule {}
pub trait IDOMCSSPageRule: Sized + std::ops::Deref {
    unsafe fn selectorText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectorText)
    }
    unsafe fn setSelectorText_(&self, selectorText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectorText : selectorText)
    }
    unsafe fn style(&self) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSValue(pub id);
impl std::ops::Deref for DOMCSSValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSValue {}
impl DOMCSSValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSValue").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMCSSValue {}
impl PNSCopying for DOMCSSValue {}
impl std::convert::TryFrom<DOMObject> for DOMCSSValue {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMCSSValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSValue").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSValue(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMCSSValue")
        }
    }
}
impl IWebScriptObject for DOMCSSValue {}
impl INSObject for DOMCSSValue {}
impl PNSObject for DOMCSSValue {}
impl IDOMCSSValue for DOMCSSValue {}
pub trait IDOMCSSValue: Sized + std::ops::Deref {
    unsafe fn cssText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cssText)
    }
    unsafe fn setCssText_(&self, cssText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCssText : cssText)
    }
    unsafe fn cssValueType(&self) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cssValueType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSPrimitiveValue(pub id);
impl std::ops::Deref for DOMCSSPrimitiveValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSPrimitiveValue {}
impl DOMCSSPrimitiveValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSPrimitiveValue").unwrap(), alloc) })
    }
}
impl IDOMCSSValue for DOMCSSPrimitiveValue {}
impl From<DOMCSSPrimitiveValue> for DOMCSSValue {
    fn from(child: DOMCSSPrimitiveValue) -> DOMCSSValue {
        DOMCSSValue(child.0)
    }
}
impl std::convert::TryFrom<DOMCSSValue> for DOMCSSPrimitiveValue {
    type Error = &'static str;
    fn try_from(parent: DOMCSSValue) -> Result<DOMCSSPrimitiveValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSPrimitiveValue").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSPrimitiveValue(parent.0))
        } else {
            Err("This DOMCSSValue cannot be downcasted to DOMCSSPrimitiveValue")
        }
    }
}
impl IDOMObject for DOMCSSPrimitiveValue {}
impl PNSCopying for DOMCSSPrimitiveValue {}
impl IWebScriptObject for DOMCSSPrimitiveValue {}
impl INSObject for DOMCSSPrimitiveValue {}
impl PNSObject for DOMCSSPrimitiveValue {}
impl IDOMCSSPrimitiveValue for DOMCSSPrimitiveValue {}
pub trait IDOMCSSPrimitiveValue: Sized + std::ops::Deref {
    unsafe fn setFloatValue_floatValue_(&self, unitType: ::std::os::raw::c_ushort, floatValue: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatValue : unitType, floatValue : floatValue)
    }
    unsafe fn getFloatValue_(&self, unitType: ::std::os::raw::c_ushort) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloatValue : unitType)
    }
    unsafe fn setStringValue_stringValue_(
        &self,
        stringType: ::std::os::raw::c_ushort,
        stringValue: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStringValue : stringType, stringValue : stringValue)
    }
    unsafe fn getStringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getStringValue)
    }
    unsafe fn getCounterValue(&self) -> DOMCounter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getCounterValue)
    }
    unsafe fn getRectValue(&self) -> DOMRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getRectValue)
    }
    unsafe fn getRGBColorValue(&self) -> DOMRGBColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getRGBColorValue)
    }
    unsafe fn primitiveType(&self) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveType)
    }
}
impl DOMCSSPrimitiveValue_DOMCSSPrimitiveValueDeprecated for DOMCSSPrimitiveValue {}
pub trait DOMCSSPrimitiveValue_DOMCSSPrimitiveValueDeprecated: Sized + std::ops::Deref {
    unsafe fn setFloatValue__(&self, unitType: ::std::os::raw::c_ushort, floatValue: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatValue : unitType, floatValue : floatValue)
    }
    unsafe fn setStringValue__(&self, stringType: ::std::os::raw::c_ushort, stringValue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStringValue : stringType, stringValue : stringValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSRuleList(pub id);
impl std::ops::Deref for DOMCSSRuleList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSRuleList {}
impl DOMCSSRuleList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSRuleList").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMCSSRuleList {}
impl PNSCopying for DOMCSSRuleList {}
impl std::convert::TryFrom<DOMObject> for DOMCSSRuleList {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMCSSRuleList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSRuleList").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSRuleList(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMCSSRuleList")
        }
    }
}
impl IWebScriptObject for DOMCSSRuleList {}
impl INSObject for DOMCSSRuleList {}
impl PNSObject for DOMCSSRuleList {}
impl IDOMCSSRuleList for DOMCSSRuleList {}
pub trait IDOMCSSRuleList: Sized + std::ops::Deref {
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMCSSRule
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSStyleDeclaration(pub id);
impl std::ops::Deref for DOMCSSStyleDeclaration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSStyleDeclaration {}
impl DOMCSSStyleDeclaration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSStyleDeclaration").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMCSSStyleDeclaration {}
impl PNSCopying for DOMCSSStyleDeclaration {}
impl std::convert::TryFrom<DOMObject> for DOMCSSStyleDeclaration {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMCSSStyleDeclaration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSStyleDeclaration").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSStyleDeclaration(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMCSSStyleDeclaration")
        }
    }
}
impl IWebScriptObject for DOMCSSStyleDeclaration {}
impl INSObject for DOMCSSStyleDeclaration {}
impl PNSObject for DOMCSSStyleDeclaration {}
impl IDOMCSSStyleDeclaration for DOMCSSStyleDeclaration {}
pub trait IDOMCSSStyleDeclaration: Sized + std::ops::Deref {
    unsafe fn getPropertyValue_(&self, propertyName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPropertyValue : propertyName)
    }
    unsafe fn getPropertyCSSValue_(&self, propertyName: NSString) -> DOMCSSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPropertyCSSValue : propertyName)
    }
    unsafe fn removeProperty_(&self, propertyName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeProperty : propertyName)
    }
    unsafe fn getPropertyPriority_(&self, propertyName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPropertyPriority : propertyName)
    }
    unsafe fn setProperty_value_priority_(
        &self,
        propertyName: NSString,
        value: NSString,
        priority: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperty : propertyName, value : value, priority : priority)
    }
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn getPropertyShorthand_(&self, propertyName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPropertyShorthand : propertyName)
    }
    unsafe fn isPropertyImplicit_(&self, propertyName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isPropertyImplicit : propertyName)
    }
    unsafe fn cssText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cssText)
    }
    unsafe fn setCssText_(&self, cssText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCssText : cssText)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn parentRule(&self) -> DOMCSSRule
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentRule)
    }
}
impl DOMCSSStyleDeclaration_DOMCSSStyleDeclarationDeprecated for DOMCSSStyleDeclaration {}
pub trait DOMCSSStyleDeclaration_DOMCSSStyleDeclarationDeprecated: Sized + std::ops::Deref {
    unsafe fn setProperty___(&self, propertyName: NSString, value: NSString, priority: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperty : propertyName, value : value, priority : priority)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSStyleRule(pub id);
impl std::ops::Deref for DOMCSSStyleRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSStyleRule {}
impl DOMCSSStyleRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSStyleRule").unwrap(), alloc) })
    }
}
impl IDOMCSSRule for DOMCSSStyleRule {}
impl std::convert::TryFrom<DOMCSSRule> for DOMCSSStyleRule {
    type Error = &'static str;
    fn try_from(parent: DOMCSSRule) -> Result<DOMCSSStyleRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSStyleRule").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSStyleRule(parent.0))
        } else {
            Err("This DOMCSSRule cannot be downcasted to DOMCSSStyleRule")
        }
    }
}
impl IDOMObject for DOMCSSStyleRule {}
impl PNSCopying for DOMCSSStyleRule {}
impl IWebScriptObject for DOMCSSStyleRule {}
impl INSObject for DOMCSSStyleRule {}
impl PNSObject for DOMCSSStyleRule {}
impl IDOMCSSStyleRule for DOMCSSStyleRule {}
pub trait IDOMCSSStyleRule: Sized + std::ops::Deref {
    unsafe fn selectorText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectorText)
    }
    unsafe fn setSelectorText_(&self, selectorText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectorText : selectorText)
    }
    unsafe fn style(&self) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSStyleSheet(pub id);
impl std::ops::Deref for DOMCSSStyleSheet {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSStyleSheet {}
impl DOMCSSStyleSheet {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSStyleSheet").unwrap(), alloc) })
    }
}
impl IDOMStyleSheet for DOMCSSStyleSheet {}
impl From<DOMCSSStyleSheet> for DOMStyleSheet {
    fn from(child: DOMCSSStyleSheet) -> DOMStyleSheet {
        DOMStyleSheet(child.0)
    }
}
impl std::convert::TryFrom<DOMStyleSheet> for DOMCSSStyleSheet {
    type Error = &'static str;
    fn try_from(parent: DOMStyleSheet) -> Result<DOMCSSStyleSheet, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSStyleSheet").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSStyleSheet(parent.0))
        } else {
            Err("This DOMStyleSheet cannot be downcasted to DOMCSSStyleSheet")
        }
    }
}
impl IDOMObject for DOMCSSStyleSheet {}
impl PNSCopying for DOMCSSStyleSheet {}
impl IWebScriptObject for DOMCSSStyleSheet {}
impl INSObject for DOMCSSStyleSheet {}
impl PNSObject for DOMCSSStyleSheet {}
impl IDOMCSSStyleSheet for DOMCSSStyleSheet {}
pub trait IDOMCSSStyleSheet: Sized + std::ops::Deref {
    unsafe fn insertRule_index_(
        &self,
        rule: NSString,
        index: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertRule : rule, index : index)
    }
    unsafe fn deleteRule_(&self, index: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteRule : index)
    }
    unsafe fn addRule_style_index_(
        &self,
        selector: NSString,
        style: NSString,
        index: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRule : selector, style : style, index : index)
    }
    unsafe fn removeRule_(&self, index: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRule : index)
    }
    unsafe fn ownerRule(&self) -> DOMCSSRule
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerRule)
    }
    unsafe fn cssRules(&self) -> DOMCSSRuleList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cssRules)
    }
    unsafe fn rules(&self) -> DOMCSSRuleList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rules)
    }
}
impl DOMCSSStyleSheet_DOMCSSStyleSheetDeprecated for DOMCSSStyleSheet {}
pub trait DOMCSSStyleSheet_DOMCSSStyleSheetDeprecated: Sized + std::ops::Deref {
    unsafe fn insertRule__(
        &self,
        rule: NSString,
        index: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertRule : rule, index : index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSUnknownRule(pub id);
impl std::ops::Deref for DOMCSSUnknownRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSUnknownRule {}
impl DOMCSSUnknownRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSUnknownRule").unwrap(), alloc) })
    }
}
impl IDOMCSSRule for DOMCSSUnknownRule {}
impl std::convert::TryFrom<DOMCSSRule> for DOMCSSUnknownRule {
    type Error = &'static str;
    fn try_from(parent: DOMCSSRule) -> Result<DOMCSSUnknownRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSUnknownRule").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSUnknownRule(parent.0))
        } else {
            Err("This DOMCSSRule cannot be downcasted to DOMCSSUnknownRule")
        }
    }
}
impl IDOMObject for DOMCSSUnknownRule {}
impl PNSCopying for DOMCSSUnknownRule {}
impl IWebScriptObject for DOMCSSUnknownRule {}
impl INSObject for DOMCSSUnknownRule {}
impl PNSObject for DOMCSSUnknownRule {}
impl IDOMCSSUnknownRule for DOMCSSUnknownRule {}
pub trait IDOMCSSUnknownRule: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCSSValueList(pub id);
impl std::ops::Deref for DOMCSSValueList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCSSValueList {}
impl DOMCSSValueList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCSSValueList").unwrap(), alloc) })
    }
}
impl IDOMCSSValue for DOMCSSValueList {}
impl std::convert::TryFrom<DOMCSSValue> for DOMCSSValueList {
    type Error = &'static str;
    fn try_from(parent: DOMCSSValue) -> Result<DOMCSSValueList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCSSValueList").unwrap()) };
        if is_kind_of {
            Ok(DOMCSSValueList(parent.0))
        } else {
            Err("This DOMCSSValue cannot be downcasted to DOMCSSValueList")
        }
    }
}
impl IDOMObject for DOMCSSValueList {}
impl PNSCopying for DOMCSSValueList {}
impl IWebScriptObject for DOMCSSValueList {}
impl INSObject for DOMCSSValueList {}
impl PNSObject for DOMCSSValueList {}
impl IDOMCSSValueList for DOMCSSValueList {}
pub trait IDOMCSSValueList: Sized + std::ops::Deref {
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMCSSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMCounter(pub id);
impl std::ops::Deref for DOMCounter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMCounter {}
impl DOMCounter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMCounter").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMCounter {}
impl PNSCopying for DOMCounter {}
impl std::convert::TryFrom<DOMObject> for DOMCounter {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMCounter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMCounter").unwrap()) };
        if is_kind_of {
            Ok(DOMCounter(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMCounter")
        }
    }
}
impl IWebScriptObject for DOMCounter {}
impl INSObject for DOMCounter {}
impl PNSObject for DOMCounter {}
impl IDOMCounter for DOMCounter {}
pub trait IDOMCounter: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn listStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listStyle)
    }
    unsafe fn separator(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, separator)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMRGBColor(pub id);
impl std::ops::Deref for DOMRGBColor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMRGBColor {}
impl DOMRGBColor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMRGBColor").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMRGBColor {}
impl PNSCopying for DOMRGBColor {}
impl std::convert::TryFrom<DOMObject> for DOMRGBColor {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMRGBColor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMRGBColor").unwrap()) };
        if is_kind_of {
            Ok(DOMRGBColor(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMRGBColor")
        }
    }
}
impl IWebScriptObject for DOMRGBColor {}
impl INSObject for DOMRGBColor {}
impl PNSObject for DOMRGBColor {}
impl IDOMRGBColor for DOMRGBColor {}
pub trait IDOMRGBColor: Sized + std::ops::Deref {
    unsafe fn red(&self) -> DOMCSSPrimitiveValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, red)
    }
    unsafe fn green(&self) -> DOMCSSPrimitiveValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, green)
    }
    unsafe fn blue(&self) -> DOMCSSPrimitiveValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blue)
    }
    unsafe fn alpha(&self) -> DOMCSSPrimitiveValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alpha)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMRect(pub id);
impl std::ops::Deref for DOMRect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMRect {}
impl DOMRect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMRect").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMRect {}
impl PNSCopying for DOMRect {}
impl std::convert::TryFrom<DOMObject> for DOMRect {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMRect, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMRect").unwrap()) };
        if is_kind_of {
            Ok(DOMRect(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMRect")
        }
    }
}
impl IWebScriptObject for DOMRect {}
impl INSObject for DOMRect {}
impl PNSObject for DOMRect {}
impl IDOMRect for DOMRect {}
pub trait IDOMRect: Sized + std::ops::Deref {
    unsafe fn top(&self) -> DOMCSSPrimitiveValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, top)
    }
    unsafe fn right(&self) -> DOMCSSPrimitiveValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, right)
    }
    unsafe fn bottom(&self) -> DOMCSSPrimitiveValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottom)
    }
    unsafe fn left(&self) -> DOMCSSPrimitiveValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, left)
    }
}
impl DOMCSSStyleDeclaration_DOMCSS2Properties for DOMCSSStyleDeclaration {}
pub trait DOMCSSStyleDeclaration_DOMCSS2Properties: Sized + std::ops::Deref {
    unsafe fn azimuth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, azimuth)
    }
    unsafe fn setAzimuth_(&self, azimuth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAzimuth : azimuth)
    }
    unsafe fn background(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, background)
    }
    unsafe fn setBackground_(&self, background: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackground : background)
    }
    unsafe fn backgroundAttachment(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundAttachment)
    }
    unsafe fn setBackgroundAttachment_(&self, backgroundAttachment: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundAttachment : backgroundAttachment)
    }
    unsafe fn backgroundColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, backgroundColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : backgroundColor)
    }
    unsafe fn backgroundImage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundImage)
    }
    unsafe fn setBackgroundImage_(&self, backgroundImage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundImage : backgroundImage)
    }
    unsafe fn backgroundPosition(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundPosition)
    }
    unsafe fn setBackgroundPosition_(&self, backgroundPosition: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundPosition : backgroundPosition)
    }
    unsafe fn backgroundRepeat(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundRepeat)
    }
    unsafe fn setBackgroundRepeat_(&self, backgroundRepeat: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundRepeat : backgroundRepeat)
    }
    unsafe fn border(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, border)
    }
    unsafe fn setBorder_(&self, border: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorder : border)
    }
    unsafe fn borderCollapse(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderCollapse)
    }
    unsafe fn setBorderCollapse_(&self, borderCollapse: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderCollapse : borderCollapse)
    }
    unsafe fn borderColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderColor)
    }
    unsafe fn setBorderColor_(&self, borderColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderColor : borderColor)
    }
    unsafe fn borderSpacing(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderSpacing)
    }
    unsafe fn setBorderSpacing_(&self, borderSpacing: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderSpacing : borderSpacing)
    }
    unsafe fn borderStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderStyle)
    }
    unsafe fn setBorderStyle_(&self, borderStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderStyle : borderStyle)
    }
    unsafe fn borderTop(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderTop)
    }
    unsafe fn setBorderTop_(&self, borderTop: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderTop : borderTop)
    }
    unsafe fn borderRight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderRight)
    }
    unsafe fn setBorderRight_(&self, borderRight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderRight : borderRight)
    }
    unsafe fn borderBottom(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderBottom)
    }
    unsafe fn setBorderBottom_(&self, borderBottom: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderBottom : borderBottom)
    }
    unsafe fn borderLeft(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderLeft)
    }
    unsafe fn setBorderLeft_(&self, borderLeft: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderLeft : borderLeft)
    }
    unsafe fn borderTopColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderTopColor)
    }
    unsafe fn setBorderTopColor_(&self, borderTopColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderTopColor : borderTopColor)
    }
    unsafe fn borderRightColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderRightColor)
    }
    unsafe fn setBorderRightColor_(&self, borderRightColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderRightColor : borderRightColor)
    }
    unsafe fn borderBottomColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderBottomColor)
    }
    unsafe fn setBorderBottomColor_(&self, borderBottomColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderBottomColor : borderBottomColor)
    }
    unsafe fn borderLeftColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderLeftColor)
    }
    unsafe fn setBorderLeftColor_(&self, borderLeftColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderLeftColor : borderLeftColor)
    }
    unsafe fn borderTopStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderTopStyle)
    }
    unsafe fn setBorderTopStyle_(&self, borderTopStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderTopStyle : borderTopStyle)
    }
    unsafe fn borderRightStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderRightStyle)
    }
    unsafe fn setBorderRightStyle_(&self, borderRightStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderRightStyle : borderRightStyle)
    }
    unsafe fn borderBottomStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderBottomStyle)
    }
    unsafe fn setBorderBottomStyle_(&self, borderBottomStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderBottomStyle : borderBottomStyle)
    }
    unsafe fn borderLeftStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderLeftStyle)
    }
    unsafe fn setBorderLeftStyle_(&self, borderLeftStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderLeftStyle : borderLeftStyle)
    }
    unsafe fn borderTopWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderTopWidth)
    }
    unsafe fn setBorderTopWidth_(&self, borderTopWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderTopWidth : borderTopWidth)
    }
    unsafe fn borderRightWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderRightWidth)
    }
    unsafe fn setBorderRightWidth_(&self, borderRightWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderRightWidth : borderRightWidth)
    }
    unsafe fn borderBottomWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderBottomWidth)
    }
    unsafe fn setBorderBottomWidth_(&self, borderBottomWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderBottomWidth : borderBottomWidth)
    }
    unsafe fn borderLeftWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderLeftWidth)
    }
    unsafe fn setBorderLeftWidth_(&self, borderLeftWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderLeftWidth : borderLeftWidth)
    }
    unsafe fn borderWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderWidth)
    }
    unsafe fn setBorderWidth_(&self, borderWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderWidth : borderWidth)
    }
    unsafe fn bottom(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottom)
    }
    unsafe fn setBottom_(&self, bottom: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottom : bottom)
    }
    unsafe fn captionSide(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captionSide)
    }
    unsafe fn setCaptionSide_(&self, captionSide: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaptionSide : captionSide)
    }
    unsafe fn clear(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clear)
    }
    unsafe fn setClear_(&self, clear: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClear : clear)
    }
    unsafe fn clip(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clip)
    }
    unsafe fn setClip_(&self, clip: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClip : clip)
    }
    unsafe fn color(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn content(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, content)
    }
    unsafe fn setContent_(&self, content: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContent : content)
    }
    unsafe fn counterIncrement(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, counterIncrement)
    }
    unsafe fn setCounterIncrement_(&self, counterIncrement: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCounterIncrement : counterIncrement)
    }
    unsafe fn counterReset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, counterReset)
    }
    unsafe fn setCounterReset_(&self, counterReset: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCounterReset : counterReset)
    }
    unsafe fn cue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cue)
    }
    unsafe fn setCue_(&self, cue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCue : cue)
    }
    unsafe fn cueAfter(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cueAfter)
    }
    unsafe fn setCueAfter_(&self, cueAfter: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCueAfter : cueAfter)
    }
    unsafe fn cueBefore(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cueBefore)
    }
    unsafe fn setCueBefore_(&self, cueBefore: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCueBefore : cueBefore)
    }
    unsafe fn cursor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursor)
    }
    unsafe fn setCursor_(&self, cursor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCursor : cursor)
    }
    unsafe fn direction(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
    unsafe fn setDirection_(&self, direction: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDirection : direction)
    }
    unsafe fn display(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, display)
    }
    unsafe fn setDisplay_(&self, display: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplay : display)
    }
    unsafe fn elevation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elevation)
    }
    unsafe fn setElevation_(&self, elevation: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElevation : elevation)
    }
    unsafe fn emptyCells(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emptyCells)
    }
    unsafe fn setEmptyCells_(&self, emptyCells: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmptyCells : emptyCells)
    }
    unsafe fn cssFloat(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cssFloat)
    }
    unsafe fn setCssFloat_(&self, cssFloat: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCssFloat : cssFloat)
    }
    unsafe fn font(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, font)
    }
    unsafe fn setFont_(&self, font: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFont : font)
    }
    unsafe fn fontFamily(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontFamily)
    }
    unsafe fn setFontFamily_(&self, fontFamily: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontFamily : fontFamily)
    }
    unsafe fn fontSize(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontSize)
    }
    unsafe fn setFontSize_(&self, fontSize: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontSize : fontSize)
    }
    unsafe fn fontSizeAdjust(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontSizeAdjust)
    }
    unsafe fn setFontSizeAdjust_(&self, fontSizeAdjust: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontSizeAdjust : fontSizeAdjust)
    }
    unsafe fn fontStretch(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontStretch)
    }
    unsafe fn setFontStretch_(&self, fontStretch: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontStretch : fontStretch)
    }
    unsafe fn fontStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontStyle)
    }
    unsafe fn setFontStyle_(&self, fontStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontStyle : fontStyle)
    }
    unsafe fn fontVariant(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontVariant)
    }
    unsafe fn setFontVariant_(&self, fontVariant: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontVariant : fontVariant)
    }
    unsafe fn fontWeight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontWeight)
    }
    unsafe fn setFontWeight_(&self, fontWeight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontWeight : fontWeight)
    }
    unsafe fn height(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn left(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, left)
    }
    unsafe fn setLeft_(&self, left: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeft : left)
    }
    unsafe fn letterSpacing(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, letterSpacing)
    }
    unsafe fn setLetterSpacing_(&self, letterSpacing: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLetterSpacing : letterSpacing)
    }
    unsafe fn lineHeight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineHeight)
    }
    unsafe fn setLineHeight_(&self, lineHeight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineHeight : lineHeight)
    }
    unsafe fn listStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listStyle)
    }
    unsafe fn setListStyle_(&self, listStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListStyle : listStyle)
    }
    unsafe fn listStyleImage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listStyleImage)
    }
    unsafe fn setListStyleImage_(&self, listStyleImage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListStyleImage : listStyleImage)
    }
    unsafe fn listStylePosition(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listStylePosition)
    }
    unsafe fn setListStylePosition_(&self, listStylePosition: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListStylePosition : listStylePosition)
    }
    unsafe fn listStyleType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listStyleType)
    }
    unsafe fn setListStyleType_(&self, listStyleType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListStyleType : listStyleType)
    }
    unsafe fn margin(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, margin)
    }
    unsafe fn setMargin_(&self, margin: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMargin : margin)
    }
    unsafe fn marginTop(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marginTop)
    }
    unsafe fn setMarginTop_(&self, marginTop: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarginTop : marginTop)
    }
    unsafe fn marginRight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marginRight)
    }
    unsafe fn setMarginRight_(&self, marginRight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarginRight : marginRight)
    }
    unsafe fn marginBottom(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marginBottom)
    }
    unsafe fn setMarginBottom_(&self, marginBottom: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarginBottom : marginBottom)
    }
    unsafe fn marginLeft(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marginLeft)
    }
    unsafe fn setMarginLeft_(&self, marginLeft: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarginLeft : marginLeft)
    }
    unsafe fn markerOffset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, markerOffset)
    }
    unsafe fn setMarkerOffset_(&self, markerOffset: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarkerOffset : markerOffset)
    }
    unsafe fn marks(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marks)
    }
    unsafe fn setMarks_(&self, marks: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarks : marks)
    }
    unsafe fn maxHeight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxHeight)
    }
    unsafe fn setMaxHeight_(&self, maxHeight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxHeight : maxHeight)
    }
    unsafe fn maxWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxWidth)
    }
    unsafe fn setMaxWidth_(&self, maxWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxWidth : maxWidth)
    }
    unsafe fn minHeight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minHeight)
    }
    unsafe fn setMinHeight_(&self, minHeight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinHeight : minHeight)
    }
    unsafe fn minWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minWidth)
    }
    unsafe fn setMinWidth_(&self, minWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinWidth : minWidth)
    }
    unsafe fn orphans(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orphans)
    }
    unsafe fn setOrphans_(&self, orphans: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrphans : orphans)
    }
    unsafe fn outline(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outline)
    }
    unsafe fn setOutline_(&self, outline: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutline : outline)
    }
    unsafe fn outlineColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outlineColor)
    }
    unsafe fn setOutlineColor_(&self, outlineColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutlineColor : outlineColor)
    }
    unsafe fn outlineStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outlineStyle)
    }
    unsafe fn setOutlineStyle_(&self, outlineStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutlineStyle : outlineStyle)
    }
    unsafe fn outlineWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outlineWidth)
    }
    unsafe fn setOutlineWidth_(&self, outlineWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutlineWidth : outlineWidth)
    }
    unsafe fn overflow(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overflow)
    }
    unsafe fn setOverflow_(&self, overflow: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverflow : overflow)
    }
    unsafe fn padding(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, padding)
    }
    unsafe fn setPadding_(&self, padding: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPadding : padding)
    }
    unsafe fn paddingTop(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingTop)
    }
    unsafe fn setPaddingTop_(&self, paddingTop: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingTop : paddingTop)
    }
    unsafe fn paddingRight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingRight)
    }
    unsafe fn setPaddingRight_(&self, paddingRight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingRight : paddingRight)
    }
    unsafe fn paddingBottom(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingBottom)
    }
    unsafe fn setPaddingBottom_(&self, paddingBottom: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingBottom : paddingBottom)
    }
    unsafe fn paddingLeft(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingLeft)
    }
    unsafe fn setPaddingLeft_(&self, paddingLeft: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingLeft : paddingLeft)
    }
    unsafe fn page(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, page)
    }
    unsafe fn setPage_(&self, page: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPage : page)
    }
    unsafe fn pageBreakAfter(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageBreakAfter)
    }
    unsafe fn setPageBreakAfter_(&self, pageBreakAfter: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageBreakAfter : pageBreakAfter)
    }
    unsafe fn pageBreakBefore(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageBreakBefore)
    }
    unsafe fn setPageBreakBefore_(&self, pageBreakBefore: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageBreakBefore : pageBreakBefore)
    }
    unsafe fn pageBreakInside(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageBreakInside)
    }
    unsafe fn setPageBreakInside_(&self, pageBreakInside: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageBreakInside : pageBreakInside)
    }
    unsafe fn pause(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn setPause_(&self, pause: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPause : pause)
    }
    unsafe fn pauseAfter(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pauseAfter)
    }
    unsafe fn setPauseAfter_(&self, pauseAfter: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPauseAfter : pauseAfter)
    }
    unsafe fn pauseBefore(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pauseBefore)
    }
    unsafe fn setPauseBefore_(&self, pauseBefore: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPauseBefore : pauseBefore)
    }
    unsafe fn pitch(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn setPitch_(&self, pitch: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitch : pitch)
    }
    unsafe fn pitchRange(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitchRange)
    }
    unsafe fn setPitchRange_(&self, pitchRange: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitchRange : pitchRange)
    }
    unsafe fn playDuring(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playDuring)
    }
    unsafe fn setPlayDuring_(&self, playDuring: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayDuring : playDuring)
    }
    unsafe fn position(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn quotes(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quotes)
    }
    unsafe fn setQuotes_(&self, quotes: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuotes : quotes)
    }
    unsafe fn richness(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, richness)
    }
    unsafe fn setRichness_(&self, richness: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRichness : richness)
    }
    unsafe fn right(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, right)
    }
    unsafe fn setRight_(&self, right: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRight : right)
    }
    unsafe fn size(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn speak(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speak)
    }
    unsafe fn setSpeak_(&self, speak: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeak : speak)
    }
    unsafe fn speakHeader(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speakHeader)
    }
    unsafe fn setSpeakHeader_(&self, speakHeader: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeakHeader : speakHeader)
    }
    unsafe fn speakNumeral(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speakNumeral)
    }
    unsafe fn setSpeakNumeral_(&self, speakNumeral: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeakNumeral : speakNumeral)
    }
    unsafe fn speakPunctuation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speakPunctuation)
    }
    unsafe fn setSpeakPunctuation_(&self, speakPunctuation: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeakPunctuation : speakPunctuation)
    }
    unsafe fn speechRate(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechRate)
    }
    unsafe fn setSpeechRate_(&self, speechRate: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeechRate : speechRate)
    }
    unsafe fn stress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stress)
    }
    unsafe fn setStress_(&self, stress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStress : stress)
    }
    unsafe fn tableLayout(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tableLayout)
    }
    unsafe fn setTableLayout_(&self, tableLayout: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTableLayout : tableLayout)
    }
    unsafe fn textAlign(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textAlign)
    }
    unsafe fn setTextAlign_(&self, textAlign: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextAlign : textAlign)
    }
    unsafe fn textDecoration(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textDecoration)
    }
    unsafe fn setTextDecoration_(&self, textDecoration: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextDecoration : textDecoration)
    }
    unsafe fn textIndent(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textIndent)
    }
    unsafe fn setTextIndent_(&self, textIndent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextIndent : textIndent)
    }
    unsafe fn textShadow(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textShadow)
    }
    unsafe fn setTextShadow_(&self, textShadow: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextShadow : textShadow)
    }
    unsafe fn textTransform(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textTransform)
    }
    unsafe fn setTextTransform_(&self, textTransform: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextTransform : textTransform)
    }
    unsafe fn top(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, top)
    }
    unsafe fn setTop_(&self, top: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTop : top)
    }
    unsafe fn unicodeBidi(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unicodeBidi)
    }
    unsafe fn setUnicodeBidi_(&self, unicodeBidi: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnicodeBidi : unicodeBidi)
    }
    unsafe fn verticalAlign(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalAlign)
    }
    unsafe fn setVerticalAlign_(&self, verticalAlign: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVerticalAlign : verticalAlign)
    }
    unsafe fn visibility(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibility)
    }
    unsafe fn setVisibility_(&self, visibility: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibility : visibility)
    }
    unsafe fn voiceFamily(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voiceFamily)
    }
    unsafe fn setVoiceFamily_(&self, voiceFamily: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoiceFamily : voiceFamily)
    }
    unsafe fn volume(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volume)
    }
    unsafe fn setVolume_(&self, volume: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolume : volume)
    }
    unsafe fn whiteSpace(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, whiteSpace)
    }
    unsafe fn setWhiteSpace_(&self, whiteSpace: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWhiteSpace : whiteSpace)
    }
    unsafe fn widows(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widows)
    }
    unsafe fn setWidows_(&self, widows: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidows : widows)
    }
    unsafe fn width(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn wordSpacing(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wordSpacing)
    }
    unsafe fn setWordSpacing_(&self, wordSpacing: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWordSpacing : wordSpacing)
    }
    unsafe fn zIndex(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zIndex)
    }
    unsafe fn setZIndex_(&self, zIndex: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZIndex : zIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMBlob(pub id);
impl std::ops::Deref for DOMBlob {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMBlob {}
impl DOMBlob {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMBlob").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMBlob {}
impl PNSCopying for DOMBlob {}
impl std::convert::TryFrom<DOMObject> for DOMBlob {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMBlob, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMBlob").unwrap()) };
        if is_kind_of {
            Ok(DOMBlob(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMBlob")
        }
    }
}
impl IWebScriptObject for DOMBlob {}
impl INSObject for DOMBlob {}
impl PNSObject for DOMBlob {}
impl IDOMBlob for DOMBlob {}
pub trait IDOMBlob: Sized + std::ops::Deref {
    unsafe fn size(&self) -> ::std::os::raw::c_ulonglong
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMFile(pub id);
impl std::ops::Deref for DOMFile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMFile {}
impl DOMFile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMFile").unwrap(), alloc) })
    }
}
impl IDOMBlob for DOMFile {}
impl From<DOMFile> for DOMBlob {
    fn from(child: DOMFile) -> DOMBlob {
        DOMBlob(child.0)
    }
}
impl std::convert::TryFrom<DOMBlob> for DOMFile {
    type Error = &'static str;
    fn try_from(parent: DOMBlob) -> Result<DOMFile, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMFile").unwrap()) };
        if is_kind_of {
            Ok(DOMFile(parent.0))
        } else {
            Err("This DOMBlob cannot be downcasted to DOMFile")
        }
    }
}
impl IDOMObject for DOMFile {}
impl PNSCopying for DOMFile {}
impl IWebScriptObject for DOMFile {}
impl INSObject for DOMFile {}
impl PNSObject for DOMFile {}
impl IDOMFile for DOMFile {}
pub trait IDOMFile: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMFileList(pub id);
impl std::ops::Deref for DOMFileList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMFileList {}
impl DOMFileList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMFileList").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMFileList {}
impl PNSCopying for DOMFileList {}
impl std::convert::TryFrom<DOMObject> for DOMFileList {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMFileList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMFileList").unwrap()) };
        if is_kind_of {
            Ok(DOMFileList(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMFileList")
        }
    }
}
impl IWebScriptObject for DOMFileList {}
impl INSObject for DOMFileList {}
impl PNSObject for DOMFileList {}
impl IDOMFileList for DOMFileList {}
pub trait IDOMFileList: Sized + std::ops::Deref {
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLElement(pub id);
impl std::ops::Deref for DOMHTMLElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLElement {}
impl DOMHTMLElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLElement").unwrap(), alloc) })
    }
}
impl IDOMElement for DOMHTMLElement {}
impl From<DOMHTMLElement> for DOMElement {
    fn from(child: DOMHTMLElement) -> DOMElement {
        DOMElement(child.0)
    }
}
impl std::convert::TryFrom<DOMElement> for DOMHTMLElement {
    type Error = &'static str;
    fn try_from(parent: DOMElement) -> Result<DOMHTMLElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLElement(parent.0))
        } else {
            Err("This DOMElement cannot be downcasted to DOMHTMLElement")
        }
    }
}
impl IDOMNode for DOMHTMLElement {}
impl PDOMEventTarget for DOMHTMLElement {}
impl IDOMObject for DOMHTMLElement {}
impl PNSCopying for DOMHTMLElement {}
impl IWebScriptObject for DOMHTMLElement {}
impl INSObject for DOMHTMLElement {}
impl PNSObject for DOMHTMLElement {}
impl IDOMHTMLElement for DOMHTMLElement {}
pub trait IDOMHTMLElement: Sized + std::ops::Deref {
    unsafe fn click(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, click)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn lang(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lang)
    }
    unsafe fn setLang_(&self, lang: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLang : lang)
    }
    unsafe fn dir(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dir)
    }
    unsafe fn setDir_(&self, dir: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDir : dir)
    }
    unsafe fn tabIndex(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabIndex)
    }
    unsafe fn setTabIndex_(&self, tabIndex: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTabIndex : tabIndex)
    }
    unsafe fn accessKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessKey)
    }
    unsafe fn setAccessKey_(&self, accessKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessKey : accessKey)
    }
    unsafe fn innerText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, innerText)
    }
    unsafe fn setInnerText_(&self, innerText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInnerText : innerText)
    }
    unsafe fn outerText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerText)
    }
    unsafe fn setOuterText_(&self, outerText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOuterText : outerText)
    }
    unsafe fn contentEditable(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentEditable)
    }
    unsafe fn setContentEditable_(&self, contentEditable: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentEditable : contentEditable)
    }
    unsafe fn isContentEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContentEditable)
    }
    unsafe fn idName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, idName)
    }
    unsafe fn setIdName_(&self, idName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdName : idName)
    }
    unsafe fn children(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, children)
    }
    unsafe fn titleDisplayString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleDisplayString)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLAnchorElement(pub id);
impl std::ops::Deref for DOMHTMLAnchorElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLAnchorElement {}
impl DOMHTMLAnchorElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLAnchorElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLAnchorElement {}
impl From<DOMHTMLAnchorElement> for DOMHTMLElement {
    fn from(child: DOMHTMLAnchorElement) -> DOMHTMLElement {
        DOMHTMLElement(child.0)
    }
}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLAnchorElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLAnchorElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLAnchorElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLAnchorElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLAnchorElement")
        }
    }
}
impl IDOMElement for DOMHTMLAnchorElement {}
impl IDOMNode for DOMHTMLAnchorElement {}
impl PDOMEventTarget for DOMHTMLAnchorElement {}
impl IDOMObject for DOMHTMLAnchorElement {}
impl PNSCopying for DOMHTMLAnchorElement {}
impl IWebScriptObject for DOMHTMLAnchorElement {}
impl INSObject for DOMHTMLAnchorElement {}
impl PNSObject for DOMHTMLAnchorElement {}
impl IDOMHTMLAnchorElement for DOMHTMLAnchorElement {}
pub trait IDOMHTMLAnchorElement: Sized + std::ops::Deref {
    unsafe fn charset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charset)
    }
    unsafe fn setCharset_(&self, charset: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharset : charset)
    }
    unsafe fn coords(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coords)
    }
    unsafe fn setCoords_(&self, coords: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCoords : coords)
    }
    unsafe fn hreflang(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hreflang)
    }
    unsafe fn setHreflang_(&self, hreflang: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHreflang : hreflang)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn rel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rel)
    }
    unsafe fn setRel_(&self, rel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRel : rel)
    }
    unsafe fn rev(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rev)
    }
    unsafe fn setRev_(&self, rev: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRev : rev)
    }
    unsafe fn shape(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn setShape_(&self, shape: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShape : shape)
    }
    unsafe fn target(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn accessKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessKey)
    }
    unsafe fn setAccessKey_(&self, accessKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessKey : accessKey)
    }
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn absoluteLinkURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteLinkURL)
    }
    unsafe fn href(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, href)
    }
    unsafe fn setHref_(&self, href: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHref : href)
    }
    unsafe fn protocol(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocol)
    }
    unsafe fn host(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, host)
    }
    unsafe fn hostname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostname)
    }
    unsafe fn port(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, port)
    }
    unsafe fn pathname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathname)
    }
    unsafe fn search(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, search)
    }
    unsafe fn hashName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hashName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLAppletElement(pub id);
impl std::ops::Deref for DOMHTMLAppletElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLAppletElement {}
impl DOMHTMLAppletElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLAppletElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLAppletElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLAppletElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLAppletElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLAppletElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLAppletElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLAppletElement")
        }
    }
}
impl IDOMElement for DOMHTMLAppletElement {}
impl IDOMNode for DOMHTMLAppletElement {}
impl PDOMEventTarget for DOMHTMLAppletElement {}
impl IDOMObject for DOMHTMLAppletElement {}
impl PNSCopying for DOMHTMLAppletElement {}
impl IWebScriptObject for DOMHTMLAppletElement {}
impl INSObject for DOMHTMLAppletElement {}
impl PNSObject for DOMHTMLAppletElement {}
impl IDOMHTMLAppletElement for DOMHTMLAppletElement {}
pub trait IDOMHTMLAppletElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn alt(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alt)
    }
    unsafe fn setAlt_(&self, alt: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlt : alt)
    }
    unsafe fn archive(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, archive)
    }
    unsafe fn setArchive_(&self, archive: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArchive : archive)
    }
    unsafe fn code(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, code)
    }
    unsafe fn setCode_(&self, code: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCode : code)
    }
    unsafe fn codeBase(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, codeBase)
    }
    unsafe fn setCodeBase_(&self, codeBase: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCodeBase : codeBase)
    }
    unsafe fn height(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn hspace(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hspace)
    }
    unsafe fn setHspace_(&self, hspace: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHspace : hspace)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn object(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, object)
    }
    unsafe fn setObject_(&self, object: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : object)
    }
    unsafe fn vspace(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vspace)
    }
    unsafe fn setVspace_(&self, vspace: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVspace : vspace)
    }
    unsafe fn width(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLAreaElement(pub id);
impl std::ops::Deref for DOMHTMLAreaElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLAreaElement {}
impl DOMHTMLAreaElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLAreaElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLAreaElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLAreaElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLAreaElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLAreaElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLAreaElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLAreaElement")
        }
    }
}
impl IDOMElement for DOMHTMLAreaElement {}
impl IDOMNode for DOMHTMLAreaElement {}
impl PDOMEventTarget for DOMHTMLAreaElement {}
impl IDOMObject for DOMHTMLAreaElement {}
impl PNSCopying for DOMHTMLAreaElement {}
impl IWebScriptObject for DOMHTMLAreaElement {}
impl INSObject for DOMHTMLAreaElement {}
impl PNSObject for DOMHTMLAreaElement {}
impl IDOMHTMLAreaElement for DOMHTMLAreaElement {}
pub trait IDOMHTMLAreaElement: Sized + std::ops::Deref {
    unsafe fn alt(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alt)
    }
    unsafe fn setAlt_(&self, alt: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlt : alt)
    }
    unsafe fn coords(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coords)
    }
    unsafe fn setCoords_(&self, coords: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCoords : coords)
    }
    unsafe fn noHref(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noHref)
    }
    unsafe fn setNoHref_(&self, noHref: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNoHref : noHref)
    }
    unsafe fn shape(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn setShape_(&self, shape: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShape : shape)
    }
    unsafe fn target(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn accessKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessKey)
    }
    unsafe fn setAccessKey_(&self, accessKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessKey : accessKey)
    }
    unsafe fn absoluteLinkURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteLinkURL)
    }
    unsafe fn href(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, href)
    }
    unsafe fn setHref_(&self, href: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHref : href)
    }
    unsafe fn protocol(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocol)
    }
    unsafe fn host(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, host)
    }
    unsafe fn hostname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostname)
    }
    unsafe fn port(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, port)
    }
    unsafe fn pathname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathname)
    }
    unsafe fn search(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, search)
    }
    unsafe fn hashName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hashName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLBRElement(pub id);
impl std::ops::Deref for DOMHTMLBRElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLBRElement {}
impl DOMHTMLBRElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLBRElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLBRElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLBRElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLBRElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLBRElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLBRElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLBRElement")
        }
    }
}
impl IDOMElement for DOMHTMLBRElement {}
impl IDOMNode for DOMHTMLBRElement {}
impl PDOMEventTarget for DOMHTMLBRElement {}
impl IDOMObject for DOMHTMLBRElement {}
impl PNSCopying for DOMHTMLBRElement {}
impl IWebScriptObject for DOMHTMLBRElement {}
impl INSObject for DOMHTMLBRElement {}
impl PNSObject for DOMHTMLBRElement {}
impl IDOMHTMLBRElement for DOMHTMLBRElement {}
pub trait IDOMHTMLBRElement: Sized + std::ops::Deref {
    unsafe fn clear(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clear)
    }
    unsafe fn setClear_(&self, clear: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClear : clear)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLBaseElement(pub id);
impl std::ops::Deref for DOMHTMLBaseElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLBaseElement {}
impl DOMHTMLBaseElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLBaseElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLBaseElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLBaseElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLBaseElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLBaseElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLBaseElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLBaseElement")
        }
    }
}
impl IDOMElement for DOMHTMLBaseElement {}
impl IDOMNode for DOMHTMLBaseElement {}
impl PDOMEventTarget for DOMHTMLBaseElement {}
impl IDOMObject for DOMHTMLBaseElement {}
impl PNSCopying for DOMHTMLBaseElement {}
impl IWebScriptObject for DOMHTMLBaseElement {}
impl INSObject for DOMHTMLBaseElement {}
impl PNSObject for DOMHTMLBaseElement {}
impl IDOMHTMLBaseElement for DOMHTMLBaseElement {}
pub trait IDOMHTMLBaseElement: Sized + std::ops::Deref {
    unsafe fn href(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, href)
    }
    unsafe fn setHref_(&self, href: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHref : href)
    }
    unsafe fn target(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLBaseFontElement(pub id);
impl std::ops::Deref for DOMHTMLBaseFontElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLBaseFontElement {}
impl DOMHTMLBaseFontElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLBaseFontElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLBaseFontElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLBaseFontElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLBaseFontElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLBaseFontElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLBaseFontElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLBaseFontElement")
        }
    }
}
impl IDOMElement for DOMHTMLBaseFontElement {}
impl IDOMNode for DOMHTMLBaseFontElement {}
impl PDOMEventTarget for DOMHTMLBaseFontElement {}
impl IDOMObject for DOMHTMLBaseFontElement {}
impl PNSCopying for DOMHTMLBaseFontElement {}
impl IWebScriptObject for DOMHTMLBaseFontElement {}
impl INSObject for DOMHTMLBaseFontElement {}
impl PNSObject for DOMHTMLBaseFontElement {}
impl IDOMHTMLBaseFontElement for DOMHTMLBaseFontElement {}
pub trait IDOMHTMLBaseFontElement: Sized + std::ops::Deref {
    unsafe fn color(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn face(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, face)
    }
    unsafe fn setFace_(&self, face: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFace : face)
    }
    unsafe fn size(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLBodyElement(pub id);
impl std::ops::Deref for DOMHTMLBodyElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLBodyElement {}
impl DOMHTMLBodyElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLBodyElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLBodyElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLBodyElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLBodyElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLBodyElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLBodyElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLBodyElement")
        }
    }
}
impl IDOMElement for DOMHTMLBodyElement {}
impl IDOMNode for DOMHTMLBodyElement {}
impl PDOMEventTarget for DOMHTMLBodyElement {}
impl IDOMObject for DOMHTMLBodyElement {}
impl PNSCopying for DOMHTMLBodyElement {}
impl IWebScriptObject for DOMHTMLBodyElement {}
impl INSObject for DOMHTMLBodyElement {}
impl PNSObject for DOMHTMLBodyElement {}
impl IDOMHTMLBodyElement for DOMHTMLBodyElement {}
pub trait IDOMHTMLBodyElement: Sized + std::ops::Deref {
    unsafe fn aLink(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aLink)
    }
    unsafe fn setALink_(&self, aLink: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setALink : aLink)
    }
    unsafe fn background(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, background)
    }
    unsafe fn setBackground_(&self, background: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackground : background)
    }
    unsafe fn bgColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bgColor)
    }
    unsafe fn setBgColor_(&self, bgColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBgColor : bgColor)
    }
    unsafe fn link(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, link)
    }
    unsafe fn setLink_(&self, link: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLink : link)
    }
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
    unsafe fn vLink(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vLink)
    }
    unsafe fn setVLink_(&self, vLink: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVLink : vLink)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLButtonElement(pub id);
impl std::ops::Deref for DOMHTMLButtonElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLButtonElement {}
impl DOMHTMLButtonElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLButtonElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLButtonElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLButtonElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLButtonElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLButtonElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLButtonElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLButtonElement")
        }
    }
}
impl IDOMElement for DOMHTMLButtonElement {}
impl IDOMNode for DOMHTMLButtonElement {}
impl PDOMEventTarget for DOMHTMLButtonElement {}
impl IDOMObject for DOMHTMLButtonElement {}
impl PNSCopying for DOMHTMLButtonElement {}
impl IWebScriptObject for DOMHTMLButtonElement {}
impl INSObject for DOMHTMLButtonElement {}
impl PNSObject for DOMHTMLButtonElement {}
impl IDOMHTMLButtonElement for DOMHTMLButtonElement {}
pub trait IDOMHTMLButtonElement: Sized + std::ops::Deref {
    unsafe fn click(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, click)
    }
    unsafe fn autofocus(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autofocus)
    }
    unsafe fn setAutofocus_(&self, autofocus: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutofocus : autofocus)
    }
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn willValidate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willValidate)
    }
    unsafe fn accessKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessKey)
    }
    unsafe fn setAccessKey_(&self, accessKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessKey : accessKey)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLCollection(pub id);
impl std::ops::Deref for DOMHTMLCollection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLCollection {}
impl DOMHTMLCollection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLCollection").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMHTMLCollection {}
impl PNSCopying for DOMHTMLCollection {}
impl std::convert::TryFrom<DOMObject> for DOMHTMLCollection {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMHTMLCollection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLCollection").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLCollection(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMHTMLCollection")
        }
    }
}
impl IWebScriptObject for DOMHTMLCollection {}
impl INSObject for DOMHTMLCollection {}
impl PNSObject for DOMHTMLCollection {}
impl IDOMHTMLCollection for DOMHTMLCollection {}
pub trait IDOMHTMLCollection: Sized + std::ops::Deref {
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn namedItem_(&self, name: NSString) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, namedItem : name)
    }
    unsafe fn tags_(&self, name: NSString) -> DOMNodeList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tags : name)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLDListElement(pub id);
impl std::ops::Deref for DOMHTMLDListElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLDListElement {}
impl DOMHTMLDListElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLDListElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLDListElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLDListElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLDListElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLDListElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLDListElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLDListElement")
        }
    }
}
impl IDOMElement for DOMHTMLDListElement {}
impl IDOMNode for DOMHTMLDListElement {}
impl PDOMEventTarget for DOMHTMLDListElement {}
impl IDOMObject for DOMHTMLDListElement {}
impl PNSCopying for DOMHTMLDListElement {}
impl IWebScriptObject for DOMHTMLDListElement {}
impl INSObject for DOMHTMLDListElement {}
impl PNSObject for DOMHTMLDListElement {}
impl IDOMHTMLDListElement for DOMHTMLDListElement {}
pub trait IDOMHTMLDListElement: Sized + std::ops::Deref {
    unsafe fn compact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compact)
    }
    unsafe fn setCompact_(&self, compact: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompact : compact)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLDirectoryElement(pub id);
impl std::ops::Deref for DOMHTMLDirectoryElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLDirectoryElement {}
impl DOMHTMLDirectoryElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLDirectoryElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLDirectoryElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLDirectoryElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLDirectoryElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLDirectoryElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLDirectoryElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLDirectoryElement")
        }
    }
}
impl IDOMElement for DOMHTMLDirectoryElement {}
impl IDOMNode for DOMHTMLDirectoryElement {}
impl PDOMEventTarget for DOMHTMLDirectoryElement {}
impl IDOMObject for DOMHTMLDirectoryElement {}
impl PNSCopying for DOMHTMLDirectoryElement {}
impl IWebScriptObject for DOMHTMLDirectoryElement {}
impl INSObject for DOMHTMLDirectoryElement {}
impl PNSObject for DOMHTMLDirectoryElement {}
impl IDOMHTMLDirectoryElement for DOMHTMLDirectoryElement {}
pub trait IDOMHTMLDirectoryElement: Sized + std::ops::Deref {
    unsafe fn compact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compact)
    }
    unsafe fn setCompact_(&self, compact: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompact : compact)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLDivElement(pub id);
impl std::ops::Deref for DOMHTMLDivElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLDivElement {}
impl DOMHTMLDivElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLDivElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLDivElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLDivElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLDivElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLDivElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLDivElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLDivElement")
        }
    }
}
impl IDOMElement for DOMHTMLDivElement {}
impl IDOMNode for DOMHTMLDivElement {}
impl PDOMEventTarget for DOMHTMLDivElement {}
impl IDOMObject for DOMHTMLDivElement {}
impl PNSCopying for DOMHTMLDivElement {}
impl IWebScriptObject for DOMHTMLDivElement {}
impl INSObject for DOMHTMLDivElement {}
impl PNSObject for DOMHTMLDivElement {}
impl IDOMHTMLDivElement for DOMHTMLDivElement {}
pub trait IDOMHTMLDivElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLDocument(pub id);
impl std::ops::Deref for DOMHTMLDocument {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLDocument {}
impl DOMHTMLDocument {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLDocument").unwrap(), alloc) })
    }
}
impl IDOMDocument for DOMHTMLDocument {}
impl From<DOMHTMLDocument> for DOMDocument {
    fn from(child: DOMHTMLDocument) -> DOMDocument {
        DOMDocument(child.0)
    }
}
impl std::convert::TryFrom<DOMDocument> for DOMHTMLDocument {
    type Error = &'static str;
    fn try_from(parent: DOMDocument) -> Result<DOMHTMLDocument, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLDocument").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLDocument(parent.0))
        } else {
            Err("This DOMDocument cannot be downcasted to DOMHTMLDocument")
        }
    }
}
impl IDOMNode for DOMHTMLDocument {}
impl PDOMEventTarget for DOMHTMLDocument {}
impl IDOMObject for DOMHTMLDocument {}
impl PNSCopying for DOMHTMLDocument {}
impl IWebScriptObject for DOMHTMLDocument {}
impl INSObject for DOMHTMLDocument {}
impl PNSObject for DOMHTMLDocument {}
impl IDOMHTMLDocument for DOMHTMLDocument {}
pub trait IDOMHTMLDocument: Sized + std::ops::Deref {
    unsafe fn open(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, open)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn write_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, write : text)
    }
    unsafe fn writeln_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeln : text)
    }
    unsafe fn clear(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clear)
    }
    unsafe fn captureEvents(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captureEvents)
    }
    unsafe fn releaseEvents(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseEvents)
    }
    unsafe fn embeds(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, embeds)
    }
    unsafe fn plugins(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, plugins)
    }
    unsafe fn scripts(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scripts)
    }
    unsafe fn width(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn dir(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dir)
    }
    unsafe fn setDir_(&self, dir: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDir : dir)
    }
    unsafe fn designMode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, designMode)
    }
    unsafe fn setDesignMode_(&self, designMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesignMode : designMode)
    }
    unsafe fn compatMode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compatMode)
    }
    unsafe fn bgColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bgColor)
    }
    unsafe fn setBgColor_(&self, bgColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBgColor : bgColor)
    }
    unsafe fn fgColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fgColor)
    }
    unsafe fn setFgColor_(&self, fgColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFgColor : fgColor)
    }
    unsafe fn alinkColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alinkColor)
    }
    unsafe fn setAlinkColor_(&self, alinkColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlinkColor : alinkColor)
    }
    unsafe fn linkColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkColor)
    }
    unsafe fn setLinkColor_(&self, linkColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinkColor : linkColor)
    }
    unsafe fn vlinkColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vlinkColor)
    }
    unsafe fn setVlinkColor_(&self, vlinkColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVlinkColor : vlinkColor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLEmbedElement(pub id);
impl std::ops::Deref for DOMHTMLEmbedElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLEmbedElement {}
impl DOMHTMLEmbedElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLEmbedElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLEmbedElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLEmbedElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLEmbedElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLEmbedElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLEmbedElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLEmbedElement")
        }
    }
}
impl IDOMElement for DOMHTMLEmbedElement {}
impl IDOMNode for DOMHTMLEmbedElement {}
impl PDOMEventTarget for DOMHTMLEmbedElement {}
impl IDOMObject for DOMHTMLEmbedElement {}
impl PNSCopying for DOMHTMLEmbedElement {}
impl IWebScriptObject for DOMHTMLEmbedElement {}
impl INSObject for DOMHTMLEmbedElement {}
impl PNSObject for DOMHTMLEmbedElement {}
impl IDOMHTMLEmbedElement for DOMHTMLEmbedElement {}
pub trait IDOMHTMLEmbedElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn height(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn src(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, src)
    }
    unsafe fn setSrc_(&self, src: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSrc : src)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn width(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLFieldSetElement(pub id);
impl std::ops::Deref for DOMHTMLFieldSetElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLFieldSetElement {}
impl DOMHTMLFieldSetElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLFieldSetElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLFieldSetElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLFieldSetElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLFieldSetElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLFieldSetElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLFieldSetElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLFieldSetElement")
        }
    }
}
impl IDOMElement for DOMHTMLFieldSetElement {}
impl IDOMNode for DOMHTMLFieldSetElement {}
impl PDOMEventTarget for DOMHTMLFieldSetElement {}
impl IDOMObject for DOMHTMLFieldSetElement {}
impl PNSCopying for DOMHTMLFieldSetElement {}
impl IWebScriptObject for DOMHTMLFieldSetElement {}
impl INSObject for DOMHTMLFieldSetElement {}
impl PNSObject for DOMHTMLFieldSetElement {}
impl IDOMHTMLFieldSetElement for DOMHTMLFieldSetElement {}
pub trait IDOMHTMLFieldSetElement: Sized + std::ops::Deref {
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLFontElement(pub id);
impl std::ops::Deref for DOMHTMLFontElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLFontElement {}
impl DOMHTMLFontElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLFontElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLFontElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLFontElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLFontElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLFontElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLFontElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLFontElement")
        }
    }
}
impl IDOMElement for DOMHTMLFontElement {}
impl IDOMNode for DOMHTMLFontElement {}
impl PDOMEventTarget for DOMHTMLFontElement {}
impl IDOMObject for DOMHTMLFontElement {}
impl PNSCopying for DOMHTMLFontElement {}
impl IWebScriptObject for DOMHTMLFontElement {}
impl INSObject for DOMHTMLFontElement {}
impl PNSObject for DOMHTMLFontElement {}
impl IDOMHTMLFontElement for DOMHTMLFontElement {}
pub trait IDOMHTMLFontElement: Sized + std::ops::Deref {
    unsafe fn color(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn face(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, face)
    }
    unsafe fn setFace_(&self, face: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFace : face)
    }
    unsafe fn size(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLFormElement(pub id);
impl std::ops::Deref for DOMHTMLFormElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLFormElement {}
impl DOMHTMLFormElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLFormElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLFormElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLFormElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLFormElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLFormElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLFormElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLFormElement")
        }
    }
}
impl IDOMElement for DOMHTMLFormElement {}
impl IDOMNode for DOMHTMLFormElement {}
impl PDOMEventTarget for DOMHTMLFormElement {}
impl IDOMObject for DOMHTMLFormElement {}
impl PNSCopying for DOMHTMLFormElement {}
impl IWebScriptObject for DOMHTMLFormElement {}
impl INSObject for DOMHTMLFormElement {}
impl PNSObject for DOMHTMLFormElement {}
impl IDOMHTMLFormElement for DOMHTMLFormElement {}
pub trait IDOMHTMLFormElement: Sized + std::ops::Deref {
    unsafe fn submit(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submit)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn acceptCharset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptCharset)
    }
    unsafe fn setAcceptCharset_(&self, acceptCharset: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcceptCharset : acceptCharset)
    }
    unsafe fn action(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn setAction_(&self, action: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAction : action)
    }
    unsafe fn enctype(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enctype)
    }
    unsafe fn setEnctype_(&self, enctype: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnctype : enctype)
    }
    unsafe fn encoding(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encoding)
    }
    unsafe fn setEncoding_(&self, encoding: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEncoding : encoding)
    }
    unsafe fn method(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, method)
    }
    unsafe fn setMethod_(&self, method: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMethod : method)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn target(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn elements(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLFrameElement(pub id);
impl std::ops::Deref for DOMHTMLFrameElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLFrameElement {}
impl DOMHTMLFrameElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLFrameElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLFrameElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLFrameElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLFrameElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLFrameElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLFrameElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLFrameElement")
        }
    }
}
impl IDOMElement for DOMHTMLFrameElement {}
impl IDOMNode for DOMHTMLFrameElement {}
impl PDOMEventTarget for DOMHTMLFrameElement {}
impl IDOMObject for DOMHTMLFrameElement {}
impl PNSCopying for DOMHTMLFrameElement {}
impl IWebScriptObject for DOMHTMLFrameElement {}
impl INSObject for DOMHTMLFrameElement {}
impl PNSObject for DOMHTMLFrameElement {}
impl IDOMHTMLFrameElement for DOMHTMLFrameElement {}
pub trait IDOMHTMLFrameElement: Sized + std::ops::Deref {
    unsafe fn frameBorder(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameBorder)
    }
    unsafe fn setFrameBorder_(&self, frameBorder: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameBorder : frameBorder)
    }
    unsafe fn longDesc(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longDesc)
    }
    unsafe fn setLongDesc_(&self, longDesc: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLongDesc : longDesc)
    }
    unsafe fn marginHeight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marginHeight)
    }
    unsafe fn setMarginHeight_(&self, marginHeight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarginHeight : marginHeight)
    }
    unsafe fn marginWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marginWidth)
    }
    unsafe fn setMarginWidth_(&self, marginWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarginWidth : marginWidth)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn noResize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noResize)
    }
    unsafe fn setNoResize_(&self, noResize: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNoResize : noResize)
    }
    unsafe fn scrolling(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrolling)
    }
    unsafe fn setScrolling_(&self, scrolling: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScrolling : scrolling)
    }
    unsafe fn src(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, src)
    }
    unsafe fn setSrc_(&self, src: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSrc : src)
    }
    unsafe fn contentDocument(&self) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentDocument)
    }
    unsafe fn contentWindow(&self) -> DOMAbstractView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentWindow)
    }
    unsafe fn location(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn setLocation_(&self, location: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocation : location)
    }
    unsafe fn width(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLFrameSetElement(pub id);
impl std::ops::Deref for DOMHTMLFrameSetElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLFrameSetElement {}
impl DOMHTMLFrameSetElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLFrameSetElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLFrameSetElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLFrameSetElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLFrameSetElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLFrameSetElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLFrameSetElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLFrameSetElement")
        }
    }
}
impl IDOMElement for DOMHTMLFrameSetElement {}
impl IDOMNode for DOMHTMLFrameSetElement {}
impl PDOMEventTarget for DOMHTMLFrameSetElement {}
impl IDOMObject for DOMHTMLFrameSetElement {}
impl PNSCopying for DOMHTMLFrameSetElement {}
impl IWebScriptObject for DOMHTMLFrameSetElement {}
impl INSObject for DOMHTMLFrameSetElement {}
impl PNSObject for DOMHTMLFrameSetElement {}
impl IDOMHTMLFrameSetElement for DOMHTMLFrameSetElement {}
pub trait IDOMHTMLFrameSetElement: Sized + std::ops::Deref {
    unsafe fn cols(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cols)
    }
    unsafe fn setCols_(&self, cols: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCols : cols)
    }
    unsafe fn rows(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rows)
    }
    unsafe fn setRows_(&self, rows: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRows : rows)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLHRElement(pub id);
impl std::ops::Deref for DOMHTMLHRElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLHRElement {}
impl DOMHTMLHRElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLHRElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLHRElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLHRElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLHRElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLHRElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLHRElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLHRElement")
        }
    }
}
impl IDOMElement for DOMHTMLHRElement {}
impl IDOMNode for DOMHTMLHRElement {}
impl PDOMEventTarget for DOMHTMLHRElement {}
impl IDOMObject for DOMHTMLHRElement {}
impl PNSCopying for DOMHTMLHRElement {}
impl IWebScriptObject for DOMHTMLHRElement {}
impl INSObject for DOMHTMLHRElement {}
impl PNSObject for DOMHTMLHRElement {}
impl IDOMHTMLHRElement for DOMHTMLHRElement {}
pub trait IDOMHTMLHRElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn noShade(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noShade)
    }
    unsafe fn setNoShade_(&self, noShade: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNoShade : noShade)
    }
    unsafe fn size(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn width(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLHeadElement(pub id);
impl std::ops::Deref for DOMHTMLHeadElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLHeadElement {}
impl DOMHTMLHeadElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLHeadElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLHeadElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLHeadElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLHeadElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLHeadElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLHeadElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLHeadElement")
        }
    }
}
impl IDOMElement for DOMHTMLHeadElement {}
impl IDOMNode for DOMHTMLHeadElement {}
impl PDOMEventTarget for DOMHTMLHeadElement {}
impl IDOMObject for DOMHTMLHeadElement {}
impl PNSCopying for DOMHTMLHeadElement {}
impl IWebScriptObject for DOMHTMLHeadElement {}
impl INSObject for DOMHTMLHeadElement {}
impl PNSObject for DOMHTMLHeadElement {}
impl IDOMHTMLHeadElement for DOMHTMLHeadElement {}
pub trait IDOMHTMLHeadElement: Sized + std::ops::Deref {
    unsafe fn profile(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profile)
    }
    unsafe fn setProfile_(&self, profile: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfile : profile)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLHeadingElement(pub id);
impl std::ops::Deref for DOMHTMLHeadingElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLHeadingElement {}
impl DOMHTMLHeadingElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLHeadingElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLHeadingElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLHeadingElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLHeadingElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLHeadingElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLHeadingElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLHeadingElement")
        }
    }
}
impl IDOMElement for DOMHTMLHeadingElement {}
impl IDOMNode for DOMHTMLHeadingElement {}
impl PDOMEventTarget for DOMHTMLHeadingElement {}
impl IDOMObject for DOMHTMLHeadingElement {}
impl PNSCopying for DOMHTMLHeadingElement {}
impl IWebScriptObject for DOMHTMLHeadingElement {}
impl INSObject for DOMHTMLHeadingElement {}
impl PNSObject for DOMHTMLHeadingElement {}
impl IDOMHTMLHeadingElement for DOMHTMLHeadingElement {}
pub trait IDOMHTMLHeadingElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLHtmlElement(pub id);
impl std::ops::Deref for DOMHTMLHtmlElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLHtmlElement {}
impl DOMHTMLHtmlElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLHtmlElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLHtmlElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLHtmlElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLHtmlElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLHtmlElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLHtmlElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLHtmlElement")
        }
    }
}
impl IDOMElement for DOMHTMLHtmlElement {}
impl IDOMNode for DOMHTMLHtmlElement {}
impl PDOMEventTarget for DOMHTMLHtmlElement {}
impl IDOMObject for DOMHTMLHtmlElement {}
impl PNSCopying for DOMHTMLHtmlElement {}
impl IWebScriptObject for DOMHTMLHtmlElement {}
impl INSObject for DOMHTMLHtmlElement {}
impl PNSObject for DOMHTMLHtmlElement {}
impl IDOMHTMLHtmlElement for DOMHTMLHtmlElement {}
pub trait IDOMHTMLHtmlElement: Sized + std::ops::Deref {
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn setVersion_(&self, version: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersion : version)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLIFrameElement(pub id);
impl std::ops::Deref for DOMHTMLIFrameElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLIFrameElement {}
impl DOMHTMLIFrameElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLIFrameElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLIFrameElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLIFrameElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLIFrameElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLIFrameElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLIFrameElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLIFrameElement")
        }
    }
}
impl IDOMElement for DOMHTMLIFrameElement {}
impl IDOMNode for DOMHTMLIFrameElement {}
impl PDOMEventTarget for DOMHTMLIFrameElement {}
impl IDOMObject for DOMHTMLIFrameElement {}
impl PNSCopying for DOMHTMLIFrameElement {}
impl IWebScriptObject for DOMHTMLIFrameElement {}
impl INSObject for DOMHTMLIFrameElement {}
impl PNSObject for DOMHTMLIFrameElement {}
impl IDOMHTMLIFrameElement for DOMHTMLIFrameElement {}
pub trait IDOMHTMLIFrameElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn frameBorder(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameBorder)
    }
    unsafe fn setFrameBorder_(&self, frameBorder: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameBorder : frameBorder)
    }
    unsafe fn height(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn longDesc(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longDesc)
    }
    unsafe fn setLongDesc_(&self, longDesc: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLongDesc : longDesc)
    }
    unsafe fn marginHeight(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marginHeight)
    }
    unsafe fn setMarginHeight_(&self, marginHeight: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarginHeight : marginHeight)
    }
    unsafe fn marginWidth(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, marginWidth)
    }
    unsafe fn setMarginWidth_(&self, marginWidth: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarginWidth : marginWidth)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn scrolling(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrolling)
    }
    unsafe fn setScrolling_(&self, scrolling: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScrolling : scrolling)
    }
    unsafe fn src(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, src)
    }
    unsafe fn setSrc_(&self, src: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSrc : src)
    }
    unsafe fn width(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn contentDocument(&self) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentDocument)
    }
    unsafe fn contentWindow(&self) -> DOMAbstractView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentWindow)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLImageElement(pub id);
impl std::ops::Deref for DOMHTMLImageElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLImageElement {}
impl DOMHTMLImageElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLImageElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLImageElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLImageElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLImageElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLImageElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLImageElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLImageElement")
        }
    }
}
impl IDOMElement for DOMHTMLImageElement {}
impl IDOMNode for DOMHTMLImageElement {}
impl PDOMEventTarget for DOMHTMLImageElement {}
impl IDOMObject for DOMHTMLImageElement {}
impl PNSCopying for DOMHTMLImageElement {}
impl IWebScriptObject for DOMHTMLImageElement {}
impl INSObject for DOMHTMLImageElement {}
impl PNSObject for DOMHTMLImageElement {}
impl IDOMHTMLImageElement for DOMHTMLImageElement {}
pub trait IDOMHTMLImageElement: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn alt(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alt)
    }
    unsafe fn setAlt_(&self, alt: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlt : alt)
    }
    unsafe fn border(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, border)
    }
    unsafe fn setBorder_(&self, border: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorder : border)
    }
    unsafe fn height(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn hspace(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hspace)
    }
    unsafe fn setHspace_(&self, hspace: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHspace : hspace)
    }
    unsafe fn isMap(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMap)
    }
    unsafe fn setIsMap_(&self, isMap: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsMap : isMap)
    }
    unsafe fn longDesc(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longDesc)
    }
    unsafe fn setLongDesc_(&self, longDesc: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLongDesc : longDesc)
    }
    unsafe fn src(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, src)
    }
    unsafe fn setSrc_(&self, src: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSrc : src)
    }
    unsafe fn useMap(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useMap)
    }
    unsafe fn setUseMap_(&self, useMap: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseMap : useMap)
    }
    unsafe fn vspace(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vspace)
    }
    unsafe fn setVspace_(&self, vspace: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVspace : vspace)
    }
    unsafe fn width(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn complete(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, complete)
    }
    unsafe fn lowsrc(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowsrc)
    }
    unsafe fn setLowsrc_(&self, lowsrc: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowsrc : lowsrc)
    }
    unsafe fn naturalHeight(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, naturalHeight)
    }
    unsafe fn naturalWidth(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, naturalWidth)
    }
    unsafe fn x(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, x)
    }
    unsafe fn y(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, y)
    }
    unsafe fn altDisplayString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altDisplayString)
    }
    unsafe fn absoluteImageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteImageURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLInputElement(pub id);
impl std::ops::Deref for DOMHTMLInputElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLInputElement {}
impl DOMHTMLInputElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLInputElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLInputElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLInputElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLInputElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLInputElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLInputElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLInputElement")
        }
    }
}
impl IDOMElement for DOMHTMLInputElement {}
impl IDOMNode for DOMHTMLInputElement {}
impl PDOMEventTarget for DOMHTMLInputElement {}
impl IDOMObject for DOMHTMLInputElement {}
impl PNSCopying for DOMHTMLInputElement {}
impl IWebScriptObject for DOMHTMLInputElement {}
impl INSObject for DOMHTMLInputElement {}
impl PNSObject for DOMHTMLInputElement {}
impl IDOMHTMLInputElement for DOMHTMLInputElement {}
pub trait IDOMHTMLInputElement: Sized + std::ops::Deref {
    unsafe fn select(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, select)
    }
    unsafe fn setSelectionRange_end_(
        &self,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionRange : start, end : end)
    }
    unsafe fn click(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, click)
    }
    unsafe fn accept(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accept)
    }
    unsafe fn setAccept_(&self, accept: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccept : accept)
    }
    unsafe fn alt(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alt)
    }
    unsafe fn setAlt_(&self, alt: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlt : alt)
    }
    unsafe fn autofocus(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autofocus)
    }
    unsafe fn setAutofocus_(&self, autofocus: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutofocus : autofocus)
    }
    unsafe fn defaultChecked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultChecked)
    }
    unsafe fn setDefaultChecked_(&self, defaultChecked: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultChecked : defaultChecked)
    }
    unsafe fn checked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, checked)
    }
    unsafe fn setChecked_(&self, checked: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChecked : checked)
    }
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
    unsafe fn files(&self) -> DOMFileList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, files)
    }
    unsafe fn setFiles_(&self, files: DOMFileList)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFiles : files)
    }
    unsafe fn indeterminate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indeterminate)
    }
    unsafe fn setIndeterminate_(&self, indeterminate: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndeterminate : indeterminate)
    }
    unsafe fn maxLength(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxLength)
    }
    unsafe fn setMaxLength_(&self, maxLength: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxLength : maxLength)
    }
    unsafe fn multiple(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiple)
    }
    unsafe fn setMultiple_(&self, multiple: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMultiple : multiple)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn readOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readOnly)
    }
    unsafe fn setReadOnly_(&self, readOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadOnly : readOnly)
    }
    unsafe fn size(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn src(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, src)
    }
    unsafe fn setSrc_(&self, src: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSrc : src)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn defaultValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultValue)
    }
    unsafe fn setDefaultValue_(&self, defaultValue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultValue : defaultValue)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn willValidate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willValidate)
    }
    unsafe fn selectionStart(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionStart)
    }
    unsafe fn setSelectionStart_(&self, selectionStart: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionStart : selectionStart)
    }
    unsafe fn selectionEnd(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionEnd)
    }
    unsafe fn setSelectionEnd_(&self, selectionEnd: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionEnd : selectionEnd)
    }
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn useMap(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useMap)
    }
    unsafe fn setUseMap_(&self, useMap: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseMap : useMap)
    }
    unsafe fn accessKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessKey)
    }
    unsafe fn setAccessKey_(&self, accessKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessKey : accessKey)
    }
    unsafe fn altDisplayString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altDisplayString)
    }
    unsafe fn absoluteImageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteImageURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLLIElement(pub id);
impl std::ops::Deref for DOMHTMLLIElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLLIElement {}
impl DOMHTMLLIElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLLIElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLLIElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLLIElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLLIElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLLIElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLLIElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLLIElement")
        }
    }
}
impl IDOMElement for DOMHTMLLIElement {}
impl IDOMNode for DOMHTMLLIElement {}
impl PDOMEventTarget for DOMHTMLLIElement {}
impl IDOMObject for DOMHTMLLIElement {}
impl PNSCopying for DOMHTMLLIElement {}
impl IWebScriptObject for DOMHTMLLIElement {}
impl INSObject for DOMHTMLLIElement {}
impl PNSObject for DOMHTMLLIElement {}
impl IDOMHTMLLIElement for DOMHTMLLIElement {}
pub trait IDOMHTMLLIElement: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn value(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLLabelElement(pub id);
impl std::ops::Deref for DOMHTMLLabelElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLLabelElement {}
impl DOMHTMLLabelElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLLabelElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLLabelElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLLabelElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLLabelElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLLabelElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLLabelElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLLabelElement")
        }
    }
}
impl IDOMElement for DOMHTMLLabelElement {}
impl IDOMNode for DOMHTMLLabelElement {}
impl PDOMEventTarget for DOMHTMLLabelElement {}
impl IDOMObject for DOMHTMLLabelElement {}
impl PNSCopying for DOMHTMLLabelElement {}
impl IWebScriptObject for DOMHTMLLabelElement {}
impl INSObject for DOMHTMLLabelElement {}
impl PNSObject for DOMHTMLLabelElement {}
impl IDOMHTMLLabelElement for DOMHTMLLabelElement {}
pub trait IDOMHTMLLabelElement: Sized + std::ops::Deref {
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
    unsafe fn htmlFor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, htmlFor)
    }
    unsafe fn setHtmlFor_(&self, htmlFor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHtmlFor : htmlFor)
    }
    unsafe fn accessKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessKey)
    }
    unsafe fn setAccessKey_(&self, accessKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessKey : accessKey)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLLegendElement(pub id);
impl std::ops::Deref for DOMHTMLLegendElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLLegendElement {}
impl DOMHTMLLegendElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLLegendElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLLegendElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLLegendElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLLegendElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLLegendElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLLegendElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLLegendElement")
        }
    }
}
impl IDOMElement for DOMHTMLLegendElement {}
impl IDOMNode for DOMHTMLLegendElement {}
impl PDOMEventTarget for DOMHTMLLegendElement {}
impl IDOMObject for DOMHTMLLegendElement {}
impl PNSCopying for DOMHTMLLegendElement {}
impl IWebScriptObject for DOMHTMLLegendElement {}
impl INSObject for DOMHTMLLegendElement {}
impl PNSObject for DOMHTMLLegendElement {}
impl IDOMHTMLLegendElement for DOMHTMLLegendElement {}
pub trait IDOMHTMLLegendElement: Sized + std::ops::Deref {
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn accessKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessKey)
    }
    unsafe fn setAccessKey_(&self, accessKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessKey : accessKey)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLLinkElement(pub id);
impl std::ops::Deref for DOMHTMLLinkElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLLinkElement {}
impl DOMHTMLLinkElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLLinkElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLLinkElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLLinkElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLLinkElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLLinkElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLLinkElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLLinkElement")
        }
    }
}
impl IDOMElement for DOMHTMLLinkElement {}
impl IDOMNode for DOMHTMLLinkElement {}
impl PDOMEventTarget for DOMHTMLLinkElement {}
impl IDOMObject for DOMHTMLLinkElement {}
impl PNSCopying for DOMHTMLLinkElement {}
impl IWebScriptObject for DOMHTMLLinkElement {}
impl INSObject for DOMHTMLLinkElement {}
impl PNSObject for DOMHTMLLinkElement {}
impl IDOMHTMLLinkElement for DOMHTMLLinkElement {}
pub trait IDOMHTMLLinkElement: Sized + std::ops::Deref {
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn charset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charset)
    }
    unsafe fn setCharset_(&self, charset: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharset : charset)
    }
    unsafe fn href(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, href)
    }
    unsafe fn setHref_(&self, href: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHref : href)
    }
    unsafe fn hreflang(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hreflang)
    }
    unsafe fn setHreflang_(&self, hreflang: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHreflang : hreflang)
    }
    unsafe fn media(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, media)
    }
    unsafe fn setMedia_(&self, media: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMedia : media)
    }
    unsafe fn rel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rel)
    }
    unsafe fn setRel_(&self, rel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRel : rel)
    }
    unsafe fn rev(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rev)
    }
    unsafe fn setRev_(&self, rev: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRev : rev)
    }
    unsafe fn target(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn sheet(&self) -> DOMStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sheet)
    }
    unsafe fn absoluteLinkURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteLinkURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLMapElement(pub id);
impl std::ops::Deref for DOMHTMLMapElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLMapElement {}
impl DOMHTMLMapElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLMapElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLMapElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLMapElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLMapElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLMapElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLMapElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLMapElement")
        }
    }
}
impl IDOMElement for DOMHTMLMapElement {}
impl IDOMNode for DOMHTMLMapElement {}
impl PDOMEventTarget for DOMHTMLMapElement {}
impl IDOMObject for DOMHTMLMapElement {}
impl PNSCopying for DOMHTMLMapElement {}
impl IWebScriptObject for DOMHTMLMapElement {}
impl INSObject for DOMHTMLMapElement {}
impl PNSObject for DOMHTMLMapElement {}
impl IDOMHTMLMapElement for DOMHTMLMapElement {}
pub trait IDOMHTMLMapElement: Sized + std::ops::Deref {
    unsafe fn areas(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areas)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLMarqueeElement(pub id);
impl std::ops::Deref for DOMHTMLMarqueeElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLMarqueeElement {}
impl DOMHTMLMarqueeElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLMarqueeElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLMarqueeElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLMarqueeElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLMarqueeElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLMarqueeElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLMarqueeElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLMarqueeElement")
        }
    }
}
impl IDOMElement for DOMHTMLMarqueeElement {}
impl IDOMNode for DOMHTMLMarqueeElement {}
impl PDOMEventTarget for DOMHTMLMarqueeElement {}
impl IDOMObject for DOMHTMLMarqueeElement {}
impl PNSCopying for DOMHTMLMarqueeElement {}
impl IWebScriptObject for DOMHTMLMarqueeElement {}
impl INSObject for DOMHTMLMarqueeElement {}
impl PNSObject for DOMHTMLMarqueeElement {}
impl IDOMHTMLMarqueeElement for DOMHTMLMarqueeElement {}
pub trait IDOMHTMLMarqueeElement: Sized + std::ops::Deref {
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLMenuElement(pub id);
impl std::ops::Deref for DOMHTMLMenuElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLMenuElement {}
impl DOMHTMLMenuElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLMenuElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLMenuElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLMenuElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLMenuElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLMenuElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLMenuElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLMenuElement")
        }
    }
}
impl IDOMElement for DOMHTMLMenuElement {}
impl IDOMNode for DOMHTMLMenuElement {}
impl PDOMEventTarget for DOMHTMLMenuElement {}
impl IDOMObject for DOMHTMLMenuElement {}
impl PNSCopying for DOMHTMLMenuElement {}
impl IWebScriptObject for DOMHTMLMenuElement {}
impl INSObject for DOMHTMLMenuElement {}
impl PNSObject for DOMHTMLMenuElement {}
impl IDOMHTMLMenuElement for DOMHTMLMenuElement {}
pub trait IDOMHTMLMenuElement: Sized + std::ops::Deref {
    unsafe fn compact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compact)
    }
    unsafe fn setCompact_(&self, compact: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompact : compact)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLMetaElement(pub id);
impl std::ops::Deref for DOMHTMLMetaElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLMetaElement {}
impl DOMHTMLMetaElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLMetaElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLMetaElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLMetaElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLMetaElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLMetaElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLMetaElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLMetaElement")
        }
    }
}
impl IDOMElement for DOMHTMLMetaElement {}
impl IDOMNode for DOMHTMLMetaElement {}
impl PDOMEventTarget for DOMHTMLMetaElement {}
impl IDOMObject for DOMHTMLMetaElement {}
impl PNSCopying for DOMHTMLMetaElement {}
impl IWebScriptObject for DOMHTMLMetaElement {}
impl INSObject for DOMHTMLMetaElement {}
impl PNSObject for DOMHTMLMetaElement {}
impl IDOMHTMLMetaElement for DOMHTMLMetaElement {}
pub trait IDOMHTMLMetaElement: Sized + std::ops::Deref {
    unsafe fn content(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, content)
    }
    unsafe fn setContent_(&self, content: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContent : content)
    }
    unsafe fn httpEquiv(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, httpEquiv)
    }
    unsafe fn setHttpEquiv_(&self, httpEquiv: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHttpEquiv : httpEquiv)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn scheme(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheme)
    }
    unsafe fn setScheme_(&self, scheme: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScheme : scheme)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLModElement(pub id);
impl std::ops::Deref for DOMHTMLModElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLModElement {}
impl DOMHTMLModElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLModElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLModElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLModElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLModElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLModElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLModElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLModElement")
        }
    }
}
impl IDOMElement for DOMHTMLModElement {}
impl IDOMNode for DOMHTMLModElement {}
impl PDOMEventTarget for DOMHTMLModElement {}
impl IDOMObject for DOMHTMLModElement {}
impl PNSCopying for DOMHTMLModElement {}
impl IWebScriptObject for DOMHTMLModElement {}
impl INSObject for DOMHTMLModElement {}
impl PNSObject for DOMHTMLModElement {}
impl IDOMHTMLModElement for DOMHTMLModElement {}
pub trait IDOMHTMLModElement: Sized + std::ops::Deref {
    unsafe fn cite(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cite)
    }
    unsafe fn setCite_(&self, cite: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCite : cite)
    }
    unsafe fn dateTime(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateTime)
    }
    unsafe fn setDateTime_(&self, dateTime: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDateTime : dateTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLOListElement(pub id);
impl std::ops::Deref for DOMHTMLOListElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLOListElement {}
impl DOMHTMLOListElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLOListElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLOListElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLOListElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLOListElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLOListElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLOListElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLOListElement")
        }
    }
}
impl IDOMElement for DOMHTMLOListElement {}
impl IDOMNode for DOMHTMLOListElement {}
impl PDOMEventTarget for DOMHTMLOListElement {}
impl IDOMObject for DOMHTMLOListElement {}
impl PNSCopying for DOMHTMLOListElement {}
impl IWebScriptObject for DOMHTMLOListElement {}
impl INSObject for DOMHTMLOListElement {}
impl PNSObject for DOMHTMLOListElement {}
impl IDOMHTMLOListElement for DOMHTMLOListElement {}
pub trait IDOMHTMLOListElement: Sized + std::ops::Deref {
    unsafe fn compact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compact)
    }
    unsafe fn setCompact_(&self, compact: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompact : compact)
    }
    unsafe fn start(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn setStart_(&self, start: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStart : start)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLObjectElement(pub id);
impl std::ops::Deref for DOMHTMLObjectElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLObjectElement {}
impl DOMHTMLObjectElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLObjectElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLObjectElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLObjectElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLObjectElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLObjectElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLObjectElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLObjectElement")
        }
    }
}
impl IDOMElement for DOMHTMLObjectElement {}
impl IDOMNode for DOMHTMLObjectElement {}
impl PDOMEventTarget for DOMHTMLObjectElement {}
impl IDOMObject for DOMHTMLObjectElement {}
impl PNSCopying for DOMHTMLObjectElement {}
impl IWebScriptObject for DOMHTMLObjectElement {}
impl INSObject for DOMHTMLObjectElement {}
impl PNSObject for DOMHTMLObjectElement {}
impl IDOMHTMLObjectElement for DOMHTMLObjectElement {}
pub trait IDOMHTMLObjectElement: Sized + std::ops::Deref {
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
    unsafe fn code(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, code)
    }
    unsafe fn setCode_(&self, code: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCode : code)
    }
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn archive(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, archive)
    }
    unsafe fn setArchive_(&self, archive: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArchive : archive)
    }
    unsafe fn border(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, border)
    }
    unsafe fn setBorder_(&self, border: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorder : border)
    }
    unsafe fn codeBase(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, codeBase)
    }
    unsafe fn setCodeBase_(&self, codeBase: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCodeBase : codeBase)
    }
    unsafe fn codeType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, codeType)
    }
    unsafe fn setCodeType_(&self, codeType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCodeType : codeType)
    }
    unsafe fn data(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn setData_(&self, data: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setData : data)
    }
    unsafe fn declare(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, declare)
    }
    unsafe fn setDeclare_(&self, declare: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeclare : declare)
    }
    unsafe fn height(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn hspace(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hspace)
    }
    unsafe fn setHspace_(&self, hspace: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHspace : hspace)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn standby(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, standby)
    }
    unsafe fn setStandby_(&self, standby: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStandby : standby)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn useMap(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useMap)
    }
    unsafe fn setUseMap_(&self, useMap: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseMap : useMap)
    }
    unsafe fn vspace(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vspace)
    }
    unsafe fn setVspace_(&self, vspace: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVspace : vspace)
    }
    unsafe fn width(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn contentDocument(&self) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentDocument)
    }
    unsafe fn absoluteImageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteImageURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLOptGroupElement(pub id);
impl std::ops::Deref for DOMHTMLOptGroupElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLOptGroupElement {}
impl DOMHTMLOptGroupElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLOptGroupElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLOptGroupElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLOptGroupElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLOptGroupElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLOptGroupElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLOptGroupElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLOptGroupElement")
        }
    }
}
impl IDOMElement for DOMHTMLOptGroupElement {}
impl IDOMNode for DOMHTMLOptGroupElement {}
impl PDOMEventTarget for DOMHTMLOptGroupElement {}
impl IDOMObject for DOMHTMLOptGroupElement {}
impl PNSCopying for DOMHTMLOptGroupElement {}
impl IWebScriptObject for DOMHTMLOptGroupElement {}
impl INSObject for DOMHTMLOptGroupElement {}
impl PNSObject for DOMHTMLOptGroupElement {}
impl IDOMHTMLOptGroupElement for DOMHTMLOptGroupElement {}
pub trait IDOMHTMLOptGroupElement: Sized + std::ops::Deref {
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLOptionElement(pub id);
impl std::ops::Deref for DOMHTMLOptionElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLOptionElement {}
impl DOMHTMLOptionElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLOptionElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLOptionElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLOptionElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLOptionElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLOptionElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLOptionElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLOptionElement")
        }
    }
}
impl IDOMElement for DOMHTMLOptionElement {}
impl IDOMNode for DOMHTMLOptionElement {}
impl PDOMEventTarget for DOMHTMLOptionElement {}
impl IDOMObject for DOMHTMLOptionElement {}
impl PNSCopying for DOMHTMLOptionElement {}
impl IWebScriptObject for DOMHTMLOptionElement {}
impl INSObject for DOMHTMLOptionElement {}
impl PNSObject for DOMHTMLOptionElement {}
impl IDOMHTMLOptionElement for DOMHTMLOptionElement {}
pub trait IDOMHTMLOptionElement: Sized + std::ops::Deref {
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn defaultSelected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultSelected)
    }
    unsafe fn setDefaultSelected_(&self, defaultSelected: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultSelected : defaultSelected)
    }
    unsafe fn selected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selected)
    }
    unsafe fn setSelected_(&self, selected: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelected : selected)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn index(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLOptionsCollection(pub id);
impl std::ops::Deref for DOMHTMLOptionsCollection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLOptionsCollection {}
impl DOMHTMLOptionsCollection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLOptionsCollection").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMHTMLOptionsCollection {}
impl PNSCopying for DOMHTMLOptionsCollection {}
impl std::convert::TryFrom<DOMObject> for DOMHTMLOptionsCollection {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMHTMLOptionsCollection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLOptionsCollection").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLOptionsCollection(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMHTMLOptionsCollection")
        }
    }
}
impl IWebScriptObject for DOMHTMLOptionsCollection {}
impl INSObject for DOMHTMLOptionsCollection {}
impl PNSObject for DOMHTMLOptionsCollection {}
impl IDOMHTMLOptionsCollection for DOMHTMLOptionsCollection {}
pub trait IDOMHTMLOptionsCollection: Sized + std::ops::Deref {
    unsafe fn namedItem_(&self, name: NSString) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, namedItem : name)
    }
    unsafe fn add_index_(&self, option: DOMHTMLOptionElement, index: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, add : option, index : index)
    }
    unsafe fn remove_(&self, index: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, remove : index)
    }
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn selectedIndex(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedIndex)
    }
    unsafe fn setSelectedIndex_(&self, selectedIndex: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedIndex : selectedIndex)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn setLength_(&self, length: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLength : length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLParagraphElement(pub id);
impl std::ops::Deref for DOMHTMLParagraphElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLParagraphElement {}
impl DOMHTMLParagraphElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLParagraphElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLParagraphElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLParagraphElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLParagraphElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLParagraphElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLParagraphElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLParagraphElement")
        }
    }
}
impl IDOMElement for DOMHTMLParagraphElement {}
impl IDOMNode for DOMHTMLParagraphElement {}
impl PDOMEventTarget for DOMHTMLParagraphElement {}
impl IDOMObject for DOMHTMLParagraphElement {}
impl PNSCopying for DOMHTMLParagraphElement {}
impl IWebScriptObject for DOMHTMLParagraphElement {}
impl INSObject for DOMHTMLParagraphElement {}
impl PNSObject for DOMHTMLParagraphElement {}
impl IDOMHTMLParagraphElement for DOMHTMLParagraphElement {}
pub trait IDOMHTMLParagraphElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLParamElement(pub id);
impl std::ops::Deref for DOMHTMLParamElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLParamElement {}
impl DOMHTMLParamElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLParamElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLParamElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLParamElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLParamElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLParamElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLParamElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLParamElement")
        }
    }
}
impl IDOMElement for DOMHTMLParamElement {}
impl IDOMNode for DOMHTMLParamElement {}
impl PDOMEventTarget for DOMHTMLParamElement {}
impl IDOMObject for DOMHTMLParamElement {}
impl PNSCopying for DOMHTMLParamElement {}
impl IWebScriptObject for DOMHTMLParamElement {}
impl INSObject for DOMHTMLParamElement {}
impl PNSObject for DOMHTMLParamElement {}
impl IDOMHTMLParamElement for DOMHTMLParamElement {}
pub trait IDOMHTMLParamElement: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn valueType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueType)
    }
    unsafe fn setValueType_(&self, valueType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueType : valueType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLPreElement(pub id);
impl std::ops::Deref for DOMHTMLPreElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLPreElement {}
impl DOMHTMLPreElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLPreElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLPreElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLPreElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLPreElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLPreElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLPreElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLPreElement")
        }
    }
}
impl IDOMElement for DOMHTMLPreElement {}
impl IDOMNode for DOMHTMLPreElement {}
impl PDOMEventTarget for DOMHTMLPreElement {}
impl IDOMObject for DOMHTMLPreElement {}
impl PNSCopying for DOMHTMLPreElement {}
impl IWebScriptObject for DOMHTMLPreElement {}
impl INSObject for DOMHTMLPreElement {}
impl PNSObject for DOMHTMLPreElement {}
impl IDOMHTMLPreElement for DOMHTMLPreElement {}
pub trait IDOMHTMLPreElement: Sized + std::ops::Deref {
    unsafe fn width(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn wrap(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wrap)
    }
    unsafe fn setWrap_(&self, wrap: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWrap : wrap)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLQuoteElement(pub id);
impl std::ops::Deref for DOMHTMLQuoteElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLQuoteElement {}
impl DOMHTMLQuoteElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLQuoteElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLQuoteElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLQuoteElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLQuoteElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLQuoteElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLQuoteElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLQuoteElement")
        }
    }
}
impl IDOMElement for DOMHTMLQuoteElement {}
impl IDOMNode for DOMHTMLQuoteElement {}
impl PDOMEventTarget for DOMHTMLQuoteElement {}
impl IDOMObject for DOMHTMLQuoteElement {}
impl PNSCopying for DOMHTMLQuoteElement {}
impl IWebScriptObject for DOMHTMLQuoteElement {}
impl INSObject for DOMHTMLQuoteElement {}
impl PNSObject for DOMHTMLQuoteElement {}
impl IDOMHTMLQuoteElement for DOMHTMLQuoteElement {}
pub trait IDOMHTMLQuoteElement: Sized + std::ops::Deref {
    unsafe fn cite(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cite)
    }
    unsafe fn setCite_(&self, cite: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCite : cite)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLScriptElement(pub id);
impl std::ops::Deref for DOMHTMLScriptElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLScriptElement {}
impl DOMHTMLScriptElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLScriptElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLScriptElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLScriptElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLScriptElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLScriptElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLScriptElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLScriptElement")
        }
    }
}
impl IDOMElement for DOMHTMLScriptElement {}
impl IDOMNode for DOMHTMLScriptElement {}
impl PDOMEventTarget for DOMHTMLScriptElement {}
impl IDOMObject for DOMHTMLScriptElement {}
impl PNSCopying for DOMHTMLScriptElement {}
impl IWebScriptObject for DOMHTMLScriptElement {}
impl INSObject for DOMHTMLScriptElement {}
impl PNSObject for DOMHTMLScriptElement {}
impl IDOMHTMLScriptElement for DOMHTMLScriptElement {}
pub trait IDOMHTMLScriptElement: Sized + std::ops::Deref {
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
    unsafe fn htmlFor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, htmlFor)
    }
    unsafe fn setHtmlFor_(&self, htmlFor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHtmlFor : htmlFor)
    }
    unsafe fn event(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, event)
    }
    unsafe fn setEvent_(&self, event: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEvent : event)
    }
    unsafe fn charset(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charset)
    }
    unsafe fn setCharset_(&self, charset: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharset : charset)
    }
    unsafe fn defer(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defer)
    }
    unsafe fn setDefer_(&self, defer: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefer : defer)
    }
    unsafe fn src(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, src)
    }
    unsafe fn setSrc_(&self, src: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSrc : src)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLSelectElement(pub id);
impl std::ops::Deref for DOMHTMLSelectElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLSelectElement {}
impl DOMHTMLSelectElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLSelectElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLSelectElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLSelectElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLSelectElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLSelectElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLSelectElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLSelectElement")
        }
    }
}
impl IDOMElement for DOMHTMLSelectElement {}
impl IDOMNode for DOMHTMLSelectElement {}
impl PDOMEventTarget for DOMHTMLSelectElement {}
impl IDOMObject for DOMHTMLSelectElement {}
impl PNSCopying for DOMHTMLSelectElement {}
impl IWebScriptObject for DOMHTMLSelectElement {}
impl INSObject for DOMHTMLSelectElement {}
impl PNSObject for DOMHTMLSelectElement {}
impl IDOMHTMLSelectElement for DOMHTMLSelectElement {}
pub trait IDOMHTMLSelectElement: Sized + std::ops::Deref {
    unsafe fn item_(&self, index: ::std::os::raw::c_uint) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, item : index)
    }
    unsafe fn namedItem_(&self, name: NSString) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, namedItem : name)
    }
    unsafe fn add_before_(&self, element: DOMHTMLElement, before: DOMHTMLElement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, add : element, before : before)
    }
    unsafe fn remove_(&self, index: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, remove : index)
    }
    unsafe fn autofocus(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autofocus)
    }
    unsafe fn setAutofocus_(&self, autofocus: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutofocus : autofocus)
    }
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
    unsafe fn multiple(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiple)
    }
    unsafe fn setMultiple_(&self, multiple: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMultiple : multiple)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn size(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn options(&self) -> DOMHTMLOptionsCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn length(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn selectedIndex(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedIndex)
    }
    unsafe fn setSelectedIndex_(&self, selectedIndex: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedIndex : selectedIndex)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn willValidate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willValidate)
    }
}
impl DOMHTMLSelectElement_DOMHTMLSelectElementDeprecated for DOMHTMLSelectElement {}
pub trait DOMHTMLSelectElement_DOMHTMLSelectElementDeprecated: Sized + std::ops::Deref {
    unsafe fn add__(&self, element: DOMHTMLElement, before: DOMHTMLElement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, add : element, before : before)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLStyleElement(pub id);
impl std::ops::Deref for DOMHTMLStyleElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLStyleElement {}
impl DOMHTMLStyleElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLStyleElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLStyleElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLStyleElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLStyleElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLStyleElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLStyleElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLStyleElement")
        }
    }
}
impl IDOMElement for DOMHTMLStyleElement {}
impl IDOMNode for DOMHTMLStyleElement {}
impl PDOMEventTarget for DOMHTMLStyleElement {}
impl IDOMObject for DOMHTMLStyleElement {}
impl PNSCopying for DOMHTMLStyleElement {}
impl IWebScriptObject for DOMHTMLStyleElement {}
impl INSObject for DOMHTMLStyleElement {}
impl PNSObject for DOMHTMLStyleElement {}
impl IDOMHTMLStyleElement for DOMHTMLStyleElement {}
pub trait IDOMHTMLStyleElement: Sized + std::ops::Deref {
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn media(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, media)
    }
    unsafe fn setMedia_(&self, media: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMedia : media)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn sheet(&self) -> DOMStyleSheet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sheet)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLTableCaptionElement(pub id);
impl std::ops::Deref for DOMHTMLTableCaptionElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLTableCaptionElement {}
impl DOMHTMLTableCaptionElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLTableCaptionElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLTableCaptionElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLTableCaptionElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLTableCaptionElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLTableCaptionElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLTableCaptionElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLTableCaptionElement")
        }
    }
}
impl IDOMElement for DOMHTMLTableCaptionElement {}
impl IDOMNode for DOMHTMLTableCaptionElement {}
impl PDOMEventTarget for DOMHTMLTableCaptionElement {}
impl IDOMObject for DOMHTMLTableCaptionElement {}
impl PNSCopying for DOMHTMLTableCaptionElement {}
impl IWebScriptObject for DOMHTMLTableCaptionElement {}
impl INSObject for DOMHTMLTableCaptionElement {}
impl PNSObject for DOMHTMLTableCaptionElement {}
impl IDOMHTMLTableCaptionElement for DOMHTMLTableCaptionElement {}
pub trait IDOMHTMLTableCaptionElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLTableCellElement(pub id);
impl std::ops::Deref for DOMHTMLTableCellElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLTableCellElement {}
impl DOMHTMLTableCellElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLTableCellElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLTableCellElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLTableCellElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLTableCellElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLTableCellElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLTableCellElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLTableCellElement")
        }
    }
}
impl IDOMElement for DOMHTMLTableCellElement {}
impl IDOMNode for DOMHTMLTableCellElement {}
impl PDOMEventTarget for DOMHTMLTableCellElement {}
impl IDOMObject for DOMHTMLTableCellElement {}
impl PNSCopying for DOMHTMLTableCellElement {}
impl IWebScriptObject for DOMHTMLTableCellElement {}
impl INSObject for DOMHTMLTableCellElement {}
impl PNSObject for DOMHTMLTableCellElement {}
impl IDOMHTMLTableCellElement for DOMHTMLTableCellElement {}
pub trait IDOMHTMLTableCellElement: Sized + std::ops::Deref {
    unsafe fn cellIndex(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellIndex)
    }
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn axis(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, axis)
    }
    unsafe fn setAxis_(&self, axis: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAxis : axis)
    }
    unsafe fn bgColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bgColor)
    }
    unsafe fn setBgColor_(&self, bgColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBgColor : bgColor)
    }
    unsafe fn ch(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ch)
    }
    unsafe fn setCh_(&self, ch: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCh : ch)
    }
    unsafe fn chOff(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chOff)
    }
    unsafe fn setChOff_(&self, chOff: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChOff : chOff)
    }
    unsafe fn colSpan(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colSpan)
    }
    unsafe fn setColSpan_(&self, colSpan: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColSpan : colSpan)
    }
    unsafe fn rowSpan(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rowSpan)
    }
    unsafe fn setRowSpan_(&self, rowSpan: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRowSpan : rowSpan)
    }
    unsafe fn headers(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headers)
    }
    unsafe fn setHeaders_(&self, headers: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaders : headers)
    }
    unsafe fn height(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn noWrap(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noWrap)
    }
    unsafe fn setNoWrap_(&self, noWrap: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNoWrap : noWrap)
    }
    unsafe fn vAlign(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vAlign)
    }
    unsafe fn setVAlign_(&self, vAlign: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVAlign : vAlign)
    }
    unsafe fn width(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn abbr(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, abbr)
    }
    unsafe fn setAbbr_(&self, abbr: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAbbr : abbr)
    }
    unsafe fn scope(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scope)
    }
    unsafe fn setScope_(&self, scope: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScope : scope)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLTableColElement(pub id);
impl std::ops::Deref for DOMHTMLTableColElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLTableColElement {}
impl DOMHTMLTableColElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLTableColElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLTableColElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLTableColElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLTableColElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLTableColElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLTableColElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLTableColElement")
        }
    }
}
impl IDOMElement for DOMHTMLTableColElement {}
impl IDOMNode for DOMHTMLTableColElement {}
impl PDOMEventTarget for DOMHTMLTableColElement {}
impl IDOMObject for DOMHTMLTableColElement {}
impl PNSCopying for DOMHTMLTableColElement {}
impl IWebScriptObject for DOMHTMLTableColElement {}
impl INSObject for DOMHTMLTableColElement {}
impl PNSObject for DOMHTMLTableColElement {}
impl IDOMHTMLTableColElement for DOMHTMLTableColElement {}
pub trait IDOMHTMLTableColElement: Sized + std::ops::Deref {
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn ch(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ch)
    }
    unsafe fn setCh_(&self, ch: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCh : ch)
    }
    unsafe fn chOff(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chOff)
    }
    unsafe fn setChOff_(&self, chOff: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChOff : chOff)
    }
    unsafe fn span(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, span)
    }
    unsafe fn setSpan_(&self, span: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpan : span)
    }
    unsafe fn vAlign(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vAlign)
    }
    unsafe fn setVAlign_(&self, vAlign: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVAlign : vAlign)
    }
    unsafe fn width(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLTableElement(pub id);
impl std::ops::Deref for DOMHTMLTableElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLTableElement {}
impl DOMHTMLTableElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLTableElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLTableElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLTableElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLTableElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLTableElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLTableElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLTableElement")
        }
    }
}
impl IDOMElement for DOMHTMLTableElement {}
impl IDOMNode for DOMHTMLTableElement {}
impl PDOMEventTarget for DOMHTMLTableElement {}
impl IDOMObject for DOMHTMLTableElement {}
impl PNSCopying for DOMHTMLTableElement {}
impl IWebScriptObject for DOMHTMLTableElement {}
impl INSObject for DOMHTMLTableElement {}
impl PNSObject for DOMHTMLTableElement {}
impl IDOMHTMLTableElement for DOMHTMLTableElement {}
pub trait IDOMHTMLTableElement: Sized + std::ops::Deref {
    unsafe fn createTHead(&self) -> DOMHTMLElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createTHead)
    }
    unsafe fn deleteTHead(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteTHead)
    }
    unsafe fn createTFoot(&self) -> DOMHTMLElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createTFoot)
    }
    unsafe fn deleteTFoot(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteTFoot)
    }
    unsafe fn createCaption(&self) -> DOMHTMLElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createCaption)
    }
    unsafe fn deleteCaption(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteCaption)
    }
    unsafe fn insertRow_(&self, index: ::std::os::raw::c_int) -> DOMHTMLElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertRow : index)
    }
    unsafe fn deleteRow_(&self, index: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteRow : index)
    }
    unsafe fn caption(&self) -> DOMHTMLTableCaptionElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, caption)
    }
    unsafe fn setCaption_(&self, caption: DOMHTMLTableCaptionElement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaption : caption)
    }
    unsafe fn tHead(&self) -> DOMHTMLTableSectionElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tHead)
    }
    unsafe fn setTHead_(&self, tHead: DOMHTMLTableSectionElement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTHead : tHead)
    }
    unsafe fn tFoot(&self) -> DOMHTMLTableSectionElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tFoot)
    }
    unsafe fn setTFoot_(&self, tFoot: DOMHTMLTableSectionElement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTFoot : tFoot)
    }
    unsafe fn rows(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rows)
    }
    unsafe fn tBodies(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tBodies)
    }
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn bgColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bgColor)
    }
    unsafe fn setBgColor_(&self, bgColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBgColor : bgColor)
    }
    unsafe fn border(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, border)
    }
    unsafe fn setBorder_(&self, border: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorder : border)
    }
    unsafe fn cellPadding(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellPadding)
    }
    unsafe fn setCellPadding_(&self, cellPadding: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCellPadding : cellPadding)
    }
    unsafe fn cellSpacing(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellSpacing)
    }
    unsafe fn setCellSpacing_(&self, cellSpacing: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCellSpacing : cellSpacing)
    }
    unsafe fn frameBorders(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameBorders)
    }
    unsafe fn setFrameBorders_(&self, frameBorders: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameBorders : frameBorders)
    }
    unsafe fn rules(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rules)
    }
    unsafe fn setRules_(&self, rules: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRules : rules)
    }
    unsafe fn summary(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summary)
    }
    unsafe fn setSummary_(&self, summary: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSummary : summary)
    }
    unsafe fn width(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLTableRowElement(pub id);
impl std::ops::Deref for DOMHTMLTableRowElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLTableRowElement {}
impl DOMHTMLTableRowElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLTableRowElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLTableRowElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLTableRowElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLTableRowElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLTableRowElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLTableRowElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLTableRowElement")
        }
    }
}
impl IDOMElement for DOMHTMLTableRowElement {}
impl IDOMNode for DOMHTMLTableRowElement {}
impl PDOMEventTarget for DOMHTMLTableRowElement {}
impl IDOMObject for DOMHTMLTableRowElement {}
impl PNSCopying for DOMHTMLTableRowElement {}
impl IWebScriptObject for DOMHTMLTableRowElement {}
impl INSObject for DOMHTMLTableRowElement {}
impl PNSObject for DOMHTMLTableRowElement {}
impl IDOMHTMLTableRowElement for DOMHTMLTableRowElement {}
pub trait IDOMHTMLTableRowElement: Sized + std::ops::Deref {
    unsafe fn insertCell_(&self, index: ::std::os::raw::c_int) -> DOMHTMLElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertCell : index)
    }
    unsafe fn deleteCell_(&self, index: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteCell : index)
    }
    unsafe fn rowIndex(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rowIndex)
    }
    unsafe fn sectionRowIndex(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sectionRowIndex)
    }
    unsafe fn cells(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cells)
    }
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn bgColor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bgColor)
    }
    unsafe fn setBgColor_(&self, bgColor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBgColor : bgColor)
    }
    unsafe fn ch(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ch)
    }
    unsafe fn setCh_(&self, ch: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCh : ch)
    }
    unsafe fn chOff(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chOff)
    }
    unsafe fn setChOff_(&self, chOff: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChOff : chOff)
    }
    unsafe fn vAlign(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vAlign)
    }
    unsafe fn setVAlign_(&self, vAlign: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVAlign : vAlign)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLTableSectionElement(pub id);
impl std::ops::Deref for DOMHTMLTableSectionElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLTableSectionElement {}
impl DOMHTMLTableSectionElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLTableSectionElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLTableSectionElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLTableSectionElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLTableSectionElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLTableSectionElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLTableSectionElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLTableSectionElement")
        }
    }
}
impl IDOMElement for DOMHTMLTableSectionElement {}
impl IDOMNode for DOMHTMLTableSectionElement {}
impl PDOMEventTarget for DOMHTMLTableSectionElement {}
impl IDOMObject for DOMHTMLTableSectionElement {}
impl PNSCopying for DOMHTMLTableSectionElement {}
impl IWebScriptObject for DOMHTMLTableSectionElement {}
impl INSObject for DOMHTMLTableSectionElement {}
impl PNSObject for DOMHTMLTableSectionElement {}
impl IDOMHTMLTableSectionElement for DOMHTMLTableSectionElement {}
pub trait IDOMHTMLTableSectionElement: Sized + std::ops::Deref {
    unsafe fn insertRow_(&self, index: ::std::os::raw::c_int) -> DOMHTMLElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertRow : index)
    }
    unsafe fn deleteRow_(&self, index: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteRow : index)
    }
    unsafe fn align(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, align)
    }
    unsafe fn setAlign_(&self, align: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlign : align)
    }
    unsafe fn ch(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ch)
    }
    unsafe fn setCh_(&self, ch: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCh : ch)
    }
    unsafe fn chOff(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chOff)
    }
    unsafe fn setChOff_(&self, chOff: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChOff : chOff)
    }
    unsafe fn vAlign(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vAlign)
    }
    unsafe fn setVAlign_(&self, vAlign: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVAlign : vAlign)
    }
    unsafe fn rows(&self) -> DOMHTMLCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rows)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLTextAreaElement(pub id);
impl std::ops::Deref for DOMHTMLTextAreaElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLTextAreaElement {}
impl DOMHTMLTextAreaElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLTextAreaElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLTextAreaElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLTextAreaElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLTextAreaElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLTextAreaElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLTextAreaElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLTextAreaElement")
        }
    }
}
impl IDOMElement for DOMHTMLTextAreaElement {}
impl IDOMNode for DOMHTMLTextAreaElement {}
impl PDOMEventTarget for DOMHTMLTextAreaElement {}
impl IDOMObject for DOMHTMLTextAreaElement {}
impl PNSCopying for DOMHTMLTextAreaElement {}
impl IWebScriptObject for DOMHTMLTextAreaElement {}
impl INSObject for DOMHTMLTextAreaElement {}
impl PNSObject for DOMHTMLTextAreaElement {}
impl IDOMHTMLTextAreaElement for DOMHTMLTextAreaElement {}
pub trait IDOMHTMLTextAreaElement: Sized + std::ops::Deref {
    unsafe fn select(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, select)
    }
    unsafe fn setSelectionRange_end_(
        &self,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionRange : start, end : end)
    }
    unsafe fn autofocus(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autofocus)
    }
    unsafe fn setAutofocus_(&self, autofocus: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutofocus : autofocus)
    }
    unsafe fn disabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabled)
    }
    unsafe fn setDisabled_(&self, disabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisabled : disabled)
    }
    unsafe fn form(&self) -> DOMHTMLFormElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, form)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn readOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readOnly)
    }
    unsafe fn setReadOnly_(&self, readOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadOnly : readOnly)
    }
    unsafe fn rows(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rows)
    }
    unsafe fn setRows_(&self, rows: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRows : rows)
    }
    unsafe fn cols(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cols)
    }
    unsafe fn setCols_(&self, cols: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCols : cols)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn defaultValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultValue)
    }
    unsafe fn setDefaultValue_(&self, defaultValue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultValue : defaultValue)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn willValidate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willValidate)
    }
    unsafe fn selectionStart(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionStart)
    }
    unsafe fn setSelectionStart_(&self, selectionStart: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionStart : selectionStart)
    }
    unsafe fn selectionEnd(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionEnd)
    }
    unsafe fn setSelectionEnd_(&self, selectionEnd: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionEnd : selectionEnd)
    }
    unsafe fn accessKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessKey)
    }
    unsafe fn setAccessKey_(&self, accessKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessKey : accessKey)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLTitleElement(pub id);
impl std::ops::Deref for DOMHTMLTitleElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLTitleElement {}
impl DOMHTMLTitleElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLTitleElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLTitleElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLTitleElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLTitleElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLTitleElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLTitleElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLTitleElement")
        }
    }
}
impl IDOMElement for DOMHTMLTitleElement {}
impl IDOMNode for DOMHTMLTitleElement {}
impl PDOMEventTarget for DOMHTMLTitleElement {}
impl IDOMObject for DOMHTMLTitleElement {}
impl PNSCopying for DOMHTMLTitleElement {}
impl IWebScriptObject for DOMHTMLTitleElement {}
impl INSObject for DOMHTMLTitleElement {}
impl PNSObject for DOMHTMLTitleElement {}
impl IDOMHTMLTitleElement for DOMHTMLTitleElement {}
pub trait IDOMHTMLTitleElement: Sized + std::ops::Deref {
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMHTMLUListElement(pub id);
impl std::ops::Deref for DOMHTMLUListElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMHTMLUListElement {}
impl DOMHTMLUListElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMHTMLUListElement").unwrap(), alloc) })
    }
}
impl IDOMHTMLElement for DOMHTMLUListElement {}
impl std::convert::TryFrom<DOMHTMLElement> for DOMHTMLUListElement {
    type Error = &'static str;
    fn try_from(parent: DOMHTMLElement) -> Result<DOMHTMLUListElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMHTMLUListElement").unwrap()) };
        if is_kind_of {
            Ok(DOMHTMLUListElement(parent.0))
        } else {
            Err("This DOMHTMLElement cannot be downcasted to DOMHTMLUListElement")
        }
    }
}
impl IDOMElement for DOMHTMLUListElement {}
impl IDOMNode for DOMHTMLUListElement {}
impl PDOMEventTarget for DOMHTMLUListElement {}
impl IDOMObject for DOMHTMLUListElement {}
impl PNSCopying for DOMHTMLUListElement {}
impl IWebScriptObject for DOMHTMLUListElement {}
impl INSObject for DOMHTMLUListElement {}
impl PNSObject for DOMHTMLUListElement {}
impl IDOMHTMLUListElement for DOMHTMLUListElement {}
pub trait IDOMHTMLUListElement: Sized + std::ops::Deref {
    unsafe fn compact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compact)
    }
    unsafe fn setCompact_(&self, compact: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompact : compact)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMRange(pub id);
impl std::ops::Deref for DOMRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMRange {}
impl DOMRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMRange").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMRange {}
impl PNSCopying for DOMRange {}
impl std::convert::TryFrom<DOMObject> for DOMRange {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMRange, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMRange").unwrap()) };
        if is_kind_of {
            Ok(DOMRange(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMRange")
        }
    }
}
impl IWebScriptObject for DOMRange {}
impl INSObject for DOMRange {}
impl PNSObject for DOMRange {}
impl IDOMRange for DOMRange {}
pub trait IDOMRange: Sized + std::ops::Deref {
    unsafe fn setStart_offset_(&self, refNode: DOMNode, offset: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStart : refNode, offset : offset)
    }
    unsafe fn setEnd_offset_(&self, refNode: DOMNode, offset: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnd : refNode, offset : offset)
    }
    unsafe fn setStartBefore_(&self, refNode: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartBefore : refNode)
    }
    unsafe fn setStartAfter_(&self, refNode: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartAfter : refNode)
    }
    unsafe fn setEndBefore_(&self, refNode: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndBefore : refNode)
    }
    unsafe fn setEndAfter_(&self, refNode: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndAfter : refNode)
    }
    unsafe fn collapse_(&self, toStart: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, collapse : toStart)
    }
    unsafe fn selectNode_(&self, refNode: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectNode : refNode)
    }
    unsafe fn selectNodeContents_(&self, refNode: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectNodeContents : refNode)
    }
    unsafe fn compareBoundaryPoints_sourceRange_(
        &self,
        how: ::std::os::raw::c_ushort,
        sourceRange: DOMRange,
    ) -> ::std::os::raw::c_short
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareBoundaryPoints : how, sourceRange : sourceRange)
    }
    unsafe fn deleteContents(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteContents)
    }
    unsafe fn extractContents(&self) -> DOMDocumentFragment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extractContents)
    }
    unsafe fn cloneContents(&self) -> DOMDocumentFragment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloneContents)
    }
    unsafe fn insertNode_(&self, newNode: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertNode : newNode)
    }
    unsafe fn surroundContents_(&self, newParent: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, surroundContents : newParent)
    }
    unsafe fn cloneRange(&self) -> DOMRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloneRange)
    }
    unsafe fn toString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toString)
    }
    unsafe fn detach(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detach)
    }
    unsafe fn createContextualFragment_(&self, html: NSString) -> DOMDocumentFragment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createContextualFragment : html)
    }
    unsafe fn compareNode_(&self, refNode: DOMNode) -> ::std::os::raw::c_short
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareNode : refNode)
    }
    unsafe fn intersectsNode_(&self, refNode: DOMNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intersectsNode : refNode)
    }
    unsafe fn comparePoint_offset_(
        &self,
        refNode: DOMNode,
        offset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_short
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, comparePoint : refNode, offset : offset)
    }
    unsafe fn isPointInRange_offset_(&self, refNode: DOMNode, offset: ::std::os::raw::c_int) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isPointInRange : refNode, offset : offset)
    }
    unsafe fn startContainer(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startContainer)
    }
    unsafe fn startOffset(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startOffset)
    }
    unsafe fn endContainer(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endContainer)
    }
    unsafe fn endOffset(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endOffset)
    }
    unsafe fn collapsed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collapsed)
    }
    unsafe fn commonAncestorContainer(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commonAncestorContainer)
    }
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
}
impl DOMRange_DOMRangeDeprecated for DOMRange {}
pub trait DOMRange_DOMRangeDeprecated: Sized + std::ops::Deref {
    unsafe fn setStart__(&self, refNode: DOMNode, offset: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStart : refNode, offset : offset)
    }
    unsafe fn setEnd__(&self, refNode: DOMNode, offset: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnd : refNode, offset : offset)
    }
    unsafe fn compareBoundaryPoints__(
        &self,
        how: ::std::os::raw::c_ushort,
        sourceRange: DOMRange,
    ) -> ::std::os::raw::c_short
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareBoundaryPoints : how, sourceRange : sourceRange)
    }
}
impl DOMNode_DOMNodeExtensions for DOMNode {}
pub trait DOMNode_DOMNodeExtensions: Sized + std::ops::Deref {
    unsafe fn boundingBox(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBox)
    }
    unsafe fn lineBoxRects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineBoxRects)
    }
}
impl DOMElement_DOMElementAppKitExtensions for DOMElement {}
pub trait DOMElement_DOMElementAppKitExtensions: Sized + std::ops::Deref {
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
}
impl DOMHTMLDocument_DOMHTMLDocumentExtensions for DOMHTMLDocument {}
pub trait DOMHTMLDocument_DOMHTMLDocumentExtensions: Sized + std::ops::Deref {
    unsafe fn createDocumentFragmentWithMarkupString_baseURL_(
        &self,
        markupString: NSString,
        baseURL: NSURL,
    ) -> DOMDocumentFragment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createDocumentFragmentWithMarkupString : markupString, baseURL : baseURL)
    }
    unsafe fn createDocumentFragmentWithText_(&self, text: NSString) -> DOMDocumentFragment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createDocumentFragmentWithText : text)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMAbstractView(pub id);
impl std::ops::Deref for DOMAbstractView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMAbstractView {}
impl DOMAbstractView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMAbstractView").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMAbstractView {}
impl PNSCopying for DOMAbstractView {}
impl std::convert::TryFrom<DOMObject> for DOMAbstractView {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMAbstractView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMAbstractView").unwrap()) };
        if is_kind_of {
            Ok(DOMAbstractView(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMAbstractView")
        }
    }
}
impl IWebScriptObject for DOMAbstractView {}
impl INSObject for DOMAbstractView {}
impl PNSObject for DOMAbstractView {}
impl IDOMAbstractView for DOMAbstractView {}
pub trait IDOMAbstractView: Sized + std::ops::Deref {
    unsafe fn document(&self) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, document)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMEvent(pub id);
impl std::ops::Deref for DOMEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMEvent {}
impl DOMEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMEvent").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMEvent {}
impl PNSCopying for DOMEvent {}
impl std::convert::TryFrom<DOMObject> for DOMEvent {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMEvent, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMEvent").unwrap()) };
        if is_kind_of {
            Ok(DOMEvent(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMEvent")
        }
    }
}
impl IWebScriptObject for DOMEvent {}
impl INSObject for DOMEvent {}
impl PNSObject for DOMEvent {}
impl IDOMEvent for DOMEvent {}
pub trait IDOMEvent: Sized + std::ops::Deref {
    unsafe fn stopPropagation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopPropagation)
    }
    unsafe fn preventDefault(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preventDefault)
    }
    unsafe fn initEvent_canBubbleArg_cancelableArg_(
        &self,
        eventTypeArg: NSString,
        canBubbleArg: BOOL,
        cancelableArg: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initEvent : eventTypeArg, canBubbleArg : canBubbleArg, cancelableArg : cancelableArg)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn target(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn currentTarget(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentTarget)
    }
    unsafe fn eventPhase(&self) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventPhase)
    }
    unsafe fn bubbles(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bubbles)
    }
    unsafe fn cancelable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelable)
    }
    unsafe fn timeStamp(&self) -> DOMTimeStamp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeStamp)
    }
    unsafe fn srcElement(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, srcElement)
    }
    unsafe fn returnValue(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, returnValue)
    }
    unsafe fn setReturnValue_(&self, returnValue: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReturnValue : returnValue)
    }
    unsafe fn cancelBubble(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelBubble)
    }
    unsafe fn setCancelBubble_(&self, cancelBubble: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCancelBubble : cancelBubble)
    }
}
impl DOMEvent_DOMEventDeprecated for DOMEvent {}
pub trait DOMEvent_DOMEventDeprecated: Sized + std::ops::Deref {
    unsafe fn initEvent___(&self, eventTypeArg: NSString, canBubbleArg: BOOL, cancelableArg: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initEvent : eventTypeArg, canBubbleArg : canBubbleArg, cancelableArg : cancelableArg)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMUIEvent(pub id);
impl std::ops::Deref for DOMUIEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMUIEvent {}
impl DOMUIEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMUIEvent").unwrap(), alloc) })
    }
}
impl IDOMEvent for DOMUIEvent {}
impl From<DOMUIEvent> for DOMEvent {
    fn from(child: DOMUIEvent) -> DOMEvent {
        DOMEvent(child.0)
    }
}
impl std::convert::TryFrom<DOMEvent> for DOMUIEvent {
    type Error = &'static str;
    fn try_from(parent: DOMEvent) -> Result<DOMUIEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMUIEvent").unwrap()) };
        if is_kind_of {
            Ok(DOMUIEvent(parent.0))
        } else {
            Err("This DOMEvent cannot be downcasted to DOMUIEvent")
        }
    }
}
impl IDOMObject for DOMUIEvent {}
impl PNSCopying for DOMUIEvent {}
impl IWebScriptObject for DOMUIEvent {}
impl INSObject for DOMUIEvent {}
impl PNSObject for DOMUIEvent {}
impl IDOMUIEvent for DOMUIEvent {}
pub trait IDOMUIEvent: Sized + std::ops::Deref {
    unsafe fn initUIEvent_canBubble_cancelable_view_detail_(
        &self,
        type_: NSString,
        canBubble: BOOL,
        cancelable: BOOL,
        view: DOMAbstractView,
        detail: ::std::os::raw::c_int,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initUIEvent : type_, canBubble : canBubble, cancelable : cancelable, view : view, detail : detail)
    }
    unsafe fn view(&self) -> DOMAbstractView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, view)
    }
    unsafe fn detail(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detail)
    }
    unsafe fn keyCode(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyCode)
    }
    unsafe fn charCode(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charCode)
    }
    unsafe fn layerX(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerX)
    }
    unsafe fn layerY(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerY)
    }
    unsafe fn pageX(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageX)
    }
    unsafe fn pageY(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageY)
    }
    unsafe fn which(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, which)
    }
}
impl DOMUIEvent_DOMUIEventDeprecated for DOMUIEvent {}
pub trait DOMUIEvent_DOMUIEventDeprecated: Sized + std::ops::Deref {
    unsafe fn initUIEvent_____(
        &self,
        type_: NSString,
        canBubble: BOOL,
        cancelable: BOOL,
        view: DOMAbstractView,
        detail: ::std::os::raw::c_int,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initUIEvent : type_, canBubble : canBubble, cancelable : cancelable, view : view, detail : detail)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMKeyboardEvent(pub id);
impl std::ops::Deref for DOMKeyboardEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMKeyboardEvent {}
impl DOMKeyboardEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMKeyboardEvent").unwrap(), alloc) })
    }
}
impl IDOMUIEvent for DOMKeyboardEvent {}
impl From<DOMKeyboardEvent> for DOMUIEvent {
    fn from(child: DOMKeyboardEvent) -> DOMUIEvent {
        DOMUIEvent(child.0)
    }
}
impl std::convert::TryFrom<DOMUIEvent> for DOMKeyboardEvent {
    type Error = &'static str;
    fn try_from(parent: DOMUIEvent) -> Result<DOMKeyboardEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMKeyboardEvent").unwrap()) };
        if is_kind_of {
            Ok(DOMKeyboardEvent(parent.0))
        } else {
            Err("This DOMUIEvent cannot be downcasted to DOMKeyboardEvent")
        }
    }
}
impl IDOMEvent for DOMKeyboardEvent {}
impl IDOMObject for DOMKeyboardEvent {}
impl PNSCopying for DOMKeyboardEvent {}
impl IWebScriptObject for DOMKeyboardEvent {}
impl INSObject for DOMKeyboardEvent {}
impl PNSObject for DOMKeyboardEvent {}
impl IDOMKeyboardEvent for DOMKeyboardEvent {}
pub trait IDOMKeyboardEvent: Sized + std::ops::Deref {
    unsafe fn getModifierState_(&self, keyIdentifierArg: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getModifierState : keyIdentifierArg)
    }
    unsafe fn initKeyboardEvent_canBubble_cancelable_view_keyIdentifier_location_ctrlKey_altKey_shiftKey_metaKey_altGraphKey_(
        &self,
        type_: NSString,
        canBubble: BOOL,
        cancelable: BOOL,
        view: DOMAbstractView,
        keyIdentifier: NSString,
        location: ::std::os::raw::c_uint,
        ctrlKey: BOOL,
        altKey: BOOL,
        shiftKey: BOOL,
        metaKey: BOOL,
        altGraphKey: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initKeyboardEvent : type_, canBubble : canBubble, cancelable : cancelable, view : view, keyIdentifier : keyIdentifier, location : location, ctrlKey : ctrlKey, altKey : altKey, shiftKey : shiftKey, metaKey : metaKey, altGraphKey : altGraphKey)
    }
    unsafe fn initKeyboardEvent_canBubble_cancelable_view_keyIdentifier_location_ctrlKey_altKey_shiftKey_metaKey_(
        &self,
        type_: NSString,
        canBubble: BOOL,
        cancelable: BOOL,
        view: DOMAbstractView,
        keyIdentifier: NSString,
        location: ::std::os::raw::c_uint,
        ctrlKey: BOOL,
        altKey: BOOL,
        shiftKey: BOOL,
        metaKey: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initKeyboardEvent : type_, canBubble : canBubble, cancelable : cancelable, view : view, keyIdentifier : keyIdentifier, location : location, ctrlKey : ctrlKey, altKey : altKey, shiftKey : shiftKey, metaKey : metaKey)
    }
    unsafe fn initKeyboardEvent_canBubble_cancelable_view_keyIdentifier_keyLocation_ctrlKey_altKey_shiftKey_metaKey_altGraphKey_(
        &self,
        type_: NSString,
        canBubble: BOOL,
        cancelable: BOOL,
        view: DOMAbstractView,
        keyIdentifier: NSString,
        keyLocation: ::std::os::raw::c_uint,
        ctrlKey: BOOL,
        altKey: BOOL,
        shiftKey: BOOL,
        metaKey: BOOL,
        altGraphKey: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initKeyboardEvent : type_, canBubble : canBubble, cancelable : cancelable, view : view, keyIdentifier : keyIdentifier, keyLocation : keyLocation, ctrlKey : ctrlKey, altKey : altKey, shiftKey : shiftKey, metaKey : metaKey, altGraphKey : altGraphKey)
    }
    unsafe fn initKeyboardEvent_canBubble_cancelable_view_keyIdentifier_keyLocation_ctrlKey_altKey_shiftKey_metaKey_(
        &self,
        type_: NSString,
        canBubble: BOOL,
        cancelable: BOOL,
        view: DOMAbstractView,
        keyIdentifier: NSString,
        keyLocation: ::std::os::raw::c_uint,
        ctrlKey: BOOL,
        altKey: BOOL,
        shiftKey: BOOL,
        metaKey: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initKeyboardEvent : type_, canBubble : canBubble, cancelable : cancelable, view : view, keyIdentifier : keyIdentifier, keyLocation : keyLocation, ctrlKey : ctrlKey, altKey : altKey, shiftKey : shiftKey, metaKey : metaKey)
    }
    unsafe fn keyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyIdentifier)
    }
    unsafe fn location(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn keyLocation(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyLocation)
    }
    unsafe fn ctrlKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ctrlKey)
    }
    unsafe fn shiftKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shiftKey)
    }
    unsafe fn altKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altKey)
    }
    unsafe fn metaKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metaKey)
    }
    unsafe fn altGraphKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altGraphKey)
    }
    unsafe fn keyCode(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyCode)
    }
    unsafe fn charCode(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charCode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMMouseEvent(pub id);
impl std::ops::Deref for DOMMouseEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMMouseEvent {}
impl DOMMouseEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMMouseEvent").unwrap(), alloc) })
    }
}
impl IDOMUIEvent for DOMMouseEvent {}
impl std::convert::TryFrom<DOMUIEvent> for DOMMouseEvent {
    type Error = &'static str;
    fn try_from(parent: DOMUIEvent) -> Result<DOMMouseEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMMouseEvent").unwrap()) };
        if is_kind_of {
            Ok(DOMMouseEvent(parent.0))
        } else {
            Err("This DOMUIEvent cannot be downcasted to DOMMouseEvent")
        }
    }
}
impl IDOMEvent for DOMMouseEvent {}
impl IDOMObject for DOMMouseEvent {}
impl PNSCopying for DOMMouseEvent {}
impl IWebScriptObject for DOMMouseEvent {}
impl INSObject for DOMMouseEvent {}
impl PNSObject for DOMMouseEvent {}
impl IDOMMouseEvent for DOMMouseEvent {}
pub trait IDOMMouseEvent: Sized + std::ops::Deref {
    unsafe fn screenX(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenX)
    }
    unsafe fn screenY(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenY)
    }
    unsafe fn clientX(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientX)
    }
    unsafe fn clientY(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientY)
    }
    unsafe fn ctrlKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ctrlKey)
    }
    unsafe fn shiftKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shiftKey)
    }
    unsafe fn altKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altKey)
    }
    unsafe fn metaKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metaKey)
    }
    unsafe fn button(&self) -> ::std::os::raw::c_short
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, button)
    }
    unsafe fn relatedTarget(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relatedTarget)
    }
    unsafe fn offsetX(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsetX)
    }
    unsafe fn offsetY(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsetY)
    }
    unsafe fn x(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, x)
    }
    unsafe fn y(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, y)
    }
    unsafe fn fromElement(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fromElement)
    }
    unsafe fn toElement(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toElement)
    }
}
impl DOMMouseEvent_DOMMouseEventDeprecated for DOMMouseEvent {}
pub trait DOMMouseEvent_DOMMouseEventDeprecated: Sized + std::ops::Deref {
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMMutationEvent(pub id);
impl std::ops::Deref for DOMMutationEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMMutationEvent {}
impl DOMMutationEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMMutationEvent").unwrap(), alloc) })
    }
}
impl IDOMEvent for DOMMutationEvent {}
impl std::convert::TryFrom<DOMEvent> for DOMMutationEvent {
    type Error = &'static str;
    fn try_from(parent: DOMEvent) -> Result<DOMMutationEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMMutationEvent").unwrap()) };
        if is_kind_of {
            Ok(DOMMutationEvent(parent.0))
        } else {
            Err("This DOMEvent cannot be downcasted to DOMMutationEvent")
        }
    }
}
impl IDOMObject for DOMMutationEvent {}
impl PNSCopying for DOMMutationEvent {}
impl IWebScriptObject for DOMMutationEvent {}
impl INSObject for DOMMutationEvent {}
impl PNSObject for DOMMutationEvent {}
impl IDOMMutationEvent for DOMMutationEvent {}
pub trait IDOMMutationEvent: Sized + std::ops::Deref {
    unsafe fn newValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newValue)
    }
    unsafe fn initMutationEvent_canBubble_cancelable_relatedNode_prevValue_newValue_attrName_attrChange_(
        &self,
        type_: NSString,
        canBubble: BOOL,
        cancelable: BOOL,
        relatedNode: DOMNode,
        prevValue: NSString,
        newValue: NSString,
        attrName: NSString,
        attrChange: ::std::os::raw::c_ushort,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initMutationEvent : type_, canBubble : canBubble, cancelable : cancelable, relatedNode : relatedNode, prevValue : prevValue, newValue : newValue, attrName : attrName, attrChange : attrChange)
    }
    unsafe fn relatedNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relatedNode)
    }
    unsafe fn prevValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prevValue)
    }
    unsafe fn attrName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attrName)
    }
    unsafe fn attrChange(&self) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attrChange)
    }
}
impl DOMMutationEvent_DOMMutationEventDeprecated for DOMMutationEvent {}
pub trait DOMMutationEvent_DOMMutationEventDeprecated: Sized + std::ops::Deref {
    unsafe fn initMutationEvent________(
        &self,
        type_: NSString,
        canBubble: BOOL,
        cancelable: BOOL,
        relatedNode: DOMNode,
        prevValue: NSString,
        newValue: NSString,
        attrName: NSString,
        attrChange: ::std::os::raw::c_ushort,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initMutationEvent : type_, canBubble : canBubble, cancelable : cancelable, relatedNode : relatedNode, prevValue : prevValue, newValue : newValue, attrName : attrName, attrChange : attrChange)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMOverflowEvent(pub id);
impl std::ops::Deref for DOMOverflowEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMOverflowEvent {}
impl DOMOverflowEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMOverflowEvent").unwrap(), alloc) })
    }
}
impl IDOMEvent for DOMOverflowEvent {}
impl std::convert::TryFrom<DOMEvent> for DOMOverflowEvent {
    type Error = &'static str;
    fn try_from(parent: DOMEvent) -> Result<DOMOverflowEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMOverflowEvent").unwrap()) };
        if is_kind_of {
            Ok(DOMOverflowEvent(parent.0))
        } else {
            Err("This DOMEvent cannot be downcasted to DOMOverflowEvent")
        }
    }
}
impl IDOMObject for DOMOverflowEvent {}
impl PNSCopying for DOMOverflowEvent {}
impl IWebScriptObject for DOMOverflowEvent {}
impl INSObject for DOMOverflowEvent {}
impl PNSObject for DOMOverflowEvent {}
impl IDOMOverflowEvent for DOMOverflowEvent {}
pub trait IDOMOverflowEvent: Sized + std::ops::Deref {
    unsafe fn initOverflowEvent_horizontalOverflow_verticalOverflow_(
        &self,
        orient: ::std::os::raw::c_ushort,
        horizontalOverflow: BOOL,
        verticalOverflow: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initOverflowEvent : orient, horizontalOverflow : horizontalOverflow, verticalOverflow : verticalOverflow)
    }
    unsafe fn orient(&self) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orient)
    }
    unsafe fn horizontalOverflow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalOverflow)
    }
    unsafe fn verticalOverflow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalOverflow)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMProgressEvent(pub id);
impl std::ops::Deref for DOMProgressEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMProgressEvent {}
impl DOMProgressEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMProgressEvent").unwrap(), alloc) })
    }
}
impl IDOMEvent for DOMProgressEvent {}
impl std::convert::TryFrom<DOMEvent> for DOMProgressEvent {
    type Error = &'static str;
    fn try_from(parent: DOMEvent) -> Result<DOMProgressEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMProgressEvent").unwrap()) };
        if is_kind_of {
            Ok(DOMProgressEvent(parent.0))
        } else {
            Err("This DOMEvent cannot be downcasted to DOMProgressEvent")
        }
    }
}
impl IDOMObject for DOMProgressEvent {}
impl PNSCopying for DOMProgressEvent {}
impl IWebScriptObject for DOMProgressEvent {}
impl INSObject for DOMProgressEvent {}
impl PNSObject for DOMProgressEvent {}
impl IDOMProgressEvent for DOMProgressEvent {}
pub trait IDOMProgressEvent: Sized + std::ops::Deref {
    unsafe fn lengthComputable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lengthComputable)
    }
    unsafe fn loaded(&self) -> ::std::os::raw::c_ulonglong
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loaded)
    }
    unsafe fn total(&self) -> ::std::os::raw::c_ulonglong
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, total)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMWheelEvent(pub id);
impl std::ops::Deref for DOMWheelEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMWheelEvent {}
impl DOMWheelEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMWheelEvent").unwrap(), alloc) })
    }
}
impl IDOMMouseEvent for DOMWheelEvent {}
impl From<DOMWheelEvent> for DOMMouseEvent {
    fn from(child: DOMWheelEvent) -> DOMMouseEvent {
        DOMMouseEvent(child.0)
    }
}
impl std::convert::TryFrom<DOMMouseEvent> for DOMWheelEvent {
    type Error = &'static str;
    fn try_from(parent: DOMMouseEvent) -> Result<DOMWheelEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMWheelEvent").unwrap()) };
        if is_kind_of {
            Ok(DOMWheelEvent(parent.0))
        } else {
            Err("This DOMMouseEvent cannot be downcasted to DOMWheelEvent")
        }
    }
}
impl IDOMUIEvent for DOMWheelEvent {}
impl IDOMEvent for DOMWheelEvent {}
impl IDOMObject for DOMWheelEvent {}
impl PNSCopying for DOMWheelEvent {}
impl IWebScriptObject for DOMWheelEvent {}
impl INSObject for DOMWheelEvent {}
impl PNSObject for DOMWheelEvent {}
impl IDOMWheelEvent for DOMWheelEvent {}
pub trait IDOMWheelEvent: Sized + std::ops::Deref {
    unsafe fn initWheelEvent_wheelDeltaY_view_screenX_screenY_clientX_clientY_ctrlKey_altKey_shiftKey_metaKey_(
        &self,
        wheelDeltaX: ::std::os::raw::c_int,
        wheelDeltaY: ::std::os::raw::c_int,
        view: DOMAbstractView,
        screenX: ::std::os::raw::c_int,
        screenY: ::std::os::raw::c_int,
        clientX: ::std::os::raw::c_int,
        clientY: ::std::os::raw::c_int,
        ctrlKey: BOOL,
        altKey: BOOL,
        shiftKey: BOOL,
        metaKey: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWheelEvent : wheelDeltaX, wheelDeltaY : wheelDeltaY, view : view, screenX : screenX, screenY : screenY, clientX : clientX, clientY : clientY, ctrlKey : ctrlKey, altKey : altKey, shiftKey : shiftKey, metaKey : metaKey)
    }
    unsafe fn wheelDeltaX(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wheelDeltaX)
    }
    unsafe fn wheelDeltaY(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wheelDeltaY)
    }
    unsafe fn wheelDelta(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wheelDelta)
    }
    unsafe fn isHorizontal(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHorizontal)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMNodeIterator(pub id);
impl std::ops::Deref for DOMNodeIterator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMNodeIterator {}
impl DOMNodeIterator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMNodeIterator").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMNodeIterator {}
impl PNSCopying for DOMNodeIterator {}
impl std::convert::TryFrom<DOMObject> for DOMNodeIterator {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMNodeIterator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMNodeIterator").unwrap()) };
        if is_kind_of {
            Ok(DOMNodeIterator(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMNodeIterator")
        }
    }
}
impl IWebScriptObject for DOMNodeIterator {}
impl INSObject for DOMNodeIterator {}
impl PNSObject for DOMNodeIterator {}
impl IDOMNodeIterator for DOMNodeIterator {}
pub trait IDOMNodeIterator: Sized + std::ops::Deref {
    unsafe fn nextNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextNode)
    }
    unsafe fn previousNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousNode)
    }
    unsafe fn detach(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detach)
    }
    unsafe fn root(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, root)
    }
    unsafe fn whatToShow(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, whatToShow)
    }
    unsafe fn filter(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filter)
    }
    unsafe fn expandEntityReferences(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expandEntityReferences)
    }
    unsafe fn referenceNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, referenceNode)
    }
    unsafe fn pointerBeforeReferenceNode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointerBeforeReferenceNode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMTreeWalker(pub id);
impl std::ops::Deref for DOMTreeWalker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMTreeWalker {}
impl DOMTreeWalker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMTreeWalker").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMTreeWalker {}
impl PNSCopying for DOMTreeWalker {}
impl std::convert::TryFrom<DOMObject> for DOMTreeWalker {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMTreeWalker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMTreeWalker").unwrap()) };
        if is_kind_of {
            Ok(DOMTreeWalker(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMTreeWalker")
        }
    }
}
impl IWebScriptObject for DOMTreeWalker {}
impl INSObject for DOMTreeWalker {}
impl PNSObject for DOMTreeWalker {}
impl IDOMTreeWalker for DOMTreeWalker {}
pub trait IDOMTreeWalker: Sized + std::ops::Deref {
    unsafe fn parentNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentNode)
    }
    unsafe fn firstChild(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstChild)
    }
    unsafe fn lastChild(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastChild)
    }
    unsafe fn previousSibling(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousSibling)
    }
    unsafe fn nextSibling(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextSibling)
    }
    unsafe fn previousNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousNode)
    }
    unsafe fn nextNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextNode)
    }
    unsafe fn root(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, root)
    }
    unsafe fn whatToShow(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, whatToShow)
    }
    unsafe fn filter(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filter)
    }
    unsafe fn expandEntityReferences(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expandEntityReferences)
    }
    unsafe fn currentNode(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentNode)
    }
    unsafe fn setCurrentNode_(&self, currentNode: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentNode : currentNode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMXPathExpression(pub id);
impl std::ops::Deref for DOMXPathExpression {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMXPathExpression {}
impl DOMXPathExpression {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMXPathExpression").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMXPathExpression {}
impl PNSCopying for DOMXPathExpression {}
impl std::convert::TryFrom<DOMObject> for DOMXPathExpression {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMXPathExpression, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMXPathExpression").unwrap()) };
        if is_kind_of {
            Ok(DOMXPathExpression(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMXPathExpression")
        }
    }
}
impl IWebScriptObject for DOMXPathExpression {}
impl INSObject for DOMXPathExpression {}
impl PNSObject for DOMXPathExpression {}
impl IDOMXPathExpression for DOMXPathExpression {}
pub trait IDOMXPathExpression: Sized + std::ops::Deref {
    unsafe fn evaluate_type_inResult_(
        &self,
        contextNode: DOMNode,
        type_: ::std::os::raw::c_ushort,
        inResult: DOMXPathResult,
    ) -> DOMXPathResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluate : contextNode, r#type : type_, inResult : inResult)
    }
}
impl DOMXPathExpression_DOMXPathExpressionDeprecated for DOMXPathExpression {}
pub trait DOMXPathExpression_DOMXPathExpressionDeprecated: Sized + std::ops::Deref {
    unsafe fn evaluate___(
        &self,
        contextNode: DOMNode,
        type_: ::std::os::raw::c_ushort,
        inResult: DOMXPathResult,
    ) -> DOMXPathResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluate : contextNode, type_ : type_, inResult : inResult)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DOMXPathResult(pub id);
impl std::ops::Deref for DOMXPathResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DOMXPathResult {}
impl DOMXPathResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DOMXPathResult").unwrap(), alloc) })
    }
}
impl IDOMObject for DOMXPathResult {}
impl PNSCopying for DOMXPathResult {}
impl std::convert::TryFrom<DOMObject> for DOMXPathResult {
    type Error = &'static str;
    fn try_from(parent: DOMObject) -> Result<DOMXPathResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DOMXPathResult").unwrap()) };
        if is_kind_of {
            Ok(DOMXPathResult(parent.0))
        } else {
            Err("This DOMObject cannot be downcasted to DOMXPathResult")
        }
    }
}
impl IWebScriptObject for DOMXPathResult {}
impl INSObject for DOMXPathResult {}
impl PNSObject for DOMXPathResult {}
impl IDOMXPathResult for DOMXPathResult {}
pub trait IDOMXPathResult: Sized + std::ops::Deref {
    unsafe fn iterateNext(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iterateNext)
    }
    unsafe fn snapshotItem_(&self, index: ::std::os::raw::c_uint) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, snapshotItem : index)
    }
    unsafe fn resultType(&self) -> ::std::os::raw::c_ushort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultType)
    }
    unsafe fn numberValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberValue)
    }
    unsafe fn stringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringValue)
    }
    unsafe fn booleanValue(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, booleanValue)
    }
    unsafe fn singleNodeValue(&self) -> DOMNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, singleNodeValue)
    }
    unsafe fn invalidIteratorState(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidIteratorState)
    }
    unsafe fn snapshotLength(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotLength)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebArchive(pub id);
impl std::ops::Deref for WebArchive {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebArchive {}
impl WebArchive {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebArchive").unwrap(), alloc) })
    }
}
impl PNSCoding for WebArchive {}
impl PNSCopying for WebArchive {}
impl INSObject for WebArchive {}
impl PNSObject for WebArchive {}
impl std::convert::TryFrom<NSObject> for WebArchive {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebArchive, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebArchive").unwrap()) };
        if is_kind_of {
            Ok(WebArchive(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebArchive")
        }
    }
}
impl IWebArchive for WebArchive {}
pub trait IWebArchive: Sized + std::ops::Deref {
    unsafe fn initWithMainResource_subresources_subframeArchives_(
        &self,
        mainResource: WebResource,
        subresources: NSArray,
        subframeArchives: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMainResource : mainResource, subresources : subresources, subframeArchives : subframeArchives)
    }
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn mainResource(&self) -> WebResource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainResource)
    }
    unsafe fn subresources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subresources)
    }
    unsafe fn subframeArchives(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subframeArchives)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebBackForwardList(pub id);
impl std::ops::Deref for WebBackForwardList {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebBackForwardList {}
impl WebBackForwardList {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebBackForwardList").unwrap(), alloc) })
    }
}
impl INSObject for WebBackForwardList {}
impl PNSObject for WebBackForwardList {}
impl std::convert::TryFrom<NSObject> for WebBackForwardList {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebBackForwardList, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebBackForwardList").unwrap()) };
        if is_kind_of {
            Ok(WebBackForwardList(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebBackForwardList")
        }
    }
}
impl IWebBackForwardList for WebBackForwardList {}
pub trait IWebBackForwardList: Sized + std::ops::Deref {
    unsafe fn addItem_(&self, item: WebHistoryItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addItem : item)
    }
    unsafe fn goBack(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, goBack)
    }
    unsafe fn goForward(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, goForward)
    }
    unsafe fn goToItem_(&self, item: WebHistoryItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToItem : item)
    }
    unsafe fn backListWithLimit_(&self, limit: ::std::os::raw::c_int) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, backListWithLimit : limit)
    }
    unsafe fn forwardListWithLimit_(&self, limit: ::std::os::raw::c_int) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, forwardListWithLimit : limit)
    }
    unsafe fn containsItem_(&self, item: WebHistoryItem) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsItem : item)
    }
    unsafe fn itemAtIndex_(&self, index: ::std::os::raw::c_int) -> WebHistoryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemAtIndex : index)
    }
    unsafe fn backItem(&self) -> WebHistoryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backItem)
    }
    unsafe fn currentItem(&self) -> WebHistoryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentItem)
    }
    unsafe fn forwardItem(&self) -> WebHistoryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forwardItem)
    }
    unsafe fn capacity(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capacity)
    }
    unsafe fn setCapacity_(&self, capacity: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCapacity : capacity)
    }
    unsafe fn backListCount(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backListCount)
    }
    unsafe fn forwardListCount(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forwardListCount)
    }
}
impl WebBackForwardList_WebBackForwardListDeprecated for WebBackForwardList {}
pub trait WebBackForwardList_WebBackForwardListDeprecated: Sized + std::ops::Deref {
    unsafe fn setPageCacheSize_(&self, size: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPageCacheSize : size)
    }
    unsafe fn pageCacheSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageCacheSize)
    }
}
impl DOMNode_WebDOMNodeOperations for DOMNode {}
pub trait DOMNode_WebDOMNodeOperations: Sized + std::ops::Deref {
    unsafe fn webArchive(&self) -> WebArchive
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webArchive)
    }
}
impl DOMDocument_WebDOMDocumentOperations for DOMDocument {}
pub trait DOMDocument_WebDOMDocumentOperations: Sized + std::ops::Deref {
    unsafe fn URLWithAttributeString_(&self, string: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLWithAttributeString : string)
    }
    unsafe fn webFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webFrame)
    }
}
impl DOMRange_WebDOMRangeOperations for DOMRange {}
pub trait DOMRange_WebDOMRangeOperations: Sized + std::ops::Deref {
    unsafe fn webArchive(&self) -> WebArchive
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webArchive)
    }
    unsafe fn markupString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, markupString)
    }
}
impl DOMHTMLFrameElement_WebDOMHTMLFrameElementOperations for DOMHTMLFrameElement {}
pub trait DOMHTMLFrameElement_WebDOMHTMLFrameElementOperations: Sized + std::ops::Deref {
    unsafe fn contentFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentFrame)
    }
}
impl DOMHTMLIFrameElement_WebDOMHTMLIFrameElementOperations for DOMHTMLIFrameElement {}
pub trait DOMHTMLIFrameElement_WebDOMHTMLIFrameElementOperations: Sized + std::ops::Deref {
    unsafe fn contentFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentFrame)
    }
}
impl DOMHTMLObjectElement_WebDOMHTMLObjectElementOperations for DOMHTMLObjectElement {}
pub trait DOMHTMLObjectElement_WebDOMHTMLObjectElementOperations: Sized + std::ops::Deref {
    unsafe fn contentFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentFrame)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebDataSource(pub id);
impl std::ops::Deref for WebDataSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebDataSource {}
impl WebDataSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebDataSource").unwrap(), alloc) })
    }
}
impl INSObject for WebDataSource {}
impl PNSObject for WebDataSource {}
impl std::convert::TryFrom<NSObject> for WebDataSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebDataSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebDataSource").unwrap()) };
        if is_kind_of {
            Ok(WebDataSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebDataSource")
        }
    }
}
impl IWebDataSource for WebDataSource {}
pub trait IWebDataSource: Sized + std::ops::Deref {
    unsafe fn initWithRequest_(&self, request: NSURLRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRequest : request)
    }
    unsafe fn subresourceForURL_(&self, URL: NSURL) -> WebResource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, subresourceForURL : URL)
    }
    unsafe fn addSubresource_(&self, subresource: WebResource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSubresource : subresource)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn representation(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, representation)
    }
    unsafe fn webFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webFrame)
    }
    unsafe fn initialRequest(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialRequest)
    }
    unsafe fn request(&self) -> NSMutableURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, request)
    }
    unsafe fn response(&self) -> NSURLResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, response)
    }
    unsafe fn textEncodingName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textEncodingName)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn pageTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageTitle)
    }
    unsafe fn unreachableURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unreachableURL)
    }
    unsafe fn webArchive(&self) -> WebArchive
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webArchive)
    }
    unsafe fn mainResource(&self) -> WebResource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainResource)
    }
    unsafe fn subresources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subresources)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebDownload(pub id);
impl std::ops::Deref for WebDownload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebDownload {}
impl WebDownload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebDownload").unwrap(), alloc) })
    }
}
impl INSURLDownload for WebDownload {}
impl std::convert::TryFrom<NSURLDownload> for WebDownload {
    type Error = &'static str;
    fn try_from(parent: NSURLDownload) -> Result<WebDownload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebDownload").unwrap()) };
        if is_kind_of {
            Ok(WebDownload(parent.0))
        } else {
            Err("This NSURLDownload cannot be downcasted to WebDownload")
        }
    }
}
impl INSObject for WebDownload {}
impl PNSObject for WebDownload {}
impl IWebDownload for WebDownload {}
pub trait IWebDownload: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebFrame(pub id);
impl std::ops::Deref for WebFrame {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebFrame {}
impl WebFrame {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebFrame").unwrap(), alloc) })
    }
}
impl INSObject for WebFrame {}
impl PNSObject for WebFrame {}
impl std::convert::TryFrom<NSObject> for WebFrame {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebFrame, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebFrame").unwrap()) };
        if is_kind_of {
            Ok(WebFrame(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebFrame")
        }
    }
}
impl IWebFrame for WebFrame {}
pub trait IWebFrame: Sized + std::ops::Deref {
    unsafe fn initWithName_webFrameView_webView_(
        &self,
        name: NSString,
        view: WebFrameView,
        webView: WebView,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, webFrameView : view, webView : webView)
    }
    unsafe fn loadRequest_(&self, request: NSURLRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadRequest : request)
    }
    unsafe fn loadData_MIMEType_textEncodingName_baseURL_(
        &self,
        data: NSData,
        MIMEType: NSString,
        encodingName: NSString,
        URL: NSURL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadData : data, MIMEType : MIMEType, textEncodingName : encodingName, baseURL : URL)
    }
    unsafe fn loadHTMLString_baseURL_(&self, string: NSString, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadHTMLString : string, baseURL : URL)
    }
    unsafe fn loadAlternateHTMLString_baseURL_forUnreachableURL_(
        &self,
        string: NSString,
        baseURL: NSURL,
        unreachableURL: NSURL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadAlternateHTMLString : string, baseURL : baseURL, forUnreachableURL : unreachableURL)
    }
    unsafe fn loadArchive_(&self, archive: WebArchive)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadArchive : archive)
    }
    unsafe fn stopLoading(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopLoading)
    }
    unsafe fn reload(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reload)
    }
    unsafe fn reloadFromOrigin(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadFromOrigin)
    }
    unsafe fn findFrameNamed_(&self, name: NSString) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findFrameNamed : name)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn webView(&self) -> WebView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webView)
    }
    unsafe fn frameView(&self) -> WebFrameView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameView)
    }
    unsafe fn DOMDocument(&self) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, DOMDocument)
    }
    unsafe fn frameElement(&self) -> DOMHTMLElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameElement)
    }
    unsafe fn dataSource(&self) -> WebDataSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSource)
    }
    unsafe fn provisionalDataSource(&self) -> WebDataSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, provisionalDataSource)
    }
    unsafe fn parentFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentFrame)
    }
    unsafe fn childFrames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childFrames)
    }
    unsafe fn windowObject(&self) -> WebScriptObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowObject)
    }
    unsafe fn globalContext(&self) -> JSGlobalContextRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, globalContext)
    }
    unsafe fn javaScriptContext(&self) -> JSContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, javaScriptContext)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebFrameView(pub id);
impl std::ops::Deref for WebFrameView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebFrameView {}
impl WebFrameView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebFrameView").unwrap(), alloc) })
    }
}
impl INSView for WebFrameView {}
impl PNSAnimatablePropertyContainer for WebFrameView {}
impl PNSUserInterfaceItemIdentification for WebFrameView {}
impl PNSDraggingDestination for WebFrameView {}
impl PNSAppearanceCustomization for WebFrameView {}
impl PNSAccessibilityElement for WebFrameView {}
impl PNSAccessibility for WebFrameView {}
impl std::convert::TryFrom<NSView> for WebFrameView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<WebFrameView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebFrameView").unwrap()) };
        if is_kind_of {
            Ok(WebFrameView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to WebFrameView")
        }
    }
}
impl INSResponder for WebFrameView {}
impl PNSCoding for WebFrameView {}
impl INSObject for WebFrameView {}
impl PNSObject for WebFrameView {}
impl IWebFrameView for WebFrameView {}
pub trait IWebFrameView: Sized + std::ops::Deref {
    unsafe fn printOperationWithPrintInfo_(&self, printInfo: NSPrintInfo) -> NSPrintOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, printOperationWithPrintInfo : printInfo)
    }
    unsafe fn printDocumentView(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, printDocumentView)
    }
    unsafe fn webFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webFrame)
    }
    unsafe fn documentView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentView)
    }
    unsafe fn allowsScrolling(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsScrolling)
    }
    unsafe fn setAllowsScrolling_(&self, allowsScrolling: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsScrolling : allowsScrolling)
    }
    unsafe fn canPrintHeadersAndFooters(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canPrintHeadersAndFooters)
    }
    unsafe fn documentViewShouldHandlePrint(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentViewShouldHandlePrint)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebHistory(pub id);
impl std::ops::Deref for WebHistory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebHistory {}
impl WebHistory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebHistory").unwrap(), alloc) })
    }
}
impl INSObject for WebHistory {}
impl PNSObject for WebHistory {}
impl std::convert::TryFrom<NSObject> for WebHistory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebHistory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebHistory").unwrap()) };
        if is_kind_of {
            Ok(WebHistory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebHistory")
        }
    }
}
impl IWebHistory for WebHistory {}
pub trait IWebHistory: Sized + std::ops::Deref {
    unsafe fn loadFromURL_error_(&self, URL: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromURL : URL, error : error)
    }
    unsafe fn saveToURL_error_(&self, URL: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToURL : URL, error : error)
    }
    unsafe fn addItems_(&self, newItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addItems : newItems)
    }
    unsafe fn removeItems_(&self, items: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeItems : items)
    }
    unsafe fn removeAllItems(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllItems)
    }
    unsafe fn orderedItemsLastVisitedOnDay_(&self, calendarDate: NSCalendarDate) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, orderedItemsLastVisitedOnDay : calendarDate)
    }
    unsafe fn itemForURL_(&self, URL: NSURL) -> WebHistoryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemForURL : URL)
    }
    unsafe fn orderedLastVisitedDays(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orderedLastVisitedDays)
    }
    unsafe fn historyItemLimit(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, historyItemLimit)
    }
    unsafe fn setHistoryItemLimit_(&self, historyItemLimit: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHistoryItemLimit : historyItemLimit)
    }
    unsafe fn historyAgeInDaysLimit(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, historyAgeInDaysLimit)
    }
    unsafe fn setHistoryAgeInDaysLimit_(&self, historyAgeInDaysLimit: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHistoryAgeInDaysLimit : historyAgeInDaysLimit)
    }
    unsafe fn optionalSharedHistory() -> WebHistory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebHistory").unwrap(), optionalSharedHistory)
    }
    unsafe fn setOptionalSharedHistory_(history: WebHistory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebHistory").unwrap(), setOptionalSharedHistory : history)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebHistoryItem(pub id);
impl std::ops::Deref for WebHistoryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebHistoryItem {}
impl WebHistoryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebHistoryItem").unwrap(), alloc) })
    }
}
impl PNSCopying for WebHistoryItem {}
impl INSObject for WebHistoryItem {}
impl PNSObject for WebHistoryItem {}
impl std::convert::TryFrom<NSObject> for WebHistoryItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebHistoryItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebHistoryItem").unwrap()) };
        if is_kind_of {
            Ok(WebHistoryItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebHistoryItem")
        }
    }
}
impl IWebHistoryItem for WebHistoryItem {}
pub trait IWebHistoryItem: Sized + std::ops::Deref {
    unsafe fn initWithURLString_title_lastVisitedTimeInterval_(
        &self,
        URLString: NSString,
        title: NSString,
        time: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURLString : URLString, title : title, lastVisitedTimeInterval : time)
    }
    unsafe fn originalURLString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalURLString)
    }
    unsafe fn URLString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLString)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn lastVisitedTimeInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastVisitedTimeInterval)
    }
    unsafe fn alternateTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternateTitle)
    }
    unsafe fn setAlternateTitle_(&self, alternateTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlternateTitle : alternateTitle)
    }
    unsafe fn icon(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, icon)
    }
}
pub trait NSObject_WebPlugIn: Sized + std::ops::Deref {
    unsafe fn webPlugInInitialize(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webPlugInInitialize)
    }
    unsafe fn webPlugInStart(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webPlugInStart)
    }
    unsafe fn webPlugInStop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webPlugInStop)
    }
    unsafe fn webPlugInDestroy(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webPlugInDestroy)
    }
    unsafe fn webPlugInSetIsSelected_(&self, isSelected: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webPlugInSetIsSelected : isSelected)
    }
    unsafe fn webPlugInMainResourceDidReceiveResponse_(&self, response: NSURLResponse)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webPlugInMainResourceDidReceiveResponse : response)
    }
    unsafe fn webPlugInMainResourceDidReceiveData_(&self, data: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webPlugInMainResourceDidReceiveData : data)
    }
    unsafe fn webPlugInMainResourceDidFailWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webPlugInMainResourceDidFailWithError : error)
    }
    unsafe fn webPlugInMainResourceDidFinishLoading(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webPlugInMainResourceDidFinishLoading)
    }
    unsafe fn objectForWebScript(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectForWebScript)
    }
}
pub trait NSObject_WebPlugInContainer: Sized + std::ops::Deref {
    unsafe fn webPlugInContainerLoadRequest_inFrame_(&self, request: NSURLRequest, target: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webPlugInContainerLoadRequest : request, inFrame : target)
    }
    unsafe fn webPlugInContainerShowStatus_(&self, message: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, webPlugInContainerShowStatus : message)
    }
    unsafe fn webPlugInContainerSelectionColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webPlugInContainerSelectionColor)
    }
    unsafe fn webFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webFrame)
    }
}
pub type WebCacheModel = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebPreferences(pub id);
impl std::ops::Deref for WebPreferences {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebPreferences {}
impl WebPreferences {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebPreferences").unwrap(), alloc) })
    }
}
impl PNSCoding for WebPreferences {}
impl INSObject for WebPreferences {}
impl PNSObject for WebPreferences {}
impl std::convert::TryFrom<NSObject> for WebPreferences {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebPreferences, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebPreferences").unwrap()) };
        if is_kind_of {
            Ok(WebPreferences(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebPreferences")
        }
    }
}
impl IWebPreferences for WebPreferences {}
pub trait IWebPreferences: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_(&self, anIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : anIdentifier)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn standardFontFamily(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, standardFontFamily)
    }
    unsafe fn setStandardFontFamily_(&self, standardFontFamily: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStandardFontFamily : standardFontFamily)
    }
    unsafe fn fixedFontFamily(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fixedFontFamily)
    }
    unsafe fn setFixedFontFamily_(&self, fixedFontFamily: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFixedFontFamily : fixedFontFamily)
    }
    unsafe fn serifFontFamily(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serifFontFamily)
    }
    unsafe fn setSerifFontFamily_(&self, serifFontFamily: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSerifFontFamily : serifFontFamily)
    }
    unsafe fn sansSerifFontFamily(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sansSerifFontFamily)
    }
    unsafe fn setSansSerifFontFamily_(&self, sansSerifFontFamily: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSansSerifFontFamily : sansSerifFontFamily)
    }
    unsafe fn cursiveFontFamily(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursiveFontFamily)
    }
    unsafe fn setCursiveFontFamily_(&self, cursiveFontFamily: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCursiveFontFamily : cursiveFontFamily)
    }
    unsafe fn fantasyFontFamily(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fantasyFontFamily)
    }
    unsafe fn setFantasyFontFamily_(&self, fantasyFontFamily: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFantasyFontFamily : fantasyFontFamily)
    }
    unsafe fn defaultFontSize(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultFontSize)
    }
    unsafe fn setDefaultFontSize_(&self, defaultFontSize: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultFontSize : defaultFontSize)
    }
    unsafe fn defaultFixedFontSize(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultFixedFontSize)
    }
    unsafe fn setDefaultFixedFontSize_(&self, defaultFixedFontSize: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultFixedFontSize : defaultFixedFontSize)
    }
    unsafe fn minimumFontSize(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumFontSize)
    }
    unsafe fn setMinimumFontSize_(&self, minimumFontSize: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumFontSize : minimumFontSize)
    }
    unsafe fn minimumLogicalFontSize(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumLogicalFontSize)
    }
    unsafe fn setMinimumLogicalFontSize_(&self, minimumLogicalFontSize: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumLogicalFontSize : minimumLogicalFontSize)
    }
    unsafe fn defaultTextEncodingName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultTextEncodingName)
    }
    unsafe fn setDefaultTextEncodingName_(&self, defaultTextEncodingName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultTextEncodingName : defaultTextEncodingName)
    }
    unsafe fn userStyleSheetEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userStyleSheetEnabled)
    }
    unsafe fn setUserStyleSheetEnabled_(&self, userStyleSheetEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserStyleSheetEnabled : userStyleSheetEnabled)
    }
    unsafe fn userStyleSheetLocation(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userStyleSheetLocation)
    }
    unsafe fn setUserStyleSheetLocation_(&self, userStyleSheetLocation: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserStyleSheetLocation : userStyleSheetLocation)
    }
    unsafe fn isJavaEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isJavaEnabled)
    }
    unsafe fn setJavaEnabled_(&self, javaEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJavaEnabled : javaEnabled)
    }
    unsafe fn isJavaScriptEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isJavaScriptEnabled)
    }
    unsafe fn setJavaScriptEnabled_(&self, javaScriptEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJavaScriptEnabled : javaScriptEnabled)
    }
    unsafe fn javaScriptCanOpenWindowsAutomatically(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, javaScriptCanOpenWindowsAutomatically)
    }
    unsafe fn setJavaScriptCanOpenWindowsAutomatically_(
        &self,
        javaScriptCanOpenWindowsAutomatically: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJavaScriptCanOpenWindowsAutomatically : javaScriptCanOpenWindowsAutomatically)
    }
    unsafe fn arePlugInsEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arePlugInsEnabled)
    }
    unsafe fn setPlugInsEnabled_(&self, plugInsEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlugInsEnabled : plugInsEnabled)
    }
    unsafe fn allowsAnimatedImages(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAnimatedImages)
    }
    unsafe fn setAllowsAnimatedImages_(&self, allowsAnimatedImages: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAnimatedImages : allowsAnimatedImages)
    }
    unsafe fn allowsAnimatedImageLooping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAnimatedImageLooping)
    }
    unsafe fn setAllowsAnimatedImageLooping_(&self, allowsAnimatedImageLooping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAnimatedImageLooping : allowsAnimatedImageLooping)
    }
    unsafe fn loadsImagesAutomatically(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loadsImagesAutomatically)
    }
    unsafe fn setLoadsImagesAutomatically_(&self, loadsImagesAutomatically: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoadsImagesAutomatically : loadsImagesAutomatically)
    }
    unsafe fn autosaves(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autosaves)
    }
    unsafe fn setAutosaves_(&self, autosaves: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutosaves : autosaves)
    }
    unsafe fn shouldPrintBackgrounds(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldPrintBackgrounds)
    }
    unsafe fn setShouldPrintBackgrounds_(&self, shouldPrintBackgrounds: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldPrintBackgrounds : shouldPrintBackgrounds)
    }
    unsafe fn privateBrowsingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, privateBrowsingEnabled)
    }
    unsafe fn setPrivateBrowsingEnabled_(&self, privateBrowsingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrivateBrowsingEnabled : privateBrowsingEnabled)
    }
    unsafe fn tabsToLinks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabsToLinks)
    }
    unsafe fn setTabsToLinks_(&self, tabsToLinks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTabsToLinks : tabsToLinks)
    }
    unsafe fn usesPageCache(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesPageCache)
    }
    unsafe fn setUsesPageCache_(&self, usesPageCache: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesPageCache : usesPageCache)
    }
    unsafe fn cacheModel(&self) -> WebCacheModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cacheModel)
    }
    unsafe fn setCacheModel_(&self, cacheModel: WebCacheModel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCacheModel : cacheModel)
    }
    unsafe fn suppressesIncrementalRendering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suppressesIncrementalRendering)
    }
    unsafe fn setSuppressesIncrementalRendering_(&self, suppressesIncrementalRendering: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuppressesIncrementalRendering : suppressesIncrementalRendering)
    }
    unsafe fn allowsAirPlayForMediaPlayback(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAirPlayForMediaPlayback)
    }
    unsafe fn setAllowsAirPlayForMediaPlayback_(&self, allowsAirPlayForMediaPlayback: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAirPlayForMediaPlayback : allowsAirPlayForMediaPlayback)
    }
    unsafe fn standardPreferences() -> WebPreferences
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebPreferences").unwrap(), standardPreferences)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebResource(pub id);
impl std::ops::Deref for WebResource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebResource {}
impl WebResource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebResource").unwrap(), alloc) })
    }
}
impl PNSCoding for WebResource {}
impl PNSCopying for WebResource {}
impl INSObject for WebResource {}
impl PNSObject for WebResource {}
impl std::convert::TryFrom<NSObject> for WebResource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<WebResource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebResource").unwrap()) };
        if is_kind_of {
            Ok(WebResource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to WebResource")
        }
    }
}
impl IWebResource for WebResource {}
pub trait IWebResource: Sized + std::ops::Deref {
    unsafe fn initWithData_URL_MIMEType_textEncodingName_frameName_(
        &self,
        data: NSData,
        URL: NSURL,
        MIMEType: NSString,
        textEncodingName: NSString,
        frameName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, URL : URL, MIMEType : MIMEType, textEncodingName : textEncodingName, frameName : frameName)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn MIMEType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIMEType)
    }
    unsafe fn textEncodingName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textEncodingName)
    }
    unsafe fn frameName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct WebView(pub id);
impl std::ops::Deref for WebView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for WebView {}
impl WebView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), alloc) })
    }
}
impl INSView for WebView {}
impl PNSAnimatablePropertyContainer for WebView {}
impl PNSUserInterfaceItemIdentification for WebView {}
impl PNSDraggingDestination for WebView {}
impl PNSAppearanceCustomization for WebView {}
impl PNSAccessibilityElement for WebView {}
impl PNSAccessibility for WebView {}
impl std::convert::TryFrom<NSView> for WebView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<WebView, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"WebView").unwrap()) };
        if is_kind_of {
            Ok(WebView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to WebView")
        }
    }
}
impl INSResponder for WebView {}
impl PNSCoding for WebView {}
impl INSObject for WebView {}
impl PNSObject for WebView {}
impl IWebView for WebView {}
pub trait IWebView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_frameName_groupName_(
        &self,
        frame: NSRect,
        frameName: NSString,
        groupName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame, frameName : frameName, groupName : groupName)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn setMaintainsBackForwardList_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaintainsBackForwardList : flag)
    }
    unsafe fn goBack(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, goBack)
    }
    unsafe fn goForward(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, goForward)
    }
    unsafe fn goToBackForwardItem_(&self, item: WebHistoryItem) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goToBackForwardItem : item)
    }
    unsafe fn userAgentForURL_(&self, URL: NSURL) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userAgentForURL : URL)
    }
    unsafe fn stringByEvaluatingJavaScriptFromString_(&self, script: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringByEvaluatingJavaScriptFromString : script)
    }
    unsafe fn searchFor_direction_caseSensitive_wrap_(
        &self,
        string: NSString,
        forward: BOOL,
        caseFlag: BOOL,
        wrapFlag: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchFor : string, direction : forward, caseSensitive : caseFlag, wrap : wrapFlag)
    }
    unsafe fn elementAtPoint_(&self, point: NSPoint) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementAtPoint : point)
    }
    unsafe fn writeSelectionWithPasteboardTypes_toPasteboard_(
        &self,
        types: NSArray,
        pasteboard: NSPasteboard,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeSelectionWithPasteboardTypes : types, toPasteboard : pasteboard)
    }
    unsafe fn pasteboardTypesForElement_(&self, element: NSDictionary) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pasteboardTypesForElement : element)
    }
    unsafe fn writeElement_withPasteboardTypes_toPasteboard_(
        &self,
        element: NSDictionary,
        types: NSArray,
        pasteboard: NSPasteboard,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeElement : element, withPasteboardTypes : types, toPasteboard : pasteboard)
    }
    unsafe fn moveDragCaretToPoint_(&self, point: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveDragCaretToPoint : point)
    }
    unsafe fn removeDragCaret(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeDragCaret)
    }
    unsafe fn shouldCloseWithWindow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCloseWithWindow)
    }
    unsafe fn setShouldCloseWithWindow_(&self, shouldCloseWithWindow: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldCloseWithWindow : shouldCloseWithWindow)
    }
    unsafe fn UIDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UIDelegate)
    }
    unsafe fn setUIDelegate_(&self, UIDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUIDelegate : UIDelegate)
    }
    unsafe fn resourceLoadDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceLoadDelegate)
    }
    unsafe fn setResourceLoadDelegate_(&self, resourceLoadDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResourceLoadDelegate : resourceLoadDelegate)
    }
    unsafe fn downloadDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadDelegate)
    }
    unsafe fn setDownloadDelegate_(&self, downloadDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownloadDelegate : downloadDelegate)
    }
    unsafe fn frameLoadDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameLoadDelegate)
    }
    unsafe fn setFrameLoadDelegate_(&self, frameLoadDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameLoadDelegate : frameLoadDelegate)
    }
    unsafe fn policyDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, policyDelegate)
    }
    unsafe fn setPolicyDelegate_(&self, policyDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPolicyDelegate : policyDelegate)
    }
    unsafe fn mainFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainFrame)
    }
    unsafe fn selectedFrame(&self) -> WebFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedFrame)
    }
    unsafe fn backForwardList(&self) -> WebBackForwardList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backForwardList)
    }
    unsafe fn textSizeMultiplier(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textSizeMultiplier)
    }
    unsafe fn setTextSizeMultiplier_(&self, textSizeMultiplier: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextSizeMultiplier : textSizeMultiplier)
    }
    unsafe fn applicationNameForUserAgent(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationNameForUserAgent)
    }
    unsafe fn setApplicationNameForUserAgent_(&self, applicationNameForUserAgent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApplicationNameForUserAgent : applicationNameForUserAgent)
    }
    unsafe fn customUserAgent(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customUserAgent)
    }
    unsafe fn setCustomUserAgent_(&self, customUserAgent: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomUserAgent : customUserAgent)
    }
    unsafe fn supportsTextEncoding(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsTextEncoding)
    }
    unsafe fn customTextEncodingName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customTextEncodingName)
    }
    unsafe fn setCustomTextEncodingName_(&self, customTextEncodingName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomTextEncodingName : customTextEncodingName)
    }
    unsafe fn mediaStyle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaStyle)
    }
    unsafe fn setMediaStyle_(&self, mediaStyle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaStyle : mediaStyle)
    }
    unsafe fn windowScriptObject(&self) -> WebScriptObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowScriptObject)
    }
    unsafe fn preferences(&self) -> WebPreferences
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferences)
    }
    unsafe fn setPreferences_(&self, preferences: WebPreferences)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferences : preferences)
    }
    unsafe fn preferencesIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferencesIdentifier)
    }
    unsafe fn setPreferencesIdentifier_(&self, preferencesIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferencesIdentifier : preferencesIdentifier)
    }
    unsafe fn hostWindow(&self) -> NSWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostWindow)
    }
    unsafe fn setHostWindow_(&self, hostWindow: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHostWindow : hostWindow)
    }
    unsafe fn groupName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupName)
    }
    unsafe fn setGroupName_(&self, groupName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroupName : groupName)
    }
    unsafe fn estimatedProgress(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, estimatedProgress)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn pasteboardTypesForSelection(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pasteboardTypesForSelection)
    }
    unsafe fn drawsBackground(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawsBackground)
    }
    unsafe fn setDrawsBackground_(&self, drawsBackground: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsBackground : drawsBackground)
    }
    unsafe fn shouldUpdateWhileOffscreen(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldUpdateWhileOffscreen)
    }
    unsafe fn setShouldUpdateWhileOffscreen_(&self, shouldUpdateWhileOffscreen: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldUpdateWhileOffscreen : shouldUpdateWhileOffscreen)
    }
    unsafe fn mainFrameURL(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainFrameURL)
    }
    unsafe fn setMainFrameURL_(&self, mainFrameURL: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMainFrameURL : mainFrameURL)
    }
    unsafe fn mainFrameDocument(&self) -> DOMDocument
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainFrameDocument)
    }
    unsafe fn mainFrameTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainFrameTitle)
    }
    unsafe fn mainFrameIcon(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainFrameIcon)
    }
    unsafe fn canShowMIMEType_(MIMEType: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), canShowMIMEType : MIMEType)
    }
    unsafe fn canShowMIMETypeAsHTML_(MIMEType: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), canShowMIMETypeAsHTML : MIMEType)
    }
    unsafe fn MIMETypesShownAsHTML() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), MIMETypesShownAsHTML)
    }
    unsafe fn setMIMETypesShownAsHTML_(MIMETypes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), setMIMETypesShownAsHTML : MIMETypes)
    }
    unsafe fn URLFromPasteboard_(pasteboard: NSPasteboard) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), URLFromPasteboard : pasteboard)
    }
    unsafe fn URLTitleFromPasteboard_(pasteboard: NSPasteboard) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), URLTitleFromPasteboard : pasteboard)
    }
    unsafe fn registerURLSchemeAsLocal_(scheme: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), registerURLSchemeAsLocal : scheme)
    }
    unsafe fn registerViewClass_representationClass_forMIMEType_(
        viewClass: Class,
        representationClass: Class,
        MIMEType: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"WebView").unwrap(), registerViewClass : viewClass, representationClass : representationClass, forMIMEType : MIMEType)
    }
}
impl WebView_WebIBActions for WebView {}
pub trait WebView_WebIBActions: Sized + std::ops::Deref {
    unsafe fn takeStringURLFrom_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, takeStringURLFrom : sender)
    }
    unsafe fn stopLoading_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopLoading : sender)
    }
    unsafe fn reload_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reload : sender)
    }
    unsafe fn reloadFromOrigin_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reloadFromOrigin : sender)
    }
    unsafe fn goBack_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goBack : sender)
    }
    unsafe fn goForward_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, goForward : sender)
    }
    unsafe fn makeTextLarger_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeTextLarger : sender)
    }
    unsafe fn makeTextSmaller_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeTextSmaller : sender)
    }
    unsafe fn makeTextStandardSize_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeTextStandardSize : sender)
    }
    unsafe fn toggleContinuousSpellChecking_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleContinuousSpellChecking : sender)
    }
    unsafe fn toggleSmartInsertDelete_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleSmartInsertDelete : sender)
    }
    unsafe fn canGoBack(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoBack)
    }
    unsafe fn canGoForward(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canGoForward)
    }
    unsafe fn canMakeTextLarger(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canMakeTextLarger)
    }
    unsafe fn canMakeTextSmaller(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canMakeTextSmaller)
    }
    unsafe fn canMakeTextStandardSize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canMakeTextStandardSize)
    }
}
impl WebView_WebViewCSS for WebView {}
pub trait WebView_WebViewCSS: Sized + std::ops::Deref {
    unsafe fn computedStyleForElement_pseudoElement_(
        &self,
        element: DOMElement,
        pseudoElement: NSString,
    ) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, computedStyleForElement : element, pseudoElement : pseudoElement)
    }
}
impl WebView_WebViewEditing for WebView {}
pub trait WebView_WebViewEditing: Sized + std::ops::Deref {
    unsafe fn editableDOMRangeForPoint_(&self, point: NSPoint) -> DOMRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, editableDOMRangeForPoint : point)
    }
    unsafe fn setSelectedDOMRange_affinity_(
        &self,
        range: DOMRange,
        selectionAffinity: NSSelectionAffinity,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedDOMRange : range, affinity : selectionAffinity)
    }
    unsafe fn styleDeclarationWithText_(&self, text: NSString) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, styleDeclarationWithText : text)
    }
    unsafe fn selectedDOMRange(&self) -> DOMRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedDOMRange)
    }
    unsafe fn selectionAffinity(&self) -> NSSelectionAffinity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionAffinity)
    }
    unsafe fn maintainsInactiveSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maintainsInactiveSelection)
    }
    unsafe fn isEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEditable)
    }
    unsafe fn setEditable_(&self, editable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditable : editable)
    }
    unsafe fn typingStyle(&self) -> DOMCSSStyleDeclaration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typingStyle)
    }
    unsafe fn setTypingStyle_(&self, typingStyle: DOMCSSStyleDeclaration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTypingStyle : typingStyle)
    }
    unsafe fn smartInsertDeleteEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smartInsertDeleteEnabled)
    }
    unsafe fn setSmartInsertDeleteEnabled_(&self, smartInsertDeleteEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmartInsertDeleteEnabled : smartInsertDeleteEnabled)
    }
    unsafe fn isContinuousSpellCheckingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContinuousSpellCheckingEnabled)
    }
    unsafe fn setContinuousSpellCheckingEnabled_(&self, continuousSpellCheckingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContinuousSpellCheckingEnabled : continuousSpellCheckingEnabled)
    }
    unsafe fn spellCheckerDocumentTag(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spellCheckerDocumentTag)
    }
    unsafe fn undoManager(&self) -> NSUndoManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, undoManager)
    }
    unsafe fn editingDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, editingDelegate)
    }
    unsafe fn setEditingDelegate_(&self, editingDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditingDelegate : editingDelegate)
    }
}
impl WebView_WebViewUndoableEditing for WebView {}
pub trait WebView_WebViewUndoableEditing: Sized + std::ops::Deref {
    unsafe fn replaceSelectionWithNode_(&self, node: DOMNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceSelectionWithNode : node)
    }
    unsafe fn replaceSelectionWithText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceSelectionWithText : text)
    }
    unsafe fn replaceSelectionWithMarkupString_(&self, markupString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceSelectionWithMarkupString : markupString)
    }
    unsafe fn replaceSelectionWithArchive_(&self, archive: WebArchive)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceSelectionWithArchive : archive)
    }
    unsafe fn deleteSelection(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteSelection)
    }
    unsafe fn applyStyle_(&self, style: DOMCSSStyleDeclaration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyStyle : style)
    }
}
impl WebView_WebViewEditingActions for WebView {}
pub trait WebView_WebViewEditingActions: Sized + std::ops::Deref {
    unsafe fn copy_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copy : sender)
    }
    unsafe fn cut_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cut : sender)
    }
    unsafe fn paste_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paste : sender)
    }
    unsafe fn copyFont_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFont : sender)
    }
    unsafe fn pasteFont_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pasteFont : sender)
    }
    unsafe fn delete_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, delete : sender)
    }
    unsafe fn pasteAsPlainText_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pasteAsPlainText : sender)
    }
    unsafe fn pasteAsRichText_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pasteAsRichText : sender)
    }
    unsafe fn changeFont_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeFont : sender)
    }
    unsafe fn changeAttributes_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeAttributes : sender)
    }
    unsafe fn changeDocumentBackgroundColor_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeDocumentBackgroundColor : sender)
    }
    unsafe fn changeColor_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeColor : sender)
    }
    unsafe fn alignCenter_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, alignCenter : sender)
    }
    unsafe fn alignJustified_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, alignJustified : sender)
    }
    unsafe fn alignLeft_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, alignLeft : sender)
    }
    unsafe fn alignRight_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, alignRight : sender)
    }
    unsafe fn checkSpelling_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkSpelling : sender)
    }
    unsafe fn showGuessPanel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showGuessPanel : sender)
    }
    unsafe fn performFindPanelAction_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performFindPanelAction : sender)
    }
    unsafe fn startSpeaking_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startSpeaking : sender)
    }
    unsafe fn stopSpeaking_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopSpeaking : sender)
    }
    unsafe fn moveToBeginningOfSentence_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveToBeginningOfSentence : sender)
    }
    unsafe fn moveToBeginningOfSentenceAndModifySelection_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveToBeginningOfSentenceAndModifySelection : sender)
    }
    unsafe fn moveToEndOfSentence_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveToEndOfSentence : sender)
    }
    unsafe fn moveToEndOfSentenceAndModifySelection_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveToEndOfSentenceAndModifySelection : sender)
    }
    unsafe fn selectSentence_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectSentence : sender)
    }
    unsafe fn overWrite_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, overWrite : sender)
    }
}
unsafe extern "C" {
    pub static NSReadAccessURLDocumentOption: NSAttributedStringDocumentReadingOptionKey;
}
unsafe extern "C" {
    pub static WKErrorDomain: NSString;
}
unsafe extern "C" {
    pub static WKWebExtensionMatchPatternErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionActiveTab: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionAlarms: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionClipboardWrite: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionContextMenus: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionCookies: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionDeclarativeNetRequest: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionDeclarativeNetRequestFeedback: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionDeclarativeNetRequestWithHostAccess:
        WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionMenus: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionNativeMessaging: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionScripting: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionStorage: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionTabs: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionUnlimitedStorage: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionWebNavigation: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionPermissionWebRequest: WKWebExtensionPermission;
}
unsafe extern "C" {
    pub static WKWebExtensionErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static WKWebExtensionContextErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static WKWebExtensionContextErrorsDidUpdateNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextPermissionsWereGrantedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextPermissionsWereDeniedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextGrantedPermissionsWereRemovedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextDeniedPermissionsWereRemovedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextPermissionMatchPatternsWereGrantedNotification:
        NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextPermissionMatchPatternsWereDeniedNotification:
        NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextGrantedPermissionMatchPatternsWereRemovedNotification:
        NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextDeniedPermissionMatchPatternsWereRemovedNotification:
        NSNotificationName;
}
unsafe extern "C" {
    pub static WKWebExtensionContextNotificationUserInfoKeyPermissions:
        WKWebExtensionContextNotificationUserInfoKey;
}
unsafe extern "C" {
    pub static WKWebExtensionContextNotificationUserInfoKeyMatchPatterns:
        WKWebExtensionContextNotificationUserInfoKey;
}
unsafe extern "C" {
    pub static WKWebExtensionDataTypeLocal: WKWebExtensionDataType;
}
unsafe extern "C" {
    pub static WKWebExtensionDataTypeSession: WKWebExtensionDataType;
}
unsafe extern "C" {
    pub static WKWebExtensionDataTypeSynchronized: WKWebExtensionDataType;
}
unsafe extern "C" {
    pub static WKWebExtensionDataRecordErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static WKWebExtensionMessagePortErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeFetchCache: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeDiskCache: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeMemoryCache: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeOfflineWebApplicationCache: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeCookies: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeSessionStorage: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeLocalStorage: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeWebSQLDatabases: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeIndexedDBDatabases: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeServiceWorkerRegistrations: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeFileSystem: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeSearchFieldRecentSearches: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeMediaKeys: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeHashSalt: NSString;
}
unsafe extern "C" {
    pub static WKWebsiteDataTypeScreenTime: NSString;
}

unsafe impl objc2::encode::RefEncode for WKBackForwardListItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKBackForwardListItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKBackForwardList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKBackForwardList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKContentRuleList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKContentRuleList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKContentRuleListStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKContentRuleListStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKContentWorld {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKContentWorld {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKDownload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKDownload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKFindConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKFindConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKFindResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKFindResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKSecurityOrigin {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKSecurityOrigin {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKFrameInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKFrameInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKHTTPCookieStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKHTTPCookieStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebpagePreferences {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebpagePreferences {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKNavigation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKNavigation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKNavigationAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKNavigationAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKNavigationResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKNavigationResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKOpenPanelParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKOpenPanelParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKPDFConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKPDFConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKPreferences {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKPreferences {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKProcessPool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKProcessPool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKScriptMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKScriptMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKSnapshotConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKSnapshotConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKPreviewElementInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKPreviewElementInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKUserContentController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKUserContentController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKUserScript {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKUserScript {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionMatchPattern {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionMatchPattern {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionControllerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionControllerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionDataRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionDataRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionMessagePort {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionMessagePort {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionTabConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionTabConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebExtensionWindowConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebExtensionWindowConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebViewConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebViewConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebsiteDataRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebsiteDataRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWebsiteDataStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWebsiteDataStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WKWindowFeatures {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WKWindowFeatures {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebScriptObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebScriptObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebUndefined {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebUndefined {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMAttr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMAttr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCharacterData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCharacterData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCDATASection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCDATASection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMComment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMComment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMDocument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMDocument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMDocumentFragment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMDocumentFragment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMDocumentType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMDocumentType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMEntity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMEntity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMEntityReference {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMEntityReference {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMImplementation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMImplementation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMNamedNodeMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMNamedNodeMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMNodeList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMNodeList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMProcessingInstruction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMProcessingInstruction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMStyleSheet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMStyleSheet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMStyleSheetList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMStyleSheetList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMMediaList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMMediaList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSCharsetRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSCharsetRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSFontFaceRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSFontFaceRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSImportRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSImportRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSMediaRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSMediaRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSPageRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSPageRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSPrimitiveValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSPrimitiveValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSRuleList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSRuleList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSStyleDeclaration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSStyleDeclaration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSStyleRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSStyleRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSStyleSheet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSStyleSheet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSUnknownRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSUnknownRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCSSValueList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCSSValueList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMCounter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMCounter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMRGBColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMRGBColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMRect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMRect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMBlob {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMBlob {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMFile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMFile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMFileList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMFileList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLAnchorElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLAnchorElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLAppletElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLAppletElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLAreaElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLAreaElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLBRElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLBRElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLBaseElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLBaseElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLBaseFontElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLBaseFontElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLBodyElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLBodyElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLButtonElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLButtonElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLDListElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLDListElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLDirectoryElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLDirectoryElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLDivElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLDivElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLDocument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLDocument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLEmbedElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLEmbedElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLFieldSetElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLFieldSetElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLFontElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLFontElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLFormElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLFormElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLFrameElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLFrameElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLFrameSetElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLFrameSetElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLHRElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLHRElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLHeadElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLHeadElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLHeadingElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLHeadingElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLHtmlElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLHtmlElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLIFrameElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLIFrameElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLImageElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLImageElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLInputElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLInputElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLLIElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLLIElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLLabelElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLLabelElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLLegendElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLLegendElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLLinkElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLLinkElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLMapElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLMapElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLMarqueeElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLMarqueeElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLMenuElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLMenuElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLMetaElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLMetaElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLModElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLModElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLOListElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLOListElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLObjectElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLObjectElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLOptGroupElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLOptGroupElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLOptionElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLOptionElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLOptionsCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLOptionsCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLParagraphElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLParagraphElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLParamElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLParamElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLPreElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLPreElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLQuoteElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLQuoteElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLScriptElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLScriptElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLSelectElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLSelectElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLStyleElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLStyleElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLTableCaptionElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLTableCaptionElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLTableCellElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLTableCellElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLTableColElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLTableColElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLTableElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLTableElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLTableRowElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLTableRowElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLTableSectionElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLTableSectionElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLTextAreaElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLTextAreaElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLTitleElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLTitleElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMHTMLUListElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMHTMLUListElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMAbstractView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMAbstractView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMUIEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMUIEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMKeyboardEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMKeyboardEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMMouseEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMMouseEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMMutationEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMMutationEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMOverflowEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMOverflowEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMProgressEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMProgressEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMWheelEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMWheelEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMNodeIterator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMNodeIterator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMTreeWalker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMTreeWalker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMXPathExpression {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMXPathExpression {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DOMXPathResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DOMXPathResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebArchive {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebArchive {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebBackForwardList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebBackForwardList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebDataSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebDataSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebDownload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebDownload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebFrame {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebFrame {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebFrameView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebFrameView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebHistory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebHistory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebHistoryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebHistoryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebPreferences {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebPreferences {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for WebView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for WebView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
