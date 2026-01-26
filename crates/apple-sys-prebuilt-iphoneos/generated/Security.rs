#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CFNetwork::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct audit_token_t {
    pub val: [::std::os::raw::c_uint; 8usize],
}
pub trait IClass: Sized + std::ops::Deref {}
pub trait IProtocol: Sized + std::ops::Deref {}
pub type IMP = ::std::option::Option<unsafe extern "C" fn()>;
pub type NSUInteger = ::std::os::raw::c_ulong;
pub trait PNSObject: Sized + std::ops::Deref {
    unsafe fn isEqual_(&self, object: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqual : object)
    }
    unsafe fn class(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, class)
    }
    unsafe fn self_(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, self_)
    }
    unsafe fn performSelector_(&self, aSelector: objc2::runtime::Sel) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSelector : aSelector)
    }
    unsafe fn performSelector_withObject_(&self, aSelector: objc2::runtime::Sel, object: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSelector : aSelector, withObject : object)
    }
    unsafe fn performSelector_withObject_withObject_(
        &self,
        aSelector: objc2::runtime::Sel,
        object1: id,
        object2: id,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSelector : aSelector, withObject : object1, withObject : object2)
    }
    unsafe fn isProxy(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isProxy)
    }
    unsafe fn isKindOfClass_(&self, aClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isKindOfClass : aClass)
    }
    unsafe fn isMemberOfClass_(&self, aClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isMemberOfClass : aClass)
    }
    unsafe fn conformsToProtocol_(&self, aProtocol: Protocol) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, conformsToProtocol : aProtocol)
    }
    unsafe fn respondsToSelector_(&self, aSelector: objc2::runtime::Sel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondsToSelector : aSelector)
    }
    unsafe fn retain(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retain)
    }
    unsafe fn release(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, release)
    }
    unsafe fn autorelease(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autorelease)
    }
    unsafe fn retainCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retainCount)
    }
    unsafe fn zone(&self) -> *mut _NSZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zone)
    }
    unsafe fn hash(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hash)
    }
    unsafe fn superclass(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, superclass)
    }
    unsafe fn description(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, description)
    }
    unsafe fn debugDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, debugDescription)
    }
}
pub trait INSObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn dealloc(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dealloc)
    }
    unsafe fn finalize(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finalize)
    }
    unsafe fn copy(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, copy)
    }
    unsafe fn mutableCopy(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mutableCopy)
    }
    unsafe fn methodForSelector_(&self, aSelector: objc2::runtime::Sel) -> IMP
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, methodForSelector : aSelector)
    }
    unsafe fn doesNotRecognizeSelector_(&self, aSelector: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, doesNotRecognizeSelector : aSelector)
    }
    unsafe fn forwardingTargetForSelector_(&self, aSelector: objc2::runtime::Sel) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, forwardingTargetForSelector : aSelector)
    }
    unsafe fn forwardInvocation_(&self, anInvocation: NSInvocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, forwardInvocation : anInvocation)
    }
    unsafe fn methodSignatureForSelector_(&self, aSelector: objc2::runtime::Sel) -> NSMethodSignature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, methodSignatureForSelector : aSelector)
    }
    unsafe fn allowsWeakReference(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsWeakReference)
    }
    unsafe fn retainWeakReference(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retainWeakReference)
    }
    unsafe fn load()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), load)
    }
    unsafe fn initialize()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), initialize)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), new)
    }
    unsafe fn allocWithZone_(zone: *mut _NSZone) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), allocWithZone : zone)
    }
    unsafe fn alloc() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), alloc)
    }
    unsafe fn copyWithZone_(zone: *mut _NSZone) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), copyWithZone : zone)
    }
    unsafe fn mutableCopyWithZone_(zone: *mut _NSZone) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), mutableCopyWithZone : zone)
    }
    unsafe fn instancesRespondToSelector_(aSelector: objc2::runtime::Sel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), instancesRespondToSelector : aSelector)
    }
    unsafe fn conformsToProtocol_(protocol: Protocol) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), conformsToProtocol : protocol)
    }
    unsafe fn instanceMethodForSelector_(aSelector: objc2::runtime::Sel) -> IMP
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), instanceMethodForSelector : aSelector)
    }
    unsafe fn instanceMethodSignatureForSelector_(
        aSelector: objc2::runtime::Sel,
    ) -> NSMethodSignature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), instanceMethodSignatureForSelector : aSelector)
    }
    unsafe fn isSubclassOfClass_(aClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), isSubclassOfClass : aClass)
    }
    unsafe fn hash() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), hash)
    }
    unsafe fn superclass() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), superclass)
    }
    unsafe fn class() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), class)
    }
    unsafe fn description() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), description)
    }
    unsafe fn debugDescription() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), debugDescription)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OS_object(pub id);
impl std::ops::Deref for OS_object {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OS_object {}
impl OS_object {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OS_object").unwrap(), alloc) })
    }
}
impl INSObject for OS_object {}
impl PNSObject for OS_object {}
impl std::convert::TryFrom<NSObject> for OS_object {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OS_object, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OS_object").unwrap()) };
        if is_kind_of {
            Ok(OS_object(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OS_object")
        }
    }
}
impl IOS_object for OS_object {}
pub trait IOS_object: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OS_os_workgroup(pub id);
impl std::ops::Deref for OS_os_workgroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OS_os_workgroup {}
impl OS_os_workgroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OS_os_workgroup").unwrap(), alloc) })
    }
}
impl IOS_object for OS_os_workgroup {}
impl From<OS_os_workgroup> for OS_object {
    fn from(child: OS_os_workgroup) -> OS_object {
        OS_object(child.0)
    }
}
impl std::convert::TryFrom<OS_object> for OS_os_workgroup {
    type Error = &'static str;
    fn try_from(parent: OS_object) -> Result<OS_os_workgroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OS_os_workgroup").unwrap()) };
        if is_kind_of {
            Ok(OS_os_workgroup(parent.0))
        } else {
            Err("This OS_object cannot be downcasted to OS_os_workgroup")
        }
    }
}
impl INSObject for OS_os_workgroup {}
impl PNSObject for OS_os_workgroup {}
impl IOS_os_workgroup for OS_os_workgroup {}
pub trait IOS_os_workgroup: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub trait POS_os_workgroup_interval: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OS_os_workgroup_interval(pub id);
impl std::ops::Deref for OS_os_workgroup_interval {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OS_os_workgroup_interval {}
impl OS_os_workgroup_interval {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OS_os_workgroup_interval").unwrap(), alloc) })
    }
}
impl POS_os_workgroup_interval for OS_os_workgroup_interval {}
impl IOS_os_workgroup for OS_os_workgroup_interval {}
impl From<OS_os_workgroup_interval> for OS_os_workgroup {
    fn from(child: OS_os_workgroup_interval) -> OS_os_workgroup {
        OS_os_workgroup(child.0)
    }
}
impl std::convert::TryFrom<OS_os_workgroup> for OS_os_workgroup_interval {
    type Error = &'static str;
    fn try_from(parent: OS_os_workgroup) -> Result<OS_os_workgroup_interval, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OS_os_workgroup_interval").unwrap()) };
        if is_kind_of {
            Ok(OS_os_workgroup_interval(parent.0))
        } else {
            Err("This OS_os_workgroup cannot be downcasted to OS_os_workgroup_interval")
        }
    }
}
impl IOS_object for OS_os_workgroup_interval {}
impl INSObject for OS_os_workgroup_interval {}
impl PNSObject for OS_os_workgroup_interval {}
impl IOS_os_workgroup_interval for OS_os_workgroup_interval {}
pub trait IOS_os_workgroup_interval: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub trait POS_os_workgroup_parallel: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OS_os_workgroup_parallel(pub id);
impl std::ops::Deref for OS_os_workgroup_parallel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OS_os_workgroup_parallel {}
impl OS_os_workgroup_parallel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OS_os_workgroup_parallel").unwrap(), alloc) })
    }
}
impl POS_os_workgroup_parallel for OS_os_workgroup_parallel {}
impl IOS_os_workgroup for OS_os_workgroup_parallel {}
impl std::convert::TryFrom<OS_os_workgroup> for OS_os_workgroup_parallel {
    type Error = &'static str;
    fn try_from(parent: OS_os_workgroup) -> Result<OS_os_workgroup_parallel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OS_os_workgroup_parallel").unwrap()) };
        if is_kind_of {
            Ok(OS_os_workgroup_parallel(parent.0))
        } else {
            Err("This OS_os_workgroup cannot be downcasted to OS_os_workgroup_parallel")
        }
    }
}
impl IOS_object for OS_os_workgroup_parallel {}
impl INSObject for OS_os_workgroup_parallel {}
impl PNSObject for OS_os_workgroup_parallel {}
impl IOS_os_workgroup_parallel for OS_os_workgroup_parallel {}
pub trait IOS_os_workgroup_parallel: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub type dispatch_data_t = NSObject;
pub type DERByte = u8;
pub type DERSize = usize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DERItem {
    pub data: *mut DERByte,
    pub length: DERSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecCertificate {
    _unused: [u8; 0],
}
pub type SecCertificateRef = *mut __SecCertificate;
pub type OpaqueSecCertificateRef = __SecCertificate;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecIdentity {
    _unused: [u8; 0],
}
pub type SecIdentityRef = *mut __SecIdentity;
pub type OpaqueSecIdentityRef = __SecIdentity;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecKey {
    _unused: [u8; 0],
}
pub type SecKeyRef = *mut __SecKey;
pub type OpaqueSecKeyRef = __SecKey;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecPolicy {
    _unused: [u8; 0],
}
pub type SecPolicyRef = *mut __SecPolicy;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecAccessControl {
    _unused: [u8; 0],
}
pub type SecAccessControlRef = *mut __SecAccessControl;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecKeychain {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecKeychainItem {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecKeychainSearch {
    _unused: [u8; 0],
}
pub type SecKeychainAttrType = OSType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecKeychainAttribute {
    pub tag: SecKeychainAttrType,
    pub length: UInt32,
    pub data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecKeychainAttributeList {
    pub count: UInt32,
    pub attr: *mut SecKeychainAttribute,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecTrustedApplication {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecAccess {
    _unused: [u8; 0],
}
pub type SecAccessRef = *mut __SecAccess;
pub type OpaqueSecAccessRef = __SecAccess;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecACL {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecPassword {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecKeychainAttributeInfo {
    pub count: UInt32,
    pub tag: *mut UInt32,
    pub format: *mut UInt32,
}
pub type uint32 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_data {
    pub Length: usize,
    pub Data: *mut u8,
}
pub type SecAsn1Item = cssm_data;
pub type SecAsn1Oid = cssm_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecAsn1Template_struct {
    pub kind: u32,
    pub offset: u32,
    pub sub: *const ::std::os::raw::c_void,
    pub size: u32,
}
pub type SecAsn1Template = SecAsn1Template_struct;
pub type CSSM_KEYATTR_FLAGS = uint32;
pub type CSSM_KEYUSE = uint32;
pub type SecKeyUsage = u32;
pub type SecAccessControlCreateFlags = CFOptionFlags;
pub type SecCredentialType = uint32;
pub type SecPadding = u32;
pub type SecKeySizes = u32;
pub type SecKeyAlgorithm = CFStringRef;
pub type SecKeyKeyExchangeParameter = CFStringRef;
pub type SecKeyOperationType = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecRandom {
    _unused: [u8; 0],
}
pub type SecRandomRef = *const __SecRandom;
pub type SecExternalFormat = u32;
pub type SecExternalItemType = u32;
pub type SecItemImportExportFlags = u32;
pub type SecKeyImportExportFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecKeyImportExportParameters {
    pub version: u32,
    pub flags: SecKeyImportExportFlags,
    pub passphrase: CFTypeRef,
    pub alertTitle: CFStringRef,
    pub alertPrompt: CFStringRef,
    pub accessRef: SecAccessRef,
    pub keyUsage: CSSM_KEYUSE,
    pub keyAttributes: CSSM_KEYATTR_FLAGS,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecItemImportExportKeyParameters {
    pub version: u32,
    pub flags: SecKeyImportExportFlags,
    pub passphrase: CFTypeRef,
    pub alertTitle: CFStringRef,
    pub alertPrompt: CFStringRef,
    pub accessRef: SecAccessRef,
    pub keyUsage: CFArrayRef,
    pub keyAttributes: CFArrayRef,
}
pub type SecTrustResultType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecTrust {
    _unused: [u8; 0],
}
pub type SecTrustRef = *mut __SecTrust;
pub type SecTrustCallback = *mut ::std::os::raw::c_void;
pub type SecTrustWithErrorCallback = *mut ::std::os::raw::c_void;
pub use self::SecTrustResultType as SecTrustUserSetting;
pub type SecTrustOptionFlags = u32;
pub type SSLCipherSuite = u16;
pub type SSLCiphersuiteGroup = ::std::os::raw::c_int;
pub type sec_trust_t = NSObject;
pub type sec_identity_t = NSObject;
pub type sec_certificate_t = NSObject;
pub type tls_protocol_version_t = u16;
pub type tls_ciphersuite_t = u16;
pub type tls_ciphersuite_group_t = u16;
pub type SSLProtocol = ::std::os::raw::c_int;
pub type sec_protocol_pre_shared_key_selection_t = *mut ::std::os::raw::c_void;
pub type sec_protocol_key_update_t = *mut ::std::os::raw::c_void;
pub type sec_protocol_challenge_t = *mut ::std::os::raw::c_void;
pub type sec_protocol_verify_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecCode {
    _unused: [u8; 0],
}
pub type SecCodeRef = *mut __SecCode;
pub type SecStaticCodeRef = *const __SecCode;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecRequirement {
    _unused: [u8; 0],
}
pub type SecRequirementRef = *mut __SecRequirement;
pub type SecCSFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecTask {
    _unused: [u8; 0],
}
pub type SecTaskRef = *mut __SecTask;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CMSDecoder {
    _unused: [u8; 0],
}
pub type CMSDecoderRef = *mut _CMSDecoder;
pub type CMSSignerStatus = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CMSEncoder {
    _unused: [u8; 0],
}
pub type CMSEncoderRef = *mut _CMSEncoder;
pub type CMSSignedAttributes = u32;
pub type CMSCertificateChainMode = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SSLContext {
    _unused: [u8; 0],
}
pub type SSLContextRef = *mut SSLContext;
pub type SSLConnectionRef = *const ::std::os::raw::c_void;
pub type SSLSessionOption = ::std::os::raw::c_int;
pub type SSLSessionState = ::std::os::raw::c_int;
pub type SSLClientCertificateState = ::std::os::raw::c_int;
pub type SSLReadFunc = ::std::option::Option<
    unsafe extern "C" fn(
        connection: SSLConnectionRef,
        data: *mut ::std::os::raw::c_void,
        dataLength: *mut usize,
    ) -> OSStatus,
>;
pub type SSLWriteFunc = ::std::option::Option<
    unsafe extern "C" fn(
        connection: SSLConnectionRef,
        data: *const ::std::os::raw::c_void,
        dataLength: *mut usize,
    ) -> OSStatus,
>;
pub type SSLProtocolSide = ::std::os::raw::c_int;
pub type SSLConnectionType = ::std::os::raw::c_int;
pub type SSLAuthenticate = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecAsn1Coder {
    _unused: [u8; 0],
}
pub type SecAsn1CoderRef = *mut SecAsn1Coder;
pub type instancetype = id;
unsafe extern "C" {
    pub fn SecCopyErrorMessageString(
        status: OSStatus,
        reserved: *mut ::std::os::raw::c_void,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SecCertificateGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecCertificateCreateWithData(
        allocator: CFAllocatorRef,
        data: CFDataRef,
    ) -> SecCertificateRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyData(certificate: SecCertificateRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopySubjectSummary(certificate: SecCertificateRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyCommonName(
        certificate: SecCertificateRef,
        commonName: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateCopyEmailAddresses(
        certificate: SecCertificateRef,
        emailAddresses: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateCopyNormalizedIssuerSequence(certificate: SecCertificateRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyNormalizedSubjectSequence(certificate: SecCertificateRef)
        -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyKey(certificate: SecCertificateRef) -> SecKeyRef;
}
unsafe extern "C" {
    #[link_name = "\u{1}_SecCertificateCopyPublicKey$LEGACYMAC"]
    pub fn SecCertificateCopyPublicKey(
        certificate: SecCertificateRef,
        key: *mut SecKeyRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateCopySerialNumberData(
        certificate: SecCertificateRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyNotValidBeforeDate(certificate: SecCertificateRef) -> CFDateRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyNotValidAfterDate(certificate: SecCertificateRef) -> CFDateRef;
}
unsafe extern "C" {
    #[link_name = "\u{1}_SecCertificateCopySerialNumber$LEGACYMAC"]
    pub fn SecCertificateCopySerialNumber(
        certificate: SecCertificateRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub static kSecPropertyKeyType: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyKeyLabel: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyKeyLocalizedLabel: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyKeyValue: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeWarning: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeSuccess: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeSection: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeData: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeString: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeURL: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeDate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeArray: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeNumber: CFStringRef;
}
unsafe extern "C" {
    pub fn SecIdentityGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecIdentityCreate(
        allocator: CFAllocatorRef,
        certificate: SecCertificateRef,
        privateKey: SecKeyRef,
    ) -> SecIdentityRef;
}
unsafe extern "C" {
    pub fn SecIdentityCopyCertificate(
        identityRef: SecIdentityRef,
        certificateRef: *mut SecCertificateRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecIdentityCopyPrivateKey(
        identityRef: SecIdentityRef,
        privateKeyRef: *mut SecKeyRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAccessControlGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecAccessControlCreateWithFlags(
        allocator: CFAllocatorRef,
        protection: CFTypeRef,
        flags: SecAccessControlCreateFlags,
        error: *mut CFErrorRef,
    ) -> SecAccessControlRef;
}
unsafe extern "C" {
    pub static kSecClass: CFStringRef;
}
unsafe extern "C" {
    pub static kSecClassInternetPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kSecClassGenericPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kSecClassCertificate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecClassKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecClassIdentity: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessible: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessControl: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessGroup: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrSynchronizable: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrSynchronizableAny: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCreationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrModificationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrComment: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCreator: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrType: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrLabel: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrIsInvisible: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrIsNegative: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccount: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrService: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrGeneric: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrSecurityDomain: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrServer: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocol: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationType: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPath: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrSubject: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrIssuer: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrSubjectKeyID: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPublicKeyHash: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCertificateType: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCertificateEncoding: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyClass: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrApplicationLabel: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrIsPermanent: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrIsSensitive: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrIsExtractable: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrApplicationTag: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyType: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeySizeInBits: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrEffectiveKeySize: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCanEncrypt: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCanDecrypt: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCanDerive: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCanSign: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCanVerify: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCanWrap: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrCanUnwrap: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrSyncViewHint: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrTokenID: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPersistantReference: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPersistentReference: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessibleWhenUnlocked: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessibleAfterFirstUnlock: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessibleAlways: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessibleWhenUnlockedThisDeviceOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessibleAlwaysThisDeviceOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolFTP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolFTPAccount: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolHTTP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolIRC: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolNNTP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolPOP3: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolSMTP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolSOCKS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolIMAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolLDAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolAppleTalk: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolAFP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolTelnet: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolSSH: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolFTPS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolHTTPS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolHTTPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolHTTPSProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolFTPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolSMB: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolRTSP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolRTSPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolDAAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolEPPC: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolIPP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolNNTPS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolLDAPS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolTelnetS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolIMAPS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolIRCS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrProtocolPOP3S: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationTypeNTLM: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationTypeMSN: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationTypeDPA: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationTypeRPA: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationTypeHTTPBasic: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationTypeHTTPDigest: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationTypeHTMLForm: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAuthenticationTypeDefault: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyClassPublic: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyClassPrivate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyClassSymmetric: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeRSA: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeEC: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeECSECPrimeRandom: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchPolicy: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchItemList: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchSearchList: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchIssuers: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchEmailAddressIfPresent: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchSubjectContains: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchHostOrSubdomainOfHost: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchCaseInsensitive: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchTrustedOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchValidOnDate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchLimit: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchLimitOne: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchLimitAll: CFStringRef;
}
unsafe extern "C" {
    pub static kSecReturnData: CFStringRef;
}
unsafe extern "C" {
    pub static kSecReturnAttributes: CFStringRef;
}
unsafe extern "C" {
    pub static kSecReturnRef: CFStringRef;
}
unsafe extern "C" {
    pub static kSecReturnPersistentRef: CFStringRef;
}
unsafe extern "C" {
    pub static kSecValueData: CFStringRef;
}
unsafe extern "C" {
    pub static kSecValueRef: CFStringRef;
}
unsafe extern "C" {
    pub static kSecValuePersistentRef: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseItemList: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseOperationPrompt: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseNoAuthenticationUI: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseAuthenticationUI: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseAuthenticationContext: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseDataProtectionKeychain: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseAuthenticationUIAllow: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseAuthenticationUIFail: CFStringRef;
}
unsafe extern "C" {
    pub static kSecUseAuthenticationUISkip: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrTokenIDSecureEnclave: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrAccessGroupToken: CFStringRef;
}
unsafe extern "C" {
    pub fn SecItemCopyMatching(query: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecItemAdd(attributes: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecItemUpdate(query: CFDictionaryRef, attributesToUpdate: CFDictionaryRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecItemDelete(query: CFDictionaryRef) -> OSStatus;
}
unsafe extern "C" {
    pub static kSecPrivateKeyAttrs: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPublicKeyAttrs: CFStringRef;
}
unsafe extern "C" {
    pub fn SecKeyGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecKeyGeneratePair(
        parameters: CFDictionaryRef,
        publicKey: *mut SecKeyRef,
        privateKey: *mut SecKeyRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeyCreateRandomKey(parameters: CFDictionaryRef, error: *mut CFErrorRef) -> SecKeyRef;
}
unsafe extern "C" {
    pub fn SecKeyCreateWithData(
        keyData: CFDataRef,
        attributes: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;
}
unsafe extern "C" {
    pub fn SecKeyGetBlockSize(key: SecKeyRef) -> usize;
}
unsafe extern "C" {
    pub fn SecKeyCopyExternalRepresentation(key: SecKeyRef, error: *mut CFErrorRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecKeyCopyAttributes(key: SecKeyRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SecKeyCopyPublicKey(key: SecKeyRef) -> SecKeyRef;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureRaw: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPKCS1v15Raw: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPKCS1v15SHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePKCS1v15SHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPSSSHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPSSSHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPSSSHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPSSSHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureDigestPSSSHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePSSSHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePSSSHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePSSSHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePSSSHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSASignatureMessagePSSSHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestX962: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestX962SHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestX962SHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestX962SHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestX962SHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestX962SHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageX962SHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageX962SHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageX962SHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageX962SHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageX962SHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestRFC4754: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestRFC4754SHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestRFC4754SHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestRFC4754SHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestRFC4754SHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureDigestRFC4754SHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageRFC4754SHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageRFC4754SHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageRFC4754SHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageRFC4754SHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureMessageRFC4754SHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDSASignatureRFC4754: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionRaw: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionPKCS1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA1AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA224AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA256AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA384AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmRSAEncryptionOAEPSHA512AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardX963SHA1AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardX963SHA224AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardX963SHA256AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardX963SHA384AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardX963SHA512AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA1AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA224AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA256AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA384AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorX963SHA512AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA224AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA256AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA384AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionStandardVariableIVX963SHA512AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA224AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA256AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA384AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA512AESGCM: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeStandard: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeStandardX963SHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeCofactor: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA1: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA224: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA256: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA384: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub static kSecKeyAlgorithmECDHKeyExchangeCofactorX963SHA512: SecKeyAlgorithm;
}
unsafe extern "C" {
    pub fn SecKeyCreateSignature(
        key: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        dataToSign: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecKeyVerifySignature(
        key: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        signedData: CFDataRef,
        signature: CFDataRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SecKeyCreateEncryptedData(
        key: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        plaintext: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecKeyCreateDecryptedData(
        key: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        ciphertext: CFDataRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub static kSecKeyKeyExchangeParameterRequestedSize: SecKeyKeyExchangeParameter;
}
unsafe extern "C" {
    pub static kSecKeyKeyExchangeParameterSharedInfo: SecKeyKeyExchangeParameter;
}
unsafe extern "C" {
    pub fn SecKeyCopyKeyExchangeResult(
        privateKey: SecKeyRef,
        algorithm: SecKeyAlgorithm,
        publicKey: SecKeyRef,
        parameters: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecKeyIsAlgorithmSupported(
        key: SecKeyRef,
        operation: SecKeyOperationType,
        algorithm: SecKeyAlgorithm,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static kSecPolicyAppleX509Basic: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleSSL: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleSMIME: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleEAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleIPsec: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleCodeSigning: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyMacAppStoreReceipt: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleIDValidation: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleTimeStamping: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleRevocation: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyApplePassbookSigning: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyApplePayIssuerEncryption: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleSSLServer: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleSSLClient: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleEAPServer: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleEAPClient: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleIPSecServer: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyAppleIPSecClient: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyOid: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyClient: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyRevocationFlags: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyTeamIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub fn SecPolicyGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecPolicyCopyProperties(policyRef: SecPolicyRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SecPolicyCreateBasicX509() -> SecPolicyRef;
}
unsafe extern "C" {
    pub fn SecPolicyCreateSSL(server: Boolean, hostname: CFStringRef) -> SecPolicyRef;
}
unsafe extern "C" {
    pub fn SecPolicyCreateRevocation(revocationFlags: CFOptionFlags) -> SecPolicyRef;
}
unsafe extern "C" {
    pub fn SecPolicyCreateWithProperties(
        policyIdentifier: CFTypeRef,
        properties: CFDictionaryRef,
    ) -> SecPolicyRef;
}
unsafe extern "C" {
    pub static kSecRandomDefault: SecRandomRef;
}
unsafe extern "C" {
    pub fn SecRandomCopyBytes(
        rnd: SecRandomRef,
        count: usize,
        bytes: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static kSecImportExportPassphrase: CFStringRef;
}
unsafe extern "C" {
    pub static kSecImportToMemoryOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kSecImportItemLabel: CFStringRef;
}
unsafe extern "C" {
    pub static kSecImportItemKeyID: CFStringRef;
}
unsafe extern "C" {
    pub static kSecImportItemTrust: CFStringRef;
}
unsafe extern "C" {
    pub static kSecImportItemCertChain: CFStringRef;
}
unsafe extern "C" {
    pub static kSecImportItemIdentity: CFStringRef;
}
unsafe extern "C" {
    pub fn SecPKCS12Import(
        pkcs12_data: CFDataRef,
        options: CFDictionaryRef,
        items: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kSecPropertyTypeTitle: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPropertyTypeError: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustEvaluationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustExtendedValidation: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustOrganizationName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustResultValue: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustRevocationChecked: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustRevocationValidUntilDate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustCertificateTransparency: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustCertificateTransparencyWhiteList: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustQCStatements: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTrustQWACValidation: CFStringRef;
}
unsafe extern "C" {
    pub fn SecTrustGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecTrustCreateWithCertificates(
        certificates: CFTypeRef,
        policies: CFTypeRef,
        trust: *mut SecTrustRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSetPolicies(trust: SecTrustRef, policies: CFTypeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustCopyPolicies(trust: SecTrustRef, policies: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSetNetworkFetchAllowed(trust: SecTrustRef, allowFetch: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustGetNetworkFetchAllowed(trust: SecTrustRef, allowFetch: *mut Boolean)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSetAnchorCertificates(
        trust: SecTrustRef,
        anchorCertificates: CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSetAnchorCertificatesOnly(
        trust: SecTrustRef,
        anchorCertificatesOnly: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustCopyCustomAnchorCertificates(
        trust: SecTrustRef,
        anchors: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSetVerifyDate(trust: SecTrustRef, verifyDate: CFDateRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustGetVerifyTime(trust: SecTrustRef) -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub fn SecTrustEvaluate(trust: SecTrustRef, result: *mut SecTrustResultType) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustEvaluateAsync(
        trust: SecTrustRef,
        queue: NSObject,
        result: SecTrustCallback,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustEvaluateWithError(trust: SecTrustRef, error: *mut CFErrorRef) -> bool;
}
unsafe extern "C" {
    pub fn SecTrustEvaluateAsyncWithError(
        trust: SecTrustRef,
        queue: NSObject,
        result: SecTrustWithErrorCallback,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustGetTrustResult(trust: SecTrustRef, result: *mut SecTrustResultType) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustCopyPublicKey(trust: SecTrustRef) -> SecKeyRef;
}
unsafe extern "C" {
    pub fn SecTrustCopyKey(trust: SecTrustRef) -> SecKeyRef;
}
unsafe extern "C" {
    pub fn SecTrustGetCertificateCount(trust: SecTrustRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn SecTrustGetCertificateAtIndex(trust: SecTrustRef, ix: CFIndex) -> SecCertificateRef;
}
unsafe extern "C" {
    pub fn SecTrustCopyExceptions(trust: SecTrustRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecTrustSetExceptions(trust: SecTrustRef, exceptions: CFDataRef) -> bool;
}
unsafe extern "C" {
    pub fn SecTrustCopyProperties(trust: SecTrustRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SecTrustCopyResult(trust: SecTrustRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SecTrustSetOCSPResponse(trust: SecTrustRef, responseData: CFTypeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSetSignedCertificateTimestamps(
        trust: SecTrustRef,
        sctArray: CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustCopyCertificateChain(trust: SecTrustRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub static kSecSharedPassword: CFStringRef;
}
unsafe extern "C" {
    pub fn SecAddSharedWebCredential(
        fqdn: CFStringRef,
        account: CFStringRef,
        password: CFStringRef,
        completionHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn SecRequestSharedWebCredential(
        fqdn: CFStringRef,
        account: CFStringRef,
        completionHandler: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn SecCreateSharedWebCredentialPassword() -> CFStringRef;
}
unsafe extern "C" {
    pub fn sec_retain(obj: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn sec_release(obj: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn sec_trust_create(trust: SecTrustRef) -> sec_trust_t;
}
unsafe extern "C" {
    pub fn sec_trust_copy_ref(trust: NSObject) -> SecTrustRef;
}
unsafe extern "C" {
    pub fn sec_identity_create(identity: SecIdentityRef) -> sec_identity_t;
}
unsafe extern "C" {
    pub fn sec_identity_create_with_certificates(
        identity: SecIdentityRef,
        certificates: CFArrayRef,
    ) -> sec_identity_t;
}
unsafe extern "C" {
    pub fn sec_identity_access_certificates(
        identity: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn sec_identity_copy_ref(identity: NSObject) -> SecIdentityRef;
}
unsafe extern "C" {
    pub fn sec_identity_copy_certificates_ref(identity: NSObject) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn sec_certificate_create(certificate: SecCertificateRef) -> sec_certificate_t;
}
unsafe extern "C" {
    pub fn sec_certificate_copy_ref(certificate: NSObject) -> SecCertificateRef;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_get_negotiated_protocol(
        metadata: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_copy_negotiated_protocol(
        metadata: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_copy_peer_public_key(metadata: NSObject) -> dispatch_data_t;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_get_negotiated_tls_protocol_version(
        metadata: NSObject,
    ) -> tls_protocol_version_t;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_get_negotiated_protocol_version(metadata: NSObject)
        -> SSLProtocol;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_get_negotiated_tls_ciphersuite(
        metadata: NSObject,
    ) -> tls_ciphersuite_t;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_get_negotiated_ciphersuite(metadata: NSObject) -> SSLCipherSuite;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_get_early_data_accepted(metadata: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_access_peer_certificate_chain(
        metadata: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_access_ocsp_response(
        metadata: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_access_supported_signature_algorithms(
        metadata: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_access_distinguished_names(
        metadata: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_access_pre_shared_keys(
        metadata: NSObject,
        handler: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_get_server_name(
        metadata: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_copy_server_name(
        metadata: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_peers_are_equal(metadataA: NSObject, metadataB: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_challenge_parameters_are_equal(
        metadataA: NSObject,
        metadataB: NSObject,
    ) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_create_secret(
        metadata: NSObject,
        label_len: usize,
        label: *const ::std::os::raw::c_char,
        exporter_length: usize,
    ) -> dispatch_data_t;
}
unsafe extern "C" {
    pub fn sec_protocol_metadata_create_secret_with_context(
        metadata: NSObject,
        label_len: usize,
        label: *const ::std::os::raw::c_char,
        context_len: usize,
        context: *const u8,
        exporter_length: usize,
    ) -> dispatch_data_t;
}
unsafe extern "C" {
    pub fn sec_protocol_options_are_equal(optionsA: NSObject, optionsB: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_local_identity(options: NSObject, identity: NSObject);
}
unsafe extern "C" {
    pub fn sec_protocol_options_append_tls_ciphersuite(
        options: NSObject,
        ciphersuite: tls_ciphersuite_t,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_add_tls_ciphersuite(options: NSObject, ciphersuite: SSLCipherSuite);
}
unsafe extern "C" {
    pub fn sec_protocol_options_append_tls_ciphersuite_group(
        options: NSObject,
        group: tls_ciphersuite_group_t,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_add_tls_ciphersuite_group(
        options: NSObject,
        group: SSLCiphersuiteGroup,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_min_version(options: NSObject, version: SSLProtocol);
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_min_tls_protocol_version(
        options: NSObject,
        version: tls_protocol_version_t,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_get_default_min_tls_protocol_version() -> tls_protocol_version_t;
}
unsafe extern "C" {
    pub fn sec_protocol_options_get_default_min_dtls_protocol_version() -> tls_protocol_version_t;
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_max_version(options: NSObject, version: SSLProtocol);
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_max_tls_protocol_version(
        options: NSObject,
        version: tls_protocol_version_t,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_get_default_max_tls_protocol_version() -> tls_protocol_version_t;
}
unsafe extern "C" {
    pub fn sec_protocol_options_get_default_max_dtls_protocol_version() -> tls_protocol_version_t;
}
unsafe extern "C" {
    pub fn sec_protocol_options_get_enable_encrypted_client_hello(options: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_options_get_quic_use_legacy_codepoint(options: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn sec_protocol_options_add_tls_application_protocol(
        options: NSObject,
        application_protocol: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_server_name(
        options: NSObject,
        server_name: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_diffie_hellman_parameters(
        options: NSObject,
        params: NSObject,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_add_pre_shared_key(
        options: NSObject,
        psk: NSObject,
        psk_identity: NSObject,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_pre_shared_key_identity_hint(
        options: NSObject,
        psk_identity_hint: NSObject,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_pre_shared_key_selection_block(
        options: NSObject,
        psk_selection_block: sec_protocol_pre_shared_key_selection_t,
        psk_selection_queue: NSObject,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_tickets_enabled(options: NSObject, tickets_enabled: bool);
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_is_fallback_attempt(
        options: NSObject,
        is_fallback_attempt: bool,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_resumption_enabled(
        options: NSObject,
        resumption_enabled: bool,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_false_start_enabled(
        options: NSObject,
        false_start_enabled: bool,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_ocsp_enabled(options: NSObject, ocsp_enabled: bool);
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_sct_enabled(options: NSObject, sct_enabled: bool);
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_tls_renegotiation_enabled(
        options: NSObject,
        renegotiation_enabled: bool,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_peer_authentication_required(
        options: NSObject,
        peer_authentication_required: bool,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_peer_authentication_optional(
        options: NSObject,
        peer_authentication_optional: bool,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_enable_encrypted_client_hello(
        options: NSObject,
        enable_encrypted_client_hello: bool,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_quic_use_legacy_codepoint(
        options: NSObject,
        quic_use_legacy_codepoint: bool,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_key_update_block(
        options: NSObject,
        key_update_block: sec_protocol_key_update_t,
        key_update_queue: NSObject,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_challenge_block(
        options: NSObject,
        challenge_block: sec_protocol_challenge_t,
        challenge_queue: NSObject,
    );
}
unsafe extern "C" {
    pub fn sec_protocol_options_set_verify_block(
        options: NSObject,
        verify_block: sec_protocol_verify_t,
        verify_block_queue: NSObject,
    );
}
unsafe extern "C" {
    pub static CSSMOID_MD2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MD4: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MD5: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_RSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MD2WithRSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MD4WithRSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MD5WithRSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA1WithRSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA224WithRSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA256WithRSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA384WithRSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA512WithRSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA1WithRSA_OIW: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_RSAWithOAEP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_OAEP_MGF1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_OAEP_ID_PSPECIFIED: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DES_CBC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_PUB_NUMBER: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_STATIC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_ONE_FLOW: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_EPHEM: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_HYBRID1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_HYBRID2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_HYBRID_ONEFLOW: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_MQV1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_MQV2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_STATIC_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_ONE_FLOW_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_EPHEM_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_HYBRID1_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_DH_HYBRID2_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_MQV1_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ANSI_MQV2_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS3: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DH: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DSA_CMS: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DSA_JDK: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA1WithDSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA1WithDSA_CMS: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA1WithDSA_JDK: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA224: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA256: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA384: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SHA512: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ecPublicKey: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ECDSA_WithSHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ECDSA_WithSHA224: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ECDSA_WithSHA256: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ECDSA_WithSHA384: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ECDSA_WithSHA512: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ECDSA_WithSpecified: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_ISIGN: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_X509_BASIC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_SSL: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_LOCAL_CERT_GEN: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_CSR_GEN: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_REVOCATION_CRL: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_REVOCATION_OCSP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_SMIME: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_EAP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_CODE_SIGN: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_SW_UPDATE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_IP_SEC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_ICHAT: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_RESOURCE_SIGN: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_PKINIT_CLIENT: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_PKINIT_SERVER: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_CODE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_PACKAGE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_MACAPPSTORE_RECEIPT: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_APPLEID_SHARING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_TIMESTAMPING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_REVOCATION: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_PASSBOOK_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_MOBILE_STORE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_ESCROW_SERVICE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_PROFILE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_QA_PROFILE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_TEST_MOBILE_STORE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_PCS_ESCROW_SERVICE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_TP_PROVISIONING_PROFILE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_FEE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_ASC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_FEE_MD5: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_FEE_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_FEED: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_FEEDEXP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_ECDSA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_IDENTITY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_EMAIL_SIGN: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_EMAIL_ENCRYPT: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_ARCHIVE_LIST: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_ARCHIVE_STORE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_ARCHIVE_FETCH: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_ARCHIVE_REMOVE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_SHARED_SERVICES: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_VALUE_USERNAME: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_VALUE_PASSWORD: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_VALUE_HOSTNAME: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_VALUE_RENEW: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_VALUE_ASYNC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_REQ_VALUE_IS_PENDING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_DIGEST_ALG: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_ENCRYPT_ALG: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_HMAC_SHA1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_pbeWithMD2AndDES: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_pbeWithMD2AndRC2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_pbeWithMD5AndDES: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_pbeWithMD5AndRC2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_pbeWithSHA1AndDES: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_pbeWithSHA1AndRC2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_PBKDF2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_PBES2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_PBMAC1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_RC2_CBC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_DES_EDE3_CBC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS5_RC5_CBC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_pbeWithSHAAnd128BitRC4: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_pbeWithSHAAnd40BitRC4: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_pbeWithSHAAnd3Key3DESCBC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_pbeWithSHAAnd2Key3DESCBC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_pbeWithSHAAnd128BitRC2CBC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_pbewithSHAAnd40BitRC2CBC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static kSecCFErrorArchitecture: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorPattern: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorResourceSeal: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorResourceAdded: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorResourceAltered: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorResourceMissing: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorResourceSideband: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorResourceRecursive: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorInfoPlist: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorGuestAttributes: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorRequirementSyntax: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCFErrorPath: CFStringRef;
}
unsafe extern "C" {
    pub fn SecStaticCodeGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecStaticCodeCreateWithPath(
        path: CFURLRef,
        flags: SecCSFlags,
        staticCode: *mut SecStaticCodeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kSecCodeAttributeArchitecture: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeAttributeSubarchitecture: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeAttributeUniversalFileOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeAttributeBundleVersion: CFStringRef;
}
unsafe extern "C" {
    pub fn SecStaticCodeCreateWithPathAndAttributes(
        path: CFURLRef,
        flags: SecCSFlags,
        attributes: CFDictionaryRef,
        staticCode: *mut SecStaticCodeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecStaticCodeCheckValidity(
        staticCode: SecStaticCodeRef,
        flags: SecCSFlags,
        requirement: SecRequirementRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecStaticCodeCheckValidityWithErrors(
        staticCode: SecStaticCodeRef,
        flags: SecCSFlags,
        requirement: SecRequirementRef,
        errors: *mut CFErrorRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecCodeCopySelf(flags: SecCSFlags, self_: *mut SecCodeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeCopyStaticCode(
        code: SecCodeRef,
        flags: SecCSFlags,
        staticCode: *mut SecStaticCodeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeCopyHost(guest: SecCodeRef, flags: SecCSFlags, host: *mut SecCodeRef)
        -> OSStatus;
}
unsafe extern "C" {
    pub static kSecGuestAttributeCanonical: CFStringRef;
}
unsafe extern "C" {
    pub static kSecGuestAttributeHash: CFStringRef;
}
unsafe extern "C" {
    pub static kSecGuestAttributeMachPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSecGuestAttributePid: CFStringRef;
}
unsafe extern "C" {
    pub static kSecGuestAttributeAudit: CFStringRef;
}
unsafe extern "C" {
    pub static kSecGuestAttributeDynamicCode: CFStringRef;
}
unsafe extern "C" {
    pub static kSecGuestAttributeDynamicCodeInfoPlist: CFStringRef;
}
unsafe extern "C" {
    pub static kSecGuestAttributeArchitecture: CFStringRef;
}
unsafe extern "C" {
    pub static kSecGuestAttributeSubarchitecture: CFStringRef;
}
unsafe extern "C" {
    pub fn SecCodeCheckValidity(
        code: SecCodeRef,
        flags: SecCSFlags,
        requirement: SecRequirementRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeCheckValidityWithErrors(
        code: SecCodeRef,
        flags: SecCSFlags,
        requirement: SecRequirementRef,
        errors: *mut CFErrorRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeValidateFileResource(
        code: SecStaticCodeRef,
        relativePath: CFStringRef,
        fileData: CFDataRef,
        flags: SecCSFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeCopyPath(
        staticCode: SecStaticCodeRef,
        flags: SecCSFlags,
        path: *mut CFURLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeCopyDesignatedRequirement(
        code: SecStaticCodeRef,
        flags: SecCSFlags,
        requirement: *mut SecRequirementRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kSecCodeInfoCertificates: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoChangedFiles: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoCMS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoDesignatedRequirement: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoEntitlements: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoEntitlementsDict: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoFlags: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoDigestAlgorithm: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoDigestAlgorithms: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoPlatformIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoImplicitDesignatedRequirement: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoDefaultDesignatedLightweightCodeRequirement: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoMainExecutable: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoPList: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoRequirements: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoRequirementData: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoSource: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoTeamIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoTime: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoTimestamp: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoTrust: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoUnique: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoCdHashes: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoRuntimeVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCodeInfoStapledNotarizationTicket: CFStringRef;
}
unsafe extern "C" {
    pub fn SecCodeCopySigningInformation(
        code: SecStaticCodeRef,
        flags: SecCSFlags,
        information: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeMapMemory(code: SecStaticCodeRef, flags: SecCSFlags) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecRequirementGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecRequirementCreateWithData(
        data: CFDataRef,
        flags: SecCSFlags,
        requirement: *mut SecRequirementRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecRequirementCreateWithString(
        text: CFStringRef,
        flags: SecCSFlags,
        requirement: *mut SecRequirementRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecRequirementCreateWithStringAndErrors(
        text: CFStringRef,
        flags: SecCSFlags,
        errors: *mut CFErrorRef,
        requirement: *mut SecRequirementRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecRequirementCopyData(
        requirement: SecRequirementRef,
        flags: SecCSFlags,
        data: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecRequirementCopyString(
        requirement: SecRequirementRef,
        flags: SecCSFlags,
        text: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTaskGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecTaskCreateWithAuditToken(
        allocator: CFAllocatorRef,
        token: audit_token_t,
    ) -> SecTaskRef;
}
unsafe extern "C" {
    pub fn SecTaskCreateFromSelf(allocator: CFAllocatorRef) -> SecTaskRef;
}
unsafe extern "C" {
    pub fn SecTaskCopyValueForEntitlement(
        task: SecTaskRef,
        entitlement: CFStringRef,
        error: *mut CFErrorRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn SecTaskCopyValuesForEntitlements(
        task: SecTaskRef,
        entitlements: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SecTaskCopySigningIdentifier(task: SecTaskRef, error: *mut CFErrorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SecTaskGetCodeSignStatus(task: SecTaskRef) -> u32;
}
unsafe extern "C" {
    pub fn CMSDecoderGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMSDecoderCreate(cmsDecoderOut: *mut CMSDecoderRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderUpdateMessage(
        cmsDecoder: CMSDecoderRef,
        msgBytes: *const ::std::os::raw::c_void,
        msgBytesLen: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderFinalizeMessage(cmsDecoder: CMSDecoderRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderSetDetachedContent(
        cmsDecoder: CMSDecoderRef,
        detachedContent: CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopyDetachedContent(
        cmsDecoder: CMSDecoderRef,
        detachedContentOut: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderGetNumSigners(
        cmsDecoder: CMSDecoderRef,
        numSignersOut: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopySignerStatus(
        cmsDecoder: CMSDecoderRef,
        signerIndex: usize,
        policyOrArray: CFTypeRef,
        evaluateSecTrust: Boolean,
        signerStatusOut: *mut CMSSignerStatus,
        secTrustOut: *mut SecTrustRef,
        certVerifyResultCodeOut: *mut OSStatus,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopySignerEmailAddress(
        cmsDecoder: CMSDecoderRef,
        signerIndex: usize,
        signerEmailAddressOut: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopySignerCert(
        cmsDecoder: CMSDecoderRef,
        signerIndex: usize,
        signerCertOut: *mut SecCertificateRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderIsContentEncrypted(
        cmsDecoder: CMSDecoderRef,
        isEncryptedOut: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopyEncapsulatedContentType(
        cmsDecoder: CMSDecoderRef,
        eContentTypeOut: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopyAllCerts(cmsDecoder: CMSDecoderRef, certsOut: *mut CFArrayRef)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopyContent(cmsDecoder: CMSDecoderRef, contentOut: *mut CFDataRef)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopySignerSigningTime(
        cmsDecoder: CMSDecoderRef,
        signerIndex: usize,
        signingTime: *mut CFAbsoluteTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMSEncoderCreate(cmsEncoderOut: *mut CMSEncoderRef) -> OSStatus;
}
unsafe extern "C" {
    pub static kCMSEncoderDigestAlgorithmSHA1: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSEncoderDigestAlgorithmSHA256: CFStringRef;
}
unsafe extern "C" {
    pub fn CMSEncoderSetSignerAlgorithm(
        cmsEncoder: CMSEncoderRef,
        digestAlgorithm: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderAddSigners(cmsEncoder: CMSEncoderRef, signerOrArray: CFTypeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderCopySigners(
        cmsEncoder: CMSEncoderRef,
        signersOut: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderAddRecipients(
        cmsEncoder: CMSEncoderRef,
        recipientOrArray: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderCopyRecipients(
        cmsEncoder: CMSEncoderRef,
        recipientsOut: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderSetHasDetachedContent(
        cmsEncoder: CMSEncoderRef,
        detachedContent: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderGetHasDetachedContent(
        cmsEncoder: CMSEncoderRef,
        detachedContentOut: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderSetEncapsulatedContentTypeOID(
        cmsEncoder: CMSEncoderRef,
        eContentTypeOID: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderCopyEncapsulatedContentType(
        cmsEncoder: CMSEncoderRef,
        eContentTypeOut: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderAddSupportingCerts(
        cmsEncoder: CMSEncoderRef,
        certOrArray: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderCopySupportingCerts(
        cmsEncoder: CMSEncoderRef,
        certsOut: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderAddSignedAttributes(
        cmsEncoder: CMSEncoderRef,
        signedAttributes: CMSSignedAttributes,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderSetCertificateChainMode(
        cmsEncoder: CMSEncoderRef,
        chainMode: CMSCertificateChainMode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderGetCertificateChainMode(
        cmsEncoder: CMSEncoderRef,
        chainModeOut: *mut CMSCertificateChainMode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderUpdateContent(
        cmsEncoder: CMSEncoderRef,
        content: *const ::std::os::raw::c_void,
        contentLen: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderCopyEncodedContent(
        cmsEncoder: CMSEncoderRef,
        encodedContentOut: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncodeContent(
        signers: CFTypeRef,
        recipients: CFTypeRef,
        eContentTypeOID: CFTypeRef,
        detachedContent: Boolean,
        signedAttributes: CMSSignedAttributes,
        content: *const ::std::os::raw::c_void,
        contentLen: usize,
        encodedContentOut: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_default: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_ATSv1: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_ATSv1_noPFS: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_standard: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_RC4_fallback: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_TLSv1_fallback: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_TLSv1_RC4_fallback: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_legacy: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_legacy_DHE: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_anonymous: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_3DES_fallback: CFStringRef;
}
unsafe extern "C" {
    pub static kSSLSessionConfig_TLSv1_3DES_fallback: CFStringRef;
}
unsafe extern "C" {
    pub fn SSLContextGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SSLCreateContext(
        alloc: CFAllocatorRef,
        protocolSide: SSLProtocolSide,
        connectionType: SSLConnectionType,
    ) -> SSLContextRef;
}
unsafe extern "C" {
    pub fn SSLGetSessionState(context: SSLContextRef, state: *mut SSLSessionState) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetSessionOption(
        context: SSLContextRef,
        option: SSLSessionOption,
        value: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetSessionOption(
        context: SSLContextRef,
        option: SSLSessionOption,
        value: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetIOFuncs(
        context: SSLContextRef,
        readFunc: SSLReadFunc,
        writeFunc: SSLWriteFunc,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetSessionConfig(context: SSLContextRef, config: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetProtocolVersionMin(context: SSLContextRef, minVersion: SSLProtocol) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetProtocolVersionMin(
        context: SSLContextRef,
        minVersion: *mut SSLProtocol,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetProtocolVersionMax(context: SSLContextRef, maxVersion: SSLProtocol) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetProtocolVersionMax(
        context: SSLContextRef,
        maxVersion: *mut SSLProtocol,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetCertificate(context: SSLContextRef, certRefs: CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetConnection(context: SSLContextRef, connection: SSLConnectionRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetConnection(context: SSLContextRef, connection: *mut SSLConnectionRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetPeerDomainName(
        context: SSLContextRef,
        peerName: *const ::std::os::raw::c_char,
        peerNameLen: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetPeerDomainNameLength(context: SSLContextRef, peerNameLen: *mut usize) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetPeerDomainName(
        context: SSLContextRef,
        peerName: *mut ::std::os::raw::c_char,
        peerNameLen: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLCopyRequestedPeerNameLength(ctx: SSLContextRef, peerNameLen: *mut usize) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLCopyRequestedPeerName(
        context: SSLContextRef,
        peerName: *mut ::std::os::raw::c_char,
        peerNameLen: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetDatagramHelloCookie(
        dtlsContext: SSLContextRef,
        cookie: *const ::std::os::raw::c_void,
        cookieLen: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetMaxDatagramRecordSize(dtlsContext: SSLContextRef, maxSize: usize) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetMaxDatagramRecordSize(dtlsContext: SSLContextRef, maxSize: *mut usize)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetNegotiatedProtocolVersion(
        context: SSLContextRef,
        protocol: *mut SSLProtocol,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetNumberSupportedCiphers(context: SSLContextRef, numCiphers: *mut usize)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetSupportedCiphers(
        context: SSLContextRef,
        ciphers: *mut SSLCipherSuite,
        numCiphers: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetNumberEnabledCiphers(context: SSLContextRef, numCiphers: *mut usize) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetEnabledCiphers(
        context: SSLContextRef,
        ciphers: *const SSLCipherSuite,
        numCiphers: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetEnabledCiphers(
        context: SSLContextRef,
        ciphers: *mut SSLCipherSuite,
        numCiphers: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetSessionTicketsEnabled(context: SSLContextRef, enabled: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLCopyPeerTrust(context: SSLContextRef, trust: *mut SecTrustRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetPeerID(
        context: SSLContextRef,
        peerID: *const ::std::os::raw::c_void,
        peerIDLen: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetPeerID(
        context: SSLContextRef,
        peerID: *mut *const ::std::os::raw::c_void,
        peerIDLen: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetNegotiatedCipher(
        context: SSLContextRef,
        cipherSuite: *mut SSLCipherSuite,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetALPNProtocols(context: SSLContextRef, protocols: CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLCopyALPNProtocols(context: SSLContextRef, protocols: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetOCSPResponse(context: SSLContextRef, response: CFDataRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetEncryptionCertificate(context: SSLContextRef, certRefs: CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetClientSideAuthenticate(context: SSLContextRef, auth: SSLAuthenticate) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLAddDistinguishedName(
        context: SSLContextRef,
        derDN: *const ::std::os::raw::c_void,
        derDNLen: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLCopyDistinguishedNames(context: SSLContextRef, names: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetClientCertificateState(
        context: SSLContextRef,
        clientState: *mut SSLClientCertificateState,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLHandshake(context: SSLContextRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLReHandshake(context: SSLContextRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLWrite(
        context: SSLContextRef,
        data: *const ::std::os::raw::c_void,
        dataLength: usize,
        processed: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLRead(
        context: SSLContextRef,
        data: *mut ::std::os::raw::c_void,
        dataLength: usize,
        processed: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetBufferedReadSize(context: SSLContextRef, bufferSize: *mut usize) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetDatagramWriteSize(dtlsContext: SSLContextRef, bufSize: *mut usize) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLClose(context: SSLContextRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetError(context: SSLContextRef, status: OSStatus) -> OSStatus;
}
unsafe extern "C" {
    pub static oidRsa: DERItem;
}
unsafe extern "C" {
    pub static oidMd2Rsa: DERItem;
}
unsafe extern "C" {
    pub static oidMd4Rsa: DERItem;
}
unsafe extern "C" {
    pub static oidMd5Rsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha1Rsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha256Rsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha384Rsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha512Rsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha224Rsa: DERItem;
}
unsafe extern "C" {
    pub static oidEcPubKey: DERItem;
}
unsafe extern "C" {
    pub static oidSha1Ecdsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha224Ecdsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha256Ecdsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha384Ecdsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha512Ecdsa: DERItem;
}
unsafe extern "C" {
    pub static oidSha1Dsa: DERItem;
}
unsafe extern "C" {
    pub static oidMd2: DERItem;
}
unsafe extern "C" {
    pub static oidMd4: DERItem;
}
unsafe extern "C" {
    pub static oidMd5: DERItem;
}
unsafe extern "C" {
    pub static oidSha1: DERItem;
}
unsafe extern "C" {
    pub static oidSha1DsaOIW: DERItem;
}
unsafe extern "C" {
    pub static oidSha1DsaCommonOIW: DERItem;
}
unsafe extern "C" {
    pub static oidSha1RsaOIW: DERItem;
}
unsafe extern "C" {
    pub static oidSha256: DERItem;
}
unsafe extern "C" {
    pub static oidSha384: DERItem;
}
unsafe extern "C" {
    pub static oidSha512: DERItem;
}
unsafe extern "C" {
    pub static oidSha224: DERItem;
}
unsafe extern "C" {
    pub static oidFee: DERItem;
}
unsafe extern "C" {
    pub static oidMd5Fee: DERItem;
}
unsafe extern "C" {
    pub static oidSha1Fee: DERItem;
}
unsafe extern "C" {
    pub static oidEcPrime192v1: DERItem;
}
unsafe extern "C" {
    pub static oidEcPrime256v1: DERItem;
}
unsafe extern "C" {
    pub static oidAnsip384r1: DERItem;
}
unsafe extern "C" {
    pub static oidAnsip521r1: DERItem;
}
unsafe extern "C" {
    pub static oidSubjectKeyIdentifier: DERItem;
}
unsafe extern "C" {
    pub static oidKeyUsage: DERItem;
}
unsafe extern "C" {
    pub static oidPrivateKeyUsagePeriod: DERItem;
}
unsafe extern "C" {
    pub static oidSubjectAltName: DERItem;
}
unsafe extern "C" {
    pub static oidIssuerAltName: DERItem;
}
unsafe extern "C" {
    pub static oidBasicConstraints: DERItem;
}
unsafe extern "C" {
    pub static oidNameConstraints: DERItem;
}
unsafe extern "C" {
    pub static oidCrlDistributionPoints: DERItem;
}
unsafe extern "C" {
    pub static oidCertificatePolicies: DERItem;
}
unsafe extern "C" {
    pub static oidAnyPolicy: DERItem;
}
unsafe extern "C" {
    pub static oidPolicyMappings: DERItem;
}
unsafe extern "C" {
    pub static oidAuthorityKeyIdentifier: DERItem;
}
unsafe extern "C" {
    pub static oidPolicyConstraints: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsage: DERItem;
}
unsafe extern "C" {
    pub static oidAnyExtendedKeyUsage: DERItem;
}
unsafe extern "C" {
    pub static oidInhibitAnyPolicy: DERItem;
}
unsafe extern "C" {
    pub static oidAuthorityInfoAccess: DERItem;
}
unsafe extern "C" {
    pub static oidSubjectInfoAccess: DERItem;
}
unsafe extern "C" {
    pub static oidAdOCSP: DERItem;
}
unsafe extern "C" {
    pub static oidAdCAIssuer: DERItem;
}
unsafe extern "C" {
    pub static oidNetscapeCertType: DERItem;
}
unsafe extern "C" {
    pub static oidEntrustVersInfo: DERItem;
}
unsafe extern "C" {
    pub static oidMSNTPrincipalName: DERItem;
}
unsafe extern "C" {
    pub static oidQtCps: DERItem;
}
unsafe extern "C" {
    pub static oidQtUNotice: DERItem;
}
unsafe extern "C" {
    pub static oidCommonName: DERItem;
}
unsafe extern "C" {
    pub static oidCountryName: DERItem;
}
unsafe extern "C" {
    pub static oidLocalityName: DERItem;
}
unsafe extern "C" {
    pub static oidStateOrProvinceName: DERItem;
}
unsafe extern "C" {
    pub static oidOrganizationName: DERItem;
}
unsafe extern "C" {
    pub static oidOrganizationalUnitName: DERItem;
}
unsafe extern "C" {
    pub static oidDescription: DERItem;
}
unsafe extern "C" {
    pub static oidEmailAddress: DERItem;
}
unsafe extern "C" {
    pub static oidFriendlyName: DERItem;
}
unsafe extern "C" {
    pub static oidLocalKeyId: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageServerAuth: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageClientAuth: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageCodeSigning: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageEmailProtection: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageTimeStamping: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageOCSPSigning: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageIPSec: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageMicrosoftSGC: DERItem;
}
unsafe extern "C" {
    pub static oidExtendedKeyUsageNetscapeSGC: DERItem;
}
unsafe extern "C" {
    pub static oidGoogleEmbeddedSignedCertificateTimestamp: DERItem;
}
unsafe extern "C" {
    pub static oidGoogleOCSPSignedCertificateTimestamp: DERItem;
}
unsafe extern "C" {
    pub fn SecAsn1CoderCreate(coder: *mut SecAsn1CoderRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAsn1CoderRelease(coder: SecAsn1CoderRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAsn1Decode(
        coder: SecAsn1CoderRef,
        src: *const ::std::os::raw::c_void,
        len: usize,
        templates: *const SecAsn1Template,
        dest: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAsn1DecodeData(
        coder: SecAsn1CoderRef,
        src: *const SecAsn1Item,
        templ: *const SecAsn1Template,
        dest: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAsn1EncodeItem(
        coder: SecAsn1CoderRef,
        src: *const ::std::os::raw::c_void,
        templates: *const SecAsn1Template,
        dest: *mut SecAsn1Item,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAsn1Malloc(coder: SecAsn1CoderRef, len: usize) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn SecAsn1AllocItem(coder: SecAsn1CoderRef, item: *mut SecAsn1Item, len: usize)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAsn1AllocCopy(
        coder: SecAsn1CoderRef,
        src: *const ::std::os::raw::c_void,
        len: usize,
        dest: *mut SecAsn1Item,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAsn1AllocCopyItem(
        coder: SecAsn1CoderRef,
        src: *const SecAsn1Item,
        dest: *mut SecAsn1Item,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAsn1OidCompare(oid1: *const SecAsn1Oid, oid2: *const SecAsn1Oid) -> bool;
}
unsafe extern "C" {
    pub static kSecAsn1AnyTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1BitStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1BMPStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1BooleanTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1EnumeratedTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1GeneralizedTimeTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1IA5StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1IntegerTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1UnsignedIntegerTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1NullTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1ObjectIDTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1OctetStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PrintableStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1T61StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1UniversalStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1UTCTimeTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1UTF8StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1VisibleStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1TeletexStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToAnyTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToBitStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToBMPStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToBooleanTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToEnumeratedTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToGeneralizedTimeTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToIA5StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToIntegerTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToNullTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToObjectIDTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToOctetStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToPrintableStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToT61StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToUniversalStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToUTCTimeTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToUTF8StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToVisibleStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1PointerToTeletexStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfAnyTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfBitStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfBMPStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfBooleanTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfEnumeratedTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfGeneralizedTimeTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfIA5StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfIntegerTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfNullTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfObjectIDTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfOctetStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfPrintableStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfT61StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfUniversalStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfUTCTimeTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfUTF8StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfVisibleStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SequenceOfTeletexStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfAnyTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfBitStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfBMPStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfBooleanTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfEnumeratedTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfGeneralizedTimeTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfIA5StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfIntegerTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfNullTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfObjectIDTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfOctetStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfPrintableStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfT61StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfUniversalStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfUTCTimeTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfUTF8StringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfVisibleStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SetOfTeletexStringTemplate: [SecAsn1Template; 0usize];
}
unsafe extern "C" {
    pub static kSecAsn1SkipTemplate: [SecAsn1Template; 0usize];
}

unsafe impl objc2::encode::RefEncode for audit_token_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for audit_token_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("audit_token_t", &[]);
}
unsafe impl objc2::encode::RefEncode for OS_object {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OS_object {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OS_os_workgroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OS_os_workgroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OS_os_workgroup_interval {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OS_os_workgroup_interval {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OS_os_workgroup_parallel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OS_os_workgroup_parallel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DERItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DERItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DERItem", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecCertificate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecCertificate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecCertificate", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecIdentity", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecKey", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecPolicy {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecPolicy {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecPolicy", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecAccessControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecAccessControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecAccessControl", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecKeychain {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecKeychain {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecKeychain", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecKeychainItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecKeychainItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecKeychainItem", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecKeychainSearch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecKeychainSearch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecKeychainSearch", &[]);
}
unsafe impl objc2::encode::RefEncode for SecKeychainAttribute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecKeychainAttribute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecKeychainAttribute", &[]);
}
unsafe impl objc2::encode::RefEncode for SecKeychainAttributeList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecKeychainAttributeList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecKeychainAttributeList", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecTrustedApplication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecTrustedApplication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecTrustedApplication", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecAccess {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecAccess {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecAccess", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecACL {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecACL {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecACL", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecPassword {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecPassword {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecPassword", &[]);
}
unsafe impl objc2::encode::RefEncode for SecKeychainAttributeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecKeychainAttributeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecKeychainAttributeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_data", &[]);
}
unsafe impl objc2::encode::RefEncode for SecAsn1Template_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecAsn1Template_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecAsn1Template_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecRandom {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecRandom {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecRandom", &[]);
}
unsafe impl objc2::encode::RefEncode for SecKeyImportExportParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecKeyImportExportParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecKeyImportExportParameters", &[]);
}
unsafe impl objc2::encode::RefEncode for SecItemImportExportKeyParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecItemImportExportKeyParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecItemImportExportKeyParameters", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecTrust {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecTrust {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecTrust", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecCode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecCode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecCode", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecRequirement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecRequirement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecRequirement", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecTask", &[]);
}
unsafe impl objc2::encode::RefEncode for _CMSDecoder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CMSDecoder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CMSDecoder", &[]);
}
unsafe impl objc2::encode::RefEncode for _CMSEncoder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CMSEncoder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CMSEncoder", &[]);
}
unsafe impl objc2::encode::RefEncode for SSLContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SSLContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SSLContext", &[]);
}
unsafe impl objc2::encode::RefEncode for SecAsn1Coder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecAsn1Coder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecAsn1Coder", &[]);
}
