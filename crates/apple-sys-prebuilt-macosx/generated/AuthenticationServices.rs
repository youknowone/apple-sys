#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::LocalAuthentication::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ASPresentationAnchor = NSWindow;
pub type ASViewController = NSViewController;
pub type ASImage = NSImage;
pub type ASWebAuthenticationSessionErrorCode = NSInteger;
pub type ASWebAuthenticationSessionCompletionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASWebAuthenticationSession(pub id);
impl std::ops::Deref for ASWebAuthenticationSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASWebAuthenticationSession {}
impl ASWebAuthenticationSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSession").unwrap(), alloc) })
    }
}
impl INSObject for ASWebAuthenticationSession {}
impl PNSObject for ASWebAuthenticationSession {}
impl std::convert::TryFrom<NSObject> for ASWebAuthenticationSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASWebAuthenticationSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASWebAuthenticationSession").unwrap()) };
        if is_kind_of {
            Ok(ASWebAuthenticationSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASWebAuthenticationSession")
        }
    }
}
impl IASWebAuthenticationSession for ASWebAuthenticationSession {}
pub trait IASWebAuthenticationSession: Sized + std::ops::Deref {
    unsafe fn initWithURL_callbackURLScheme_completionHandler_(
        &self,
        URL: NSURL,
        callbackURLScheme: NSString,
        completionHandler: ASWebAuthenticationSessionCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL, callbackURLScheme : callbackURLScheme, completionHandler : completionHandler)
    }
    unsafe fn initWithURL_callback_completionHandler_(
        &self,
        URL: NSURL,
        callback: ASWebAuthenticationSessionCallback,
        completionHandler: ASWebAuthenticationSessionCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL, callback : callback, completionHandler : completionHandler)
    }
    unsafe fn start(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn presentationContextProvider(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationContextProvider)
    }
    unsafe fn setPresentationContextProvider_(&self, presentationContextProvider: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresentationContextProvider : presentationContextProvider)
    }
    unsafe fn prefersEphemeralWebBrowserSession(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prefersEphemeralWebBrowserSession)
    }
    unsafe fn setPrefersEphemeralWebBrowserSession_(&self, prefersEphemeralWebBrowserSession: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrefersEphemeralWebBrowserSession : prefersEphemeralWebBrowserSession)
    }
    unsafe fn additionalHeaderFields(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalHeaderFields)
    }
    unsafe fn setAdditionalHeaderFields_(&self, additionalHeaderFields: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalHeaderFields : additionalHeaderFields)
    }
    unsafe fn canStart(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canStart)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSession").unwrap(), new)
    }
}
pub trait PASWebAuthenticationPresentationContextProviding: Sized + std::ops::Deref {
    unsafe fn presentationAnchorForWebAuthenticationSession_(
        &self,
        session: ASWebAuthenticationSession,
    ) -> ASPresentationAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentationAnchorForWebAuthenticationSession : session)
    }
}
pub trait PASWebAuthenticationSessionRequestDelegate: Sized + std::ops::Deref {
    unsafe fn authenticationSessionRequest_didCompleteWithCallbackURL_(
        &self,
        authenticationSessionRequest: ASWebAuthenticationSessionRequest,
        callbackURL: NSURL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authenticationSessionRequest : authenticationSessionRequest, didCompleteWithCallbackURL : callbackURL)
    }
    unsafe fn authenticationSessionRequest_didCancelWithError_(
        &self,
        authenticationSessionRequest: ASWebAuthenticationSessionRequest,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authenticationSessionRequest : authenticationSessionRequest, didCancelWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASWebAuthenticationSessionRequest(pub id);
impl std::ops::Deref for ASWebAuthenticationSessionRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASWebAuthenticationSessionRequest {}
impl ASWebAuthenticationSessionRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ASWebAuthenticationSessionRequest {}
impl PNSCopying for ASWebAuthenticationSessionRequest {}
impl INSObject for ASWebAuthenticationSessionRequest {}
impl PNSObject for ASWebAuthenticationSessionRequest {}
impl std::convert::TryFrom<NSObject> for ASWebAuthenticationSessionRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASWebAuthenticationSessionRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASWebAuthenticationSessionRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASWebAuthenticationSessionRequest")
        }
    }
}
impl IASWebAuthenticationSessionRequest for ASWebAuthenticationSessionRequest {}
pub trait IASWebAuthenticationSessionRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn cancelWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelWithError : error)
    }
    unsafe fn completeWithCallbackURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeWithCallbackURL : url)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn callbackURLScheme(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callbackURLScheme)
    }
    unsafe fn shouldUseEphemeralSession(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldUseEphemeralSession)
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
    unsafe fn additionalHeaderFields(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalHeaderFields)
    }
    unsafe fn callback(&self) -> ASWebAuthenticationSessionCallback
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callback)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionRequest").unwrap(), new)
    }
}
pub trait PASWebAuthenticationSessionWebBrowserSessionHandling: Sized + std::ops::Deref {
    unsafe fn beginHandlingWebAuthenticationSessionRequest_(
        &self,
        request: ASWebAuthenticationSessionRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginHandlingWebAuthenticationSessionRequest : request)
    }
    unsafe fn cancelWebAuthenticationSessionRequest_(
        &self,
        request: ASWebAuthenticationSessionRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelWebAuthenticationSessionRequest : request)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASWebAuthenticationSessionWebBrowserSessionManager(pub id);
impl std::ops::Deref for ASWebAuthenticationSessionWebBrowserSessionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASWebAuthenticationSessionWebBrowserSessionManager {}
impl ASWebAuthenticationSessionWebBrowserSessionManager {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionWebBrowserSessionManager").unwrap(), alloc)
        })
    }
}
impl INSObject for ASWebAuthenticationSessionWebBrowserSessionManager {}
impl PNSObject for ASWebAuthenticationSessionWebBrowserSessionManager {}
impl std::convert::TryFrom<NSObject> for ASWebAuthenticationSessionWebBrowserSessionManager {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASWebAuthenticationSessionWebBrowserSessionManager, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionWebBrowserSessionManager").unwrap())
        };
        if is_kind_of {
            Ok(ASWebAuthenticationSessionWebBrowserSessionManager(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASWebAuthenticationSessionWebBrowserSessionManager" ,)
        }
    }
}
impl IASWebAuthenticationSessionWebBrowserSessionManager
    for ASWebAuthenticationSessionWebBrowserSessionManager
{
}
pub trait IASWebAuthenticationSessionWebBrowserSessionManager: Sized + std::ops::Deref {
    unsafe fn sessionHandler(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionHandler)
    }
    unsafe fn setSessionHandler_(&self, sessionHandler: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSessionHandler : sessionHandler)
    }
    unsafe fn wasLaunchedByAuthenticationServices(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wasLaunchedByAuthenticationServices)
    }
    unsafe fn sharedManager() -> ASWebAuthenticationSessionWebBrowserSessionManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionWebBrowserSessionManager").unwrap(), sharedManager)
    }
}
pub type ASCredentialIdentityStoreErrorCode = NSInteger;
pub type ASCredentialIdentityTypes = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASCredentialIdentityStore(pub id);
impl std::ops::Deref for ASCredentialIdentityStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASCredentialIdentityStore {}
impl ASCredentialIdentityStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASCredentialIdentityStore").unwrap(), alloc) })
    }
}
impl INSObject for ASCredentialIdentityStore {}
impl PNSObject for ASCredentialIdentityStore {}
impl std::convert::TryFrom<NSObject> for ASCredentialIdentityStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASCredentialIdentityStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASCredentialIdentityStore").unwrap()) };
        if is_kind_of {
            Ok(ASCredentialIdentityStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASCredentialIdentityStore")
        }
    }
}
impl IASCredentialIdentityStore for ASCredentialIdentityStore {}
pub trait IASCredentialIdentityStore: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn getCredentialIdentityStoreStateWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getCredentialIdentityStoreStateWithCompletion : completion)
    }
    unsafe fn getCredentialIdentitiesForService_credentialIdentityTypes_completionHandler_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        credentialIdentityTypes: ASCredentialIdentityTypes,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getCredentialIdentitiesForService : serviceIdentifier, credentialIdentityTypes : credentialIdentityTypes, completionHandler : completionHandler)
    }
    unsafe fn saveCredentialIdentities_completion_(
        &self,
        credentialIdentities: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveCredentialIdentities : credentialIdentities, completion : completion)
    }
    unsafe fn saveCredentialIdentityEntries_completion_(
        &self,
        credentialIdentities: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveCredentialIdentityEntries : credentialIdentities, completion : completion)
    }
    unsafe fn removeCredentialIdentities_completion_(
        &self,
        credentialIdentities: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeCredentialIdentities : credentialIdentities, completion : completion)
    }
    unsafe fn removeCredentialIdentityEntries_completion_(
        &self,
        credentialIdentities: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeCredentialIdentityEntries : credentialIdentities, completion : completion)
    }
    unsafe fn removeAllCredentialIdentitiesWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAllCredentialIdentitiesWithCompletion : completion)
    }
    unsafe fn replaceCredentialIdentitiesWithIdentities_completion_(
        &self,
        newCredentialIdentities: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceCredentialIdentitiesWithIdentities : newCredentialIdentities, completion : completion)
    }
    unsafe fn replaceCredentialIdentityEntries_completion_(
        &self,
        newCredentialIdentities: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceCredentialIdentityEntries : newCredentialIdentities, completion : completion)
    }
    unsafe fn sharedStore() -> ASCredentialIdentityStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASCredentialIdentityStore").unwrap(), sharedStore)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASCredentialIdentityStoreState(pub id);
impl std::ops::Deref for ASCredentialIdentityStoreState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASCredentialIdentityStoreState {}
impl ASCredentialIdentityStoreState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASCredentialIdentityStoreState").unwrap(), alloc) })
    }
}
impl INSObject for ASCredentialIdentityStoreState {}
impl PNSObject for ASCredentialIdentityStoreState {}
impl std::convert::TryFrom<NSObject> for ASCredentialIdentityStoreState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASCredentialIdentityStoreState, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASCredentialIdentityStoreState").unwrap())
        };
        if is_kind_of {
            Ok(ASCredentialIdentityStoreState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASCredentialIdentityStoreState")
        }
    }
}
impl IASCredentialIdentityStoreState for ASCredentialIdentityStoreState {}
pub trait IASCredentialIdentityStoreState: Sized + std::ops::Deref {
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn supportsIncrementalUpdates(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsIncrementalUpdates)
    }
}
pub type ASExtensionErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASCredentialProviderExtensionContext(pub id);
impl std::ops::Deref for ASCredentialProviderExtensionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASCredentialProviderExtensionContext {}
impl ASCredentialProviderExtensionContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASCredentialProviderExtensionContext").unwrap(), alloc) })
    }
}
impl INSExtensionContext for ASCredentialProviderExtensionContext {}
impl std::convert::TryFrom<NSExtensionContext> for ASCredentialProviderExtensionContext {
    type Error = &'static str;
    fn try_from(
        parent: NSExtensionContext,
    ) -> Result<ASCredentialProviderExtensionContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASCredentialProviderExtensionContext").unwrap())
        };
        if is_kind_of {
            Ok(ASCredentialProviderExtensionContext(parent.0))
        } else {
            Err ("This NSExtensionContext cannot be downcasted to ASCredentialProviderExtensionContext" ,)
        }
    }
}
impl INSObject for ASCredentialProviderExtensionContext {}
impl PNSObject for ASCredentialProviderExtensionContext {}
impl IASCredentialProviderExtensionContext for ASCredentialProviderExtensionContext {}
pub trait IASCredentialProviderExtensionContext: Sized + std::ops::Deref {
    unsafe fn completeRequestWithSelectedCredential_completionHandler_(
        &self,
        credential: ASPasswordCredential,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeRequestWithSelectedCredential : credential, completionHandler : completionHandler)
    }
    unsafe fn completeAssertionRequestWithSelectedPasskeyCredential_completionHandler_(
        &self,
        credential: ASPasskeyAssertionCredential,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeAssertionRequestWithSelectedPasskeyCredential : credential, completionHandler : completionHandler)
    }
    unsafe fn completeRegistrationRequestWithSelectedPasskeyCredential_completionHandler_(
        &self,
        credential: ASPasskeyRegistrationCredential,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeRegistrationRequestWithSelectedPasskeyCredential : credential, completionHandler : completionHandler)
    }
    unsafe fn completeOneTimeCodeRequestWithSelectedCredential_completionHandler_(
        &self,
        credential: ASOneTimeCodeCredential,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeOneTimeCodeRequestWithSelectedCredential : credential, completionHandler : completionHandler)
    }
    unsafe fn completeSavePasswordRequestWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeSavePasswordRequestWithCompletionHandler : completionHandler)
    }
    unsafe fn completeGeneratePasswordRequestWithResults_completionHandler_(
        &self,
        results: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeGeneratePasswordRequestWithResults : results, completionHandler : completionHandler)
    }
    unsafe fn completeExtensionConfigurationRequest(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completeExtensionConfigurationRequest)
    }
    unsafe fn completeRequestReturningItems_completionHandler_(
        &self,
        items: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeRequestReturningItems : items, completionHandler : completionHandler)
    }
    unsafe fn cancelRequestWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelRequestWithError : error)
    }
}
pub type ASCredentialServiceIdentifierType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASCredentialServiceIdentifier(pub id);
impl std::ops::Deref for ASCredentialServiceIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASCredentialServiceIdentifier {}
impl ASCredentialServiceIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASCredentialServiceIdentifier").unwrap(), alloc) })
    }
}
impl PNSCopying for ASCredentialServiceIdentifier {}
impl PNSSecureCoding for ASCredentialServiceIdentifier {}
impl INSObject for ASCredentialServiceIdentifier {}
impl PNSObject for ASCredentialServiceIdentifier {}
impl std::convert::TryFrom<NSObject> for ASCredentialServiceIdentifier {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASCredentialServiceIdentifier, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASCredentialServiceIdentifier").unwrap())
        };
        if is_kind_of {
            Ok(ASCredentialServiceIdentifier(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASCredentialServiceIdentifier")
        }
    }
}
impl IASCredentialServiceIdentifier for ASCredentialServiceIdentifier {}
pub trait IASCredentialServiceIdentifier: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_type_(
        &self,
        identifier: NSString,
        type_: ASCredentialServiceIdentifierType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, r#type : type_)
    }
    unsafe fn initWithIdentifier_type_displayName_(
        &self,
        identifier: NSString,
        type_: ASCredentialServiceIdentifierType,
        displayName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, r#type : type_, displayName : displayName)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn type_(&self) -> ASCredentialServiceIdentifierType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
pub trait PASAuthorizationCredential: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasswordCredential(pub id);
impl std::ops::Deref for ASPasswordCredential {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasswordCredential {}
impl ASPasswordCredential {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasswordCredential").unwrap(), alloc) })
    }
}
impl PASAuthorizationCredential for ASPasswordCredential {}
impl INSObject for ASPasswordCredential {}
impl PNSObject for ASPasswordCredential {}
impl std::convert::TryFrom<NSObject> for ASPasswordCredential {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPasswordCredential, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasswordCredential").unwrap()) };
        if is_kind_of {
            Ok(ASPasswordCredential(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasswordCredential")
        }
    }
}
impl IASPasswordCredential for ASPasswordCredential {}
pub trait IASPasswordCredential: Sized + std::ops::Deref {
    unsafe fn initWithUser_password_(&self, user: NSString, password: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUser : user, password : password)
    }
    unsafe fn user(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, user)
    }
    unsafe fn password(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, password)
    }
    unsafe fn credentialWithUser_password_(user: NSString, password: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasswordCredential").unwrap(), credentialWithUser : user, password : password)
    }
}
pub trait PASCredentialIdentity: Sized + std::ops::Deref {
    unsafe fn serviceIdentifier(&self) -> ASCredentialServiceIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceIdentifier)
    }
    unsafe fn user(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, user)
    }
    unsafe fn recordIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordIdentifier)
    }
    unsafe fn rank(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rank)
    }
    unsafe fn setRank_(&self, rank: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRank : rank)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasswordCredentialIdentity(pub id);
impl std::ops::Deref for ASPasswordCredentialIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasswordCredentialIdentity {}
impl ASPasswordCredentialIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasswordCredentialIdentity").unwrap(), alloc) })
    }
}
impl PNSCopying for ASPasswordCredentialIdentity {}
impl PNSSecureCoding for ASPasswordCredentialIdentity {}
impl PASCredentialIdentity for ASPasswordCredentialIdentity {}
impl INSObject for ASPasswordCredentialIdentity {}
impl PNSObject for ASPasswordCredentialIdentity {}
impl std::convert::TryFrom<NSObject> for ASPasswordCredentialIdentity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPasswordCredentialIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasswordCredentialIdentity").unwrap()) };
        if is_kind_of {
            Ok(ASPasswordCredentialIdentity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasswordCredentialIdentity")
        }
    }
}
impl IASPasswordCredentialIdentity for ASPasswordCredentialIdentity {}
pub trait IASPasswordCredentialIdentity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithServiceIdentifier_user_recordIdentifier_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        user: NSString,
        recordIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceIdentifier : serviceIdentifier, user : user, recordIdentifier : recordIdentifier)
    }
    unsafe fn serviceIdentifier(&self) -> ASCredentialServiceIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceIdentifier)
    }
    unsafe fn user(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, user)
    }
    unsafe fn recordIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordIdentifier)
    }
    unsafe fn rank(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rank)
    }
    unsafe fn setRank_(&self, rank: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRank : rank)
    }
    unsafe fn identityWithServiceIdentifier_user_recordIdentifier_(
        serviceIdentifier: ASCredentialServiceIdentifier,
        user: NSString,
        recordIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasswordCredentialIdentity").unwrap(), identityWithServiceIdentifier : serviceIdentifier, user : user, recordIdentifier : recordIdentifier)
    }
}
pub type ASAuthorizationScope = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorization(pub id);
impl std::ops::Deref for ASAuthorization {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorization {}
impl ASAuthorization {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorization").unwrap(), alloc) })
    }
}
impl INSObject for ASAuthorization {}
impl PNSObject for ASAuthorization {}
impl std::convert::TryFrom<NSObject> for ASAuthorization {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAuthorization, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorization").unwrap()) };
        if is_kind_of {
            Ok(ASAuthorization(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAuthorization")
        }
    }
}
impl IASAuthorization for ASAuthorization {}
pub trait IASAuthorization: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn provider(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, provider)
    }
    unsafe fn credential(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credential)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorization").unwrap(), new)
    }
}
pub type ASUserDetectionStatus = NSInteger;
pub type ASUserAgeRange = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationAppleIDCredential(pub id);
impl std::ops::Deref for ASAuthorizationAppleIDCredential {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationAppleIDCredential {}
impl ASAuthorizationAppleIDCredential {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDCredential").unwrap(), alloc) })
    }
}
impl PASAuthorizationCredential for ASAuthorizationAppleIDCredential {}
impl INSObject for ASAuthorizationAppleIDCredential {}
impl PNSObject for ASAuthorizationAppleIDCredential {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationAppleIDCredential {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAuthorizationAppleIDCredential, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDCredential").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationAppleIDCredential(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAuthorizationAppleIDCredential")
        }
    }
}
impl IASAuthorizationAppleIDCredential for ASAuthorizationAppleIDCredential {}
pub trait IASAuthorizationAppleIDCredential: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn user(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, user)
    }
    unsafe fn state(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn authorizedScopes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizedScopes)
    }
    unsafe fn authorizationCode(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationCode)
    }
    unsafe fn identityToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityToken)
    }
    unsafe fn email(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, email)
    }
    unsafe fn fullName(&self) -> NSPersonNameComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullName)
    }
    unsafe fn realUserStatus(&self) -> ASUserDetectionStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, realUserStatus)
    }
    unsafe fn userAgeRange(&self) -> ASUserAgeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userAgeRange)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDCredential").unwrap(), new)
    }
}
pub trait PASAuthorizationProvider: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationRequest(pub id);
impl std::ops::Deref for ASAuthorizationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationRequest {}
impl ASAuthorizationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for ASAuthorizationRequest {}
impl PNSSecureCoding for ASAuthorizationRequest {}
impl INSObject for ASAuthorizationRequest {}
impl PNSObject for ASAuthorizationRequest {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAuthorizationRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationRequest").unwrap()) };
        if is_kind_of {
            Ok(ASAuthorizationRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAuthorizationRequest")
        }
    }
}
impl IASAuthorizationRequest for ASAuthorizationRequest {}
pub trait IASAuthorizationRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn provider(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, provider)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationRequest").unwrap(), new)
    }
}
pub type ASAuthorizationOpenIDOperation = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationOpenIDRequest(pub id);
impl std::ops::Deref for ASAuthorizationOpenIDRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationOpenIDRequest {}
impl ASAuthorizationOpenIDRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationOpenIDRequest").unwrap(), alloc) })
    }
}
impl IASAuthorizationRequest for ASAuthorizationOpenIDRequest {}
impl PNSCopying for ASAuthorizationOpenIDRequest {}
impl PNSSecureCoding for ASAuthorizationOpenIDRequest {}
impl From<ASAuthorizationOpenIDRequest> for ASAuthorizationRequest {
    fn from(child: ASAuthorizationOpenIDRequest) -> ASAuthorizationRequest {
        ASAuthorizationRequest(child.0)
    }
}
impl std::convert::TryFrom<ASAuthorizationRequest> for ASAuthorizationOpenIDRequest {
    type Error = &'static str;
    fn try_from(
        parent: ASAuthorizationRequest,
    ) -> Result<ASAuthorizationOpenIDRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationOpenIDRequest").unwrap()) };
        if is_kind_of {
            Ok(ASAuthorizationOpenIDRequest(parent.0))
        } else {
            Err("This ASAuthorizationRequest cannot be downcasted to ASAuthorizationOpenIDRequest")
        }
    }
}
impl INSObject for ASAuthorizationOpenIDRequest {}
impl PNSObject for ASAuthorizationOpenIDRequest {}
impl IASAuthorizationOpenIDRequest for ASAuthorizationOpenIDRequest {}
pub trait IASAuthorizationOpenIDRequest: Sized + std::ops::Deref {
    unsafe fn requestedScopes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedScopes)
    }
    unsafe fn setRequestedScopes_(&self, requestedScopes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequestedScopes : requestedScopes)
    }
    unsafe fn state(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn setState_(&self, state: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setState : state)
    }
    unsafe fn nonce(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nonce)
    }
    unsafe fn setNonce_(&self, nonce: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNonce : nonce)
    }
    unsafe fn requestedOperation(&self) -> ASAuthorizationOpenIDOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedOperation)
    }
    unsafe fn setRequestedOperation_(&self, requestedOperation: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequestedOperation : requestedOperation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationAppleIDRequest(pub id);
impl std::ops::Deref for ASAuthorizationAppleIDRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationAppleIDRequest {}
impl ASAuthorizationAppleIDRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDRequest").unwrap(), alloc) })
    }
}
impl IASAuthorizationOpenIDRequest for ASAuthorizationAppleIDRequest {}
impl From<ASAuthorizationAppleIDRequest> for ASAuthorizationOpenIDRequest {
    fn from(child: ASAuthorizationAppleIDRequest) -> ASAuthorizationOpenIDRequest {
        ASAuthorizationOpenIDRequest(child.0)
    }
}
impl std::convert::TryFrom<ASAuthorizationOpenIDRequest> for ASAuthorizationAppleIDRequest {
    type Error = &'static str;
    fn try_from(
        parent: ASAuthorizationOpenIDRequest,
    ) -> Result<ASAuthorizationAppleIDRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationAppleIDRequest(parent.0))
        } else {
            Err ("This ASAuthorizationOpenIDRequest cannot be downcasted to ASAuthorizationAppleIDRequest" ,)
        }
    }
}
impl IASAuthorizationRequest for ASAuthorizationAppleIDRequest {}
impl PNSCopying for ASAuthorizationAppleIDRequest {}
impl PNSSecureCoding for ASAuthorizationAppleIDRequest {}
impl INSObject for ASAuthorizationAppleIDRequest {}
impl PNSObject for ASAuthorizationAppleIDRequest {}
impl IASAuthorizationAppleIDRequest for ASAuthorizationAppleIDRequest {}
pub trait IASAuthorizationAppleIDRequest: Sized + std::ops::Deref {
    unsafe fn user(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, user)
    }
    unsafe fn setUser_(&self, user: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUser : user)
    }
}
pub type ASAuthorizationAppleIDProviderCredentialState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationAppleIDProvider(pub id);
impl std::ops::Deref for ASAuthorizationAppleIDProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationAppleIDProvider {}
impl ASAuthorizationAppleIDProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDProvider").unwrap(), alloc) })
    }
}
impl PASAuthorizationProvider for ASAuthorizationAppleIDProvider {}
impl INSObject for ASAuthorizationAppleIDProvider {}
impl PNSObject for ASAuthorizationAppleIDProvider {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationAppleIDProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAuthorizationAppleIDProvider, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDProvider").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationAppleIDProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAuthorizationAppleIDProvider")
        }
    }
}
impl IASAuthorizationAppleIDProvider for ASAuthorizationAppleIDProvider {}
pub trait IASAuthorizationAppleIDProvider: Sized + std::ops::Deref {
    unsafe fn createRequest(&self) -> ASAuthorizationAppleIDRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createRequest)
    }
    unsafe fn getCredentialStateForUserID_completion_(
        &self,
        userID: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getCredentialStateForUserID : userID, completion : completion)
    }
}
pub trait PASAuthorizationControllerDelegate: Sized + std::ops::Deref {
    unsafe fn authorizationController_didCompleteWithAuthorization_(
        &self,
        controller: ASAuthorizationController,
        authorization: ASAuthorization,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationController : controller, didCompleteWithAuthorization : authorization)
    }
    unsafe fn authorizationController_didCompleteWithError_(
        &self,
        controller: ASAuthorizationController,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationController : controller, didCompleteWithError : error)
    }
    unsafe fn authorizationController_didCompleteWithCustomMethod_(
        &self,
        controller: ASAuthorizationController,
        method: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationController : controller, didCompleteWithCustomMethod : method)
    }
}
pub trait PASAuthorizationControllerPresentationContextProviding: Sized + std::ops::Deref {
    unsafe fn presentationAnchorForAuthorizationController_(
        &self,
        controller: ASAuthorizationController,
    ) -> ASPresentationAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentationAnchorForAuthorizationController : controller)
    }
}
pub type ASAuthorizationControllerRequestOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationController(pub id);
impl std::ops::Deref for ASAuthorizationController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationController {}
impl ASAuthorizationController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationController").unwrap(), alloc) })
    }
}
impl INSObject for ASAuthorizationController {}
impl PNSObject for ASAuthorizationController {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAuthorizationController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationController").unwrap()) };
        if is_kind_of {
            Ok(ASAuthorizationController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAuthorizationController")
        }
    }
}
impl IASAuthorizationController for ASAuthorizationController {}
pub trait IASAuthorizationController: Sized + std::ops::Deref {
    unsafe fn initWithAuthorizationRequests_(&self, authorizationRequests: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAuthorizationRequests : authorizationRequests)
    }
    unsafe fn performRequests(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, performRequests)
    }
    unsafe fn performAutoFillAssistedRequests(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, performAutoFillAssistedRequests)
    }
    unsafe fn performRequestsWithOptions_(&self, options: ASAuthorizationControllerRequestOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequestsWithOptions : options)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn authorizationRequests(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRequests)
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
    unsafe fn presentationContextProvider(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationContextProvider)
    }
    unsafe fn setPresentationContextProvider_(&self, presentationContextProvider: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresentationContextProvider : presentationContextProvider)
    }
    unsafe fn customAuthorizationMethods(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customAuthorizationMethods)
    }
    unsafe fn setCustomAuthorizationMethods_(&self, customAuthorizationMethods: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomAuthorizationMethods : customAuthorizationMethods)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationController").unwrap(), new)
    }
}
pub type ASAuthorizationError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPasswordRequest(pub id);
impl std::ops::Deref for ASAuthorizationPasswordRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPasswordRequest {}
impl ASAuthorizationPasswordRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPasswordRequest").unwrap(), alloc) })
    }
}
impl IASAuthorizationRequest for ASAuthorizationPasswordRequest {}
impl PNSCopying for ASAuthorizationPasswordRequest {}
impl PNSSecureCoding for ASAuthorizationPasswordRequest {}
impl std::convert::TryFrom<ASAuthorizationRequest> for ASAuthorizationPasswordRequest {
    type Error = &'static str;
    fn try_from(
        parent: ASAuthorizationRequest,
    ) -> Result<ASAuthorizationPasswordRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPasswordRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPasswordRequest(parent.0))
        } else {
            Err ("This ASAuthorizationRequest cannot be downcasted to ASAuthorizationPasswordRequest" ,)
        }
    }
}
impl INSObject for ASAuthorizationPasswordRequest {}
impl PNSObject for ASAuthorizationPasswordRequest {}
impl IASAuthorizationPasswordRequest for ASAuthorizationPasswordRequest {}
pub trait IASAuthorizationPasswordRequest: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPasswordProvider(pub id);
impl std::ops::Deref for ASAuthorizationPasswordProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPasswordProvider {}
impl ASAuthorizationPasswordProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPasswordProvider").unwrap(), alloc) })
    }
}
impl PASAuthorizationProvider for ASAuthorizationPasswordProvider {}
impl INSObject for ASAuthorizationPasswordProvider {}
impl PNSObject for ASAuthorizationPasswordProvider {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPasswordProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAuthorizationPasswordProvider, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPasswordProvider").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPasswordProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAuthorizationPasswordProvider")
        }
    }
}
impl IASAuthorizationPasswordProvider for ASAuthorizationPasswordProvider {}
pub trait IASAuthorizationPasswordProvider: Sized + std::ops::Deref {
    unsafe fn createRequest(&self) -> ASAuthorizationPasswordRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createRequest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSingleSignOnCredential(pub id);
impl std::ops::Deref for ASAuthorizationSingleSignOnCredential {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSingleSignOnCredential {}
impl ASAuthorizationSingleSignOnCredential {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnCredential").unwrap(), alloc) })
    }
}
impl PASAuthorizationCredential for ASAuthorizationSingleSignOnCredential {}
impl INSObject for ASAuthorizationSingleSignOnCredential {}
impl PNSObject for ASAuthorizationSingleSignOnCredential {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationSingleSignOnCredential {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAuthorizationSingleSignOnCredential, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnCredential").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSingleSignOnCredential(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAuthorizationSingleSignOnCredential")
        }
    }
}
impl IASAuthorizationSingleSignOnCredential for ASAuthorizationSingleSignOnCredential {}
pub trait IASAuthorizationSingleSignOnCredential: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn state(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn accessToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessToken)
    }
    unsafe fn identityToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityToken)
    }
    unsafe fn authorizedScopes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizedScopes)
    }
    unsafe fn authenticatedResponse(&self) -> NSHTTPURLResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticatedResponse)
    }
    unsafe fn privateKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, privateKeys)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnCredential").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSingleSignOnRequest(pub id);
impl std::ops::Deref for ASAuthorizationSingleSignOnRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSingleSignOnRequest {}
impl ASAuthorizationSingleSignOnRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnRequest").unwrap(), alloc) })
    }
}
impl IASAuthorizationOpenIDRequest for ASAuthorizationSingleSignOnRequest {}
impl std::convert::TryFrom<ASAuthorizationOpenIDRequest> for ASAuthorizationSingleSignOnRequest {
    type Error = &'static str;
    fn try_from(
        parent: ASAuthorizationOpenIDRequest,
    ) -> Result<ASAuthorizationSingleSignOnRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSingleSignOnRequest(parent.0))
        } else {
            Err ("This ASAuthorizationOpenIDRequest cannot be downcasted to ASAuthorizationSingleSignOnRequest" ,)
        }
    }
}
impl IASAuthorizationRequest for ASAuthorizationSingleSignOnRequest {}
impl PNSCopying for ASAuthorizationSingleSignOnRequest {}
impl PNSSecureCoding for ASAuthorizationSingleSignOnRequest {}
impl INSObject for ASAuthorizationSingleSignOnRequest {}
impl PNSObject for ASAuthorizationSingleSignOnRequest {}
impl IASAuthorizationSingleSignOnRequest for ASAuthorizationSingleSignOnRequest {}
pub trait IASAuthorizationSingleSignOnRequest: Sized + std::ops::Deref {
    unsafe fn authorizationOptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationOptions)
    }
    unsafe fn setAuthorizationOptions_(&self, authorizationOptions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorizationOptions : authorizationOptions)
    }
    unsafe fn isUserInterfaceEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserInterfaceEnabled)
    }
    unsafe fn setUserInterfaceEnabled_(&self, userInterfaceEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInterfaceEnabled : userInterfaceEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSingleSignOnProvider(pub id);
impl std::ops::Deref for ASAuthorizationSingleSignOnProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSingleSignOnProvider {}
impl ASAuthorizationSingleSignOnProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnProvider").unwrap(), alloc) })
    }
}
impl PASAuthorizationProvider for ASAuthorizationSingleSignOnProvider {}
impl INSObject for ASAuthorizationSingleSignOnProvider {}
impl PNSObject for ASAuthorizationSingleSignOnProvider {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationSingleSignOnProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAuthorizationSingleSignOnProvider, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnProvider").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSingleSignOnProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAuthorizationSingleSignOnProvider")
        }
    }
}
impl IASAuthorizationSingleSignOnProvider for ASAuthorizationSingleSignOnProvider {}
pub trait IASAuthorizationSingleSignOnProvider: Sized + std::ops::Deref {
    unsafe fn createRequest(&self) -> ASAuthorizationSingleSignOnRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createRequest)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn canPerformAuthorization(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canPerformAuthorization)
    }
    unsafe fn authorizationProviderWithIdentityProviderURL_(url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnProvider").unwrap(), authorizationProviderWithIdentityProviderURL : url)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSingleSignOnProvider").unwrap(), new)
    }
}
pub type ASAuthorizationProviderAuthorizationOperation = NSString;
pub trait PASAuthorizationProviderExtensionAuthorizationRequestHandler:
    Sized + std::ops::Deref
{
    unsafe fn beginAuthorizationWithRequest_(
        &self,
        request: ASAuthorizationProviderExtensionAuthorizationRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginAuthorizationWithRequest : request)
    }
    unsafe fn cancelAuthorizationWithRequest_(
        &self,
        request: ASAuthorizationProviderExtensionAuthorizationRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelAuthorizationWithRequest : request)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationProviderExtensionAuthorizationRequest(pub id);
impl std::ops::Deref for ASAuthorizationProviderExtensionAuthorizationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationProviderExtensionAuthorizationRequest {}
impl ASAuthorizationProviderExtensionAuthorizationRequest {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionAuthorizationRequest").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationProviderExtensionAuthorizationRequest {}
impl PNSObject for ASAuthorizationProviderExtensionAuthorizationRequest {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationProviderExtensionAuthorizationRequest {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationProviderExtensionAuthorizationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionAuthorizationRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationProviderExtensionAuthorizationRequest(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationProviderExtensionAuthorizationRequest" ,)
        }
    }
}
impl IASAuthorizationProviderExtensionAuthorizationRequest
    for ASAuthorizationProviderExtensionAuthorizationRequest
{
}
pub trait IASAuthorizationProviderExtensionAuthorizationRequest: Sized + std::ops::Deref {
    unsafe fn doNotHandle(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doNotHandle)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn complete(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, complete)
    }
    unsafe fn completeWithHTTPAuthorizationHeaders_(&self, httpAuthorizationHeaders: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeWithHTTPAuthorizationHeaders : httpAuthorizationHeaders)
    }
    unsafe fn completeWithHTTPResponse_httpBody_(
        &self,
        httpResponse: NSHTTPURLResponse,
        httpBody: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeWithHTTPResponse : httpResponse, httpBody : httpBody)
    }
    unsafe fn completeWithAuthorizationResult_(
        &self,
        authorizationResult: ASAuthorizationProviderExtensionAuthorizationResult,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeWithAuthorizationResult : authorizationResult)
    }
    unsafe fn completeWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeWithError : error)
    }
    unsafe fn presentAuthorizationViewControllerWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentAuthorizationViewControllerWithCompletion : completion)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn requestedOperation(&self) -> ASAuthorizationProviderAuthorizationOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedOperation)
    }
    unsafe fn httpHeaders(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, httpHeaders)
    }
    unsafe fn httpBody(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, httpBody)
    }
    unsafe fn realm(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, realm)
    }
    unsafe fn extensionData(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionData)
    }
    unsafe fn callerBundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callerBundleIdentifier)
    }
    unsafe fn authorizationOptions(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationOptions)
    }
    unsafe fn isCallerManaged(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCallerManaged)
    }
    unsafe fn callerTeamIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callerTeamIdentifier)
    }
    unsafe fn localizedCallerDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedCallerDisplayName)
    }
    unsafe fn callerAuditToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callerAuditToken)
    }
    unsafe fn isUserInterfaceEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserInterfaceEnabled)
    }
    unsafe fn loginManager(&self) -> ASAuthorizationProviderExtensionLoginManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationProviderExtensionAuthorizationResult(pub id);
impl std::ops::Deref for ASAuthorizationProviderExtensionAuthorizationResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationProviderExtensionAuthorizationResult {}
impl ASAuthorizationProviderExtensionAuthorizationResult {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionAuthorizationResult").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationProviderExtensionAuthorizationResult {}
impl PNSObject for ASAuthorizationProviderExtensionAuthorizationResult {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationProviderExtensionAuthorizationResult {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationProviderExtensionAuthorizationResult, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionAuthorizationResult").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationProviderExtensionAuthorizationResult(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationProviderExtensionAuthorizationResult" ,)
        }
    }
}
impl IASAuthorizationProviderExtensionAuthorizationResult
    for ASAuthorizationProviderExtensionAuthorizationResult
{
}
pub trait IASAuthorizationProviderExtensionAuthorizationResult: Sized + std::ops::Deref {
    unsafe fn initWithHTTPAuthorizationHeaders_(
        &self,
        httpAuthorizationHeaders: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHTTPAuthorizationHeaders : httpAuthorizationHeaders)
    }
    unsafe fn initWithHTTPResponse_httpBody_(
        &self,
        httpResponse: NSHTTPURLResponse,
        httpBody: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHTTPResponse : httpResponse, httpBody : httpBody)
    }
    unsafe fn httpAuthorizationHeaders(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, httpAuthorizationHeaders)
    }
    unsafe fn setHttpAuthorizationHeaders_(&self, httpAuthorizationHeaders: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHttpAuthorizationHeaders : httpAuthorizationHeaders)
    }
    unsafe fn httpResponse(&self) -> NSHTTPURLResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, httpResponse)
    }
    unsafe fn setHttpResponse_(&self, httpResponse: NSHTTPURLResponse)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHttpResponse : httpResponse)
    }
    unsafe fn httpBody(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, httpBody)
    }
    unsafe fn setHttpBody_(&self, httpBody: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHttpBody : httpBody)
    }
    unsafe fn privateKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, privateKeys)
    }
    unsafe fn setPrivateKeys_(&self, privateKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrivateKeys : privateKeys)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASOneTimeCodeCredentialIdentity(pub id);
impl std::ops::Deref for ASOneTimeCodeCredentialIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASOneTimeCodeCredentialIdentity {}
impl ASOneTimeCodeCredentialIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASOneTimeCodeCredentialIdentity").unwrap(), alloc) })
    }
}
impl PNSCopying for ASOneTimeCodeCredentialIdentity {}
impl PNSSecureCoding for ASOneTimeCodeCredentialIdentity {}
impl PASCredentialIdentity for ASOneTimeCodeCredentialIdentity {}
impl INSObject for ASOneTimeCodeCredentialIdentity {}
impl PNSObject for ASOneTimeCodeCredentialIdentity {}
impl std::convert::TryFrom<NSObject> for ASOneTimeCodeCredentialIdentity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASOneTimeCodeCredentialIdentity, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASOneTimeCodeCredentialIdentity").unwrap())
        };
        if is_kind_of {
            Ok(ASOneTimeCodeCredentialIdentity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASOneTimeCodeCredentialIdentity")
        }
    }
}
impl IASOneTimeCodeCredentialIdentity for ASOneTimeCodeCredentialIdentity {}
pub trait IASOneTimeCodeCredentialIdentity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithServiceIdentifier_label_recordIdentifier_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        label: NSString,
        recordIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceIdentifier : serviceIdentifier, label : label, recordIdentifier : recordIdentifier)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
}
pub type ASAuthorizationPublicKeyCredentialUserVerificationPreference = NSString;
pub type ASAuthorizationPublicKeyCredentialAttestationKind = NSString;
pub type ASAuthorizationPublicKeyCredentialResidentKeyPreference = NSString;
pub type ASAuthorizationPublicKeyCredentialAttachment = NSInteger;
pub trait PASAuthorizationPublicKeyCredentialRegistrationRequest: Sized + std::ops::Deref {
    unsafe fn relyingPartyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingPartyIdentifier)
    }
    unsafe fn userID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userID)
    }
    unsafe fn setUserID_(&self, userID: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserID : userID)
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
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn setDisplayName_(&self, displayName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayName : displayName)
    }
    unsafe fn challenge(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, challenge)
    }
    unsafe fn setChallenge_(&self, challenge: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChallenge : challenge)
    }
    unsafe fn userVerificationPreference(
        &self,
    ) -> ASAuthorizationPublicKeyCredentialUserVerificationPreference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userVerificationPreference)
    }
    unsafe fn setUserVerificationPreference_(&self, userVerificationPreference: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserVerificationPreference : userVerificationPreference)
    }
    unsafe fn attestationPreference(&self) -> ASAuthorizationPublicKeyCredentialAttestationKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attestationPreference)
    }
    unsafe fn setAttestationPreference_(&self, attestationPreference: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttestationPreference : attestationPreference)
    }
}
pub type ASCredentialRequestType = NSInteger;
pub trait PASCredentialRequest: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> ASCredentialRequestType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn credentialIdentity(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialIdentity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyCredentialRequestParameters(pub id);
impl std::ops::Deref for ASPasskeyCredentialRequestParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyCredentialRequestParameters {}
impl ASPasskeyCredentialRequestParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyCredentialRequestParameters").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ASPasskeyCredentialRequestParameters {}
impl PNSCopying for ASPasskeyCredentialRequestParameters {}
impl INSObject for ASPasskeyCredentialRequestParameters {}
impl PNSObject for ASPasskeyCredentialRequestParameters {}
impl std::convert::TryFrom<NSObject> for ASPasskeyCredentialRequestParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPasskeyCredentialRequestParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyCredentialRequestParameters").unwrap())
        };
        if is_kind_of {
            Ok(ASPasskeyCredentialRequestParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasskeyCredentialRequestParameters")
        }
    }
}
impl IASPasskeyCredentialRequestParameters for ASPasskeyCredentialRequestParameters {}
pub trait IASPasskeyCredentialRequestParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn relyingPartyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingPartyIdentifier)
    }
    unsafe fn clientDataHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientDataHash)
    }
    unsafe fn userVerificationPreference(
        &self,
    ) -> ASAuthorizationPublicKeyCredentialUserVerificationPreference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userVerificationPreference)
    }
    unsafe fn allowedCredentials(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedCredentials)
    }
    unsafe fn extensionInput(&self) -> ASPasskeyAssertionCredentialExtensionInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionInput)
    }
}
pub type ASGeneratedPasswordKind = NSString;
pub type ASSavePasswordRequestEvent = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASSavePasswordRequest(pub id);
impl std::ops::Deref for ASSavePasswordRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASSavePasswordRequest {}
impl ASSavePasswordRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASSavePasswordRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for ASSavePasswordRequest {}
impl PNSSecureCoding for ASSavePasswordRequest {}
impl INSObject for ASSavePasswordRequest {}
impl PNSObject for ASSavePasswordRequest {}
impl std::convert::TryFrom<NSObject> for ASSavePasswordRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASSavePasswordRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASSavePasswordRequest").unwrap()) };
        if is_kind_of {
            Ok(ASSavePasswordRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASSavePasswordRequest")
        }
    }
}
impl IASSavePasswordRequest for ASSavePasswordRequest {}
pub trait IASSavePasswordRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithServiceIdentifier_credential_sessionID_event_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        credential: ASPasswordCredential,
        sessionID: NSString,
        event: ASSavePasswordRequestEvent,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceIdentifier : serviceIdentifier, credential : credential, sessionID : sessionID, event : event)
    }
    unsafe fn initWithServiceIdentifier_credential_title_sessionID_event_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        credential: ASPasswordCredential,
        title: NSString,
        sessionID: NSString,
        event: ASSavePasswordRequestEvent,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceIdentifier : serviceIdentifier, credential : credential, title : title, sessionID : sessionID, event : event)
    }
    unsafe fn initWithServiceIdentifier_credential_sessionID_event_passwordKind_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        credential: ASPasswordCredential,
        sessionID: NSString,
        event: ASSavePasswordRequestEvent,
        passwordKind: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceIdentifier : serviceIdentifier, credential : credential, sessionID : sessionID, event : event, passwordKind : passwordKind)
    }
    unsafe fn initWithServiceIdentifier_credential_title_sessionID_event_passwordKind_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        credential: ASPasswordCredential,
        title: NSString,
        sessionID: NSString,
        event: ASSavePasswordRequestEvent,
        passwordKind: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceIdentifier : serviceIdentifier, credential : credential, title : title, sessionID : sessionID, event : event, passwordKind : passwordKind)
    }
    unsafe fn serviceIdentifier(&self) -> ASCredentialServiceIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceIdentifier)
    }
    unsafe fn credential(&self) -> ASPasswordCredential
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credential)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn sessionID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionID)
    }
    unsafe fn event(&self) -> ASSavePasswordRequestEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, event)
    }
    unsafe fn passwordKind(&self) -> ASGeneratedPasswordKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passwordKind)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASSavePasswordRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASGeneratePasswordsRequest(pub id);
impl std::ops::Deref for ASGeneratePasswordsRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASGeneratePasswordsRequest {}
impl ASGeneratePasswordsRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASGeneratePasswordsRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for ASGeneratePasswordsRequest {}
impl PNSSecureCoding for ASGeneratePasswordsRequest {}
impl INSObject for ASGeneratePasswordsRequest {}
impl PNSObject for ASGeneratePasswordsRequest {}
impl std::convert::TryFrom<NSObject> for ASGeneratePasswordsRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASGeneratePasswordsRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASGeneratePasswordsRequest").unwrap()) };
        if is_kind_of {
            Ok(ASGeneratePasswordsRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASGeneratePasswordsRequest")
        }
    }
}
impl IASGeneratePasswordsRequest for ASGeneratePasswordsRequest {}
pub trait IASGeneratePasswordsRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithServiceIdentifier_passwordFieldPasswordRules_confirmPasswordFieldPasswordRules_passwordRulesFromQuirks_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        passwordFieldPasswordRules: NSString,
        confirmPasswordFieldPasswordRules: NSString,
        passwordRulesFromQuirks: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceIdentifier : serviceIdentifier, passwordFieldPasswordRules : passwordFieldPasswordRules, confirmPasswordFieldPasswordRules : confirmPasswordFieldPasswordRules, passwordRulesFromQuirks : passwordRulesFromQuirks)
    }
    unsafe fn serviceIdentifier(&self) -> ASCredentialServiceIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceIdentifier)
    }
    unsafe fn passwordFieldPasswordRules(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passwordFieldPasswordRules)
    }
    unsafe fn confirmPasswordFieldPasswordRules(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confirmPasswordFieldPasswordRules)
    }
    unsafe fn passwordRulesFromQuirks(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passwordRulesFromQuirks)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASGeneratePasswordsRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASCredentialProviderViewController(pub id);
impl std::ops::Deref for ASCredentialProviderViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASCredentialProviderViewController {}
impl ASCredentialProviderViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASCredentialProviderViewController").unwrap(), alloc) })
    }
}
impl INSViewController for ASCredentialProviderViewController {}
impl PNSEditor for ASCredentialProviderViewController {}
impl PNSSeguePerforming for ASCredentialProviderViewController {}
impl PNSUserInterfaceItemIdentification for ASCredentialProviderViewController {}
impl std::convert::TryFrom<NSViewController> for ASCredentialProviderViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSViewController,
    ) -> Result<ASCredentialProviderViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASCredentialProviderViewController").unwrap())
        };
        if is_kind_of {
            Ok(ASCredentialProviderViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to ASCredentialProviderViewController")
        }
    }
}
impl INSResponder for ASCredentialProviderViewController {}
impl PNSCoding for ASCredentialProviderViewController {}
impl INSObject for ASCredentialProviderViewController {}
impl PNSObject for ASCredentialProviderViewController {}
impl IASCredentialProviderViewController for ASCredentialProviderViewController {}
pub trait IASCredentialProviderViewController: Sized + std::ops::Deref {
    unsafe fn prepareCredentialListForServiceIdentifiers_(&self, serviceIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareCredentialListForServiceIdentifiers : serviceIdentifiers)
    }
    unsafe fn prepareCredentialListForServiceIdentifiers_requestParameters_(
        &self,
        serviceIdentifiers: NSArray,
        requestParameters: ASPasskeyCredentialRequestParameters,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareCredentialListForServiceIdentifiers : serviceIdentifiers, requestParameters : requestParameters)
    }
    unsafe fn prepareOneTimeCodeCredentialListForServiceIdentifiers_(
        &self,
        serviceIdentifiers: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareOneTimeCodeCredentialListForServiceIdentifiers : serviceIdentifiers)
    }
    unsafe fn provideCredentialWithoutUserInteractionForIdentity_(
        &self,
        credentialIdentity: ASPasswordCredentialIdentity,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideCredentialWithoutUserInteractionForIdentity : credentialIdentity)
    }
    unsafe fn provideCredentialWithoutUserInteractionForRequest_(&self, credentialRequest: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideCredentialWithoutUserInteractionForRequest : credentialRequest)
    }
    unsafe fn prepareInterfaceToProvideCredentialForIdentity_(
        &self,
        credentialIdentity: ASPasswordCredentialIdentity,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareInterfaceToProvideCredentialForIdentity : credentialIdentity)
    }
    unsafe fn prepareInterfaceToProvideCredentialForRequest_(&self, credentialRequest: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareInterfaceToProvideCredentialForRequest : credentialRequest)
    }
    unsafe fn prepareInterfaceForExtensionConfiguration(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareInterfaceForExtensionConfiguration)
    }
    unsafe fn prepareInterfaceForPasskeyRegistration_(&self, registrationRequest: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareInterfaceForPasskeyRegistration : registrationRequest)
    }
    unsafe fn performPasskeyRegistrationWithoutUserInteractionIfPossible_(
        &self,
        registrationRequest: ASPasskeyCredentialRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performPasskeyRegistrationWithoutUserInteractionIfPossible : registrationRequest)
    }
    unsafe fn reportPublicKeyCredentialUpdateForRelyingParty_userHandle_newName_(
        &self,
        relyingParty: NSString,
        userHandle: NSData,
        newName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportPublicKeyCredentialUpdateForRelyingParty : relyingParty, userHandle : userHandle, newName : newName)
    }
    unsafe fn reportUnknownPublicKeyCredentialForRelyingParty_credentialID_(
        &self,
        relyingParty: NSString,
        credentialID: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportUnknownPublicKeyCredentialForRelyingParty : relyingParty, credentialID : credentialID)
    }
    unsafe fn reportAllAcceptedPublicKeyCredentialsForRelyingParty_userHandle_acceptedCredentialIDs_(
        &self,
        relyingParty: NSString,
        userHandle: NSData,
        acceptedCredentialIDs: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportAllAcceptedPublicKeyCredentialsForRelyingParty : relyingParty, userHandle : userHandle, acceptedCredentialIDs : acceptedCredentialIDs)
    }
    unsafe fn reportUnusedPasswordCredentialForDomain_userName_(
        &self,
        domain: NSString,
        userName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportUnusedPasswordCredentialForDomain : domain, userName : userName)
    }
    unsafe fn performSavePasswordRequestWithoutUserInteractionIfPossible_(
        &self,
        savePasswordRequest: ASSavePasswordRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSavePasswordRequestWithoutUserInteractionIfPossible : savePasswordRequest)
    }
    unsafe fn prepareInterfaceForSavePasswordRequest_(
        &self,
        savePasswordRequest: ASSavePasswordRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareInterfaceForSavePasswordRequest : savePasswordRequest)
    }
    unsafe fn performGeneratePasswordsRequestWithoutUserInteraction_(
        &self,
        generatePasswordsRequest: ASGeneratePasswordsRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performGeneratePasswordsRequestWithoutUserInteraction : generatePasswordsRequest)
    }
    unsafe fn prepareInterfaceForGeneratePasswordsRequest_(
        &self,
        generatePasswordsRequest: ASGeneratePasswordsRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareInterfaceForGeneratePasswordsRequest : generatePasswordsRequest)
    }
    unsafe fn extensionContext(&self) -> ASCredentialProviderExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionContext)
    }
}
pub trait PASAccountAuthenticationModificationControllerDelegate: Sized + std::ops::Deref {
    unsafe fn accountAuthenticationModificationController_didSuccessfullyCompleteRequest_withUserInfo_(
        &self,
        controller: ASAccountAuthenticationModificationController,
        request: ASAccountAuthenticationModificationRequest,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountAuthenticationModificationController : controller, didSuccessfullyCompleteRequest : request, withUserInfo : userInfo)
    }
    unsafe fn accountAuthenticationModificationController_didFailRequest_withError_(
        &self,
        controller: ASAccountAuthenticationModificationController,
        request: ASAccountAuthenticationModificationRequest,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountAuthenticationModificationController : controller, didFailRequest : request, withError : error)
    }
}
pub trait PASAccountAuthenticationModificationControllerPresentationContextProviding:
    Sized + std::ops::Deref
{
    unsafe fn presentationAnchorForAccountAuthenticationModificationController_(
        &self,
        controller: ASAccountAuthenticationModificationController,
    ) -> ASPresentationAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentationAnchorForAccountAuthenticationModificationController : controller)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccountAuthenticationModificationController(pub id);
impl std::ops::Deref for ASAccountAuthenticationModificationController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAccountAuthenticationModificationController {}
impl ASAccountAuthenticationModificationController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationController").unwrap(), alloc) })
    }
}
impl INSObject for ASAccountAuthenticationModificationController {}
impl PNSObject for ASAccountAuthenticationModificationController {}
impl std::convert::TryFrom<NSObject> for ASAccountAuthenticationModificationController {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAccountAuthenticationModificationController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationController").unwrap())
        };
        if is_kind_of {
            Ok(ASAccountAuthenticationModificationController(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASAccountAuthenticationModificationController" ,)
        }
    }
}
impl IASAccountAuthenticationModificationController
    for ASAccountAuthenticationModificationController
{
}
pub trait IASAccountAuthenticationModificationController: Sized + std::ops::Deref {
    unsafe fn performRequest_(&self, request: ASAccountAuthenticationModificationRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequest : request)
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
    unsafe fn presentationContextProvider(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationContextProvider)
    }
    unsafe fn setPresentationContextProvider_(&self, presentationContextProvider: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresentationContextProvider : presentationContextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccountAuthenticationModificationExtensionContext(pub id);
impl std::ops::Deref for ASAccountAuthenticationModificationExtensionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAccountAuthenticationModificationExtensionContext {}
impl ASAccountAuthenticationModificationExtensionContext {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationExtensionContext").unwrap(), alloc)
        })
    }
}
impl INSExtensionContext for ASAccountAuthenticationModificationExtensionContext {}
impl std::convert::TryFrom<NSExtensionContext>
    for ASAccountAuthenticationModificationExtensionContext
{
    type Error = &'static str;
    fn try_from(
        parent: NSExtensionContext,
    ) -> Result<ASAccountAuthenticationModificationExtensionContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationExtensionContext").unwrap())
        };
        if is_kind_of {
            Ok(ASAccountAuthenticationModificationExtensionContext(
                parent.0,
            ))
        } else {
            Err ("This NSExtensionContext cannot be downcasted to ASAccountAuthenticationModificationExtensionContext" ,)
        }
    }
}
impl INSObject for ASAccountAuthenticationModificationExtensionContext {}
impl PNSObject for ASAccountAuthenticationModificationExtensionContext {}
impl IASAccountAuthenticationModificationExtensionContext
    for ASAccountAuthenticationModificationExtensionContext
{
}
pub trait IASAccountAuthenticationModificationExtensionContext: Sized + std::ops::Deref {
    unsafe fn getSignInWithAppleUpgradeAuthorizationWithState_nonce_completionHandler_(
        &self,
        state: NSString,
        nonce: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSignInWithAppleUpgradeAuthorizationWithState : state, nonce : nonce, completionHandler : completionHandler)
    }
    unsafe fn completeUpgradeToSignInWithAppleWithUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeUpgradeToSignInWithAppleWithUserInfo : userInfo)
    }
    unsafe fn completeChangePasswordRequestWithUpdatedCredential_userInfo_(
        &self,
        updatedCredential: ASPasswordCredential,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeChangePasswordRequestWithUpdatedCredential : updatedCredential, userInfo : userInfo)
    }
    unsafe fn cancelRequestWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelRequestWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccountAuthenticationModificationRequest(pub id);
impl std::ops::Deref for ASAccountAuthenticationModificationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAccountAuthenticationModificationRequest {}
impl ASAccountAuthenticationModificationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationRequest").unwrap(), alloc) })
    }
}
impl INSObject for ASAccountAuthenticationModificationRequest {}
impl PNSObject for ASAccountAuthenticationModificationRequest {}
impl std::convert::TryFrom<NSObject> for ASAccountAuthenticationModificationRequest {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAccountAuthenticationModificationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAccountAuthenticationModificationRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAccountAuthenticationModificationRequest")
        }
    }
}
impl IASAccountAuthenticationModificationRequest for ASAccountAuthenticationModificationRequest {}
pub trait IASAccountAuthenticationModificationRequest: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest(pub id);
impl std::ops::Deref
    for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
{
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message
    for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
{
}
impl ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest").unwrap(), alloc)
        })
    }
}
impl IASAccountAuthenticationModificationRequest
    for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
{
}
impl From<ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest>
    for ASAccountAuthenticationModificationRequest
{
    fn from(
        child: ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest,
    ) -> ASAccountAuthenticationModificationRequest {
        ASAccountAuthenticationModificationRequest(child.0)
    }
}
impl std::convert::TryFrom<ASAccountAuthenticationModificationRequest>
    for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
{
    type Error = &'static str;
    fn try_from(
        parent: ASAccountAuthenticationModificationRequest,
    ) -> Result<
        ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest,
        Self::Error,
    > {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest").unwrap())
        };
        if is_kind_of {
            Ok(
                ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest(
                    parent.0,
                ),
            )
        } else {
            Err ("This ASAccountAuthenticationModificationRequest cannot be downcasted to ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest" ,)
        }
    }
}
impl INSObject for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest {}
impl PNSObject for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest {}
impl IASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
    for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
{
}
pub trait IASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest:
    Sized + std::ops::Deref
{
    unsafe fn initWithUser_serviceIdentifier_userInfo_(
        &self,
        user: NSString,
        serviceIdentifier: ASCredentialServiceIdentifier,
        userInfo: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUser : user, serviceIdentifier : serviceIdentifier, userInfo : userInfo)
    }
    unsafe fn user(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, user)
    }
    unsafe fn serviceIdentifier(&self) -> ASCredentialServiceIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceIdentifier)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest(pub id);
impl std::ops::Deref for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message
    for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest
{
}
impl ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest").unwrap(), alloc)
        })
    }
}
impl IASAccountAuthenticationModificationRequest
    for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest
{
}
impl std::convert::TryFrom<ASAccountAuthenticationModificationRequest>
    for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest
{
    type Error = &'static str;
    fn try_from(
        parent: ASAccountAuthenticationModificationRequest,
    ) -> Result<
        ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest,
        Self::Error,
    > {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest(parent.0))
        } else {
            Err ("This ASAccountAuthenticationModificationRequest cannot be downcasted to ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest" ,)
        }
    }
}
impl INSObject for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {}
impl PNSObject for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {}
impl IASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest
    for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest
{
}
pub trait IASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest:
    Sized + std::ops::Deref
{
    unsafe fn initWithUser_serviceIdentifier_userInfo_(
        &self,
        user: NSString,
        serviceIdentifier: ASCredentialServiceIdentifier,
        userInfo: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUser : user, serviceIdentifier : serviceIdentifier, userInfo : userInfo)
    }
    unsafe fn user(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, user)
    }
    unsafe fn serviceIdentifier(&self) -> ASCredentialServiceIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceIdentifier)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccountAuthenticationModificationViewController(pub id);
impl std::ops::Deref for ASAccountAuthenticationModificationViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAccountAuthenticationModificationViewController {}
impl ASAccountAuthenticationModificationViewController {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationViewController").unwrap(), alloc)
        })
    }
}
impl INSViewController for ASAccountAuthenticationModificationViewController {}
impl PNSEditor for ASAccountAuthenticationModificationViewController {}
impl PNSSeguePerforming for ASAccountAuthenticationModificationViewController {}
impl PNSUserInterfaceItemIdentification for ASAccountAuthenticationModificationViewController {}
impl std::convert::TryFrom<NSViewController> for ASAccountAuthenticationModificationViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSViewController,
    ) -> Result<ASAccountAuthenticationModificationViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccountAuthenticationModificationViewController").unwrap())
        };
        if is_kind_of {
            Ok(ASAccountAuthenticationModificationViewController(parent.0))
        } else {
            Err ("This NSViewController cannot be downcasted to ASAccountAuthenticationModificationViewController" ,)
        }
    }
}
impl INSResponder for ASAccountAuthenticationModificationViewController {}
impl PNSCoding for ASAccountAuthenticationModificationViewController {}
impl INSObject for ASAccountAuthenticationModificationViewController {}
impl PNSObject for ASAccountAuthenticationModificationViewController {}
impl IASAccountAuthenticationModificationViewController
    for ASAccountAuthenticationModificationViewController
{
}
pub trait IASAccountAuthenticationModificationViewController: Sized + std::ops::Deref {
    unsafe fn convertAccountToSignInWithAppleWithoutUserInteractionForServiceIdentifier_existingCredential_userInfo_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        existingCredential: ASPasswordCredential,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertAccountToSignInWithAppleWithoutUserInteractionForServiceIdentifier : serviceIdentifier, existingCredential : existingCredential, userInfo : userInfo)
    }
    unsafe fn prepareInterfaceToConvertAccountToSignInWithAppleForServiceIdentifier_existingCredential_userInfo_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        existingCredential: ASPasswordCredential,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareInterfaceToConvertAccountToSignInWithAppleForServiceIdentifier : serviceIdentifier, existingCredential : existingCredential, userInfo : userInfo)
    }
    unsafe fn changePasswordWithoutUserInteractionForServiceIdentifier_existingCredential_newPassword_userInfo_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        existingCredential: ASPasswordCredential,
        newPassword: NSString,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changePasswordWithoutUserInteractionForServiceIdentifier : serviceIdentifier, existingCredential : existingCredential, newPassword : newPassword, userInfo : userInfo)
    }
    unsafe fn prepareInterfaceToChangePasswordForServiceIdentifier_existingCredential_newPassword_userInfo_(
        &self,
        serviceIdentifier: ASCredentialServiceIdentifier,
        existingCredential: ASPasswordCredential,
        newPassword: NSString,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareInterfaceToChangePasswordForServiceIdentifier : serviceIdentifier, existingCredential : existingCredential, newPassword : newPassword, userInfo : userInfo)
    }
    unsafe fn cancelRequest(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelRequest)
    }
    unsafe fn extensionContext(&self) -> ASAccountAuthenticationModificationExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionContext)
    }
}
pub type ASAuthorizationAppleIDButtonType = NSInteger;
pub type ASAuthorizationAppleIDButtonStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationAppleIDButton(pub id);
impl std::ops::Deref for ASAuthorizationAppleIDButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationAppleIDButton {}
impl ASAuthorizationAppleIDButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDButton").unwrap(), alloc) })
    }
}
impl PNSAccessibilityButton for ASAuthorizationAppleIDButton {}
impl INSControl for ASAuthorizationAppleIDButton {}
impl std::convert::TryFrom<NSControl> for ASAuthorizationAppleIDButton {
    type Error = &'static str;
    fn try_from(parent: NSControl) -> Result<ASAuthorizationAppleIDButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDButton").unwrap()) };
        if is_kind_of {
            Ok(ASAuthorizationAppleIDButton(parent.0))
        } else {
            Err("This NSControl cannot be downcasted to ASAuthorizationAppleIDButton")
        }
    }
}
impl INSView for ASAuthorizationAppleIDButton {}
impl PNSAnimatablePropertyContainer for ASAuthorizationAppleIDButton {}
impl PNSUserInterfaceItemIdentification for ASAuthorizationAppleIDButton {}
impl PNSDraggingDestination for ASAuthorizationAppleIDButton {}
impl PNSAppearanceCustomization for ASAuthorizationAppleIDButton {}
impl PNSAccessibilityElement for ASAuthorizationAppleIDButton {}
impl PNSAccessibility for ASAuthorizationAppleIDButton {}
impl INSResponder for ASAuthorizationAppleIDButton {}
impl PNSCoding for ASAuthorizationAppleIDButton {}
impl INSObject for ASAuthorizationAppleIDButton {}
impl PNSObject for ASAuthorizationAppleIDButton {}
impl IASAuthorizationAppleIDButton for ASAuthorizationAppleIDButton {}
pub trait IASAuthorizationAppleIDButton: Sized + std::ops::Deref {
    unsafe fn initWithAuthorizationButtonType_authorizationButtonStyle_(
        &self,
        type_: ASAuthorizationAppleIDButtonType,
        style: ASAuthorizationAppleIDButtonStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAuthorizationButtonType : type_, authorizationButtonStyle : style)
    }
    unsafe fn cornerRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cornerRadius)
    }
    unsafe fn setCornerRadius_(&self, cornerRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCornerRadius : cornerRadius)
    }
    unsafe fn buttonWithType_style_(
        type_: ASAuthorizationAppleIDButtonType,
        style: ASAuthorizationAppleIDButtonStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationAppleIDButton").unwrap(), buttonWithType : type_, style : style)
    }
}
pub type ASCOSEAlgorithmIdentifier = NSInteger;
pub trait PASPublicKeyCredential: Sized + std::ops::Deref {
    unsafe fn rawClientDataJSON(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawClientDataJSON)
    }
    unsafe fn credentialID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialID)
    }
}
pub trait PASAuthorizationPublicKeyCredentialAssertion: Sized + std::ops::Deref {
    unsafe fn rawAuthenticatorData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawAuthenticatorData)
    }
    unsafe fn userID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userID)
    }
    unsafe fn signature(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signature)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput {}
impl ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput {}
impl PNSObject for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput {}
impl std::convert::TryFrom<NSObject>
    for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput
{
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput
    for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput
{
}
pub trait IASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput:
    Sized + std::ops::Deref
{
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn readData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readData)
    }
    unsafe fn didWrite(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didWrite)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPlatformPublicKeyCredentialAssertion(pub id);
impl std::ops::Deref for ASAuthorizationPlatformPublicKeyCredentialAssertion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPlatformPublicKeyCredentialAssertion {}
impl ASAuthorizationPlatformPublicKeyCredentialAssertion {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialAssertion").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialAssertion
    for ASAuthorizationPlatformPublicKeyCredentialAssertion
{
}
impl INSObject for ASAuthorizationPlatformPublicKeyCredentialAssertion {}
impl PNSObject for ASAuthorizationPlatformPublicKeyCredentialAssertion {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPlatformPublicKeyCredentialAssertion {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPlatformPublicKeyCredentialAssertion, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialAssertion").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPlatformPublicKeyCredentialAssertion(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPlatformPublicKeyCredentialAssertion" ,)
        }
    }
}
impl IASAuthorizationPlatformPublicKeyCredentialAssertion
    for ASAuthorizationPlatformPublicKeyCredentialAssertion
{
}
pub trait IASAuthorizationPlatformPublicKeyCredentialAssertion: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn attachment(&self) -> ASAuthorizationPublicKeyCredentialAttachment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachment)
    }
    unsafe fn largeBlob(&self) -> ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeBlob)
    }
    unsafe fn prf(&self) -> ASAuthorizationPublicKeyCredentialPRFAssertionOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prf)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialAssertion").unwrap(), new)
    }
}
pub trait PASAuthorizationPublicKeyCredentialDescriptor: Sized + std::ops::Deref {
    unsafe fn credentialID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialID)
    }
    unsafe fn setCredentialID_(&self, credentialID: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCredentialID : credentialID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPlatformPublicKeyCredentialDescriptor(pub id);
impl std::ops::Deref for ASAuthorizationPlatformPublicKeyCredentialDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}
impl ASAuthorizationPlatformPublicKeyCredentialDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialDescriptor").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialDescriptor
    for ASAuthorizationPlatformPublicKeyCredentialDescriptor
{
}
impl INSObject for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}
impl PNSObject for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPlatformPublicKeyCredentialDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPlatformPublicKeyCredentialDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPlatformPublicKeyCredentialDescriptor(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPlatformPublicKeyCredentialDescriptor" ,)
        }
    }
}
impl IASAuthorizationPlatformPublicKeyCredentialDescriptor
    for ASAuthorizationPlatformPublicKeyCredentialDescriptor
{
}
pub trait IASAuthorizationPlatformPublicKeyCredentialDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithCredentialID_(&self, credentialID: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCredentialID : credentialID)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialDescriptor").unwrap(), new)
    }
}
pub trait PASAuthorizationPublicKeyCredentialAssertionRequest: Sized + std::ops::Deref {
    unsafe fn challenge(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, challenge)
    }
    unsafe fn setChallenge_(&self, challenge: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChallenge : challenge)
    }
    unsafe fn relyingPartyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingPartyIdentifier)
    }
    unsafe fn setRelyingPartyIdentifier_(&self, relyingPartyIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelyingPartyIdentifier : relyingPartyIdentifier)
    }
    unsafe fn allowedCredentials(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedCredentials)
    }
    unsafe fn setAllowedCredentials_(&self, allowedCredentials: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedCredentials : allowedCredentials)
    }
    unsafe fn userVerificationPreference(
        &self,
    ) -> ASAuthorizationPublicKeyCredentialUserVerificationPreference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userVerificationPreference)
    }
    unsafe fn setUserVerificationPreference_(&self, userVerificationPreference: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserVerificationPreference : userVerificationPreference)
    }
}
pub type ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {}
impl ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {}
impl PNSObject for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialLargeBlobAssertionInput
    for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput
{
}
pub trait IASAuthorizationPublicKeyCredentialLargeBlobAssertionInput:
    Sized + std::ops::Deref
{
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithOperation_(
        &self,
        operation: ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOperation : operation)
    }
    unsafe fn operation(&self) -> ASAuthorizationPublicKeyCredentialLargeBlobAssertionOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operation)
    }
    unsafe fn dataToWrite(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataToWrite)
    }
    unsafe fn setDataToWrite_(&self, dataToWrite: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataToWrite : dataToWrite)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput").unwrap(), new)
    }
}
pub trait PASAuthorizationWebBrowserExternallyAuthenticatableRequest:
    Sized + std::ops::Deref
{
    unsafe fn authenticatedContext(&self) -> LAContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticatedContext)
    }
    unsafe fn setAuthenticatedContext_(&self, authenticatedContext: LAContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthenticatedContext : authenticatedContext)
    }
}
pub trait PASAuthorizationWebBrowserPlatformPublicKeyCredentialAssertionRequest:
    Sized + std::ops::Deref
{
    unsafe fn clientData(&self) -> ASPublicKeyCredentialClientData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientData)
    }
    unsafe fn shouldShowHybridTransport(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldShowHybridTransport)
    }
    unsafe fn setShouldShowHybridTransport_(&self, shouldShowHybridTransport: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldShowHybridTransport : shouldShowHybridTransport)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPlatformPublicKeyCredentialAssertionRequest(pub id);
impl std::ops::Deref for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}
impl ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialAssertionRequest").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialAssertionRequest
    for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
{
}
impl IASAuthorizationRequest for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}
impl PNSCopying for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}
impl PNSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}
impl std::convert::TryFrom<ASAuthorizationRequest>
    for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
{
    type Error = &'static str;
    fn try_from(
        parent: ASAuthorizationRequest,
    ) -> Result<ASAuthorizationPlatformPublicKeyCredentialAssertionRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialAssertionRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPlatformPublicKeyCredentialAssertionRequest(
                parent.0,
            ))
        } else {
            Err ("This ASAuthorizationRequest cannot be downcasted to ASAuthorizationPlatformPublicKeyCredentialAssertionRequest" ,)
        }
    }
}
impl INSObject for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}
impl PNSObject for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}
impl IASAuthorizationPlatformPublicKeyCredentialAssertionRequest
    for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
{
}
pub trait IASAuthorizationPlatformPublicKeyCredentialAssertionRequest:
    Sized + std::ops::Deref
{
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn allowedCredentials(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedCredentials)
    }
    unsafe fn setAllowedCredentials_(&self, allowedCredentials: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedCredentials : allowedCredentials)
    }
    unsafe fn largeBlob(&self) -> ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeBlob)
    }
    unsafe fn setLargeBlob_(
        &self,
        largeBlob: ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLargeBlob : largeBlob)
    }
    unsafe fn prf(&self) -> ASAuthorizationPublicKeyCredentialPRFAssertionInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prf)
    }
    unsafe fn setPrf_(&self, prf: ASAuthorizationPublicKeyCredentialPRFAssertionInput)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrf : prf)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialAssertionRequest").unwrap(), new)
    }
}
impl ASAuthorizationPlatformPublicKeyCredentialAssertionRequest_
    for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
{
}
pub trait ASAuthorizationPlatformPublicKeyCredentialAssertionRequest_:
    Sized + std::ops::Deref
{
}
pub type ASAuthorizationPublicKeyCredentialLargeBlobSupportRequirement = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput {}
impl ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput {}
impl PNSObject for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput {}
impl std::convert::TryFrom<NSObject>
    for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput
{
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput
    for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput
{
}
pub trait IASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput:
    Sized + std::ops::Deref
{
    unsafe fn initWithSupportRequirement_(
        &self,
        requirement: ASAuthorizationPublicKeyCredentialLargeBlobSupportRequirement,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSupportRequirement : requirement)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn supportRequirement(
        &self,
    ) -> ASAuthorizationPublicKeyCredentialLargeBlobSupportRequirement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportRequirement)
    }
    unsafe fn setSupportRequirement_(
        &self,
        supportRequirement: ASAuthorizationPublicKeyCredentialLargeBlobSupportRequirement,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportRequirement : supportRequirement)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput").unwrap(), new)
    }
}
pub trait PASAuthorizationWebBrowserPlatformPublicKeyCredentialRegistrationRequest:
    Sized + std::ops::Deref
{
    unsafe fn clientData(&self) -> ASPublicKeyCredentialClientData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientData)
    }
    unsafe fn excludedCredentials(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedCredentials)
    }
    unsafe fn setExcludedCredentials_(&self, excludedCredentials: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedCredentials : excludedCredentials)
    }
    unsafe fn shouldShowHybridTransport(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldShowHybridTransport)
    }
    unsafe fn setShouldShowHybridTransport_(&self, shouldShowHybridTransport: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldShowHybridTransport : shouldShowHybridTransport)
    }
}
pub type ASAuthorizationPlatformPublicKeyCredentialRegistrationRequestStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest(pub id);
impl std::ops::Deref for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}
impl ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialRegistrationRequest
    for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
{
}
impl IASAuthorizationRequest for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}
impl PNSCopying for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}
impl PNSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}
impl std::convert::TryFrom<ASAuthorizationRequest>
    for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
{
    type Error = &'static str;
    fn try_from(
        parent: ASAuthorizationRequest,
    ) -> Result<ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest(parent.0))
        } else {
            Err ("This ASAuthorizationRequest cannot be downcasted to ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest" ,)
        }
    }
}
impl INSObject for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}
impl PNSObject for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {}
impl IASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
    for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
{
}
pub trait IASAuthorizationPlatformPublicKeyCredentialRegistrationRequest:
    Sized + std::ops::Deref
{
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn largeBlob(&self) -> ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeBlob)
    }
    unsafe fn setLargeBlob_(
        &self,
        largeBlob: ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLargeBlob : largeBlob)
    }
    unsafe fn prf(&self) -> ASAuthorizationPublicKeyCredentialPRFRegistrationInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prf)
    }
    unsafe fn setPrf_(&self, prf: ASAuthorizationPublicKeyCredentialPRFRegistrationInput)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrf : prf)
    }
    unsafe fn requestStyle(
        &self,
    ) -> ASAuthorizationPlatformPublicKeyCredentialRegistrationRequestStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestStyle)
    }
    unsafe fn setRequestStyle_(
        &self,
        requestStyle: ASAuthorizationPlatformPublicKeyCredentialRegistrationRequestStyle,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequestStyle : requestStyle)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest").unwrap(), new)
    }
}
impl ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest_
    for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
{
}
pub trait ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest_:
    Sized + std::ops::Deref
{
}
pub trait PASAuthorizationWebBrowserPlatformPublicKeyCredentialProvider:
    Sized + std::ops::Deref
{
    unsafe fn createCredentialRegistrationRequestWithClientData_name_userID_(
        &self,
        clientData: ASPublicKeyCredentialClientData,
        name: NSString,
        userID: NSData,
    ) -> ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialRegistrationRequestWithClientData : clientData, name : name, userID : userID)
    }
    unsafe fn createCredentialRegistrationRequestWithClientData_name_userID_requestStyle_(
        &self,
        clientData: ASPublicKeyCredentialClientData,
        name: NSString,
        userID: NSData,
        requestStyle: ASAuthorizationPlatformPublicKeyCredentialRegistrationRequestStyle,
    ) -> ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialRegistrationRequestWithClientData : clientData, name : name, userID : userID, requestStyle : requestStyle)
    }
    unsafe fn createCredentialAssertionRequestWithClientData_(
        &self,
        clientData: ASPublicKeyCredentialClientData,
    ) -> ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialAssertionRequestWithClientData : clientData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPlatformPublicKeyCredentialProvider(pub id);
impl std::ops::Deref for ASAuthorizationPlatformPublicKeyCredentialProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPlatformPublicKeyCredentialProvider {}
impl ASAuthorizationPlatformPublicKeyCredentialProvider {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialProvider").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationProvider for ASAuthorizationPlatformPublicKeyCredentialProvider {}
impl INSObject for ASAuthorizationPlatformPublicKeyCredentialProvider {}
impl PNSObject for ASAuthorizationPlatformPublicKeyCredentialProvider {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPlatformPublicKeyCredentialProvider {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPlatformPublicKeyCredentialProvider, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialProvider").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPlatformPublicKeyCredentialProvider(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPlatformPublicKeyCredentialProvider" ,)
        }
    }
}
impl IASAuthorizationPlatformPublicKeyCredentialProvider
    for ASAuthorizationPlatformPublicKeyCredentialProvider
{
}
pub trait IASAuthorizationPlatformPublicKeyCredentialProvider: Sized + std::ops::Deref {
    unsafe fn initWithRelyingPartyIdentifier_(
        &self,
        relyingPartyIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRelyingPartyIdentifier : relyingPartyIdentifier)
    }
    unsafe fn createCredentialRegistrationRequestWithChallenge_name_userID_(
        &self,
        challenge: NSData,
        name: NSString,
        userID: NSData,
    ) -> ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialRegistrationRequestWithChallenge : challenge, name : name, userID : userID)
    }
    unsafe fn createCredentialRegistrationRequestWithChallenge_name_userID_requestStyle_(
        &self,
        challenge: NSData,
        name: NSString,
        userID: NSData,
        requestStyle: ASAuthorizationPlatformPublicKeyCredentialRegistrationRequestStyle,
    ) -> ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialRegistrationRequestWithChallenge : challenge, name : name, userID : userID, requestStyle : requestStyle)
    }
    unsafe fn createCredentialAssertionRequestWithChallenge_(
        &self,
        challenge: NSData,
    ) -> ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialAssertionRequestWithChallenge : challenge)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn relyingPartyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingPartyIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialProvider").unwrap(), new)
    }
}
impl ASAuthorizationPlatformPublicKeyCredentialProvider_
    for ASAuthorizationPlatformPublicKeyCredentialProvider
{
}
pub trait ASAuthorizationPlatformPublicKeyCredentialProvider_: Sized + std::ops::Deref {}
pub trait PASAuthorizationPublicKeyCredentialRegistration: Sized + std::ops::Deref {
    unsafe fn rawAttestationObject(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawAttestationObject)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}
impl ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput").unwrap(), alloc)
        })
    }
}
impl PNSCopying for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}
impl PNSSecureCoding for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}
impl INSObject for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}
impl PNSObject for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}
impl std::convert::TryFrom<NSObject>
    for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput
{
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput
    for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput
{
}
pub trait IASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput:
    Sized + std::ops::Deref
{
    unsafe fn isSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPlatformPublicKeyCredentialRegistration(pub id);
impl std::ops::Deref for ASAuthorizationPlatformPublicKeyCredentialRegistration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPlatformPublicKeyCredentialRegistration {}
impl ASAuthorizationPlatformPublicKeyCredentialRegistration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialRegistration").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialRegistration
    for ASAuthorizationPlatformPublicKeyCredentialRegistration
{
}
impl INSObject for ASAuthorizationPlatformPublicKeyCredentialRegistration {}
impl PNSObject for ASAuthorizationPlatformPublicKeyCredentialRegistration {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPlatformPublicKeyCredentialRegistration {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPlatformPublicKeyCredentialRegistration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialRegistration").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPlatformPublicKeyCredentialRegistration(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPlatformPublicKeyCredentialRegistration" ,)
        }
    }
}
impl IASAuthorizationPlatformPublicKeyCredentialRegistration
    for ASAuthorizationPlatformPublicKeyCredentialRegistration
{
}
pub trait IASAuthorizationPlatformPublicKeyCredentialRegistration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn attachment(&self) -> ASAuthorizationPublicKeyCredentialAttachment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachment)
    }
    unsafe fn largeBlob(&self) -> ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeBlob)
    }
    unsafe fn prf(&self) -> ASAuthorizationPublicKeyCredentialPRFRegistrationOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prf)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPlatformPublicKeyCredentialRegistration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialParameters(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialParameters {}
impl ASAuthorizationPublicKeyCredentialParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialParameters").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ASAuthorizationPublicKeyCredentialParameters {}
impl PNSCopying for ASAuthorizationPublicKeyCredentialParameters {}
impl INSObject for ASAuthorizationPublicKeyCredentialParameters {}
impl PNSObject for ASAuthorizationPublicKeyCredentialParameters {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPublicKeyCredentialParameters {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialParameters").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialParameters(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialParameters" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialParameters
    for ASAuthorizationPublicKeyCredentialParameters
{
}
pub trait IASAuthorizationPublicKeyCredentialParameters: Sized + std::ops::Deref {
    unsafe fn initWithAlgorithm_(&self, algorithm: ASCOSEAlgorithmIdentifier) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAlgorithm : algorithm)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn algorithm(&self) -> ASCOSEAlgorithmIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, algorithm)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertion(pub id);
impl std::ops::Deref for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}
impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialAssertion").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialAssertion
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion
{
}
impl INSObject for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}
impl PNSObject for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationSecurityKeyPublicKeyCredentialAssertion, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialAssertion").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSecurityKeyPublicKeyCredentialAssertion(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationSecurityKeyPublicKeyCredentialAssertion" ,)
        }
    }
}
impl IASAuthorizationSecurityKeyPublicKeyCredentialAssertion
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion
{
}
pub trait IASAuthorizationSecurityKeyPublicKeyCredentialAssertion: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn appID(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appID)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialAssertion").unwrap(), new)
    }
}
pub type ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor(pub id);
impl std::ops::Deref for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {}
impl ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialDescriptor
    for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor
{
}
impl INSObject for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {}
impl PNSObject for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor" ,)
        }
    }
}
impl IASAuthorizationSecurityKeyPublicKeyCredentialDescriptor
    for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor
{
}
pub trait IASAuthorizationSecurityKeyPublicKeyCredentialDescriptor:
    Sized + std::ops::Deref
{
    unsafe fn initWithCredentialID_transports_(
        &self,
        credentialID: NSData,
        allowedTransports: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCredentialID : credentialID, transports : allowedTransports)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn transports(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transports)
    }
    unsafe fn setTransports_(&self, transports: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransports : transports)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor").unwrap(), new)
    }
}
pub trait PASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialAssertionRequest:
    Sized + std::ops::Deref
{
    unsafe fn clientData(&self) -> ASPublicKeyCredentialClientData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest(pub id);
impl std::ops::Deref for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}
impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialAssertionRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
}
impl IASAuthorizationRequest for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}
impl PNSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}
impl PNSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}
impl std::convert::TryFrom<ASAuthorizationRequest>
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
    type Error = &'static str;
    fn try_from(
        parent: ASAuthorizationRequest,
    ) -> Result<ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest(parent.0))
        } else {
            Err ("This ASAuthorizationRequest cannot be downcasted to ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest" ,)
        }
    }
}
impl INSObject for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}
impl PNSObject for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}
impl IASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
}
pub trait IASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest:
    Sized + std::ops::Deref
{
    unsafe fn allowedCredentials(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedCredentials)
    }
    unsafe fn setAllowedCredentials_(&self, allowedCredentials: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedCredentials : allowedCredentials)
    }
    unsafe fn appID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appID)
    }
    unsafe fn setAppID_(&self, appID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppID : appID)
    }
}
impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest_
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
}
pub trait ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest_:
    Sized + std::ops::Deref
{
}
pub trait PASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialProvider:
    Sized + std::ops::Deref
{
    unsafe fn createCredentialRegistrationRequestWithClientData_displayName_name_userID_(
        &self,
        clientData: ASPublicKeyCredentialClientData,
        displayName: NSString,
        name: NSString,
        userID: NSData,
    ) -> ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialRegistrationRequestWithClientData : clientData, displayName : displayName, name : name, userID : userID)
    }
    unsafe fn createCredentialAssertionRequestWithClientData_(
        &self,
        clientData: ASPublicKeyCredentialClientData,
    ) -> ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialAssertionRequestWithClientData : clientData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSecurityKeyPublicKeyCredentialProvider(pub id);
impl std::ops::Deref for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {}
impl ASAuthorizationSecurityKeyPublicKeyCredentialProvider {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialProvider").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationProvider for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {}
impl INSObject for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {}
impl PNSObject for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationSecurityKeyPublicKeyCredentialProvider, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialProvider").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSecurityKeyPublicKeyCredentialProvider(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationSecurityKeyPublicKeyCredentialProvider" ,)
        }
    }
}
impl IASAuthorizationSecurityKeyPublicKeyCredentialProvider
    for ASAuthorizationSecurityKeyPublicKeyCredentialProvider
{
}
pub trait IASAuthorizationSecurityKeyPublicKeyCredentialProvider: Sized + std::ops::Deref {
    unsafe fn initWithRelyingPartyIdentifier_(
        &self,
        relyingPartyIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRelyingPartyIdentifier : relyingPartyIdentifier)
    }
    unsafe fn createCredentialRegistrationRequestWithChallenge_displayName_name_userID_(
        &self,
        challenge: NSData,
        displayName: NSString,
        name: NSString,
        userID: NSData,
    ) -> ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialRegistrationRequestWithChallenge : challenge, displayName : displayName, name : name, userID : userID)
    }
    unsafe fn createCredentialAssertionRequestWithChallenge_(
        &self,
        challenge: NSData,
    ) -> ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCredentialAssertionRequestWithChallenge : challenge)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn relyingPartyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingPartyIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialProvider").unwrap(), new)
    }
}
impl ASAuthorizationSecurityKeyPublicKeyCredentialProvider_
    for ASAuthorizationSecurityKeyPublicKeyCredentialProvider
{
}
pub trait ASAuthorizationSecurityKeyPublicKeyCredentialProvider_: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSecurityKeyPublicKeyCredentialRegistration(pub id);
impl std::ops::Deref for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}
impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialRegistration").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialRegistration
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration
{
}
impl INSObject for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}
impl PNSObject for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationSecurityKeyPublicKeyCredentialRegistration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialRegistration").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSecurityKeyPublicKeyCredentialRegistration(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationSecurityKeyPublicKeyCredentialRegistration" ,)
        }
    }
}
impl IASAuthorizationSecurityKeyPublicKeyCredentialRegistration
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration
{
}
pub trait IASAuthorizationSecurityKeyPublicKeyCredentialRegistration:
    Sized + std::ops::Deref
{
    unsafe fn transports(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transports)
    }
}
pub trait PASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialRegistrationRequest:
    Sized + std::ops::Deref
{
    unsafe fn clientData(&self) -> ASPublicKeyCredentialClientData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest(pub id);
impl std::ops::Deref for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest").unwrap(), alloc)
        })
    }
}
impl PASAuthorizationPublicKeyCredentialRegistrationRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
{
}
impl IASAuthorizationRequest for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
impl PNSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
impl PNSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
impl std::convert::TryFrom<ASAuthorizationRequest>
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
{
    type Error = &'static str;
    fn try_from(
        parent: ASAuthorizationRequest,
    ) -> Result<ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest(parent.0))
        } else {
            Err ("This ASAuthorizationRequest cannot be downcasted to ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest" ,)
        }
    }
}
impl INSObject for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
impl PNSObject for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
impl IASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
{
}
pub trait IASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest:
    Sized + std::ops::Deref
{
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn credentialParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialParameters)
    }
    unsafe fn setCredentialParameters_(&self, credentialParameters: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCredentialParameters : credentialParameters)
    }
    unsafe fn excludedCredentials(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedCredentials)
    }
    unsafe fn setExcludedCredentials_(&self, excludedCredentials: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedCredentials : excludedCredentials)
    }
    unsafe fn residentKeyPreference(
        &self,
    ) -> ASAuthorizationPublicKeyCredentialResidentKeyPreference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, residentKeyPreference)
    }
    unsafe fn setResidentKeyPreference_(&self, residentKeyPreference: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResidentKeyPreference : residentKeyPreference)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest").unwrap(), new)
    }
}
impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest_
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
{
}
pub trait ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest_:
    Sized + std::ops::Deref
{
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationWebBrowserPlatformPublicKeyCredential(pub id);
impl std::ops::Deref for ASAuthorizationWebBrowserPlatformPublicKeyCredential {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationWebBrowserPlatformPublicKeyCredential {}
impl ASAuthorizationWebBrowserPlatformPublicKeyCredential {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationWebBrowserPlatformPublicKeyCredential").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationWebBrowserPlatformPublicKeyCredential {}
impl PNSObject for ASAuthorizationWebBrowserPlatformPublicKeyCredential {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationWebBrowserPlatformPublicKeyCredential {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationWebBrowserPlatformPublicKeyCredential, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationWebBrowserPlatformPublicKeyCredential").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationWebBrowserPlatformPublicKeyCredential(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationWebBrowserPlatformPublicKeyCredential" ,)
        }
    }
}
impl IASAuthorizationWebBrowserPlatformPublicKeyCredential
    for ASAuthorizationWebBrowserPlatformPublicKeyCredential
{
}
pub trait IASAuthorizationWebBrowserPlatformPublicKeyCredential: Sized + std::ops::Deref {
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
    unsafe fn customTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customTitle)
    }
    unsafe fn relyingParty(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingParty)
    }
    unsafe fn credentialID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialID)
    }
    unsafe fn userHandle(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userHandle)
    }
    unsafe fn providerName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerName)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationWebBrowserPlatformPublicKeyCredential").unwrap(), new)
    }
}
pub type ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationWebBrowserPublicKeyCredentialManager(pub id);
impl std::ops::Deref for ASAuthorizationWebBrowserPublicKeyCredentialManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationWebBrowserPublicKeyCredentialManager {}
impl ASAuthorizationWebBrowserPublicKeyCredentialManager {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationWebBrowserPublicKeyCredentialManager").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationWebBrowserPublicKeyCredentialManager {}
impl PNSObject for ASAuthorizationWebBrowserPublicKeyCredentialManager {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationWebBrowserPublicKeyCredentialManager {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationWebBrowserPublicKeyCredentialManager, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationWebBrowserPublicKeyCredentialManager").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationWebBrowserPublicKeyCredentialManager(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationWebBrowserPublicKeyCredentialManager" ,)
        }
    }
}
impl IASAuthorizationWebBrowserPublicKeyCredentialManager
    for ASAuthorizationWebBrowserPublicKeyCredentialManager
{
}
pub trait IASAuthorizationWebBrowserPublicKeyCredentialManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn requestAuthorizationForPublicKeyCredentials_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAuthorizationForPublicKeyCredentials : completionHandler)
    }
    unsafe fn platformCredentialsForRelyingParty_completionHandler_(
        &self,
        relyingParty: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, platformCredentialsForRelyingParty : relyingParty, completionHandler : completionHandler)
    }
    unsafe fn authorizationStateForPlatformCredentials(
        &self,
    ) -> ASAuthorizationWebBrowserPublicKeyCredentialManagerAuthorizationState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationStateForPlatformCredentials)
    }
    unsafe fn isDeviceConfiguredForPasskeys() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationWebBrowserPublicKeyCredentialManager").unwrap(), isDeviceConfiguredForPasskeys)
    }
}
pub type ASPublicKeyCredentialClientDataCrossOriginValue = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPublicKeyCredentialClientData(pub id);
impl std::ops::Deref for ASPublicKeyCredentialClientData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPublicKeyCredentialClientData {}
impl ASPublicKeyCredentialClientData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPublicKeyCredentialClientData").unwrap(), alloc) })
    }
}
impl INSObject for ASPublicKeyCredentialClientData {}
impl PNSObject for ASPublicKeyCredentialClientData {}
impl std::convert::TryFrom<NSObject> for ASPublicKeyCredentialClientData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPublicKeyCredentialClientData, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPublicKeyCredentialClientData").unwrap())
        };
        if is_kind_of {
            Ok(ASPublicKeyCredentialClientData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPublicKeyCredentialClientData")
        }
    }
}
impl IASPublicKeyCredentialClientData for ASPublicKeyCredentialClientData {}
pub trait IASPublicKeyCredentialClientData: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithChallenge_origin_(&self, challenge: NSData, origin: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChallenge : challenge, origin : origin)
    }
    unsafe fn challenge(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, challenge)
    }
    unsafe fn setChallenge_(&self, challenge: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChallenge : challenge)
    }
    unsafe fn origin(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, origin)
    }
    unsafe fn setOrigin_(&self, origin: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrigin : origin)
    }
    unsafe fn topOrigin(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topOrigin)
    }
    unsafe fn setTopOrigin_(&self, topOrigin: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopOrigin : topOrigin)
    }
    unsafe fn crossOrigin(&self) -> ASPublicKeyCredentialClientDataCrossOriginValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossOrigin)
    }
    unsafe fn setCrossOrigin_(&self, crossOrigin: ASPublicKeyCredentialClientDataCrossOriginValue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossOrigin : crossOrigin)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPublicKeyCredentialClientData").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASOneTimeCodeCredential(pub id);
impl std::ops::Deref for ASOneTimeCodeCredential {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASOneTimeCodeCredential {}
impl ASOneTimeCodeCredential {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASOneTimeCodeCredential").unwrap(), alloc) })
    }
}
impl PASAuthorizationCredential for ASOneTimeCodeCredential {}
impl INSObject for ASOneTimeCodeCredential {}
impl PNSObject for ASOneTimeCodeCredential {}
impl std::convert::TryFrom<NSObject> for ASOneTimeCodeCredential {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASOneTimeCodeCredential, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASOneTimeCodeCredential").unwrap()) };
        if is_kind_of {
            Ok(ASOneTimeCodeCredential(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASOneTimeCodeCredential")
        }
    }
}
impl IASOneTimeCodeCredential for ASOneTimeCodeCredential {}
pub trait IASOneTimeCodeCredential: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCode_(&self, code: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCode : code)
    }
    unsafe fn code(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, code)
    }
    unsafe fn credentialWithCode_(code: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASOneTimeCodeCredential").unwrap(), credentialWithCode : code)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASOneTimeCodeCredentialRequest(pub id);
impl std::ops::Deref for ASOneTimeCodeCredentialRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASOneTimeCodeCredentialRequest {}
impl ASOneTimeCodeCredentialRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASOneTimeCodeCredentialRequest").unwrap(), alloc) })
    }
}
impl PASCredentialRequest for ASOneTimeCodeCredentialRequest {}
impl INSObject for ASOneTimeCodeCredentialRequest {}
impl PNSObject for ASOneTimeCodeCredentialRequest {}
impl std::convert::TryFrom<NSObject> for ASOneTimeCodeCredentialRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASOneTimeCodeCredentialRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASOneTimeCodeCredentialRequest").unwrap())
        };
        if is_kind_of {
            Ok(ASOneTimeCodeCredentialRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASOneTimeCodeCredentialRequest")
        }
    }
}
impl IASOneTimeCodeCredentialRequest for ASOneTimeCodeCredentialRequest {}
pub trait IASOneTimeCodeCredentialRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCredentialIdentity_(
        &self,
        credentialIdentity: ASOneTimeCodeCredentialIdentity,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCredentialIdentity : credentialIdentity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationProviderExtensionKerberosMapping(pub id);
impl std::ops::Deref for ASAuthorizationProviderExtensionKerberosMapping {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationProviderExtensionKerberosMapping {}
impl ASAuthorizationProviderExtensionKerberosMapping {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionKerberosMapping").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationProviderExtensionKerberosMapping {}
impl PNSObject for ASAuthorizationProviderExtensionKerberosMapping {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationProviderExtensionKerberosMapping {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationProviderExtensionKerberosMapping, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionKerberosMapping").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationProviderExtensionKerberosMapping(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationProviderExtensionKerberosMapping" ,)
        }
    }
}
impl IASAuthorizationProviderExtensionKerberosMapping
    for ASAuthorizationProviderExtensionKerberosMapping
{
}
pub trait IASAuthorizationProviderExtensionKerberosMapping: Sized + std::ops::Deref {
    unsafe fn ticketKeyPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ticketKeyPath)
    }
    unsafe fn setTicketKeyPath_(&self, ticketKeyPath: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTicketKeyPath : ticketKeyPath)
    }
    unsafe fn messageBufferKeyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageBufferKeyName)
    }
    unsafe fn setMessageBufferKeyName_(&self, messageBufferKeyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessageBufferKeyName : messageBufferKeyName)
    }
    unsafe fn realmKeyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, realmKeyName)
    }
    unsafe fn setRealmKeyName_(&self, realmKeyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRealmKeyName : realmKeyName)
    }
    unsafe fn serviceNameKeyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceNameKeyName)
    }
    unsafe fn setServiceNameKeyName_(&self, serviceNameKeyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServiceNameKeyName : serviceNameKeyName)
    }
    unsafe fn clientNameKeyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientNameKeyName)
    }
    unsafe fn setClientNameKeyName_(&self, clientNameKeyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClientNameKeyName : clientNameKeyName)
    }
    unsafe fn encryptionKeyTypeKeyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptionKeyTypeKeyName)
    }
    unsafe fn setEncryptionKeyTypeKeyName_(&self, encryptionKeyTypeKeyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEncryptionKeyTypeKeyName : encryptionKeyTypeKeyName)
    }
    unsafe fn sessionKeyKeyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionKeyKeyName)
    }
    unsafe fn setSessionKeyKeyName_(&self, sessionKeyKeyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSessionKeyKeyName : sessionKeyKeyName)
    }
}
pub type ASAuthorizationProviderExtensionFederationType = NSInteger;
pub type ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy = NSUInteger;
pub type ASAuthorizationProviderExtensionEncryptionAlgorithm = NSNumber;
pub type ASAuthorizationProviderExtensionSigningAlgorithm = NSNumber;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationProviderExtensionLoginConfiguration(pub id);
impl std::ops::Deref for ASAuthorizationProviderExtensionLoginConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationProviderExtensionLoginConfiguration {}
impl ASAuthorizationProviderExtensionLoginConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionLoginConfiguration").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationProviderExtensionLoginConfiguration {}
impl PNSObject for ASAuthorizationProviderExtensionLoginConfiguration {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationProviderExtensionLoginConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationProviderExtensionLoginConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionLoginConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationProviderExtensionLoginConfiguration(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationProviderExtensionLoginConfiguration" ,)
        }
    }
}
impl IASAuthorizationProviderExtensionLoginConfiguration
    for ASAuthorizationProviderExtensionLoginConfiguration
{
}
pub trait IASAuthorizationProviderExtensionLoginConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithClientID_issuer_tokenEndpointURL_jwksEndpointURL_audience_(
        &self,
        clientID: NSString,
        issuer: NSString,
        tokenEndpointURL: NSURL,
        jwksEndpointURL: NSURL,
        audience: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithClientID : clientID, issuer : issuer, tokenEndpointURL : tokenEndpointURL, jwksEndpointURL : jwksEndpointURL, audience : audience)
    }
    unsafe fn setCustomAssertionRequestHeaderClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomAssertionRequestHeaderClaims : claims, returningError : error)
    }
    unsafe fn setCustomAssertionRequestBodyClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomAssertionRequestBodyClaims : claims, returningError : error)
    }
    unsafe fn setCustomLoginRequestHeaderClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomLoginRequestHeaderClaims : claims, returningError : error)
    }
    unsafe fn setCustomLoginRequestBodyClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomLoginRequestBodyClaims : claims, returningError : error)
    }
    unsafe fn setCustomRefreshRequestHeaderClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomRefreshRequestHeaderClaims : claims, returningError : error)
    }
    unsafe fn setCustomRefreshRequestBodyClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomRefreshRequestBodyClaims : claims, returningError : error)
    }
    unsafe fn setCustomKeyExchangeRequestHeaderClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomKeyExchangeRequestHeaderClaims : claims, returningError : error)
    }
    unsafe fn setCustomKeyExchangeRequestBodyClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomKeyExchangeRequestBodyClaims : claims, returningError : error)
    }
    unsafe fn setCustomKeyRequestHeaderClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomKeyRequestHeaderClaims : claims, returningError : error)
    }
    unsafe fn setCustomKeyRequestBodyClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomKeyRequestBodyClaims : claims, returningError : error)
    }
    unsafe fn invalidCredentialPredicate(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidCredentialPredicate)
    }
    unsafe fn setInvalidCredentialPredicate_(&self, invalidCredentialPredicate: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInvalidCredentialPredicate : invalidCredentialPredicate)
    }
    unsafe fn accountDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountDisplayName)
    }
    unsafe fn setAccountDisplayName_(&self, accountDisplayName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountDisplayName : accountDisplayName)
    }
    unsafe fn clientID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientID)
    }
    unsafe fn issuer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issuer)
    }
    unsafe fn audience(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audience)
    }
    unsafe fn setAudience_(&self, audience: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudience : audience)
    }
    unsafe fn tokenEndpointURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tokenEndpointURL)
    }
    unsafe fn setTokenEndpointURL_(&self, tokenEndpointURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTokenEndpointURL : tokenEndpointURL)
    }
    unsafe fn jwksEndpointURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jwksEndpointURL)
    }
    unsafe fn setJwksEndpointURL_(&self, jwksEndpointURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJwksEndpointURL : jwksEndpointURL)
    }
    unsafe fn jwksTrustedRootCertificates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jwksTrustedRootCertificates)
    }
    unsafe fn setJwksTrustedRootCertificates_(&self, jwksTrustedRootCertificates: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJwksTrustedRootCertificates : jwksTrustedRootCertificates)
    }
    unsafe fn deviceContext(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceContext)
    }
    unsafe fn setDeviceContext_(&self, deviceContext: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeviceContext : deviceContext)
    }
    unsafe fn userSecureEnclaveKeyBiometricPolicy(
        &self,
    ) -> ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userSecureEnclaveKeyBiometricPolicy)
    }
    unsafe fn setUserSecureEnclaveKeyBiometricPolicy_(
        &self,
        userSecureEnclaveKeyBiometricPolicy : ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserSecureEnclaveKeyBiometricPolicy : userSecureEnclaveKeyBiometricPolicy)
    }
    unsafe fn nonceEndpointURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nonceEndpointURL)
    }
    unsafe fn setNonceEndpointURL_(&self, nonceEndpointURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNonceEndpointURL : nonceEndpointURL)
    }
    unsafe fn nonceResponseKeypath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nonceResponseKeypath)
    }
    unsafe fn setNonceResponseKeypath_(&self, nonceResponseKeypath: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNonceResponseKeypath : nonceResponseKeypath)
    }
    unsafe fn serverNonceClaimName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverNonceClaimName)
    }
    unsafe fn setServerNonceClaimName_(&self, serverNonceClaimName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerNonceClaimName : serverNonceClaimName)
    }
    unsafe fn customNonceRequestValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customNonceRequestValues)
    }
    unsafe fn setCustomNonceRequestValues_(&self, customNonceRequestValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomNonceRequestValues : customNonceRequestValues)
    }
    unsafe fn additionalScopes(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalScopes)
    }
    unsafe fn setAdditionalScopes_(&self, additionalScopes: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalScopes : additionalScopes)
    }
    unsafe fn additionalAuthorizationScopes(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalAuthorizationScopes)
    }
    unsafe fn setAdditionalAuthorizationScopes_(&self, additionalAuthorizationScopes: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalAuthorizationScopes : additionalAuthorizationScopes)
    }
    unsafe fn includePreviousRefreshTokenInLoginRequest(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includePreviousRefreshTokenInLoginRequest)
    }
    unsafe fn setIncludePreviousRefreshTokenInLoginRequest_(
        &self,
        includePreviousRefreshTokenInLoginRequest: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludePreviousRefreshTokenInLoginRequest : includePreviousRefreshTokenInLoginRequest)
    }
    unsafe fn previousRefreshTokenClaimName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousRefreshTokenClaimName)
    }
    unsafe fn setPreviousRefreshTokenClaimName_(&self, previousRefreshTokenClaimName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviousRefreshTokenClaimName : previousRefreshTokenClaimName)
    }
    unsafe fn customRequestJWTParameterName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customRequestJWTParameterName)
    }
    unsafe fn setCustomRequestJWTParameterName_(&self, customRequestJWTParameterName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomRequestJWTParameterName : customRequestJWTParameterName)
    }
    unsafe fn customLoginRequestValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customLoginRequestValues)
    }
    unsafe fn setCustomLoginRequestValues_(&self, customLoginRequestValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomLoginRequestValues : customLoginRequestValues)
    }
    unsafe fn uniqueIdentifierClaimName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifierClaimName)
    }
    unsafe fn setUniqueIdentifierClaimName_(&self, uniqueIdentifierClaimName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUniqueIdentifierClaimName : uniqueIdentifierClaimName)
    }
    unsafe fn groupRequestClaimName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupRequestClaimName)
    }
    unsafe fn setGroupRequestClaimName_(&self, groupRequestClaimName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroupRequestClaimName : groupRequestClaimName)
    }
    unsafe fn groupResponseClaimName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupResponseClaimName)
    }
    unsafe fn setGroupResponseClaimName_(&self, groupResponseClaimName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroupResponseClaimName : groupResponseClaimName)
    }
    unsafe fn kerberosTicketMappings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kerberosTicketMappings)
    }
    unsafe fn setKerberosTicketMappings_(&self, kerberosTicketMappings: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKerberosTicketMappings : kerberosTicketMappings)
    }
    unsafe fn refreshEndpointURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refreshEndpointURL)
    }
    unsafe fn setRefreshEndpointURL_(&self, refreshEndpointURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRefreshEndpointURL : refreshEndpointURL)
    }
    unsafe fn customRefreshRequestValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customRefreshRequestValues)
    }
    unsafe fn setCustomRefreshRequestValues_(&self, customRefreshRequestValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomRefreshRequestValues : customRefreshRequestValues)
    }
    unsafe fn federationType(&self) -> ASAuthorizationProviderExtensionFederationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, federationType)
    }
    unsafe fn setFederationType_(
        &self,
        federationType: ASAuthorizationProviderExtensionFederationType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFederationType : federationType)
    }
    unsafe fn federationRequestURN(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, federationRequestURN)
    }
    unsafe fn setFederationRequestURN_(&self, federationRequestURN: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFederationRequestURN : federationRequestURN)
    }
    unsafe fn federationMEXURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, federationMEXURL)
    }
    unsafe fn setFederationMEXURL_(&self, federationMEXURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFederationMEXURL : federationMEXURL)
    }
    unsafe fn federationUserPreauthenticationURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, federationUserPreauthenticationURL)
    }
    unsafe fn setFederationUserPreauthenticationURL_(
        &self,
        federationUserPreauthenticationURL: NSURL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFederationUserPreauthenticationURL : federationUserPreauthenticationURL)
    }
    unsafe fn federationMEXURLKeypath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, federationMEXURLKeypath)
    }
    unsafe fn setFederationMEXURLKeypath_(&self, federationMEXURLKeypath: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFederationMEXURLKeypath : federationMEXURLKeypath)
    }
    unsafe fn federationPredicate(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, federationPredicate)
    }
    unsafe fn setFederationPredicate_(&self, federationPredicate: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFederationPredicate : federationPredicate)
    }
    unsafe fn customFederationUserPreauthenticationRequestValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customFederationUserPreauthenticationRequestValues)
    }
    unsafe fn setCustomFederationUserPreauthenticationRequestValues_(
        &self,
        customFederationUserPreauthenticationRequestValues: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomFederationUserPreauthenticationRequestValues : customFederationUserPreauthenticationRequestValues)
    }
    unsafe fn loginRequestEncryptionPublicKey(&self) -> SecKeyRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginRequestEncryptionPublicKey)
    }
    unsafe fn setLoginRequestEncryptionPublicKey_(&self, loginRequestEncryptionPublicKey: SecKeyRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoginRequestEncryptionPublicKey : loginRequestEncryptionPublicKey)
    }
    unsafe fn loginRequestEncryptionAPVPrefix(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginRequestEncryptionAPVPrefix)
    }
    unsafe fn setLoginRequestEncryptionAPVPrefix_(&self, loginRequestEncryptionAPVPrefix: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoginRequestEncryptionAPVPrefix : loginRequestEncryptionAPVPrefix)
    }
    unsafe fn loginRequestEncryptionAlgorithm(
        &self,
    ) -> ASAuthorizationProviderExtensionEncryptionAlgorithm
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginRequestEncryptionAlgorithm)
    }
    unsafe fn setLoginRequestEncryptionAlgorithm_(&self, loginRequestEncryptionAlgorithm: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoginRequestEncryptionAlgorithm : loginRequestEncryptionAlgorithm)
    }
    unsafe fn loginRequestHPKEPreSharedKey(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginRequestHPKEPreSharedKey)
    }
    unsafe fn setLoginRequestHPKEPreSharedKey_(&self, loginRequestHPKEPreSharedKey: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoginRequestHPKEPreSharedKey : loginRequestHPKEPreSharedKey)
    }
    unsafe fn loginRequestHPKEPreSharedKeyID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginRequestHPKEPreSharedKeyID)
    }
    unsafe fn setLoginRequestHPKEPreSharedKeyID_(&self, loginRequestHPKEPreSharedKeyID: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoginRequestHPKEPreSharedKeyID : loginRequestHPKEPreSharedKeyID)
    }
    unsafe fn keyEndpointURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyEndpointURL)
    }
    unsafe fn setKeyEndpointURL_(&self, keyEndpointURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyEndpointURL : keyEndpointURL)
    }
    unsafe fn customKeyExchangeRequestValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customKeyExchangeRequestValues)
    }
    unsafe fn setCustomKeyExchangeRequestValues_(&self, customKeyExchangeRequestValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomKeyExchangeRequestValues : customKeyExchangeRequestValues)
    }
    unsafe fn customKeyRequestValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customKeyRequestValues)
    }
    unsafe fn setCustomKeyRequestValues_(&self, customKeyRequestValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomKeyRequestValues : customKeyRequestValues)
    }
    unsafe fn hpkePreSharedKey(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hpkePreSharedKey)
    }
    unsafe fn setHpkePreSharedKey_(&self, hpkePreSharedKey: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHpkePreSharedKey : hpkePreSharedKey)
    }
    unsafe fn hpkePreSharedKeyID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hpkePreSharedKeyID)
    }
    unsafe fn setHpkePreSharedKeyID_(&self, hpkePreSharedKeyID: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHpkePreSharedKeyID : hpkePreSharedKeyID)
    }
    unsafe fn hpkeAuthPublicKey(&self) -> SecKeyRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hpkeAuthPublicKey)
    }
    unsafe fn setHpkeAuthPublicKey_(&self, hpkeAuthPublicKey: SecKeyRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHpkeAuthPublicKey : hpkeAuthPublicKey)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionLoginConfiguration").unwrap(), new)
    }
    unsafe fn configurationWithOpenIDConfigurationURL_clientID_issuer_completion_(
        openIDConfigurationURL: NSURL,
        clientID: NSString,
        issuer: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionLoginConfiguration").unwrap(), configurationWithOpenIDConfigurationURL : openIDConfigurationURL, clientID : clientID, issuer : issuer, completion : completion)
    }
}
pub type ASAuthorizationProviderExtensionKeyType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationProviderExtensionLoginManager(pub id);
impl std::ops::Deref for ASAuthorizationProviderExtensionLoginManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationProviderExtensionLoginManager {}
impl ASAuthorizationProviderExtensionLoginManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionLoginManager").unwrap(), alloc) })
    }
}
impl INSObject for ASAuthorizationProviderExtensionLoginManager {}
impl PNSObject for ASAuthorizationProviderExtensionLoginManager {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationProviderExtensionLoginManager {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationProviderExtensionLoginManager, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionLoginManager").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationProviderExtensionLoginManager(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationProviderExtensionLoginManager" ,)
        }
    }
}
impl IASAuthorizationProviderExtensionLoginManager
    for ASAuthorizationProviderExtensionLoginManager
{
}
pub trait IASAuthorizationProviderExtensionLoginManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn saveUserLoginConfiguration_error_(
        &self,
        userLoginConfiguration: ASAuthorizationProviderExtensionUserLoginConfiguration,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveUserLoginConfiguration : userLoginConfiguration, error : error)
    }
    unsafe fn saveLoginConfiguration_error_(
        &self,
        loginConfiguration: ASAuthorizationProviderExtensionLoginConfiguration,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveLoginConfiguration : loginConfiguration, error : error)
    }
    unsafe fn saveCertificate_keyType_(
        &self,
        certificate: SecCertificateRef,
        keyType: ASAuthorizationProviderExtensionKeyType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveCertificate : certificate, keyType : keyType)
    }
    unsafe fn copyKeyForKeyType_(
        &self,
        keyType: ASAuthorizationProviderExtensionKeyType,
    ) -> SecKeyRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyKeyForKeyType : keyType)
    }
    unsafe fn copyIdentityForKeyType_(
        &self,
        keyType: ASAuthorizationProviderExtensionKeyType,
    ) -> SecIdentityRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyIdentityForKeyType : keyType)
    }
    unsafe fn beginKeyRotationForKeyType_(
        &self,
        keyType: ASAuthorizationProviderExtensionKeyType,
    ) -> SecKeyRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginKeyRotationForKeyType : keyType)
    }
    unsafe fn completeKeyRotationForKeyType_(
        &self,
        keyType: ASAuthorizationProviderExtensionKeyType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeKeyRotationForKeyType : keyType)
    }
    unsafe fn userNeedsReauthenticationWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userNeedsReauthenticationWithCompletion : completion)
    }
    unsafe fn deviceRegistrationsNeedsRepair(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceRegistrationsNeedsRepair)
    }
    unsafe fn userRegistrationsNeedsRepair(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userRegistrationsNeedsRepair)
    }
    unsafe fn decryptionKeysNeedRepair(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decryptionKeysNeedRepair)
    }
    unsafe fn resetKeys(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetKeys)
    }
    unsafe fn resetDeviceKeys(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetDeviceKeys)
    }
    unsafe fn resetUserSecureEnclaveKey(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetUserSecureEnclaveKey)
    }
    unsafe fn attestKey_clientDataHash_completion_(
        &self,
        keyType: ASAuthorizationProviderExtensionKeyType,
        clientDataHash: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attestKey : keyType, clientDataHash : clientDataHash, completion : completion)
    }
    unsafe fn attestPendingKey_clientDataHash_completion_(
        &self,
        keyType: ASAuthorizationProviderExtensionKeyType,
        clientDataHash: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attestPendingKey : keyType, clientDataHash : clientDataHash, completion : completion)
    }
    unsafe fn presentRegistrationViewControllerWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentRegistrationViewControllerWithCompletion : completion)
    }
    unsafe fn isDeviceRegistered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDeviceRegistered)
    }
    unsafe fn isUserRegistered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserRegistered)
    }
    unsafe fn registrationToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registrationToken)
    }
    unsafe fn authenticationMethod(&self) -> ASAuthorizationProviderExtensionAuthenticationMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationMethod)
    }
    unsafe fn extensionData(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionData)
    }
    unsafe fn loginUserName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginUserName)
    }
    unsafe fn setLoginUserName_(&self, loginUserName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoginUserName : loginUserName)
    }
    unsafe fn userLoginConfiguration(
        &self,
    ) -> ASAuthorizationProviderExtensionUserLoginConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userLoginConfiguration)
    }
    unsafe fn ssoTokens(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssoTokens)
    }
    unsafe fn setSsoTokens_(&self, ssoTokens: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSsoTokens : ssoTokens)
    }
    unsafe fn loginConfiguration(&self) -> ASAuthorizationProviderExtensionLoginConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginConfiguration)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionLoginManager").unwrap(), new)
    }
}
pub type ASAuthorizationProviderExtensionAuthenticationMethod = NSInteger;
pub type ASAuthorizationProviderExtensionRequestOptions = NSUInteger;
pub type ASAuthorizationProviderExtensionRegistrationResult = NSInteger;
pub type ASAuthorizationProviderExtensionSupportedGrantTypes = NSInteger;
pub type ASAuthorizationProviderExtensionPlatformSSOProtocolVersion = NSInteger;
pub trait PASAuthorizationProviderExtensionRegistrationHandler: Sized + std::ops::Deref {
    unsafe fn beginDeviceRegistrationUsingLoginManager_options_completion_(
        &self,
        loginManager: ASAuthorizationProviderExtensionLoginManager,
        options: ASAuthorizationProviderExtensionRequestOptions,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginDeviceRegistrationUsingLoginManager : loginManager, options : options, completion : completion)
    }
    unsafe fn beginUserRegistrationUsingLoginManager_userName_authenticationMethod_options_completion_(
        &self,
        loginManager: ASAuthorizationProviderExtensionLoginManager,
        userName: NSString,
        authenticationMethod: ASAuthorizationProviderExtensionAuthenticationMethod,
        options: ASAuthorizationProviderExtensionRequestOptions,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginUserRegistrationUsingLoginManager : loginManager, userName : userName, authenticationMethod : authenticationMethod, options : options, completion : completion)
    }
    unsafe fn registrationDidComplete(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registrationDidComplete)
    }
    unsafe fn registrationDidCancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registrationDidCancel)
    }
    unsafe fn supportedGrantTypes(&self) -> ASAuthorizationProviderExtensionSupportedGrantTypes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedGrantTypes)
    }
    unsafe fn protocolVersion(&self) -> ASAuthorizationProviderExtensionPlatformSSOProtocolVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolVersion)
    }
    unsafe fn keyWillRotateForKeyType_newKey_loginManager_completion_(
        &self,
        keyType: ASAuthorizationProviderExtensionKeyType,
        newKey: SecKeyRef,
        loginManager: ASAuthorizationProviderExtensionLoginManager,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, keyWillRotateForKeyType : keyType, newKey : newKey, loginManager : loginManager, completion : completion)
    }
    unsafe fn displayNamesForGroups_loginManager_completion_(
        &self,
        groups: NSArray,
        loginManager: ASAuthorizationProviderExtensionLoginManager,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayNamesForGroups : groups, loginManager : loginManager, completion : completion)
    }
    unsafe fn profilePictureForUserUsingLoginManager_completion_(
        &self,
        loginManager: ASAuthorizationProviderExtensionLoginManager,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, profilePictureForUserUsingLoginManager : loginManager, completion : completion)
    }
    unsafe fn supportedDeviceSigningAlgorithms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedDeviceSigningAlgorithms)
    }
    unsafe fn supportedDeviceEncryptionAlgorithms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedDeviceEncryptionAlgorithms)
    }
    unsafe fn supportedUserSecureEnclaveKeySigningAlgorithms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedUserSecureEnclaveKeySigningAlgorithms)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationProviderExtensionUserLoginConfiguration(pub id);
impl std::ops::Deref for ASAuthorizationProviderExtensionUserLoginConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationProviderExtensionUserLoginConfiguration {}
impl ASAuthorizationProviderExtensionUserLoginConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionUserLoginConfiguration").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationProviderExtensionUserLoginConfiguration {}
impl PNSObject for ASAuthorizationProviderExtensionUserLoginConfiguration {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationProviderExtensionUserLoginConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationProviderExtensionUserLoginConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionUserLoginConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationProviderExtensionUserLoginConfiguration(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationProviderExtensionUserLoginConfiguration" ,)
        }
    }
}
impl IASAuthorizationProviderExtensionUserLoginConfiguration
    for ASAuthorizationProviderExtensionUserLoginConfiguration
{
}
pub trait IASAuthorizationProviderExtensionUserLoginConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLoginUserName_(&self, loginUserName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLoginUserName : loginUserName)
    }
    unsafe fn setCustomAssertionRequestHeaderClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomAssertionRequestHeaderClaims : claims, returningError : error)
    }
    unsafe fn setCustomAssertionRequestBodyClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomAssertionRequestBodyClaims : claims, returningError : error)
    }
    unsafe fn setCustomLoginRequestHeaderClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomLoginRequestHeaderClaims : claims, returningError : error)
    }
    unsafe fn setCustomLoginRequestBodyClaims_returningError_(
        &self,
        claims: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomLoginRequestBodyClaims : claims, returningError : error)
    }
    unsafe fn loginUserName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loginUserName)
    }
    unsafe fn setLoginUserName_(&self, loginUserName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoginUserName : loginUserName)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationProviderExtensionUserLoginConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyAssertionCredential(pub id);
impl std::ops::Deref for ASPasskeyAssertionCredential {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyAssertionCredential {}
impl ASPasskeyAssertionCredential {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyAssertionCredential").unwrap(), alloc) })
    }
}
impl PASAuthorizationCredential for ASPasskeyAssertionCredential {}
impl INSObject for ASPasskeyAssertionCredential {}
impl PNSObject for ASPasskeyAssertionCredential {}
impl std::convert::TryFrom<NSObject> for ASPasskeyAssertionCredential {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPasskeyAssertionCredential, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyAssertionCredential").unwrap()) };
        if is_kind_of {
            Ok(ASPasskeyAssertionCredential(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasskeyAssertionCredential")
        }
    }
}
impl IASPasskeyAssertionCredential for ASPasskeyAssertionCredential {}
pub trait IASPasskeyAssertionCredential: Sized + std::ops::Deref {
    unsafe fn initWithUserHandle_relyingParty_signature_clientDataHash_authenticatorData_credentialID_(
        &self,
        userHandle: NSData,
        relyingParty: NSString,
        signature: NSData,
        clientDataHash: NSData,
        authenticatorData: NSData,
        credentialID: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUserHandle : userHandle, relyingParty : relyingParty, signature : signature, clientDataHash : clientDataHash, authenticatorData : authenticatorData, credentialID : credentialID)
    }
    unsafe fn initWithUserHandle_relyingParty_signature_clientDataHash_authenticatorData_credentialID_extensionOutput_(
        &self,
        userHandle: NSData,
        relyingParty: NSString,
        signature: NSData,
        clientDataHash: NSData,
        authenticatorData: NSData,
        credentialID: NSData,
        extensionOutput: ASPasskeyAssertionCredentialExtensionOutput,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUserHandle : userHandle, relyingParty : relyingParty, signature : signature, clientDataHash : clientDataHash, authenticatorData : authenticatorData, credentialID : credentialID, extensionOutput : extensionOutput)
    }
    unsafe fn userHandle(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userHandle)
    }
    unsafe fn relyingParty(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingParty)
    }
    unsafe fn signature(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signature)
    }
    unsafe fn clientDataHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientDataHash)
    }
    unsafe fn authenticatorData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticatorData)
    }
    unsafe fn credentialID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialID)
    }
    unsafe fn extensionOutput(&self) -> ASPasskeyAssertionCredentialExtensionOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionOutput)
    }
    unsafe fn setExtensionOutput_(
        &self,
        extensionOutput: ASPasskeyAssertionCredentialExtensionOutput,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtensionOutput : extensionOutput)
    }
    unsafe fn credentialWithUserHandle_relyingParty_signature_clientDataHash_authenticatorData_credentialID_(
        userHandle: NSData,
        relyingParty: NSString,
        signature: NSData,
        clientDataHash: NSData,
        authenticatorData: NSData,
        credentialID: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyAssertionCredential").unwrap(), credentialWithUserHandle : userHandle, relyingParty : relyingParty, signature : signature, clientDataHash : clientDataHash, authenticatorData : authenticatorData, credentialID : credentialID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyCredentialIdentity(pub id);
impl std::ops::Deref for ASPasskeyCredentialIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyCredentialIdentity {}
impl ASPasskeyCredentialIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyCredentialIdentity").unwrap(), alloc) })
    }
}
impl PNSCopying for ASPasskeyCredentialIdentity {}
impl PNSSecureCoding for ASPasskeyCredentialIdentity {}
impl PASCredentialIdentity for ASPasskeyCredentialIdentity {}
impl INSObject for ASPasskeyCredentialIdentity {}
impl PNSObject for ASPasskeyCredentialIdentity {}
impl std::convert::TryFrom<NSObject> for ASPasskeyCredentialIdentity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPasskeyCredentialIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyCredentialIdentity").unwrap()) };
        if is_kind_of {
            Ok(ASPasskeyCredentialIdentity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasskeyCredentialIdentity")
        }
    }
}
impl IASPasskeyCredentialIdentity for ASPasskeyCredentialIdentity {}
pub trait IASPasskeyCredentialIdentity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRelyingPartyIdentifier_userName_credentialID_userHandle_recordIdentifier_(
        &self,
        relyingPartyIdentifier: NSString,
        userName: NSString,
        credentialID: NSData,
        userHandle: NSData,
        recordIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRelyingPartyIdentifier : relyingPartyIdentifier, userName : userName, credentialID : credentialID, userHandle : userHandle, recordIdentifier : recordIdentifier)
    }
    unsafe fn relyingPartyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingPartyIdentifier)
    }
    unsafe fn userName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userName)
    }
    unsafe fn credentialID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialID)
    }
    unsafe fn userHandle(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userHandle)
    }
    unsafe fn recordIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordIdentifier)
    }
    unsafe fn rank(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rank)
    }
    unsafe fn setRank_(&self, rank: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRank : rank)
    }
    unsafe fn identityWithRelyingPartyIdentifier_userName_credentialID_userHandle_recordIdentifier_(
        relyingPartyIdentifier: NSString,
        userName: NSString,
        credentialID: NSData,
        userHandle: NSData,
        recordIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyCredentialIdentity").unwrap(), identityWithRelyingPartyIdentifier : relyingPartyIdentifier, userName : userName, credentialID : credentialID, userHandle : userHandle, recordIdentifier : recordIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyCredentialRequest(pub id);
impl std::ops::Deref for ASPasskeyCredentialRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyCredentialRequest {}
impl ASPasskeyCredentialRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyCredentialRequest").unwrap(), alloc) })
    }
}
impl PASCredentialRequest for ASPasskeyCredentialRequest {}
impl INSObject for ASPasskeyCredentialRequest {}
impl PNSObject for ASPasskeyCredentialRequest {}
impl std::convert::TryFrom<NSObject> for ASPasskeyCredentialRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPasskeyCredentialRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyCredentialRequest").unwrap()) };
        if is_kind_of {
            Ok(ASPasskeyCredentialRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasskeyCredentialRequest")
        }
    }
}
impl IASPasskeyCredentialRequest for ASPasskeyCredentialRequest {}
pub trait IASPasskeyCredentialRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms_(
        &self,
        credentialIdentity: ASPasskeyCredentialIdentity,
        clientDataHash: NSData,
        userVerificationPreference: NSString,
        supportedAlgorithms: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCredentialIdentity : credentialIdentity, clientDataHash : clientDataHash, userVerificationPreference : userVerificationPreference, supportedAlgorithms : supportedAlgorithms)
    }
    unsafe fn initWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms_assertionExtensionInput_(
        &self,
        credentialIdentity: ASPasskeyCredentialIdentity,
        clientDataHash: NSData,
        userVerificationPreference: NSString,
        supportedAlgorithms: NSArray,
        assertionExtensionInput: ASPasskeyAssertionCredentialExtensionInput,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCredentialIdentity : credentialIdentity, clientDataHash : clientDataHash, userVerificationPreference : userVerificationPreference, supportedAlgorithms : supportedAlgorithms, assertionExtensionInput : assertionExtensionInput)
    }
    unsafe fn initWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms_registrationExtensionInput_(
        &self,
        credentialIdentity: ASPasskeyCredentialIdentity,
        clientDataHash: NSData,
        userVerificationPreference: NSString,
        supportedAlgorithms: NSArray,
        registrationExtensionInput: ASPasskeyRegistrationCredentialExtensionInput,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCredentialIdentity : credentialIdentity, clientDataHash : clientDataHash, userVerificationPreference : userVerificationPreference, supportedAlgorithms : supportedAlgorithms, registrationExtensionInput : registrationExtensionInput)
    }
    unsafe fn clientDataHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientDataHash)
    }
    unsafe fn userVerificationPreference(
        &self,
    ) -> ASAuthorizationPublicKeyCredentialUserVerificationPreference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userVerificationPreference)
    }
    unsafe fn setUserVerificationPreference_(&self, userVerificationPreference: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserVerificationPreference : userVerificationPreference)
    }
    unsafe fn supportedAlgorithms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedAlgorithms)
    }
    unsafe fn excludedCredentials(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedCredentials)
    }
    unsafe fn assertionExtensionInput(&self) -> ASPasskeyAssertionCredentialExtensionInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assertionExtensionInput)
    }
    unsafe fn registrationExtensionInput(&self) -> ASPasskeyRegistrationCredentialExtensionInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registrationExtensionInput)
    }
    unsafe fn requestWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms_(
        credentialIdentity: ASPasskeyCredentialIdentity,
        clientDataHash: NSData,
        userVerificationPreference: NSString,
        supportedAlgorithms: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyCredentialRequest").unwrap(), requestWithCredentialIdentity : credentialIdentity, clientDataHash : clientDataHash, userVerificationPreference : userVerificationPreference, supportedAlgorithms : supportedAlgorithms)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyRegistrationCredential(pub id);
impl std::ops::Deref for ASPasskeyRegistrationCredential {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyRegistrationCredential {}
impl ASPasskeyRegistrationCredential {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyRegistrationCredential").unwrap(), alloc) })
    }
}
impl PASAuthorizationCredential for ASPasskeyRegistrationCredential {}
impl INSObject for ASPasskeyRegistrationCredential {}
impl PNSObject for ASPasskeyRegistrationCredential {}
impl std::convert::TryFrom<NSObject> for ASPasskeyRegistrationCredential {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPasskeyRegistrationCredential, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyRegistrationCredential").unwrap())
        };
        if is_kind_of {
            Ok(ASPasskeyRegistrationCredential(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasskeyRegistrationCredential")
        }
    }
}
impl IASPasskeyRegistrationCredential for ASPasskeyRegistrationCredential {}
pub trait IASPasskeyRegistrationCredential: Sized + std::ops::Deref {
    unsafe fn initWithRelyingParty_clientDataHash_credentialID_attestationObject_(
        &self,
        relyingParty: NSString,
        clientDataHash: NSData,
        credentialID: NSData,
        attestationObject: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRelyingParty : relyingParty, clientDataHash : clientDataHash, credentialID : credentialID, attestationObject : attestationObject)
    }
    unsafe fn initWithRelyingParty_clientDataHash_credentialID_attestationObject_extensionOutput_(
        &self,
        relyingParty: NSString,
        clientDataHash: NSData,
        credentialID: NSData,
        attestationObject: NSData,
        extensionOutput: ASPasskeyRegistrationCredentialExtensionOutput,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRelyingParty : relyingParty, clientDataHash : clientDataHash, credentialID : credentialID, attestationObject : attestationObject, extensionOutput : extensionOutput)
    }
    unsafe fn relyingParty(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingParty)
    }
    unsafe fn clientDataHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientDataHash)
    }
    unsafe fn credentialID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialID)
    }
    unsafe fn attestationObject(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attestationObject)
    }
    unsafe fn extensionOutput(&self) -> ASPasskeyRegistrationCredentialExtensionOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionOutput)
    }
    unsafe fn setExtensionOutput_(
        &self,
        extensionOutput: ASPasskeyRegistrationCredentialExtensionOutput,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtensionOutput : extensionOutput)
    }
    unsafe fn credentialWithRelyingParty_clientDataHash_credentialID_attestationObject_(
        relyingParty: NSString,
        clientDataHash: NSData,
        credentialID: NSData,
        attestationObject: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyRegistrationCredential").unwrap(), credentialWithRelyingParty : relyingParty, clientDataHash : clientDataHash, credentialID : credentialID, attestationObject : attestationObject)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasswordCredentialRequest(pub id);
impl std::ops::Deref for ASPasswordCredentialRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasswordCredentialRequest {}
impl ASPasswordCredentialRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasswordCredentialRequest").unwrap(), alloc) })
    }
}
impl PASCredentialRequest for ASPasswordCredentialRequest {}
impl INSObject for ASPasswordCredentialRequest {}
impl PNSObject for ASPasswordCredentialRequest {}
impl std::convert::TryFrom<NSObject> for ASPasswordCredentialRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPasswordCredentialRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasswordCredentialRequest").unwrap()) };
        if is_kind_of {
            Ok(ASPasswordCredentialRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasswordCredentialRequest")
        }
    }
}
impl IASPasswordCredentialRequest for ASPasswordCredentialRequest {}
pub trait IASPasswordCredentialRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCredentialIdentity_(
        &self,
        credentialIdentity: ASPasswordCredentialIdentity,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCredentialIdentity : credentialIdentity)
    }
    unsafe fn requestWithCredentialIdentity_(
        credentialIdentity: ASPasswordCredentialIdentity,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasswordCredentialRequest").unwrap(), requestWithCredentialIdentity : credentialIdentity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASSettingsHelper(pub id);
impl std::ops::Deref for ASSettingsHelper {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASSettingsHelper {}
impl ASSettingsHelper {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASSettingsHelper").unwrap(), alloc) })
    }
}
impl INSObject for ASSettingsHelper {}
impl PNSObject for ASSettingsHelper {}
impl std::convert::TryFrom<NSObject> for ASSettingsHelper {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASSettingsHelper, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASSettingsHelper").unwrap()) };
        if is_kind_of {
            Ok(ASSettingsHelper(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASSettingsHelper")
        }
    }
}
impl IASSettingsHelper for ASSettingsHelper {}
pub trait IASSettingsHelper: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn openCredentialProviderAppSettingsWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASSettingsHelper").unwrap(), openCredentialProviderAppSettingsWithCompletionHandler : completionHandler)
    }
    unsafe fn openVerificationCodeAppSettingsWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASSettingsHelper").unwrap(), openVerificationCodeAppSettingsWithCompletionHandler : completionHandler)
    }
    unsafe fn requestToTurnOnCredentialProviderExtensionWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASSettingsHelper").unwrap(), requestToTurnOnCredentialProviderExtensionWithCompletionHandler : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASSettingsHelper").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASWebAuthenticationSessionCallback(pub id);
impl std::ops::Deref for ASWebAuthenticationSessionCallback {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASWebAuthenticationSessionCallback {}
impl ASWebAuthenticationSessionCallback {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionCallback").unwrap(), alloc) })
    }
}
impl INSObject for ASWebAuthenticationSessionCallback {}
impl PNSObject for ASWebAuthenticationSessionCallback {}
impl std::convert::TryFrom<NSObject> for ASWebAuthenticationSessionCallback {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASWebAuthenticationSessionCallback, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionCallback").unwrap())
        };
        if is_kind_of {
            Ok(ASWebAuthenticationSessionCallback(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASWebAuthenticationSessionCallback")
        }
    }
}
impl IASWebAuthenticationSessionCallback for ASWebAuthenticationSessionCallback {}
pub trait IASWebAuthenticationSessionCallback: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn matchesURL_(&self, url: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesURL : url)
    }
    unsafe fn callbackWithCustomScheme_(customScheme: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionCallback").unwrap(), callbackWithCustomScheme : customScheme)
    }
    unsafe fn callbackWithHTTPSHost_path_(host: NSString, path: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionCallback").unwrap(), callbackWithHTTPSHost : host, path : path)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASWebAuthenticationSessionCallback").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyAssertionCredentialExtensionOutput(pub id);
impl std::ops::Deref for ASPasskeyAssertionCredentialExtensionOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyAssertionCredentialExtensionOutput {}
impl ASPasskeyAssertionCredentialExtensionOutput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyAssertionCredentialExtensionOutput").unwrap(), alloc) })
    }
}
impl PNSCopying for ASPasskeyAssertionCredentialExtensionOutput {}
impl PNSSecureCoding for ASPasskeyAssertionCredentialExtensionOutput {}
impl INSObject for ASPasskeyAssertionCredentialExtensionOutput {}
impl PNSObject for ASPasskeyAssertionCredentialExtensionOutput {}
impl std::convert::TryFrom<NSObject> for ASPasskeyAssertionCredentialExtensionOutput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASPasskeyAssertionCredentialExtensionOutput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyAssertionCredentialExtensionOutput").unwrap())
        };
        if is_kind_of {
            Ok(ASPasskeyAssertionCredentialExtensionOutput(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasskeyAssertionCredentialExtensionOutput")
        }
    }
}
impl IASPasskeyAssertionCredentialExtensionOutput for ASPasskeyAssertionCredentialExtensionOutput {}
pub trait IASPasskeyAssertionCredentialExtensionOutput: Sized + std::ops::Deref {
    unsafe fn initWithLargeBlobOutput_(
        &self,
        largeBlob: ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLargeBlobOutput : largeBlob)
    }
    unsafe fn largeBlobAssertionOutput(
        &self,
    ) -> ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeBlobAssertionOutput)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyAssertionCredentialExtensionInput(pub id);
impl std::ops::Deref for ASPasskeyAssertionCredentialExtensionInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyAssertionCredentialExtensionInput {}
impl ASPasskeyAssertionCredentialExtensionInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyAssertionCredentialExtensionInput").unwrap(), alloc) })
    }
}
impl PNSCopying for ASPasskeyAssertionCredentialExtensionInput {}
impl PNSSecureCoding for ASPasskeyAssertionCredentialExtensionInput {}
impl INSObject for ASPasskeyAssertionCredentialExtensionInput {}
impl PNSObject for ASPasskeyAssertionCredentialExtensionInput {}
impl std::convert::TryFrom<NSObject> for ASPasskeyAssertionCredentialExtensionInput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASPasskeyAssertionCredentialExtensionInput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyAssertionCredentialExtensionInput").unwrap())
        };
        if is_kind_of {
            Ok(ASPasskeyAssertionCredentialExtensionInput(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPasskeyAssertionCredentialExtensionInput")
        }
    }
}
impl IASPasskeyAssertionCredentialExtensionInput for ASPasskeyAssertionCredentialExtensionInput {}
pub trait IASPasskeyAssertionCredentialExtensionInput: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn largeBlob(&self) -> ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeBlob)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyAssertionCredentialExtensionInput").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyRegistrationCredentialExtensionInput(pub id);
impl std::ops::Deref for ASPasskeyRegistrationCredentialExtensionInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyRegistrationCredentialExtensionInput {}
impl ASPasskeyRegistrationCredentialExtensionInput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyRegistrationCredentialExtensionInput").unwrap(), alloc) })
    }
}
impl PNSCopying for ASPasskeyRegistrationCredentialExtensionInput {}
impl PNSSecureCoding for ASPasskeyRegistrationCredentialExtensionInput {}
impl INSObject for ASPasskeyRegistrationCredentialExtensionInput {}
impl PNSObject for ASPasskeyRegistrationCredentialExtensionInput {}
impl std::convert::TryFrom<NSObject> for ASPasskeyRegistrationCredentialExtensionInput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASPasskeyRegistrationCredentialExtensionInput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyRegistrationCredentialExtensionInput").unwrap())
        };
        if is_kind_of {
            Ok(ASPasskeyRegistrationCredentialExtensionInput(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASPasskeyRegistrationCredentialExtensionInput" ,)
        }
    }
}
impl IASPasskeyRegistrationCredentialExtensionInput
    for ASPasskeyRegistrationCredentialExtensionInput
{
}
pub trait IASPasskeyRegistrationCredentialExtensionInput: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn largeBlob(&self) -> ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeBlob)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyRegistrationCredentialExtensionInput").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPasskeyRegistrationCredentialExtensionOutput(pub id);
impl std::ops::Deref for ASPasskeyRegistrationCredentialExtensionOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPasskeyRegistrationCredentialExtensionOutput {}
impl ASPasskeyRegistrationCredentialExtensionOutput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASPasskeyRegistrationCredentialExtensionOutput").unwrap(), alloc)
        })
    }
}
impl PNSCopying for ASPasskeyRegistrationCredentialExtensionOutput {}
impl PNSSecureCoding for ASPasskeyRegistrationCredentialExtensionOutput {}
impl INSObject for ASPasskeyRegistrationCredentialExtensionOutput {}
impl PNSObject for ASPasskeyRegistrationCredentialExtensionOutput {}
impl std::convert::TryFrom<NSObject> for ASPasskeyRegistrationCredentialExtensionOutput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASPasskeyRegistrationCredentialExtensionOutput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPasskeyRegistrationCredentialExtensionOutput").unwrap())
        };
        if is_kind_of {
            Ok(ASPasskeyRegistrationCredentialExtensionOutput(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to ASPasskeyRegistrationCredentialExtensionOutput" ,)
        }
    }
}
impl IASPasskeyRegistrationCredentialExtensionOutput
    for ASPasskeyRegistrationCredentialExtensionOutput
{
}
pub trait IASPasskeyRegistrationCredentialExtensionOutput: Sized + std::ops::Deref {
    unsafe fn initWithLargeBlobOutput_(
        &self,
        largeBlob: ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLargeBlobOutput : largeBlob)
    }
    unsafe fn largeBlobRegistrationOutput(
        &self,
    ) -> ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeBlobRegistrationOutput)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialPRFAssertionInputValues(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {}
impl ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFAssertionInputValues").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {}
impl PNSObject for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialPRFAssertionInputValues, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFAssertionInputValues").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialPRFAssertionInputValues(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialPRFAssertionInputValues" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialPRFAssertionInputValues
    for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues
{
}
pub trait IASAuthorizationPublicKeyCredentialPRFAssertionInputValues:
    Sized + std::ops::Deref
{
    unsafe fn initWithSaltInput1_saltInput2_(
        &self,
        saltInput1: NSData,
        saltInput2: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSaltInput1 : saltInput1, saltInput2 : saltInput2)
    }
    unsafe fn saltInput1(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saltInput1)
    }
    unsafe fn saltInput2(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saltInput2)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialPRFAssertionInput(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialPRFAssertionInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialPRFAssertionInput {}
impl ASAuthorizationPublicKeyCredentialPRFAssertionInput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFAssertionInput").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationPublicKeyCredentialPRFAssertionInput {}
impl PNSObject for ASAuthorizationPublicKeyCredentialPRFAssertionInput {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPublicKeyCredentialPRFAssertionInput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialPRFAssertionInput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFAssertionInput").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialPRFAssertionInput(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialPRFAssertionInput" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialPRFAssertionInput
    for ASAuthorizationPublicKeyCredentialPRFAssertionInput
{
}
pub trait IASAuthorizationPublicKeyCredentialPRFAssertionInput: Sized + std::ops::Deref {
    unsafe fn initWithInputValues_perCredentialInputValues_(
        &self,
        inputValues: ASAuthorizationPublicKeyCredentialPRFAssertionInputValues,
        perCredentialInputValues: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInputValues : inputValues, perCredentialInputValues : perCredentialInputValues)
    }
    unsafe fn inputValues(&self) -> ASAuthorizationPublicKeyCredentialPRFAssertionInputValues
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputValues)
    }
    unsafe fn perCredentialInputValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perCredentialInputValues)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialPRFAssertionOutput(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialPRFAssertionOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialPRFAssertionOutput {}
impl ASAuthorizationPublicKeyCredentialPRFAssertionOutput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFAssertionOutput").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationPublicKeyCredentialPRFAssertionOutput {}
impl PNSObject for ASAuthorizationPublicKeyCredentialPRFAssertionOutput {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPublicKeyCredentialPRFAssertionOutput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialPRFAssertionOutput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFAssertionOutput").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialPRFAssertionOutput(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialPRFAssertionOutput" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialPRFAssertionOutput
    for ASAuthorizationPublicKeyCredentialPRFAssertionOutput
{
}
pub trait IASAuthorizationPublicKeyCredentialPRFAssertionOutput: Sized + std::ops::Deref {
    unsafe fn first(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, first)
    }
    unsafe fn second(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, second)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialPRFRegistrationInput(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialPRFRegistrationInput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialPRFRegistrationInput {}
impl ASAuthorizationPublicKeyCredentialPRFRegistrationInput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFRegistrationInput").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationPublicKeyCredentialPRFRegistrationInput {}
impl PNSObject for ASAuthorizationPublicKeyCredentialPRFRegistrationInput {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPublicKeyCredentialPRFRegistrationInput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialPRFRegistrationInput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFRegistrationInput").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialPRFRegistrationInput(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialPRFRegistrationInput" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialPRFRegistrationInput
    for ASAuthorizationPublicKeyCredentialPRFRegistrationInput
{
}
pub trait IASAuthorizationPublicKeyCredentialPRFRegistrationInput: Sized + std::ops::Deref {
    unsafe fn initWithInputValues_(
        &self,
        inputValues: ASAuthorizationPublicKeyCredentialPRFAssertionInputValues,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInputValues : inputValues)
    }
    unsafe fn shouldCheckForSupport(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCheckForSupport)
    }
    unsafe fn inputValues(&self) -> ASAuthorizationPublicKeyCredentialPRFAssertionInputValues
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputValues)
    }
    unsafe fn checkForSupport() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFRegistrationInput").unwrap(), checkForSupport)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAuthorizationPublicKeyCredentialPRFRegistrationOutput(pub id);
impl std::ops::Deref for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {}
impl ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFRegistrationOutput").unwrap(), alloc)
        })
    }
}
impl INSObject for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {}
impl PNSObject for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {}
impl std::convert::TryFrom<NSObject> for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ASAuthorizationPublicKeyCredentialPRFRegistrationOutput, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAuthorizationPublicKeyCredentialPRFRegistrationOutput").unwrap())
        };
        if is_kind_of {
            Ok(ASAuthorizationPublicKeyCredentialPRFRegistrationOutput(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to ASAuthorizationPublicKeyCredentialPRFRegistrationOutput" ,)
        }
    }
}
impl IASAuthorizationPublicKeyCredentialPRFRegistrationOutput
    for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput
{
}
pub trait IASAuthorizationPublicKeyCredentialPRFRegistrationOutput:
    Sized + std::ops::Deref
{
    unsafe fn isSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSupported)
    }
    unsafe fn first(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, first)
    }
    unsafe fn second(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, second)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASGeneratedPassword(pub id);
impl std::ops::Deref for ASGeneratedPassword {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASGeneratedPassword {}
impl ASGeneratedPassword {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASGeneratedPassword").unwrap(), alloc) })
    }
}
impl PNSCopying for ASGeneratedPassword {}
impl PNSSecureCoding for ASGeneratedPassword {}
impl INSObject for ASGeneratedPassword {}
impl PNSObject for ASGeneratedPassword {}
impl std::convert::TryFrom<NSObject> for ASGeneratedPassword {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASGeneratedPassword, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASGeneratedPassword").unwrap()) };
        if is_kind_of {
            Ok(ASGeneratedPassword(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASGeneratedPassword")
        }
    }
}
impl IASGeneratedPassword for ASGeneratedPassword {}
pub trait IASGeneratedPassword: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithKind_value_(&self, kind: NSString, value: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKind : kind, value : value)
    }
    unsafe fn kind(&self) -> ASGeneratedPasswordKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kind)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASGeneratedPassword").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static ASWebAuthenticationSessionErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static ASCredentialIdentityStoreErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static ASExtensionErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static ASAuthorizationScopeFullName: ASAuthorizationScope;
}
unsafe extern "C" {
    pub static ASAuthorizationScopeEmail: ASAuthorizationScope;
}
unsafe extern "C" {
    pub static ASAuthorizationOperationImplicit: ASAuthorizationOpenIDOperation;
}
unsafe extern "C" {
    pub static ASAuthorizationOperationLogin: ASAuthorizationOpenIDOperation;
}
unsafe extern "C" {
    pub static ASAuthorizationOperationRefresh: ASAuthorizationOpenIDOperation;
}
unsafe extern "C" {
    pub static ASAuthorizationOperationLogout: ASAuthorizationOpenIDOperation;
}
unsafe extern "C" {
    pub static ASAuthorizationAppleIDProviderCredentialRevokedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static ASAuthorizationErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderAuthorizationOperationConfigurationRemoved:
        ASAuthorizationProviderAuthorizationOperation;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderAuthorizationOperationDirectRequest:
        ASAuthorizationProviderAuthorizationOperation;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialUserVerificationPreferencePreferred:
        ASAuthorizationPublicKeyCredentialUserVerificationPreference;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialUserVerificationPreferenceRequired:
        ASAuthorizationPublicKeyCredentialUserVerificationPreference;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialUserVerificationPreferenceDiscouraged:
        ASAuthorizationPublicKeyCredentialUserVerificationPreference;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialAttestationKindNone:
        ASAuthorizationPublicKeyCredentialAttestationKind;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialAttestationKindDirect:
        ASAuthorizationPublicKeyCredentialAttestationKind;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialAttestationKindIndirect:
        ASAuthorizationPublicKeyCredentialAttestationKind;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialAttestationKindEnterprise:
        ASAuthorizationPublicKeyCredentialAttestationKind;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialResidentKeyPreferenceDiscouraged:
        ASAuthorizationPublicKeyCredentialResidentKeyPreference;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialResidentKeyPreferencePreferred:
        ASAuthorizationPublicKeyCredentialResidentKeyPreference;
}
unsafe extern "C" {
    pub static ASAuthorizationPublicKeyCredentialResidentKeyPreferenceRequired:
        ASAuthorizationPublicKeyCredentialResidentKeyPreference;
}
unsafe extern "C" {
    pub static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportUSB:
        ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport;
}
unsafe extern "C" {
    pub static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportNFC:
        ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport;
}
unsafe extern "C" {
    pub static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportBluetooth:
        ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport;
}
unsafe extern "C" {
    pub fn ASAuthorizationAllSupportedPublicKeyCredentialDescriptorTransports() -> NSArray;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderExtensionEncryptionAlgorithmECDHE_A256GCM:
        ASAuthorizationProviderExtensionEncryptionAlgorithm;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderExtensionEncryptionAlgorithmHPKE_P256_SHA256_AES_GCM_256:
        ASAuthorizationProviderExtensionEncryptionAlgorithm;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderExtensionEncryptionAlgorithmHPKE_P384_SHA384_AES_GCM_256:
        ASAuthorizationProviderExtensionEncryptionAlgorithm;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderExtensionEncryptionAlgorithmHPKE_Curve25519_SHA256_ChachaPoly:
        ASAuthorizationProviderExtensionEncryptionAlgorithm;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderExtensionSigningAlgorithmES256:
        ASAuthorizationProviderExtensionSigningAlgorithm;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderExtensionSigningAlgorithmES384:
        ASAuthorizationProviderExtensionSigningAlgorithm;
}
unsafe extern "C" {
    pub static ASAuthorizationProviderExtensionSigningAlgorithmEd25519:
        ASAuthorizationProviderExtensionSigningAlgorithm;
}

unsafe impl objc2::encode::RefEncode for ASWebAuthenticationSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASWebAuthenticationSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASWebAuthenticationSessionRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASWebAuthenticationSessionRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASWebAuthenticationSessionWebBrowserSessionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASWebAuthenticationSessionWebBrowserSessionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASCredentialIdentityStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASCredentialIdentityStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASCredentialIdentityStoreState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASCredentialIdentityStoreState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASCredentialProviderExtensionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASCredentialProviderExtensionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASCredentialServiceIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASCredentialServiceIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasswordCredential {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasswordCredential {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasswordCredentialIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasswordCredentialIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorization {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorization {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationAppleIDCredential {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationAppleIDCredential {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationOpenIDRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationOpenIDRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationAppleIDRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationAppleIDRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationAppleIDProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationAppleIDProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPasswordRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPasswordRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPasswordProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPasswordProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSingleSignOnCredential {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSingleSignOnCredential {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSingleSignOnRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSingleSignOnRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSingleSignOnProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSingleSignOnProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationProviderExtensionAuthorizationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationProviderExtensionAuthorizationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationProviderExtensionAuthorizationResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationProviderExtensionAuthorizationResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASOneTimeCodeCredentialIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASOneTimeCodeCredentialIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyCredentialRequestParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyCredentialRequestParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASSavePasswordRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASSavePasswordRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASGeneratePasswordsRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASGeneratePasswordsRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASCredentialProviderViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASCredentialProviderViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccountAuthenticationModificationController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccountAuthenticationModificationController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccountAuthenticationModificationExtensionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccountAuthenticationModificationExtensionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccountAuthenticationModificationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccountAuthenticationModificationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccountAuthenticationModificationViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccountAuthenticationModificationViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationAppleIDButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationAppleIDButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialLargeBlobAssertionOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPlatformPublicKeyCredentialAssertion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPlatformPublicKeyCredentialAssertion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPlatformPublicKeyCredentialDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPlatformPublicKeyCredentialDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPlatformPublicKeyCredentialProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPlatformPublicKeyCredentialProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPlatformPublicKeyCredentialRegistration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPlatformPublicKeyCredentialRegistration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationWebBrowserPlatformPublicKeyCredential {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationWebBrowserPlatformPublicKeyCredential {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationWebBrowserPublicKeyCredentialManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationWebBrowserPublicKeyCredentialManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPublicKeyCredentialClientData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPublicKeyCredentialClientData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASOneTimeCodeCredential {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASOneTimeCodeCredential {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASOneTimeCodeCredentialRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASOneTimeCodeCredentialRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationProviderExtensionKerberosMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationProviderExtensionKerberosMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationProviderExtensionLoginConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationProviderExtensionLoginConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationProviderExtensionLoginManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationProviderExtensionLoginManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationProviderExtensionUserLoginConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationProviderExtensionUserLoginConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyAssertionCredential {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyAssertionCredential {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyCredentialIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyCredentialIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyCredentialRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyCredentialRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyRegistrationCredential {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyRegistrationCredential {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasswordCredentialRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasswordCredentialRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASSettingsHelper {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASSettingsHelper {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASWebAuthenticationSessionCallback {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASWebAuthenticationSessionCallback {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyAssertionCredentialExtensionOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyAssertionCredentialExtensionOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyAssertionCredentialExtensionInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyAssertionCredentialExtensionInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyRegistrationCredentialExtensionInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyRegistrationCredentialExtensionInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPasskeyRegistrationCredentialExtensionOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPasskeyRegistrationCredentialExtensionOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialPRFAssertionInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialPRFAssertionInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialPRFAssertionOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialPRFAssertionOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialPRFRegistrationInput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialPRFRegistrationInput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAuthorizationPublicKeyCredentialPRFRegistrationOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASGeneratedPassword {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASGeneratedPassword {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
