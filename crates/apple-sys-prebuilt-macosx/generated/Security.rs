#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use libc::{gid_t, id_t, mach_port_t, mode_t, pid_t, uid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type fpos_t = i64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            __n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            __n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type SInt64 = ::std::os::raw::c_longlong;
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
pub type SecKeychainRef = *mut __SecKeychain;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecKeychainItem {
    _unused: [u8; 0],
}
pub type SecKeychainItemRef = *mut __SecKeychainItem;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecKeychainSearch {
    _unused: [u8; 0],
}
pub type SecKeychainSearchRef = *mut __SecKeychainSearch;
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
pub type SecKeychainStatus = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecTrustedApplication {
    _unused: [u8; 0],
}
pub type SecTrustedApplicationRef = *mut __SecTrustedApplication;
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
pub type SecACLRef = *mut __SecACL;
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
pub type sint64 = i64;
pub type uint64 = u64;
pub type sint32 = i32;
pub type sint16 = i16;
pub type sint8 = i8;
pub type uint32 = u32;
pub type uint16 = u16;
pub type uint8 = u8;
pub type CSSM_INTPTR = isize;
pub type CSSM_SIZE = usize;
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
pub struct SecAsn1AlgId {
    pub algorithm: SecAsn1Oid,
    pub parameters: SecAsn1Item,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecAsn1Template_struct {
    pub kind: u32,
    pub offset: u32,
    pub sub: *const ::std::os::raw::c_void,
    pub size: u32,
}
pub type SecAsn1Template = SecAsn1Template_struct;
pub type CSSM_HANDLE = CSSM_INTPTR;
pub type CSSM_HANDLE_PTR = *mut CSSM_INTPTR;
pub type CSSM_LONG_HANDLE = uint64;
pub type CSSM_LONG_HANDLE_PTR = *mut uint64;
pub type CSSM_MODULE_HANDLE = CSSM_HANDLE;
pub type CSSM_MODULE_HANDLE_PTR = *mut CSSM_HANDLE;
pub type CSSM_CC_HANDLE = CSSM_LONG_HANDLE;
pub type CSSM_CSP_HANDLE = CSSM_MODULE_HANDLE;
pub type CSSM_TP_HANDLE = CSSM_MODULE_HANDLE;
pub type CSSM_AC_HANDLE = CSSM_MODULE_HANDLE;
pub type CSSM_CL_HANDLE = CSSM_MODULE_HANDLE;
pub type CSSM_DL_HANDLE = CSSM_MODULE_HANDLE;
pub type CSSM_DB_HANDLE = CSSM_MODULE_HANDLE;
pub type CSSM_BOOL = sint32;
pub type CSSM_RETURN = sint32;
pub type CSSM_STRING = [::std::os::raw::c_char; 68usize];
pub type CSSM_DATA_PTR = *mut SecAsn1Item;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_guid {
    pub Data1: uint32,
    pub Data2: uint16,
    pub Data3: uint16,
    pub Data4: [uint8; 8usize],
}
pub type CSSM_GUID = cssm_guid;
pub type CSSM_GUID_PTR = *mut cssm_guid;
pub type CSSM_BITMASK = uint32;
pub type CSSM_KEY_HIERARCHY = CSSM_BITMASK;
pub type CSSM_PVC_MODE = CSSM_BITMASK;
pub type CSSM_PRIVILEGE_SCOPE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_version {
    pub Major: uint32,
    pub Minor: uint32,
}
pub type CSSM_VERSION = cssm_version;
pub type CSSM_VERSION_PTR = *mut cssm_version;
pub type CSSM_SERVICE_MASK = uint32;
pub type CSSM_SERVICE_TYPE = CSSM_SERVICE_MASK;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_subservice_uid {
    pub Guid: CSSM_GUID,
    pub Version: CSSM_VERSION,
    pub SubserviceId: uint32,
    pub SubserviceType: CSSM_SERVICE_TYPE,
}
pub type CSSM_SUBSERVICE_UID = cssm_subservice_uid;
pub type CSSM_SUBSERVICE_UID_PTR = *mut cssm_subservice_uid;
pub type CSSM_MODULE_EVENT = uint32;
pub type CSSM_MODULE_EVENT_PTR = *mut uint32;
pub type CSSM_API_ModuleEventHandler = ::std::option::Option<
    unsafe extern "C" fn(
        ModuleGuid: *const CSSM_GUID,
        AppNotifyCallbackCtx: *mut ::std::os::raw::c_void,
        SubserviceId: uint32,
        ServiceType: CSSM_SERVICE_TYPE,
        EventType: CSSM_MODULE_EVENT,
    ) -> CSSM_RETURN,
>;
pub type CSSM_ATTACH_FLAGS = uint32;
pub type CSSM_PRIVILEGE = uint64;
pub type CSSM_USEE_TAG = CSSM_PRIVILEGE;
pub type CSSM_NET_ADDRESS_TYPE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_net_address {
    pub AddressType: CSSM_NET_ADDRESS_TYPE,
    pub Address: SecAsn1Item,
}
pub type CSSM_NET_ADDRESS = cssm_net_address;
pub type CSSM_NET_ADDRESS_PTR = *mut cssm_net_address;
pub type CSSM_NET_PROTOCOL = uint32;
pub type CSSM_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(
        OutData: CSSM_DATA_PTR,
        CallerCtx: *mut ::std::os::raw::c_void,
    ) -> CSSM_RETURN,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_crypto_data {
    pub Param: SecAsn1Item,
    pub Callback: CSSM_CALLBACK,
    pub CallerCtx: *mut ::std::os::raw::c_void,
}
pub type CSSM_CRYPTO_DATA = cssm_crypto_data;
pub type CSSM_CRYPTO_DATA_PTR = *mut cssm_crypto_data;
pub type CSSM_WORDID_TYPE = sint32;
pub type CSSM_LIST_ELEMENT_TYPE = uint32;
pub type CSSM_LIST_ELEMENT_TYPE_PTR = *mut uint32;
pub type CSSM_LIST_TYPE = uint32;
pub type CSSM_LIST_TYPE_PTR = *mut uint32;
pub type CSSM_LIST_ELEMENT_PTR = *mut cssm_list_element;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_list {
    pub ListType: CSSM_LIST_TYPE,
    pub Head: CSSM_LIST_ELEMENT_PTR,
    pub Tail: CSSM_LIST_ELEMENT_PTR,
}
pub type CSSM_LIST = cssm_list;
pub type CSSM_LIST_PTR = *mut cssm_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_list_element {
    pub __bindgen_anon_1: cssm_list_element__bindgen_ty_1,
    pub NextElement: *mut cssm_list_element,
    pub WordID: CSSM_WORDID_TYPE,
    pub ElementType: CSSM_LIST_ELEMENT_TYPE,
    pub Element: cssm_list_element__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cssm_list_element__bindgen_ty_1 {
    pub Sublist: CSSM_LIST,
    pub Word: SecAsn1Item,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_TUPLE {
    pub Issuer: CSSM_LIST,
    pub Subject: CSSM_LIST,
    pub Delegate: CSSM_BOOL,
    pub AuthorizationTag: CSSM_LIST,
    pub ValidityPeriod: CSSM_LIST,
}
pub type CSSM_TUPLE_PTR = *mut CSSM_TUPLE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_tuplegroup {
    pub NumberOfTuples: uint32,
    pub Tuples: CSSM_TUPLE_PTR,
}
pub type CSSM_TUPLEGROUP = cssm_tuplegroup;
pub type CSSM_TUPLEGROUP_PTR = *mut cssm_tuplegroup;
pub type CSSM_SAMPLE_TYPE = CSSM_WORDID_TYPE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_sample {
    pub TypedSample: CSSM_LIST,
    pub Verifier: *const CSSM_SUBSERVICE_UID,
}
pub type CSSM_SAMPLE = cssm_sample;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_samplegroup {
    pub NumberOfSamples: uint32,
    pub Samples: *const CSSM_SAMPLE,
}
pub type CSSM_SAMPLEGROUP = cssm_samplegroup;
pub type CSSM_SAMPLEGROUP_PTR = *mut cssm_samplegroup;
pub type CSSM_MALLOC = ::std::option::Option<
    unsafe extern "C" fn(
        size: CSSM_SIZE,
        allocref: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type CSSM_FREE = ::std::option::Option<
    unsafe extern "C" fn(
        memblock: *mut ::std::os::raw::c_void,
        allocref: *mut ::std::os::raw::c_void,
    ),
>;
pub type CSSM_REALLOC = ::std::option::Option<
    unsafe extern "C" fn(
        memblock: *mut ::std::os::raw::c_void,
        size: CSSM_SIZE,
        allocref: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type CSSM_CALLOC = ::std::option::Option<
    unsafe extern "C" fn(
        num: uint32,
        size: CSSM_SIZE,
        allocref: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_memory_funcs {
    pub malloc_func: CSSM_MALLOC,
    pub free_func: CSSM_FREE,
    pub realloc_func: CSSM_REALLOC,
    pub calloc_func: CSSM_CALLOC,
    pub AllocRef: *mut ::std::os::raw::c_void,
}
pub type CSSM_MEMORY_FUNCS = cssm_memory_funcs;
pub type CSSM_API_MEMORY_FUNCS = CSSM_MEMORY_FUNCS;
pub type CSSM_API_MEMORY_FUNCS_PTR = *mut CSSM_API_MEMORY_FUNCS;
pub type CSSM_CHALLENGE_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(
        Challenge: *const CSSM_LIST,
        Response: CSSM_SAMPLEGROUP_PTR,
        CallerCtx: *mut ::std::os::raw::c_void,
        MemFuncs: *const CSSM_MEMORY_FUNCS,
    ) -> CSSM_RETURN,
>;
pub type CSSM_CERT_TYPE = uint32;
pub type CSSM_CERT_TYPE_PTR = *mut uint32;
pub type CSSM_CERT_ENCODING = uint32;
pub type CSSM_CERT_ENCODING_PTR = *mut uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_encoded_cert {
    pub CertType: CSSM_CERT_TYPE,
    pub CertEncoding: CSSM_CERT_ENCODING,
    pub CertBlob: SecAsn1Item,
}
pub type CSSM_ENCODED_CERT = cssm_encoded_cert;
pub type CSSM_ENCODED_CERT_PTR = *mut cssm_encoded_cert;
pub type CSSM_CERT_PARSE_FORMAT = uint32;
pub type CSSM_CERT_PARSE_FORMAT_PTR = *mut uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_parsed_cert {
    pub CertType: CSSM_CERT_TYPE,
    pub ParsedCertFormat: CSSM_CERT_PARSE_FORMAT,
    pub ParsedCert: *mut ::std::os::raw::c_void,
}
pub type CSSM_PARSED_CERT = cssm_parsed_cert;
pub type CSSM_PARSED_CERT_PTR = *mut cssm_parsed_cert;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_cert_pair {
    pub EncodedCert: CSSM_ENCODED_CERT,
    pub ParsedCert: CSSM_PARSED_CERT,
}
pub type CSSM_CERT_PAIR_PTR = *mut cssm_cert_pair;
pub type CSSM_CERTGROUP_TYPE = uint32;
pub type CSSM_CERTGROUP_TYPE_PTR = *mut uint32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_certgroup {
    pub __bindgen_anon_1: cssm_certgroup__bindgen_ty_1,
    pub CertType: CSSM_CERT_TYPE,
    pub CertEncoding: CSSM_CERT_ENCODING,
    pub NumCerts: uint32,
    pub GroupList: cssm_certgroup__bindgen_ty_1,
    pub CertGroupType: CSSM_CERTGROUP_TYPE,
    pub Reserved: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cssm_certgroup__bindgen_ty_1 {
    pub CertList: CSSM_DATA_PTR,
    pub EncodedCertList: CSSM_ENCODED_CERT_PTR,
    pub ParsedCertList: CSSM_PARSED_CERT_PTR,
    pub PairCertList: CSSM_CERT_PAIR_PTR,
}
pub type CSSM_CERTGROUP = cssm_certgroup;
pub type CSSM_CERTGROUP_PTR = *mut cssm_certgroup;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_base_certs {
    pub TPHandle: CSSM_TP_HANDLE,
    pub CLHandle: CSSM_CL_HANDLE,
    pub Certs: CSSM_CERTGROUP,
}
pub type CSSM_BASE_CERTS = cssm_base_certs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_access_credentials {
    pub EntryTag: CSSM_STRING,
    pub BaseCerts: CSSM_BASE_CERTS,
    pub Samples: CSSM_SAMPLEGROUP,
    pub Callback: CSSM_CHALLENGE_CALLBACK,
    pub CallerCtx: *mut ::std::os::raw::c_void,
}
pub type CSSM_ACCESS_CREDENTIALS = cssm_access_credentials;
pub type CSSM_ACCESS_CREDENTIALS_PTR = *mut cssm_access_credentials;
pub type CSSM_ACL_SUBJECT_TYPE = sint32;
pub type CSSM_ACL_AUTHORIZATION_TAG = sint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_authorizationgroup {
    pub NumberOfAuthTags: uint32,
    pub AuthTags: *mut CSSM_ACL_AUTHORIZATION_TAG,
}
pub type CSSM_AUTHORIZATIONGROUP = cssm_authorizationgroup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_acl_validity_period {
    pub StartDate: SecAsn1Item,
    pub EndDate: SecAsn1Item,
}
pub type CSSM_ACL_VALIDITY_PERIOD = cssm_acl_validity_period;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_acl_entry_prototype {
    pub TypedSubject: CSSM_LIST,
    pub Delegate: CSSM_BOOL,
    pub Authorization: CSSM_AUTHORIZATIONGROUP,
    pub TimeRange: CSSM_ACL_VALIDITY_PERIOD,
    pub EntryTag: CSSM_STRING,
}
pub type CSSM_ACL_ENTRY_PROTOTYPE = cssm_acl_entry_prototype;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_acl_owner_prototype {
    pub TypedSubject: CSSM_LIST,
    pub Delegate: CSSM_BOOL,
}
pub type CSSM_ACL_OWNER_PROTOTYPE = cssm_acl_owner_prototype;
pub type CSSM_ACL_OWNER_PROTOTYPE_PTR = *mut cssm_acl_owner_prototype;
pub type CSSM_ACL_SUBJECT_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(
        SubjectRequest: *const CSSM_LIST,
        SubjectResponse: CSSM_LIST_PTR,
        CallerContext: *mut ::std::os::raw::c_void,
        MemFuncs: *const CSSM_MEMORY_FUNCS,
    ) -> CSSM_RETURN,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_acl_entry_input {
    pub Prototype: CSSM_ACL_ENTRY_PROTOTYPE,
    pub Callback: CSSM_ACL_SUBJECT_CALLBACK,
    pub CallerContext: *mut ::std::os::raw::c_void,
}
pub type CSSM_ACL_ENTRY_INPUT = cssm_acl_entry_input;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_resource_control_context {
    pub AccessCred: CSSM_ACCESS_CREDENTIALS_PTR,
    pub InitialAclEntry: CSSM_ACL_ENTRY_INPUT,
}
pub type CSSM_RESOURCE_CONTROL_CONTEXT = cssm_resource_control_context;
pub type CSSM_ACL_HANDLE = CSSM_HANDLE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_acl_entry_info {
    pub EntryPublicInfo: CSSM_ACL_ENTRY_PROTOTYPE,
    pub EntryHandle: CSSM_ACL_HANDLE,
}
pub type CSSM_ACL_ENTRY_INFO = cssm_acl_entry_info;
pub type CSSM_ACL_ENTRY_INFO_PTR = *mut cssm_acl_entry_info;
pub type CSSM_ACL_EDIT_MODE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_acl_edit {
    pub EditMode: CSSM_ACL_EDIT_MODE,
    pub OldEntryHandle: CSSM_ACL_HANDLE,
    pub NewEntry: *const CSSM_ACL_ENTRY_INPUT,
}
pub type CSSM_ACL_EDIT = cssm_acl_edit;
pub type CSSM_PROC_ADDR = ::std::option::Option<unsafe extern "C" fn()>;
pub type CSSM_PROC_ADDR_PTR = *mut CSSM_PROC_ADDR;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_func_name_addr {
    pub Name: CSSM_STRING,
    pub Address: CSSM_PROC_ADDR,
}
pub type CSSM_FUNC_NAME_ADDR = cssm_func_name_addr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_date {
    pub Year: [uint8; 4usize],
    pub Month: [uint8; 2usize],
    pub Day: [uint8; 2usize],
}
pub type CSSM_DATE = cssm_date;
pub type CSSM_DATE_PTR = *mut cssm_date;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_range {
    pub Min: uint32,
    pub Max: uint32,
}
pub type CSSM_RANGE_PTR = *mut cssm_range;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_query_size_data {
    pub SizeInputBlock: uint32,
    pub SizeOutputBlock: uint32,
}
pub type CSSM_QUERY_SIZE_DATA_PTR = *mut cssm_query_size_data;
pub type CSSM_HEADERVERSION = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_key_size {
    pub LogicalKeySizeInBits: uint32,
    pub EffectiveKeySizeInBits: uint32,
}
pub type CSSM_KEY_SIZE_PTR = *mut cssm_key_size;
pub type CSSM_KEYBLOB_TYPE = uint32;
pub type CSSM_KEYBLOB_FORMAT = uint32;
pub type CSSM_KEYCLASS = uint32;
pub type CSSM_KEYATTR_FLAGS = uint32;
pub type CSSM_KEYUSE = uint32;
pub type CSSM_ALGORITHMS = uint32;
pub type CSSM_ENCRYPT_MODE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_keyheader {
    pub HeaderVersion: CSSM_HEADERVERSION,
    pub CspId: CSSM_GUID,
    pub BlobType: CSSM_KEYBLOB_TYPE,
    pub Format: CSSM_KEYBLOB_FORMAT,
    pub AlgorithmId: CSSM_ALGORITHMS,
    pub KeyClass: CSSM_KEYCLASS,
    pub LogicalKeySizeInBits: uint32,
    pub KeyAttr: CSSM_KEYATTR_FLAGS,
    pub KeyUsage: CSSM_KEYUSE,
    pub StartDate: CSSM_DATE,
    pub EndDate: CSSM_DATE,
    pub WrapAlgorithmId: CSSM_ALGORITHMS,
    pub WrapMode: CSSM_ENCRYPT_MODE,
    pub Reserved: uint32,
}
pub type CSSM_KEYHEADER = cssm_keyheader;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_key {
    pub KeyHeader: CSSM_KEYHEADER,
    pub KeyData: SecAsn1Item,
}
pub type CSSM_KEY = cssm_key;
pub type CSSM_KEY_PTR = *mut cssm_key;
pub type CSSM_WRAP_KEY = CSSM_KEY;
pub type CSSM_WRAP_KEY_PTR = *mut CSSM_KEY;
pub type CSSM_CSPTYPE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_dl_db_handle {
    pub DLHandle: CSSM_DL_HANDLE,
    pub DBHandle: CSSM_DB_HANDLE,
}
pub type CSSM_DL_DB_HANDLE = cssm_dl_db_handle;
pub type CSSM_DL_DB_HANDLE_PTR = *mut cssm_dl_db_handle;
pub type CSSM_CONTEXT_TYPE = uint32;
pub type CSSM_ATTRIBUTE_TYPE = uint32;
pub type CSSM_PADDING = uint32;
pub type CSSM_KEY_TYPE = CSSM_ALGORITHMS;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_context_attribute {
    pub AttributeType: CSSM_ATTRIBUTE_TYPE,
    pub AttributeLength: uint32,
    pub Attribute: cssm_context_attribute_cssm_context_attribute_value,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cssm_context_attribute_cssm_context_attribute_value {
    pub String: *mut ::std::os::raw::c_char,
    pub Uint32: uint32,
    pub AccessCredentials: CSSM_ACCESS_CREDENTIALS_PTR,
    pub Key: CSSM_KEY_PTR,
    pub Data: CSSM_DATA_PTR,
    pub Padding: CSSM_PADDING,
    pub Date: CSSM_DATE_PTR,
    pub Range: CSSM_RANGE_PTR,
    pub CryptoData: CSSM_CRYPTO_DATA_PTR,
    pub Version: CSSM_VERSION_PTR,
    pub DLDBHandle: CSSM_DL_DB_HANDLE_PTR,
    pub KRProfile: *mut cssm_kr_profile,
}
pub type CSSM_CONTEXT_ATTRIBUTE = cssm_context_attribute;
pub type CSSM_CONTEXT_ATTRIBUTE_PTR = *mut cssm_context_attribute;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_context {
    pub ContextType: CSSM_CONTEXT_TYPE,
    pub AlgorithmType: CSSM_ALGORITHMS,
    pub NumberOfAttributes: uint32,
    pub ContextAttributes: CSSM_CONTEXT_ATTRIBUTE_PTR,
    pub CSPHandle: CSSM_CSP_HANDLE,
    pub Privileged: CSSM_BOOL,
    pub EncryptionProhibited: uint32,
    pub WorkFactor: uint32,
    pub Reserved: uint32,
}
pub type CSSM_CONTEXT = cssm_context;
pub type CSSM_CONTEXT_PTR = *mut cssm_context;
pub type CSSM_SC_FLAGS = uint32;
pub type CSSM_CSP_READER_FLAGS = uint32;
pub type CSSM_CSP_FLAGS = uint32;
pub type CSSM_PKCS_OAEP_MGF = uint32;
pub type CSSM_PKCS_OAEP_PSOURCE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_csp_operational_statistics {
    pub UserAuthenticated: CSSM_BOOL,
    pub DeviceFlags: CSSM_CSP_FLAGS,
    pub TokenMaxSessionCount: uint32,
    pub TokenOpenedSessionCount: uint32,
    pub TokenMaxRWSessionCount: uint32,
    pub TokenOpenedRWSessionCount: uint32,
    pub TokenTotalPublicMem: uint32,
    pub TokenFreePublicMem: uint32,
    pub TokenTotalPrivateMem: uint32,
    pub TokenFreePrivateMem: uint32,
}
pub type CSSM_CSP_OPERATIONAL_STATISTICS = cssm_csp_operational_statistics;
pub type CSSM_PKCS5_PBKDF2_PRF = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_tp_authority_id {
    pub AuthorityCert: *mut SecAsn1Item,
    pub AuthorityLocation: CSSM_NET_ADDRESS_PTR,
}
pub type CSSM_TP_AUTHORITY_ID = cssm_tp_authority_id;
pub type CSSM_TP_AUTHORITY_REQUEST_TYPE = uint32;
pub type CSSM_TP_AUTHORITY_REQUEST_TYPE_PTR = *mut uint32;
pub type CSSM_TP_VERIFICATION_RESULTS_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(
        ModuleHandle: CSSM_MODULE_HANDLE,
        CallerCtx: *mut ::std::os::raw::c_void,
        VerifiedCert: CSSM_DATA_PTR,
    ) -> CSSM_RETURN,
>;
pub type CSSM_OID_PTR = *mut SecAsn1Oid;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_field {
    pub FieldOid: SecAsn1Oid,
    pub FieldValue: SecAsn1Item,
}
pub type CSSM_FIELD = cssm_field;
pub type CSSM_FIELD_PTR = *mut cssm_field;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_tp_policyinfo {
    pub NumberOfPolicyIds: uint32,
    pub PolicyIds: CSSM_FIELD_PTR,
    pub PolicyControl: *mut ::std::os::raw::c_void,
}
pub type CSSM_TP_POLICYINFO = cssm_tp_policyinfo;
pub type CSSM_TP_SERVICES = uint32;
pub type CSSM_TP_ACTION = uint32;
pub type CSSM_TP_STOP_ON = uint32;
pub type CSSM_TIMESTRING = *mut ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_dl_db_list {
    pub NumHandles: uint32,
    pub DLDBHandle: CSSM_DL_DB_HANDLE_PTR,
}
pub type CSSM_DL_DB_LIST = cssm_dl_db_list;
pub type CSSM_DL_DB_LIST_PTR = *mut cssm_dl_db_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_tp_callerauth_context {
    pub Policy: CSSM_TP_POLICYINFO,
    pub VerifyTime: CSSM_TIMESTRING,
    pub VerificationAbortOn: CSSM_TP_STOP_ON,
    pub CallbackWithVerifiedCert: CSSM_TP_VERIFICATION_RESULTS_CALLBACK,
    pub NumberOfAnchorCerts: uint32,
    pub AnchorCerts: CSSM_DATA_PTR,
    pub DBList: CSSM_DL_DB_LIST_PTR,
    pub CallerCredentials: CSSM_ACCESS_CREDENTIALS_PTR,
}
pub type CSSM_TP_CALLERAUTH_CONTEXT = cssm_tp_callerauth_context;
pub type CSSM_TP_CALLERAUTH_CONTEXT_PTR = *mut cssm_tp_callerauth_context;
pub type CSSM_CRL_PARSE_FORMAT = uint32;
pub type CSSM_CRL_PARSE_FORMAT_PTR = *mut uint32;
pub type CSSM_CRL_TYPE = uint32;
pub type CSSM_CRL_TYPE_PTR = *mut uint32;
pub type CSSM_CRL_ENCODING = uint32;
pub type CSSM_CRL_ENCODING_PTR = *mut uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_encoded_crl {
    pub CrlType: CSSM_CRL_TYPE,
    pub CrlEncoding: CSSM_CRL_ENCODING,
    pub CrlBlob: SecAsn1Item,
}
pub type CSSM_ENCODED_CRL = cssm_encoded_crl;
pub type CSSM_ENCODED_CRL_PTR = *mut cssm_encoded_crl;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_parsed_crl {
    pub CrlType: CSSM_CRL_TYPE,
    pub ParsedCrlFormat: CSSM_CRL_PARSE_FORMAT,
    pub ParsedCrl: *mut ::std::os::raw::c_void,
}
pub type CSSM_PARSED_CRL = cssm_parsed_crl;
pub type CSSM_PARSED_CRL_PTR = *mut cssm_parsed_crl;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_crl_pair {
    pub EncodedCrl: CSSM_ENCODED_CRL,
    pub ParsedCrl: CSSM_PARSED_CRL,
}
pub type CSSM_CRL_PAIR_PTR = *mut cssm_crl_pair;
pub type CSSM_CRLGROUP_TYPE = uint32;
pub type CSSM_CRLGROUP_TYPE_PTR = *mut uint32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_crlgroup {
    pub __bindgen_anon_1: cssm_crlgroup__bindgen_ty_1,
    pub CrlType: CSSM_CRL_TYPE,
    pub CrlEncoding: CSSM_CRL_ENCODING,
    pub NumberOfCrls: uint32,
    pub GroupCrlList: cssm_crlgroup__bindgen_ty_1,
    pub CrlGroupType: CSSM_CRLGROUP_TYPE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cssm_crlgroup__bindgen_ty_1 {
    pub CrlList: CSSM_DATA_PTR,
    pub EncodedCrlList: CSSM_ENCODED_CRL_PTR,
    pub ParsedCrlList: CSSM_PARSED_CRL_PTR,
    pub PairCrlList: CSSM_CRL_PAIR_PTR,
}
pub type CSSM_CRLGROUP = cssm_crlgroup;
pub type CSSM_EVIDENCE_FORM = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_evidence {
    pub EvidenceForm: CSSM_EVIDENCE_FORM,
    pub Evidence: *mut ::std::os::raw::c_void,
}
pub type CSSM_EVIDENCE_PTR = *mut cssm_evidence;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_verify_context {
    pub Action: CSSM_TP_ACTION,
    pub ActionData: SecAsn1Item,
    pub Crls: CSSM_CRLGROUP,
    pub Cred: CSSM_TP_CALLERAUTH_CONTEXT_PTR,
}
pub type CSSM_TP_VERIFY_CONTEXT = cssm_tp_verify_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_tp_verify_context_result {
    pub NumberOfEvidences: uint32,
    pub Evidence: CSSM_EVIDENCE_PTR,
}
pub type CSSM_TP_VERIFY_CONTEXT_RESULT_PTR = *mut cssm_tp_verify_context_result;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_tp_request_set {
    pub NumberOfRequests: uint32,
    pub Requests: *mut ::std::os::raw::c_void,
}
pub type CSSM_TP_REQUEST_SET = cssm_tp_request_set;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_tp_result_set {
    pub NumberOfResults: uint32,
    pub Results: *mut ::std::os::raw::c_void,
}
pub type CSSM_TP_RESULT_SET_PTR = *mut cssm_tp_result_set;
pub type CSSM_TP_CONFIRM_STATUS_PTR = *mut uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_tp_confirm_response {
    pub NumberOfResponses: uint32,
    pub Responses: CSSM_TP_CONFIRM_STATUS_PTR,
}
pub type CSSM_TP_CONFIRM_RESPONSE = cssm_tp_confirm_response;
pub type CSSM_TP_CONFIRM_RESPONSE_PTR = *mut cssm_tp_confirm_response;
pub type CSSM_TP_CERTISSUE_STATUS = uint32;
pub type CSSM_TP_CERTCHANGE_ACTION = uint32;
pub type CSSM_TP_CERTCHANGE_REASON = uint32;
pub type CSSM_TP_CERTCHANGE_STATUS = uint32;
pub type CSSM_TP_CERTVERIFY_STATUS = uint32;
pub type CSSM_TP_CERTNOTARIZE_STATUS = uint32;
pub type CSSM_TP_CERTRECLAIM_STATUS = uint32;
pub type CSSM_TP_CRLISSUE_STATUS = uint32;
pub type CSSM_TP_FORM_TYPE = uint32;
pub type CSSM_CL_TEMPLATE_TYPE = uint32;
pub type CSSM_CERT_BUNDLE_TYPE = uint32;
pub type CSSM_CERT_BUNDLE_ENCODING = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_cert_bundle_header {
    pub BundleType: CSSM_CERT_BUNDLE_TYPE,
    pub BundleEncoding: CSSM_CERT_BUNDLE_ENCODING,
}
pub type CSSM_CERT_BUNDLE_HEADER = cssm_cert_bundle_header;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_cert_bundle {
    pub BundleHeader: CSSM_CERT_BUNDLE_HEADER,
    pub Bundle: SecAsn1Item,
}
pub type CSSM_CERT_BUNDLE = cssm_cert_bundle;
pub type CSSM_DB_ATTRIBUTE_NAME_FORMAT = uint32;
pub type CSSM_DB_ATTRIBUTE_NAME_FORMAT_PTR = *mut uint32;
pub type CSSM_DB_ATTRIBUTE_FORMAT = uint32;
pub type CSSM_DB_ATTRIBUTE_FORMAT_PTR = *mut uint32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_attribute_info {
    pub AttributeNameFormat: CSSM_DB_ATTRIBUTE_NAME_FORMAT,
    pub Label: cssm_db_attribute_info_cssm_db_attribute_label,
    pub AttributeFormat: CSSM_DB_ATTRIBUTE_FORMAT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cssm_db_attribute_info_cssm_db_attribute_label {
    pub AttributeName: *mut ::std::os::raw::c_char,
    pub AttributeOID: SecAsn1Oid,
    pub AttributeID: uint32,
}
pub type CSSM_DB_ATTRIBUTE_INFO = cssm_db_attribute_info;
pub type CSSM_DB_ATTRIBUTE_INFO_PTR = *mut cssm_db_attribute_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_attribute_data {
    pub Info: CSSM_DB_ATTRIBUTE_INFO,
    pub NumberOfValues: uint32,
    pub Value: CSSM_DATA_PTR,
}
pub type CSSM_DB_ATTRIBUTE_DATA = cssm_db_attribute_data;
pub type CSSM_DB_ATTRIBUTE_DATA_PTR = *mut cssm_db_attribute_data;
pub type CSSM_DB_RECORDTYPE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_db_record_attribute_info {
    pub DataRecordType: CSSM_DB_RECORDTYPE,
    pub NumberOfAttributes: uint32,
    pub AttributeInfo: CSSM_DB_ATTRIBUTE_INFO_PTR,
}
pub type CSSM_DB_RECORD_ATTRIBUTE_INFO_PTR = *mut cssm_db_record_attribute_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_db_record_attribute_data {
    pub DataRecordType: CSSM_DB_RECORDTYPE,
    pub SemanticInformation: uint32,
    pub NumberOfAttributes: uint32,
    pub AttributeData: CSSM_DB_ATTRIBUTE_DATA_PTR,
}
pub type CSSM_DB_RECORD_ATTRIBUTE_DATA = cssm_db_record_attribute_data;
pub type CSSM_DB_RECORD_ATTRIBUTE_DATA_PTR = *mut cssm_db_record_attribute_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_db_parsing_module_info {
    pub RecordType: CSSM_DB_RECORDTYPE,
    pub ModuleSubserviceUid: CSSM_SUBSERVICE_UID,
}
pub type CSSM_DB_PARSING_MODULE_INFO_PTR = *mut cssm_db_parsing_module_info;
pub type CSSM_DB_INDEX_TYPE = uint32;
pub type CSSM_DB_INDEXED_DATA_LOCATION = uint32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_index_info {
    pub IndexType: CSSM_DB_INDEX_TYPE,
    pub IndexedDataLocation: CSSM_DB_INDEXED_DATA_LOCATION,
    pub Info: CSSM_DB_ATTRIBUTE_INFO,
}
pub type CSSM_DB_INDEX_INFO = cssm_db_index_info;
pub type CSSM_DB_INDEX_INFO_PTR = *mut cssm_db_index_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_unique_record {
    pub RecordLocator: CSSM_DB_INDEX_INFO,
    pub RecordIdentifier: SecAsn1Item,
}
pub type CSSM_DB_UNIQUE_RECORD = cssm_db_unique_record;
pub type CSSM_DB_UNIQUE_RECORD_PTR = *mut cssm_db_unique_record;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_db_record_index_info {
    pub DataRecordType: CSSM_DB_RECORDTYPE,
    pub NumberOfIndexes: uint32,
    pub IndexInfo: CSSM_DB_INDEX_INFO_PTR,
}
pub type CSSM_DB_RECORD_INDEX_INFO_PTR = *mut cssm_db_record_index_info;
pub type CSSM_DB_ACCESS_TYPE = uint32;
pub type CSSM_DB_ACCESS_TYPE_PTR = *mut uint32;
pub type CSSM_DB_MODIFY_MODE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_dbinfo {
    pub NumberOfRecordTypes: uint32,
    pub DefaultParsingModules: CSSM_DB_PARSING_MODULE_INFO_PTR,
    pub RecordAttributeNames: CSSM_DB_RECORD_ATTRIBUTE_INFO_PTR,
    pub RecordIndexes: CSSM_DB_RECORD_INDEX_INFO_PTR,
    pub IsLocal: CSSM_BOOL,
    pub AccessPath: *mut ::std::os::raw::c_char,
    pub Reserved: *mut ::std::os::raw::c_void,
}
pub type CSSM_DBINFO = cssm_dbinfo;
pub type CSSM_DB_OPERATOR = uint32;
pub type CSSM_DB_OPERATOR_PTR = *mut uint32;
pub type CSSM_DB_CONJUNCTIVE = uint32;
pub type CSSM_DB_CONJUNCTIVE_PTR = *mut uint32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_selection_predicate {
    pub DbOperator: CSSM_DB_OPERATOR,
    pub Attribute: CSSM_DB_ATTRIBUTE_DATA,
}
pub type CSSM_SELECTION_PREDICATE_PTR = *mut cssm_selection_predicate;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_query_limits {
    pub TimeLimit: uint32,
    pub SizeLimit: uint32,
}
pub type CSSM_QUERY_LIMITS = cssm_query_limits;
pub type CSSM_QUERY_FLAGS = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_query {
    pub RecordType: CSSM_DB_RECORDTYPE,
    pub Conjunctive: CSSM_DB_CONJUNCTIVE,
    pub NumSelectionPredicates: uint32,
    pub SelectionPredicate: CSSM_SELECTION_PREDICATE_PTR,
    pub QueryLimits: CSSM_QUERY_LIMITS,
    pub QueryFlags: CSSM_QUERY_FLAGS,
}
pub type CSSM_QUERY = cssm_query;
pub type CSSM_DLTYPE_PTR = *mut uint32;
pub type CSSM_DL_CUSTOM_ATTRIBUTES = *mut ::std::os::raw::c_void;
pub type CSSM_DL_LDAP_ATTRIBUTES = *mut ::std::os::raw::c_void;
pub type CSSM_DL_ODBC_ATTRIBUTES = *mut ::std::os::raw::c_void;
pub type CSSM_DL_FFS_ATTRIBUTES = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_dl_pkcs11_attributes {
    pub DeviceAccessFlags: uint32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_name_list {
    pub NumStrings: uint32,
    pub String: *mut *mut ::std::os::raw::c_char,
}
pub type CSSM_NAME_LIST_PTR = *mut cssm_name_list;
pub type CSSM_DB_RETRIEVAL_MODES = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_db_schema_attribute_info {
    pub AttributeId: uint32,
    pub AttributeName: *mut ::std::os::raw::c_char,
    pub AttributeNameID: SecAsn1Oid,
    pub DataType: CSSM_DB_ATTRIBUTE_FORMAT,
}
pub type CSSM_DB_SCHEMA_ATTRIBUTE_INFO = cssm_db_schema_attribute_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_db_schema_index_info {
    pub AttributeId: uint32,
    pub IndexId: uint32,
    pub IndexType: CSSM_DB_INDEX_TYPE,
    pub IndexedDataLocation: CSSM_DB_INDEXED_DATA_LOCATION,
}
pub type CSSM_DB_SCHEMA_INDEX_INFO = cssm_db_schema_index_info;
pub type CSSM_BER_TAG = uint8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_x509_type_value_pair {
    pub type_: SecAsn1Oid,
    pub valueType: CSSM_BER_TAG,
    pub value: SecAsn1Item,
}
pub type CSSM_X509_TYPE_VALUE_PAIR_PTR = *mut cssm_x509_type_value_pair;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_x509_rdn {
    pub numberOfPairs: uint32,
    pub AttributeTypeAndValue: CSSM_X509_TYPE_VALUE_PAIR_PTR,
}
pub type CSSM_X509_RDN_PTR = *mut cssm_x509_rdn;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_x509_name {
    pub numberOfRDNs: uint32,
    pub RelativeDistinguishedName: CSSM_X509_RDN_PTR,
}
pub type CSSM_X509_NAME = cssm_x509_name;
pub type CSSM_X509_NAME_PTR = *mut cssm_x509_name;
pub type CSSM_X509_OPTION = CSSM_BOOL;
pub type extension_data_format = ::std::os::raw::c_uint;
pub use self::extension_data_format as CSSM_X509EXT_DATA_FORMAT;
pub type SecKeyUsage = u32;
pub type SecAccessControlCreateFlags = CFOptionFlags;
pub type SecAccessOwnerType = UInt32;
pub type SecCredentialType = uint32;
pub type SecPadding = u32;
pub type SecKeySizes = u32;
pub type SecKeyGeneratePairBlock = *mut ::std::os::raw::c_void;
pub type SecKeyAlgorithm = CFStringRef;
pub type SecKeyKeyExchangeParameter = CFStringRef;
pub type SecKeyOperationType = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SecRandom {
    _unused: [u8; 0],
}
pub type SecRandomRef = *const __SecRandom;
pub type __CE_GeneralNameType = ::std::os::raw::c_uint;
pub use self::__CE_GeneralNameType as CE_GeneralNameType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_OtherName {
    pub typeId: SecAsn1Oid,
    pub value: SecAsn1Item,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_GeneralName {
    pub nameType: CE_GeneralNameType,
    pub berEncoded: CSSM_BOOL,
    pub name: SecAsn1Item,
}
pub type CE_GeneralName = __CE_GeneralName;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_GeneralNames {
    pub numNames: uint32,
    pub generalName: *mut CE_GeneralName,
}
pub type CE_GeneralNames = __CE_GeneralNames;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_AuthorityKeyID {
    pub keyIdentifierPresent: CSSM_BOOL,
    pub keyIdentifier: SecAsn1Item,
    pub generalNamesPresent: CSSM_BOOL,
    pub generalNames: *mut CE_GeneralNames,
    pub serialNumberPresent: CSSM_BOOL,
    pub serialNumber: SecAsn1Item,
}
pub type CE_AuthorityKeyID = __CE_AuthorityKeyID;
pub type CE_SubjectKeyID = SecAsn1Item;
pub type CE_KeyUsage = uint16;
pub type CE_CrlReason = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_ExtendedKeyUsage {
    pub numPurposes: uint32,
    pub purposes: CSSM_OID_PTR,
}
pub type CE_ExtendedKeyUsage = __CE_ExtendedKeyUsage;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_BasicConstraints {
    pub cA: CSSM_BOOL,
    pub pathLenConstraintPresent: CSSM_BOOL,
    pub pathLenConstraint: uint32,
}
pub type CE_BasicConstraints = __CE_BasicConstraints;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_PolicyQualifierInfo {
    pub policyQualifierId: SecAsn1Oid,
    pub qualifier: SecAsn1Item,
}
pub type CE_PolicyQualifierInfo = __CE_PolicyQualifierInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_PolicyInformation {
    pub certPolicyId: SecAsn1Oid,
    pub numPolicyQualifiers: uint32,
    pub policyQualifiers: *mut CE_PolicyQualifierInfo,
}
pub type CE_PolicyInformation = __CE_PolicyInformation;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_CertPolicies {
    pub numPolicies: uint32,
    pub policies: *mut CE_PolicyInformation,
}
pub type CE_CertPolicies = __CE_CertPolicies;
pub type CE_NetscapeCertType = uint16;
pub type CE_CrlDistReasonFlags = uint8;
pub type __CE_CrlDistributionPointNameType = ::std::os::raw::c_uint;
pub use self::__CE_CrlDistributionPointNameType as CE_CrlDistributionPointNameType;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_DistributionPointName {
    pub __bindgen_anon_1: __CE_DistributionPointName__bindgen_ty_1,
    pub nameType: CE_CrlDistributionPointNameType,
    pub dpn: __CE_DistributionPointName__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __CE_DistributionPointName__bindgen_ty_1 {
    pub fullName: *mut CE_GeneralNames,
    pub rdn: CSSM_X509_RDN_PTR,
}
pub type CE_DistributionPointName = __CE_DistributionPointName;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_CRLDistributionPoint {
    pub distPointName: *mut CE_DistributionPointName,
    pub reasonsPresent: CSSM_BOOL,
    pub reasons: CE_CrlDistReasonFlags,
    pub crlIssuer: *mut CE_GeneralNames,
}
pub type CE_CRLDistributionPoint = __CE_CRLDistributionPoint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_CRLDistPointsSyntax {
    pub numDistPoints: uint32,
    pub distPoints: *mut CE_CRLDistributionPoint,
}
pub type CE_CRLDistPointsSyntax = __CE_CRLDistPointsSyntax;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_AccessDescription {
    pub accessMethod: SecAsn1Oid,
    pub accessLocation: CE_GeneralName,
}
pub type CE_AccessDescription = __CE_AccessDescription;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_AuthorityInfoAccess {
    pub numAccessDescriptions: uint32,
    pub accessDescriptions: *mut CE_AccessDescription,
}
pub type CE_AuthorityInfoAccess = __CE_AuthorityInfoAccess;
pub type CE_NameRegistrationAuthorities = CE_GeneralNames;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_SemanticsInformation {
    pub semanticsIdentifier: *mut SecAsn1Oid,
    pub nameRegistrationAuthorities: *mut CE_NameRegistrationAuthorities,
}
pub type CE_SemanticsInformation = __CE_SemanticsInformation;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_QC_Statement {
    pub statementId: SecAsn1Oid,
    pub semanticsInfo: *mut CE_SemanticsInformation,
    pub otherInfo: *mut SecAsn1Item,
}
pub type CE_QC_Statement = __CE_QC_Statement;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_QC_Statements {
    pub numQCStatements: uint32,
    pub qcStatements: *mut CE_QC_Statement,
}
pub type CE_QC_Statements = __CE_QC_Statements;
pub type CE_CrlNumber = uint32;
pub type CE_DeltaCrl = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_IssuingDistributionPoint {
    pub distPointName: *mut CE_DistributionPointName,
    pub onlyUserCertsPresent: CSSM_BOOL,
    pub onlyUserCerts: CSSM_BOOL,
    pub onlyCACertsPresent: CSSM_BOOL,
    pub onlyCACerts: CSSM_BOOL,
    pub onlySomeReasonsPresent: CSSM_BOOL,
    pub onlySomeReasons: CE_CrlDistReasonFlags,
    pub indirectCrlPresent: CSSM_BOOL,
    pub indirectCrl: CSSM_BOOL,
}
pub type CE_IssuingDistributionPoint = __CE_IssuingDistributionPoint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_GeneralSubtree {
    pub base: *mut CE_GeneralNames,
    pub minimum: uint32,
    pub maximumPresent: CSSM_BOOL,
    pub maximum: uint32,
}
pub type CE_GeneralSubtree = __CE_GeneralSubtree;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_GeneralSubtrees {
    pub numSubtrees: uint32,
    pub subtrees: *mut CE_GeneralSubtree,
}
pub type CE_GeneralSubtrees = __CE_GeneralSubtrees;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_NameConstraints {
    pub permitted: *mut CE_GeneralSubtrees,
    pub excluded: *mut CE_GeneralSubtrees,
}
pub type CE_NameConstraints = __CE_NameConstraints;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_PolicyMapping {
    pub issuerDomainPolicy: SecAsn1Oid,
    pub subjectDomainPolicy: SecAsn1Oid,
}
pub type CE_PolicyMapping = __CE_PolicyMapping;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_PolicyMappings {
    pub numPolicyMappings: uint32,
    pub policyMappings: *mut CE_PolicyMapping,
}
pub type CE_PolicyMappings = __CE_PolicyMappings;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CE_PolicyConstraints {
    pub requireExplicitPolicyPresent: CSSM_BOOL,
    pub requireExplicitPolicy: uint32,
    pub inhibitPolicyMappingPresent: CSSM_BOOL,
    pub inhibitPolicyMapping: uint32,
}
pub type CE_PolicyConstraints = __CE_PolicyConstraints;
pub type CE_InhibitAnyPolicy = uint32;
pub type __CE_DataType = ::std::os::raw::c_uint;
pub use self::__CE_DataType as CE_DataType;
#[repr(C)]
#[derive(Copy, Clone)]
pub union CE_Data {
    pub authorityKeyID: CE_AuthorityKeyID,
    pub subjectKeyID: CE_SubjectKeyID,
    pub keyUsage: CE_KeyUsage,
    pub subjectAltName: CE_GeneralNames,
    pub issuerAltName: CE_GeneralNames,
    pub extendedKeyUsage: CE_ExtendedKeyUsage,
    pub basicConstraints: CE_BasicConstraints,
    pub certPolicies: CE_CertPolicies,
    pub netscapeCertType: CE_NetscapeCertType,
    pub crlNumber: CE_CrlNumber,
    pub deltaCrl: CE_DeltaCrl,
    pub crlReason: CE_CrlReason,
    pub crlDistPoints: CE_CRLDistPointsSyntax,
    pub issuingDistPoint: CE_IssuingDistributionPoint,
    pub authorityInfoAccess: CE_AuthorityInfoAccess,
    pub qualifiedCertStatements: CE_QC_Statements,
    pub nameConstraints: CE_NameConstraints,
    pub policyMappings: CE_PolicyMappings,
    pub policyConstraints: CE_PolicyConstraints,
    pub inhibitAnyPolicy: CE_InhibitAnyPolicy,
    pub rawData: SecAsn1Item,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_DataAndType {
    pub type_: CE_DataType,
    pub extension: CE_Data,
    pub critical: CSSM_BOOL,
}
pub type CE_DataAndType = __CE_DataAndType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_acl_process_subject_selector {
    pub version: uint16,
    pub mask: uint16,
    pub uid: uint32,
    pub gid: uint32,
}
pub type CSSM_ACL_PROCESS_SUBJECT_SELECTOR = cssm_acl_process_subject_selector;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_acl_keychain_prompt_selector {
    pub version: uint16,
    pub flags: uint16,
}
pub type CSSM_ACL_KEYCHAIN_PROMPT_SELECTOR = cssm_acl_keychain_prompt_selector;
pub type CSSM_ACL_PREAUTH_TRACKING_STATE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_appledl_open_parameters {
    pub length: uint32,
    pub version: uint32,
    pub autoCommit: CSSM_BOOL,
    pub mask: uint32,
    pub mode: mode_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_applecspdl_db_settings_parameters {
    pub idleTimeout: uint32,
    pub lockOnSleep: uint8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_applecspdl_db_is_locked_parameters {
    pub isLocked: uint8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_applecspdl_db_change_password_parameters {
    pub accessCredentials: *mut CSSM_ACCESS_CREDENTIALS,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_APPLE_TP_NAME_OID {
    pub string: *const ::std::os::raw::c_char,
    pub oid: *const SecAsn1Oid,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_APPLE_TP_CERT_REQUEST {
    pub cspHand: CSSM_CSP_HANDLE,
    pub clHand: CSSM_CL_HANDLE,
    pub serialNumber: uint32,
    pub numSubjectNames: uint32,
    pub subjectNames: *mut CSSM_APPLE_TP_NAME_OID,
    pub numIssuerNames: uint32,
    pub issuerNames: *mut CSSM_APPLE_TP_NAME_OID,
    pub issuerNameX509: CSSM_X509_NAME_PTR,
    pub certPublicKey: *const CSSM_KEY,
    pub issuerPrivateKey: *const CSSM_KEY,
    pub signatureAlg: CSSM_ALGORITHMS,
    pub signatureOid: SecAsn1Oid,
    pub notBefore: uint32,
    pub notAfter: uint32,
    pub numExtensions: uint32,
    pub extensions: *mut CE_DataAndType,
    pub challengeString: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_APPLE_TP_SSL_OPTIONS {
    pub Version: uint32,
    pub ServerNameLen: uint32,
    pub ServerName: *const ::std::os::raw::c_char,
    pub Flags: uint32,
}
pub type CSSM_APPLE_TP_CRL_OPT_FLAGS = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_APPLE_TP_CRL_OPTIONS {
    pub Version: uint32,
    pub CrlFlags: CSSM_APPLE_TP_CRL_OPT_FLAGS,
    pub crlStore: CSSM_DL_DB_HANDLE_PTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_APPLE_TP_SMIME_OPTIONS {
    pub Version: uint32,
    pub IntendedUsage: CE_KeyUsage,
    pub SenderEmailLen: uint32,
    pub SenderEmail: *const ::std::os::raw::c_char,
}
pub type CSSM_APPLE_TP_ACTION_FLAGS = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_APPLE_TP_ACTION_DATA {
    pub Version: uint32,
    pub ActionFlags: CSSM_APPLE_TP_ACTION_FLAGS,
}
pub type CSSM_TP_APPLE_CERT_STATUS = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_TP_APPLE_EVIDENCE_INFO {
    pub StatusBits: CSSM_TP_APPLE_CERT_STATUS,
    pub NumStatusCodes: uint32,
    pub StatusCodes: *mut CSSM_RETURN,
    pub Index: uint32,
    pub DlDbHandle: CSSM_DL_DB_HANDLE,
    pub UniqueRecord: CSSM_DB_UNIQUE_RECORD_PTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_TP_APPLE_EVIDENCE_HEADER {
    pub Version: uint32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSSM_APPLE_CL_CSR_REQUEST {
    pub subjectNameX509: CSSM_X509_NAME_PTR,
    pub signatureAlg: CSSM_ALGORITHMS,
    pub signatureOid: SecAsn1Oid,
    pub cspHand: CSSM_CSP_HANDLE,
    pub subjectPublicKey: *const CSSM_KEY,
    pub subjectPrivateKey: *const CSSM_KEY,
    pub challengeString: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecKeychainSettings {
    pub version: UInt32,
    pub lockOnSleep: Boolean,
    pub useLockInterval: Boolean,
    pub lockInterval: UInt32,
}
pub type SecAuthenticationType = FourCharCode;
pub type SecProtocolType = FourCharCode;
pub type SecKeychainEvent = UInt32;
pub type SecKeychainEventMask = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecKeychainCallbackInfo {
    pub version: UInt32,
    pub item: SecKeychainItemRef,
    pub keychain: SecKeychainRef,
    pub pid: pid_t,
}
pub type SecPreferencesDomain = ::std::os::raw::c_int;
pub type SecKeychainCallback = ::std::option::Option<
    unsafe extern "C" fn(
        keychainEvent: SecKeychainEvent,
        info: *mut SecKeychainCallbackInfo,
        context: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
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
pub type AuthorizationFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationOpaqueRef {
    _unused: [u8; 0],
}
pub type AuthorizationRef = *const AuthorizationOpaqueRef;
pub type AuthorizationString = *const ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationItem {
    pub name: AuthorizationString,
    pub valueLength: usize,
    pub value: *mut ::std::os::raw::c_void,
    pub flags: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationItemSet {
    pub count: UInt32,
    pub items: *mut AuthorizationItem,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationExternalForm {
    pub bytes: [::std::os::raw::c_char; 32usize],
}
pub type AuthorizationRights = AuthorizationItemSet;
pub type AuthorizationEnvironment = AuthorizationItemSet;
pub type AuthorizationAsyncCallback = *mut ::std::os::raw::c_void;
pub type SecuritySessionId = UInt32;
pub type SessionAttributeBits = UInt32;
pub type SessionCreationFlags = UInt32;
pub type CSSM_MANAGER_EVENT_TYPES = uint32;
pub type CSSM_CONTEXT_EVENT = uint32;
pub type CSSM_KRSP_HANDLE = uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_kr_name {
    pub Type: uint8,
    pub Length: uint8,
    pub Name: *mut ::std::os::raw::c_char,
}
pub type CSSM_KR_NAME = cssm_kr_name;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cssm_kr_profile {
    pub UserName: CSSM_KR_NAME,
    pub UserCertificate: CSSM_CERTGROUP_PTR,
    pub KRSCertChain: CSSM_CERTGROUP_PTR,
    pub LE_KRANum: uint8,
    pub LE_KRACertChainList: CSSM_CERTGROUP_PTR,
    pub ENT_KRANum: uint8,
    pub ENT_KRACertChainList: CSSM_CERTGROUP_PTR,
    pub INDIV_KRANum: uint8,
    pub INDIV_KRACertChainList: CSSM_CERTGROUP_PTR,
    pub INDIV_AuthenticationInfo: CSSM_DATA_PTR,
    pub KRSPFlags: uint32,
    pub KRSPExtensions: CSSM_DATA_PTR,
}
pub type CSSM_KR_POLICY_TYPE = uint32;
pub type CSSM_KR_POLICY_FLAGS = uint32;
pub type MDS_HANDLE = CSSM_DL_HANDLE;
pub type MDS_DB_HANDLE = CSSM_DL_DB_HANDLE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mds_funcs {
    pub DbOpen: ::std::option::Option<
        unsafe extern "C" fn(
            MdsHandle: MDS_HANDLE,
            DbName: *const ::std::os::raw::c_char,
            DbLocation: *const CSSM_NET_ADDRESS,
            AccessRequest: CSSM_DB_ACCESS_TYPE,
            AccessCred: *const CSSM_ACCESS_CREDENTIALS,
            OpenParameters: *const ::std::os::raw::c_void,
            hMds: *mut CSSM_DB_HANDLE,
        ) -> CSSM_RETURN,
    >,
    pub DbClose:
        ::std::option::Option<unsafe extern "C" fn(MdsDbHandle: MDS_DB_HANDLE) -> CSSM_RETURN>,
    pub GetDbNames: ::std::option::Option<
        unsafe extern "C" fn(
            MdsHandle: MDS_HANDLE,
            NameList: *mut CSSM_NAME_LIST_PTR,
        ) -> CSSM_RETURN,
    >,
    pub GetDbNameFromHandle: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            DbName: *mut *mut ::std::os::raw::c_char,
        ) -> CSSM_RETURN,
    >,
    pub FreeNameList: ::std::option::Option<
        unsafe extern "C" fn(MdsHandle: MDS_HANDLE, NameList: CSSM_NAME_LIST_PTR) -> CSSM_RETURN,
    >,
    pub DataInsert: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            RecordType: CSSM_DB_RECORDTYPE,
            Attributes: *const CSSM_DB_RECORD_ATTRIBUTE_DATA,
            Data: *const SecAsn1Item,
            UniqueId: *mut CSSM_DB_UNIQUE_RECORD_PTR,
        ) -> CSSM_RETURN,
    >,
    pub DataDelete: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            UniqueRecordIdentifier: *const CSSM_DB_UNIQUE_RECORD,
        ) -> CSSM_RETURN,
    >,
    pub DataModify: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            RecordType: CSSM_DB_RECORDTYPE,
            UniqueRecordIdentifier: CSSM_DB_UNIQUE_RECORD_PTR,
            AttributesToBeModified: *const CSSM_DB_RECORD_ATTRIBUTE_DATA,
            DataToBeModified: *const SecAsn1Item,
            ModifyMode: CSSM_DB_MODIFY_MODE,
        ) -> CSSM_RETURN,
    >,
    pub DataGetFirst: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            Query: *const CSSM_QUERY,
            ResultsHandle: CSSM_HANDLE_PTR,
            Attributes: CSSM_DB_RECORD_ATTRIBUTE_DATA_PTR,
            Data: CSSM_DATA_PTR,
            UniqueId: *mut CSSM_DB_UNIQUE_RECORD_PTR,
        ) -> CSSM_RETURN,
    >,
    pub DataGetNext: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            ResultsHandle: CSSM_HANDLE,
            Attributes: CSSM_DB_RECORD_ATTRIBUTE_DATA_PTR,
            Data: CSSM_DATA_PTR,
            UniqueId: *mut CSSM_DB_UNIQUE_RECORD_PTR,
        ) -> CSSM_RETURN,
    >,
    pub DataAbortQuery: ::std::option::Option<
        unsafe extern "C" fn(MdsDbHandle: MDS_DB_HANDLE, ResultsHandle: CSSM_HANDLE) -> CSSM_RETURN,
    >,
    pub DataGetFromUniqueRecordId: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            UniqueRecord: *const CSSM_DB_UNIQUE_RECORD,
            Attributes: CSSM_DB_RECORD_ATTRIBUTE_DATA_PTR,
            Data: CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub FreeUniqueRecord: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            UniqueRecord: CSSM_DB_UNIQUE_RECORD_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CreateRelation: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            RelationID: CSSM_DB_RECORDTYPE,
            RelationName: *const ::std::os::raw::c_char,
            NumberOfAttributes: uint32,
            pAttributeInfo: *const CSSM_DB_SCHEMA_ATTRIBUTE_INFO,
            NumberOfIndexes: uint32,
            pIndexInfo: *const CSSM_DB_SCHEMA_INDEX_INFO,
        ) -> CSSM_RETURN,
    >,
    pub DestroyRelation: ::std::option::Option<
        unsafe extern "C" fn(
            MdsDbHandle: MDS_DB_HANDLE,
            RelationID: CSSM_DB_RECORDTYPE,
        ) -> CSSM_RETURN,
    >,
}
pub type MDS_FUNCS_PTR = *mut mds_funcs;
pub type SecKeychainPromptSelector = uint16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueSecIdentitySearchRef {
    _unused: [u8; 0],
}
pub type SecIdentitySearchRef = *mut OpaqueSecIdentitySearchRef;
pub type SecItemClass = FourCharCode;
pub type SecItemAttr = FourCharCode;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaquePolicySearchRef {
    _unused: [u8; 0],
}
pub type SecPolicySearchRef = *mut OpaquePolicySearchRef;
pub type SecTrustSettingsKeyUsage = u32;
pub type SecTrustSettingsResult = u32;
pub type SecTrustSettingsDomain = u32;
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
pub type SecGuestRef = u32;
pub type SecCSFlags = u32;
pub type SecCodeSignatureFlags = u32;
pub type SecCodeStatus = u32;
pub type SecRequirementType = u32;
pub type SecCSDigestAlgorithm = u32;
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
pub type SecTransformRef = CFTypeRef;
pub type SecGroupTransformRef = CFTypeRef;
pub type SecMessageBlock = *mut ::std::os::raw::c_void;
pub type SecTransformMetaAttributeType = CFIndex;
pub type SecTransformStringOrAttributeRef = CFTypeRef;
pub type SecTransformActionBlock = *mut ::std::os::raw::c_void;
pub type SecTransformAttributeActionBlock = *mut ::std::os::raw::c_void;
pub type SecTransformDataBlock = *mut ::std::os::raw::c_void;
pub type SecTransformInstanceBlock = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueSecTransformImplementation {
    _unused: [u8; 0],
}
pub type SecTransformImplementationRef = *const OpaqueSecTransformImplementation;
pub type SecTransformCreateFP = ::std::option::Option<
    unsafe extern "C" fn(
        name: CFStringRef,
        newTransform: SecTransformRef,
        ref_: SecTransformImplementationRef,
    ) -> SecTransformInstanceBlock,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationValue {
    pub length: usize,
    pub data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationValueVector {
    pub count: UInt32,
    pub values: *mut AuthorizationValue,
}
pub type AuthorizationContextFlags = UInt32;
pub type AuthorizationMechanismId = AuthorizationString;
pub type AuthorizationPluginId = AuthorizationString;
pub type AuthorizationPluginRef = *mut ::std::os::raw::c_void;
pub type AuthorizationMechanismRef = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __OpaqueAuthorizationEngine {
    _unused: [u8; 0],
}
pub type AuthorizationEngineRef = *mut __OpaqueAuthorizationEngine;
pub type AuthorizationSessionId = *mut ::std::os::raw::c_void;
pub type AuthorizationResult = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationCallbacks {
    pub version: UInt32,
    pub SetResult: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            inResult: AuthorizationResult,
        ) -> OSStatus,
    >,
    pub RequestInterrupt:
        ::std::option::Option<unsafe extern "C" fn(inEngine: AuthorizationEngineRef) -> OSStatus>,
    pub DidDeactivate:
        ::std::option::Option<unsafe extern "C" fn(inEngine: AuthorizationEngineRef) -> OSStatus>,
    pub GetContextValue: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            inKey: AuthorizationString,
            outContextFlags: *mut AuthorizationContextFlags,
            outValue: *mut *const AuthorizationValue,
        ) -> OSStatus,
    >,
    pub SetContextValue: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            inKey: AuthorizationString,
            inContextFlags: AuthorizationContextFlags,
            inValue: *const AuthorizationValue,
        ) -> OSStatus,
    >,
    pub GetHintValue: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            inKey: AuthorizationString,
            outValue: *mut *const AuthorizationValue,
        ) -> OSStatus,
    >,
    pub SetHintValue: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            inKey: AuthorizationString,
            inValue: *const AuthorizationValue,
        ) -> OSStatus,
    >,
    pub GetArguments: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            outArguments: *mut *const AuthorizationValueVector,
        ) -> OSStatus,
    >,
    pub GetSessionId: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            outSessionId: *mut AuthorizationSessionId,
        ) -> OSStatus,
    >,
    pub GetImmutableHintValue: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            inKey: AuthorizationString,
            outValue: *mut *const AuthorizationValue,
        ) -> OSStatus,
    >,
    pub GetLAContext: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            outValue: *mut CFTypeRef,
        ) -> OSStatus,
    >,
    pub GetTokenIdentities: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            context: CFTypeRef,
            outValue: *mut CFArrayRef,
        ) -> OSStatus,
    >,
    pub GetTKTokenWatcher: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            outValue: *mut CFTypeRef,
        ) -> OSStatus,
    >,
    pub RemoveHintValue: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            inKey: AuthorizationString,
        ) -> OSStatus,
    >,
    pub RemoveContextValue: ::std::option::Option<
        unsafe extern "C" fn(
            inEngine: AuthorizationEngineRef,
            inKey: AuthorizationString,
        ) -> OSStatus,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationPluginInterface {
    pub version: UInt32,
    pub PluginDestroy:
        ::std::option::Option<unsafe extern "C" fn(inPlugin: AuthorizationPluginRef) -> OSStatus>,
    pub MechanismCreate: ::std::option::Option<
        unsafe extern "C" fn(
            inPlugin: AuthorizationPluginRef,
            inEngine: AuthorizationEngineRef,
            mechanismId: AuthorizationMechanismId,
            outMechanism: *mut AuthorizationMechanismRef,
        ) -> OSStatus,
    >,
    pub MechanismInvoke: ::std::option::Option<
        unsafe extern "C" fn(inMechanism: AuthorizationMechanismRef) -> OSStatus,
    >,
    pub MechanismDeactivate: ::std::option::Option<
        unsafe extern "C" fn(inMechanism: AuthorizationMechanismRef) -> OSStatus,
    >,
    pub MechanismDestroy: ::std::option::Option<
        unsafe extern "C" fn(inMechanism: AuthorizationMechanismRef) -> OSStatus,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecAsn1Coder {
    _unused: [u8; 0],
}
pub type SecAsn1CoderRef = *mut SecAsn1Coder;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueSecureDownload {
    _unused: [u8; 0],
}
pub type SecureDownloadRef = *mut OpaqueSecureDownload;
pub type _SecureDownloadTrustCallbackResult = ::std::os::raw::c_uint;
pub use self::_SecureDownloadTrustCallbackResult as SecureDownloadTrustCallbackResult;
pub type SecureDownloadTrustSetupCallback = ::std::option::Option<
    unsafe extern "C" fn(
        trustRef: SecTrustRef,
        setupContext: *mut ::std::os::raw::c_void,
    ) -> SecureDownloadTrustCallbackResult,
>;
pub type SecureDownloadTrustEvaluateCallback = ::std::option::Option<
    unsafe extern "C" fn(
        trustRef: SecTrustRef,
        result: SecTrustResultType,
        evaluateContext: *mut ::std::os::raw::c_void,
    ) -> SecTrustResultType,
>;
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
    pub fn SecCertificateCreateFromData(
        data: *const SecAsn1Item,
        type_: CSSM_CERT_TYPE,
        encoding: CSSM_CERT_ENCODING,
        certificate: *mut SecCertificateRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateAddToKeychain(
        certificate: SecCertificateRef,
        keychain: SecKeychainRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateGetData(certificate: SecCertificateRef, data: CSSM_DATA_PTR) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateGetType(
        certificate: SecCertificateRef,
        certificateType: *mut CSSM_CERT_TYPE,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateGetSubject(
        certificate: SecCertificateRef,
        subject: *mut *const CSSM_X509_NAME,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateGetIssuer(
        certificate: SecCertificateRef,
        issuer: *mut *const CSSM_X509_NAME,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateGetCLHandle(
        certificate: SecCertificateRef,
        clHandle: *mut CSSM_CL_HANDLE,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateGetAlgorithmID(
        certificate: SecCertificateRef,
        algid: *mut *const SecAsn1AlgId,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateCopyPreference(
        name: CFStringRef,
        keyUsage: uint32,
        certificate: *mut SecCertificateRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateCopyPreferred(
        name: CFStringRef,
        keyUsage: CFArrayRef,
    ) -> SecCertificateRef;
}
unsafe extern "C" {
    pub fn SecCertificateSetPreference(
        certificate: SecCertificateRef,
        name: CFStringRef,
        keyUsage: uint32,
        date: CFDateRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCertificateSetPreferred(
        certificate: SecCertificateRef,
        name: CFStringRef,
        keyUsage: CFArrayRef,
    ) -> OSStatus;
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
    pub fn SecCertificateCopyValues(
        certificate: SecCertificateRef,
        keys: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyLongDescription(
        alloc: CFAllocatorRef,
        certificate: SecCertificateRef,
        error: *mut CFErrorRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyShortDescription(
        alloc: CFAllocatorRef,
        certificate: SecCertificateRef,
        error: *mut CFErrorRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyNormalizedIssuerContent(
        certificate: SecCertificateRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecCertificateCopyNormalizedSubjectContent(
        certificate: SecCertificateRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
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
    pub fn SecIdentityCreateWithCertificate(
        keychainOrArray: CFTypeRef,
        certificateRef: SecCertificateRef,
        identityRef: *mut SecIdentityRef,
    ) -> OSStatus;
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
    pub fn SecIdentityCopyPreference(
        name: CFStringRef,
        keyUsage: CSSM_KEYUSE,
        validIssuers: CFArrayRef,
        identity: *mut SecIdentityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecIdentityCopyPreferred(
        name: CFStringRef,
        keyUsage: CFArrayRef,
        validIssuers: CFArrayRef,
    ) -> SecIdentityRef;
}
unsafe extern "C" {
    pub fn SecIdentitySetPreference(
        identity: SecIdentityRef,
        name: CFStringRef,
        keyUsage: CSSM_KEYUSE,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecIdentitySetPreferred(
        identity: SecIdentityRef,
        name: CFStringRef,
        keyUsage: CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecIdentityCopySystemIdentity(
        domain: CFStringRef,
        idRef: *mut SecIdentityRef,
        actualDomain: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecIdentitySetSystemIdentity(domain: CFStringRef, idRef: SecIdentityRef) -> OSStatus;
}
unsafe extern "C" {
    pub static kSecIdentityDomainDefault: CFStringRef;
}
unsafe extern "C" {
    pub static kSecIdentityDomainKerberosKDC: CFStringRef;
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
    pub static kSecAttrAccess: CFStringRef;
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
    pub static kSecAttrPRF: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrSalt: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrRounds: CFStringRef;
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
    pub static kSecAttrKeyTypeDSA: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeAES: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeDES: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyType3DES: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeRC4: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeRC2: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeCAST: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeECDSA: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeEC: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrKeyTypeECSECPrimeRandom: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPRFHmacAlgSHA1: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPRFHmacAlgSHA224: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPRFHmacAlgSHA256: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPRFHmacAlgSHA384: CFStringRef;
}
unsafe extern "C" {
    pub static kSecAttrPRFHmacAlgSHA512: CFStringRef;
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
    pub static kSecMatchSubjectStartsWith: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchSubjectEndsWith: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchSubjectWholeString: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchCaseInsensitive: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchDiacriticInsensitive: CFStringRef;
}
unsafe extern "C" {
    pub static kSecMatchWidthInsensitive: CFStringRef;
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
    pub static kSecUseKeychain: CFStringRef;
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
    pub static kSecACLAuthorizationAny: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationLogin: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationGenKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationDelete: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationExportWrapped: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationExportClear: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationImportWrapped: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationImportClear: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationSign: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationEncrypt: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationDecrypt: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationMAC: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationDerive: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationKeychainCreate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationKeychainDelete: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationKeychainItemRead: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationKeychainItemInsert: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationKeychainItemModify: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationKeychainItemDelete: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationChangeACL: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationChangeOwner: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationPartitionID: CFStringRef;
}
unsafe extern "C" {
    pub static kSecACLAuthorizationIntegrity: CFStringRef;
}
unsafe extern "C" {
    pub fn SecAccessGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecAccessCreate(
        descriptor: CFStringRef,
        trustedlist: CFArrayRef,
        accessRef: *mut SecAccessRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAccessCreateFromOwnerAndACL(
        owner: *const CSSM_ACL_OWNER_PROTOTYPE,
        aclCount: uint32,
        acls: *const CSSM_ACL_ENTRY_INFO,
        accessRef: *mut SecAccessRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAccessCreateWithOwnerAndACL(
        userId: uid_t,
        groupId: gid_t,
        ownerType: SecAccessOwnerType,
        acls: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> SecAccessRef;
}
unsafe extern "C" {
    pub fn SecAccessGetOwnerAndACL(
        accessRef: SecAccessRef,
        owner: *mut CSSM_ACL_OWNER_PROTOTYPE_PTR,
        aclCount: *mut uint32,
        acls: *mut CSSM_ACL_ENTRY_INFO_PTR,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAccessCopyOwnerAndACL(
        accessRef: SecAccessRef,
        userId: *mut uid_t,
        groupId: *mut gid_t,
        ownerType: *mut SecAccessOwnerType,
        aclList: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAccessCopyACLList(accessRef: SecAccessRef, aclList: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAccessCopySelectedACLList(
        accessRef: SecAccessRef,
        action: CSSM_ACL_AUTHORIZATION_TAG,
        aclList: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecAccessCopyMatchingACLList(
        accessRef: SecAccessRef,
        authorizationTag: CFTypeRef,
    ) -> CFArrayRef;
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
    pub fn SecKeyCreatePair(
        keychainRef: SecKeychainRef,
        algorithm: CSSM_ALGORITHMS,
        keySizeInBits: uint32,
        contextHandle: CSSM_CC_HANDLE,
        publicKeyUsage: CSSM_KEYUSE,
        publicKeyAttr: uint32,
        privateKeyUsage: CSSM_KEYUSE,
        privateKeyAttr: uint32,
        initialAccess: SecAccessRef,
        publicKey: *mut SecKeyRef,
        privateKey: *mut SecKeyRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeyGenerate(
        keychainRef: SecKeychainRef,
        algorithm: CSSM_ALGORITHMS,
        keySizeInBits: uint32,
        contextHandle: CSSM_CC_HANDLE,
        keyUsage: CSSM_KEYUSE,
        keyAttr: uint32,
        initialAccess: SecAccessRef,
        keyRef: *mut SecKeyRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeyGetCSSMKey(key: SecKeyRef, cssmKey: *mut *const CSSM_KEY) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeyGetCSPHandle(keyRef: SecKeyRef, cspHandle: *mut CSSM_CSP_HANDLE) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeyGetCredentials(
        keyRef: SecKeyRef,
        operation: CSSM_ACL_AUTHORIZATION_TAG,
        credentialType: SecCredentialType,
        outCredentials: *mut *const CSSM_ACCESS_CREDENTIALS,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeyGenerateSymmetric(
        parameters: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;
}
unsafe extern "C" {
    pub fn SecKeyCreateFromData(
        parameters: CFDictionaryRef,
        keyData: CFDataRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;
}
unsafe extern "C" {
    pub fn SecKeyGeneratePairAsync(
        parameters: CFDictionaryRef,
        deliveryQueue: NSObject,
        result: SecKeyGeneratePairBlock,
    );
}
unsafe extern "C" {
    pub fn SecKeyDeriveFromPassword(
        password: CFStringRef,
        parameters: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;
}
unsafe extern "C" {
    pub fn SecKeyWrapSymmetric(
        keyToWrap: SecKeyRef,
        wrappingKey: SecKeyRef,
        parameters: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SecKeyUnwrapSymmetric(
        keyToUnwrap: *mut CFDataRef,
        unwrappingKey: SecKeyRef,
        parameters: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;
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
    pub static kSecPolicyAppleiChat: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyApplePKINITClient: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyApplePKINITServer: CFStringRef;
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
    pub static kSecPolicyKU_DigitalSignature: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyKU_NonRepudiation: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyKU_KeyEncipherment: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyKU_DataEncipherment: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyKU_KeyAgreement: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyKU_KeyCertSign: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyKU_CRLSign: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyKU_EncipherOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPolicyKU_DecipherOnly: CFStringRef;
}
unsafe extern "C" {
    pub fn SecPolicyCreateWithOID(policyOID: CFTypeRef) -> SecPolicyRef;
}
unsafe extern "C" {
    pub fn SecPolicyGetOID(policyRef: SecPolicyRef, oid: *mut SecAsn1Oid) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecPolicyGetValue(policyRef: SecPolicyRef, value: *mut SecAsn1Item) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecPolicySetValue(policyRef: SecPolicyRef, value: *const SecAsn1Item) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecPolicySetProperties(policyRef: SecPolicyRef, properties: CFDictionaryRef)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn SecPolicyGetTPHandle(policyRef: SecPolicyRef, tpHandle: *mut CSSM_TP_HANDLE)
        -> OSStatus;
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
    pub static gGuidCssm: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleFileDL: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleCSP: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleCSPDL: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleX509CL: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleX509TP: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleLDAPDL: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleDotMacTP: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleSdCSPDL: CSSM_GUID;
}
unsafe extern "C" {
    pub static gGuidAppleDotMacDL: CSSM_GUID;
}
unsafe extern "C" {
    pub fn cssmPerror(how: *const ::std::os::raw::c_char, error: CSSM_RETURN);
}
unsafe extern "C" {
    pub fn cssmOidToAlg(oid: *const SecAsn1Oid, alg: *mut CSSM_ALGORITHMS) -> bool;
}
unsafe extern "C" {
    pub fn cssmAlgToOid(algId: CSSM_ALGORITHMS) -> *const SecAsn1Oid;
}
unsafe extern "C" {
    pub fn SecKeychainGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecKeychainGetVersion(returnVers: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainOpen(
        pathName: *const ::std::os::raw::c_char,
        keychain: *mut SecKeychainRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainCreate(
        pathName: *const ::std::os::raw::c_char,
        passwordLength: UInt32,
        password: *const ::std::os::raw::c_void,
        promptUser: Boolean,
        initialAccess: SecAccessRef,
        keychain: *mut SecKeychainRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainDelete(keychainOrArray: SecKeychainRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSetSettings(
        keychain: SecKeychainRef,
        newSettings: *const SecKeychainSettings,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainCopySettings(
        keychain: SecKeychainRef,
        outSettings: *mut SecKeychainSettings,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainUnlock(
        keychain: SecKeychainRef,
        passwordLength: UInt32,
        password: *const ::std::os::raw::c_void,
        usePassword: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainLock(keychain: SecKeychainRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainLockAll() -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainCopyDefault(keychain: *mut SecKeychainRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSetDefault(keychain: SecKeychainRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainCopySearchList(searchList: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSetSearchList(searchList: CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainCopyDomainDefault(
        domain: SecPreferencesDomain,
        keychain: *mut SecKeychainRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSetDomainDefault(
        domain: SecPreferencesDomain,
        keychain: SecKeychainRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainCopyDomainSearchList(
        domain: SecPreferencesDomain,
        searchList: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSetDomainSearchList(
        domain: SecPreferencesDomain,
        searchList: CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSetPreferenceDomain(domain: SecPreferencesDomain) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainGetPreferenceDomain(domain: *mut SecPreferencesDomain) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainGetStatus(
        keychain: SecKeychainRef,
        keychainStatus: *mut SecKeychainStatus,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainGetPath(
        keychain: SecKeychainRef,
        ioPathLength: *mut UInt32,
        pathName: *mut ::std::os::raw::c_char,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainAttributeInfoForItemID(
        keychain: SecKeychainRef,
        itemID: UInt32,
        info: *mut *mut SecKeychainAttributeInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainFreeAttributeInfo(info: *mut SecKeychainAttributeInfo) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainAddCallback(
        callbackFunction: SecKeychainCallback,
        eventMask: SecKeychainEventMask,
        userContext: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainRemoveCallback(callbackFunction: SecKeychainCallback) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainAddInternetPassword(
        keychain: SecKeychainRef,
        serverNameLength: UInt32,
        serverName: *const ::std::os::raw::c_char,
        securityDomainLength: UInt32,
        securityDomain: *const ::std::os::raw::c_char,
        accountNameLength: UInt32,
        accountName: *const ::std::os::raw::c_char,
        pathLength: UInt32,
        path: *const ::std::os::raw::c_char,
        port: UInt16,
        protocol: SecProtocolType,
        authenticationType: SecAuthenticationType,
        passwordLength: UInt32,
        passwordData: *const ::std::os::raw::c_void,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainFindInternetPassword(
        keychainOrArray: CFTypeRef,
        serverNameLength: UInt32,
        serverName: *const ::std::os::raw::c_char,
        securityDomainLength: UInt32,
        securityDomain: *const ::std::os::raw::c_char,
        accountNameLength: UInt32,
        accountName: *const ::std::os::raw::c_char,
        pathLength: UInt32,
        path: *const ::std::os::raw::c_char,
        port: UInt16,
        protocol: SecProtocolType,
        authenticationType: SecAuthenticationType,
        passwordLength: *mut UInt32,
        passwordData: *mut *mut ::std::os::raw::c_void,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainAddGenericPassword(
        keychain: SecKeychainRef,
        serviceNameLength: UInt32,
        serviceName: *const ::std::os::raw::c_char,
        accountNameLength: UInt32,
        accountName: *const ::std::os::raw::c_char,
        passwordLength: UInt32,
        passwordData: *const ::std::os::raw::c_void,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainFindGenericPassword(
        keychainOrArray: CFTypeRef,
        serviceNameLength: UInt32,
        serviceName: *const ::std::os::raw::c_char,
        accountNameLength: UInt32,
        accountName: *const ::std::os::raw::c_char,
        passwordLength: *mut UInt32,
        passwordData: *mut *mut ::std::os::raw::c_void,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSetUserInteractionAllowed(state: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainGetUserInteractionAllowed(state: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainGetCSPHandle(
        keychain: SecKeychainRef,
        cspHandle: *mut CSSM_CSP_HANDLE,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainGetDLDBHandle(
        keychain: SecKeychainRef,
        dldbHandle: *mut CSSM_DL_DB_HANDLE,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainCopyAccess(keychain: SecKeychainRef, access: *mut SecAccessRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSetAccess(keychain: SecKeychainRef, access: SecAccessRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemExport(
        keychainItemOrArray: CFTypeRef,
        outputFormat: SecExternalFormat,
        flags: SecItemImportExportFlags,
        keyParams: *const SecKeyImportExportParameters,
        exportedData: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecItemExport(
        secItemOrArray: CFTypeRef,
        outputFormat: SecExternalFormat,
        flags: SecItemImportExportFlags,
        keyParams: *const SecItemImportExportKeyParameters,
        exportedData: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemImport(
        importedData: CFDataRef,
        fileNameOrExtension: CFStringRef,
        inputFormat: *mut SecExternalFormat,
        itemType: *mut SecExternalItemType,
        flags: SecItemImportExportFlags,
        keyParams: *const SecKeyImportExportParameters,
        importKeychain: SecKeychainRef,
        outItems: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecItemImport(
        importedData: CFDataRef,
        fileNameOrExtension: CFStringRef,
        inputFormat: *mut SecExternalFormat,
        itemType: *mut SecExternalItemType,
        flags: SecItemImportExportFlags,
        keyParams: *const SecItemImportExportKeyParameters,
        importKeychain: SecKeychainRef,
        outItems: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kSecImportExportPassphrase: CFStringRef;
}
unsafe extern "C" {
    pub static kSecImportExportKeychain: CFStringRef;
}
unsafe extern "C" {
    pub static kSecImportExportAccess: CFStringRef;
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
    pub fn SecTrustSetOptions(trustRef: SecTrustRef, options: SecTrustOptionFlags) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSetParameters(
        trustRef: SecTrustRef,
        action: CSSM_TP_ACTION,
        actionData: CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSetKeychains(trust: SecTrustRef, keychainOrArray: CFTypeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustGetResult(
        trustRef: SecTrustRef,
        result: *mut SecTrustResultType,
        certChain: *mut CFArrayRef,
        statusChain: *mut *mut CSSM_TP_APPLE_EVIDENCE_INFO,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustGetCssmResult(
        trust: SecTrustRef,
        result: *mut CSSM_TP_VERIFY_CONTEXT_RESULT_PTR,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustGetCssmResultCode(trust: SecTrustRef, resultCode: *mut OSStatus) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustGetTPHandle(trust: SecTrustRef, handle: *mut CSSM_TP_HANDLE) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustCopyAnchorCertificates(anchors: *mut CFArrayRef) -> OSStatus;
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
    pub fn AuthorizationCreate(
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        authorization: *mut AuthorizationRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationFree(
        authorization: AuthorizationRef,
        flags: AuthorizationFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationCopyRights(
        authorization: AuthorizationRef,
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        authorizedRights: *mut *mut AuthorizationRights,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationCopyRightsAsync(
        authorization: AuthorizationRef,
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        callbackBlock: AuthorizationAsyncCallback,
    );
}
unsafe extern "C" {
    pub fn AuthorizationCopyInfo(
        authorization: AuthorizationRef,
        tag: AuthorizationString,
        info: *mut *mut AuthorizationItemSet,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationMakeExternalForm(
        authorization: AuthorizationRef,
        extForm: *mut AuthorizationExternalForm,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationCreateFromExternalForm(
        extForm: *const AuthorizationExternalForm,
        authorization: *mut AuthorizationRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationFreeItemSet(set: *mut AuthorizationItemSet) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationExecuteWithPrivileges(
        authorization: AuthorizationRef,
        pathToTool: *const ::std::os::raw::c_char,
        options: AuthorizationFlags,
        arguments: *const *mut ::std::os::raw::c_char,
        communicationsPipe: *mut *mut FILE,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationCopyPrivilegedReference(
        authorization: *mut AuthorizationRef,
        flags: AuthorizationFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SessionGetInfo(
        session: SecuritySessionId,
        sessionId: *mut SecuritySessionId,
        attributes: *mut SessionAttributeBits,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SessionCreate(flags: SessionCreationFlags, attributes: SessionAttributeBits)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn CSSM_Init(
        Version: *const CSSM_VERSION,
        Scope: CSSM_PRIVILEGE_SCOPE,
        CallerGuid: *const CSSM_GUID,
        KeyHierarchy: CSSM_KEY_HIERARCHY,
        PvcPolicy: *mut CSSM_PVC_MODE,
        Reserved: *const ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_Terminate() -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_ModuleLoad(
        ModuleGuid: *const CSSM_GUID,
        KeyHierarchy: CSSM_KEY_HIERARCHY,
        AppNotifyCallback: CSSM_API_ModuleEventHandler,
        AppNotifyCallbackCtx: *mut ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_ModuleUnload(
        ModuleGuid: *const CSSM_GUID,
        AppNotifyCallback: CSSM_API_ModuleEventHandler,
        AppNotifyCallbackCtx: *mut ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_Introduce(
        ModuleID: *const CSSM_GUID,
        KeyHierarchy: CSSM_KEY_HIERARCHY,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_Unintroduce(ModuleID: *const CSSM_GUID) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_ModuleAttach(
        ModuleGuid: *const CSSM_GUID,
        Version: *const CSSM_VERSION,
        MemoryFuncs: *const CSSM_API_MEMORY_FUNCS,
        SubserviceID: uint32,
        SubServiceType: CSSM_SERVICE_TYPE,
        AttachFlags: CSSM_ATTACH_FLAGS,
        KeyHierarchy: CSSM_KEY_HIERARCHY,
        FunctionTable: *mut CSSM_FUNC_NAME_ADDR,
        NumFunctionTable: uint32,
        Reserved: *const ::std::os::raw::c_void,
        NewModuleHandle: CSSM_MODULE_HANDLE_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_ModuleDetach(ModuleHandle: CSSM_MODULE_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_SetPrivilege(Privilege: CSSM_PRIVILEGE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetPrivilege(Privilege: *mut CSSM_PRIVILEGE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetModuleGUIDFromHandle(
        ModuleHandle: CSSM_MODULE_HANDLE,
        ModuleGUID: CSSM_GUID_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetSubserviceUIDFromHandle(
        ModuleHandle: CSSM_MODULE_HANDLE,
        SubserviceUID: CSSM_SUBSERVICE_UID_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_ListAttachedModuleManagers(
        NumberOfModuleManagers: *mut uint32,
        ModuleManagerGuids: CSSM_GUID_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetAPIMemoryFunctions(
        AddInHandle: CSSM_MODULE_HANDLE,
        AppMemoryFuncs: CSSM_API_MEMORY_FUNCS_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreateSignatureContext(
        CSPHandle: CSSM_CSP_HANDLE,
        AlgorithmID: CSSM_ALGORITHMS,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        Key: *const CSSM_KEY,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreateSymmetricContext(
        CSPHandle: CSSM_CSP_HANDLE,
        AlgorithmID: CSSM_ALGORITHMS,
        Mode: CSSM_ENCRYPT_MODE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        Key: *const CSSM_KEY,
        InitVector: *const SecAsn1Item,
        Padding: CSSM_PADDING,
        Reserved: *mut ::std::os::raw::c_void,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreateDigestContext(
        CSPHandle: CSSM_CSP_HANDLE,
        AlgorithmID: CSSM_ALGORITHMS,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreateMacContext(
        CSPHandle: CSSM_CSP_HANDLE,
        AlgorithmID: CSSM_ALGORITHMS,
        Key: *const CSSM_KEY,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreateRandomGenContext(
        CSPHandle: CSSM_CSP_HANDLE,
        AlgorithmID: CSSM_ALGORITHMS,
        Seed: *const CSSM_CRYPTO_DATA,
        Length: CSSM_SIZE,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreateAsymmetricContext(
        CSPHandle: CSSM_CSP_HANDLE,
        AlgorithmID: CSSM_ALGORITHMS,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        Key: *const CSSM_KEY,
        Padding: CSSM_PADDING,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreateDeriveKeyContext(
        CSPHandle: CSSM_CSP_HANDLE,
        AlgorithmID: CSSM_ALGORITHMS,
        DeriveKeyType: CSSM_KEY_TYPE,
        DeriveKeyLengthInBits: uint32,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        BaseKey: *const CSSM_KEY,
        IterationCount: uint32,
        Salt: *const SecAsn1Item,
        Seed: *const CSSM_CRYPTO_DATA,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreateKeyGenContext(
        CSPHandle: CSSM_CSP_HANDLE,
        AlgorithmID: CSSM_ALGORITHMS,
        KeySizeInBits: uint32,
        Seed: *const CSSM_CRYPTO_DATA,
        Salt: *const SecAsn1Item,
        StartDate: *const CSSM_DATE,
        EndDate: *const CSSM_DATE,
        Params: *const SecAsn1Item,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_CreatePassThroughContext(
        CSPHandle: CSSM_CSP_HANDLE,
        Key: *const CSSM_KEY,
        NewContextHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetContext(CCHandle: CSSM_CC_HANDLE, Context: *mut CSSM_CONTEXT_PTR)
        -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_FreeContext(Context: CSSM_CONTEXT_PTR) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_SetContext(CCHandle: CSSM_CC_HANDLE, Context: *const CSSM_CONTEXT) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DeleteContext(CCHandle: CSSM_CC_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetContextAttribute(
        Context: *const CSSM_CONTEXT,
        AttributeType: uint32,
        ContextAttribute: *mut CSSM_CONTEXT_ATTRIBUTE_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_UpdateContextAttributes(
        CCHandle: CSSM_CC_HANDLE,
        NumberOfAttributes: uint32,
        ContextAttributes: *const CSSM_CONTEXT_ATTRIBUTE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DeleteContextAttributes(
        CCHandle: CSSM_CC_HANDLE,
        NumberOfAttributes: uint32,
        ContextAttributes: *const CSSM_CONTEXT_ATTRIBUTE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_Login(
        CSPHandle: CSSM_CSP_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        LoginName: *const SecAsn1Item,
        Reserved: *const ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_Logout(CSPHandle: CSSM_CSP_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_GetLoginAcl(
        CSPHandle: CSSM_CSP_HANDLE,
        SelectionTag: *const CSSM_STRING,
        NumberOfAclInfos: *mut uint32,
        AclInfos: *mut CSSM_ACL_ENTRY_INFO_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_ChangeLoginAcl(
        CSPHandle: CSSM_CSP_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        AclEdit: *const CSSM_ACL_EDIT,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetKeyAcl(
        CSPHandle: CSSM_CSP_HANDLE,
        Key: *const CSSM_KEY,
        SelectionTag: *const CSSM_STRING,
        NumberOfAclInfos: *mut uint32,
        AclInfos: *mut CSSM_ACL_ENTRY_INFO_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_ChangeKeyAcl(
        CSPHandle: CSSM_CSP_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        AclEdit: *const CSSM_ACL_EDIT,
        Key: *const CSSM_KEY,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetKeyOwner(
        CSPHandle: CSSM_CSP_HANDLE,
        Key: *const CSSM_KEY,
        Owner: CSSM_ACL_OWNER_PROTOTYPE_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_ChangeKeyOwner(
        CSPHandle: CSSM_CSP_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        Key: *const CSSM_KEY,
        NewOwner: *const CSSM_ACL_OWNER_PROTOTYPE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_GetLoginOwner(
        CSPHandle: CSSM_CSP_HANDLE,
        Owner: CSSM_ACL_OWNER_PROTOTYPE_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_ChangeLoginOwner(
        CSPHandle: CSSM_CSP_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        NewOwner: *const CSSM_ACL_OWNER_PROTOTYPE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_SignData(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
        DigestAlgorithm: CSSM_ALGORITHMS,
        Signature: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_SignDataInit(CCHandle: CSSM_CC_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_SignDataUpdate(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_SignDataFinal(CCHandle: CSSM_CC_HANDLE, Signature: CSSM_DATA_PTR) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyData(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
        DigestAlgorithm: CSSM_ALGORITHMS,
        Signature: *const SecAsn1Item,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyDataInit(CCHandle: CSSM_CC_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyDataUpdate(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyDataFinal(
        CCHandle: CSSM_CC_HANDLE,
        Signature: *const SecAsn1Item,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DigestData(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
        Digest: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DigestDataInit(CCHandle: CSSM_CC_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DigestDataUpdate(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DigestDataClone(
        CCHandle: CSSM_CC_HANDLE,
        ClonednewCCHandle: *mut CSSM_CC_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DigestDataFinal(CCHandle: CSSM_CC_HANDLE, Digest: CSSM_DATA_PTR) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateMac(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
        Mac: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateMacInit(CCHandle: CSSM_CC_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateMacUpdate(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateMacFinal(CCHandle: CSSM_CC_HANDLE, Mac: CSSM_DATA_PTR) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyMac(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
        Mac: *const SecAsn1Item,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyMacInit(CCHandle: CSSM_CC_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyMacUpdate(
        CCHandle: CSSM_CC_HANDLE,
        DataBufs: *const SecAsn1Item,
        DataBufCount: uint32,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyMacFinal(CCHandle: CSSM_CC_HANDLE, Mac: *const SecAsn1Item) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_QuerySize(
        CCHandle: CSSM_CC_HANDLE,
        Encrypt: CSSM_BOOL,
        QuerySizeCount: uint32,
        DataBlockSizes: CSSM_QUERY_SIZE_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_EncryptData(
        CCHandle: CSSM_CC_HANDLE,
        ClearBufs: *const SecAsn1Item,
        ClearBufCount: uint32,
        CipherBufs: CSSM_DATA_PTR,
        CipherBufCount: uint32,
        bytesEncrypted: *mut CSSM_SIZE,
        RemData: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_EncryptDataP(
        CCHandle: CSSM_CC_HANDLE,
        ClearBufs: *const SecAsn1Item,
        ClearBufCount: uint32,
        CipherBufs: CSSM_DATA_PTR,
        CipherBufCount: uint32,
        bytesEncrypted: *mut CSSM_SIZE,
        RemData: CSSM_DATA_PTR,
        Privilege: CSSM_PRIVILEGE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_EncryptDataInit(CCHandle: CSSM_CC_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_EncryptDataInitP(
        CCHandle: CSSM_CC_HANDLE,
        Privilege: CSSM_PRIVILEGE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_EncryptDataUpdate(
        CCHandle: CSSM_CC_HANDLE,
        ClearBufs: *const SecAsn1Item,
        ClearBufCount: uint32,
        CipherBufs: CSSM_DATA_PTR,
        CipherBufCount: uint32,
        bytesEncrypted: *mut CSSM_SIZE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_EncryptDataFinal(CCHandle: CSSM_CC_HANDLE, RemData: CSSM_DATA_PTR) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DecryptData(
        CCHandle: CSSM_CC_HANDLE,
        CipherBufs: *const SecAsn1Item,
        CipherBufCount: uint32,
        ClearBufs: CSSM_DATA_PTR,
        ClearBufCount: uint32,
        bytesDecrypted: *mut CSSM_SIZE,
        RemData: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DecryptDataP(
        CCHandle: CSSM_CC_HANDLE,
        CipherBufs: *const SecAsn1Item,
        CipherBufCount: uint32,
        ClearBufs: CSSM_DATA_PTR,
        ClearBufCount: uint32,
        bytesDecrypted: *mut CSSM_SIZE,
        RemData: CSSM_DATA_PTR,
        Privilege: CSSM_PRIVILEGE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DecryptDataInit(CCHandle: CSSM_CC_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DecryptDataInitP(
        CCHandle: CSSM_CC_HANDLE,
        Privilege: CSSM_PRIVILEGE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DecryptDataUpdate(
        CCHandle: CSSM_CC_HANDLE,
        CipherBufs: *const SecAsn1Item,
        CipherBufCount: uint32,
        ClearBufs: CSSM_DATA_PTR,
        ClearBufCount: uint32,
        bytesDecrypted: *mut CSSM_SIZE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DecryptDataFinal(CCHandle: CSSM_CC_HANDLE, RemData: CSSM_DATA_PTR) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_QueryKeySizeInBits(
        CSPHandle: CSSM_CSP_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        Key: *const CSSM_KEY,
        KeySize: CSSM_KEY_SIZE_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateKey(
        CCHandle: CSSM_CC_HANDLE,
        KeyUsage: uint32,
        KeyAttr: uint32,
        KeyLabel: *const SecAsn1Item,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
        Key: CSSM_KEY_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateKeyP(
        CCHandle: CSSM_CC_HANDLE,
        KeyUsage: uint32,
        KeyAttr: uint32,
        KeyLabel: *const SecAsn1Item,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
        Key: CSSM_KEY_PTR,
        Privilege: CSSM_PRIVILEGE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateKeyPair(
        CCHandle: CSSM_CC_HANDLE,
        PublicKeyUsage: uint32,
        PublicKeyAttr: uint32,
        PublicKeyLabel: *const SecAsn1Item,
        PublicKey: CSSM_KEY_PTR,
        PrivateKeyUsage: uint32,
        PrivateKeyAttr: uint32,
        PrivateKeyLabel: *const SecAsn1Item,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
        PrivateKey: CSSM_KEY_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateKeyPairP(
        CCHandle: CSSM_CC_HANDLE,
        PublicKeyUsage: uint32,
        PublicKeyAttr: uint32,
        PublicKeyLabel: *const SecAsn1Item,
        PublicKey: CSSM_KEY_PTR,
        PrivateKeyUsage: uint32,
        PrivateKeyAttr: uint32,
        PrivateKeyLabel: *const SecAsn1Item,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
        PrivateKey: CSSM_KEY_PTR,
        Privilege: CSSM_PRIVILEGE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateRandom(
        CCHandle: CSSM_CC_HANDLE,
        RandomNumber: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_ObtainPrivateKeyFromPublicKey(
        CSPHandle: CSSM_CSP_HANDLE,
        PublicKey: *const CSSM_KEY,
        PrivateKey: CSSM_KEY_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_WrapKey(
        CCHandle: CSSM_CC_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        Key: *const CSSM_KEY,
        DescriptiveData: *const SecAsn1Item,
        WrappedKey: CSSM_WRAP_KEY_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_UnwrapKey(
        CCHandle: CSSM_CC_HANDLE,
        PublicKey: *const CSSM_KEY,
        WrappedKey: *const CSSM_WRAP_KEY,
        KeyUsage: uint32,
        KeyAttr: uint32,
        KeyLabel: *const SecAsn1Item,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
        UnwrappedKey: CSSM_KEY_PTR,
        DescriptiveData: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_WrapKeyP(
        CCHandle: CSSM_CC_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        Key: *const CSSM_KEY,
        DescriptiveData: *const SecAsn1Item,
        WrappedKey: CSSM_WRAP_KEY_PTR,
        Privilege: CSSM_PRIVILEGE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_UnwrapKeyP(
        CCHandle: CSSM_CC_HANDLE,
        PublicKey: *const CSSM_KEY,
        WrappedKey: *const CSSM_WRAP_KEY,
        KeyUsage: uint32,
        KeyAttr: uint32,
        KeyLabel: *const SecAsn1Item,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
        UnwrappedKey: CSSM_KEY_PTR,
        DescriptiveData: CSSM_DATA_PTR,
        Privilege: CSSM_PRIVILEGE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DeriveKey(
        CCHandle: CSSM_CC_HANDLE,
        Param: CSSM_DATA_PTR,
        KeyUsage: uint32,
        KeyAttr: uint32,
        KeyLabel: *const SecAsn1Item,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
        DerivedKey: CSSM_KEY_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_FreeKey(
        CSPHandle: CSSM_CSP_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        KeyPtr: CSSM_KEY_PTR,
        Delete: CSSM_BOOL,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GenerateAlgorithmParams(
        CCHandle: CSSM_CC_HANDLE,
        ParamBits: uint32,
        Param: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_GetOperationalStatistics(
        CSPHandle: CSSM_CSP_HANDLE,
        Statistics: *mut CSSM_CSP_OPERATIONAL_STATISTICS,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_GetTimeValue(
        CSPHandle: CSSM_CSP_HANDLE,
        TimeAlgorithm: CSSM_ALGORITHMS,
        TimeData: *mut SecAsn1Item,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_RetrieveUniqueId(
        CSPHandle: CSSM_CSP_HANDLE,
        UniqueID: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_RetrieveCounter(CSPHandle: CSSM_CSP_HANDLE, Counter: CSSM_DATA_PTR) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_VerifyDevice(
        CSPHandle: CSSM_CSP_HANDLE,
        DeviceCert: *const SecAsn1Item,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CSP_PassThrough(
        CCHandle: CSSM_CC_HANDLE,
        PassThroughId: uint32,
        InData: *const ::std::os::raw::c_void,
        OutData: *mut *mut ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_SubmitCredRequest(
        TPHandle: CSSM_TP_HANDLE,
        PreferredAuthority: *const CSSM_TP_AUTHORITY_ID,
        RequestType: CSSM_TP_AUTHORITY_REQUEST_TYPE,
        RequestInput: *const CSSM_TP_REQUEST_SET,
        CallerAuthContext: *const CSSM_TP_CALLERAUTH_CONTEXT,
        EstimatedTime: *mut sint32,
        ReferenceIdentifier: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_RetrieveCredResult(
        TPHandle: CSSM_TP_HANDLE,
        ReferenceIdentifier: *const SecAsn1Item,
        CallerAuthCredentials: *const CSSM_TP_CALLERAUTH_CONTEXT,
        EstimatedTime: *mut sint32,
        ConfirmationRequired: *mut CSSM_BOOL,
        RetrieveOutput: *mut CSSM_TP_RESULT_SET_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_ConfirmCredResult(
        TPHandle: CSSM_TP_HANDLE,
        ReferenceIdentifier: *const SecAsn1Item,
        CallerAuthCredentials: *const CSSM_TP_CALLERAUTH_CONTEXT,
        Responses: *const CSSM_TP_CONFIRM_RESPONSE,
        PreferredAuthority: *const CSSM_TP_AUTHORITY_ID,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_ReceiveConfirmation(
        TPHandle: CSSM_TP_HANDLE,
        ReferenceIdentifier: *const SecAsn1Item,
        Responses: *mut CSSM_TP_CONFIRM_RESPONSE_PTR,
        ElapsedTime: *mut sint32,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertReclaimKey(
        TPHandle: CSSM_TP_HANDLE,
        CertGroup: *const CSSM_CERTGROUP,
        CertIndex: uint32,
        KeyCacheHandle: CSSM_LONG_HANDLE,
        CSPHandle: CSSM_CSP_HANDLE,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertReclaimAbort(
        TPHandle: CSSM_TP_HANDLE,
        KeyCacheHandle: CSSM_LONG_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_FormRequest(
        TPHandle: CSSM_TP_HANDLE,
        PreferredAuthority: *const CSSM_TP_AUTHORITY_ID,
        FormType: CSSM_TP_FORM_TYPE,
        BlankForm: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_FormSubmit(
        TPHandle: CSSM_TP_HANDLE,
        FormType: CSSM_TP_FORM_TYPE,
        Form: *const SecAsn1Item,
        ClearanceAuthority: *const CSSM_TP_AUTHORITY_ID,
        RepresentedAuthority: *const CSSM_TP_AUTHORITY_ID,
        Credentials: CSSM_ACCESS_CREDENTIALS_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertGroupVerify(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CSPHandle: CSSM_CSP_HANDLE,
        CertGroupToBeVerified: *const CSSM_CERTGROUP,
        VerifyContext: *const CSSM_TP_VERIFY_CONTEXT,
        VerifyContextResult: CSSM_TP_VERIFY_CONTEXT_RESULT_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertCreateTemplate(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        NumberOfFields: uint32,
        CertFields: *const CSSM_FIELD,
        CertTemplate: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertGetAllTemplateFields(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CertTemplate: *const SecAsn1Item,
        NumberOfFields: *mut uint32,
        CertFields: *mut CSSM_FIELD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertSign(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CertTemplateToBeSigned: *const SecAsn1Item,
        SignerCertGroup: *const CSSM_CERTGROUP,
        SignerVerifyContext: *const CSSM_TP_VERIFY_CONTEXT,
        SignerVerifyResult: CSSM_TP_VERIFY_CONTEXT_RESULT_PTR,
        SignedCert: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CrlVerify(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CSPHandle: CSSM_CSP_HANDLE,
        CrlToBeVerified: *const CSSM_ENCODED_CRL,
        SignerCertGroup: *const CSSM_CERTGROUP,
        VerifyContext: *const CSSM_TP_VERIFY_CONTEXT,
        RevokerVerifyResult: CSSM_TP_VERIFY_CONTEXT_RESULT_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CrlCreateTemplate(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        NumberOfFields: uint32,
        CrlFields: *const CSSM_FIELD,
        NewCrlTemplate: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertRevoke(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CSPHandle: CSSM_CSP_HANDLE,
        OldCrlTemplate: *const SecAsn1Item,
        CertGroupToBeRevoked: *const CSSM_CERTGROUP,
        RevokerCertGroup: *const CSSM_CERTGROUP,
        RevokerVerifyContext: *const CSSM_TP_VERIFY_CONTEXT,
        RevokerVerifyResult: CSSM_TP_VERIFY_CONTEXT_RESULT_PTR,
        Reason: CSSM_TP_CERTCHANGE_REASON,
        NewCrlTemplate: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertRemoveFromCrlTemplate(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CSPHandle: CSSM_CSP_HANDLE,
        OldCrlTemplate: *const SecAsn1Item,
        CertGroupToBeRemoved: *const CSSM_CERTGROUP,
        RevokerCertGroup: *const CSSM_CERTGROUP,
        RevokerVerifyContext: *const CSSM_TP_VERIFY_CONTEXT,
        RevokerVerifyResult: CSSM_TP_VERIFY_CONTEXT_RESULT_PTR,
        NewCrlTemplate: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CrlSign(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CrlToBeSigned: *const CSSM_ENCODED_CRL,
        SignerCertGroup: *const CSSM_CERTGROUP,
        SignerVerifyContext: *const CSSM_TP_VERIFY_CONTEXT,
        SignerVerifyResult: CSSM_TP_VERIFY_CONTEXT_RESULT_PTR,
        SignedCrl: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_ApplyCrlToDb(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CSPHandle: CSSM_CSP_HANDLE,
        CrlToBeApplied: *const CSSM_ENCODED_CRL,
        SignerCertGroup: *const CSSM_CERTGROUP,
        ApplyCrlVerifyContext: *const CSSM_TP_VERIFY_CONTEXT,
        ApplyCrlVerifyResult: CSSM_TP_VERIFY_CONTEXT_RESULT_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertGroupConstruct(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CSPHandle: CSSM_CSP_HANDLE,
        DBList: *const CSSM_DL_DB_LIST,
        ConstructParams: *const ::std::os::raw::c_void,
        CertGroupFrag: *const CSSM_CERTGROUP,
        CertGroup: *mut CSSM_CERTGROUP_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertGroupPrune(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        DBList: *const CSSM_DL_DB_LIST,
        OrderedCertGroup: *const CSSM_CERTGROUP,
        PrunedCertGroup: *mut CSSM_CERTGROUP_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_CertGroupToTupleGroup(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CertGroup: *const CSSM_CERTGROUP,
        TupleGroup: *mut CSSM_TUPLEGROUP_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_TupleGroupToCertGroup(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        TupleGroup: *const CSSM_TUPLEGROUP,
        CertTemplates: *mut CSSM_CERTGROUP_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_TP_PassThrough(
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        DBList: *const CSSM_DL_DB_LIST,
        PassThroughId: uint32,
        InputParams: *const ::std::os::raw::c_void,
        OutputParams: *mut *mut ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_AC_AuthCompute(
        ACHandle: CSSM_AC_HANDLE,
        BaseAuthorizations: *const CSSM_TUPLEGROUP,
        Credentials: *const CSSM_TUPLEGROUP,
        NumberOfRequestors: uint32,
        Requestors: *const CSSM_LIST,
        RequestedAuthorizationPeriod: *const CSSM_LIST,
        RequestedAuthorization: *const CSSM_LIST,
        AuthorizationResult: CSSM_TUPLEGROUP_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_AC_PassThrough(
        ACHandle: CSSM_AC_HANDLE,
        TPHandle: CSSM_TP_HANDLE,
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        DBList: *const CSSM_DL_DB_LIST,
        PassThroughId: uint32,
        InputParams: *const ::std::os::raw::c_void,
        OutputParams: *mut *mut ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertCreateTemplate(
        CLHandle: CSSM_CL_HANDLE,
        NumberOfFields: uint32,
        CertFields: *const CSSM_FIELD,
        CertTemplate: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGetAllTemplateFields(
        CLHandle: CSSM_CL_HANDLE,
        CertTemplate: *const SecAsn1Item,
        NumberOfFields: *mut uint32,
        CertFields: *mut CSSM_FIELD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertSign(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CertTemplate: *const SecAsn1Item,
        SignScope: *const CSSM_FIELD,
        ScopeSize: uint32,
        SignedCert: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertVerify(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CertToBeVerified: *const SecAsn1Item,
        SignerCert: *const SecAsn1Item,
        VerifyScope: *const CSSM_FIELD,
        ScopeSize: uint32,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertVerifyWithKey(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CertToBeVerified: *const SecAsn1Item,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGetFirstFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        Cert: *const SecAsn1Item,
        CertField: *const SecAsn1Oid,
        ResultsHandle: CSSM_HANDLE_PTR,
        NumberOfMatchedFields: *mut uint32,
        Value: *mut CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGetNextFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        ResultsHandle: CSSM_HANDLE,
        Value: *mut CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertAbortQuery(
        CLHandle: CSSM_CL_HANDLE,
        ResultsHandle: CSSM_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGetKeyInfo(
        CLHandle: CSSM_CL_HANDLE,
        Cert: *const SecAsn1Item,
        Key: *mut CSSM_KEY_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGetAllFields(
        CLHandle: CSSM_CL_HANDLE,
        Cert: *const SecAsn1Item,
        NumberOfFields: *mut uint32,
        CertFields: *mut CSSM_FIELD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_FreeFields(
        CLHandle: CSSM_CL_HANDLE,
        NumberOfFields: uint32,
        Fields: *mut CSSM_FIELD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_FreeFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        CertOrCrlOid: *const SecAsn1Oid,
        Value: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertCache(
        CLHandle: CSSM_CL_HANDLE,
        Cert: *const SecAsn1Item,
        CertHandle: CSSM_HANDLE_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGetFirstCachedFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        CertHandle: CSSM_HANDLE,
        CertField: *const SecAsn1Oid,
        ResultsHandle: CSSM_HANDLE_PTR,
        NumberOfMatchedFields: *mut uint32,
        Value: *mut CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGetNextCachedFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        ResultsHandle: CSSM_HANDLE,
        Value: *mut CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertAbortCache(CLHandle: CSSM_CL_HANDLE, CertHandle: CSSM_HANDLE)
        -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGroupToSignedBundle(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CertGroupToBundle: *const CSSM_CERTGROUP,
        BundleInfo: *const CSSM_CERT_BUNDLE_HEADER,
        SignedBundle: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertGroupFromVerifiedBundle(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CertBundle: *const CSSM_CERT_BUNDLE,
        SignerCert: *const SecAsn1Item,
        CertGroup: *mut CSSM_CERTGROUP_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CertDescribeFormat(
        CLHandle: CSSM_CL_HANDLE,
        NumberOfFields: *mut uint32,
        OidList: *mut CSSM_OID_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlCreateTemplate(
        CLHandle: CSSM_CL_HANDLE,
        NumberOfFields: uint32,
        CrlTemplate: *const CSSM_FIELD,
        NewCrl: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlSetFields(
        CLHandle: CSSM_CL_HANDLE,
        NumberOfFields: uint32,
        CrlTemplate: *const CSSM_FIELD,
        OldCrl: *const SecAsn1Item,
        ModifiedCrl: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlAddCert(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        Cert: *const SecAsn1Item,
        NumberOfFields: uint32,
        CrlEntryFields: *const CSSM_FIELD,
        OldCrl: *const SecAsn1Item,
        NewCrl: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlRemoveCert(
        CLHandle: CSSM_CL_HANDLE,
        Cert: *const SecAsn1Item,
        OldCrl: *const SecAsn1Item,
        NewCrl: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlSign(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        UnsignedCrl: *const SecAsn1Item,
        SignScope: *const CSSM_FIELD,
        ScopeSize: uint32,
        SignedCrl: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlVerify(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CrlToBeVerified: *const SecAsn1Item,
        SignerCert: *const SecAsn1Item,
        VerifyScope: *const CSSM_FIELD,
        ScopeSize: uint32,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlVerifyWithKey(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        CrlToBeVerified: *const SecAsn1Item,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_IsCertInCrl(
        CLHandle: CSSM_CL_HANDLE,
        Cert: *const SecAsn1Item,
        Crl: *const SecAsn1Item,
        CertFound: *mut CSSM_BOOL,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlGetFirstFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        Crl: *const SecAsn1Item,
        CrlField: *const SecAsn1Oid,
        ResultsHandle: CSSM_HANDLE_PTR,
        NumberOfMatchedFields: *mut uint32,
        Value: *mut CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlGetNextFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        ResultsHandle: CSSM_HANDLE,
        Value: *mut CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlAbortQuery(
        CLHandle: CSSM_CL_HANDLE,
        ResultsHandle: CSSM_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlGetAllFields(
        CLHandle: CSSM_CL_HANDLE,
        Crl: *const SecAsn1Item,
        NumberOfCrlFields: *mut uint32,
        CrlFields: *mut CSSM_FIELD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlCache(
        CLHandle: CSSM_CL_HANDLE,
        Crl: *const SecAsn1Item,
        CrlHandle: CSSM_HANDLE_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_IsCertInCachedCrl(
        CLHandle: CSSM_CL_HANDLE,
        Cert: *const SecAsn1Item,
        CrlHandle: CSSM_HANDLE,
        CertFound: *mut CSSM_BOOL,
        CrlRecordIndex: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlGetFirstCachedFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        CrlHandle: CSSM_HANDLE,
        CrlRecordIndex: *const SecAsn1Item,
        CrlField: *const SecAsn1Oid,
        ResultsHandle: CSSM_HANDLE_PTR,
        NumberOfMatchedFields: *mut uint32,
        Value: *mut CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlGetNextCachedFieldValue(
        CLHandle: CSSM_CL_HANDLE,
        ResultsHandle: CSSM_HANDLE,
        Value: *mut CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlGetAllCachedRecordFields(
        CLHandle: CSSM_CL_HANDLE,
        CrlHandle: CSSM_HANDLE,
        CrlRecordIndex: *const SecAsn1Item,
        NumberOfFields: *mut uint32,
        CrlFields: *mut CSSM_FIELD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlAbortCache(CLHandle: CSSM_CL_HANDLE, CrlHandle: CSSM_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_CrlDescribeFormat(
        CLHandle: CSSM_CL_HANDLE,
        NumberOfFields: *mut uint32,
        OidList: *mut CSSM_OID_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_CL_PassThrough(
        CLHandle: CSSM_CL_HANDLE,
        CCHandle: CSSM_CC_HANDLE,
        PassThroughId: uint32,
        InputParams: *const ::std::os::raw::c_void,
        OutputParams: *mut *mut ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DbOpen(
        DLHandle: CSSM_DL_HANDLE,
        DbName: *const ::std::os::raw::c_char,
        DbLocation: *const CSSM_NET_ADDRESS,
        AccessRequest: CSSM_DB_ACCESS_TYPE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        OpenParameters: *const ::std::os::raw::c_void,
        DbHandle: *mut CSSM_DB_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DbClose(DLDBHandle: CSSM_DL_DB_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DbCreate(
        DLHandle: CSSM_DL_HANDLE,
        DbName: *const ::std::os::raw::c_char,
        DbLocation: *const CSSM_NET_ADDRESS,
        DBInfo: *const CSSM_DBINFO,
        AccessRequest: CSSM_DB_ACCESS_TYPE,
        CredAndAclEntry: *const CSSM_RESOURCE_CONTROL_CONTEXT,
        OpenParameters: *const ::std::os::raw::c_void,
        DbHandle: *mut CSSM_DB_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DbDelete(
        DLHandle: CSSM_DL_HANDLE,
        DbName: *const ::std::os::raw::c_char,
        DbLocation: *const CSSM_NET_ADDRESS,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_CreateRelation(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        RelationID: CSSM_DB_RECORDTYPE,
        RelationName: *const ::std::os::raw::c_char,
        NumberOfAttributes: uint32,
        pAttributeInfo: *const CSSM_DB_SCHEMA_ATTRIBUTE_INFO,
        NumberOfIndexes: uint32,
        pIndexInfo: *const CSSM_DB_SCHEMA_INDEX_INFO,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DestroyRelation(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        RelationID: CSSM_DB_RECORDTYPE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_Authenticate(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        AccessRequest: CSSM_DB_ACCESS_TYPE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_GetDbAcl(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        SelectionTag: *const CSSM_STRING,
        NumberOfAclInfos: *mut uint32,
        AclInfos: *mut CSSM_ACL_ENTRY_INFO_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_ChangeDbAcl(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        AclEdit: *const CSSM_ACL_EDIT,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_GetDbOwner(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        Owner: CSSM_ACL_OWNER_PROTOTYPE_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_ChangeDbOwner(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        AccessCred: *const CSSM_ACCESS_CREDENTIALS,
        NewOwner: *const CSSM_ACL_OWNER_PROTOTYPE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_GetDbNames(
        DLHandle: CSSM_DL_HANDLE,
        NameList: *mut CSSM_NAME_LIST_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_GetDbNameFromHandle(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        DbName: *mut *mut ::std::os::raw::c_char,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_FreeNameList(
        DLHandle: CSSM_DL_HANDLE,
        NameList: CSSM_NAME_LIST_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DataInsert(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        RecordType: CSSM_DB_RECORDTYPE,
        Attributes: *const CSSM_DB_RECORD_ATTRIBUTE_DATA,
        Data: *const SecAsn1Item,
        UniqueId: *mut CSSM_DB_UNIQUE_RECORD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DataDelete(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        UniqueRecordIdentifier: *const CSSM_DB_UNIQUE_RECORD,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DataModify(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        RecordType: CSSM_DB_RECORDTYPE,
        UniqueRecordIdentifier: CSSM_DB_UNIQUE_RECORD_PTR,
        AttributesToBeModified: *const CSSM_DB_RECORD_ATTRIBUTE_DATA,
        DataToBeModified: *const SecAsn1Item,
        ModifyMode: CSSM_DB_MODIFY_MODE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DataGetFirst(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        Query: *const CSSM_QUERY,
        ResultsHandle: CSSM_HANDLE_PTR,
        Attributes: CSSM_DB_RECORD_ATTRIBUTE_DATA_PTR,
        Data: CSSM_DATA_PTR,
        UniqueId: *mut CSSM_DB_UNIQUE_RECORD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DataGetNext(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        ResultsHandle: CSSM_HANDLE,
        Attributes: CSSM_DB_RECORD_ATTRIBUTE_DATA_PTR,
        Data: CSSM_DATA_PTR,
        UniqueId: *mut CSSM_DB_UNIQUE_RECORD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DataAbortQuery(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        ResultsHandle: CSSM_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_DataGetFromUniqueRecordId(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        UniqueRecord: *const CSSM_DB_UNIQUE_RECORD,
        Attributes: CSSM_DB_RECORD_ATTRIBUTE_DATA_PTR,
        Data: CSSM_DATA_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_FreeUniqueRecord(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        UniqueRecord: CSSM_DB_UNIQUE_RECORD_PTR,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn CSSM_DL_PassThrough(
        DLDBHandle: CSSM_DL_DB_HANDLE,
        PassThroughId: uint32,
        InputParams: *const ::std::os::raw::c_void,
        OutputParams: *mut *mut ::std::os::raw::c_void,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn MDS_Initialize(
        pCallerGuid: *const CSSM_GUID,
        pMemoryFunctions: *const CSSM_MEMORY_FUNCS,
        pDlFunctions: MDS_FUNCS_PTR,
        hMds: *mut MDS_HANDLE,
    ) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn MDS_Terminate(MdsHandle: MDS_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn MDS_Install(MdsHandle: MDS_HANDLE) -> CSSM_RETURN;
}
unsafe extern "C" {
    pub fn MDS_Uninstall(MdsHandle: MDS_HANDLE) -> CSSM_RETURN;
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
    pub static CSSMOID_ObjectClass: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_AliasedEntryName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_KnowledgeInformation: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CommonName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_Surname: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SerialNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CountryName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_LocalityName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_StateProvinceName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveStateProvinceName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_StreetAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveStreetAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_OrganizationName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveOrganizationName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_OrganizationalUnitName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveOrganizationalUnitName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_Title: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_Description: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SearchGuide: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_BusinessCategory: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PostalAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectivePostalAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PostalCode: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectivePostalCode: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PostOfficeBox: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectivePostOfficeBox: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PhysicalDeliveryOfficeName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectivePhysicalDeliveryOfficeName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_TelephoneNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveTelephoneNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_TelexNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveTelexNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_TelexTerminalIdentifier: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveTelexTerminalIdentifier: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_FacsimileTelephoneNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveFacsimileTelephoneNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X_121Address: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_InternationalISDNNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CollectiveInternationalISDNNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_RegisteredAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DestinationIndicator: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PreferredDeliveryMethod: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PresentationAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SupportedApplicationContext: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_Member: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_Owner: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_RoleOccupant: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SeeAlso: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_UserPassword: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_UserCertificate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CACertificate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_AuthorityRevocationList: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CertificateRevocationList: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CrossCertificatePair: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_Name: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_GivenName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_Initials: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_GenerationQualifier: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_UniqueIdentifier: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DNQualifier: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_EnhancedSearchGuide: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ProtocolInformation: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DistinguishedName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_UniqueMember: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_HouseIdentifier: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_EmailAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_UnstructuredName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ContentType: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MessageDigest: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SigningTime: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CounterSignature: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ChallengePassword: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_UnstructuredAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ExtendedCertificateAttributes: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_Id_Ct_TSTInfo: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_TimeStampToken: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_QT_CPS: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_QT_UNOTICE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_AD_OCSP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_AD_CA_ISSUERS: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_AD_TIME_STAMPING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_AD_CA_REPOSITORY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PDA_DATE_OF_BIRTH: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PDA_PLACE_OF_BIRTH: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PDA_GENDER: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PDA_COUNTRY_CITIZEN: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PDA_COUNTRY_RESIDENCE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_OID_QCS_SYNTAX_V1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_OID_QCS_SYNTAX_V2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ETSI_QCS_QC_COMPLIANCE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ETSI_QCS_QC_LIMIT_VALUE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ETSI_QCS_QC_RETENTION: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ETSI_QCS_QC_SSCD: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS7_Data: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS7_SignedData: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS7_EnvelopedData: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS7_SignedAndEnvelopedData: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS7_DigestedData: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS7_EncryptedData: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS7_DataWithAttributes: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS7_EncryptedPrivateKeyInfo: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_FriendlyName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_LocalKeyId: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_CertTypes: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_CrlTypes: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_X509Certificate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_SdsiCertificate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS9_X509Crl: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_keyBag: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_shroudedKeyBag: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_certBag: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_crlBag: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_secretBag: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKCS12_safeContentsBag: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_UserID: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DomainComponent: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_KERBv5_PKINIT_AUTH_DATA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_KERBv5_PKINIT_DH_KEY_DATA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_KERBv5_PKINIT_RKEY_DATA: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X9_62: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X9_62_FieldType: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X9_62_PubKeyType: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X9_62_EllCurve: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X9_62_C_TwoCurve: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X9_62_PrimeCurve: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X9_62_SigType: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp192r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp256r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_Certicom: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CerticomEllCurve: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp112r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp112r2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp128r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp128r2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp160k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp160r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp160r2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp192k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp224k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp224r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp256k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp384r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_secp521r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect113r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect113r2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect131r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect131r2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect163k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect163r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect163r2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect193r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect193r2: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect233k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect233r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect239k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect283k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect283r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect409k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect409r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect571k1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_sect571r1: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3SignedCertificate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3SignedCertificateCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3Certificate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1Version: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SerialNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1IssuerName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1IssuerNameStd: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1IssuerNameCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1IssuerNameLDAP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1ValidityNotBefore: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1ValidityNotAfter: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SubjectName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SubjectNameStd: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SubjectNameCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SubjectNameLDAP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CSSMKeyStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SubjectPublicKeyCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SubjectPublicKeyAlgorithm: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SubjectPublicKeyAlgorithmParameters: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SubjectPublicKey: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CertificateIssuerUniqueId: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CertificateSubjectUniqueId: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateExtensionsStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateExtensionsCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateNumberOfExtensions: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateExtensionStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateExtensionCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateExtensionId: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateExtensionCritical: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateExtensionType: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V3CertificateExtensionValue: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SignatureStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SignatureCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SignatureAlgorithm: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SignatureAlgorithmTBS: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1SignatureAlgorithmParameters: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1Signature: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SubjectSignatureBitmap: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SubjectPicture: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SubjectEmailAddress: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_UseExemptions: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SubjectDirectoryAttributes: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SubjectKeyIdentifier: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_KeyUsage: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PrivateKeyUsagePeriod: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SubjectAltName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_IssuerAltName: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_BasicConstraints: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CrlNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CrlReason: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_HoldInstructionCode: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_InvalidityDate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DeltaCrlIndicator: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_IssuingDistributionPoint: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_IssuingDistributionPoints: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CertIssuer: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_NameConstraints: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CrlDistributionPoints: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_CertificatePolicies: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PolicyMappings: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PolicyConstraints: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_AuthorityKeyIdentifier: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ExtendedKeyUsage: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_InhibitAnyPolicy: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_AuthorityInfoAccess: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_BiometricInfo: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_QC_Statements: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_SubjectInfoAccess: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ExtendedKeyUsageAny: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ServerAuth: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ClientAuth: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ExtendedUseCodeSigning: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_EmailProtection: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_TimeStamping: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_OCSPSigning: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_KERBv5_PKINIT_KP_CLIENT_AUTH: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_KERBv5_PKINIT_KP_KDC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_EKU_IPSec: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_EXTENSION: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_IDENTITY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_EMAIL_SIGN: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_EMAIL_ENCRYPT: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_CERT_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_DOTMAC_CERT_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_ADC_CERT_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MACAPPSTORE_CERT_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MACAPPSTORE_RECEIPT_CERT_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLEID_CERT_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLEID_SHARING_CERT_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MOBILE_STORE_SIGNING_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_TEST_MOBILE_STORE_SIGNING_POLICY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_CODE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_CODE_SIGNING_DEV: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_RESOURCE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_ICHAT_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_ICHAT_ENCRYPTION: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_SYSTEM_IDENTITY: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_PASSBOOK_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_PROFILE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EKU_QA_PROFILE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_CODE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_APPLE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_ADC_DEV_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_ADC_APPLE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_PASSBOOK_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_MACAPPSTORE_RECEIPT: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_INTERMEDIATE_MARKER: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_WWDR_INTERMEDIATE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_ITMS_INTERMEDIATE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_AAI_INTERMEDIATE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_APPLEID_INTERMEDIATE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_APPLEID_SHARING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_SYSINT2_INTERMEDIATE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_DEVELOPER_AUTHENTICATION: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_SERVER_AUTHENTICATION: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_ESCROW_SERVICE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_APPLE_EXTENSION_PROVISIONING_PROFILE_SIGNING: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_NetscapeCertType: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_NetscapeCertSequence: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_NetscapeSGC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_MicrosoftSGC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLSignedCrlStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLSignedCrlCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLTbsCertListStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLTbsCertListCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLVersion: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLIssuerStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLIssuerNameCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLIssuerNameLDAP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLThisUpdate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLNextUpdate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLRevokedCertificatesStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLRevokedCertificatesCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLNumberOfRevokedCertEntries: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLRevokedEntryStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLRevokedEntryCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLRevokedEntrySerialNumber: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V1CRLRevokedEntryRevocationDate: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntryAllExtensionsStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntryAllExtensionsCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntryNumberOfExtensions: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntrySingleExtensionStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntrySingleExtensionCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntryExtensionId: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntryExtensionCritical: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntryExtensionType: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLRevokedEntryExtensionValue: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLAllExtensionsStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLAllExtensionsCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLNumberOfExtensions: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLSingleExtensionStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLSingleExtensionCStruct: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLExtensionId: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLExtensionCritical: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_X509V2CRLExtensionType: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKIX_OCSP: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKIX_OCSP_BASIC: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKIX_OCSP_NONCE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKIX_OCSP_CRL: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKIX_OCSP_RESPONSE: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKIX_OCSP_NOCHECK: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKIX_OCSP_ARCHIVE_CUTOFF: SecAsn1Oid;
}
unsafe extern "C" {
    pub static CSSMOID_PKIX_OCSP_SERVICE_LOCATOR: SecAsn1Oid;
}
unsafe extern "C" {
    pub fn SecACLGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecACLCreateFromSimpleContents(
        access: SecAccessRef,
        applicationList: CFArrayRef,
        description: CFStringRef,
        promptSelector: *const CSSM_ACL_KEYCHAIN_PROMPT_SELECTOR,
        newAcl: *mut SecACLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLCreateWithSimpleContents(
        access: SecAccessRef,
        applicationList: CFArrayRef,
        description: CFStringRef,
        promptSelector: SecKeychainPromptSelector,
        newAcl: *mut SecACLRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLRemove(aclRef: SecACLRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLCopySimpleContents(
        acl: SecACLRef,
        applicationList: *mut CFArrayRef,
        description: *mut CFStringRef,
        promptSelector: *mut CSSM_ACL_KEYCHAIN_PROMPT_SELECTOR,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLCopyContents(
        acl: SecACLRef,
        applicationList: *mut CFArrayRef,
        description: *mut CFStringRef,
        promptSelector: *mut SecKeychainPromptSelector,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLSetSimpleContents(
        acl: SecACLRef,
        applicationList: CFArrayRef,
        description: CFStringRef,
        promptSelector: *const CSSM_ACL_KEYCHAIN_PROMPT_SELECTOR,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLSetContents(
        acl: SecACLRef,
        applicationList: CFArrayRef,
        description: CFStringRef,
        promptSelector: SecKeychainPromptSelector,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLGetAuthorizations(
        acl: SecACLRef,
        tags: *mut CSSM_ACL_AUTHORIZATION_TAG,
        tagCount: *mut uint32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLCopyAuthorizations(acl: SecACLRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SecACLSetAuthorizations(
        acl: SecACLRef,
        tags: *mut CSSM_ACL_AUTHORIZATION_TAG,
        tagCount: uint32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecACLUpdateAuthorizations(acl: SecACLRef, authorizations: CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub static kSecOIDADC_CERT_POLICY: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_CERT_POLICY: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EKU_CODE_SIGNING: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EKU_CODE_SIGNING_DEV: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EKU_ICHAT_ENCRYPTION: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EKU_ICHAT_SIGNING: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EKU_RESOURCE_SIGNING: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EKU_SYSTEM_IDENTITY: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_ADC_APPLE_SIGNING: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_ADC_DEV_SIGNING: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_APPLE_SIGNING: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_CODE_SIGNING: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_INTERMEDIATE_MARKER: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_WWDR_INTERMEDIATE: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_ITMS_INTERMEDIATE: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_AAI_INTERMEDIATE: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAPPLE_EXTENSION_APPLEID_INTERMEDIATE: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAuthorityInfoAccess: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDAuthorityKeyIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDBasicConstraints: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDBiometricInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCSSMKeyStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCertIssuer: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCertificatePolicies: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDClientAuth: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCollectiveStateProvinceName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCollectiveStreetAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCommonName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCountryName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCrlDistributionPoints: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCrlNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDCrlReason: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDDOTMAC_CERT_EMAIL_ENCRYPT: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDDOTMAC_CERT_EMAIL_SIGN: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDDOTMAC_CERT_EXTENSION: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDDOTMAC_CERT_IDENTITY: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDDOTMAC_CERT_POLICY: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDDeltaCrlIndicator: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDEKU_IPSec: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDEmailAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDEmailProtection: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDExtendedKeyUsage: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDExtendedKeyUsageAny: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDExtendedUseCodeSigning: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDGivenName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDHoldInstructionCode: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDInvalidityDate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDIssuerAltName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDIssuingDistributionPoint: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDIssuingDistributionPoints: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDKERBv5_PKINIT_KP_CLIENT_AUTH: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDKERBv5_PKINIT_KP_KDC: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDKeyUsage: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDLocalityName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDMS_NTPrincipalName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDMicrosoftSGC: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDNameConstraints: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDNetscapeCertSequence: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDNetscapeCertType: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDNetscapeSGC: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDOCSPSigning: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDOrganizationName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDOrganizationalUnitName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDPolicyConstraints: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDPolicyMappings: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDPrivateKeyUsagePeriod: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDQC_Statements: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDServerAuth: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDStateProvinceName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDStreetAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSubjectAltName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSubjectDirectoryAttributes: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSubjectEmailAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSubjectInfoAccess: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSubjectKeyIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSubjectPicture: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSubjectSignatureBitmap: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSurname: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDTimeStamping: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDTitle: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDUseExemptions: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1CertificateIssuerUniqueId: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1CertificateSubjectUniqueId: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1IssuerName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1IssuerNameCStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1IssuerNameLDAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1IssuerNameStd: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1Signature: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SignatureAlgorithm: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SignatureAlgorithmParameters: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SignatureAlgorithmTBS: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SignatureCStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SignatureStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SubjectName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SubjectNameCStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SubjectNameLDAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SubjectNameStd: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SubjectPublicKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SubjectPublicKeyAlgorithm: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SubjectPublicKeyAlgorithmParameters: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1SubjectPublicKeyCStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1ValidityNotAfter: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1ValidityNotBefore: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V1Version: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3Certificate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateCStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateExtensionCStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateExtensionCritical: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateExtensionId: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateExtensionStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateExtensionType: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateExtensionValue: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateExtensionsCStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateExtensionsStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3CertificateNumberOfExtensions: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3SignedCertificate: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDX509V3SignedCertificateCStruct: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOIDSRVName: CFStringRef;
}
unsafe extern "C" {
    pub fn SecIdentitySearchGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecIdentitySearchCreate(
        keychainOrArray: CFTypeRef,
        keyUsage: CSSM_KEYUSE,
        searchRef: *mut SecIdentitySearchRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecIdentitySearchCopyNext(
        searchRef: SecIdentitySearchRef,
        identity: *mut SecIdentityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecKeychainItemModifyAttributesAndData(
        itemRef: SecKeychainItemRef,
        attrList: *const SecKeychainAttributeList,
        length: UInt32,
        data: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemCreateFromContent(
        itemClass: SecItemClass,
        attrList: *mut SecKeychainAttributeList,
        length: UInt32,
        data: *const ::std::os::raw::c_void,
        keychainRef: SecKeychainRef,
        initialAccess: SecAccessRef,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemModifyContent(
        itemRef: SecKeychainItemRef,
        attrList: *const SecKeychainAttributeList,
        length: UInt32,
        data: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemCopyContent(
        itemRef: SecKeychainItemRef,
        itemClass: *mut SecItemClass,
        attrList: *mut SecKeychainAttributeList,
        length: *mut UInt32,
        outData: *mut *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemFreeContent(
        attrList: *mut SecKeychainAttributeList,
        data: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemCopyAttributesAndData(
        itemRef: SecKeychainItemRef,
        info: *mut SecKeychainAttributeInfo,
        itemClass: *mut SecItemClass,
        attrList: *mut *mut SecKeychainAttributeList,
        length: *mut UInt32,
        outData: *mut *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemFreeAttributesAndData(
        attrList: *mut SecKeychainAttributeList,
        data: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemDelete(itemRef: SecKeychainItemRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemCopyKeychain(
        itemRef: SecKeychainItemRef,
        keychainRef: *mut SecKeychainRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemCreateCopy(
        itemRef: SecKeychainItemRef,
        destKeychainRef: SecKeychainRef,
        initialAccess: SecAccessRef,
        itemCopy: *mut SecKeychainItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemCreatePersistentReference(
        itemRef: SecKeychainItemRef,
        persistentItemRef: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemCopyFromPersistentReference(
        persistentItemRef: CFDataRef,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemGetDLDBHandle(
        keyItemRef: SecKeychainItemRef,
        dldbHandle: *mut CSSM_DL_DB_HANDLE,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemGetUniqueRecordID(
        itemRef: SecKeychainItemRef,
        uniqueRecordID: *mut *const CSSM_DB_UNIQUE_RECORD,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemCopyAccess(
        itemRef: SecKeychainItemRef,
        access: *mut SecAccessRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainItemSetAccess(itemRef: SecKeychainItemRef, access: SecAccessRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSearchGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecKeychainSearchCreateFromAttributes(
        keychainOrArray: CFTypeRef,
        itemClass: SecItemClass,
        attrList: *const SecKeychainAttributeList,
        searchRef: *mut SecKeychainSearchRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecKeychainSearchCopyNext(
        searchRef: SecKeychainSearchRef,
        itemRef: *mut SecKeychainItemRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecPolicySearchGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecPolicySearchCreate(
        certType: CSSM_CERT_TYPE,
        policyOID: *const SecAsn1Oid,
        value: *const SecAsn1Item,
        searchRef: *mut SecPolicySearchRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecPolicySearchCopyNext(
        searchRef: SecPolicySearchRef,
        policyRef: *mut SecPolicyRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustedApplicationGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecTrustedApplicationCreateFromPath(
        path: *const ::std::os::raw::c_char,
        app: *mut SecTrustedApplicationRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustedApplicationCopyData(
        appRef: SecTrustedApplicationRef,
        data: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustedApplicationSetData(
        appRef: SecTrustedApplicationRef,
        data: CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSettingsCopyTrustSettings(
        certRef: SecCertificateRef,
        domain: SecTrustSettingsDomain,
        trustSettings: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSettingsSetTrustSettings(
        certRef: SecCertificateRef,
        domain: SecTrustSettingsDomain,
        trustSettingsDictOrArray: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSettingsRemoveTrustSettings(
        certRef: SecCertificateRef,
        domain: SecTrustSettingsDomain,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSettingsCopyCertificates(
        domain: SecTrustSettingsDomain,
        certArray: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSettingsCopyModificationDate(
        certRef: SecCertificateRef,
        domain: SecTrustSettingsDomain,
        modificationDate: *mut CFDateRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSettingsCreateExternalRepresentation(
        domain: SecTrustSettingsDomain,
        trustSettings: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecTrustSettingsImportExternalRepresentation(
        domain: SecTrustSettingsDomain,
        trustSettings: CFDataRef,
    ) -> OSStatus;
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
    pub fn SecCodeCopyGuestWithAttributes(
        host: SecCodeRef,
        attributes: CFDictionaryRef,
        flags: SecCSFlags,
        guest: *mut SecCodeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecCodeCreateWithXPCMessage(
        message: NSObject,
        flags: SecCSFlags,
        target: *mut SecCodeRef,
    ) -> OSStatus;
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
    pub fn SecHostCreateGuest(
        host: SecGuestRef,
        status: u32,
        path: CFURLRef,
        attributes: CFDictionaryRef,
        flags: SecCSFlags,
        newGuest: *mut SecGuestRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecHostRemoveGuest(host: SecGuestRef, guest: SecGuestRef, flags: SecCSFlags)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn SecHostSelectGuest(guestRef: SecGuestRef, flags: SecCSFlags) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecHostSelectedGuest(flags: SecCSFlags, guestRef: *mut SecGuestRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecHostSetGuestStatus(
        guestRef: SecGuestRef,
        status: u32,
        attributes: CFDictionaryRef,
        flags: SecCSFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecHostSetHostingPort(hostingPort: mach_port_t, flags: SecCSFlags) -> OSStatus;
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
    pub fn AuthorizationRightGet(
        rightName: *const ::std::os::raw::c_char,
        rightDefinition: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationRightSet(
        authRef: AuthorizationRef,
        rightName: *const ::std::os::raw::c_char,
        rightDefinition: CFTypeRef,
        descriptionKey: CFStringRef,
        bundle: CFBundleRef,
        localeTableName: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AuthorizationRightRemove(
        authRef: AuthorizationRef,
        rightName: *const ::std::os::raw::c_char,
    ) -> OSStatus;
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
    pub fn CMSDecoderSetSearchKeychain(
        cmsDecoder: CMSDecoderRef,
        keychainOrArray: CFTypeRef,
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
    pub fn CMSDecoderCopySignerTimestamp(
        cmsDecoder: CMSDecoderRef,
        signerIndex: usize,
        timestamp: *mut CFAbsoluteTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopySignerTimestampWithPolicy(
        cmsDecoder: CMSDecoderRef,
        timeStampPolicy: CFTypeRef,
        signerIndex: usize,
        timestamp: *mut CFAbsoluteTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSDecoderCopySignerTimestampCertificates(
        cmsDecoder: CMSDecoderRef,
        signerIndex: usize,
        certificateRefs: *mut CFArrayRef,
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
    pub fn CMSEncoderSetEncapsulatedContentType(
        cmsEncoder: CMSEncoderRef,
        eContentType: *const SecAsn1Oid,
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
    pub fn CMSEncode(
        signers: CFTypeRef,
        recipients: CFTypeRef,
        eContentType: *const SecAsn1Oid,
        detachedContent: Boolean,
        signedAttributes: CMSSignedAttributes,
        content: *const ::std::os::raw::c_void,
        contentLen: usize,
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
    pub fn CMSEncoderCopySignerTimestamp(
        cmsEncoder: CMSEncoderRef,
        signerIndex: usize,
        timestamp: *mut CFAbsoluteTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSEncoderCopySignerTimestampWithPolicy(
        cmsEncoder: CMSEncoderRef,
        timeStampPolicy: CFTypeRef,
        signerIndex: usize,
        timestamp: *mut CFAbsoluteTime,
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
    pub fn SSLNewContext(isServer: Boolean, contextPtr: *mut SSLContextRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLDisposeContext(context: SSLContextRef) -> OSStatus;
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
    pub fn SSLSetProtocolVersionEnabled(
        context: SSLContextRef,
        protocol: SSLProtocol,
        enable: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetProtocolVersionEnabled(
        context: SSLContextRef,
        protocol: SSLProtocol,
        enable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetProtocolVersion(context: SSLContextRef, version: SSLProtocol) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetProtocolVersion(context: SSLContextRef, protocol: *mut SSLProtocol) -> OSStatus;
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
    pub fn SSLSetEnableCertVerify(context: SSLContextRef, enableVerify: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetEnableCertVerify(context: SSLContextRef, enableVerify: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetAllowsExpiredCerts(context: SSLContextRef, allowsExpired: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetAllowsExpiredCerts(
        context: SSLContextRef,
        allowsExpired: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetAllowsExpiredRoots(context: SSLContextRef, allowsExpired: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetAllowsExpiredRoots(
        context: SSLContextRef,
        allowsExpired: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetAllowsAnyRoot(context: SSLContextRef, anyRoot: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetAllowsAnyRoot(context: SSLContextRef, anyRoot: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetTrustedRoots(
        context: SSLContextRef,
        trustedRoots: CFArrayRef,
        replaceExisting: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLCopyTrustedRoots(context: SSLContextRef, trustedRoots: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLCopyPeerCertificates(context: SSLContextRef, certs: *mut CFArrayRef) -> OSStatus;
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
    pub fn SSLSetCertificateAuthorities(
        context: SSLContextRef,
        certificateOrArray: CFTypeRef,
        replaceExisting: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLCopyCertificateAuthorities(
        context: SSLContextRef,
        certificates: *mut CFArrayRef,
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
    pub fn SSLSetDiffieHellmanParams(
        context: SSLContextRef,
        dhParams: *const ::std::os::raw::c_void,
        dhParamsLen: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetDiffieHellmanParams(
        context: SSLContextRef,
        dhParams: *mut *const ::std::os::raw::c_void,
        dhParamsLen: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLSetRsaBlinding(context: SSLContextRef, blinding: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn SSLGetRsaBlinding(context: SSLContextRef, blinding: *mut Boolean) -> OSStatus;
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
    pub static kSecTransformErrorDomain: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformPreviousErrorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformAbortOriginatorKey: CFStringRef;
}
unsafe extern "C" {
    pub fn SecTransformGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecGroupTransformGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kSecTransformInputAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformOutputAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformDebugAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformTransformName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformAbortAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub fn SecTransformCreateFromExternalRepresentation(
        dictionary: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> SecTransformRef;
}
unsafe extern "C" {
    pub fn SecTransformCopyExternalRepresentation(transformRef: SecTransformRef)
        -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SecTransformCreateGroupTransform() -> SecGroupTransformRef;
}
unsafe extern "C" {
    pub fn SecTransformConnectTransforms(
        sourceTransformRef: SecTransformRef,
        sourceAttributeName: CFStringRef,
        destinationTransformRef: SecTransformRef,
        destinationAttributeName: CFStringRef,
        group: SecGroupTransformRef,
        error: *mut CFErrorRef,
    ) -> SecGroupTransformRef;
}
unsafe extern "C" {
    pub fn SecTransformSetAttribute(
        transformRef: SecTransformRef,
        key: CFStringRef,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SecTransformGetAttribute(transformRef: SecTransformRef, key: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn SecTransformFindByName(
        transform: SecGroupTransformRef,
        name: CFStringRef,
    ) -> SecTransformRef;
}
unsafe extern "C" {
    pub fn SecTransformExecute(
        transformRef: SecTransformRef,
        errorRef: *mut CFErrorRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn SecTransformExecuteAsync(
        transformRef: SecTransformRef,
        deliveryQueue: NSObject,
        deliveryBlock: SecMessageBlock,
    );
}
unsafe extern "C" {
    pub fn SecTransformSetAttributeAction(
        ref_: SecTransformImplementationRef,
        action: CFStringRef,
        attribute: SecTransformStringOrAttributeRef,
        newAction: SecTransformAttributeActionBlock,
    ) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn SecTransformSetDataAction(
        ref_: SecTransformImplementationRef,
        action: CFStringRef,
        newAction: SecTransformDataBlock,
    ) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn SecTransformSetTransformAction(
        ref_: SecTransformImplementationRef,
        action: CFStringRef,
        newAction: SecTransformActionBlock,
    ) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn SecTranformCustomGetAttribute(
        ref_: SecTransformImplementationRef,
        attribute: SecTransformStringOrAttributeRef,
        type_: SecTransformMetaAttributeType,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn SecTransformCustomSetAttribute(
        ref_: SecTransformImplementationRef,
        attribute: SecTransformStringOrAttributeRef,
        type_: SecTransformMetaAttributeType,
        value: CFTypeRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn SecTransformPushbackAttribute(
        ref_: SecTransformImplementationRef,
        attribute: SecTransformStringOrAttributeRef,
        value: CFTypeRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub static kSecTransformActionCanExecute: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformActionStartingExecution: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformActionFinalize: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformActionExternalizeExtraData: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformActionProcessData: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformActionInternalizeExtraData: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformActionAttributeNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kSecTransformActionAttributeValidation: CFStringRef;
}
unsafe extern "C" {
    pub fn SecTransformRegister(
        uniqueName: CFStringRef,
        createTransformFunction: SecTransformCreateFP,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SecTransformCreate(name: CFStringRef, error: *mut CFErrorRef) -> SecTransformRef;
}
unsafe extern "C" {
    pub fn SecTransformNoData() -> CFTypeRef;
}
unsafe extern "C" {
    pub static kSecBase64Encoding: CFStringRef;
}
unsafe extern "C" {
    pub static kSecBase32Encoding: CFStringRef;
}
unsafe extern "C" {
    pub static kSecZLibEncoding: CFStringRef;
}
unsafe extern "C" {
    pub static kSecEncodeTypeAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kSecLineLength64: CFStringRef;
}
unsafe extern "C" {
    pub static kSecLineLength76: CFStringRef;
}
unsafe extern "C" {
    pub static kSecEncodeLineLengthAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kSecCompressionRatio: CFStringRef;
}
unsafe extern "C" {
    pub fn SecEncodeTransformCreate(
        encodeType: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> SecTransformRef;
}
unsafe extern "C" {
    pub static kSecDecodeTypeAttribute: CFStringRef;
}
unsafe extern "C" {
    pub fn SecDecodeTransformCreate(
        DecodeType: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> SecTransformRef;
}
unsafe extern "C" {
    pub static kSecDigestMD2: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestMD4: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestMD5: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestSHA1: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestSHA2: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestHMACMD5: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestHMACSHA1: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestHMACSHA2: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestTypeAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestLengthAttribute: CFStringRef;
}
unsafe extern "C" {
    pub static kSecDigestHMACKeyAttribute: CFStringRef;
}
unsafe extern "C" {
    pub fn SecDigestTransformCreate(
        digestType: CFTypeRef,
        digestLength: CFIndex,
        error: *mut CFErrorRef,
    ) -> SecTransformRef;
}
unsafe extern "C" {
    pub fn SecDigestTransformGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kSecPaddingNoneKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPaddingPKCS1Key: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPaddingPKCS5Key: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPaddingPKCS7Key: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPaddingOAEPKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecModeNoneKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecModeECBKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecModeCBCKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecModeCFBKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecModeOFBKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecEncryptKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecPaddingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecIVKey: CFStringRef;
}
unsafe extern "C" {
    pub static kSecEncryptionMode: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOAEPMessageLengthAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOAEPEncodingParametersAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecOAEPMGF1DigestAlgorithmAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub fn SecEncryptTransformCreate(keyRef: SecKeyRef, error: *mut CFErrorRef) -> SecTransformRef;
}
unsafe extern "C" {
    pub fn SecDecryptTransformCreate(keyRef: SecKeyRef, error: *mut CFErrorRef) -> SecTransformRef;
}
unsafe extern "C" {
    pub fn SecDecryptTransformGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SecEncryptTransformGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kSecKeyAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kSecSignatureAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kSecInputIsAttributeName: CFStringRef;
}
unsafe extern "C" {
    pub static kSecInputIsPlainText: CFStringRef;
}
unsafe extern "C" {
    pub static mut kSecInputIsDigest: CFStringRef;
}
unsafe extern "C" {
    pub static mut kSecInputIsRaw: CFStringRef;
}
unsafe extern "C" {
    pub fn SecSignTransformCreate(key: SecKeyRef, error: *mut CFErrorRef) -> SecTransformRef;
}
unsafe extern "C" {
    pub fn SecVerifyTransformCreate(
        key: SecKeyRef,
        signature: CFDataRef,
        error: *mut CFErrorRef,
    ) -> SecTransformRef;
}
unsafe extern "C" {
    pub fn SecTransformCreateReadTransformWithReadStream(
        inputStream: CFReadStreamRef,
    ) -> SecTransformRef;
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
unsafe extern "C" {
    pub fn SecureDownloadCreateWithTicket(
        ticket: CFDataRef,
        setup: SecureDownloadTrustSetupCallback,
        setupContext: *mut ::std::os::raw::c_void,
        evaluate: SecureDownloadTrustEvaluateCallback,
        evaluateContext: *mut ::std::os::raw::c_void,
        downloadRef: *mut SecureDownloadRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecureDownloadCopyURLs(
        downloadRef: SecureDownloadRef,
        urls: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecureDownloadCopyName(
        downloadRef: SecureDownloadRef,
        name: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecureDownloadCopyCreationDate(
        downloadRef: SecureDownloadRef,
        date: *mut CFDateRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecureDownloadGetDownloadSize(
        downloadRef: SecureDownloadRef,
        downloadSize: *mut SInt64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecureDownloadUpdateWithData(
        downloadRef: SecureDownloadRef,
        data: CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecureDownloadFinished(downloadRef: SecureDownloadRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecureDownloadRelease(downloadRef: SecureDownloadRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn SecureDownloadCopyTicketLocation(
        url: CFURLRef,
        ticketLocation: *mut CFURLRef,
    ) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for __sbuf {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __sbuf {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__sbuf", &[]);
}
unsafe impl objc2::encode::RefEncode for __sFILEX {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __sFILEX {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__sFILEX", &[]);
}
unsafe impl objc2::encode::RefEncode for __sFILE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __sFILE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__sFILE", &[]);
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
unsafe impl objc2::encode::RefEncode for SecAsn1AlgId {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecAsn1AlgId {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecAsn1AlgId", &[]);
}
unsafe impl objc2::encode::RefEncode for SecAsn1Template_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecAsn1Template_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecAsn1Template_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_guid {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_guid {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_guid", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_version {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_version {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_version", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_subservice_uid {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_subservice_uid {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_subservice_uid", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_net_address {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_net_address {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_net_address", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_crypto_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_crypto_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_crypto_data", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_list {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_list {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_list", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_list_element {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_list_element {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_list_element", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_list_element__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_list_element__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_list_element__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_TUPLE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_TUPLE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_TUPLE", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tuplegroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tuplegroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tuplegroup", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_sample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_sample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_sample", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_samplegroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_samplegroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_samplegroup", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_memory_funcs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_memory_funcs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_memory_funcs", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_encoded_cert {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_encoded_cert {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_encoded_cert", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_parsed_cert {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_parsed_cert {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_parsed_cert", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_cert_pair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_cert_pair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_cert_pair", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_certgroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_certgroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_certgroup", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_certgroup__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_certgroup__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_certgroup__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_base_certs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_base_certs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_base_certs", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_access_credentials {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_access_credentials {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_access_credentials", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_authorizationgroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_authorizationgroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_authorizationgroup", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_acl_validity_period {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_acl_validity_period {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_acl_validity_period", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_acl_entry_prototype {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_acl_entry_prototype {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_acl_entry_prototype", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_acl_owner_prototype {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_acl_owner_prototype {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_acl_owner_prototype", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_acl_entry_input {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_acl_entry_input {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_acl_entry_input", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_resource_control_context {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_resource_control_context {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_resource_control_context", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_acl_entry_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_acl_entry_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_acl_entry_info", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_acl_edit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_acl_edit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_acl_edit", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_func_name_addr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_func_name_addr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_func_name_addr", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_date {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_date {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_date", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_range {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_range {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_range", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_query_size_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_query_size_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_query_size_data", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_key_size {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_key_size {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_key_size", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_keyheader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_keyheader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_keyheader", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_key {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_key {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_key", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_dl_db_handle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_dl_db_handle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_dl_db_handle", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_context_attribute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_context_attribute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_context_attribute", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_context_attribute_cssm_context_attribute_value {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_context_attribute_cssm_context_attribute_value {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_context_attribute_cssm_context_attribute_value", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_context {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_context {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_context", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_csp_operational_statistics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_csp_operational_statistics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_csp_operational_statistics", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tp_authority_id {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tp_authority_id {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tp_authority_id", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_field {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_field {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_field", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tp_policyinfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tp_policyinfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tp_policyinfo", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_dl_db_list {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_dl_db_list {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_dl_db_list", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tp_callerauth_context {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tp_callerauth_context {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tp_callerauth_context", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_encoded_crl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_encoded_crl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_encoded_crl", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_parsed_crl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_parsed_crl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_parsed_crl", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_crl_pair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_crl_pair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_crl_pair", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_crlgroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_crlgroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_crlgroup", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_crlgroup__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_crlgroup__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_crlgroup__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_evidence {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_evidence {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_evidence", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tp_verify_context {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tp_verify_context {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tp_verify_context", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tp_verify_context_result {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tp_verify_context_result {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tp_verify_context_result", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tp_request_set {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tp_request_set {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tp_request_set", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tp_result_set {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tp_result_set {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tp_result_set", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_tp_confirm_response {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_tp_confirm_response {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_tp_confirm_response", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_cert_bundle_header {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_cert_bundle_header {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_cert_bundle_header", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_cert_bundle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_cert_bundle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_cert_bundle", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_attribute_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_attribute_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_attribute_info", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_attribute_info_cssm_db_attribute_label {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_attribute_info_cssm_db_attribute_label {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_attribute_info_cssm_db_attribute_label", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_attribute_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_attribute_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_attribute_data", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_record_attribute_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_record_attribute_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_record_attribute_info", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_record_attribute_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_record_attribute_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_record_attribute_data", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_parsing_module_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_parsing_module_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_parsing_module_info", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_index_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_index_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_index_info", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_unique_record {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_unique_record {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_unique_record", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_record_index_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_record_index_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_record_index_info", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_dbinfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_dbinfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_dbinfo", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_selection_predicate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_selection_predicate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_selection_predicate", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_query_limits {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_query_limits {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_query_limits", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_query {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_query {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_query", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_dl_pkcs11_attributes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_dl_pkcs11_attributes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_dl_pkcs11_attributes", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_name_list {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_name_list {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_name_list", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_schema_attribute_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_schema_attribute_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_schema_attribute_info", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_db_schema_index_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_db_schema_index_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_db_schema_index_info", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_x509_type_value_pair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_x509_type_value_pair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_x509_type_value_pair", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_x509_rdn {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_x509_rdn {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_x509_rdn", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_x509_name {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_x509_name {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_x509_name", &[]);
}
unsafe impl objc2::encode::RefEncode for __SecRandom {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SecRandom {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SecRandom", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_OtherName {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_OtherName {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_OtherName", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_GeneralName {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_GeneralName {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_GeneralName", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_GeneralNames {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_GeneralNames {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_GeneralNames", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_AuthorityKeyID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_AuthorityKeyID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_AuthorityKeyID", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_ExtendedKeyUsage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_ExtendedKeyUsage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_ExtendedKeyUsage", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_BasicConstraints {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_BasicConstraints {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_BasicConstraints", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_PolicyQualifierInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_PolicyQualifierInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_PolicyQualifierInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_PolicyInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_PolicyInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_PolicyInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_CertPolicies {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_CertPolicies {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_CertPolicies", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_DistributionPointName {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_DistributionPointName {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_DistributionPointName", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_DistributionPointName__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_DistributionPointName__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_DistributionPointName__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_CRLDistributionPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_CRLDistributionPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_CRLDistributionPoint", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_CRLDistPointsSyntax {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_CRLDistPointsSyntax {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_CRLDistPointsSyntax", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_AccessDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_AccessDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_AccessDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_AuthorityInfoAccess {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_AuthorityInfoAccess {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_AuthorityInfoAccess", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_SemanticsInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_SemanticsInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_SemanticsInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_QC_Statement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_QC_Statement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_QC_Statement", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_QC_Statements {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_QC_Statements {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_QC_Statements", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_IssuingDistributionPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_IssuingDistributionPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_IssuingDistributionPoint", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_GeneralSubtree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_GeneralSubtree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_GeneralSubtree", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_GeneralSubtrees {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_GeneralSubtrees {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_GeneralSubtrees", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_NameConstraints {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_NameConstraints {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_NameConstraints", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_PolicyMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_PolicyMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_PolicyMapping", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_PolicyMappings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_PolicyMappings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_PolicyMappings", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_PolicyConstraints {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_PolicyConstraints {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_PolicyConstraints", &[]);
}
unsafe impl objc2::encode::RefEncode for CE_Data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CE_Data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CE_Data", &[]);
}
unsafe impl objc2::encode::RefEncode for __CE_DataAndType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CE_DataAndType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CE_DataAndType", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_acl_process_subject_selector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_acl_process_subject_selector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_acl_process_subject_selector", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_acl_keychain_prompt_selector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_acl_keychain_prompt_selector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_acl_keychain_prompt_selector", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_appledl_open_parameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_appledl_open_parameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_appledl_open_parameters", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_applecspdl_db_settings_parameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_applecspdl_db_settings_parameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_applecspdl_db_settings_parameters", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_applecspdl_db_is_locked_parameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_applecspdl_db_is_locked_parameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_applecspdl_db_is_locked_parameters", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_applecspdl_db_change_password_parameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_applecspdl_db_change_password_parameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_applecspdl_db_change_password_parameters", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_APPLE_TP_NAME_OID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_APPLE_TP_NAME_OID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_APPLE_TP_NAME_OID", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_APPLE_TP_CERT_REQUEST {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_APPLE_TP_CERT_REQUEST {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_APPLE_TP_CERT_REQUEST", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_APPLE_TP_SSL_OPTIONS {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_APPLE_TP_SSL_OPTIONS {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_APPLE_TP_SSL_OPTIONS", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_APPLE_TP_CRL_OPTIONS {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_APPLE_TP_CRL_OPTIONS {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_APPLE_TP_CRL_OPTIONS", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_APPLE_TP_SMIME_OPTIONS {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_APPLE_TP_SMIME_OPTIONS {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_APPLE_TP_SMIME_OPTIONS", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_APPLE_TP_ACTION_DATA {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_APPLE_TP_ACTION_DATA {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_APPLE_TP_ACTION_DATA", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_TP_APPLE_EVIDENCE_INFO {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_TP_APPLE_EVIDENCE_INFO {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_TP_APPLE_EVIDENCE_INFO", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_TP_APPLE_EVIDENCE_HEADER {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_TP_APPLE_EVIDENCE_HEADER {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_TP_APPLE_EVIDENCE_HEADER", &[]);
}
unsafe impl objc2::encode::RefEncode for CSSM_APPLE_CL_CSR_REQUEST {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSSM_APPLE_CL_CSR_REQUEST {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSSM_APPLE_CL_CSR_REQUEST", &[]);
}
unsafe impl objc2::encode::RefEncode for SecKeychainSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecKeychainSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecKeychainSettings", &[]);
}
unsafe impl objc2::encode::RefEncode for SecKeychainCallbackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecKeychainCallbackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecKeychainCallbackInfo", &[]);
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
unsafe impl objc2::encode::RefEncode for AuthorizationOpaqueRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationOpaqueRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationOpaqueRef", &[]);
}
unsafe impl objc2::encode::RefEncode for AuthorizationItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationItem", &[]);
}
unsafe impl objc2::encode::RefEncode for AuthorizationItemSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationItemSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationItemSet", &[]);
}
unsafe impl objc2::encode::RefEncode for AuthorizationExternalForm {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationExternalForm {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationExternalForm", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_kr_name {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_kr_name {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_kr_name", &[]);
}
unsafe impl objc2::encode::RefEncode for cssm_kr_profile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cssm_kr_profile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cssm_kr_profile", &[]);
}
unsafe impl objc2::encode::RefEncode for mds_funcs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for mds_funcs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("mds_funcs", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueSecIdentitySearchRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueSecIdentitySearchRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueSecIdentitySearchRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaquePolicySearchRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaquePolicySearchRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaquePolicySearchRef", &[]);
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
unsafe impl objc2::encode::RefEncode for OpaqueSecTransformImplementation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueSecTransformImplementation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueSecTransformImplementation", &[]);
}
unsafe impl objc2::encode::RefEncode for AuthorizationValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationValue", &[]);
}
unsafe impl objc2::encode::RefEncode for AuthorizationValueVector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationValueVector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationValueVector", &[]);
}
unsafe impl objc2::encode::RefEncode for __OpaqueAuthorizationEngine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __OpaqueAuthorizationEngine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__OpaqueAuthorizationEngine", &[]);
}
unsafe impl objc2::encode::RefEncode for AuthorizationCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for AuthorizationPluginInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationPluginInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationPluginInterface", &[]);
}
unsafe impl objc2::encode::RefEncode for SecAsn1Coder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SecAsn1Coder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SecAsn1Coder", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueSecureDownload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueSecureDownload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueSecureDownload", &[]);
}
