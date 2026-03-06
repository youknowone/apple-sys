#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Network::*;
#[allow(unused_imports)]
use crate::GSS::*;
#[allow(unused_imports)]
use libc::{id_t, key_t, time_t};

pub type cc_uint32 = u32;
pub type cc_int32 = i32;
pub type cc_int64 = i64;
pub type cc_uint64 = u64;
pub type cc_time_t = cc_uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_context_d {
    pub functions: *const cc_context_f,
    pub vector_functions: *const cc_context_f,
}
pub type cc_context_t = *mut cc_context_d;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_ccache_d {
    pub functions: *const cc_ccache_f,
    pub vector_functions: *const cc_ccache_f,
}
pub type cc_ccache_t = *mut cc_ccache_d;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_ccache_iterator_d {
    pub functions: *const cc_ccache_iterator_f,
    pub vector_functions: *const cc_ccache_iterator_f,
}
pub type cc_ccache_iterator_t = *mut cc_ccache_iterator_d;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_credentials_v4_t {
    pub version: cc_uint32,
    pub principal: [::std::os::raw::c_char; 40usize],
    pub principal_instance: [::std::os::raw::c_char; 40usize],
    pub service: [::std::os::raw::c_char; 40usize],
    pub service_instance: [::std::os::raw::c_char; 40usize],
    pub realm: [::std::os::raw::c_char; 40usize],
    pub session_key: [::std::os::raw::c_uchar; 8usize],
    pub kvno: cc_int32,
    pub string_to_key_type: cc_int32,
    pub issue_date: cc_time_t,
    pub lifetime: cc_int32,
    pub address: cc_uint32,
    pub ticket_size: cc_int32,
    pub ticket: [::std::os::raw::c_uchar; 1254usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_data {
    pub type_: cc_uint32,
    pub length: cc_uint32,
    pub data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_credentials_v5_t {
    pub client: *mut ::std::os::raw::c_char,
    pub server: *mut ::std::os::raw::c_char,
    pub keyblock: cc_data,
    pub authtime: cc_time_t,
    pub starttime: cc_time_t,
    pub endtime: cc_time_t,
    pub renew_till: cc_time_t,
    pub is_skey: cc_uint32,
    pub ticket_flags: cc_uint32,
    pub addresses: *mut *mut cc_data,
    pub ticket: cc_data,
    pub second_ticket: cc_data,
    pub authdata: *mut *mut cc_data,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_credentials_union {
    pub __bindgen_anon_1: cc_credentials_union__bindgen_ty_1,
    pub version: cc_uint32,
    pub credentials: cc_credentials_union__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cc_credentials_union__bindgen_ty_1 {
    pub credentials_v4: *mut cc_credentials_v4_t,
    pub credentials_v5: *mut cc_credentials_v5_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_credentials_d {
    pub data: *const cc_credentials_union,
    pub functions: *const cc_credentials_f,
    pub otherFunctions: *const cc_credentials_f,
}
pub type cc_credentials_t = *mut cc_credentials_d;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_credentials_iterator_d {
    pub functions: *const cc_credentials_iterator_f,
    pub vector_functions: *const cc_credentials_iterator_f,
}
pub type cc_credentials_iterator_t = *mut cc_credentials_iterator_d;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_string_d {
    pub data: *const ::std::os::raw::c_char,
    pub functions: *const cc_string_f,
    pub vector_functions: *const cc_string_f,
}
pub type cc_string_t = *mut cc_string_d;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_context_f {
    pub release: ::std::option::Option<unsafe extern "C" fn(io_context: cc_context_t) -> cc_int32>,
    pub get_change_time: ::std::option::Option<
        unsafe extern "C" fn(in_context: cc_context_t, out_time: *mut cc_time_t) -> cc_int32,
    >,
    pub get_default_ccache_name: ::std::option::Option<
        unsafe extern "C" fn(in_context: cc_context_t, out_name: *mut cc_string_t) -> cc_int32,
    >,
    pub open_ccache: ::std::option::Option<
        unsafe extern "C" fn(
            in_context: cc_context_t,
            in_name: *const ::std::os::raw::c_char,
            out_ccache: *mut cc_ccache_t,
        ) -> cc_int32,
    >,
    pub open_default_ccache: ::std::option::Option<
        unsafe extern "C" fn(in_context: cc_context_t, out_ccache: *mut cc_ccache_t) -> cc_int32,
    >,
    pub create_ccache: ::std::option::Option<
        unsafe extern "C" fn(
            in_context: cc_context_t,
            in_name: *const ::std::os::raw::c_char,
            in_cred_vers: cc_uint32,
            in_principal: *const ::std::os::raw::c_char,
            out_ccache: *mut cc_ccache_t,
        ) -> cc_int32,
    >,
    pub create_default_ccache: ::std::option::Option<
        unsafe extern "C" fn(
            in_context: cc_context_t,
            in_cred_vers: cc_uint32,
            in_principal: *const ::std::os::raw::c_char,
            out_ccache: *mut cc_ccache_t,
        ) -> cc_int32,
    >,
    pub create_new_ccache: ::std::option::Option<
        unsafe extern "C" fn(
            in_context: cc_context_t,
            in_cred_vers: cc_uint32,
            in_principal: *const ::std::os::raw::c_char,
            out_ccache: *mut cc_ccache_t,
        ) -> cc_int32,
    >,
    pub new_ccache_iterator: ::std::option::Option<
        unsafe extern "C" fn(
            in_context: cc_context_t,
            out_iterator: *mut cc_ccache_iterator_t,
        ) -> cc_int32,
    >,
    pub lock: ::std::option::Option<
        unsafe extern "C" fn(
            in_context: cc_context_t,
            in_lock_type: cc_uint32,
            in_block: cc_uint32,
        ) -> cc_int32,
    >,
    pub unlock:
        ::std::option::Option<unsafe extern "C" fn(in_cc_context: cc_context_t) -> cc_int32>,
    pub compare: ::std::option::Option<
        unsafe extern "C" fn(
            in_cc_context: cc_context_t,
            in_compare_to_context: cc_context_t,
            out_equal: *mut cc_uint32,
        ) -> cc_int32,
    >,
    pub wait_for_change:
        ::std::option::Option<unsafe extern "C" fn(in_cc_context: cc_context_t) -> cc_int32>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_ccache_f {
    pub release: ::std::option::Option<unsafe extern "C" fn(io_ccache: cc_ccache_t) -> cc_int32>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(io_ccache: cc_ccache_t) -> cc_int32>,
    pub set_default:
        ::std::option::Option<unsafe extern "C" fn(io_ccache: cc_ccache_t) -> cc_int32>,
    pub get_credentials_version: ::std::option::Option<
        unsafe extern "C" fn(
            in_ccache: cc_ccache_t,
            out_credentials_version: *mut cc_uint32,
        ) -> cc_int32,
    >,
    pub get_name: ::std::option::Option<
        unsafe extern "C" fn(in_ccache: cc_ccache_t, out_name: *mut cc_string_t) -> cc_int32,
    >,
    pub get_principal: ::std::option::Option<
        unsafe extern "C" fn(
            in_ccache: cc_ccache_t,
            in_credentials_version: cc_uint32,
            out_principal: *mut cc_string_t,
        ) -> cc_int32,
    >,
    pub set_principal: ::std::option::Option<
        unsafe extern "C" fn(
            io_ccache: cc_ccache_t,
            in_credentials_version: cc_uint32,
            in_principal: *const ::std::os::raw::c_char,
        ) -> cc_int32,
    >,
    pub store_credentials: ::std::option::Option<
        unsafe extern "C" fn(
            io_ccache: cc_ccache_t,
            in_credentials_union: *const cc_credentials_union,
        ) -> cc_int32,
    >,
    pub remove_credentials: ::std::option::Option<
        unsafe extern "C" fn(io_ccache: cc_ccache_t, in_credentials: cc_credentials_t) -> cc_int32,
    >,
    pub new_credentials_iterator: ::std::option::Option<
        unsafe extern "C" fn(
            in_ccache: cc_ccache_t,
            out_credentials_iterator: *mut cc_credentials_iterator_t,
        ) -> cc_int32,
    >,
    pub move_: ::std::option::Option<
        unsafe extern "C" fn(
            io_source_ccache: cc_ccache_t,
            io_destination_ccache: cc_ccache_t,
        ) -> cc_int32,
    >,
    pub lock: ::std::option::Option<
        unsafe extern "C" fn(
            io_ccache: cc_ccache_t,
            in_lock_type: cc_uint32,
            in_block: cc_uint32,
        ) -> cc_int32,
    >,
    pub unlock: ::std::option::Option<unsafe extern "C" fn(io_ccache: cc_ccache_t) -> cc_int32>,
    pub get_last_default_time: ::std::option::Option<
        unsafe extern "C" fn(
            in_ccache: cc_ccache_t,
            out_last_default_time: *mut cc_time_t,
        ) -> cc_int32,
    >,
    pub get_change_time: ::std::option::Option<
        unsafe extern "C" fn(in_ccache: cc_ccache_t, out_change_time: *mut cc_time_t) -> cc_int32,
    >,
    pub compare: ::std::option::Option<
        unsafe extern "C" fn(
            in_ccache: cc_ccache_t,
            in_compare_to_ccache: cc_ccache_t,
            out_equal: *mut cc_uint32,
        ) -> cc_int32,
    >,
    pub get_kdc_time_offset: ::std::option::Option<
        unsafe extern "C" fn(
            in_ccache: cc_ccache_t,
            in_credentials_version: cc_uint32,
            out_time_offset: *mut cc_time_t,
        ) -> cc_int32,
    >,
    pub set_kdc_time_offset: ::std::option::Option<
        unsafe extern "C" fn(
            io_ccache: cc_ccache_t,
            in_credentials_version: cc_uint32,
            in_time_offset: cc_time_t,
        ) -> cc_int32,
    >,
    pub clear_kdc_time_offset: ::std::option::Option<
        unsafe extern "C" fn(io_ccache: cc_ccache_t, in_credentials_version: cc_uint32) -> cc_int32,
    >,
    pub wait_for_change:
        ::std::option::Option<unsafe extern "C" fn(in_ccache: cc_ccache_t) -> cc_int32>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_string_f {
    pub release: ::std::option::Option<unsafe extern "C" fn(io_string: cc_string_t) -> cc_int32>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_credentials_f {
    pub release:
        ::std::option::Option<unsafe extern "C" fn(io_credentials: cc_credentials_t) -> cc_int32>,
    pub compare: ::std::option::Option<
        unsafe extern "C" fn(
            in_credentials: cc_credentials_t,
            in_compare_to_credentials: cc_credentials_t,
            out_equal: *mut cc_uint32,
        ) -> cc_int32,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_ccache_iterator_f {
    pub release: ::std::option::Option<
        unsafe extern "C" fn(io_ccache_iterator: cc_ccache_iterator_t) -> cc_int32,
    >,
    pub next: ::std::option::Option<
        unsafe extern "C" fn(
            in_ccache_iterator: cc_ccache_iterator_t,
            out_ccache: *mut cc_ccache_t,
        ) -> cc_int32,
    >,
    pub clone: ::std::option::Option<
        unsafe extern "C" fn(
            in_ccache_iterator: cc_ccache_iterator_t,
            out_ccache_iterator: *mut cc_ccache_iterator_t,
        ) -> cc_int32,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_credentials_iterator_f {
    pub release: ::std::option::Option<
        unsafe extern "C" fn(io_credentials_iterator: cc_credentials_iterator_t) -> cc_int32,
    >,
    pub next: ::std::option::Option<
        unsafe extern "C" fn(
            in_credentials_iterator: cc_credentials_iterator_t,
            out_credentials: *mut cc_credentials_t,
        ) -> cc_int32,
    >,
    pub clone: ::std::option::Option<
        unsafe extern "C" fn(
            in_credentials_iterator: cc_credentials_iterator_t,
            out_credentials_iterator: *mut cc_credentials_iterator_t,
        ) -> cc_int32,
    >,
}
pub type errcode_t = ::std::os::raw::c_long;
pub type com_err_handler_t = ::std::option::Option<
    unsafe extern "C" fn(
        whoami: *const ::std::os::raw::c_char,
        code: errcode_t,
        format: *const ::std::os::raw::c_char,
        args: va_list,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct error_table {
    pub messages: *const *const ::std::os::raw::c_char,
    pub base: i32,
    pub count: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _profile_t {
    _unused: [u8; 0],
}
pub type profile_t = *mut _profile_t;
pub type profile_filespec_t = *mut ::std::os::raw::c_char;
pub type profile_filespec_list_t = *mut ::std::os::raw::c_char;
pub type const_profile_filespec_t = *const ::std::os::raw::c_char;
pub type const_profile_filespec_list_t = *const ::std::os::raw::c_char;
pub type krb5_octet = ::std::os::raw::c_uchar;
pub type krb5_int16 = ::std::os::raw::c_short;
pub type krb5_ui_2 = ::std::os::raw::c_ushort;
pub type krb5_int32 = ::std::os::raw::c_int;
pub type krb5_ui_4 = ::std::os::raw::c_uint;
pub type krb5_boolean = ::std::os::raw::c_uint;
pub type krb5_msgtype = ::std::os::raw::c_uint;
pub type krb5_kvno = ::std::os::raw::c_uint;
pub type krb5_addrtype = krb5_int32;
pub type krb5_enctype = krb5_int32;
pub type krb5_cksumtype = krb5_int32;
pub type krb5_authdatatype = krb5_int32;
pub type krb5_keyusage = krb5_int32;
pub type krb5_preauthtype = krb5_int32;
pub type krb5_flags = krb5_int32;
pub type krb5_timestamp = krb5_int32;
pub type krb5_error_code = krb5_int32;
pub type krb5_deltat = krb5_int32;
pub type krb5_magic = krb5_error_code;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_data {
    pub magic: krb5_magic,
    pub length: ::std::os::raw::c_uint,
    pub data: *mut ::std::os::raw::c_char,
}
pub type krb5_data = _krb5_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_octet_data {
    pub magic: krb5_magic,
    pub length: ::std::os::raw::c_uint,
    pub data: *mut krb5_octet,
}
pub type krb5_octet_data = _krb5_octet_data;
pub type krb5_pointer = *mut ::std::os::raw::c_void;
pub type krb5_const_pointer = *const ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct krb5_principal_data {
    pub magic: krb5_magic,
    pub realm: krb5_data,
    pub data: *mut krb5_data,
    pub length: krb5_int32,
    pub type_: krb5_int32,
}
pub type krb5_principal = *mut krb5_principal_data;
pub type krb5_const_principal = *const krb5_principal_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_address {
    pub magic: krb5_magic,
    pub addrtype: krb5_addrtype,
    pub length: ::std::os::raw::c_uint,
    pub contents: *mut krb5_octet,
}
pub type krb5_address = _krb5_address;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_context {
    _unused: [u8; 0],
}
pub type krb5_context = *mut _krb5_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_auth_context {
    _unused: [u8; 0],
}
pub type krb5_auth_context = *mut _krb5_auth_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_keyblock {
    pub magic: krb5_magic,
    pub enctype: krb5_enctype,
    pub length: ::std::os::raw::c_uint,
    pub contents: *mut krb5_octet,
}
pub type krb5_keyblock = _krb5_keyblock;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_encrypt_block {
    pub magic: krb5_magic,
    pub crypto_entry: krb5_enctype,
    pub key: *mut krb5_keyblock,
}
pub type krb5_encrypt_block = _krb5_encrypt_block;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_checksum {
    pub magic: krb5_magic,
    pub checksum_type: krb5_cksumtype,
    pub length: ::std::os::raw::c_uint,
    pub contents: *mut krb5_octet,
}
pub type krb5_checksum = _krb5_checksum;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_enc_data {
    pub magic: krb5_magic,
    pub enctype: krb5_enctype,
    pub kvno: krb5_kvno,
    pub ciphertext: krb5_data,
}
pub type krb5_enc_data = _krb5_enc_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_ticket_times {
    pub authtime: krb5_timestamp,
    pub starttime: krb5_timestamp,
    pub endtime: krb5_timestamp,
    pub renew_till: krb5_timestamp,
}
pub type krb5_ticket_times = _krb5_ticket_times;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_authdata {
    pub magic: krb5_magic,
    pub ad_type: krb5_authdatatype,
    pub length: ::std::os::raw::c_uint,
    pub contents: *mut krb5_octet,
}
pub type krb5_authdata = _krb5_authdata;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_transited {
    pub magic: krb5_magic,
    pub tr_type: krb5_octet,
    pub tr_contents: krb5_data,
}
pub type krb5_transited = _krb5_transited;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_enc_tkt_part {
    pub magic: krb5_magic,
    pub flags: krb5_flags,
    pub session: *mut krb5_keyblock,
    pub client: krb5_principal,
    pub transited: krb5_transited,
    pub times: krb5_ticket_times,
    pub caddrs: *mut *mut krb5_address,
    pub authorization_data: *mut *mut krb5_authdata,
}
pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_ticket {
    pub magic: krb5_magic,
    pub server: krb5_principal,
    pub enc_part: krb5_enc_data,
    pub enc_part2: *mut krb5_enc_tkt_part,
}
pub type krb5_ticket = _krb5_ticket;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_authenticator {
    pub magic: krb5_magic,
    pub client: krb5_principal,
    pub checksum: *mut krb5_checksum,
    pub cusec: krb5_int32,
    pub ctime: krb5_timestamp,
    pub subkey: *mut krb5_keyblock,
    pub seq_number: krb5_ui_4,
    pub authorization_data: *mut *mut krb5_authdata,
}
pub type krb5_authenticator = _krb5_authenticator;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_tkt_authent {
    pub magic: krb5_magic,
    pub ticket: *mut krb5_ticket,
    pub authenticator: *mut krb5_authenticator,
    pub ap_options: krb5_flags,
}
pub type krb5_tkt_authent = _krb5_tkt_authent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_creds {
    pub magic: krb5_magic,
    pub client: krb5_principal,
    pub server: krb5_principal,
    pub keyblock: krb5_keyblock,
    pub times: krb5_ticket_times,
    pub is_skey: krb5_boolean,
    pub ticket_flags: krb5_flags,
    pub addresses: *mut *mut krb5_address,
    pub ticket: krb5_data,
    pub second_ticket: krb5_data,
    pub authdata: *mut *mut krb5_authdata,
}
pub type krb5_creds = _krb5_creds;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_last_req_entry {
    pub magic: krb5_magic,
    pub lr_type: krb5_int32,
    pub value: krb5_timestamp,
}
pub type krb5_last_req_entry = _krb5_last_req_entry;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_pa_data {
    pub magic: krb5_magic,
    pub pa_type: krb5_preauthtype,
    pub length: ::std::os::raw::c_uint,
    pub contents: *mut krb5_octet,
}
pub type krb5_pa_data = _krb5_pa_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_kdc_req {
    pub magic: krb5_magic,
    pub msg_type: krb5_msgtype,
    pub padata: *mut *mut krb5_pa_data,
    pub kdc_options: krb5_flags,
    pub client: krb5_principal,
    pub server: krb5_principal,
    pub from: krb5_timestamp,
    pub till: krb5_timestamp,
    pub rtime: krb5_timestamp,
    pub nonce: krb5_int32,
    pub nktypes: ::std::os::raw::c_int,
    pub ktype: *mut krb5_enctype,
    pub addresses: *mut *mut krb5_address,
    pub authorization_data: krb5_enc_data,
    pub unenc_authdata: *mut *mut krb5_authdata,
    pub second_ticket: *mut *mut krb5_ticket,
}
pub type krb5_kdc_req = _krb5_kdc_req;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_enc_kdc_rep_part {
    pub magic: krb5_magic,
    pub msg_type: krb5_msgtype,
    pub session: *mut krb5_keyblock,
    pub last_req: *mut *mut krb5_last_req_entry,
    pub nonce: krb5_int32,
    pub key_exp: krb5_timestamp,
    pub flags: krb5_flags,
    pub times: krb5_ticket_times,
    pub server: krb5_principal,
    pub caddrs: *mut *mut krb5_address,
}
pub type krb5_enc_kdc_rep_part = _krb5_enc_kdc_rep_part;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_kdc_rep {
    pub magic: krb5_magic,
    pub msg_type: krb5_msgtype,
    pub padata: *mut *mut krb5_pa_data,
    pub client: krb5_principal,
    pub ticket: *mut krb5_ticket,
    pub enc_part: krb5_enc_data,
    pub enc_part2: *mut krb5_enc_kdc_rep_part,
}
pub type krb5_kdc_rep = _krb5_kdc_rep;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_error {
    pub magic: krb5_magic,
    pub ctime: krb5_timestamp,
    pub cusec: krb5_int32,
    pub susec: krb5_int32,
    pub stime: krb5_timestamp,
    pub error: krb5_ui_4,
    pub client: krb5_principal,
    pub server: krb5_principal,
    pub text: krb5_data,
    pub e_data: krb5_data,
}
pub type krb5_error = _krb5_error;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_ap_req {
    pub magic: krb5_magic,
    pub ap_options: krb5_flags,
    pub ticket: *mut krb5_ticket,
    pub authenticator: krb5_enc_data,
}
pub type krb5_ap_req = _krb5_ap_req;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_ap_rep {
    pub magic: krb5_magic,
    pub enc_part: krb5_enc_data,
}
pub type krb5_ap_rep = _krb5_ap_rep;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_ap_rep_enc_part {
    pub magic: krb5_magic,
    pub ctime: krb5_timestamp,
    pub cusec: krb5_int32,
    pub subkey: *mut krb5_keyblock,
    pub seq_number: krb5_ui_4,
}
pub type krb5_ap_rep_enc_part = _krb5_ap_rep_enc_part;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_response {
    pub magic: krb5_magic,
    pub message_type: krb5_octet,
    pub response: krb5_data,
    pub expected_nonce: krb5_int32,
    pub request_time: krb5_timestamp,
}
pub type krb5_response = _krb5_response;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_cred_info {
    pub magic: krb5_magic,
    pub session: *mut krb5_keyblock,
    pub client: krb5_principal,
    pub server: krb5_principal,
    pub flags: krb5_flags,
    pub times: krb5_ticket_times,
    pub caddrs: *mut *mut krb5_address,
}
pub type krb5_cred_info = _krb5_cred_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_cred_enc_part {
    pub magic: krb5_magic,
    pub nonce: krb5_int32,
    pub timestamp: krb5_timestamp,
    pub usec: krb5_int32,
    pub s_address: *mut krb5_address,
    pub r_address: *mut krb5_address,
    pub ticket_info: *mut *mut krb5_cred_info,
}
pub type krb5_cred_enc_part = _krb5_cred_enc_part;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_cred {
    pub magic: krb5_magic,
    pub tickets: *mut *mut krb5_ticket,
    pub enc_part: krb5_enc_data,
    pub enc_part2: *mut krb5_cred_enc_part,
}
pub type krb5_cred = _krb5_cred;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _passwd_phrase_element {
    pub magic: krb5_magic,
    pub passwd: *mut krb5_data,
    pub phrase: *mut krb5_data,
}
pub type passwd_phrase_element = _passwd_phrase_element;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_pwd_data {
    pub magic: krb5_magic,
    pub sequence_count: ::std::os::raw::c_int,
    pub element: *mut *mut passwd_phrase_element,
}
pub type krb5_pwd_data = _krb5_pwd_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct krb5_replay_data {
    pub timestamp: krb5_timestamp,
    pub usec: krb5_int32,
    pub seq: krb5_ui_4,
}
pub type krb5_mk_req_checksum_func = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut ::std::os::raw::c_void,
        arg4: *mut *mut krb5_data,
    ) -> krb5_error_code,
>;
pub type krb5_cc_cursor = krb5_pointer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_ccache {
    _unused: [u8; 0],
}
pub type krb5_ccache = *mut _krb5_ccache;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_cc_ops {
    _unused: [u8; 0],
}
pub type krb5_cc_ops = _krb5_cc_ops;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_cccol_cursor {
    _unused: [u8; 0],
}
pub type krb5_cccol_cursor = *mut _krb5_cccol_cursor;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct krb5_rc_st {
    _unused: [u8; 0],
}
pub type krb5_rcache = *mut krb5_rc_st;
pub type krb5_kt_cursor = krb5_pointer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct krb5_keytab_entry_st {
    pub magic: krb5_magic,
    pub principal: krb5_principal,
    pub timestamp: krb5_timestamp,
    pub vno: krb5_kvno,
    pub key: krb5_keyblock,
}
pub type krb5_keytab_entry = krb5_keytab_entry_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_kt {
    _unused: [u8; 0],
}
pub type krb5_keytab = *mut _krb5_kt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct credentials {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_prompt {
    pub prompt: *mut ::std::os::raw::c_char,
    pub hidden: ::std::os::raw::c_int,
    pub reply: *mut krb5_data,
}
pub type krb5_prompt = _krb5_prompt;
pub type krb5_prompter_fct = ::std::option::Option<
    unsafe extern "C" fn(
        context: krb5_context,
        data: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        banner: *const ::std::os::raw::c_char,
        num_prompts: ::std::os::raw::c_int,
        prompts: *mut krb5_prompt,
    ) -> krb5_error_code,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_get_init_creds_opt {
    pub flags: krb5_flags,
    pub tkt_life: krb5_deltat,
    pub renew_life: krb5_deltat,
    pub forwardable: ::std::os::raw::c_int,
    pub proxiable: ::std::os::raw::c_int,
    pub etype_list: *mut krb5_enctype,
    pub etype_list_length: ::std::os::raw::c_int,
    pub address_list: *mut *mut krb5_address,
    pub preauth_list: *mut krb5_preauthtype,
    pub preauth_list_length: ::std::os::raw::c_int,
    pub salt: *mut krb5_data,
}
pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_gic_opt_pa_data {
    pub attr: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
}
pub type krb5_gic_opt_pa_data = _krb5_gic_opt_pa_data;
pub type krb5_gic_process_last_req = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: krb5_context,
        arg2: *mut *mut krb5_last_req_entry,
        arg3: *mut ::std::os::raw::c_void,
    ) -> krb5_error_code,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _krb5_verify_init_creds_opt {
    pub flags: krb5_flags,
    pub ap_req_nofail: ::std::os::raw::c_int,
}
pub type krb5_verify_init_creds_opt = _krb5_verify_init_creds_opt;
pub type krb5_prompt_type = krb5_int32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_name_struct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_cred_id_struct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_ctx_id_struct {
    _unused: [u8; 0],
}
pub type gss_int32 = i32;
pub type gss_OID = *mut gss_OID_desc_struct;
pub type gss_uint64 = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct apple_gss_krb5_authdata_if_relevant_key {
    pub type_: OM_uint32,
    pub length: OM_uint32,
    pub data: *mut ::std::os::raw::c_void,
}
pub type apple_gss_krb5_authdata_if_relevant = apple_gss_krb5_authdata_if_relevant_key;
pub type KLEKerberosVersion = i32;
pub type KLEDialogIdentifiers = i32;
pub type KLEDefaultLoginOptions = i32;
pub type KLERealmListIndexes = i32;
pub type KLEStatus = i32;
pub type KLStatus = i32;
pub type KLKerberosVersion = u32;
pub type KLDefaultLoginOption = u32;
pub type KLLoginMode = u32;
pub type KLDialogIdentifier = u32;
pub type KLIndex = u32;
pub type KLLifetime = u32;
pub type KLTime = u32;
pub type KLSize = u32;
pub type KLRefCon = u32;
pub type KLBoolean = i8;
pub type KLSInt16 = i16;
pub type KLIdleCallback = ::std::option::Option<unsafe extern "C" fn(appData: KLRefCon)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kim_identity_opaque {
    _unused: [u8; 0],
}
pub type KLPrincipal = *mut kim_identity_opaque;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kim_options_opaque {
    _unused: [u8; 0],
}
pub type KLLoginOptions = *mut kim_options_opaque;
pub type apiCB = cc_context_d;
pub type ccache_p = cc_ccache_d;
pub type ccache_cit_creds = cc_credentials_iterator_d;
pub type ccache_cit_ccache = cc_ccache_iterator_d;
pub type cc_data_compat = cc_data;
pub type cc_cred_vers = cc_int32;
pub type cc_result = cc_int32;
pub type cc_flags = cc_uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_credentials_v5_compat {
    pub client: *mut ::std::os::raw::c_char,
    pub server: *mut ::std::os::raw::c_char,
    pub keyblock: cc_data_compat,
    pub authtime: cc_time_t,
    pub starttime: cc_time_t,
    pub endtime: cc_time_t,
    pub renew_till: cc_time_t,
    pub is_skey: cc_uint32,
    pub ticket_flags: cc_uint32,
    pub addresses: *mut *mut cc_data_compat,
    pub ticket: cc_data_compat,
    pub second_ticket: cc_data_compat,
    pub authdata: *mut *mut cc_data_compat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cc_credentials_v4_compat {
    pub kversion: ::std::os::raw::c_uchar,
    pub principal: [::std::os::raw::c_char; 41usize],
    pub principal_instance: [::std::os::raw::c_char; 41usize],
    pub service: [::std::os::raw::c_char; 41usize],
    pub service_instance: [::std::os::raw::c_char; 41usize],
    pub realm: [::std::os::raw::c_char; 41usize],
    pub session_key: [::std::os::raw::c_uchar; 8usize],
    pub kvno: cc_int32,
    pub str_to_key: cc_int32,
    pub issue_date: ::std::os::raw::c_long,
    pub lifetime: cc_int32,
    pub address: cc_uint32,
    pub ticket_sz: cc_int32,
    pub ticket: [::std::os::raw::c_uchar; 1250usize],
    pub oops: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cred_ptr_union_compat {
    pub pV4Cred: *mut cc_credentials_v4_compat,
    pub pV5Cred: *mut cc_credentials_v5_compat,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred_union {
    pub cred_type: cc_int32,
    pub cred: cred_ptr_union_compat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct infoNC {
    pub name: *mut ::std::os::raw::c_char,
    pub principal: *mut ::std::os::raw::c_char,
    pub vers: cc_int32,
}
pub type V4Cred_type = cc_credentials_v4_compat;
pub type cc_creds = cc_credentials_v5_compat;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ccache_cit {
    _unused: [u8; 0],
}
pub type locate_service_type = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct krb5plugin_service_locate_ftable {
    pub minor_version: ::std::os::raw::c_int,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: krb5_context,
            arg2: *mut *mut ::std::os::raw::c_void,
        ) -> krb5_error_code,
    >,
    pub fini: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub lookup: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            svc: locate_service_type,
            realm: *const ::std::os::raw::c_char,
            socktype: ::std::os::raw::c_int,
            family: ::std::os::raw::c_int,
            cbfunc: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                    arg2: ::std::os::raw::c_int,
                    arg3: *mut sockaddr,
                ) -> ::std::os::raw::c_int,
            >,
            cbdata: *mut ::std::os::raw::c_void,
        ) -> krb5_error_code,
    >,
}
unsafe extern "C" {
    pub fn cc_initialize(
        out_context: *mut cc_context_t,
        in_version: cc_int32,
        out_supported_version: *mut cc_int32,
        out_vendor: *mut *const ::std::os::raw::c_char,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn com_err(
        progname: *const ::std::os::raw::c_char,
        code: errcode_t,
        format: *const ::std::os::raw::c_char,
        ...
    );
}
unsafe extern "C" {
    pub fn com_err_va(
        progname: *const ::std::os::raw::c_char,
        code: errcode_t,
        format: *const ::std::os::raw::c_char,
        args: va_list,
    );
}
unsafe extern "C" {
    pub fn error_message(code: errcode_t) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn set_com_err_hook(handler: com_err_handler_t) -> com_err_handler_t;
}
unsafe extern "C" {
    pub fn reset_com_err_hook() -> com_err_handler_t;
}
unsafe extern "C" {
    pub fn add_error_table(et: *const error_table) -> errcode_t;
}
unsafe extern "C" {
    pub fn remove_error_table(et: *const error_table) -> errcode_t;
}
unsafe extern "C" {
    pub fn profile_init(
        files: *mut const_profile_filespec_t,
        ret_profile: *mut profile_t,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_init_path(
        filelist: const_profile_filespec_list_t,
        ret_profile: *mut profile_t,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_configuration_updated();
}
unsafe extern "C" {
    pub fn profile_flush(profile: profile_t) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_flush_to_file(
        profile: profile_t,
        outfile: const_profile_filespec_t,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_flush_to_buffer(
        profile: profile_t,
        bufp: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_free_buffer(profile: profile_t, buf: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn profile_is_writable(
        profile: profile_t,
        writable: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_is_modified(
        profile: profile_t,
        modified: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_abandon(profile: profile_t);
}
unsafe extern "C" {
    pub fn profile_release(profile: profile_t);
}
unsafe extern "C" {
    pub fn profile_get_values(
        profile: profile_t,
        names: *const *const ::std::os::raw::c_char,
        ret_values: *mut *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_free_list(list: *mut *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn profile_get_string(
        profile: profile_t,
        name: *const ::std::os::raw::c_char,
        subname: *const ::std::os::raw::c_char,
        subsubname: *const ::std::os::raw::c_char,
        def_val: *const ::std::os::raw::c_char,
        ret_string: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_get_integer(
        profile: profile_t,
        name: *const ::std::os::raw::c_char,
        subname: *const ::std::os::raw::c_char,
        subsubname: *const ::std::os::raw::c_char,
        def_val: ::std::os::raw::c_int,
        ret_default: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_get_boolean(
        profile: profile_t,
        name: *const ::std::os::raw::c_char,
        subname: *const ::std::os::raw::c_char,
        subsubname: *const ::std::os::raw::c_char,
        def_val: ::std::os::raw::c_int,
        ret_default: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_get_relation_names(
        profile: profile_t,
        names: *mut *const ::std::os::raw::c_char,
        ret_names: *mut *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_get_subsection_names(
        profile: profile_t,
        names: *mut *const ::std::os::raw::c_char,
        ret_names: *mut *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_iterator_create(
        profile: profile_t,
        names: *const *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        ret_iter: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_iterator_free(iter_p: *mut *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn profile_iterator(
        iter_p: *mut *mut ::std::os::raw::c_void,
        ret_name: *mut *mut ::std::os::raw::c_char,
        ret_value: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_release_string(str_: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn profile_update_relation(
        profile: profile_t,
        names: *mut *const ::std::os::raw::c_char,
        old_value: *const ::std::os::raw::c_char,
        new_value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_clear_relation(
        profile: profile_t,
        names: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_rename_section(
        profile: profile_t,
        names: *mut *const ::std::os::raw::c_char,
        new_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn profile_add_relation(
        profile: profile_t,
        names: *mut *const ::std::os::raw::c_char,
        new_value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn initialize_prof_error_table();
}
unsafe extern "C" {
    pub fn krb5_is_referral_realm(arg1: *const krb5_data) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_c_encrypt(
        context: krb5_context,
        key: *const krb5_keyblock,
        usage: krb5_keyusage,
        cipher_state: *const krb5_data,
        input: *const krb5_data,
        output: *mut krb5_enc_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_decrypt(
        context: krb5_context,
        key: *const krb5_keyblock,
        usage: krb5_keyusage,
        cipher_state: *const krb5_data,
        input: *const krb5_enc_data,
        output: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_encrypt_length(
        context: krb5_context,
        enctype: krb5_enctype,
        inputlen: usize,
        length: *mut usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_block_size(
        context: krb5_context,
        enctype: krb5_enctype,
        blocksize: *mut usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_init_state(
        context: krb5_context,
        key: *const krb5_keyblock,
        usage: krb5_keyusage,
        new_state: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_free_state(
        context: krb5_context,
        key: *const krb5_keyblock,
        state: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_make_random_key(
        context: krb5_context,
        enctype: krb5_enctype,
        k5_random_key: *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_random_add_entropy(
        context: krb5_context,
        randsource_id: ::std::os::raw::c_uint,
        data: *const krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_random_make_octets(
        context: krb5_context,
        data: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_random_os_entropy(
        context: krb5_context,
        strong: ::std::os::raw::c_int,
        success: *mut ::std::os::raw::c_int,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_random_seed(context: krb5_context, data: *mut krb5_data) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_string_to_key(
        context: krb5_context,
        enctype: krb5_enctype,
        string: *const krb5_data,
        salt: *const krb5_data,
        key: *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_string_to_key_with_params(
        context: krb5_context,
        enctype: krb5_enctype,
        string: *const krb5_data,
        salt: *const krb5_data,
        params: *const krb5_data,
        key: *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_enctype_compare(
        context: krb5_context,
        e1: krb5_enctype,
        e2: krb5_enctype,
        similar: *mut krb5_boolean,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_make_checksum(
        context: krb5_context,
        cksumtype: krb5_cksumtype,
        key: *const krb5_keyblock,
        usage: krb5_keyusage,
        input: *const krb5_data,
        cksum: *mut krb5_checksum,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_verify_checksum(
        context: krb5_context,
        key: *const krb5_keyblock,
        usage: krb5_keyusage,
        data: *const krb5_data,
        cksum: *const krb5_checksum,
        valid: *mut krb5_boolean,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_checksum_length(
        context: krb5_context,
        cksumtype: krb5_cksumtype,
        length: *mut usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_keyed_checksum_types(
        context: krb5_context,
        enctype: krb5_enctype,
        count: *mut ::std::os::raw::c_uint,
        cksumtypes: *mut *mut krb5_cksumtype,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_c_valid_enctype(ktype: krb5_enctype) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_c_valid_cksumtype(ctype: krb5_cksumtype) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_c_is_coll_proof_cksum(ctype: krb5_cksumtype) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_c_is_keyed_cksum(ctype: krb5_cksumtype) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_encrypt(
        context: krb5_context,
        inptr: krb5_const_pointer,
        outptr: krb5_pointer,
        size: usize,
        eblock: *mut krb5_encrypt_block,
        ivec: krb5_pointer,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_decrypt(
        context: krb5_context,
        inptr: krb5_const_pointer,
        outptr: krb5_pointer,
        size: usize,
        eblock: *mut krb5_encrypt_block,
        ivec: krb5_pointer,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_process_key(
        context: krb5_context,
        eblock: *mut krb5_encrypt_block,
        key: *const krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_finish_key(
        context: krb5_context,
        eblock: *mut krb5_encrypt_block,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_string_to_key(
        context: krb5_context,
        eblock: *const krb5_encrypt_block,
        keyblock: *mut krb5_keyblock,
        data: *const krb5_data,
        salt: *const krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_init_random_key(
        context: krb5_context,
        eblock: *const krb5_encrypt_block,
        keyblock: *const krb5_keyblock,
        ptr: *mut krb5_pointer,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_finish_random_key(
        context: krb5_context,
        eblock: *const krb5_encrypt_block,
        ptr: *mut krb5_pointer,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_random_key(
        context: krb5_context,
        eblock: *const krb5_encrypt_block,
        ptr: krb5_pointer,
        keyblock: *mut *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_eblock_enctype(
        context: krb5_context,
        eblock: *const krb5_encrypt_block,
    ) -> krb5_enctype;
}
unsafe extern "C" {
    pub fn krb5_use_enctype(
        context: krb5_context,
        eblock: *mut krb5_encrypt_block,
        enctype: krb5_enctype,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_encrypt_size(length: usize, crypto: krb5_enctype) -> usize;
}
unsafe extern "C" {
    pub fn krb5_checksum_size(context: krb5_context, ctype: krb5_cksumtype) -> usize;
}
unsafe extern "C" {
    pub fn krb5_calculate_checksum(
        context: krb5_context,
        ctype: krb5_cksumtype,
        in_: krb5_const_pointer,
        in_length: usize,
        seed: krb5_const_pointer,
        seed_length: usize,
        outcksum: *mut krb5_checksum,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_verify_checksum(
        context: krb5_context,
        ctype: krb5_cksumtype,
        cksum: *const krb5_checksum,
        in_: krb5_const_pointer,
        in_length: usize,
        seed: krb5_const_pointer,
        seed_length: usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_get_name(
        context: krb5_context,
        cache: krb5_ccache,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn krb5_cc_gen_new(context: krb5_context, cache: *mut krb5_ccache) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_initialize(
        context: krb5_context,
        cache: krb5_ccache,
        principal: krb5_principal,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_destroy(context: krb5_context, cache: krb5_ccache) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_store_cred(
        context: krb5_context,
        cache: krb5_ccache,
        creds: *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_retrieve_cred(
        context: krb5_context,
        cache: krb5_ccache,
        flags: krb5_flags,
        mcreds: *mut krb5_creds,
        creds: *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_get_principal(
        context: krb5_context,
        cache: krb5_ccache,
        principal: *mut krb5_principal,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_start_seq_get(
        context: krb5_context,
        cache: krb5_ccache,
        cursor: *mut krb5_cc_cursor,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_next_cred(
        context: krb5_context,
        cache: krb5_ccache,
        cursor: *mut krb5_cc_cursor,
        creds: *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_end_seq_get(
        context: krb5_context,
        cache: krb5_ccache,
        cursor: *mut krb5_cc_cursor,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_remove_cred(
        context: krb5_context,
        cache: krb5_ccache,
        flags: krb5_flags,
        creds: *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_set_flags(
        context: krb5_context,
        cache: krb5_ccache,
        flags: krb5_flags,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_get_type(
        context: krb5_context,
        cache: krb5_ccache,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn krb5_cc_move(
        context: krb5_context,
        src: krb5_ccache,
        dst: krb5_ccache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_last_change_time(
        context: krb5_context,
        ccache: krb5_ccache,
        change_time: *mut krb5_timestamp,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_lock(context: krb5_context, ccache: krb5_ccache) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_unlock(context: krb5_context, ccache: krb5_ccache) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_cache_match(
        context: krb5_context,
        client: krb5_principal,
        id: *mut krb5_ccache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cccol_cursor_new(
        context: krb5_context,
        cursor: *mut krb5_cccol_cursor,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cccol_cursor_next(
        context: krb5_context,
        cursor: krb5_cccol_cursor,
        ccache: *mut krb5_ccache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cccol_cursor_free(
        context: krb5_context,
        cursor: *mut krb5_cccol_cursor,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cccol_last_change_time(
        context: krb5_context,
        change_time: *mut krb5_timestamp,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cccol_lock(context: krb5_context) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cccol_unlock(context: krb5_context) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_new_unique(
        context: krb5_context,
        type_: *const ::std::os::raw::c_char,
        hint: *const ::std::os::raw::c_char,
        id: *mut krb5_ccache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_get_type(
        arg1: krb5_context,
        keytab: krb5_keytab,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn krb5_kt_get_name(
        context: krb5_context,
        keytab: krb5_keytab,
        name: *mut ::std::os::raw::c_char,
        namelen: ::std::os::raw::c_uint,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_close(context: krb5_context, keytab: krb5_keytab) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_get_entry(
        context: krb5_context,
        keytab: krb5_keytab,
        principal: krb5_const_principal,
        vno: krb5_kvno,
        enctype: krb5_enctype,
        entry: *mut krb5_keytab_entry,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_start_seq_get(
        context: krb5_context,
        keytab: krb5_keytab,
        cursor: *mut krb5_kt_cursor,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_next_entry(
        context: krb5_context,
        keytab: krb5_keytab,
        entry: *mut krb5_keytab_entry,
        cursor: *mut krb5_kt_cursor,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_end_seq_get(
        context: krb5_context,
        keytab: krb5_keytab,
        cursor: *mut krb5_kt_cursor,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_init_context(arg1: *mut krb5_context) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_init_secure_context(arg1: *mut krb5_context) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_free_context(arg1: krb5_context);
}
unsafe extern "C" {
    pub fn krb5_copy_context(arg1: krb5_context, arg2: *mut krb5_context) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_set_default_tgs_enctypes(
        arg1: krb5_context,
        arg2: *const krb5_enctype,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_permitted_enctypes(
        arg1: krb5_context,
        arg2: *mut *mut krb5_enctype,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_is_thread_safe() -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_server_decrypt_ticket_keytab(
        context: krb5_context,
        kt: krb5_keytab,
        ticket: *mut krb5_ticket,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_free_tgt_creds(arg1: krb5_context, arg2: *mut *mut krb5_creds);
}
unsafe extern "C" {
    pub fn krb5_get_credentials(
        arg1: krb5_context,
        arg2: krb5_flags,
        arg3: krb5_ccache,
        arg4: *mut krb5_creds,
        arg5: *mut *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_credentials_validate(
        arg1: krb5_context,
        arg2: krb5_flags,
        arg3: krb5_ccache,
        arg4: *mut krb5_creds,
        arg5: *mut *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_credentials_renew(
        arg1: krb5_context,
        arg2: krb5_flags,
        arg3: krb5_ccache,
        arg4: *mut krb5_creds,
        arg5: *mut *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_mk_req(
        arg1: krb5_context,
        arg2: *mut krb5_auth_context,
        arg3: krb5_flags,
        arg4: *mut ::std::os::raw::c_char,
        arg5: *mut ::std::os::raw::c_char,
        arg6: *mut krb5_data,
        arg7: krb5_ccache,
        arg8: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_mk_req_extended(
        arg1: krb5_context,
        arg2: *mut krb5_auth_context,
        arg3: krb5_flags,
        arg4: *mut krb5_data,
        arg5: *mut krb5_creds,
        arg6: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_mk_rep(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_rd_rep(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *const krb5_data,
        arg4: *mut *mut krb5_ap_rep_enc_part,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_mk_error(
        arg1: krb5_context,
        arg2: *const krb5_error,
        arg3: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_rd_error(
        arg1: krb5_context,
        arg2: *const krb5_data,
        arg3: *mut *mut krb5_error,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_rd_safe(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *const krb5_data,
        arg4: *mut krb5_data,
        arg5: *mut krb5_replay_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_rd_priv(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *const krb5_data,
        arg4: *mut krb5_data,
        arg5: *mut krb5_replay_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_parse_name(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut krb5_principal,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_parse_name_flags(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: *mut krb5_principal,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_unparse_name(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_unparse_name_ext(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: *mut *mut ::std::os::raw::c_char,
        arg4: *mut ::std::os::raw::c_uint,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_unparse_name_flags(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: ::std::os::raw::c_int,
        arg4: *mut *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_unparse_name_flags_ext(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: ::std::os::raw::c_int,
        arg4: *mut *mut ::std::os::raw::c_char,
        arg5: *mut ::std::os::raw::c_uint,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_set_principal_realm(
        arg1: krb5_context,
        arg2: krb5_principal,
        arg3: *const ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_address_search(
        arg1: krb5_context,
        arg2: *const krb5_address,
        arg3: *const *mut krb5_address,
    ) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_address_compare(
        arg1: krb5_context,
        arg2: *const krb5_address,
        arg3: *const krb5_address,
    ) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_address_order(
        arg1: krb5_context,
        arg2: *const krb5_address,
        arg3: *const krb5_address,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn krb5_realm_compare(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: krb5_const_principal,
    ) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_principal_compare(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: krb5_const_principal,
    ) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_init_keyblock(
        arg1: krb5_context,
        enctype: krb5_enctype,
        length: usize,
        out: *mut *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_keyblock(
        arg1: krb5_context,
        arg2: *const krb5_keyblock,
        arg3: *mut *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_keyblock_contents(
        arg1: krb5_context,
        arg2: *const krb5_keyblock,
        arg3: *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_creds(
        arg1: krb5_context,
        arg2: *const krb5_creds,
        arg3: *mut *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_data(
        arg1: krb5_context,
        arg2: *const krb5_data,
        arg3: *mut *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_principal(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: *mut krb5_principal,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_addresses(
        arg1: krb5_context,
        arg2: *const *mut krb5_address,
        arg3: *mut *mut *mut krb5_address,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_ticket(
        arg1: krb5_context,
        arg2: *const krb5_ticket,
        arg3: *mut *mut krb5_ticket,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_authdata(
        arg1: krb5_context,
        arg2: *const *mut krb5_authdata,
        arg3: *mut *mut *mut krb5_authdata,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_authenticator(
        arg1: krb5_context,
        arg2: *const krb5_authenticator,
        arg3: *mut *mut krb5_authenticator,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_copy_checksum(
        arg1: krb5_context,
        arg2: *const krb5_checksum,
        arg3: *mut *mut krb5_checksum,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_server_rcache(
        arg1: krb5_context,
        arg2: *const krb5_data,
        arg3: *mut krb5_rcache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_build_principal_ext(
        arg1: krb5_context,
        arg2: *mut krb5_principal,
        arg3: ::std::os::raw::c_uint,
        arg4: *const ::std::os::raw::c_char,
        ...
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_build_principal(
        arg1: krb5_context,
        arg2: *mut krb5_principal,
        arg3: ::std::os::raw::c_uint,
        arg4: *const ::std::os::raw::c_char,
        ...
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_build_principal_alloc_va(
        arg1: krb5_context,
        arg2: *mut krb5_principal,
        arg3: ::std::os::raw::c_uint,
        arg4: *const ::std::os::raw::c_char,
        arg5: va_list,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_425_conv_principal(
        arg1: krb5_context,
        name: *const ::std::os::raw::c_char,
        instance: *const ::std::os::raw::c_char,
        realm: *const ::std::os::raw::c_char,
        princ: *mut krb5_principal,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_524_conv_principal(
        context: krb5_context,
        princ: krb5_const_principal,
        name: *mut ::std::os::raw::c_char,
        inst: *mut ::std::os::raw::c_char,
        realm: *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_524_convert_creds(
        context: krb5_context,
        v5creds: *mut krb5_creds,
        v4creds: *mut credentials,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn krb5_kt_resolve(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut krb5_keytab,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_default_name(
        arg1: krb5_context,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_default(arg1: krb5_context, arg2: *mut krb5_keytab) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_free_keytab_entry_contents(
        arg1: krb5_context,
        arg2: *mut krb5_keytab_entry,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_remove_entry(
        arg1: krb5_context,
        arg2: krb5_keytab,
        arg3: *mut krb5_keytab_entry,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_add_entry(
        arg1: krb5_context,
        arg2: krb5_keytab,
        arg3: *mut krb5_keytab_entry,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_principal2salt(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_resolve(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut krb5_ccache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_default_name(arg1: krb5_context) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn krb5_cc_set_default_name(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_default(arg1: krb5_context, arg2: *mut krb5_ccache) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_copy_creds(
        context: krb5_context,
        incc: krb5_ccache,
        outcc: krb5_ccache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_get_config(
        arg1: krb5_context,
        arg2: krb5_ccache,
        arg3: krb5_const_principal,
        arg4: *const ::std::os::raw::c_char,
        arg5: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cc_set_config(
        arg1: krb5_context,
        arg2: krb5_ccache,
        arg3: krb5_const_principal,
        arg4: *const ::std::os::raw::c_char,
        arg5: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_is_config_principal(arg1: krb5_context, arg2: krb5_const_principal)
        -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_free_principal(arg1: krb5_context, arg2: krb5_principal);
}
unsafe extern "C" {
    pub fn krb5_free_authenticator(arg1: krb5_context, arg2: *mut krb5_authenticator);
}
unsafe extern "C" {
    pub fn krb5_free_addresses(arg1: krb5_context, arg2: *mut *mut krb5_address);
}
unsafe extern "C" {
    pub fn krb5_free_authdata(arg1: krb5_context, arg2: *mut *mut krb5_authdata);
}
unsafe extern "C" {
    pub fn krb5_free_ticket(arg1: krb5_context, arg2: *mut krb5_ticket);
}
unsafe extern "C" {
    pub fn krb5_free_error(arg1: krb5_context, arg2: *mut krb5_error);
}
unsafe extern "C" {
    pub fn krb5_free_creds(arg1: krb5_context, arg2: *mut krb5_creds);
}
unsafe extern "C" {
    pub fn krb5_free_cred_contents(arg1: krb5_context, arg2: *mut krb5_creds);
}
unsafe extern "C" {
    pub fn krb5_free_checksum(arg1: krb5_context, arg2: *mut krb5_checksum);
}
unsafe extern "C" {
    pub fn krb5_free_checksum_contents(arg1: krb5_context, arg2: *mut krb5_checksum);
}
unsafe extern "C" {
    pub fn krb5_free_keyblock(arg1: krb5_context, arg2: *mut krb5_keyblock);
}
unsafe extern "C" {
    pub fn krb5_free_keyblock_contents(arg1: krb5_context, arg2: *mut krb5_keyblock);
}
unsafe extern "C" {
    pub fn krb5_free_ap_rep_enc_part(arg1: krb5_context, arg2: *mut krb5_ap_rep_enc_part);
}
unsafe extern "C" {
    pub fn krb5_free_data(arg1: krb5_context, arg2: *mut krb5_data);
}
unsafe extern "C" {
    pub fn krb5_free_data_contents(arg1: krb5_context, arg2: *mut krb5_data);
}
unsafe extern "C" {
    pub fn krb5_free_unparsed_name(arg1: krb5_context, arg2: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn krb5_free_cksumtypes(arg1: krb5_context, arg2: *mut krb5_cksumtype);
}
unsafe extern "C" {
    pub fn krb5_us_timeofday(
        arg1: krb5_context,
        arg2: *mut krb5_timestamp,
        arg3: *mut krb5_int32,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_timeofday(arg1: krb5_context, arg2: *mut krb5_timestamp) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_os_localaddr(
        arg1: krb5_context,
        arg2: *mut *mut *mut krb5_address,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_default_realm(
        arg1: krb5_context,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_set_default_realm(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_free_default_realm(arg1: krb5_context, arg2: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn krb5_sname_to_principal(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
        arg4: krb5_int32,
        arg5: *mut krb5_principal,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_change_password(
        context: krb5_context,
        creds: *mut krb5_creds,
        newpw: *mut ::std::os::raw::c_char,
        result_code: *mut ::std::os::raw::c_int,
        result_code_string: *mut krb5_data,
        result_string: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_set_password(
        context: krb5_context,
        creds: *mut krb5_creds,
        newpw: *mut ::std::os::raw::c_char,
        change_password_for: krb5_principal,
        result_code: *mut ::std::os::raw::c_int,
        result_code_string: *mut krb5_data,
        result_string: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_set_password_using_ccache(
        context: krb5_context,
        ccache: krb5_ccache,
        newpw: *mut ::std::os::raw::c_char,
        change_password_for: krb5_principal,
        result_code: *mut ::std::os::raw::c_int,
        result_code_string: *mut krb5_data,
        result_string: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_profile(arg1: krb5_context, arg2: *mut *mut _profile_t) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_rd_req(
        arg1: krb5_context,
        arg2: *mut krb5_auth_context,
        arg3: *const krb5_data,
        arg4: krb5_const_principal,
        arg5: krb5_keytab,
        arg6: *mut krb5_flags,
        arg7: *mut *mut krb5_ticket,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kt_read_service_key(
        arg1: krb5_context,
        arg2: krb5_pointer,
        arg3: krb5_principal,
        arg4: krb5_kvno,
        arg5: krb5_enctype,
        arg6: *mut *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_mk_safe(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *const krb5_data,
        arg4: *mut krb5_data,
        arg5: *mut krb5_replay_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_mk_priv(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *const krb5_data,
        arg4: *mut krb5_data,
        arg5: *mut krb5_replay_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_sendauth(
        arg1: krb5_context,
        arg2: *mut krb5_auth_context,
        arg3: krb5_pointer,
        arg4: *mut ::std::os::raw::c_char,
        arg5: krb5_principal,
        arg6: krb5_principal,
        arg7: krb5_flags,
        arg8: *mut krb5_data,
        arg9: *mut krb5_creds,
        arg10: krb5_ccache,
        arg11: *mut *mut krb5_error,
        arg12: *mut *mut krb5_ap_rep_enc_part,
        arg13: *mut *mut krb5_creds,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_recvauth(
        arg1: krb5_context,
        arg2: *mut krb5_auth_context,
        arg3: krb5_pointer,
        arg4: *mut ::std::os::raw::c_char,
        arg5: krb5_principal,
        arg6: krb5_int32,
        arg7: krb5_keytab,
        arg8: *mut *mut krb5_ticket,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_recvauth_version(
        arg1: krb5_context,
        arg2: *mut krb5_auth_context,
        arg3: krb5_pointer,
        arg4: krb5_principal,
        arg5: krb5_int32,
        arg6: krb5_keytab,
        arg7: *mut *mut krb5_ticket,
        arg8: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_mk_ncred(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut *mut krb5_creds,
        arg4: *mut *mut krb5_data,
        arg5: *mut krb5_replay_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_mk_1cred(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_creds,
        arg4: *mut *mut krb5_data,
        arg5: *mut krb5_replay_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_rd_cred(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_data,
        arg4: *mut *mut *mut krb5_creds,
        arg5: *mut krb5_replay_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_fwd_tgt_creds(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut ::std::os::raw::c_char,
        arg4: krb5_principal,
        arg5: krb5_principal,
        arg6: krb5_ccache,
        forwardable: ::std::os::raw::c_int,
        arg7: *mut krb5_data,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_init(arg1: krb5_context, arg2: *mut krb5_auth_context) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_free(arg1: krb5_context, arg2: krb5_auth_context) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_setflags(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: krb5_int32,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getflags(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_int32,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_set_checksum_func(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: krb5_mk_req_checksum_func,
        arg4: *mut ::std::os::raw::c_void,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_get_checksum_func(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_mk_req_checksum_func,
        arg4: *mut *mut ::std::os::raw::c_void,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_setaddrs(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_address,
        arg4: *mut krb5_address,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getaddrs(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut *mut krb5_address,
        arg4: *mut *mut krb5_address,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_setports(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_address,
        arg4: *mut krb5_address,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_setuseruserkey(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getkey(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getsendsubkey(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getrecvsubkey(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_setsendsubkey(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_setrecvsubkey(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_keyblock,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getlocalseqnumber(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_int32,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getremoteseqnumber(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_int32,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_setrcache(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: krb5_rcache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getrcache(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut krb5_rcache,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_auth_con_getauthenticator(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: *mut *mut krb5_authenticator,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_read_password(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
        arg4: *mut ::std::os::raw::c_char,
        arg5: *mut ::std::os::raw::c_uint,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_aname_to_localname(
        arg1: krb5_context,
        arg2: krb5_const_principal,
        arg3: ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_host_realm(
        arg1: krb5_context,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_free_host_realm(
        arg1: krb5_context,
        arg2: *const *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_kuserok(
        arg1: krb5_context,
        arg2: krb5_principal,
        arg3: *const ::std::os::raw::c_char,
    ) -> krb5_boolean;
}
unsafe extern "C" {
    pub fn krb5_auth_con_genaddrs(
        arg1: krb5_context,
        arg2: krb5_auth_context,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_set_real_time(
        arg1: krb5_context,
        arg2: krb5_timestamp,
        arg3: krb5_int32,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_time_offsets(
        arg1: krb5_context,
        arg2: *mut krb5_timestamp,
        arg3: *mut krb5_int32,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_string_to_enctype(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut krb5_enctype,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_string_to_salttype(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut krb5_int32,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_string_to_cksumtype(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut krb5_cksumtype,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_string_to_timestamp(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut krb5_timestamp,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_string_to_deltat(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *mut krb5_deltat,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_enctype_to_string(
        arg1: krb5_enctype,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_salttype_to_string(
        arg1: krb5_int32,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_cksumtype_to_string(
        arg1: krb5_cksumtype,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_timestamp_to_string(
        arg1: krb5_timestamp,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_timestamp_to_sfstring(
        arg1: krb5_timestamp,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_deltat_to_string(
        arg1: krb5_deltat,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_prompter_posix(
        context: krb5_context,
        data: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        banner: *const ::std::os::raw::c_char,
        num_prompts: ::std::os::raw::c_int,
        prompts: *mut krb5_prompt,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_alloc(
        context: krb5_context,
        opt: *mut *mut krb5_get_init_creds_opt,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_free(context: krb5_context, opt: *mut krb5_get_init_creds_opt);
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_init(opt: *mut krb5_get_init_creds_opt);
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_tkt_life(
        opt: *mut krb5_get_init_creds_opt,
        tkt_life: krb5_deltat,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_renew_life(
        opt: *mut krb5_get_init_creds_opt,
        renew_life: krb5_deltat,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_forwardable(
        opt: *mut krb5_get_init_creds_opt,
        forwardable: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_proxiable(
        opt: *mut krb5_get_init_creds_opt,
        proxiable: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_canonicalize(
        opt: *mut krb5_get_init_creds_opt,
        canonicalize: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_etype_list(
        opt: *mut krb5_get_init_creds_opt,
        etype_list: *mut krb5_enctype,
        etype_list_length: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_address_list(
        opt: *mut krb5_get_init_creds_opt,
        addresses: *mut *mut krb5_address,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_preauth_list(
        opt: *mut krb5_get_init_creds_opt,
        preauth_list: *mut krb5_preauthtype,
        preauth_list_length: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_salt(
        opt: *mut krb5_get_init_creds_opt,
        salt: *mut krb5_data,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_change_password_prompt(
        opt: *mut krb5_get_init_creds_opt,
        prompt: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_pa(
        context: krb5_context,
        opt: *mut krb5_get_init_creds_opt,
        attr: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_opt_set_process_last_req(
        arg1: krb5_context,
        arg2: *mut krb5_get_init_creds_opt,
        arg3: krb5_gic_process_last_req,
        arg4: *mut ::std::os::raw::c_void,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_password(
        context: krb5_context,
        creds: *mut krb5_creds,
        client: krb5_principal,
        password: *mut ::std::os::raw::c_char,
        prompter: krb5_prompter_fct,
        data: *mut ::std::os::raw::c_void,
        start_time: krb5_deltat,
        in_tkt_service: *mut ::std::os::raw::c_char,
        k5_gic_options: *mut krb5_get_init_creds_opt,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_init_creds_keytab(
        context: krb5_context,
        creds: *mut krb5_creds,
        client: krb5_principal,
        arg_keytab: krb5_keytab,
        start_time: krb5_deltat,
        in_tkt_service: *mut ::std::os::raw::c_char,
        k5_gic_options: *mut krb5_get_init_creds_opt,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_verify_init_creds_opt_init(k5_vic_options: *mut krb5_verify_init_creds_opt);
}
unsafe extern "C" {
    pub fn krb5_verify_init_creds_opt_set_ap_req_nofail(
        k5_vic_options: *mut krb5_verify_init_creds_opt,
        ap_req_nofail: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn krb5_verify_init_creds(
        context: krb5_context,
        creds: *mut krb5_creds,
        ap_req_server: krb5_principal,
        ap_req_keytab: krb5_keytab,
        ccache: *mut krb5_ccache,
        k5_vic_options: *mut krb5_verify_init_creds_opt,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_validated_creds(
        context: krb5_context,
        creds: *mut krb5_creds,
        client: krb5_principal,
        ccache: krb5_ccache,
        in_tkt_service: *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_get_renewed_creds(
        context: krb5_context,
        creds: *mut krb5_creds,
        client: krb5_principal,
        ccache: krb5_ccache,
        in_tkt_service: *mut ::std::os::raw::c_char,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_decode_ticket(
        code: *const krb5_data,
        rep: *mut *mut krb5_ticket,
    ) -> krb5_error_code;
}
unsafe extern "C" {
    pub fn krb5_appdefault_string(
        context: krb5_context,
        appname: *const ::std::os::raw::c_char,
        realm: *const krb5_data,
        option: *const ::std::os::raw::c_char,
        default_value: *const ::std::os::raw::c_char,
        ret_value: *mut *mut ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn krb5_appdefault_boolean(
        context: krb5_context,
        appname: *const ::std::os::raw::c_char,
        realm: *const krb5_data,
        option: *const ::std::os::raw::c_char,
        default_value: ::std::os::raw::c_int,
        ret_value: *mut ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn krb5_get_prompt_types(context: krb5_context) -> *mut krb5_prompt_type;
}
unsafe extern "C" {
    pub fn krb5_set_error_message(
        arg1: krb5_context,
        arg2: krb5_error_code,
        arg3: *const ::std::os::raw::c_char,
        ...
    );
}
unsafe extern "C" {
    pub fn krb5_vset_error_message(
        arg1: krb5_context,
        arg2: krb5_error_code,
        arg3: *const ::std::os::raw::c_char,
        arg4: va_list,
    );
}
unsafe extern "C" {
    pub fn krb5_get_error_message(
        arg1: krb5_context,
        arg2: krb5_error_code,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn krb5_free_error_message(arg1: krb5_context, arg2: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn krb5_clear_error_message(arg1: krb5_context);
}
unsafe extern "C" {
    pub fn initialize_krb5_error_table();
}
unsafe extern "C" {
    pub fn initialize_k524_error_table();
}
unsafe extern "C" {
    pub fn initialize_asn1_error_table();
}
unsafe extern "C" {
    pub static mut GSS_C_NT_USER_NAME: gss_OID;
}
unsafe extern "C" {
    pub static mut GSS_C_NT_MACHINE_UID_NAME: gss_OID;
}
unsafe extern "C" {
    pub static mut GSS_C_NT_STRING_UID_NAME: gss_OID;
}
unsafe extern "C" {
    pub static mut GSS_C_NT_HOSTBASED_SERVICE_X: gss_OID;
}
unsafe extern "C" {
    pub static mut GSS_C_NT_HOSTBASED_SERVICE: gss_OID;
}
unsafe extern "C" {
    pub static mut GSS_C_NT_ANONYMOUS: gss_OID;
}
unsafe extern "C" {
    pub static mut GSS_C_NT_EXPORT_NAME: gss_OID;
}
unsafe extern "C" {
    pub fn gss_str_to_oid(
        arg1: *mut OM_uint32,
        arg2: gss_buffer_t,
        arg3: *mut gss_OID,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub static mut gss_nt_user_name: gss_OID;
}
unsafe extern "C" {
    pub static mut gss_nt_machine_uid_name: gss_OID;
}
unsafe extern "C" {
    pub static mut gss_nt_string_uid_name: gss_OID;
}
unsafe extern "C" {
    pub static mut gss_nt_service_name_v2: gss_OID;
}
unsafe extern "C" {
    pub static mut gss_nt_service_name: gss_OID;
}
unsafe extern "C" {
    pub static mut gss_nt_exported_name: gss_OID;
}
unsafe extern "C" {
    pub static GSS_KRB5_NT_PRINCIPAL_NAME: *const gss_OID_desc;
}
unsafe extern "C" {
    pub static gss_mech_krb5: *const gss_OID_desc;
}
unsafe extern "C" {
    pub static gss_mech_krb5_old: *const gss_OID_desc;
}
unsafe extern "C" {
    pub static gss_mech_set_krb5: *const gss_OID_set_desc;
}
unsafe extern "C" {
    pub static gss_mech_set_krb5_old: *const gss_OID_set_desc;
}
unsafe extern "C" {
    pub static gss_mech_set_krb5_both: *const gss_OID_set_desc;
}
unsafe extern "C" {
    pub static gss_nt_krb5_name: *const gss_OID_desc;
}
unsafe extern "C" {
    pub static gss_nt_krb5_principal: *const gss_OID_desc;
}
unsafe extern "C" {
    pub static krb5_gss_oid_array: [gss_OID_desc; 0usize];
}
unsafe extern "C" {
    pub fn gss_krb5_get_tkt_flags(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        ticket_flags: *mut krb5_flags,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn apple_gss_krb5_export_authdata_if_relevant_context(
        minor_status: *mut OM_uint32,
        context_handle: *mut gss_ctx_id_t,
        version: OM_uint32,
        kctx: *mut *mut ::std::os::raw::c_void,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn apple_gss_krb5_free_authdata_if_relevant(
        minor_status: *mut OM_uint32,
        kctx: *mut ::std::os::raw::c_void,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_krb5_ui(arg1: *mut OM_uint32, arg2: OM_uint32) -> OM_uint32;
}
unsafe extern "C" {
    pub fn KLAcquireTickets(
        inPrincipal: KLPrincipal,
        outPrincipal: *mut KLPrincipal,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireNewTickets(
        inPrincipal: KLPrincipal,
        outPrincipal: *mut KLPrincipal,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireTicketsWithPassword(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        inPassword: *const ::std::os::raw::c_char,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireNewTicketsWithPassword(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        inPassword: *const ::std::os::raw::c_char,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLSetApplicationOptions(inAppOptions: *const ::std::os::raw::c_void) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetApplicationOptions(outAppOptions: *mut ::std::os::raw::c_void) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireInitialTickets(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        outPrincipal: *mut KLPrincipal,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireNewInitialTickets(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        outPrincipal: *mut KLPrincipal,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLDestroyTickets(inPrincipal: KLPrincipal) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLChangePassword(inPrincipal: KLPrincipal) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireInitialTicketsWithPassword(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        inPassword: *const ::std::os::raw::c_char,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireNewInitialTicketsWithPassword(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        inPassword: *const ::std::os::raw::c_char,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireNewInitialTicketCredentialsWithPassword(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        inPassword: *const ::std::os::raw::c_char,
        inV5Context: krb5_context,
        outGotV4Credentials: *mut KLBoolean,
        outGotV5Credentials: *mut KLBoolean,
        outV4Credentials: *mut ::std::os::raw::c_void,
        outV5Credentials: *mut krb5_creds,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLStoreNewInitialTicketCredentials(
        inPrincipal: KLPrincipal,
        inV5Context: krb5_context,
        inV4Credentials: *mut ::std::os::raw::c_void,
        inV5Credentials: *mut krb5_creds,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLVerifyInitialTickets(
        inPrincipal: KLPrincipal,
        inFailIfNoHostKey: KLBoolean,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLVerifyInitialTicketCredentials(
        inV4Credentials: *mut ::std::os::raw::c_void,
        inV5Credentials: *mut krb5_creds,
        inFailIfNoHostKey: KLBoolean,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLAcquireNewInitialTicketsWithKeytab(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        inKeytabName: *const ::std::os::raw::c_char,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLRenewInitialTickets(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        outPrincipal: *mut KLPrincipal,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLValidateInitialTickets(
        inPrincipal: KLPrincipal,
        inLoginOptions: KLLoginOptions,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLLastChangedTime(outLastChangedTime: *mut KLTime) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLCacheHasValidTickets(
        inPrincipal: KLPrincipal,
        inKerberosVersion: KLKerberosVersion,
        outFoundValidTickets: *mut KLBoolean,
        outPrincipal: *mut KLPrincipal,
        outCredCacheName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLTicketStartTime(
        inPrincipal: KLPrincipal,
        inKerberosVersion: KLKerberosVersion,
        outStartTime: *mut KLTime,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLTicketExpirationTime(
        inPrincipal: KLPrincipal,
        inKerberosVersion: KLKerberosVersion,
        outExpirationTime: *mut KLTime,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLSetSystemDefaultCache(inPrincipal: KLPrincipal) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLHandleError(
        inError: KLStatus,
        inDialogIdentifier: KLDialogIdentifier,
        inShowAlert: KLBoolean,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetErrorString(
        inError: KLStatus,
        outErrorString: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLCancelAllDialogs() -> KLStatus;
}
unsafe extern "C" {
    pub fn KLChangePasswordWithPasswords(
        inPrincipal: KLPrincipal,
        inOldPassword: *const ::std::os::raw::c_char,
        inNewPassword: *const ::std::os::raw::c_char,
        outRejected: *mut KLBoolean,
        outRejectionError: *mut *mut ::std::os::raw::c_char,
        outRejectionDescription: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLSetIdleCallback(inCallback: KLIdleCallback, inRefCon: KLRefCon) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetIdleCallback(inCallback: *mut KLIdleCallback, inRefCon: *mut KLRefCon) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetDefaultLoginOption(
        inOption: KLDefaultLoginOption,
        ioBuffer: *mut ::std::os::raw::c_void,
        ioBufferSize: *mut KLSize,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLSetDefaultLoginOption(
        inOption: KLDefaultLoginOption,
        inBuffer: *const ::std::os::raw::c_void,
        inBufferSize: KLSize,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLFindKerberosRealmByName(
        inRealmName: *const ::std::os::raw::c_char,
        outIndex: *mut KLIndex,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetKerberosRealm(
        inIndex: KLIndex,
        outRealmName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLSetKerberosRealm(
        inIndex: KLIndex,
        inRealmName: *const ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLRemoveKerberosRealm(inIndex: KLIndex) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLInsertKerberosRealm(
        inInsertBeforeIndex: KLIndex,
        inRealmName: *const ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLRemoveAllKerberosRealms() -> KLStatus;
}
unsafe extern "C" {
    pub fn KLCountKerberosRealms() -> KLSize;
}
unsafe extern "C" {
    pub fn KLGetKerberosDefaultRealm(outIndex: *mut KLIndex) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetKerberosDefaultRealmByName(
        outRealmName: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLSetKerberosDefaultRealm(inIndex: KLIndex) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLSetKerberosDefaultRealmByName(inRealm: *const ::std::os::raw::c_char) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLCreatePrincipalFromTriplet(
        inName: *const ::std::os::raw::c_char,
        inInstance: *const ::std::os::raw::c_char,
        inRealm: *const ::std::os::raw::c_char,
        outPrincipal: *mut KLPrincipal,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLCreatePrincipalFromString(
        inFullPrincipal: *const ::std::os::raw::c_char,
        inKerberosVersion: KLKerberosVersion,
        outPrincipal: *mut KLPrincipal,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLCreatePrincipalFromKerberos5Principal(
        inKerberos5Principal: krb5_principal,
        outPrincipal: *mut KLPrincipal,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLCreatePrincipalFromPrincipal(
        inPrincipal: KLPrincipal,
        outPrincipal: *mut KLPrincipal,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetTripletFromPrincipal(
        inPrincipal: KLPrincipal,
        outName: *mut *mut ::std::os::raw::c_char,
        outInstance: *mut *mut ::std::os::raw::c_char,
        outRealm: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetStringFromPrincipal(
        inPrincipal: KLPrincipal,
        inKerberosVersion: KLKerberosVersion,
        outFullPrincipal: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLGetDisplayStringFromPrincipal(
        inPrincipal: KLPrincipal,
        inKerberosVersion: KLKerberosVersion,
        outFullPrincipal: *mut *mut ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLComparePrincipal(
        inFirstPrincipal: KLPrincipal,
        inSecondPrincipal: KLPrincipal,
        outAreEquivalent: *mut KLBoolean,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLDisposePrincipal(inPrincipal: KLPrincipal) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLCreateLoginOptions(outOptions: *mut KLLoginOptions) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLLoginOptionsSetTicketLifetime(
        ioOptions: KLLoginOptions,
        inTicketLifetime: KLLifetime,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLLoginOptionsSetForwardable(
        ioOptions: KLLoginOptions,
        inForwardable: KLBoolean,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLLoginOptionsSetProxiable(
        ioOptions: KLLoginOptions,
        inProxiable: KLBoolean,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLLoginOptionsSetRenewableLifetime(
        ioOptions: KLLoginOptions,
        inRenewableLifetime: KLLifetime,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLLoginOptionsSetAddressless(
        ioOptions: KLLoginOptions,
        inAddressless: KLBoolean,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLLoginOptionsSetTicketStartTime(
        ioOptions: KLLoginOptions,
        inStartTime: KLTime,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLLoginOptionsSetServiceName(
        ioOptions: KLLoginOptions,
        inServiceName: *const ::std::os::raw::c_char,
    ) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLDisposeLoginOptions(ioOptions: KLLoginOptions) -> KLStatus;
}
unsafe extern "C" {
    pub fn KLDisposeString(inStringToDispose: *mut ::std::os::raw::c_char) -> KLStatus;
}
unsafe extern "C" {
    pub fn cc_shutdown(io_context: *mut *mut apiCB) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_get_NC_info(in_context: *mut apiCB, out_info: *mut *mut *mut infoNC) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_get_change_time(in_context: *mut apiCB, out_change_time: *mut cc_time_t) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_open(
        in_context: *mut apiCB,
        in_name: *const ::std::os::raw::c_char,
        in_version: cc_int32,
        in_flags: cc_uint32,
        out_ccache: *mut *mut ccache_p,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_create(
        in_context: *mut apiCB,
        in_name: *const ::std::os::raw::c_char,
        in_principal: *const ::std::os::raw::c_char,
        in_version: cc_int32,
        in_flags: cc_uint32,
        out_ccache: *mut *mut ccache_p,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_close(in_context: *mut apiCB, ioCCache: *mut *mut ccache_p) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_destroy(in_context: *mut apiCB, io_ccache: *mut *mut ccache_p) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_seq_fetch_NCs_begin(
        in_context: *mut apiCB,
        out_nc_iterator: *mut *mut ccache_cit,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_seq_fetch_NCs_next(
        in_context: *mut apiCB,
        out_ccache: *mut *mut ccache_p,
        in_nc_iterator: *mut ccache_cit,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_seq_fetch_NCs_end(
        in_context: *mut apiCB,
        io_nc_iterator: *mut *mut ccache_cit,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_get_name(
        in_context: *mut apiCB,
        in_ccache: *mut ccache_p,
        out_name: *mut *mut ::std::os::raw::c_char,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_get_cred_version(
        in_context: *mut apiCB,
        in_ccache: *mut ccache_p,
        out_version: *mut cc_int32,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_set_principal(
        in_context: *mut apiCB,
        in_ccache: *mut ccache_p,
        in_version: cc_int32,
        in_principal: *mut ::std::os::raw::c_char,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_get_principal(
        in_context: *mut apiCB,
        in_ccache: *mut ccache_p,
        out_principal: *mut *mut ::std::os::raw::c_char,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_store(
        in_context: *mut apiCB,
        in_ccache: *mut ccache_p,
        in_credentials: cred_union,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_remove_cred(
        in_context: *mut apiCB,
        in_ccache: *mut ccache_p,
        in_credentials: cred_union,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_seq_fetch_creds_begin(
        in_context: *mut apiCB,
        in_ccache: *const ccache_p,
        out_ccache_iterator: *mut *mut ccache_cit,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_seq_fetch_creds_next(
        in_context: *mut apiCB,
        out_cred_union: *mut *mut cred_union,
        in_ccache_iterator: *mut ccache_cit,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_seq_fetch_creds_end(
        in_context: *mut apiCB,
        io_ccache_iterator: *mut *mut ccache_cit,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_free_principal(
        in_context: *mut apiCB,
        io_principal: *mut *mut ::std::os::raw::c_char,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_free_name(
        in_context: *mut apiCB,
        io_name: *mut *mut ::std::os::raw::c_char,
    ) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_free_creds(in_context: *mut apiCB, io_cred_union: *mut *mut cred_union) -> cc_int32;
}
unsafe extern "C" {
    pub fn cc_free_NC_info(in_context: *mut apiCB, io_info: *mut *mut *mut infoNC) -> cc_int32;
}

unsafe impl objc2::encode::RefEncode for cc_context_d {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_context_d {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_context_d", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_ccache_d {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_ccache_d {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_ccache_d", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_ccache_iterator_d {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_ccache_iterator_d {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_ccache_iterator_d", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_v4_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_v4_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_v4_t", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_data", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_v5_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_v5_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_v5_t", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_union {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_union {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_union", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_union__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_union__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_union__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_d {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_d {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_d", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_iterator_d {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_iterator_d {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_iterator_d", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_string_d {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_string_d {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_string_d", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_context_f {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_context_f {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_context_f", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_ccache_f {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_ccache_f {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_ccache_f", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_string_f {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_string_f {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_string_f", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_f {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_f {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_f", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_ccache_iterator_f {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_ccache_iterator_f {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_ccache_iterator_f", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_iterator_f {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_iterator_f {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_iterator_f", &[]);
}
unsafe impl objc2::encode::RefEncode for error_table {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for error_table {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("error_table", &[]);
}
unsafe impl objc2::encode::RefEncode for _profile_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _profile_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_profile_t", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_data", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_octet_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_octet_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_octet_data", &[]);
}
unsafe impl objc2::encode::RefEncode for krb5_principal_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for krb5_principal_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("krb5_principal_data", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_address {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_address {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_address", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_context {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_context {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_context", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_auth_context {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_auth_context {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_auth_context", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_keyblock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_keyblock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_keyblock", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_encrypt_block {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_encrypt_block {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_encrypt_block", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_checksum {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_checksum {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_checksum", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_enc_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_enc_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_enc_data", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_ticket_times {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_ticket_times {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_ticket_times", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_authdata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_authdata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_authdata", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_transited {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_transited {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_transited", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_enc_tkt_part {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_enc_tkt_part {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_enc_tkt_part", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_ticket {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_ticket {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_ticket", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_authenticator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_authenticator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_authenticator", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_tkt_authent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_tkt_authent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_tkt_authent", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_creds {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_creds {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_creds", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_last_req_entry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_last_req_entry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_last_req_entry", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_pa_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_pa_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_pa_data", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_kdc_req {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_kdc_req {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_kdc_req", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_enc_kdc_rep_part {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_enc_kdc_rep_part {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_enc_kdc_rep_part", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_kdc_rep {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_kdc_rep {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_kdc_rep", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_error {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_error {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_error", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_ap_req {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_ap_req {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_ap_req", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_ap_rep {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_ap_rep {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_ap_rep", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_ap_rep_enc_part {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_ap_rep_enc_part {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_ap_rep_enc_part", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_response {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_response {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_response", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_cred_info {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_cred_info {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_cred_info", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_cred_enc_part {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_cred_enc_part {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_cred_enc_part", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_cred {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_cred {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_cred", &[]);
}
unsafe impl objc2::encode::RefEncode for _passwd_phrase_element {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _passwd_phrase_element {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_passwd_phrase_element", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_pwd_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_pwd_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_pwd_data", &[]);
}
unsafe impl objc2::encode::RefEncode for krb5_replay_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for krb5_replay_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("krb5_replay_data", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_ccache {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_ccache {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_ccache", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_cc_ops {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_cc_ops {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_cc_ops", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_cccol_cursor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_cccol_cursor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_cccol_cursor", &[]);
}
unsafe impl objc2::encode::RefEncode for krb5_rc_st {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for krb5_rc_st {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("krb5_rc_st", &[]);
}
unsafe impl objc2::encode::RefEncode for krb5_keytab_entry_st {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for krb5_keytab_entry_st {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("krb5_keytab_entry_st", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_kt {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_kt {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_kt", &[]);
}
unsafe impl objc2::encode::RefEncode for credentials {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for credentials {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("credentials", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_prompt {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_prompt {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_prompt", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_get_init_creds_opt {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_get_init_creds_opt {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_get_init_creds_opt", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_gic_opt_pa_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_gic_opt_pa_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_gic_opt_pa_data", &[]);
}
unsafe impl objc2::encode::RefEncode for _krb5_verify_init_creds_opt {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _krb5_verify_init_creds_opt {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_krb5_verify_init_creds_opt", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_name_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_name_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_name_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_cred_id_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_cred_id_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_cred_id_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_ctx_id_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_ctx_id_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_ctx_id_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for apple_gss_krb5_authdata_if_relevant_key {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for apple_gss_krb5_authdata_if_relevant_key {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("apple_gss_krb5_authdata_if_relevant_key", &[]);
}
unsafe impl objc2::encode::RefEncode for kim_identity_opaque {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for kim_identity_opaque {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("kim_identity_opaque", &[]);
}
unsafe impl objc2::encode::RefEncode for kim_options_opaque {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for kim_options_opaque {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("kim_options_opaque", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_v5_compat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_v5_compat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_v5_compat", &[]);
}
unsafe impl objc2::encode::RefEncode for cc_credentials_v4_compat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cc_credentials_v4_compat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cc_credentials_v4_compat", &[]);
}
unsafe impl objc2::encode::RefEncode for cred_ptr_union_compat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cred_ptr_union_compat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cred_ptr_union_compat", &[]);
}
unsafe impl objc2::encode::RefEncode for cred_union {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cred_union {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cred_union", &[]);
}
unsafe impl objc2::encode::RefEncode for infoNC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for infoNC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("infoNC", &[]);
}
unsafe impl objc2::encode::RefEncode for ccache_cit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ccache_cit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ccache_cit", &[]);
}
unsafe impl objc2::encode::RefEncode for krb5plugin_service_locate_ftable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for krb5plugin_service_locate_ftable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("krb5plugin_service_locate_ftable", &[]);
}
