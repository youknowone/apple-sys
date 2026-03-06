#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use libc::{key_t, sa_family_t};

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_len: u8,
    pub sa_family: sa_family_t,
    pub sa_data: [::std::os::raw::c_char; 14usize],
}
pub type sec_protocol_metadata_t = NSObject;
pub type sec_protocol_options_t = NSObject;
pub type nw_txt_record_t = NSObject;
pub type nw_txt_record_find_key_t = ::std::os::raw::c_uint;
pub type nw_txt_record_access_key_t = *mut ::std::os::raw::c_void;
pub type nw_txt_record_access_bytes_t = *mut ::std::os::raw::c_void;
pub type nw_txt_record_applier_t = *mut ::std::os::raw::c_void;
pub type nw_advertise_descriptor_t = NSObject;
pub type nw_protocol_definition_t = NSObject;
pub type nw_protocol_options_t = NSObject;
pub type nw_protocol_metadata_t = NSObject;
pub type nw_interface_t = NSObject;
pub type nw_interface_type_t = ::std::os::raw::c_uint;
pub type nw_interface_radio_type_t = ::std::os::raw::c_uint;
pub type nw_endpoint_t = NSObject;
pub type nw_endpoint_type_t = ::std::os::raw::c_uint;
pub type nw_resolver_config_t = NSObject;
pub type nw_proxy_config_t = NSObject;
pub type nw_relay_hop_t = NSObject;
pub type nw_proxy_domain_enumerator_t = *mut ::std::os::raw::c_void;
pub type nw_privacy_context_t = NSObject;
pub type nw_parameters_t = NSObject;
pub type nw_protocol_stack_t = NSObject;
pub type nw_parameters_configure_protocol_block_t = *mut ::std::os::raw::c_void;
pub type nw_parameters_attribution_t = u8;
pub type nw_parameters_iterate_interfaces_block_t = *mut ::std::os::raw::c_void;
pub type nw_parameters_iterate_interface_types_block_t = *mut ::std::os::raw::c_void;
pub type nw_service_class_t = ::std::os::raw::c_uint;
pub type nw_multipath_service_t = ::std::os::raw::c_uint;
pub type nw_protocol_stack_iterate_protocols_block_t = *mut ::std::os::raw::c_void;
pub type nw_parameters_expired_dns_behavior_t = ::std::os::raw::c_uint;
pub type nw_browse_descriptor_t = NSObject;
pub type nw_browse_result_change_t = u64;
pub type nw_browse_result_enumerate_interface_t = *mut ::std::os::raw::c_void;
pub type nw_error_domain_t = ::std::os::raw::c_uint;
pub type nw_browser_t = NSObject;
pub type nw_browser_state_t = ::std::os::raw::c_uint;
pub type nw_browser_browse_results_changed_handler_t = *mut ::std::os::raw::c_void;
pub type nw_browser_state_changed_handler_t = *mut ::std::os::raw::c_void;
pub type nw_path_t = NSObject;
pub type nw_path_status_t = ::std::os::raw::c_uint;
pub type nw_path_unsatisfied_reason_t = ::std::os::raw::c_uint;
pub type nw_path_enumerate_interfaces_block_t = *mut ::std::os::raw::c_void;
pub type nw_path_enumerate_gateways_block_t = *mut ::std::os::raw::c_void;
pub type nw_link_quality_t = ::std::os::raw::c_uint;
pub type nw_content_context_t = NSObject;
pub type nw_connection_t = NSObject;
pub type nw_connection_state_t = ::std::os::raw::c_uint;
pub type nw_connection_state_changed_handler_t = *mut ::std::os::raw::c_void;
pub type nw_connection_boolean_event_handler_t = *mut ::std::os::raw::c_void;
pub type nw_connection_path_event_handler_t = *mut ::std::os::raw::c_void;
pub type nw_connection_receive_completion_t = *mut ::std::os::raw::c_void;
pub type nw_connection_send_completion_t = *mut ::std::os::raw::c_void;
pub type nw_group_descriptor_t = NSObject;
pub type nw_group_descriptor_enumerate_endpoints_block_t = *mut ::std::os::raw::c_void;
pub type nw_connection_group_t = NSObject;
pub type nw_connection_group_state_t = ::std::os::raw::c_uint;
pub type nw_connection_group_state_changed_handler_t = *mut ::std::os::raw::c_void;
pub type nw_connection_group_receive_handler_t = *mut ::std::os::raw::c_void;
pub type nw_connection_group_send_completion_t = *mut ::std::os::raw::c_void;
pub type nw_connection_group_new_connection_handler_t = *mut ::std::os::raw::c_void;
pub type nw_establishment_report_access_block_t = *mut ::std::os::raw::c_void;
pub type nw_report_resolution_source_t = ::std::os::raw::c_uint;
pub type nw_report_resolution_protocol_t = ::std::os::raw::c_uint;
pub type nw_report_resolution_enumerator_t = *mut ::std::os::raw::c_void;
pub type nw_report_resolution_report_enumerator_t = *mut ::std::os::raw::c_void;
pub type nw_report_protocol_enumerator_t = *mut ::std::os::raw::c_void;
pub type nw_data_transfer_report_t = NSObject;
pub type nw_data_transfer_report_state_t = ::std::os::raw::c_uint;
pub type nw_data_transfer_report_collect_block_t = *mut ::std::os::raw::c_void;
pub type nw_ethernet_channel_t = NSObject;
pub type nw_ethernet_channel_state_t = ::std::os::raw::c_uint;
pub type nw_ethernet_channel_state_changed_handler_t = *mut ::std::os::raw::c_void;
pub type nw_ethernet_channel_receive_handler_t = *mut ::std::os::raw::c_void;
pub type nw_ethernet_channel_send_completion_t = *mut ::std::os::raw::c_void;
pub type nw_framer_message_t = nw_protocol_metadata_t;
pub type nw_framer_message_dispose_value_t = *mut ::std::os::raw::c_void;
pub type nw_framer_start_result_t = ::std::os::raw::c_uint;
pub type nw_framer_start_handler_t = *mut ::std::os::raw::c_void;
pub type nw_framer_input_handler_t = *mut ::std::os::raw::c_void;
pub type nw_framer_output_handler_t = *mut ::std::os::raw::c_void;
pub type nw_framer_wakeup_handler_t = *mut ::std::os::raw::c_void;
pub type nw_framer_stop_handler_t = *mut ::std::os::raw::c_void;
pub type nw_framer_cleanup_handler_t = *mut ::std::os::raw::c_void;
pub type nw_framer_parse_completion_t = *mut ::std::os::raw::c_void;
pub type nw_framer_block_t = *mut ::std::os::raw::c_void;
pub type nw_ip_version_t = ::std::os::raw::c_uint;
pub type nw_ip_local_address_preference_t = ::std::os::raw::c_uint;
pub type nw_ip_ecn_flag_t = ::std::os::raw::c_uint;
pub type nw_listener_t = NSObject;
pub type nw_listener_state_t = ::std::os::raw::c_uint;
pub type nw_listener_state_changed_handler_t = *mut ::std::os::raw::c_void;
pub type nw_listener_new_connection_handler_t = *mut ::std::os::raw::c_void;
pub type nw_listener_new_connection_group_handler_t = *mut ::std::os::raw::c_void;
pub type nw_listener_advertised_endpoint_changed_handler_t = *mut ::std::os::raw::c_void;
pub type nw_path_monitor_t = NSObject;
pub type nw_path_monitor_cancel_handler_t = *mut ::std::os::raw::c_void;
pub type nw_path_monitor_update_handler_t = *mut ::std::os::raw::c_void;
pub type nw_quic_stream_type_t = ::std::os::raw::c_uint;
pub type nw_multipath_version_t = ::std::os::raw::c_int;
pub type nw_ws_opcode_t = ::std::os::raw::c_int;
pub type nw_ws_close_code_t = ::std::os::raw::c_uint;
pub type nw_ws_version_t = ::std::os::raw::c_uint;
pub type nw_ws_pong_handler_t = *mut ::std::os::raw::c_void;
pub type nw_ws_subprotocol_enumerator_t = *mut ::std::os::raw::c_void;
pub type nw_ws_additional_header_enumerator_t = *mut ::std::os::raw::c_void;
pub type nw_ws_response_t = NSObject;
pub type nw_ws_response_status_t = ::std::os::raw::c_uint;
pub type nw_ws_client_request_handler_t = *mut ::std::os::raw::c_void;
pub trait NSURLSessionConfiguration_Network: Sized + std::ops::Deref {
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
}
unsafe extern "C" {
    pub fn nw_retain(obj: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn nw_release(obj: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn nw_txt_record_create_with_bytes(txt_bytes: *const u8, txt_len: usize)
        -> nw_txt_record_t;
}
unsafe extern "C" {
    pub fn nw_txt_record_create_dictionary() -> nw_txt_record_t;
}
unsafe extern "C" {
    pub fn nw_txt_record_copy(txt_record: NSObject) -> nw_txt_record_t;
}
unsafe extern "C" {
    pub fn nw_txt_record_find_key(
        txt_record: NSObject,
        key: *const ::std::os::raw::c_char,
    ) -> nw_txt_record_find_key_t;
}
unsafe extern "C" {
    pub fn nw_txt_record_access_key(
        txt_record: NSObject,
        key: *const ::std::os::raw::c_char,
        access_value: nw_txt_record_access_key_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_txt_record_set_key(
        txt_record: NSObject,
        key: *const ::std::os::raw::c_char,
        value: *const u8,
        value_len: usize,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_txt_record_remove_key(
        txt_record: NSObject,
        key: *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_txt_record_get_key_count(txt_record: NSObject) -> usize;
}
unsafe extern "C" {
    pub fn nw_txt_record_access_bytes(
        txt_record: NSObject,
        access_bytes: nw_txt_record_access_bytes_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_txt_record_apply(txt_record: NSObject, applier: nw_txt_record_applier_t) -> bool;
}
unsafe extern "C" {
    pub fn nw_txt_record_is_equal(left: NSObject, right: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_txt_record_is_dictionary(txt_record: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_advertise_descriptor_create_bonjour_service(
        name: *const ::std::os::raw::c_char,
        type_: *const ::std::os::raw::c_char,
        domain: *const ::std::os::raw::c_char,
    ) -> nw_advertise_descriptor_t;
}
unsafe extern "C" {
    pub fn nw_advertise_descriptor_set_txt_record(
        advertise_descriptor: NSObject,
        txt_record: *const ::std::os::raw::c_void,
        txt_length: usize,
    );
}
unsafe extern "C" {
    pub fn nw_advertise_descriptor_set_no_auto_rename(
        advertise_descriptor: NSObject,
        no_auto_rename: bool,
    );
}
unsafe extern "C" {
    pub fn nw_advertise_descriptor_get_no_auto_rename(advertise_descriptor: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_advertise_descriptor_set_txt_record_object(
        advertise_descriptor: NSObject,
        txt_record: NSObject,
    );
}
unsafe extern "C" {
    pub fn nw_advertise_descriptor_copy_txt_record_object(
        advertise_descriptor: NSObject,
    ) -> nw_txt_record_t;
}
unsafe extern "C" {
    pub fn nw_advertise_descriptor_create_application_service(
        application_service_name: *const ::std::os::raw::c_char,
    ) -> nw_advertise_descriptor_t;
}
unsafe extern "C" {
    pub fn nw_advertise_descriptor_get_application_service_name(
        advertise_descriptor: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_protocol_definition_is_equal(definition1: NSObject, definition2: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_protocol_options_copy_definition(options: NSObject) -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_protocol_metadata_copy_definition(metadata: NSObject) -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_interface_get_type(interface: NSObject) -> nw_interface_type_t;
}
unsafe extern "C" {
    pub fn nw_interface_get_name(interface: NSObject) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_interface_get_index(interface: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_type(endpoint: NSObject) -> nw_endpoint_type_t;
}
unsafe extern "C" {
    pub fn nw_endpoint_create_host(
        hostname: *const ::std::os::raw::c_char,
        port: *const ::std::os::raw::c_char,
    ) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_hostname(endpoint: NSObject) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_endpoint_copy_port_string(endpoint: NSObject) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_port(endpoint: NSObject) -> u16;
}
unsafe extern "C" {
    pub fn nw_endpoint_create_address(address: *const sockaddr) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_endpoint_copy_address_string(endpoint: NSObject) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_address(endpoint: NSObject) -> *const sockaddr;
}
unsafe extern "C" {
    pub fn nw_endpoint_create_bonjour_service(
        name: *const ::std::os::raw::c_char,
        type_: *const ::std::os::raw::c_char,
        domain: *const ::std::os::raw::c_char,
    ) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_bonjour_service_name(
        endpoint: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_bonjour_service_type(
        endpoint: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_bonjour_service_domain(
        endpoint: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_endpoint_create_url(url: *const ::std::os::raw::c_char) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_url(endpoint: NSObject) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_endpoint_copy_txt_record(endpoint: NSObject) -> nw_txt_record_t;
}
unsafe extern "C" {
    pub fn nw_endpoint_get_signature(
        endpoint: NSObject,
        out_signature_length: *mut usize,
    ) -> *const u8;
}
unsafe extern "C" {
    pub fn nw_resolver_config_create_https(url_endpoint: NSObject) -> nw_resolver_config_t;
}
unsafe extern "C" {
    pub fn nw_resolver_config_create_tls(server_endpoint: NSObject) -> nw_resolver_config_t;
}
unsafe extern "C" {
    pub fn nw_resolver_config_add_server_address(config: NSObject, server_address: NSObject);
}
unsafe extern "C" {
    pub fn nw_relay_hop_create(
        http3_relay_endpoint: NSObject,
        http2_relay_endpoint: NSObject,
        relay_tls_options: NSObject,
    ) -> nw_relay_hop_t;
}
unsafe extern "C" {
    pub fn nw_relay_hop_add_additional_http_header_field(
        relay_hop: NSObject,
        field_name: *const ::std::os::raw::c_char,
        field_value: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_proxy_config_create_relay(
        first_hop: NSObject,
        second_hop: NSObject,
    ) -> nw_proxy_config_t;
}
unsafe extern "C" {
    pub fn nw_proxy_config_create_oblivious_http(
        relay: NSObject,
        relay_resource_path: *const ::std::os::raw::c_char,
        gateway_key_config: *const u8,
        gateway_key_config_length: usize,
    ) -> nw_proxy_config_t;
}
unsafe extern "C" {
    pub fn nw_proxy_config_create_http_connect(
        proxy_endpoint: NSObject,
        proxy_tls_options: NSObject,
    ) -> nw_proxy_config_t;
}
unsafe extern "C" {
    pub fn nw_proxy_config_create_socksv5(proxy_endpoint: NSObject) -> nw_proxy_config_t;
}
unsafe extern "C" {
    pub fn nw_proxy_config_set_username_and_password(
        proxy_config: NSObject,
        username: *const ::std::os::raw::c_char,
        password: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_proxy_config_set_failover_allowed(proxy_config: NSObject, failover_allowed: bool);
}
unsafe extern "C" {
    pub fn nw_proxy_config_get_failover_allowed(proxy_config: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_proxy_config_add_match_domain(
        config: NSObject,
        match_domain: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_proxy_config_clear_match_domains(config: NSObject);
}
unsafe extern "C" {
    pub fn nw_proxy_config_add_excluded_domain(
        config: NSObject,
        excluded_domain: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_proxy_config_clear_excluded_domains(config: NSObject);
}
unsafe extern "C" {
    pub fn nw_proxy_config_enumerate_match_domains(
        config: NSObject,
        enumerator: nw_proxy_domain_enumerator_t,
    );
}
unsafe extern "C" {
    pub fn nw_proxy_config_enumerate_excluded_domains(
        config: NSObject,
        enumerator: nw_proxy_domain_enumerator_t,
    );
}
unsafe extern "C" {
    pub fn nw_privacy_context_create(
        description: *const ::std::os::raw::c_char,
    ) -> nw_privacy_context_t;
}
unsafe extern "C" {
    pub static _nw_privacy_context_default_context: nw_privacy_context_t;
}
unsafe extern "C" {
    pub fn nw_privacy_context_flush_cache(privacy_context: NSObject);
}
unsafe extern "C" {
    pub fn nw_privacy_context_disable_logging(privacy_context: NSObject);
}
unsafe extern "C" {
    pub fn nw_privacy_context_require_encrypted_name_resolution(
        privacy_context: NSObject,
        require_encrypted_name_resolution: bool,
        fallback_resolver_config: NSObject,
    );
}
unsafe extern "C" {
    pub fn nw_privacy_context_add_proxy(privacy_context: NSObject, proxy_config: NSObject);
}
unsafe extern "C" {
    pub fn nw_privacy_context_clear_proxies(privacy_context: NSObject);
}
unsafe extern "C" {
    pub static _nw_parameters_configure_protocol_default_configuration:
        nw_parameters_configure_protocol_block_t;
}
unsafe extern "C" {
    pub static _nw_parameters_configure_protocol_disable: nw_parameters_configure_protocol_block_t;
}
unsafe extern "C" {
    pub fn nw_parameters_create_secure_tcp(
        configure_tls: nw_parameters_configure_protocol_block_t,
        configure_tcp: nw_parameters_configure_protocol_block_t,
    ) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_parameters_create_secure_udp(
        configure_dtls: nw_parameters_configure_protocol_block_t,
        configure_udp: nw_parameters_configure_protocol_block_t,
    ) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_parameters_create_custom_ip(
        custom_ip_protocol_number: u8,
        configure_ip: nw_parameters_configure_protocol_block_t,
    ) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_parameters_create_quic(
        configure_quic: nw_parameters_configure_protocol_block_t,
    ) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_parameters_create_application_service() -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_parameters_create() -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_parameters_copy(parameters: NSObject) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_parameters_set_privacy_context(parameters: NSObject, privacy_context: NSObject);
}
unsafe extern "C" {
    pub fn nw_parameters_set_attribution(
        parameters: NSObject,
        attribution: nw_parameters_attribution_t,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_get_attribution(parameters: NSObject) -> nw_parameters_attribution_t;
}
unsafe extern "C" {
    pub fn nw_parameters_require_interface(parameters: NSObject, interface: NSObject);
}
unsafe extern "C" {
    pub fn nw_parameters_copy_required_interface(parameters: NSObject) -> nw_interface_t;
}
unsafe extern "C" {
    pub fn nw_parameters_prohibit_interface(parameters: NSObject, interface: NSObject);
}
unsafe extern "C" {
    pub fn nw_parameters_clear_prohibited_interfaces(parameters: NSObject);
}
unsafe extern "C" {
    pub fn nw_parameters_iterate_prohibited_interfaces(
        parameters: NSObject,
        iterate_block: nw_parameters_iterate_interfaces_block_t,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_set_required_interface_type(
        parameters: NSObject,
        interface_type: nw_interface_type_t,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_get_required_interface_type(parameters: NSObject) -> nw_interface_type_t;
}
unsafe extern "C" {
    pub fn nw_parameters_prohibit_interface_type(
        parameters: NSObject,
        interface_type: nw_interface_type_t,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_clear_prohibited_interface_types(parameters: NSObject);
}
unsafe extern "C" {
    pub fn nw_parameters_iterate_prohibited_interface_types(
        parameters: NSObject,
        iterate_block: nw_parameters_iterate_interface_types_block_t,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_set_prohibit_expensive(parameters: NSObject, prohibit_expensive: bool);
}
unsafe extern "C" {
    pub fn nw_parameters_get_prohibit_expensive(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_parameters_set_prohibit_constrained(parameters: NSObject, prohibit_constrained: bool);
}
unsafe extern "C" {
    pub fn nw_parameters_get_prohibit_constrained(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_parameters_set_allow_ultra_constrained(
        parameters: NSObject,
        allow_ultra_constrained: bool,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_get_allow_ultra_constrained(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_parameters_set_reuse_local_address(parameters: NSObject, reuse_local_address: bool);
}
unsafe extern "C" {
    pub fn nw_parameters_get_reuse_local_address(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_parameters_set_local_endpoint(parameters: NSObject, local_endpoint: NSObject);
}
unsafe extern "C" {
    pub fn nw_parameters_copy_local_endpoint(parameters: NSObject) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_parameters_set_include_peer_to_peer(parameters: NSObject, include_peer_to_peer: bool);
}
unsafe extern "C" {
    pub fn nw_parameters_get_include_peer_to_peer(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_parameters_set_fast_open_enabled(parameters: NSObject, fast_open_enabled: bool);
}
unsafe extern "C" {
    pub fn nw_parameters_get_fast_open_enabled(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_parameters_set_service_class(parameters: NSObject, service_class: nw_service_class_t);
}
unsafe extern "C" {
    pub fn nw_parameters_get_service_class(parameters: NSObject) -> nw_service_class_t;
}
unsafe extern "C" {
    pub fn nw_parameters_set_multipath_service(
        parameters: NSObject,
        multipath_service: nw_multipath_service_t,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_get_multipath_service(parameters: NSObject) -> nw_multipath_service_t;
}
unsafe extern "C" {
    pub fn nw_parameters_copy_default_protocol_stack(parameters: NSObject) -> nw_protocol_stack_t;
}
unsafe extern "C" {
    pub fn nw_protocol_stack_prepend_application_protocol(stack: NSObject, protocol: NSObject);
}
unsafe extern "C" {
    pub fn nw_protocol_stack_clear_application_protocols(stack: NSObject);
}
unsafe extern "C" {
    pub fn nw_protocol_stack_iterate_application_protocols(
        stack: NSObject,
        iterate_block: nw_protocol_stack_iterate_protocols_block_t,
    );
}
unsafe extern "C" {
    pub fn nw_protocol_stack_copy_transport_protocol(stack: NSObject) -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_protocol_stack_set_transport_protocol(stack: NSObject, protocol: NSObject);
}
unsafe extern "C" {
    pub fn nw_protocol_stack_copy_internet_protocol(stack: NSObject) -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_parameters_set_local_only(parameters: NSObject, local_only: bool);
}
unsafe extern "C" {
    pub fn nw_parameters_get_local_only(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_parameters_set_prefer_no_proxy(parameters: NSObject, prefer_no_proxy: bool);
}
unsafe extern "C" {
    pub fn nw_parameters_get_prefer_no_proxy(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_parameters_set_expired_dns_behavior(
        parameters: NSObject,
        expired_dns_behavior: nw_parameters_expired_dns_behavior_t,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_get_expired_dns_behavior(
        parameters: NSObject,
    ) -> nw_parameters_expired_dns_behavior_t;
}
unsafe extern "C" {
    pub fn nw_parameters_set_requires_dnssec_validation(
        parameters: NSObject,
        requires_dnssec_validation: bool,
    );
}
unsafe extern "C" {
    pub fn nw_parameters_requires_dnssec_validation(parameters: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_browse_descriptor_create_bonjour_service(
        type_: *const ::std::os::raw::c_char,
        domain: *const ::std::os::raw::c_char,
    ) -> nw_browse_descriptor_t;
}
unsafe extern "C" {
    pub fn nw_browse_descriptor_get_bonjour_service_type(
        descriptor: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_browse_descriptor_get_bonjour_service_domain(
        descriptor: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_browse_descriptor_set_include_txt_record(
        descriptor: NSObject,
        include_txt_record: bool,
    );
}
unsafe extern "C" {
    pub fn nw_browse_descriptor_get_include_txt_record(descriptor: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_browse_descriptor_create_application_service(
        application_service_name: *const ::std::os::raw::c_char,
    ) -> nw_browse_descriptor_t;
}
unsafe extern "C" {
    pub fn nw_browse_descriptor_get_application_service_name(
        descriptor: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_browse_result_copy_endpoint(result: NSObject) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_browse_result_get_changes(
        old_result: NSObject,
        new_result: NSObject,
    ) -> nw_browse_result_change_t;
}
unsafe extern "C" {
    pub fn nw_browse_result_get_interfaces_count(result: NSObject) -> usize;
}
unsafe extern "C" {
    pub fn nw_browse_result_copy_txt_record_object(result: NSObject) -> nw_txt_record_t;
}
unsafe extern "C" {
    pub fn nw_browse_result_enumerate_interfaces(
        result: NSObject,
        enumerator: nw_browse_result_enumerate_interface_t,
    );
}
unsafe extern "C" {
    pub fn nw_error_get_error_domain(error: NSObject) -> nw_error_domain_t;
}
unsafe extern "C" {
    pub fn nw_error_get_error_code(error: NSObject) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static kNWErrorDomainPOSIX: CFStringRef;
}
unsafe extern "C" {
    pub static kNWErrorDomainDNS: CFStringRef;
}
unsafe extern "C" {
    pub static kNWErrorDomainTLS: CFStringRef;
}
unsafe extern "C" {
    pub static kNWErrorDomainWiFiAware: CFStringRef;
}
unsafe extern "C" {
    pub fn nw_error_copy_cf_error(error: NSObject) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn nw_browser_create(descriptor: NSObject, parameters: NSObject) -> nw_browser_t;
}
unsafe extern "C" {
    pub fn nw_browser_set_queue(browser: NSObject, queue: NSObject);
}
unsafe extern "C" {
    pub fn nw_browser_set_browse_results_changed_handler(
        browser: NSObject,
        handler: nw_browser_browse_results_changed_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_browser_set_state_changed_handler(
        browser: NSObject,
        state_changed_handler: nw_browser_state_changed_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_browser_start(browser: NSObject);
}
unsafe extern "C" {
    pub fn nw_browser_cancel(browser: NSObject);
}
unsafe extern "C" {
    pub fn nw_browser_copy_parameters(browser: NSObject) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_browser_copy_browse_descriptor(browser: NSObject) -> nw_browse_descriptor_t;
}
unsafe extern "C" {
    pub fn nw_path_get_status(path: NSObject) -> nw_path_status_t;
}
unsafe extern "C" {
    pub fn nw_path_get_unsatisfied_reason(path: NSObject) -> nw_path_unsatisfied_reason_t;
}
unsafe extern "C" {
    pub fn nw_path_enumerate_interfaces(
        path: NSObject,
        enumerate_block: nw_path_enumerate_interfaces_block_t,
    );
}
unsafe extern "C" {
    pub fn nw_path_is_equal(path: NSObject, other_path: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_path_is_expensive(path: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_path_is_constrained(path: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_path_is_ultra_constrained(path: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_path_has_ipv4(path: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_path_has_ipv6(path: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_path_has_dns(path: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_path_uses_interface_type(path: NSObject, interface_type: nw_interface_type_t)
        -> bool;
}
unsafe extern "C" {
    pub fn nw_path_copy_effective_local_endpoint(path: NSObject) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_path_copy_effective_remote_endpoint(path: NSObject) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_path_enumerate_gateways(
        path: NSObject,
        enumerate_block: nw_path_enumerate_gateways_block_t,
    );
}
unsafe extern "C" {
    pub fn nw_path_get_link_quality(path: NSObject) -> nw_link_quality_t;
}
unsafe extern "C" {
    pub fn nw_content_context_create(
        context_identifier: *const ::std::os::raw::c_char,
    ) -> nw_content_context_t;
}
unsafe extern "C" {
    pub fn nw_content_context_get_identifier(context: NSObject) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_content_context_get_is_final(context: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_content_context_set_is_final(context: NSObject, is_final: bool);
}
unsafe extern "C" {
    pub fn nw_content_context_get_expiration_milliseconds(context: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_content_context_set_expiration_milliseconds(
        context: NSObject,
        expiration_milliseconds: u64,
    );
}
unsafe extern "C" {
    pub fn nw_content_context_get_relative_priority(context: NSObject) -> f64;
}
unsafe extern "C" {
    pub fn nw_content_context_set_relative_priority(context: NSObject, relative_priority: f64);
}
unsafe extern "C" {
    pub fn nw_content_context_set_antecedent(context: NSObject, antecedent_context: NSObject);
}
unsafe extern "C" {
    pub fn nw_content_context_copy_antecedent(context: NSObject) -> nw_content_context_t;
}
unsafe extern "C" {
    pub fn nw_content_context_set_metadata_for_protocol(
        context: NSObject,
        protocol_metadata: NSObject,
    );
}
unsafe extern "C" {
    pub fn nw_content_context_copy_protocol_metadata(
        context: NSObject,
        protocol: NSObject,
    ) -> nw_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_content_context_foreach_protocol_metadata(
        context: NSObject,
        foreach_block: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn nw_connection_create(endpoint: NSObject, parameters: NSObject) -> nw_connection_t;
}
unsafe extern "C" {
    pub fn nw_connection_copy_endpoint(connection: NSObject) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_connection_copy_parameters(connection: NSObject) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_connection_set_state_changed_handler(
        connection: NSObject,
        handler: nw_connection_state_changed_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_set_viability_changed_handler(
        connection: NSObject,
        handler: nw_connection_boolean_event_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_set_better_path_available_handler(
        connection: NSObject,
        handler: nw_connection_boolean_event_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_set_path_changed_handler(
        connection: NSObject,
        handler: nw_connection_path_event_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_set_queue(connection: NSObject, queue: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_start(connection: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_restart(connection: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_cancel(connection: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_force_cancel(connection: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_cancel_current_endpoint(connection: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_receive(
        connection: NSObject,
        minimum_incomplete_length: u32,
        maximum_length: u32,
        completion: nw_connection_receive_completion_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_receive_message(
        connection: NSObject,
        completion: nw_connection_receive_completion_t,
    );
}
unsafe extern "C" {
    pub static _nw_connection_send_idempotent_content: nw_connection_send_completion_t;
}
unsafe extern "C" {
    pub static _nw_content_context_default_message: nw_content_context_t;
}
unsafe extern "C" {
    pub static _nw_content_context_final_send: nw_content_context_t;
}
unsafe extern "C" {
    pub static _nw_content_context_default_stream: nw_content_context_t;
}
unsafe extern "C" {
    pub fn nw_connection_send(
        connection: NSObject,
        content: NSObject,
        context: NSObject,
        is_complete: bool,
        completion: nw_connection_send_completion_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_batch(connection: NSObject, batch_block: dispatch_block_t);
}
unsafe extern "C" {
    pub fn nw_connection_copy_description(connection: NSObject) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_connection_copy_current_path(connection: NSObject) -> nw_path_t;
}
unsafe extern "C" {
    pub fn nw_connection_copy_protocol_metadata(
        connection: NSObject,
        definition: NSObject,
    ) -> nw_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_connection_get_maximum_datagram_size(connection: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_group_descriptor_create_multiplex(remote_endpoint: NSObject)
        -> nw_group_descriptor_t;
}
unsafe extern "C" {
    pub fn nw_group_descriptor_create_multicast(multicast_group: NSObject)
        -> nw_group_descriptor_t;
}
unsafe extern "C" {
    pub fn nw_group_descriptor_add_endpoint(descriptor: NSObject, endpoint: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_group_descriptor_enumerate_endpoints(
        descriptor: NSObject,
        enumerate_block: nw_group_descriptor_enumerate_endpoints_block_t,
    );
}
unsafe extern "C" {
    pub fn nw_multicast_group_descriptor_set_specific_source(
        multicast_descriptor: NSObject,
        source: NSObject,
    );
}
unsafe extern "C" {
    pub fn nw_multicast_group_descriptor_set_disable_unicast_traffic(
        multicast_descriptor: NSObject,
        disable_unicast_traffic: bool,
    );
}
unsafe extern "C" {
    pub fn nw_multicast_group_descriptor_get_disable_unicast_traffic(
        multicast_descriptor: NSObject,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_connection_group_create(
        group_descriptor: NSObject,
        parameters: NSObject,
    ) -> nw_connection_group_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_copy_descriptor(group: NSObject) -> nw_group_descriptor_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_copy_parameters(group: NSObject) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_set_queue(group: NSObject, queue: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_group_set_state_changed_handler(
        group: NSObject,
        state_changed_handler: nw_connection_group_state_changed_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_group_set_receive_handler(
        group: NSObject,
        maximum_message_size: u32,
        reject_oversized_messages: bool,
        receive_handler: nw_connection_group_receive_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_group_start(group: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_group_cancel(group: NSObject);
}
unsafe extern "C" {
    pub fn nw_connection_group_copy_remote_endpoint_for_message(
        group: NSObject,
        context: NSObject,
    ) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_copy_local_endpoint_for_message(
        group: NSObject,
        context: NSObject,
    ) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_copy_path_for_message(
        group: NSObject,
        context: NSObject,
    ) -> nw_path_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_copy_protocol_metadata_for_message(
        group: NSObject,
        context: NSObject,
        definition: NSObject,
    ) -> nw_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_extract_connection_for_message(
        group: NSObject,
        context: NSObject,
    ) -> nw_connection_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_reply(
        group: NSObject,
        inbound_message: NSObject,
        outbound_message: NSObject,
        content: NSObject,
    );
}
unsafe extern "C" {
    pub fn nw_connection_group_extract_connection(
        group: NSObject,
        endpoint: NSObject,
        protocol_options: NSObject,
    ) -> nw_connection_t;
}
unsafe extern "C" {
    pub fn nw_connection_group_reinsert_extracted_connection(
        group: NSObject,
        connection: NSObject,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_connection_group_send_message(
        group: NSObject,
        content: NSObject,
        endpoint: NSObject,
        context: NSObject,
        completion: nw_connection_group_send_completion_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_group_set_new_connection_handler(
        group: NSObject,
        new_connection_handler: nw_connection_group_new_connection_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_group_copy_protocol_metadata(
        group: NSObject,
        definition: NSObject,
    ) -> nw_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_connection_access_establishment_report(
        connection: NSObject,
        queue: NSObject,
        access_block: nw_establishment_report_access_block_t,
    );
}
unsafe extern "C" {
    pub fn nw_establishment_report_get_duration_milliseconds(report: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_establishment_report_get_attempt_started_after_milliseconds(report: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_establishment_report_get_previous_attempt_count(report: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_establishment_report_get_used_proxy(report: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_establishment_report_get_proxy_configured(report: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_establishment_report_copy_proxy_endpoint(report: NSObject) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_resolution_report_get_source(
        resolution_report: NSObject,
    ) -> nw_report_resolution_source_t;
}
unsafe extern "C" {
    pub fn nw_resolution_report_get_milliseconds(resolution_report: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_resolution_report_get_endpoint_count(resolution_report: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_resolution_report_copy_successful_endpoint(
        resolution_report: NSObject,
    ) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_resolution_report_copy_preferred_endpoint(
        resolution_report: NSObject,
    ) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_resolution_report_get_protocol(
        resolution_report: NSObject,
    ) -> nw_report_resolution_protocol_t;
}
unsafe extern "C" {
    pub fn nw_establishment_report_enumerate_resolutions(
        report: NSObject,
        enumerate_block: nw_report_resolution_enumerator_t,
    );
}
unsafe extern "C" {
    pub fn nw_establishment_report_enumerate_resolution_reports(
        report: NSObject,
        enumerate_block: nw_report_resolution_report_enumerator_t,
    );
}
unsafe extern "C" {
    pub fn nw_establishment_report_enumerate_protocols(
        report: NSObject,
        enumerate_block: nw_report_protocol_enumerator_t,
    );
}
unsafe extern "C" {
    pub fn nw_connection_create_new_data_transfer_report(
        connection: NSObject,
    ) -> nw_data_transfer_report_t;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_state(report: NSObject) -> nw_data_transfer_report_state_t;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_collect(
        report: NSObject,
        queue: NSObject,
        collect_block: nw_data_transfer_report_collect_block_t,
    );
}
unsafe extern "C" {
    pub static _nw_data_transfer_report_all_paths: u32;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_duration_milliseconds(report: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_path_count(report: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_received_ip_packet_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_sent_ip_packet_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_received_transport_byte_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_received_transport_duplicate_byte_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_received_transport_out_of_order_byte_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_sent_transport_byte_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_sent_transport_retransmitted_byte_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_transport_smoothed_rtt_milliseconds(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_transport_minimum_rtt_milliseconds(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_transport_rtt_variance(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_received_application_byte_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_sent_application_byte_count(
        report: NSObject,
        path_index: u32,
    ) -> u64;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_copy_path_interface(
        report: NSObject,
        path_index: u32,
    ) -> nw_interface_t;
}
unsafe extern "C" {
    pub fn nw_data_transfer_report_get_path_radio_type(
        report: NSObject,
        path_index: u32,
    ) -> nw_interface_radio_type_t;
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_create(
        ether_type: u16,
        interface: NSObject,
    ) -> nw_ethernet_channel_t;
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_create_with_parameters(
        ether_type: u16,
        interface: NSObject,
        parameters: NSObject,
    ) -> nw_ethernet_channel_t;
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_set_state_changed_handler(
        ethernet_channel: NSObject,
        handler: nw_ethernet_channel_state_changed_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_set_queue(ethernet_channel: NSObject, queue: NSObject);
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_get_maximum_payload_size(ethernet_channel: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_start(ethernet_channel: NSObject);
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_cancel(ethernet_channel: NSObject);
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_set_receive_handler(
        ethernet_channel: NSObject,
        handler: nw_ethernet_channel_receive_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_ethernet_channel_send(
        ethernet_channel: NSObject,
        content: NSObject,
        vlan_tag: u16,
        remote_address: *mut ::std::os::raw::c_uchar,
        completion: nw_ethernet_channel_send_completion_t,
    );
}
unsafe extern "C" {
    pub fn nw_framer_protocol_create_message(definition: NSObject) -> nw_framer_message_t;
}
unsafe extern "C" {
    pub fn nw_protocol_metadata_is_framer_message(metadata: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_framer_message_create(framer: NSObject) -> nw_framer_message_t;
}
unsafe extern "C" {
    pub fn nw_framer_message_set_value(
        message: NSObject,
        key: *const ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_void,
        dispose_value: nw_framer_message_dispose_value_t,
    );
}
unsafe extern "C" {
    pub fn nw_framer_message_access_value(
        message: NSObject,
        key: *const ::std::os::raw::c_char,
        access_value: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_framer_message_set_object_value(
        message: NSObject,
        key: *const ::std::os::raw::c_char,
        value: id,
    );
}
unsafe extern "C" {
    pub fn nw_framer_message_copy_object_value(
        message: NSObject,
        key: *const ::std::os::raw::c_char,
    ) -> id;
}
unsafe extern "C" {
    pub fn nw_framer_create_definition(
        identifier: *const ::std::os::raw::c_char,
        flags: u32,
        start_handler: nw_framer_start_handler_t,
    ) -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_framer_create_options(framer_definition: NSObject) -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_framer_options_set_object_value(
        options: NSObject,
        key: *const ::std::os::raw::c_char,
        value: id,
    );
}
unsafe extern "C" {
    pub fn nw_framer_options_copy_object_value(
        options: NSObject,
        key: *const ::std::os::raw::c_char,
    ) -> id;
}
unsafe extern "C" {
    pub fn nw_framer_set_input_handler(framer: NSObject, input_handler: nw_framer_input_handler_t);
}
unsafe extern "C" {
    pub fn nw_framer_set_output_handler(
        framer: NSObject,
        output_handler: nw_framer_output_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_framer_set_wakeup_handler(
        framer: NSObject,
        wakeup_handler: nw_framer_wakeup_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_framer_set_stop_handler(framer: NSObject, stop_handler: nw_framer_stop_handler_t);
}
unsafe extern "C" {
    pub fn nw_framer_set_cleanup_handler(
        framer: NSObject,
        cleanup_handler: nw_framer_cleanup_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_framer_mark_ready(framer: NSObject);
}
unsafe extern "C" {
    pub fn nw_framer_prepend_application_protocol(
        framer: NSObject,
        protocol_options: NSObject,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_framer_mark_failed_with_error(framer: NSObject, error_code: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn nw_framer_parse_input(
        framer: NSObject,
        minimum_incomplete_length: usize,
        maximum_length: usize,
        temp_buffer: *mut u8,
        parse: nw_framer_parse_completion_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_framer_deliver_input(
        framer: NSObject,
        input_buffer: *const u8,
        input_length: usize,
        message: NSObject,
        is_complete: bool,
    );
}
unsafe extern "C" {
    pub fn nw_framer_deliver_input_no_copy(
        framer: NSObject,
        input_length: usize,
        message: NSObject,
        is_complete: bool,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_framer_pass_through_input(framer: NSObject);
}
unsafe extern "C" {
    pub fn nw_framer_parse_output(
        framer: NSObject,
        minimum_incomplete_length: usize,
        maximum_length: usize,
        temp_buffer: *mut u8,
        parse: nw_framer_parse_completion_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_framer_write_output(framer: NSObject, output_buffer: *const u8, output_length: usize);
}
unsafe extern "C" {
    pub fn nw_framer_write_output_data(framer: NSObject, output_data: NSObject);
}
unsafe extern "C" {
    pub fn nw_framer_write_output_no_copy(framer: NSObject, output_length: usize) -> bool;
}
unsafe extern "C" {
    pub fn nw_framer_pass_through_output(framer: NSObject);
}
unsafe extern "C" {
    pub fn nw_framer_schedule_wakeup(framer: NSObject, milliseconds: u64);
}
unsafe extern "C" {
    pub fn nw_framer_async(framer: NSObject, async_block: nw_framer_block_t);
}
unsafe extern "C" {
    pub fn nw_framer_copy_remote_endpoint(framer: NSObject) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_framer_copy_local_endpoint(framer: NSObject) -> nw_endpoint_t;
}
unsafe extern "C" {
    pub fn nw_framer_copy_parameters(framer: NSObject) -> nw_parameters_t;
}
unsafe extern "C" {
    pub fn nw_framer_copy_options(framer: NSObject) -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_protocol_copy_ip_definition() -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_ip_options_set_version(options: NSObject, version: nw_ip_version_t);
}
unsafe extern "C" {
    pub fn nw_ip_options_set_hop_limit(options: NSObject, hop_limit: u8);
}
unsafe extern "C" {
    pub fn nw_ip_options_set_use_minimum_mtu(options: NSObject, use_minimum_mtu: bool);
}
unsafe extern "C" {
    pub fn nw_ip_options_set_disable_fragmentation(options: NSObject, disable_fragmentation: bool);
}
unsafe extern "C" {
    pub fn nw_ip_options_set_calculate_receive_time(
        options: NSObject,
        calculate_receive_time: bool,
    );
}
unsafe extern "C" {
    pub fn nw_ip_options_set_local_address_preference(
        options: NSObject,
        preference: nw_ip_local_address_preference_t,
    );
}
unsafe extern "C" {
    pub fn nw_ip_options_set_disable_multicast_loopback(
        options: NSObject,
        disable_multicast_loopback: bool,
    );
}
unsafe extern "C" {
    pub fn nw_ip_create_metadata() -> nw_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_protocol_metadata_is_ip(metadata: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_ip_metadata_set_ecn_flag(metadata: NSObject, ecn_flag: nw_ip_ecn_flag_t);
}
unsafe extern "C" {
    pub fn nw_ip_metadata_get_ecn_flag(metadata: NSObject) -> nw_ip_ecn_flag_t;
}
unsafe extern "C" {
    pub fn nw_ip_metadata_set_service_class(metadata: NSObject, service_class: nw_service_class_t);
}
unsafe extern "C" {
    pub fn nw_ip_metadata_get_service_class(metadata: NSObject) -> nw_service_class_t;
}
unsafe extern "C" {
    pub fn nw_ip_metadata_get_receive_time(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_listener_create_with_port(
        port: *const ::std::os::raw::c_char,
        parameters: NSObject,
    ) -> nw_listener_t;
}
unsafe extern "C" {
    pub fn nw_listener_create_with_launchd_key(
        parameters: NSObject,
        launchd_key: *const ::std::os::raw::c_char,
    ) -> nw_listener_t;
}
unsafe extern "C" {
    pub fn nw_listener_create(parameters: NSObject) -> nw_listener_t;
}
unsafe extern "C" {
    pub fn nw_listener_create_with_connection(
        connection: NSObject,
        parameters: NSObject,
    ) -> nw_listener_t;
}
unsafe extern "C" {
    pub fn nw_listener_set_queue(listener: NSObject, queue: NSObject);
}
unsafe extern "C" {
    pub fn nw_listener_set_state_changed_handler(
        listener: NSObject,
        handler: nw_listener_state_changed_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_listener_set_new_connection_handler(
        listener: NSObject,
        handler: nw_listener_new_connection_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_listener_set_new_connection_group_handler(
        listener: NSObject,
        handler: nw_listener_new_connection_group_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_listener_get_new_connection_limit(listener: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_listener_set_new_connection_limit(listener: NSObject, new_connection_limit: u32);
}
unsafe extern "C" {
    pub fn nw_listener_set_advertise_descriptor(listener: NSObject, advertise_descriptor: NSObject);
}
unsafe extern "C" {
    pub fn nw_listener_set_advertised_endpoint_changed_handler(
        listener: NSObject,
        handler: nw_listener_advertised_endpoint_changed_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_listener_get_port(listener: NSObject) -> u16;
}
unsafe extern "C" {
    pub fn nw_listener_start(listener: NSObject);
}
unsafe extern "C" {
    pub fn nw_listener_cancel(listener: NSObject);
}
unsafe extern "C" {
    pub fn nw_path_monitor_create() -> nw_path_monitor_t;
}
unsafe extern "C" {
    pub fn nw_path_monitor_create_with_type(
        required_interface_type: nw_interface_type_t,
    ) -> nw_path_monitor_t;
}
unsafe extern "C" {
    pub fn nw_path_monitor_create_for_ethernet_channel() -> nw_path_monitor_t;
}
unsafe extern "C" {
    pub fn nw_path_monitor_prohibit_interface_type(
        monitor: NSObject,
        interface_type: nw_interface_type_t,
    );
}
unsafe extern "C" {
    pub fn nw_path_monitor_set_cancel_handler(
        monitor: NSObject,
        cancel_handler: nw_path_monitor_cancel_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_path_monitor_set_update_handler(
        monitor: NSObject,
        update_handler: nw_path_monitor_update_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_path_monitor_set_queue(monitor: NSObject, queue: NSObject);
}
unsafe extern "C" {
    pub fn nw_path_monitor_start(monitor: NSObject);
}
unsafe extern "C" {
    pub fn nw_path_monitor_cancel(monitor: NSObject);
}
unsafe extern "C" {
    pub fn nw_protocol_copy_quic_definition() -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_quic_create_options() -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_protocol_options_is_quic(options: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_quic_add_tls_application_protocol(
        options: NSObject,
        application_protocol: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_quic_copy_sec_protocol_options(options: NSObject) -> sec_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_quic_get_stream_is_unidirectional(options: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_quic_set_stream_is_unidirectional(options: NSObject, is_unidirectional: bool);
}
unsafe extern "C" {
    pub fn nw_quic_get_stream_is_datagram(options: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_quic_set_stream_is_datagram(options: NSObject, is_datagram: bool);
}
unsafe extern "C" {
    pub fn nw_quic_get_initial_max_data(options: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_initial_max_data(options: NSObject, initial_max_data: u64);
}
unsafe extern "C" {
    pub fn nw_quic_get_max_udp_payload_size(options: NSObject) -> u16;
}
unsafe extern "C" {
    pub fn nw_quic_set_max_udp_payload_size(options: NSObject, max_udp_payload_size: u16);
}
unsafe extern "C" {
    pub fn nw_quic_get_idle_timeout(options: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_quic_set_idle_timeout(options: NSObject, idle_timeout: u32);
}
unsafe extern "C" {
    pub fn nw_quic_get_initial_max_streams_bidirectional(options: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_initial_max_streams_bidirectional(
        options: NSObject,
        initial_max_streams_bidirectional: u64,
    );
}
unsafe extern "C" {
    pub fn nw_quic_get_initial_max_streams_unidirectional(options: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_initial_max_streams_unidirectional(
        options: NSObject,
        initial_max_streams_unidirectional: u64,
    );
}
unsafe extern "C" {
    pub fn nw_quic_get_initial_max_stream_data_bidirectional_local(options: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_initial_max_stream_data_bidirectional_local(
        options: NSObject,
        initial_max_stream_data_bidirectional_local: u64,
    );
}
unsafe extern "C" {
    pub fn nw_quic_get_initial_max_stream_data_bidirectional_remote(options: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_initial_max_stream_data_bidirectional_remote(
        options: NSObject,
        initial_max_stream_data_bidirectional_remote: u64,
    );
}
unsafe extern "C" {
    pub fn nw_quic_get_initial_max_stream_data_unidirectional(options: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_initial_max_stream_data_unidirectional(
        options: NSObject,
        initial_max_stream_data_unidirectional: u64,
    );
}
unsafe extern "C" {
    pub fn nw_quic_get_max_datagram_frame_size(options: NSObject) -> u16;
}
unsafe extern "C" {
    pub fn nw_quic_set_max_datagram_frame_size(options: NSObject, max_datagram_frame_size: u16);
}
unsafe extern "C" {
    pub fn nw_protocol_metadata_is_quic(metadata: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_quic_copy_sec_protocol_metadata(metadata: NSObject) -> sec_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_quic_get_stream_id(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_get_stream_type(stream_metadata: NSObject) -> u8;
}
unsafe extern "C" {
    pub fn nw_quic_get_stream_application_error(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_stream_application_error(metadata: NSObject, application_error: u64);
}
unsafe extern "C" {
    pub fn nw_quic_get_local_max_streams_bidirectional(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_local_max_streams_bidirectional(
        metadata: NSObject,
        max_streams_bidirectional: u64,
    );
}
unsafe extern "C" {
    pub fn nw_quic_get_local_max_streams_unidirectional(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_set_local_max_streams_unidirectional(
        metadata: NSObject,
        max_streams_unidirectional: u64,
    );
}
unsafe extern "C" {
    pub fn nw_quic_get_remote_max_streams_bidirectional(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_get_remote_max_streams_unidirectional(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_get_stream_usable_datagram_frame_size(metadata: NSObject) -> u16;
}
unsafe extern "C" {
    pub fn nw_quic_get_application_error(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_quic_get_application_error_reason(
        metadata: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_quic_set_application_error(
        metadata: NSObject,
        application_error: u64,
        reason: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_quic_get_keepalive_interval(metadata: NSObject) -> u16;
}
unsafe extern "C" {
    pub fn nw_quic_set_keepalive_interval(metadata: NSObject, keepalive_interval: u16);
}
unsafe extern "C" {
    pub fn nw_quic_get_remote_idle_timeout(metadata: NSObject) -> u64;
}
unsafe extern "C" {
    pub fn nw_protocol_copy_tcp_definition() -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_tcp_create_options() -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_no_delay(options: NSObject, no_delay: bool);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_no_push(options: NSObject, no_push: bool);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_no_options(options: NSObject, no_options: bool);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_enable_keepalive(options: NSObject, enable_keepalive: bool);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_keepalive_count(options: NSObject, keepalive_count: u32);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_keepalive_idle_time(options: NSObject, keepalive_idle_time: u32);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_keepalive_interval(options: NSObject, keepalive_interval: u32);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_maximum_segment_size(options: NSObject, maximum_segment_size: u32);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_connection_timeout(options: NSObject, connection_timeout: u32);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_persist_timeout(options: NSObject, persist_timeout: u32);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_retransmit_connection_drop_time(
        options: NSObject,
        retransmit_connection_drop_time: u32,
    );
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_retransmit_fin_drop(options: NSObject, retransmit_fin_drop: bool);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_disable_ack_stretching(
        options: NSObject,
        disable_ack_stretching: bool,
    );
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_enable_fast_open(options: NSObject, enable_fast_open: bool);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_disable_ecn(options: NSObject, disable_ecn: bool);
}
unsafe extern "C" {
    pub fn nw_tcp_options_set_multipath_force_version(
        options: NSObject,
        multipath_force_version: nw_multipath_version_t,
    );
}
unsafe extern "C" {
    pub fn nw_protocol_metadata_is_tcp(metadata: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_tcp_get_available_receive_buffer(metadata: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_tcp_get_available_send_buffer(metadata: NSObject) -> u32;
}
unsafe extern "C" {
    pub fn nw_protocol_copy_tls_definition() -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_tls_create_options() -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_tls_copy_sec_protocol_options(options: NSObject) -> sec_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_protocol_metadata_is_tls(metadata: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_tls_copy_sec_protocol_metadata(metadata: NSObject) -> sec_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_protocol_copy_udp_definition() -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_udp_create_options() -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_udp_options_set_prefer_no_checksum(options: NSObject, prefer_no_checksum: bool);
}
unsafe extern "C" {
    pub fn nw_udp_create_metadata() -> nw_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_protocol_metadata_is_udp(metadata: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_protocol_copy_ws_definition() -> nw_protocol_definition_t;
}
unsafe extern "C" {
    pub fn nw_ws_create_options(version: nw_ws_version_t) -> nw_protocol_options_t;
}
unsafe extern "C" {
    pub fn nw_ws_options_add_additional_header(
        options: NSObject,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_ws_options_add_subprotocol(
        options: NSObject,
        subprotocol: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_ws_options_set_auto_reply_ping(options: NSObject, auto_reply_ping: bool);
}
unsafe extern "C" {
    pub fn nw_ws_options_set_skip_handshake(options: NSObject, skip_handshake: bool);
}
unsafe extern "C" {
    pub fn nw_ws_options_set_maximum_message_size(options: NSObject, maximum_message_size: usize);
}
unsafe extern "C" {
    pub fn nw_protocol_metadata_is_ws(metadata: NSObject) -> bool;
}
unsafe extern "C" {
    pub fn nw_ws_create_metadata(opcode: nw_ws_opcode_t) -> nw_protocol_metadata_t;
}
unsafe extern "C" {
    pub fn nw_ws_metadata_get_opcode(metadata: NSObject) -> nw_ws_opcode_t;
}
unsafe extern "C" {
    pub fn nw_ws_metadata_set_close_code(metadata: NSObject, close_code: nw_ws_close_code_t);
}
unsafe extern "C" {
    pub fn nw_ws_metadata_get_close_code(metadata: NSObject) -> nw_ws_close_code_t;
}
unsafe extern "C" {
    pub fn nw_ws_metadata_set_pong_handler(
        metadata: NSObject,
        client_queue: NSObject,
        pong_handler: nw_ws_pong_handler_t,
    );
}
unsafe extern "C" {
    pub fn nw_ws_request_enumerate_subprotocols(
        request: NSObject,
        enumerator: nw_ws_subprotocol_enumerator_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_ws_request_enumerate_additional_headers(
        request: NSObject,
        enumerator: nw_ws_additional_header_enumerator_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_ws_response_create(
        status: nw_ws_response_status_t,
        selected_subprotocol: *const ::std::os::raw::c_char,
    ) -> nw_ws_response_t;
}
unsafe extern "C" {
    pub fn nw_ws_response_get_status(response: NSObject) -> nw_ws_response_status_t;
}
unsafe extern "C" {
    pub fn nw_ws_response_get_selected_subprotocol(
        response: NSObject,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn nw_ws_response_add_additional_header(
        response: NSObject,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn nw_ws_metadata_copy_server_response(metadata: NSObject) -> nw_ws_response_t;
}
unsafe extern "C" {
    pub fn nw_ws_response_enumerate_additional_headers(
        response: NSObject,
        enumerator: nw_ws_additional_header_enumerator_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn nw_ws_options_set_client_request_handler(
        options: NSObject,
        client_queue: NSObject,
        handler: nw_ws_client_request_handler_t,
    );
}

unsafe impl objc2::encode::RefEncode for sockaddr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for sockaddr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("sockaddr", &[]);
}
