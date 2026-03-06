#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::ExternalAccessory::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAccountApplicationProvider(pub id);
impl std::ops::Deref for VSAccountApplicationProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAccountApplicationProvider {}
impl VSAccountApplicationProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAccountApplicationProvider").unwrap(), alloc) })
    }
}
impl INSObject for VSAccountApplicationProvider {}
impl PNSObject for VSAccountApplicationProvider {}
impl std::convert::TryFrom<NSObject> for VSAccountApplicationProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAccountApplicationProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAccountApplicationProvider").unwrap()) };
        if is_kind_of {
            Ok(VSAccountApplicationProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAccountApplicationProvider")
        }
    }
}
impl IVSAccountApplicationProvider for VSAccountApplicationProvider {}
pub trait IVSAccountApplicationProvider: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLocalizedDisplayName_identifier_(
        &self,
        localizedDisplayName: NSString,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocalizedDisplayName : localizedDisplayName, identifier : identifier)
    }
    unsafe fn localizedDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDisplayName)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VSAccountApplicationProvider").unwrap(), new)
    }
}
pub type VSAccountAccessStatus = NSInteger;
pub type VSCheckAccessOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAccountManager(pub id);
impl std::ops::Deref for VSAccountManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAccountManager {}
impl VSAccountManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAccountManager").unwrap(), alloc) })
    }
}
impl INSObject for VSAccountManager {}
impl PNSObject for VSAccountManager {}
impl std::convert::TryFrom<NSObject> for VSAccountManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAccountManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAccountManager").unwrap()) };
        if is_kind_of {
            Ok(VSAccountManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAccountManager")
        }
    }
}
impl IVSAccountManager for VSAccountManager {}
pub trait IVSAccountManager: Sized + std::ops::Deref {
    unsafe fn checkAccessStatusWithOptions_completionHandler_(
        &self,
        options: NSDictionary,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkAccessStatusWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn enqueueAccountMetadataRequest_completionHandler_(
        &self,
        request: VSAccountMetadataRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> VSAccountManagerResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueAccountMetadataRequest : request, completionHandler : completionHandler)
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
}
pub trait PVSAccountManagerDelegate: Sized + std::ops::Deref {
    unsafe fn accountManager_shouldAuthenticateAccountProviderWithIdentifier_(
        &self,
        accountManager: VSAccountManager,
        accountProviderIdentifier: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountManager : accountManager, shouldAuthenticateAccountProviderWithIdentifier : accountProviderIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAccountManagerResult(pub id);
impl std::ops::Deref for VSAccountManagerResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAccountManagerResult {}
impl VSAccountManagerResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAccountManagerResult").unwrap(), alloc) })
    }
}
impl INSObject for VSAccountManagerResult {}
impl PNSObject for VSAccountManagerResult {}
impl std::convert::TryFrom<NSObject> for VSAccountManagerResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAccountManagerResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAccountManagerResult").unwrap()) };
        if is_kind_of {
            Ok(VSAccountManagerResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAccountManagerResult")
        }
    }
}
impl IVSAccountManagerResult for VSAccountManagerResult {}
pub trait IVSAccountManagerResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VSAccountManagerResult").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAccountMetadata(pub id);
impl std::ops::Deref for VSAccountMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAccountMetadata {}
impl VSAccountMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAccountMetadata").unwrap(), alloc) })
    }
}
impl INSObject for VSAccountMetadata {}
impl PNSObject for VSAccountMetadata {}
impl std::convert::TryFrom<NSObject> for VSAccountMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAccountMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAccountMetadata").unwrap()) };
        if is_kind_of {
            Ok(VSAccountMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAccountMetadata")
        }
    }
}
impl IVSAccountMetadata for VSAccountMetadata {}
pub trait IVSAccountMetadata: Sized + std::ops::Deref {
    unsafe fn accountProviderIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountProviderIdentifier)
    }
    unsafe fn authenticationExpirationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationExpirationDate)
    }
    unsafe fn verificationData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verificationData)
    }
    unsafe fn SAMLAttributeQueryResponse(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SAMLAttributeQueryResponse)
    }
    unsafe fn accountProviderResponse(&self) -> VSAccountProviderResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountProviderResponse)
    }
}
pub type VSAccountProviderAuthenticationScheme = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAccountProviderResponse(pub id);
impl std::ops::Deref for VSAccountProviderResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAccountProviderResponse {}
impl VSAccountProviderResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAccountProviderResponse").unwrap(), alloc) })
    }
}
impl INSObject for VSAccountProviderResponse {}
impl PNSObject for VSAccountProviderResponse {}
impl std::convert::TryFrom<NSObject> for VSAccountProviderResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAccountProviderResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAccountProviderResponse").unwrap()) };
        if is_kind_of {
            Ok(VSAccountProviderResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAccountProviderResponse")
        }
    }
}
impl IVSAccountProviderResponse for VSAccountProviderResponse {}
pub trait IVSAccountProviderResponse: Sized + std::ops::Deref {
    unsafe fn authenticationScheme(&self) -> VSAccountProviderAuthenticationScheme
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationScheme)
    }
    unsafe fn status(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn body(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAccountMetadataRequest(pub id);
impl std::ops::Deref for VSAccountMetadataRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAccountMetadataRequest {}
impl VSAccountMetadataRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAccountMetadataRequest").unwrap(), alloc) })
    }
}
impl INSObject for VSAccountMetadataRequest {}
impl PNSObject for VSAccountMetadataRequest {}
impl std::convert::TryFrom<NSObject> for VSAccountMetadataRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAccountMetadataRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAccountMetadataRequest").unwrap()) };
        if is_kind_of {
            Ok(VSAccountMetadataRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAccountMetadataRequest")
        }
    }
}
impl IVSAccountMetadataRequest for VSAccountMetadataRequest {}
pub trait IVSAccountMetadataRequest: Sized + std::ops::Deref {
    unsafe fn channelIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelIdentifier)
    }
    unsafe fn setChannelIdentifier_(&self, channelIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannelIdentifier : channelIdentifier)
    }
    unsafe fn supportedAccountProviderIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedAccountProviderIdentifiers)
    }
    unsafe fn setSupportedAccountProviderIdentifiers_(
        &self,
        supportedAccountProviderIdentifiers: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedAccountProviderIdentifiers : supportedAccountProviderIdentifiers)
    }
    unsafe fn featuredAccountProviderIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featuredAccountProviderIdentifiers)
    }
    unsafe fn setFeaturedAccountProviderIdentifiers_(
        &self,
        featuredAccountProviderIdentifiers: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFeaturedAccountProviderIdentifiers : featuredAccountProviderIdentifiers)
    }
    unsafe fn verificationToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verificationToken)
    }
    unsafe fn setVerificationToken_(&self, verificationToken: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVerificationToken : verificationToken)
    }
    unsafe fn includeAccountProviderIdentifier(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeAccountProviderIdentifier)
    }
    unsafe fn setIncludeAccountProviderIdentifier_(&self, includeAccountProviderIdentifier: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeAccountProviderIdentifier : includeAccountProviderIdentifier)
    }
    unsafe fn includeAuthenticationExpirationDate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeAuthenticationExpirationDate)
    }
    unsafe fn setIncludeAuthenticationExpirationDate_(
        &self,
        includeAuthenticationExpirationDate: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeAuthenticationExpirationDate : includeAuthenticationExpirationDate)
    }
    unsafe fn localizedVideoTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedVideoTitle)
    }
    unsafe fn setLocalizedVideoTitle_(&self, localizedVideoTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedVideoTitle : localizedVideoTitle)
    }
    unsafe fn isInterruptionAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInterruptionAllowed)
    }
    unsafe fn setInterruptionAllowed_(&self, interruptionAllowed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterruptionAllowed : interruptionAllowed)
    }
    unsafe fn forceAuthentication(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forceAuthentication)
    }
    unsafe fn setForceAuthentication_(&self, forceAuthentication: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setForceAuthentication : forceAuthentication)
    }
    unsafe fn attributeNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeNames)
    }
    unsafe fn setAttributeNames_(&self, attributeNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeNames : attributeNames)
    }
    unsafe fn supportedAuthenticationSchemes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedAuthenticationSchemes)
    }
    unsafe fn setSupportedAuthenticationSchemes_(&self, supportedAuthenticationSchemes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedAuthenticationSchemes : supportedAuthenticationSchemes)
    }
    unsafe fn accountProviderAuthenticationToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountProviderAuthenticationToken)
    }
    unsafe fn setAccountProviderAuthenticationToken_(
        &self,
        accountProviderAuthenticationToken: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountProviderAuthenticationToken : accountProviderAuthenticationToken)
    }
    unsafe fn applicationAccountProviders(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationAccountProviders)
    }
    unsafe fn setApplicationAccountProviders_(&self, applicationAccountProviders: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApplicationAccountProviders : applicationAccountProviders)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAppleSubscription(pub id);
impl std::ops::Deref for VSAppleSubscription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAppleSubscription {}
impl VSAppleSubscription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAppleSubscription").unwrap(), alloc) })
    }
}
impl INSObject for VSAppleSubscription {}
impl PNSObject for VSAppleSubscription {}
impl std::convert::TryFrom<NSObject> for VSAppleSubscription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAppleSubscription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAppleSubscription").unwrap()) };
        if is_kind_of {
            Ok(VSAppleSubscription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAppleSubscription")
        }
    }
}
impl IVSAppleSubscription for VSAppleSubscription {}
pub trait IVSAppleSubscription: Sized + std::ops::Deref {
    unsafe fn initWithCustomerID_productCodes_(
        &self,
        customerID: NSString,
        productCodes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCustomerID : customerID, productCodes : productCodes)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn customerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customerID)
    }
    unsafe fn setCustomerID_(&self, customerID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomerID : customerID)
    }
    unsafe fn productCodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productCodes)
    }
    unsafe fn setProductCodes_(&self, productCodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProductCodes : productCodes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VSAppleSubscription").unwrap(), new)
    }
}
pub type VSAutoSignInAuthorization = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAutoSignInToken(pub id);
impl std::ops::Deref for VSAutoSignInToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAutoSignInToken {}
impl VSAutoSignInToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAutoSignInToken").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for VSAutoSignInToken {}
impl INSObject for VSAutoSignInToken {}
impl PNSObject for VSAutoSignInToken {}
impl std::convert::TryFrom<NSObject> for VSAutoSignInToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAutoSignInToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAutoSignInToken").unwrap()) };
        if is_kind_of {
            Ok(VSAutoSignInToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAutoSignInToken")
        }
    }
}
impl IVSAutoSignInToken for VSAutoSignInToken {}
pub trait IVSAutoSignInToken: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn authorization(&self) -> VSAutoSignInAuthorization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorization)
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VSAutoSignInToken").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSAutoSignInTokenUpdateContext(pub id);
impl std::ops::Deref for VSAutoSignInTokenUpdateContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSAutoSignInTokenUpdateContext {}
impl VSAutoSignInTokenUpdateContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSAutoSignInTokenUpdateContext").unwrap(), alloc) })
    }
}
impl INSObject for VSAutoSignInTokenUpdateContext {}
impl PNSObject for VSAutoSignInTokenUpdateContext {}
impl std::convert::TryFrom<NSObject> for VSAutoSignInTokenUpdateContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSAutoSignInTokenUpdateContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSAutoSignInTokenUpdateContext").unwrap())
        };
        if is_kind_of {
            Ok(VSAutoSignInTokenUpdateContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSAutoSignInTokenUpdateContext")
        }
    }
}
impl IVSAutoSignInTokenUpdateContext for VSAutoSignInTokenUpdateContext {}
pub trait IVSAutoSignInTokenUpdateContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn authorization(&self) -> VSAutoSignInAuthorization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorization)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VSAutoSignInTokenUpdateContext").unwrap(), new)
    }
}
pub type VSSubscriptionAccessLevel = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSSubscription(pub id);
impl std::ops::Deref for VSSubscription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSSubscription {}
impl VSSubscription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSSubscription").unwrap(), alloc) })
    }
}
impl INSObject for VSSubscription {}
impl PNSObject for VSSubscription {}
impl std::convert::TryFrom<NSObject> for VSSubscription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSSubscription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSSubscription").unwrap()) };
        if is_kind_of {
            Ok(VSSubscription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSSubscription")
        }
    }
}
impl IVSSubscription for VSSubscription {}
pub trait IVSSubscription: Sized + std::ops::Deref {
    unsafe fn expirationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expirationDate)
    }
    unsafe fn setExpirationDate_(&self, expirationDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExpirationDate : expirationDate)
    }
    unsafe fn accessLevel(&self) -> VSSubscriptionAccessLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessLevel)
    }
    unsafe fn setAccessLevel_(&self, accessLevel: VSSubscriptionAccessLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessLevel : accessLevel)
    }
    unsafe fn tierIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tierIdentifiers)
    }
    unsafe fn setTierIdentifiers_(&self, tierIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTierIdentifiers : tierIdentifiers)
    }
    unsafe fn billingIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, billingIdentifier)
    }
    unsafe fn setBillingIdentifier_(&self, billingIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBillingIdentifier : billingIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSSubscriptionRegistrationCenter(pub id);
impl std::ops::Deref for VSSubscriptionRegistrationCenter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSSubscriptionRegistrationCenter {}
impl VSSubscriptionRegistrationCenter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSSubscriptionRegistrationCenter").unwrap(), alloc) })
    }
}
impl INSObject for VSSubscriptionRegistrationCenter {}
impl PNSObject for VSSubscriptionRegistrationCenter {}
impl std::convert::TryFrom<NSObject> for VSSubscriptionRegistrationCenter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSSubscriptionRegistrationCenter, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSSubscriptionRegistrationCenter").unwrap())
        };
        if is_kind_of {
            Ok(VSSubscriptionRegistrationCenter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSSubscriptionRegistrationCenter")
        }
    }
}
impl IVSSubscriptionRegistrationCenter for VSSubscriptionRegistrationCenter {}
pub trait IVSSubscriptionRegistrationCenter: Sized + std::ops::Deref {
    unsafe fn setCurrentSubscription_(&self, currentSubscription: VSSubscription)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentSubscription : currentSubscription)
    }
    unsafe fn defaultSubscriptionRegistrationCenter() -> VSSubscriptionRegistrationCenter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VSSubscriptionRegistrationCenter").unwrap(), defaultSubscriptionRegistrationCenter)
    }
}
pub type VSUserAccountType = NSInteger;
pub type VSOriginatingDeviceCategory = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSUserAccount(pub id);
impl std::ops::Deref for VSUserAccount {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSUserAccount {}
impl VSUserAccount {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSUserAccount").unwrap(), alloc) })
    }
}
impl INSObject for VSUserAccount {}
impl PNSObject for VSUserAccount {}
impl std::convert::TryFrom<NSObject> for VSUserAccount {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSUserAccount, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSUserAccount").unwrap()) };
        if is_kind_of {
            Ok(VSUserAccount(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSUserAccount")
        }
    }
}
impl IVSUserAccount for VSUserAccount {}
pub trait IVSUserAccount: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAccountType_updateURL_(
        &self,
        accountType: VSUserAccountType,
        url: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAccountType : accountType, updateURL : url)
    }
    unsafe fn updateURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateURL)
    }
    unsafe fn setUpdateURL_(&self, updateURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpdateURL : updateURL)
    }
    unsafe fn requiresSystemTrust(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresSystemTrust)
    }
    unsafe fn setRequiresSystemTrust_(&self, requiresSystemTrust: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresSystemTrust : requiresSystemTrust)
    }
    unsafe fn accountProviderIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountProviderIdentifier)
    }
    unsafe fn setAccountProviderIdentifier_(&self, accountProviderIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountProviderIdentifier : accountProviderIdentifier)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
    unsafe fn accountType(&self) -> VSUserAccountType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountType)
    }
    unsafe fn setAccountType_(&self, accountType: VSUserAccountType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountType : accountType)
    }
    unsafe fn isSignedOut(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSignedOut)
    }
    unsafe fn setSignedOut_(&self, signedOut: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSignedOut : signedOut)
    }
    unsafe fn subscriptionBillingCycleEndDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionBillingCycleEndDate)
    }
    unsafe fn setSubscriptionBillingCycleEndDate_(&self, subscriptionBillingCycleEndDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubscriptionBillingCycleEndDate : subscriptionBillingCycleEndDate)
    }
    unsafe fn tierIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tierIdentifiers)
    }
    unsafe fn setTierIdentifiers_(&self, tierIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTierIdentifiers : tierIdentifiers)
    }
    unsafe fn billingIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, billingIdentifier)
    }
    unsafe fn setBillingIdentifier_(&self, billingIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBillingIdentifier : billingIdentifier)
    }
    unsafe fn authenticationData(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationData)
    }
    unsafe fn setAuthenticationData_(&self, authenticationData: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthenticationData : authenticationData)
    }
    unsafe fn isFromCurrentDevice(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFromCurrentDevice)
    }
    unsafe fn deviceCategory(&self) -> VSOriginatingDeviceCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceCategory)
    }
    unsafe fn appleSubscription(&self) -> VSAppleSubscription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleSubscription)
    }
    unsafe fn setAppleSubscription_(&self, appleSubscription: VSAppleSubscription)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppleSubscription : appleSubscription)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VSUserAccount").unwrap(), new)
    }
}
pub type VSUserAccountQueryOptions = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VSUserAccountManager(pub id);
impl std::ops::Deref for VSUserAccountManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VSUserAccountManager {}
impl VSUserAccountManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VSUserAccountManager").unwrap(), alloc) })
    }
}
impl INSObject for VSUserAccountManager {}
impl PNSObject for VSUserAccountManager {}
impl std::convert::TryFrom<NSObject> for VSUserAccountManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VSUserAccountManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VSUserAccountManager").unwrap()) };
        if is_kind_of {
            Ok(VSUserAccountManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VSUserAccountManager")
        }
    }
}
impl IVSUserAccountManager for VSUserAccountManager {}
pub trait IVSUserAccountManager: Sized + std::ops::Deref {
    unsafe fn updateUserAccount_completion_(
        &self,
        account: VSUserAccount,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateUserAccount : account, completion : completion)
    }
    unsafe fn queryUserAccountsWithOptions_completion_(
        &self,
        options: VSUserAccountQueryOptions,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryUserAccountsWithOptions : options, completion : completion)
    }
    unsafe fn queryAutoSignInTokenWithCompletionHandler_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryAutoSignInTokenWithCompletionHandler : completion)
    }
    unsafe fn deleteAutoSignInTokenWithCompletionHandler_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteAutoSignInTokenWithCompletionHandler : completion)
    }
    unsafe fn sharedUserAccountManager() -> VSUserAccountManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VSUserAccountManager").unwrap(), sharedUserAccountManager)
    }
}
pub type VSErrorCode = NSInteger;
unsafe extern "C" {
    pub static VSOpenTVProviderSettingsURLString: NSString;
}
unsafe extern "C" {
    pub static VSCheckAccessOptionPrompt: VSCheckAccessOption;
}
unsafe extern "C" {
    pub static VSAccountProviderAuthenticationSchemeSAML: VSAccountProviderAuthenticationScheme;
}
unsafe extern "C" {
    pub static VSAccountProviderAuthenticationSchemeAPI: VSAccountProviderAuthenticationScheme;
}
unsafe extern "C" {
    pub static VSErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static VSErrorInfoKeySAMLResponse: NSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static VSErrorInfoKeySAMLResponseStatus: NSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static VSErrorInfoKeyAccountProviderResponse: NSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static VSErrorInfoKeyUnsupportedProviderIdentifier: NSErrorUserInfoKey;
}

unsafe impl objc2::encode::RefEncode for VSAccountApplicationProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAccountApplicationProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSAccountManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAccountManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSAccountManagerResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAccountManagerResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSAccountMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAccountMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSAccountProviderResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAccountProviderResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSAccountMetadataRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAccountMetadataRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSAppleSubscription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAppleSubscription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSAutoSignInToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAutoSignInToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSAutoSignInTokenUpdateContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSAutoSignInTokenUpdateContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSSubscription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSSubscription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSSubscriptionRegistrationCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSSubscriptionRegistrationCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSUserAccount {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSUserAccount {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VSUserAccountManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VSUserAccountManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
