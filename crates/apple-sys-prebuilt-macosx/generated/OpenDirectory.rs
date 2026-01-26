#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::SecurityFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ODContext {
    _unused: [u8; 0],
}
pub type ODContextRef = *const __ODContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ODNode {
    _unused: [u8; 0],
}
pub type ODNodeRef = *mut __ODNode;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ODQuery {
    _unused: [u8; 0],
}
pub type ODQueryRef = *mut __ODQuery;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ODRecord {
    _unused: [u8; 0],
}
pub type ODRecordRef = *mut __ODRecord;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ODSession {
    _unused: [u8; 0],
}
pub type ODSessionRef = *mut __ODSession;
pub type ODNodeType = u32;
pub type ODMatchType = u32;
pub type ODRecordType = NSString;
pub type ODAttributeType = NSString;
pub type ODAuthenticationType = NSString;
pub type ODPolicyType = NSString;
pub type ODErrorUserInfoKeyType = NSString;
pub type ODOptionKeyType = NSString;
pub type _ODRecordType = ODRecordType;
pub type _ODAttributeType = ODAttributeType;
pub type _ODAuthenticationType = ODAuthenticationType;
pub type ODPolicyKeyType = NSString;
pub type ODPolicyCategoryType = NSString;
pub type ODPolicyAttributeType = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODSession(pub id);
impl std::ops::Deref for ODSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODSession {}
impl ODSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODSession").unwrap(), alloc) })
    }
}
impl INSObject for ODSession {}
impl PNSObject for ODSession {}
impl std::convert::TryFrom<NSObject> for ODSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODSession, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODSession").unwrap()) };
        if is_kind_of {
            Ok(ODSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODSession")
        }
    }
}
impl IODSession for ODSession {}
pub trait IODSession: Sized + std::ops::Deref {
    unsafe fn initWithOptions_error_(
        &self,
        inOptions: NSDictionary,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOptions : inOptions, error : outError)
    }
    unsafe fn nodeNamesAndReturnError_(&self, outError: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeNamesAndReturnError : outError)
    }
    unsafe fn configurationAuthorizationAllowingUserInteraction_error_(
        &self,
        allowInteraction: BOOL,
        error: *mut NSError,
    ) -> SFAuthorization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, configurationAuthorizationAllowingUserInteraction : allowInteraction, error : error)
    }
    unsafe fn configurationForNodename_(&self, nodename: NSString) -> ODConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, configurationForNodename : nodename)
    }
    unsafe fn addConfiguration_authorization_error_(
        &self,
        configuration: ODConfiguration,
        authorization: SFAuthorization,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConfiguration : configuration, authorization : authorization, error : error)
    }
    unsafe fn deleteConfiguration_authorization_error_(
        &self,
        configuration: ODConfiguration,
        authorization: SFAuthorization,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteConfiguration : configuration, authorization : authorization, error : error)
    }
    unsafe fn deleteConfigurationWithNodename_authorization_error_(
        &self,
        nodename: NSString,
        authorization: SFAuthorization,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteConfigurationWithNodename : nodename, authorization : authorization, error : error)
    }
    unsafe fn configurationTemplateNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationTemplateNames)
    }
    unsafe fn mappingTemplateNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mappingTemplateNames)
    }
    unsafe fn defaultSession() -> ODSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODSession").unwrap(), defaultSession)
    }
    unsafe fn sessionWithOptions_error_(
        inOptions: NSDictionary,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODSession").unwrap(), sessionWithOptions : inOptions, error : outError)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODRecord(pub id);
impl std::ops::Deref for ODRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODRecord {}
impl ODRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODRecord").unwrap(), alloc) })
    }
}
impl INSObject for ODRecord {}
impl PNSObject for ODRecord {}
impl std::convert::TryFrom<NSObject> for ODRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODRecord, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODRecord").unwrap()) };
        if is_kind_of {
            Ok(ODRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODRecord")
        }
    }
}
impl IODRecord for ODRecord {}
pub trait IODRecord: Sized + std::ops::Deref {
    unsafe fn setNodeCredentials_password_error_(
        &self,
        inUsername: NSString,
        inPassword: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNodeCredentials : inUsername, password : inPassword, error : outError)
    }
    unsafe fn setNodeCredentialsWithRecordType_authenticationType_authenticationItems_continueItems_context_error_(
        &self,
        inRecordType: NSString,
        inType: NSString,
        inItems: NSArray,
        outItems: *mut NSArray,
        outContext: *mut id,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNodeCredentialsWithRecordType : inRecordType, authenticationType : inType, authenticationItems : inItems, continueItems : outItems, context : outContext, error : outError)
    }
    unsafe fn setNodeCredentialsUsingKerberosCache_error_(
        &self,
        inCacheName: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNodeCredentialsUsingKerberosCache : inCacheName, error : outError)
    }
    unsafe fn passwordPolicyAndReturnError_(&self, outError: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, passwordPolicyAndReturnError : outError)
    }
    unsafe fn verifyPassword_error_(&self, inPassword: NSString, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, verifyPassword : inPassword, error : outError)
    }
    unsafe fn verifyExtendedWithAuthenticationType_authenticationItems_continueItems_context_error_(
        &self,
        inType: NSString,
        inItems: NSArray,
        outItems: *mut NSArray,
        outContext: *mut id,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, verifyExtendedWithAuthenticationType : inType, authenticationItems : inItems, continueItems : outItems, context : outContext, error : outError)
    }
    unsafe fn changePassword_toPassword_error_(
        &self,
        oldPassword: NSString,
        newPassword: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changePassword : oldPassword, toPassword : newPassword, error : outError)
    }
    unsafe fn synchronizeAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, synchronizeAndReturnError : outError)
    }
    unsafe fn recordDetailsForAttributes_error_(
        &self,
        inAttributes: NSArray,
        outError: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordDetailsForAttributes : inAttributes, error : outError)
    }
    unsafe fn valuesForAttribute_error_(
        &self,
        inAttribute: NSString,
        outError: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valuesForAttribute : inAttribute, error : outError)
    }
    unsafe fn setValue_forAttribute_error_(
        &self,
        inValueOrValues: id,
        inAttribute: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : inValueOrValues, forAttribute : inAttribute, error : outError)
    }
    unsafe fn removeValuesForAttribute_error_(
        &self,
        inAttribute: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeValuesForAttribute : inAttribute, error : outError)
    }
    unsafe fn addValue_toAttribute_error_(
        &self,
        inValue: id,
        inAttribute: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addValue : inValue, toAttribute : inAttribute, error : outError)
    }
    unsafe fn removeValue_fromAttribute_error_(
        &self,
        inValue: id,
        inAttribute: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeValue : inValue, fromAttribute : inAttribute, error : outError)
    }
    unsafe fn deleteRecordAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteRecordAndReturnError : outError)
    }
    unsafe fn policiesAndReturnError_(&self, error: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, policiesAndReturnError : error)
    }
    unsafe fn effectivePoliciesAndReturnError_(&self, error: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, effectivePoliciesAndReturnError : error)
    }
    unsafe fn supportedPoliciesAndReturnError_(&self, error: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedPoliciesAndReturnError : error)
    }
    unsafe fn setPolicies_error_(&self, policies: NSDictionary, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPolicies : policies, error : error)
    }
    unsafe fn setPolicy_value_error_(
        &self,
        policy: NSString,
        value: id,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPolicy : policy, value : value, error : error)
    }
    unsafe fn removePolicy_error_(&self, policy: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePolicy : policy, error : error)
    }
    unsafe fn addAccountPolicy_toCategory_error_(
        &self,
        policy: NSDictionary,
        category: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAccountPolicy : policy, toCategory : category, error : error)
    }
    unsafe fn removeAccountPolicy_fromCategory_error_(
        &self,
        policy: NSDictionary,
        category: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAccountPolicy : policy, fromCategory : category, error : error)
    }
    unsafe fn setAccountPolicies_error_(&self, policies: NSDictionary, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountPolicies : policies, error : error)
    }
    unsafe fn accountPoliciesAndReturnError_(&self, error: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountPoliciesAndReturnError : error)
    }
    unsafe fn authenticationAllowedAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authenticationAllowedAndReturnError : error)
    }
    unsafe fn passwordChangeAllowed_error_(
        &self,
        newPassword: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, passwordChangeAllowed : newPassword, error : error)
    }
    unsafe fn willPasswordExpire_(&self, willExpireIn: u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willPasswordExpire : willExpireIn)
    }
    unsafe fn willAuthenticationsExpire_(&self, willExpireIn: u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willAuthenticationsExpire : willExpireIn)
    }
    unsafe fn recordType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordType)
    }
    unsafe fn recordName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordName)
    }
    unsafe fn secondsUntilPasswordExpires(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secondsUntilPasswordExpires)
    }
    unsafe fn secondsUntilAuthenticationsExpire(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secondsUntilAuthenticationsExpire)
    }
}
impl ODRecord_ODRecordGroupExtensions for ODRecord {}
pub trait ODRecord_ODRecordGroupExtensions: Sized + std::ops::Deref {
    unsafe fn addMemberRecord_error_(&self, inRecord: ODRecord, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMemberRecord : inRecord, error : outError)
    }
    unsafe fn removeMemberRecord_error_(&self, inRecord: ODRecord, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeMemberRecord : inRecord, error : outError)
    }
    unsafe fn isMemberRecord_error_(&self, inRecord: ODRecord, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isMemberRecord : inRecord, error : outError)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODNode(pub id);
impl std::ops::Deref for ODNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODNode {}
impl ODNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODNode").unwrap(), alloc) })
    }
}
impl INSObject for ODNode {}
impl PNSObject for ODNode {}
impl std::convert::TryFrom<NSObject> for ODNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODNode, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODNode").unwrap()) };
        if is_kind_of {
            Ok(ODNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODNode")
        }
    }
}
impl IODNode for ODNode {}
pub trait IODNode: Sized + std::ops::Deref {
    unsafe fn initWithSession_type_error_(
        &self,
        inSession: ODSession,
        inType: ODNodeType,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSession : inSession, r#type : inType, error : outError)
    }
    unsafe fn initWithSession_name_error_(
        &self,
        inSession: ODSession,
        inName: NSString,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSession : inSession, name : inName, error : outError)
    }
    unsafe fn subnodeNamesAndReturnError_(&self, outError: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, subnodeNamesAndReturnError : outError)
    }
    unsafe fn unreachableSubnodeNamesAndReturnError_(&self, outError: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unreachableSubnodeNamesAndReturnError : outError)
    }
    unsafe fn nodeDetailsForKeys_error_(
        &self,
        inKeys: NSArray,
        outError: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeDetailsForKeys : inKeys, error : outError)
    }
    unsafe fn supportedRecordTypesAndReturnError_(&self, outError: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedRecordTypesAndReturnError : outError)
    }
    unsafe fn supportedAttributesForRecordType_error_(
        &self,
        inRecordType: NSString,
        outError: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedAttributesForRecordType : inRecordType, error : outError)
    }
    unsafe fn setCredentialsWithRecordType_recordName_password_error_(
        &self,
        inRecordType: NSString,
        inRecordName: NSString,
        inPassword: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCredentialsWithRecordType : inRecordType, recordName : inRecordName, password : inPassword, error : outError)
    }
    unsafe fn setCredentialsWithRecordType_authenticationType_authenticationItems_continueItems_context_error_(
        &self,
        inRecordType: NSString,
        inType: NSString,
        inItems: NSArray,
        outItems: *mut NSArray,
        outContext: *mut id,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCredentialsWithRecordType : inRecordType, authenticationType : inType, authenticationItems : inItems, continueItems : outItems, context : outContext, error : outError)
    }
    unsafe fn setCredentialsUsingKerberosCache_error_(
        &self,
        inCacheName: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCredentialsUsingKerberosCache : inCacheName, error : outError)
    }
    unsafe fn createRecordWithRecordType_name_attributes_error_(
        &self,
        inRecordType: NSString,
        inRecordName: NSString,
        inAttributes: NSDictionary,
        outError: *mut NSError,
    ) -> ODRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createRecordWithRecordType : inRecordType, name : inRecordName, attributes : inAttributes, error : outError)
    }
    unsafe fn recordWithRecordType_name_attributes_error_(
        &self,
        inRecordType: NSString,
        inRecordName: NSString,
        inAttributes: id,
        outError: *mut NSError,
    ) -> ODRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordWithRecordType : inRecordType, name : inRecordName, attributes : inAttributes, error : outError)
    }
    unsafe fn customCall_sendData_error_(
        &self,
        inCustomCode: NSInteger,
        inSendData: NSData,
        outError: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, customCall : inCustomCode, sendData : inSendData, error : outError)
    }
    unsafe fn customFunction_payload_error_(
        &self,
        function: NSString,
        payload: id,
        error: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, customFunction : function, payload : payload, error : error)
    }
    unsafe fn policiesAndReturnError_(&self, error: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, policiesAndReturnError : error)
    }
    unsafe fn supportedPoliciesAndReturnError_(&self, error: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedPoliciesAndReturnError : error)
    }
    unsafe fn setPolicies_error_(&self, policies: NSDictionary, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPolicies : policies, error : error)
    }
    unsafe fn setPolicy_value_error_(
        &self,
        policy: NSString,
        value: id,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPolicy : policy, value : value, error : error)
    }
    unsafe fn removePolicy_error_(&self, policy: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePolicy : policy, error : error)
    }
    unsafe fn addAccountPolicy_toCategory_error_(
        &self,
        policy: NSDictionary,
        category: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAccountPolicy : policy, toCategory : category, error : error)
    }
    unsafe fn removeAccountPolicy_fromCategory_error_(
        &self,
        policy: NSDictionary,
        category: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAccountPolicy : policy, fromCategory : category, error : error)
    }
    unsafe fn setAccountPolicies_error_(&self, policies: NSDictionary, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountPolicies : policies, error : error)
    }
    unsafe fn accountPoliciesAndReturnError_(&self, error: *mut NSError) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accountPoliciesAndReturnError : error)
    }
    unsafe fn passwordContentCheck_forRecordName_error_(
        &self,
        password: NSString,
        recordName: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, passwordContentCheck : password, forRecordName : recordName, error : error)
    }
    unsafe fn nodeName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodeName)
    }
    unsafe fn configuration(&self) -> ODConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn nodeWithSession_type_error_(
        inSession: ODSession,
        inType: ODNodeType,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODNode").unwrap(), nodeWithSession : inSession, r#type : inType, error : outError)
    }
    unsafe fn nodeWithSession_name_error_(
        inSession: ODSession,
        inName: NSString,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODNode").unwrap(), nodeWithSession : inSession, name : inName, error : outError)
    }
}
pub trait PODQueryDelegate: Sized + std::ops::Deref {
    unsafe fn query_foundResults_error_(
        &self,
        inQuery: ODQuery,
        inResults: NSArray,
        inError: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, query : inQuery, foundResults : inResults, error : inError)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODQuery(pub id);
impl std::ops::Deref for ODQuery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODQuery {}
impl ODQuery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODQuery").unwrap(), alloc) })
    }
}
impl PNSCopying for ODQuery {}
impl INSObject for ODQuery {}
impl PNSObject for ODQuery {}
impl std::convert::TryFrom<NSObject> for ODQuery {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODQuery, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODQuery").unwrap()) };
        if is_kind_of {
            Ok(ODQuery(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODQuery")
        }
    }
}
impl IODQuery for ODQuery {}
pub trait IODQuery: Sized + std::ops::Deref {
    unsafe fn initWithNode_forRecordTypes_attribute_matchType_queryValues_returnAttributes_maximumResults_error_(
        &self,
        inNode: ODNode,
        inRecordTypeOrList: id,
        inAttribute: NSString,
        inMatchType: ODMatchType,
        inQueryValueOrList: id,
        inReturnAttributeOrList: id,
        inMaximumResults: NSInteger,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNode : inNode, forRecordTypes : inRecordTypeOrList, attribute : inAttribute, matchType : inMatchType, queryValues : inQueryValueOrList, returnAttributes : inReturnAttributeOrList, maximumResults : inMaximumResults, error : outError)
    }
    unsafe fn resultsAllowingPartial_error_(
        &self,
        inAllowPartialResults: BOOL,
        outError: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resultsAllowingPartial : inAllowPartialResults, error : outError)
    }
    unsafe fn scheduleInRunLoop_forMode_(&self, inRunLoop: NSRunLoop, inMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleInRunLoop : inRunLoop, forMode : inMode)
    }
    unsafe fn removeFromRunLoop_forMode_(&self, inRunLoop: NSRunLoop, inMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromRunLoop : inRunLoop, forMode : inMode)
    }
    unsafe fn synchronize(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, synchronize)
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
    unsafe fn operationQueue(&self) -> NSOperationQueue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operationQueue)
    }
    unsafe fn setOperationQueue_(&self, operationQueue: NSOperationQueue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOperationQueue : operationQueue)
    }
    unsafe fn queryWithNode_forRecordTypes_attribute_matchType_queryValues_returnAttributes_maximumResults_error_(
        inNode: ODNode,
        inRecordTypeOrList: id,
        inAttribute: NSString,
        inMatchType: ODMatchType,
        inQueryValueOrList: id,
        inReturnAttributeOrList: id,
        inMaximumResults: NSInteger,
        outError: *mut NSError,
    ) -> ODQuery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODQuery").unwrap(), queryWithNode : inNode, forRecordTypes : inRecordTypeOrList, attribute : inAttribute, matchType : inMatchType, queryValues : inQueryValueOrList, returnAttributes : inReturnAttributeOrList, maximumResults : inMaximumResults, error : outError)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODConfiguration(pub id);
impl std::ops::Deref for ODConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODConfiguration {}
impl ODConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for ODConfiguration {}
impl PNSObject for ODConfiguration {}
impl std::convert::TryFrom<NSObject> for ODConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODConfiguration").unwrap()) };
        if is_kind_of {
            Ok(ODConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODConfiguration")
        }
    }
}
impl IODConfiguration for ODConfiguration {}
pub trait IODConfiguration: Sized + std::ops::Deref {
    unsafe fn saveUsingAuthorization_error_(
        &self,
        authorization: SFAuthorization,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveUsingAuthorization : authorization, error : error)
    }
    unsafe fn addTrustType_trustAccount_trustPassword_username_password_joinExisting_error_(
        &self,
        trustType: NSString,
        account: NSString,
        accountPassword: NSString,
        username: NSString,
        password: NSString,
        join: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTrustType : trustType, trustAccount : account, trustPassword : accountPassword, username : username, password : password, joinExisting : join, error : error)
    }
    unsafe fn removeTrustUsingUsername_password_deleteTrustAccount_error_(
        &self,
        username: NSString,
        password: NSString,
        deleteAccount: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTrustUsingUsername : username, password : password, deleteTrustAccount : deleteAccount, error : error)
    }
    unsafe fn nodeName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodeName)
    }
    unsafe fn setNodeName_(&self, nodeName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNodeName : nodeName)
    }
    unsafe fn comment(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, comment)
    }
    unsafe fn setComment_(&self, comment: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComment : comment)
    }
    unsafe fn defaultMappings(&self) -> ODMappings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultMappings)
    }
    unsafe fn setDefaultMappings_(&self, defaultMappings: ODMappings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultMappings : defaultMappings)
    }
    unsafe fn templateName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, templateName)
    }
    unsafe fn setTemplateName_(&self, templateName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTemplateName : templateName)
    }
    unsafe fn virtualSubnodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, virtualSubnodes)
    }
    unsafe fn setVirtualSubnodes_(&self, virtualSubnodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVirtualSubnodes : virtualSubnodes)
    }
    unsafe fn hideRegistration(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hideRegistration)
    }
    unsafe fn setHideRegistration_(&self, hideRegistration: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHideRegistration : hideRegistration)
    }
    unsafe fn preferredDestinationHostName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDestinationHostName)
    }
    unsafe fn setPreferredDestinationHostName_(&self, preferredDestinationHostName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredDestinationHostName : preferredDestinationHostName)
    }
    unsafe fn preferredDestinationHostPort(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDestinationHostPort)
    }
    unsafe fn setPreferredDestinationHostPort_(&self, preferredDestinationHostPort: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredDestinationHostPort : preferredDestinationHostPort)
    }
    unsafe fn trustAccount(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trustAccount)
    }
    unsafe fn trustMetaAccount(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trustMetaAccount)
    }
    unsafe fn trustKerberosPrincipal(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trustKerberosPrincipal)
    }
    unsafe fn trustType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trustType)
    }
    unsafe fn trustUsesMutualAuthentication(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trustUsesMutualAuthentication)
    }
    unsafe fn trustUsesKerberosKeytab(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trustUsesKerberosKeytab)
    }
    unsafe fn trustUsesSystemKeychain(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trustUsesSystemKeychain)
    }
    unsafe fn packetSigning(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, packetSigning)
    }
    unsafe fn setPacketSigning_(&self, packetSigning: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPacketSigning : packetSigning)
    }
    unsafe fn packetEncryption(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, packetEncryption)
    }
    unsafe fn setPacketEncryption_(&self, packetEncryption: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPacketEncryption : packetEncryption)
    }
    unsafe fn manInTheMiddleProtection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manInTheMiddleProtection)
    }
    unsafe fn setManInTheMiddleProtection_(&self, manInTheMiddleProtection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManInTheMiddleProtection : manInTheMiddleProtection)
    }
    unsafe fn queryTimeoutInSeconds(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queryTimeoutInSeconds)
    }
    unsafe fn setQueryTimeoutInSeconds_(&self, queryTimeoutInSeconds: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueryTimeoutInSeconds : queryTimeoutInSeconds)
    }
    unsafe fn connectionSetupTimeoutInSeconds(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionSetupTimeoutInSeconds)
    }
    unsafe fn setConnectionSetupTimeoutInSeconds_(&self, connectionSetupTimeoutInSeconds: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectionSetupTimeoutInSeconds : connectionSetupTimeoutInSeconds)
    }
    unsafe fn connectionIdleTimeoutInSeconds(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionIdleTimeoutInSeconds)
    }
    unsafe fn setConnectionIdleTimeoutInSeconds_(&self, connectionIdleTimeoutInSeconds: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectionIdleTimeoutInSeconds : connectionIdleTimeoutInSeconds)
    }
    unsafe fn defaultModuleEntries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultModuleEntries)
    }
    unsafe fn setDefaultModuleEntries_(&self, defaultModuleEntries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultModuleEntries : defaultModuleEntries)
    }
    unsafe fn authenticationModuleEntries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationModuleEntries)
    }
    unsafe fn setAuthenticationModuleEntries_(&self, authenticationModuleEntries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthenticationModuleEntries : authenticationModuleEntries)
    }
    unsafe fn discoveryModuleEntries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveryModuleEntries)
    }
    unsafe fn setDiscoveryModuleEntries_(&self, discoveryModuleEntries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiscoveryModuleEntries : discoveryModuleEntries)
    }
    unsafe fn generalModuleEntries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, generalModuleEntries)
    }
    unsafe fn setGeneralModuleEntries_(&self, generalModuleEntries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeneralModuleEntries : generalModuleEntries)
    }
    unsafe fn configuration() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODConfiguration").unwrap(), configuration)
    }
    unsafe fn suggestedTrustAccount_(hostname: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODConfiguration").unwrap(), suggestedTrustAccount : hostname)
    }
    unsafe fn suggestedTrustPassword_(length: usize) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODConfiguration").unwrap(), suggestedTrustPassword : length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODMappings(pub id);
impl std::ops::Deref for ODMappings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODMappings {}
impl ODMappings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODMappings").unwrap(), alloc) })
    }
}
impl INSObject for ODMappings {}
impl PNSObject for ODMappings {}
impl std::convert::TryFrom<NSObject> for ODMappings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODMappings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODMappings").unwrap()) };
        if is_kind_of {
            Ok(ODMappings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODMappings")
        }
    }
}
impl IODMappings for ODMappings {}
pub trait IODMappings: Sized + std::ops::Deref {
    unsafe fn recordMapForStandardRecordType_(&self, stdType: NSString) -> ODRecordMap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordMapForStandardRecordType : stdType)
    }
    unsafe fn setRecordMap_forStandardRecordType_(&self, map: ODRecordMap, stdType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecordMap : map, forStandardRecordType : stdType)
    }
    unsafe fn comment(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, comment)
    }
    unsafe fn setComment_(&self, comment: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComment : comment)
    }
    unsafe fn templateName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, templateName)
    }
    unsafe fn setTemplateName_(&self, templateName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTemplateName : templateName)
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
    unsafe fn recordTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordTypes)
    }
    unsafe fn function(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, function)
    }
    unsafe fn setFunction_(&self, function: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunction : function)
    }
    unsafe fn functionAttributes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionAttributes)
    }
    unsafe fn setFunctionAttributes_(&self, functionAttributes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionAttributes : functionAttributes)
    }
    unsafe fn mappings() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODMappings").unwrap(), mappings)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODRecordMap(pub id);
impl std::ops::Deref for ODRecordMap {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODRecordMap {}
impl ODRecordMap {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODRecordMap").unwrap(), alloc) })
    }
}
impl INSObject for ODRecordMap {}
impl PNSObject for ODRecordMap {}
impl std::convert::TryFrom<NSObject> for ODRecordMap {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODRecordMap, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODRecordMap").unwrap()) };
        if is_kind_of {
            Ok(ODRecordMap(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODRecordMap")
        }
    }
}
impl IODRecordMap for ODRecordMap {}
pub trait IODRecordMap: Sized + std::ops::Deref {
    unsafe fn attributeMapForStandardAttribute_(
        &self,
        standardAttribute: NSString,
    ) -> ODAttributeMap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributeMapForStandardAttribute : standardAttribute)
    }
    unsafe fn setAttributeMap_forStandardAttribute_(
        &self,
        attributeMap: ODAttributeMap,
        standardAttribute: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeMap : attributeMap, forStandardAttribute : standardAttribute)
    }
    unsafe fn native(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, native)
    }
    unsafe fn setNative_(&self, native: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNative : native)
    }
    unsafe fn odPredicate(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, odPredicate)
    }
    unsafe fn setOdPredicate_(&self, odPredicate: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOdPredicate : odPredicate)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn standardAttributeTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, standardAttributeTypes)
    }
    unsafe fn recordMap() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODRecordMap").unwrap(), recordMap)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODAttributeMap(pub id);
impl std::ops::Deref for ODAttributeMap {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODAttributeMap {}
impl ODAttributeMap {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODAttributeMap").unwrap(), alloc) })
    }
}
impl INSObject for ODAttributeMap {}
impl PNSObject for ODAttributeMap {}
impl std::convert::TryFrom<NSObject> for ODAttributeMap {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODAttributeMap, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODAttributeMap").unwrap()) };
        if is_kind_of {
            Ok(ODAttributeMap(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODAttributeMap")
        }
    }
}
impl IODAttributeMap for ODAttributeMap {}
pub trait IODAttributeMap: Sized + std::ops::Deref {
    unsafe fn setStaticValue_(&self, staticValue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStaticValue : staticValue)
    }
    unsafe fn setVariableSubstitution_(&self, variableSubstitution: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVariableSubstitution : variableSubstitution)
    }
    unsafe fn customQueryFunction(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customQueryFunction)
    }
    unsafe fn setCustomQueryFunction_(&self, customQueryFunction: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomQueryFunction : customQueryFunction)
    }
    unsafe fn customTranslationFunction(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customTranslationFunction)
    }
    unsafe fn setCustomTranslationFunction_(&self, customTranslationFunction: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomTranslationFunction : customTranslationFunction)
    }
    unsafe fn customAttributes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customAttributes)
    }
    unsafe fn setCustomAttributes_(&self, customAttributes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomAttributes : customAttributes)
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
    unsafe fn attributeMapWithValue_(value: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODAttributeMap").unwrap(), attributeMapWithValue : value)
    }
    unsafe fn attributeMapWithStaticValue_(staticValue: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODAttributeMap").unwrap(), attributeMapWithStaticValue : staticValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ODModuleEntry(pub id);
impl std::ops::Deref for ODModuleEntry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ODModuleEntry {}
impl ODModuleEntry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ODModuleEntry").unwrap(), alloc) })
    }
}
impl INSObject for ODModuleEntry {}
impl PNSObject for ODModuleEntry {}
impl std::convert::TryFrom<NSObject> for ODModuleEntry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ODModuleEntry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ODModuleEntry").unwrap()) };
        if is_kind_of {
            Ok(ODModuleEntry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ODModuleEntry")
        }
    }
}
impl IODModuleEntry for ODModuleEntry {}
pub trait IODModuleEntry: Sized + std::ops::Deref {
    unsafe fn setOption_value_(&self, optionName: NSString, value: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOption : optionName, value : value)
    }
    unsafe fn option_(&self, optionName: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, option : optionName)
    }
    unsafe fn mappings(&self) -> ODMappings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mappings)
    }
    unsafe fn setMappings_(&self, mappings: ODMappings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMappings : mappings)
    }
    unsafe fn supportedOptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedOptions)
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
    unsafe fn xpcServiceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xpcServiceName)
    }
    unsafe fn setXpcServiceName_(&self, xpcServiceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXpcServiceName : xpcServiceName)
    }
    unsafe fn uuidString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uuidString)
    }
    unsafe fn setUuidString_(&self, uuidString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUuidString : uuidString)
    }
    unsafe fn moduleEntryWithName_xpcServiceName_(
        name: NSString,
        xpcServiceName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ODModuleEntry").unwrap(), moduleEntryWithName : name, xpcServiceName : xpcServiceName)
    }
}
unsafe extern "C" {
    pub static kODErrorDomainFramework: CFStringRef;
}
unsafe extern "C" {
    pub static kODSessionProxyAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kODSessionProxyPort: CFStringRef;
}
unsafe extern "C" {
    pub static kODSessionProxyUsername: CFStringRef;
}
unsafe extern "C" {
    pub static kODSessionProxyPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kODModuleConfigOptionQueryTimeout: CFStringRef;
}
unsafe extern "C" {
    pub static kODModuleConfigOptionConnectionSetupTimeout: CFStringRef;
}
unsafe extern "C" {
    pub static kODModuleConfigOptionConnectionIdleDisconnect: CFStringRef;
}
unsafe extern "C" {
    pub static kODModuleConfigOptionPacketSigning: CFStringRef;
}
unsafe extern "C" {
    pub static kODModuleConfigOptionPacketEncryption: CFStringRef;
}
unsafe extern "C" {
    pub static kODModuleConfigOptionManInTheMiddle: CFStringRef;
}
unsafe extern "C" {
    pub static kODNodeOptionsQuerySkippedSubnode: CFStringRef;
}
unsafe extern "C" {
    pub static kODRecordTypeAttributeTypes: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeAFPServer: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeAliases: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeAugments: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeAutomount: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeAutomountMap: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeAutoServerSetup: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeBootp: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeCertificateAuthorities: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeComputerLists: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeComputerGroups: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeComputers: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeConfiguration: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeEthernets: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeFileMakerServers: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeFTPServer: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeGroups: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeHostServices: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeHosts: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeLDAPServer: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeLocations: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeMounts: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeNFS: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeNetDomains: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeNetGroups: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeNetworks: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePeople: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePresetComputers: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePresetComputerGroups: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePresetComputerLists: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePresetGroups: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePresetUsers: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePrintService: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePrintServiceUser: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypePrinters: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeProtocols: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeQTSServer: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeQueryInformation: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeRecordTypes: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeResources: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeRPC: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeSMBServer: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeServer: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeServices: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeSharePoints: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeUsers: ODRecordType;
}
unsafe extern "C" {
    pub static kODRecordTypeWebServer: ODRecordType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAllAttributes: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeStandardOnly: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNativeOnly: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMetaAmbiguousName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMetaAugmentedAttributes: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMetaRecordName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAdminLimits: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAltSecurityIdentities: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAuthenticationHint: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAllTypes: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAuthorityRevocationList: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeBirthday: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCACertificate: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCapacity: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCertificateRevocationList: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeComment: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeContactGUID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeContactPerson: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCreationTimestamp: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCrossCertificatePair: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeDataStamp: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeFullName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeDNSDomain: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeDNSNameServer: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeENetAddress: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeExpire: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeFirstName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeGUID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeHardwareUUID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeHomeDirectoryQuota: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeHomeDirectorySoftQuota: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeHomeLocOwner: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeInternetAlias: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeKDCConfigData: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeKerberosServices: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeLastName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeLDAPSearchBaseSuffix: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeLocation: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMapGUID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMCXFlags: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMCXSettings: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMailAttribute: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMetaAutomountMap: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMiddleName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeModificationTimestamp: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNFSHomeDirectory: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNote: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOperatingSystem: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOperatingSystemVersion: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOwner: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOwnerGUID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePassword: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePasswordPlus: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePasswordPolicyOptions: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePasswordServerList: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePasswordServerLocation: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePicture: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePort: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePresetUserIsAdmin: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrimaryComputerGUID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrimaryComputerList: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrimaryGroupID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrinter1284DeviceID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrinterLPRHost: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrinterLPRQueue: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrinterMakeAndModel: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrinterType: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrinterURI: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrinterXRISupported: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrintServiceInfoText: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrintServiceInfoXML: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrintServiceUserData: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeRealUserID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeRelativeDNPrefix: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBAcctFlags: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBGroupRID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBHome: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBHomeDrive: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBKickoffTime: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBLogoffTime: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBLogonTime: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBPrimaryGroupSID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBPWDLastSet: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBProfilePath: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBRID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBScriptPath: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBSID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSMBUserWorkstations: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeServiceType: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSetupAdvertising: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSetupAutoRegister: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSetupLocation: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSetupOccupation: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeTimeToLive: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeTrustInformation: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeUniqueID: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeUserCertificate: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeUserPKCS12Data: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeUserShell: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeUserSMIMECertificate: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeVFSDumpFreq: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeVFSLinkDir: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeVFSPassNo: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeVFSType: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeWeblogURI: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeXMLPlist: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeProtocolNumber: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeRPCNumber: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNetworkNumber: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAccessControlEntry: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAddressLine1: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAddressLine2: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAddressLine3: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAreaCode: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAuthenticationAuthority: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAutomountInformation: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeBootParams: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeBuilding: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeServicesLocator: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCity: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCompany: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeComputers: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCountry: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeDepartment: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeDNSName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeEMailAddress: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeEMailContacts: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeFaxNumber: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeGroup: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeGroupMembers: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeGroupMembership: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeGroupServices: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeHomePhoneNumber: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeHTML: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeHomeDirectory: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeIMHandle: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeIPAddress: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeIPAddressAndENetAddress: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeIPv6Address: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeJPEGPhoto: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeJobTitle: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeKDCAuthKey: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeKeywords: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeLDAPReadReplicas: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeLDAPWriteReplicas: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMapCoordinates: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMapURI: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMIME: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMobileNumber: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNestedGroups: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNetGroups: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNickName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOrganizationInfo: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOrganizationName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePagerNumber: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePhoneContacts: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePhoneNumber: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePGPPublicKey: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePostalAddress: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePostalAddressContacts: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePostalCode: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNamePrefix: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeProfiles: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeProfilesTimestamp: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeProtocols: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeRecordName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeRelationships: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeResourceInfo: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeResourceType: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeState: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeStreet: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNameSuffix: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeURL: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeVFSOpts: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAlias: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAuthCredential: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCopyTimestamp: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeDateRecordCreated: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeKerberosRealm: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNTDomainComputerAccount: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOriginalHomeDirectory: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOriginalNFSHomeDirectory: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeOriginalNodeName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrimaryNTDomain: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePwdAgingPolicy: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeReadOnlyNode: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeTimePackage: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeTotalSize: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAuthMethod: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeMetaNodeLocation: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNodePath: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePlugInInfo: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeRecordType: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSchema: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSubNodes: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNetGroupTriplet: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSearchPath: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeSearchPolicy: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAutomaticSearchPath: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeLocalOnlySearchPath: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCustomSearchPath: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNodeOptions: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNodeSASLRealm: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAdvertisedServices: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeLocaleRelay: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeLocaleSubnets: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNetworkInterfaces: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeParentLocales: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePrimaryLocale: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeBuildVersion: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeConfigAvailable: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeConfigFile: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeCoreFWVersion: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeFunctionalState: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeFWVersion: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePluginIndex: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNumTableList: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeVersion: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypePIDValue: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeProcessName: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeTotalRefCount: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeDirRefCount: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNodeRefCount: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeRecRefCount: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAttrListRefCount: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAttrListValueRefCount: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeDirRefs: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeNodeRefs: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeRecRefs: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAttrListRefs: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAttributeTypeAttrListValueRefs: ODAttributeType;
}
unsafe extern "C" {
    pub static kODAuthenticationType2WayRandom: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationType2WayRandomChangePasswd: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeAPOP: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeCRAM_MD5: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeChangePasswd: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeClearText: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeCrypt: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeDIGEST_MD5: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeDeleteUser: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeGetEffectivePolicy: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeGetGlobalPolicy: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeGetKerberosPrincipal: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeGetPolicy: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeGetUserData: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeGetUserName: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeKerberosTickets: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeMPPEPrimaryKeys: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeMPPEMasterKeys: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeMSCHAP2: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeNTLMv2: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeNTLMv2WithSessionKey: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeNewUser: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeNewUserWithPolicy: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeNodeNativeClearTextOK: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeNodeNativeNoClearText: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeReadSecureHash: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSMBNTv2UserSessionKey: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSMBWorkstationCredentialSessionKey: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSMB_LM_Key: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSMB_NT_Key: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSMB_NT_UserSessionKey: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSMB_NT_WithUserSessionKey: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSecureHash: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetGlobalPolicy: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetLMHash: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetNTHash: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetPassword: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetPasswordAsCurrent: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetPolicy: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetPolicyAsCurrent: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetUserData: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetUserName: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeSetWorkstationPassword: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeWithAuthorizationRef: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODAuthenticationTypeWriteSecureHash: ODAuthenticationType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordCannotBeAccountName: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordChangeRequired: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordHistory: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordMinimumNumberOfCharacters: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordMaximumNumberOfCharacters: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordMaximumAgeInMinutes: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordRequiresAlpha: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordRequiresMixedCase: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordRequiresNumeric: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordRequiresSymbol: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypePasswordSelfModification: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypeAccountExpiresOnDate: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypeAccountMaximumFailedLogins: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypeAccountMaximumMinutesUntilDisabled: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypeAccountMinutesUntilFailedLoginReset: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyTypeAccountMaximumMinutesOfNonUse: ODPolicyType;
}
unsafe extern "C" {
    pub static kODPolicyKeyIdentifier: ODPolicyKeyType;
}
unsafe extern "C" {
    pub static kODPolicyKeyParameters: ODPolicyKeyType;
}
unsafe extern "C" {
    pub static kODPolicyKeyContent: ODPolicyKeyType;
}
unsafe extern "C" {
    pub static kODPolicyKeyContentDescription: ODPolicyKeyType;
}
unsafe extern "C" {
    pub static kODPolicyKeyEvaluationDetails: ODPolicyKeyType;
}
unsafe extern "C" {
    pub static kODPolicyKeyPolicySatisfied: ODPolicyKeyType;
}
unsafe extern "C" {
    pub static mut kODPolicyCategoryAuthentication: ODPolicyCategoryType;
}
unsafe extern "C" {
    pub static mut kODPolicyCategoryPasswordContent: ODPolicyCategoryType;
}
unsafe extern "C" {
    pub static mut kODPolicyCategoryPasswordChange: ODPolicyCategoryType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeRecordName: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeRecordType: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributePassword: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributePasswordHashes: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributePasswordHistory: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributePasswordHistoryDepth: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeCurrentDate: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeCurrentTime: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeCurrentTimeOfDay: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeCurrentDayOfWeek: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeFailedAuthentications: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeMaximumFailedAuthentications: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeLastFailedAuthenticationTime: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeLastAuthenticationTime: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeLastPasswordChangeTime: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeNewPasswordRequiredTime: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeCreationTime: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeExpiresEveryNDays: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeEnableOnDate: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeExpiresOnDate: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeEnableOnDayOfWeek: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeExpiresOnDayOfWeek: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeEnableAtTimeOfDay: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeExpiresAtTimeOfDay: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODPolicyAttributeDaysUntilExpiration: ODPolicyAttributeType;
}
unsafe extern "C" {
    pub static mut kODBackOffSeconds: ODErrorUserInfoKeyType;
}
unsafe extern "C" {
    pub fn ODContextGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn ODNodeGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn ODNodeCreateWithNodeType(
        allocator: CFAllocatorRef,
        session: ODSessionRef,
        nodeType: ODNodeType,
        error: *mut CFErrorRef,
    ) -> ODNodeRef;
}
unsafe extern "C" {
    pub fn ODNodeCreateWithName(
        allocator: CFAllocatorRef,
        session: ODSessionRef,
        nodeName: CFStringRef,
        error: *mut CFErrorRef,
    ) -> ODNodeRef;
}
unsafe extern "C" {
    pub fn ODNodeCreateCopy(
        allocator: CFAllocatorRef,
        node: ODNodeRef,
        error: *mut CFErrorRef,
    ) -> ODNodeRef;
}
unsafe extern "C" {
    pub fn ODNodeCopySubnodeNames(node: ODNodeRef, error: *mut CFErrorRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ODNodeCopyUnreachableSubnodeNames(node: ODNodeRef, error: *mut CFErrorRef)
        -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ODNodeGetName(node: ODNodeRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn ODNodeCopyDetails(
        node: ODNodeRef,
        keys: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODNodeCopySupportedRecordTypes(node: ODNodeRef, error: *mut CFErrorRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ODNodeCopySupportedAttributes(
        node: ODNodeRef,
        recordType: NSString,
        error: *mut CFErrorRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ODNodeSetCredentials(
        node: ODNodeRef,
        recordType: NSString,
        recordName: CFStringRef,
        password: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeSetCredentialsExtended(
        node: ODNodeRef,
        recordType: NSString,
        authType: NSString,
        authItems: CFArrayRef,
        outAuthItems: *mut CFArrayRef,
        outContext: *mut ODContextRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeSetCredentialsUsingKerberosCache(
        node: ODNodeRef,
        cacheName: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeCreateRecord(
        node: ODNodeRef,
        recordType: NSString,
        recordName: CFStringRef,
        attributeDict: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> ODRecordRef;
}
unsafe extern "C" {
    pub fn ODNodeCopyRecord(
        node: ODNodeRef,
        recordType: NSString,
        recordName: CFStringRef,
        attributes: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> ODRecordRef;
}
unsafe extern "C" {
    pub fn ODNodeCustomCall(
        node: ODNodeRef,
        customCode: CFIndex,
        data: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn ODNodeCustomFunction(
        node: ODNodeRef,
        function: CFStringRef,
        payload: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn ODNodeCopyPolicies(node: ODNodeRef, error: *mut CFErrorRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODNodeCopySupportedPolicies(node: ODNodeRef, error: *mut CFErrorRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODNodeSetPolicies(
        node: ODNodeRef,
        policies: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeSetPolicy(
        node: ODNodeRef,
        policyType: NSString,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeRemovePolicy(
        node: ODNodeRef,
        policyType: NSString,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeAddAccountPolicy(
        node: ODNodeRef,
        policy: CFDictionaryRef,
        category: NSString,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeRemoveAccountPolicy(
        node: ODNodeRef,
        policy: CFDictionaryRef,
        category: NSString,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeSetAccountPolicies(
        node: ODNodeRef,
        policies: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODNodeCopyAccountPolicies(node: ODNodeRef, error: *mut CFErrorRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODNodePasswordContentCheck(
        node: ODNodeRef,
        password: CFStringRef,
        recordName: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODQueryGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn ODQueryCreateWithNode(
        allocator: CFAllocatorRef,
        node: ODNodeRef,
        recordTypeOrList: CFTypeRef,
        attribute: NSString,
        matchType: ODMatchType,
        queryValueOrList: CFTypeRef,
        returnAttributeOrList: CFTypeRef,
        maxResults: CFIndex,
        error: *mut CFErrorRef,
    ) -> ODQueryRef;
}
unsafe extern "C" {
    pub fn ODQueryCreateWithNodeType(
        allocator: CFAllocatorRef,
        nodeType: ODNodeType,
        recordTypeOrList: CFTypeRef,
        attribute: NSString,
        matchType: ODMatchType,
        queryValueOrList: CFTypeRef,
        returnAttributeOrList: CFTypeRef,
        maxResults: CFIndex,
        error: *mut CFErrorRef,
    ) -> ODQueryRef;
}
unsafe extern "C" {
    pub fn ODQueryCopyResults(
        query: ODQueryRef,
        allowPartialResults: bool,
        error: *mut CFErrorRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ODQuerySynchronize(query: ODQueryRef);
}
unsafe extern "C" {
    pub fn ODQueryScheduleWithRunLoop(
        query: ODQueryRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn ODQueryUnscheduleFromRunLoop(
        query: ODQueryRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn ODQuerySetDispatchQueue(query: ODQueryRef, queue: NSObject);
}
unsafe extern "C" {
    pub fn ODRecordGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn ODRecordSetNodeCredentials(
        record: ODRecordRef,
        username: CFStringRef,
        password: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordSetNodeCredentialsExtended(
        record: ODRecordRef,
        recordType: NSString,
        authType: NSString,
        authItems: CFArrayRef,
        outAuthItems: *mut CFArrayRef,
        outContext: *mut ODContextRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordSetNodeCredentialsUsingKerberosCache(
        record: ODRecordRef,
        cacheName: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordCopyPasswordPolicy(
        allocator: CFAllocatorRef,
        record: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODRecordVerifyPassword(
        record: ODRecordRef,
        password: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordVerifyPasswordExtended(
        record: ODRecordRef,
        authType: NSString,
        authItems: CFArrayRef,
        outAuthItems: *mut CFArrayRef,
        outContext: *mut ODContextRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordChangePassword(
        record: ODRecordRef,
        oldPassword: CFStringRef,
        newPassword: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordGetRecordType(record: ODRecordRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn ODRecordGetRecordName(record: ODRecordRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn ODRecordCopyValues(
        record: ODRecordRef,
        attribute: NSString,
        error: *mut CFErrorRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ODRecordSetValue(
        record: ODRecordRef,
        attribute: NSString,
        valueOrValues: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordAddValue(
        record: ODRecordRef,
        attribute: NSString,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordRemoveValue(
        record: ODRecordRef,
        attribute: NSString,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordCopyDetails(
        record: ODRecordRef,
        attributes: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODRecordSynchronize(record: ODRecordRef, error: *mut CFErrorRef) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordDelete(record: ODRecordRef, error: *mut CFErrorRef) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordAddMember(
        group: ODRecordRef,
        member: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordRemoveMember(
        group: ODRecordRef,
        member: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordContainsMember(
        group: ODRecordRef,
        member: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordCopyPolicies(record: ODRecordRef, error: *mut CFErrorRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODRecordCopyEffectivePolicies(
        record: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODRecordCopySupportedPolicies(
        record: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODRecordSetPolicies(
        record: ODRecordRef,
        policies: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordSetPolicy(
        record: ODRecordRef,
        policy: NSString,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordRemovePolicy(
        record: ODRecordRef,
        policy: NSString,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordAddAccountPolicy(
        record: ODRecordRef,
        policy: CFDictionaryRef,
        category: NSString,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordRemoveAccountPolicy(
        record: ODRecordRef,
        policy: CFDictionaryRef,
        category: NSString,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordSetAccountPolicies(
        record: ODRecordRef,
        policies: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordCopyAccountPolicies(
        record: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ODRecordAuthenticationAllowed(record: ODRecordRef, error: *mut CFErrorRef) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordPasswordChangeAllowed(
        record: ODRecordRef,
        newPassword: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordWillPasswordExpire(record: ODRecordRef, willExpireIn: u64) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordWillAuthenticationsExpire(record: ODRecordRef, willExpireIn: u64) -> bool;
}
unsafe extern "C" {
    pub fn ODRecordSecondsUntilPasswordExpires(record: ODRecordRef) -> i64;
}
unsafe extern "C" {
    pub fn ODRecordSecondsUntilAuthenticationsExpire(record: ODRecordRef) -> i64;
}
unsafe extern "C" {
    pub static mut kODSessionDefault: ODSessionRef;
}
unsafe extern "C" {
    pub fn ODSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn ODSessionCreate(
        allocator: CFAllocatorRef,
        options: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> ODSessionRef;
}
unsafe extern "C" {
    pub fn ODSessionCopyNodeNames(
        allocator: CFAllocatorRef,
        session: ODSessionRef,
        error: *mut CFErrorRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub static ODFrameworkErrorDomain: NSString;
}
unsafe extern "C" {
    pub static ODSessionProxyAddress: NSString;
}
unsafe extern "C" {
    pub static ODSessionProxyPort: NSString;
}
unsafe extern "C" {
    pub static ODSessionProxyUsername: NSString;
}
unsafe extern "C" {
    pub static ODSessionProxyPassword: NSString;
}
unsafe extern "C" {
    pub static ODTrustTypeJoined: NSString;
}
unsafe extern "C" {
    pub static ODTrustTypeUsingCredentials: NSString;
}
unsafe extern "C" {
    pub static ODTrustTypeAnonymous: NSString;
}

unsafe impl objc2::encode::RefEncode for __ODContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ODContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ODContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __ODNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ODNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ODNode", &[]);
}
unsafe impl objc2::encode::RefEncode for __ODQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ODQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ODQuery", &[]);
}
unsafe impl objc2::encode::RefEncode for __ODRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ODRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ODRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for __ODSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ODSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ODSession", &[]);
}
unsafe impl objc2::encode::RefEncode for ODSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ODRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ODNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ODQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ODConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ODMappings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODMappings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ODRecordMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODRecordMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ODAttributeMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODAttributeMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ODModuleEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ODModuleEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
