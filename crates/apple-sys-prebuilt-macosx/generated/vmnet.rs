#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use libc::{in_addr_t, mode_t, sa_family_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type u_char = ::std::os::raw::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub __u6_addr: in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr__bindgen_ty_1 {
    pub __u6_addr8: [u8; 16usize],
    pub __u6_addr16: [u16; 8usize],
    pub __u6_addr32: [u32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ether_addr {
    pub octet: [u_char; 6usize],
}
pub type ether_addr_t = ether_addr;
pub type operating_modes_t = u32;
pub use self::operating_modes_t as vmnet_mode_t;
pub type interface_event_t = u32;
pub type vmnet_return_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vmpktdesc {
    pub vm_pkt_size: usize,
    pub vm_pkt_iov: *mut iovec,
    pub vm_pkt_iovcnt: u32,
    pub vm_flags: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vmnet_interface {
    _unused: [u8; 0],
}
pub type interface_ref = *mut vmnet_interface;
pub type vmnet_start_interface_completion_handler_t = *mut ::std::os::raw::c_void;
pub type vmnet_interface_event_callback_t = *mut ::std::os::raw::c_void;
pub type vmnet_interface_completion_handler_t = *mut ::std::os::raw::c_void;
pub type vmnet_interface_get_port_forwarding_rules_handler_t = *mut ::std::os::raw::c_void;
pub type vmnet_interface_get_ip_port_forwarding_rules_handler_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vmnet_network {
    _unused: [u8; 0],
}
pub type vmnet_network_ref = *mut vmnet_network;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vmnet_network_configuration {
    _unused: [u8; 0],
}
pub type vmnet_network_configuration_ref = *mut vmnet_network_configuration;
unsafe extern "C" {
    pub static vmnet_operation_mode_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_shared_interface_name_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_mac_address_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_allocate_mac_address_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_mtu_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_max_packet_size_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_interface_id_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_start_address_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_end_address_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_subnet_mask_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_nat66_prefix_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_nat66_prefix_length_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_estimated_packets_available_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_network_identifier_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_host_ip_address_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_host_subnet_mask_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_host_ipv6_address_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_enable_tso_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_enable_isolation_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_enable_checksum_offload_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_enable_virtio_header_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_read_max_packets_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static vmnet_write_max_packets_key: *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn vmnet_start_interface(
        interface_desc: NSObject,
        queue: NSObject,
        handler: vmnet_start_interface_completion_handler_t,
    ) -> interface_ref;
}
unsafe extern "C" {
    pub fn vmnet_interface_set_event_callback(
        interface: interface_ref,
        event_mask: interface_event_t,
        queue: NSObject,
        callback: vmnet_interface_event_callback_t,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_write(
        interface: interface_ref,
        packets: *mut vmpktdesc,
        pktcnt: *mut ::std::os::raw::c_int,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_read(
        interface: interface_ref,
        packets: *mut vmpktdesc,
        pktcnt: *mut ::std::os::raw::c_int,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_stop_interface(
        interface: interface_ref,
        queue: NSObject,
        handler: vmnet_interface_completion_handler_t,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_interface_add_port_forwarding_rule(
        interface: interface_ref,
        protocol: u8,
        external_port: u16,
        internal_address: in_addr,
        internal_port: u16,
        handler: vmnet_interface_completion_handler_t,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_interface_remove_port_forwarding_rule(
        interface: interface_ref,
        protocol: u8,
        external_port: u16,
        handler: vmnet_interface_completion_handler_t,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_port_forwarding_rule_get_details(
        rule: NSObject,
        protocol: *mut u8,
        external_port: *mut u16,
        internal_address: *mut in_addr,
        internal_port: *mut u16,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_interface_get_port_forwarding_rules(
        interface: interface_ref,
        handler: vmnet_interface_get_port_forwarding_rules_handler_t,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_interface_add_ip_port_forwarding_rule(
        interface: interface_ref,
        protocol: u8,
        external_port: u16,
        address_family: u8,
        internal_address: *const ::std::os::raw::c_void,
        internal_port: u16,
        handler: vmnet_interface_completion_handler_t,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_interface_remove_ip_port_forwarding_rule(
        interface: interface_ref,
        protocol: u8,
        external_port: u16,
        address_family: u8,
        handler: vmnet_interface_completion_handler_t,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_ip_port_forwarding_rule_get_details(
        rule: NSObject,
        protocol: *mut u8,
        external_port: *mut u16,
        address_family: u8,
        internal_address: *mut ::std::os::raw::c_void,
        internal_port: *mut u16,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_interface_get_ip_port_forwarding_rules(
        interface: interface_ref,
        address_family: u8,
        handler: vmnet_interface_get_ip_port_forwarding_rules_handler_t,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_copy_shared_interface_list() -> xpc_object_t;
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_create(
        mode: vmnet_mode_t,
        status: *mut vmnet_return_t,
    ) -> vmnet_network_configuration_ref;
}
unsafe extern "C" {
    pub fn vmnet_network_create(
        configuration: vmnet_network_configuration_ref,
        status: *mut vmnet_return_t,
    ) -> vmnet_network_ref;
}
unsafe extern "C" {
    pub fn vmnet_network_get_ipv4_subnet(
        network: vmnet_network_ref,
        subnet: *mut in_addr,
        mask: *mut in_addr,
    );
}
unsafe extern "C" {
    pub fn vmnet_network_get_ipv6_prefix(
        network: vmnet_network_ref,
        prefix: *mut in6_addr,
        prefix_len: *mut u8,
    );
}
unsafe extern "C" {
    pub fn vmnet_network_copy_serialization(
        network: vmnet_network_ref,
        status: *mut vmnet_return_t,
    ) -> xpc_object_t;
}
unsafe extern "C" {
    pub fn vmnet_network_create_with_serialization(
        network: NSObject,
        status: *mut vmnet_return_t,
    ) -> vmnet_network_ref;
}
unsafe extern "C" {
    pub fn vmnet_interface_start_with_network(
        network: vmnet_network_ref,
        interface_desc: NSObject,
        queue: NSObject,
        start_block: vmnet_start_interface_completion_handler_t,
    ) -> interface_ref;
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_set_external_interface(
        config: vmnet_network_configuration_ref,
        interface_name: *const ::std::os::raw::c_char,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_disable_nat44(config: vmnet_network_configuration_ref);
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_disable_nat66(config: vmnet_network_configuration_ref);
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_disable_dhcp(config: vmnet_network_configuration_ref);
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_disable_dns_proxy(config: vmnet_network_configuration_ref);
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_disable_router_advertisement(
        config: vmnet_network_configuration_ref,
    );
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_set_ipv4_subnet(
        config: vmnet_network_configuration_ref,
        subnet_addr: *const in_addr,
        subnet_mask: *const in_addr,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_set_ipv6_prefix(
        config: vmnet_network_configuration_ref,
        prefix: *const in6_addr,
        len: u8,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_add_port_forwarding_rule(
        config: vmnet_network_configuration_ref,
        protocol: u8,
        address_family: sa_family_t,
        internal_port: u16,
        external_port: u16,
        internal_address: *const ::std::os::raw::c_void,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_add_dhcp_reservation(
        config: vmnet_network_configuration_ref,
        client: *const ether_addr_t,
        reservation: *const in_addr,
    ) -> vmnet_return_t;
}
unsafe extern "C" {
    pub fn vmnet_network_configuration_set_mtu(
        config: vmnet_network_configuration_ref,
        mtu: u32,
    ) -> vmnet_return_t;
}

unsafe impl objc2::encode::RefEncode for iovec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for iovec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("iovec", &[]);
}
unsafe impl objc2::encode::RefEncode for in_addr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for in_addr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("in_addr", &[]);
}
unsafe impl objc2::encode::RefEncode for in6_addr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for in6_addr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("in6_addr", &[]);
}
unsafe impl objc2::encode::RefEncode for in6_addr__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for in6_addr__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("in6_addr__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for ether_addr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ether_addr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ether_addr", &[]);
}
unsafe impl objc2::encode::RefEncode for vmpktdesc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vmpktdesc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vmpktdesc", &[]);
}
unsafe impl objc2::encode::RefEncode for vmnet_interface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vmnet_interface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vmnet_interface", &[]);
}
unsafe impl objc2::encode::RefEncode for vmnet_network {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vmnet_network {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vmnet_network", &[]);
}
unsafe impl objc2::encode::RefEncode for vmnet_network_configuration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vmnet_network_configuration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vmnet_network_configuration", &[]);
}
