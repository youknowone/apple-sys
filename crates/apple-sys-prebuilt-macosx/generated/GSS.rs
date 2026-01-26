#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Kerberos::*;
#[allow(unused_imports)]
use libc::{id_t, key_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type OM_uint32 = u32;
pub type OM_uint64 = u64;
pub type gss_uint32 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_name_t_desc_struct {
    _unused: [u8; 0],
}
pub type gss_name_t = *mut gss_name_t_desc_struct;
pub type gss_const_name_t = *const gss_name_t_desc_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_ctx_id_t_desc_struct {
    _unused: [u8; 0],
}
pub type gss_ctx_id_t = *mut gss_ctx_id_t_desc_struct;
pub type gss_const_ctx_id_t = gss_ctx_id_t_desc_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_OID_desc_struct {
    pub length: OM_uint32,
    pub elements: *mut ::std::os::raw::c_void,
}
pub type gss_OID_desc = gss_OID_desc_struct;
pub type gss_const_OID = *const gss_OID_desc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_OID_set_desc_struct {
    pub count: usize,
    pub elements: gss_OID,
}
pub type gss_OID_set_desc = gss_OID_set_desc_struct;
pub type gss_OID_set = *mut gss_OID_set_desc_struct;
pub type gss_const_OID_set = *const gss_OID_set_desc;
pub type gss_cred_usage_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_cred_id_t_desc_struct {
    _unused: [u8; 0],
}
pub type gss_cred_id_t = *mut gss_cred_id_t_desc_struct;
pub type gss_const_cred_id_t = *const gss_cred_id_t_desc_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_buffer_desc_struct {
    pub length: usize,
    pub value: *mut ::std::os::raw::c_void,
}
pub type gss_buffer_desc = gss_buffer_desc_struct;
pub type gss_buffer_t = *mut gss_buffer_desc_struct;
pub type gss_const_buffer_t = *const gss_buffer_desc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_channel_bindings_struct {
    pub initiator_addrtype: OM_uint32,
    pub initiator_address: gss_buffer_desc,
    pub acceptor_addrtype: OM_uint32,
    pub acceptor_address: gss_buffer_desc,
    pub application_data: gss_buffer_desc,
}
pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
pub type gss_const_channel_bindings_t = *const gss_channel_bindings_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_buffer_set_desc_struct {
    pub count: usize,
    pub elements: *mut gss_buffer_desc,
}
pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_iov_buffer_desc_struct {
    pub type_: OM_uint32,
    pub buffer: gss_buffer_desc,
}
pub type gss_qop_t = OM_uint32;
pub type gss_status_id_t = *mut OM_uint32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_auth_identity {
    _unused: [u8; 0],
}
pub type gss_auth_identity_t = *mut gss_auth_identity;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct krb5_ccache_data {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_krb5_lucid_key {
    pub type_: OM_uint32,
    pub length: OM_uint32,
    pub data: *mut ::std::os::raw::c_void,
}
pub type gss_krb5_lucid_key_t = gss_krb5_lucid_key;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_krb5_rfc1964_keydata {
    pub sign_alg: OM_uint32,
    pub seal_alg: OM_uint32,
    pub ctx_key: gss_krb5_lucid_key_t,
}
pub type gss_krb5_rfc1964_keydata_t = gss_krb5_rfc1964_keydata;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_krb5_cfx_keydata {
    pub have_acceptor_subkey: OM_uint32,
    pub ctx_key: gss_krb5_lucid_key_t,
    pub acceptor_subkey: gss_krb5_lucid_key_t,
}
pub type gss_krb5_cfx_keydata_t = gss_krb5_cfx_keydata;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_krb5_lucid_context_v1 {
    pub version: OM_uint32,
    pub initiate: OM_uint32,
    pub endtime: OM_uint32,
    pub send_seq: OM_uint64,
    pub recv_seq: OM_uint64,
    pub protocol: OM_uint32,
    pub rfc1964_kd: gss_krb5_rfc1964_keydata_t,
    pub cfx_kd: gss_krb5_cfx_keydata_t,
}
pub type gss_krb5_lucid_context_v1_t = gss_krb5_lucid_context_v1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gss_krb5_lucid_context_version {
    pub version: OM_uint32,
}
pub type gss_krb5_lucid_context_version_t = gss_krb5_lucid_context_version;
unsafe extern "C" {
    pub static mut __gss_krb5_copy_ccache_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_get_tkt_flags_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_extract_authz_data_from_sec_context_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_compat_des3_mic_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_register_acceptor_identity_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_export_lucid_context_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_export_lucid_context_v1_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_set_dns_canonicalize_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_get_subkey_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_get_initiator_subkey_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_get_acceptor_subkey_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_send_to_kdc_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_get_authtime_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_get_service_keyblock_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_set_allowable_enctypes_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_set_default_realm_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_ccache_name_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_set_time_offset_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_get_time_offset_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_plugin_register_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_ntlm_get_session_key_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_ntlm_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_dn_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_nt_principal_name_referral_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ntlm_guest_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ntlm_v1_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ntlm_v2_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ntlm_session_key_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ntlm_force_v1_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_cred_no_ci_flags_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_uuid_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ntlm_support_channelbindings_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ntlm_support_lm2_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_import_cred_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ntlm_reset_keys_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_diag_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_validate_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_set_default_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_get_default_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_renew_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ctx_pfs_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_sasl_mech_name_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_mech_name_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_mech_description_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_password_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_certificate_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_secidentity_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_cred_heimbase_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_sasl_digest_md5_mechanism_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_netlogon_mechanism_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_appl_lkdc_supported_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_netlogon_set_session_key_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_netlogon_set_sign_algorithm_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_netlogon_nt_netbios_dns_name_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_inq_win2k_pac_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_inq_sspi_session_key_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_mechanism_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_ntlm_mechanism_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_iakerb_mechanism_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_pku2u_mechanism_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_spnego_mechanism_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_scram_mechanism_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_user_name_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_machine_uid_name_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_string_uid_name_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_hostbased_service_x_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_hostbased_service_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_anonymous_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_nt_export_name_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_nt_principal_name_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_krb5_nt_principal_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_peer_has_updated_spnego_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_mech_concrete_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_mech_pseudo_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_mech_composite_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_mech_nego_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_mech_glue_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_not_mech_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_deprecated_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_not_dflt_mech_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_itok_framed_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_auth_init_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_auth_targ_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_auth_init_init_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_auth_targ_init_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_auth_init_anon_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_auth_targ_anon_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_deleg_cred_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_integ_prot_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_conf_prot_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_mic_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_wrap_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_prot_ready_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_replay_det_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_oos_det_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_cbindings_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_pfs_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_compress_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub static mut __gss_c_ma_ctx_trans_oid_desc: gss_OID_desc;
}
unsafe extern "C" {
    pub fn gss_accept_sec_context(
        minor_status: *mut OM_uint32,
        context_handle: *mut gss_ctx_id_t,
        acceptor_cred_handle: gss_cred_id_t,
        input_token: gss_buffer_t,
        input_chan_bindings: gss_channel_bindings_t,
        src_name: *mut gss_name_t,
        mech_type: *mut gss_OID,
        output_token: gss_buffer_t,
        ret_flags: *mut OM_uint32,
        time_rec: *mut OM_uint32,
        delegated_cred_handle: *mut gss_cred_id_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_acquire_cred(
        minor_status: *mut OM_uint32,
        desired_name: gss_name_t,
        time_req: OM_uint32,
        desired_mechs: gss_OID_set,
        cred_usage: gss_cred_usage_t,
        output_cred_handle: *mut gss_cred_id_t,
        actual_mechs: *mut gss_OID_set,
        time_rec: *mut OM_uint32,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_acquire_cred_with_password(
        minor_status: *mut OM_uint32,
        desired_name: gss_name_t,
        password: gss_buffer_t,
        time_req: OM_uint32,
        desired_mechs: gss_OID_set,
        cred_usage: gss_cred_usage_t,
        output_cred_handle: *mut gss_cred_id_t,
        actual_mechs: *mut gss_OID_set,
        time_rec: *mut OM_uint32,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_add_buffer_set_member(
        minor_status: *mut OM_uint32,
        member_buffer: gss_buffer_t,
        buffer_set: *mut gss_buffer_set_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_add_cred(
        minor_status: *mut OM_uint32,
        input_cred_handle: gss_cred_id_t,
        desired_name: gss_name_t,
        desired_mech: gss_OID,
        cred_usage: gss_cred_usage_t,
        initiator_time_req: OM_uint32,
        acceptor_time_req: OM_uint32,
        output_cred_handle: *mut gss_cred_id_t,
        actual_mechs: *mut gss_OID_set,
        initiator_time_rec: *mut OM_uint32,
        acceptor_time_rec: *mut OM_uint32,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_add_oid_set_member(
        minor_status: *mut OM_uint32,
        member_oid: gss_const_OID,
        oid_set: *mut gss_OID_set,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_canonicalize_name(
        minor_status: *mut OM_uint32,
        input_name: gss_name_t,
        mech_type: gss_OID,
        output_name: *mut gss_name_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_compare_name(
        minor_status: *mut OM_uint32,
        name1_arg: gss_name_t,
        name2_arg: gss_name_t,
        name_equal: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_context_time(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        time_rec: *mut OM_uint32,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_create_empty_buffer_set(
        minor_status: *mut OM_uint32,
        buffer_set: *mut gss_buffer_set_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_create_empty_oid_set(
        minor_status: *mut OM_uint32,
        oid_set: *mut gss_OID_set,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_decapsulate_token(
        input_token: gss_const_buffer_t,
        oid: gss_const_OID,
        output_token: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_delete_sec_context(
        minor_status: *mut OM_uint32,
        context_handle: *mut gss_ctx_id_t,
        output_token: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_destroy_cred(min_stat: *mut OM_uint32, cred_handle: *mut gss_cred_id_t)
        -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_display_mech_attr(
        minor_status: *mut OM_uint32,
        mech_attr: gss_const_OID,
        name: gss_buffer_t,
        short_desc: gss_buffer_t,
        long_desc: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_display_name(
        minor_status: *mut OM_uint32,
        input_name: gss_name_t,
        output_name_buffer: gss_buffer_t,
        output_name_type: *mut gss_OID,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_display_status(
        minor_status: *mut OM_uint32,
        status_value: OM_uint32,
        status_type: ::std::os::raw::c_int,
        mech_type: gss_OID,
        message_content: *mut OM_uint32,
        status_string: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_duplicate_name(
        minor_status: *mut OM_uint32,
        src_name: gss_name_t,
        dest_name: *mut gss_name_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_duplicate_oid(
        minor_status: *mut OM_uint32,
        src_oid: gss_OID,
        dest_oid: *mut gss_OID,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_encapsulate_token(
        input_token: gss_const_buffer_t,
        oid: gss_const_OID,
        output_token: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_export_cred(
        minor_status: *mut OM_uint32,
        cred_handle: gss_cred_id_t,
        token: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_export_name(
        minor_status: *mut OM_uint32,
        input_name: gss_name_t,
        exported_name: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_export_sec_context(
        minor_status: *mut OM_uint32,
        context_handle: *mut gss_ctx_id_t,
        interprocess_token: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_get_mic(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        qop_req: gss_qop_t,
        message_buffer: gss_buffer_t,
        message_token: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_import_cred(
        minor_status: *mut OM_uint32,
        token: gss_buffer_t,
        cred_handle: *mut gss_cred_id_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_import_name(
        minor_status: *mut OM_uint32,
        input_name_buffer: gss_buffer_t,
        input_name_type: gss_const_OID,
        output_name: *mut gss_name_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_import_sec_context(
        minor_status: *mut OM_uint32,
        interprocess_token: gss_buffer_t,
        context_handle: *mut gss_ctx_id_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_indicate_mechs(
        minor_status: *mut OM_uint32,
        mech_set: *mut gss_OID_set,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_indicate_mechs_by_attrs(
        minor_status: *mut OM_uint32,
        desired_mech_attrs: gss_const_OID_set,
        except_mech_attrs: gss_const_OID_set,
        critical_mech_attrs: gss_const_OID_set,
        mechs: *mut gss_OID_set,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_init_sec_context(
        minor_status: *mut OM_uint32,
        initiator_cred_handle: gss_cred_id_t,
        context_handle: *mut gss_ctx_id_t,
        target_name: gss_name_t,
        input_mech_type: gss_OID,
        req_flags: OM_uint32,
        time_req: OM_uint32,
        input_chan_bindings: gss_channel_bindings_t,
        input_token: gss_buffer_t,
        actual_mech_type: *mut gss_OID,
        output_token: gss_buffer_t,
        ret_flags: *mut OM_uint32,
        time_rec: *mut OM_uint32,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_attrs_for_mech(
        minor_status: *mut OM_uint32,
        mech: gss_const_OID,
        mech_attr: *mut gss_OID_set,
        known_mech_attrs: *mut gss_OID_set,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_context(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        src_name: *mut gss_name_t,
        targ_name: *mut gss_name_t,
        lifetime_rec: *mut OM_uint32,
        mech_type: *mut gss_OID,
        ctx_flags: *mut OM_uint32,
        locally_initiated: *mut ::std::os::raw::c_int,
        xopen: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_cred(
        minor_status: *mut OM_uint32,
        cred_handle: gss_cred_id_t,
        name_ret: *mut gss_name_t,
        lifetime: *mut OM_uint32,
        cred_usage: *mut gss_cred_usage_t,
        mechanisms: *mut gss_OID_set,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_cred_by_mech(
        minor_status: *mut OM_uint32,
        cred_handle: gss_cred_id_t,
        mech_type: gss_OID,
        cred_name: *mut gss_name_t,
        initiator_lifetime: *mut OM_uint32,
        acceptor_lifetime: *mut OM_uint32,
        cred_usage: *mut gss_cred_usage_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_cred_by_oid(
        minor_status: *mut OM_uint32,
        cred_handle: gss_cred_id_t,
        desired_object: gss_OID,
        data_set: *mut gss_buffer_set_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_mech_for_saslname(
        minor_status: *mut OM_uint32,
        sasl_mech_name: gss_buffer_t,
        mech_type: *mut gss_OID,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_mechs_for_name(
        minor_status: *mut OM_uint32,
        input_name: gss_name_t,
        mech_types: *mut gss_OID_set,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_name(
        minor_status: *mut OM_uint32,
        input_name: gss_name_t,
        name_is_MN: *mut ::std::os::raw::c_int,
        MN_mech: *mut gss_OID,
        attrs: *mut gss_buffer_set_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_names_for_mech(
        minor_status: *mut OM_uint32,
        mechanism: gss_const_OID,
        name_types: *mut gss_OID_set,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_saslname_for_mech(
        minor_status: *mut OM_uint32,
        desired_mech: gss_OID,
        sasl_mech_name: gss_buffer_t,
        mech_name: gss_buffer_t,
        mech_description: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_inquire_sec_context_by_oid(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        desired_object: gss_OID,
        data_set: *mut gss_buffer_set_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_iter_creds(
        min_stat: *mut OM_uint32,
        flags: OM_uint32,
        mech: gss_const_OID,
        useriter: *mut ::std::os::raw::c_void,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_iter_creds_f(
        min_stat: *mut OM_uint32,
        flags: OM_uint32,
        mech: gss_const_OID,
        userctx: *mut ::std::os::raw::c_void,
        useriter: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: gss_OID,
                arg3: gss_cred_id_t,
            ),
        >,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_krb5_ccache_name(
        minor_status: *mut OM_uint32,
        name: *const ::std::os::raw::c_char,
        out_name: *mut *const ::std::os::raw::c_char,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_krb5_copy_ccache(
        minor_status: *mut OM_uint32,
        cred: gss_cred_id_t,
        out: *mut krb5_ccache_data,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_krb5_export_lucid_sec_context(
        minor_status: *mut OM_uint32,
        context_handle: *mut gss_ctx_id_t,
        version: OM_uint32,
        rctx: *mut *mut ::std::os::raw::c_void,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_krb5_free_lucid_sec_context(
        minor_status: *mut OM_uint32,
        c: *mut ::std::os::raw::c_void,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_krb5_set_allowable_enctypes(
        minor_status: *mut OM_uint32,
        cred: gss_cred_id_t,
        num_enctypes: OM_uint32,
        enctypes: *mut i32,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_oid_equal(a: gss_const_OID, b: gss_const_OID) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn gss_oid_to_str(
        minor_status: *mut OM_uint32,
        oid: gss_OID,
        oid_str: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_process_context_token(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        token_buffer: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_pseudo_random(
        minor_status: *mut OM_uint32,
        context: gss_ctx_id_t,
        prf_key: ::std::os::raw::c_int,
        prf_in: gss_buffer_t,
        desired_output_len: isize,
        prf_out: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_release_buffer(minor_status: *mut OM_uint32, buffer: gss_buffer_t) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_release_buffer_set(
        minor_status: *mut OM_uint32,
        buffer_set: *mut gss_buffer_set_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_release_cred(
        minor_status: *mut OM_uint32,
        cred_handle: *mut gss_cred_id_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_release_name(minor_status: *mut OM_uint32, input_name: *mut gss_name_t)
        -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_release_oid(minor_status: *mut OM_uint32, oid: *mut gss_OID) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_release_oid_set(minor_status: *mut OM_uint32, set: *mut gss_OID_set) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_seal(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        conf_req_flag: ::std::os::raw::c_int,
        qop_req: ::std::os::raw::c_int,
        input_message_buffer: gss_buffer_t,
        conf_state: *mut ::std::os::raw::c_int,
        output_message_buffer: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_set_cred_option(
        minor_status: *mut OM_uint32,
        cred_handle: *mut gss_cred_id_t,
        object: gss_OID,
        value: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_set_sec_context_option(
        minor_status: *mut OM_uint32,
        context_handle: *mut gss_ctx_id_t,
        object: gss_OID,
        value: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_sign(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        qop_req: ::std::os::raw::c_int,
        message_buffer: gss_buffer_t,
        message_token: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_test_oid_set_member(
        minor_status: *mut OM_uint32,
        member: gss_const_OID,
        set: gss_OID_set,
        present: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_unseal(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        input_message_buffer: gss_buffer_t,
        output_message_buffer: gss_buffer_t,
        conf_state: *mut ::std::os::raw::c_int,
        qop_state: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_unwrap(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        input_message_buffer: gss_buffer_t,
        output_message_buffer: gss_buffer_t,
        conf_state: *mut ::std::os::raw::c_int,
        qop_state: *mut gss_qop_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_userok(
        name: gss_name_t,
        user: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn gss_verify(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        message_buffer: gss_buffer_t,
        token_buffer: gss_buffer_t,
        qop_state: *mut ::std::os::raw::c_int,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_verify_mic(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        message_buffer: gss_buffer_t,
        token_buffer: gss_buffer_t,
        qop_state: *mut gss_qop_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_wrap(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        conf_req_flag: ::std::os::raw::c_int,
        qop_req: gss_qop_t,
        input_message_buffer: gss_buffer_t,
        conf_state: *mut ::std::os::raw::c_int,
        output_message_buffer: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_wrap_size_limit(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        conf_req_flag: ::std::os::raw::c_int,
        qop_req: gss_qop_t,
        req_output_size: OM_uint32,
        max_input_size: *mut OM_uint32,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gsskrb5_extract_authz_data_from_sec_context(
        minor_status: *mut OM_uint32,
        context_handle: gss_ctx_id_t,
        ad_type: ::std::os::raw::c_int,
        ad_data: gss_buffer_t,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gsskrb5_register_acceptor_identity(identity: *const ::std::os::raw::c_char)
        -> OM_uint32;
}
unsafe extern "C" {
    pub fn krb5_gss_register_acceptor_identity(
        identity: *const ::std::os::raw::c_char,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub static mut __gss_c_attr_local_login_user: gss_buffer_desc;
}
unsafe extern "C" {
    pub fn GSSCreateCredentialFromUUID(uuid: CFUUIDRef) -> gss_cred_id_t;
}
unsafe extern "C" {
    pub fn GSSCreateError(
        mech: gss_const_OID,
        major_status: OM_uint32,
        minor_status: OM_uint32,
    ) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn GSSCreateName(
        name: CFTypeRef,
        name_type: gss_const_OID,
        error: *mut CFErrorRef,
    ) -> gss_name_t;
}
unsafe extern "C" {
    pub fn GSSCredentialCopyName(cred: gss_cred_id_t) -> gss_name_t;
}
unsafe extern "C" {
    pub fn GSSCredentialCopyUUID(credential: gss_cred_id_t) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn GSSCredentialGetLifetime(cred: gss_cred_id_t) -> OM_uint32;
}
unsafe extern "C" {
    pub fn GSSNameCreateDisplayString(name: gss_name_t) -> CFStringRef;
}
unsafe extern "C" {
    pub fn gss_aapl_change_password(
        name: gss_name_t,
        mech: gss_const_OID,
        attributes: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> OM_uint32;
}
unsafe extern "C" {
    pub fn gss_aapl_initial_cred(
        desired_name: gss_name_t,
        desired_mech: gss_const_OID,
        attributes: CFDictionaryRef,
        output_cred_handle: *mut gss_cred_id_t,
        error: *mut CFErrorRef,
    ) -> OM_uint32;
}

unsafe impl objc2::encode::RefEncode for gss_name_t_desc_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_name_t_desc_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_name_t_desc_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_ctx_id_t_desc_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_ctx_id_t_desc_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_ctx_id_t_desc_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_OID_desc_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_OID_desc_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_OID_desc_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_OID_set_desc_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_OID_set_desc_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_OID_set_desc_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_cred_id_t_desc_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_cred_id_t_desc_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_cred_id_t_desc_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_buffer_desc_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_buffer_desc_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_buffer_desc_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_channel_bindings_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_channel_bindings_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_channel_bindings_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_buffer_set_desc_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_buffer_set_desc_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_buffer_set_desc_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_iov_buffer_desc_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_iov_buffer_desc_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_iov_buffer_desc_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_auth_identity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_auth_identity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_auth_identity", &[]);
}
unsafe impl objc2::encode::RefEncode for krb5_ccache_data {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for krb5_ccache_data {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("krb5_ccache_data", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_krb5_lucid_key {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_krb5_lucid_key {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_krb5_lucid_key", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_krb5_rfc1964_keydata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_krb5_rfc1964_keydata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_krb5_rfc1964_keydata", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_krb5_cfx_keydata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_krb5_cfx_keydata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_krb5_cfx_keydata", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_krb5_lucid_context_v1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_krb5_lucid_context_v1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_krb5_lucid_context_v1", &[]);
}
unsafe impl objc2::encode::RefEncode for gss_krb5_lucid_context_version {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gss_krb5_lucid_context_version {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gss_krb5_lucid_context_version", &[]);
}
