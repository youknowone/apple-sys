#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ACAccount(pub id);
impl std::ops::Deref for ACAccount {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ACAccount {}
impl ACAccount {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ACAccount").unwrap(), alloc) })
    }
}
impl INSObject for ACAccount {}
impl PNSObject for ACAccount {}
impl std::convert::TryFrom<NSObject> for ACAccount {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ACAccount, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ACAccount").unwrap()) };
        if is_kind_of {
            Ok(ACAccount(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ACAccount")
        }
    }
}
impl IACAccount for ACAccount {}
pub trait IACAccount: Sized + std::ops::Deref {
    unsafe fn initWithAccountType_(&self, type_: ACAccountType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAccountType : type_)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn accountType(&self) -> ACAccountType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountType)
    }
    unsafe fn setAccountType_(&self, accountType: ACAccountType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountType : accountType)
    }
    unsafe fn accountDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountDescription)
    }
    unsafe fn setAccountDescription_(&self, accountDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountDescription : accountDescription)
    }
    unsafe fn username(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, username)
    }
    unsafe fn setUsername_(&self, username: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsername : username)
    }
    unsafe fn userFullName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userFullName)
    }
    unsafe fn credential(&self) -> ACAccountCredential
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credential)
    }
    unsafe fn setCredential_(&self, credential: ACAccountCredential)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCredential : credential)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ACAccountType(pub id);
impl std::ops::Deref for ACAccountType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ACAccountType {}
impl ACAccountType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ACAccountType").unwrap(), alloc) })
    }
}
impl INSObject for ACAccountType {}
impl PNSObject for ACAccountType {}
impl std::convert::TryFrom<NSObject> for ACAccountType {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ACAccountType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ACAccountType").unwrap()) };
        if is_kind_of {
            Ok(ACAccountType(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ACAccountType")
        }
    }
}
impl IACAccountType for ACAccountType {}
pub trait IACAccountType: Sized + std::ops::Deref {
    unsafe fn accountTypeDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountTypeDescription)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn accessGranted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessGranted)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ACAccountCredential(pub id);
impl std::ops::Deref for ACAccountCredential {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ACAccountCredential {}
impl ACAccountCredential {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ACAccountCredential").unwrap(), alloc) })
    }
}
impl INSObject for ACAccountCredential {}
impl PNSObject for ACAccountCredential {}
impl std::convert::TryFrom<NSObject> for ACAccountCredential {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ACAccountCredential, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ACAccountCredential").unwrap()) };
        if is_kind_of {
            Ok(ACAccountCredential(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ACAccountCredential")
        }
    }
}
impl IACAccountCredential for ACAccountCredential {}
pub trait IACAccountCredential: Sized + std::ops::Deref {
    unsafe fn initWithOAuthToken_tokenSecret_(
        &self,
        token: NSString,
        secret: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOAuthToken : token, tokenSecret : secret)
    }
    unsafe fn initWithOAuth2Token_refreshToken_expiryDate_(
        &self,
        token: NSString,
        refreshToken: NSString,
        expiryDate: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOAuth2Token : token, refreshToken : refreshToken, expiryDate : expiryDate)
    }
    unsafe fn oauthToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, oauthToken)
    }
    unsafe fn setOauthToken_(&self, oauthToken: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOauthToken : oauthToken)
    }
}
pub type ACAccountCredentialRenewResult = NSInteger;
pub type ACAccountStoreSaveCompletionHandler = *mut ::std::os::raw::c_void;
pub type ACAccountStoreRemoveCompletionHandler = *mut ::std::os::raw::c_void;
pub type ACAccountStoreRequestAccessCompletionHandler = *mut ::std::os::raw::c_void;
pub type ACAccountStoreCredentialRenewalHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ACAccountStore(pub id);
impl std::ops::Deref for ACAccountStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ACAccountStore {}
impl ACAccountStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ACAccountStore").unwrap(), alloc) })
    }
}
impl INSObject for ACAccountStore {}
impl PNSObject for ACAccountStore {}
impl std::convert::TryFrom<NSObject> for ACAccountStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ACAccountStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ACAccountStore").unwrap()) };
        if is_kind_of {
            Ok(ACAccountStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ACAccountStore")
        }
    }
}
impl IACAccountStore for ACAccountStore {}
pub trait IACAccountStore: Sized + std::ops::Deref {
    unsafe fn accountWithIdentifier_(&self, identifier: NSString) -> ACAccount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountWithIdentifier : identifier)
    }
    unsafe fn accountTypeWithAccountTypeIdentifier_(
        &self,
        typeIdentifier: NSString,
    ) -> ACAccountType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountTypeWithAccountTypeIdentifier : typeIdentifier)
    }
    unsafe fn accountsWithAccountType_(&self, accountType: ACAccountType) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountsWithAccountType : accountType)
    }
    unsafe fn saveAccount_withCompletionHandler_(
        &self,
        account: ACAccount,
        completionHandler: ACAccountStoreSaveCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveAccount : account, withCompletionHandler : completionHandler)
    }
    unsafe fn requestAccessToAccountsWithType_withCompletionHandler_(
        &self,
        accountType: ACAccountType,
        handler: ACAccountStoreRequestAccessCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAccessToAccountsWithType : accountType, withCompletionHandler : handler)
    }
    unsafe fn requestAccessToAccountsWithType_options_completion_(
        &self,
        accountType: ACAccountType,
        options: NSDictionary,
        completion: ACAccountStoreRequestAccessCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAccessToAccountsWithType : accountType, options : options, completion : completion)
    }
    unsafe fn renewCredentialsForAccount_completion_(
        &self,
        account: ACAccount,
        completionHandler: ACAccountStoreCredentialRenewalHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renewCredentialsForAccount : account, completion : completionHandler)
    }
    unsafe fn removeAccount_withCompletionHandler_(
        &self,
        account: ACAccount,
        completionHandler: ACAccountStoreRemoveCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAccount : account, withCompletionHandler : completionHandler)
    }
    unsafe fn accounts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accounts)
    }
}
pub type ACErrorCode = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub static ACAccountTypeIdentifierTwitter: NSString;
}
unsafe extern "C" {
    pub static ACAccountTypeIdentifierFacebook: NSString;
}
unsafe extern "C" {
    pub static ACAccountTypeIdentifierSinaWeibo: NSString;
}
unsafe extern "C" {
    pub static ACAccountTypeIdentifierTencentWeibo: NSString;
}
unsafe extern "C" {
    pub static ACFacebookAppIdKey: NSString;
}
unsafe extern "C" {
    pub static ACFacebookPermissionsKey: NSString;
}
unsafe extern "C" {
    pub static ACFacebookAudienceKey: NSString;
}
unsafe extern "C" {
    pub static ACFacebookAudienceEveryone: NSString;
}
unsafe extern "C" {
    pub static ACFacebookAudienceFriends: NSString;
}
unsafe extern "C" {
    pub static ACFacebookAudienceOnlyMe: NSString;
}
unsafe extern "C" {
    pub static ACTencentWeiboAppIdKey: NSString;
}
unsafe extern "C" {
    pub static ACAccountStoreDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static ACErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for ACAccount {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ACAccount {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ACAccountType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ACAccountType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ACAccountCredential {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ACAccountCredential {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ACAccountStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ACAccountStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
