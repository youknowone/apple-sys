#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::DiskArbitration::*;
#[allow(unused_imports)]
use libc::id_t;

#[allow(unused_imports)]
use objc2::msg_send;
pub type hv_return_t = mach_error_t;
pub type hv_memory_flags_t = u64;
pub type hv_vm_config_t = NSObject;
pub type hv_ipa_t = u64;
pub type hv_allocate_flags_t = u64;
pub type hv_gic_config_t = NSObject;
pub type hv_gic_state_t = NSObject;
pub type hv_gic_intid_t = u16;
pub type hv_gic_distributor_reg_t = u16;
pub type hv_gic_redistributor_reg_t = u32;
pub type hv_gic_icc_reg_t = u16;
pub type hv_gic_ich_reg_t = u16;
pub type hv_gic_icv_reg_t = u16;
pub type hv_gic_msi_reg_t = u16;
pub type hv_vcpu_config_t = NSObject;
pub type hv_vcpu_t = u64;
pub type hv_exit_reason_t = u32;
pub type hv_exception_syndrome_t = u64;
pub type hv_exception_address_t = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hv_vcpu_exit_exception_t {
    pub syndrome: hv_exception_syndrome_t,
    pub virtual_address: hv_exception_address_t,
    pub physical_address: hv_ipa_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hv_vcpu_exit_t {
    pub reason: hv_exit_reason_t,
    pub exception: hv_vcpu_exit_exception_t,
}
pub type hv_simd_fp_uchar16_t = [u8; 16usize];
pub type hv_reg_t = u32;
pub type hv_simd_fp_reg_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hv_vcpu_sme_state_t {
    pub streaming_sve_mode_enabled: bool,
    pub za_storage_enabled: bool,
}
pub type hv_sme_z_reg_t = u32;
pub type hv_sme_p_reg_t = u32;
pub type hv_sme_zt0_uchar64_t = [u8; 64usize];
pub type hv_sys_reg_t = u16;
pub type hv_interrupt_type_t = u32;
pub type hv_cache_type_t = u32;
pub type hv_feature_reg_t = u32;
pub type hv_ipa_granule_t = u32;
unsafe extern "C" {
    pub fn hv_vm_allocate(
        uvap: *mut *mut ::std::os::raw::c_void,
        size: usize,
        flags: hv_allocate_flags_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_deallocate(uva: *mut ::std::os::raw::c_void, size: usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_create(
        vcpu: *mut hv_vcpu_t,
        exit: *mut *mut hv_vcpu_exit_t,
        config: NSObject,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_destroy(vcpu: hv_vcpu_t) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_reg(vcpu: hv_vcpu_t, reg: hv_reg_t, value: *mut u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_reg(vcpu: hv_vcpu_t, reg: hv_reg_t, value: u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_simd_fp_reg(
        vcpu: hv_vcpu_t,
        reg: hv_simd_fp_reg_t,
        value: *mut hv_simd_fp_uchar16_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_simd_fp_reg(
        vcpu: hv_vcpu_t,
        reg: hv_simd_fp_reg_t,
        value: hv_simd_fp_uchar16_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_sme_state(
        vcpu: hv_vcpu_t,
        sme_state: *mut hv_vcpu_sme_state_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_sme_state(
        vcpu: hv_vcpu_t,
        sme_state: *const hv_vcpu_sme_state_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_sme_z_reg(
        vcpu: hv_vcpu_t,
        reg: hv_sme_z_reg_t,
        value: *mut u8,
        length: usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_sme_z_reg(
        vcpu: hv_vcpu_t,
        reg: hv_sme_z_reg_t,
        value: *const u8,
        length: usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_sme_p_reg(
        vcpu: hv_vcpu_t,
        reg: hv_sme_p_reg_t,
        value: *mut u8,
        length: usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_sme_p_reg(
        vcpu: hv_vcpu_t,
        reg: hv_sme_p_reg_t,
        value: *const u8,
        length: usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_sme_za_reg(vcpu: hv_vcpu_t, value: *mut u8, length: usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_sme_za_reg(vcpu: hv_vcpu_t, value: *const u8, length: usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_sme_zt0_reg(
        vcpu: hv_vcpu_t,
        value: *mut hv_sme_zt0_uchar64_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_sme_zt0_reg(
        vcpu: hv_vcpu_t,
        value: *const hv_sme_zt0_uchar64_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_sys_reg(vcpu: hv_vcpu_t, reg: hv_sys_reg_t, value: *mut u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_sys_reg(vcpu: hv_vcpu_t, reg: hv_sys_reg_t, value: u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_pending_interrupt(
        vcpu: hv_vcpu_t,
        type_: hv_interrupt_type_t,
        pending: *mut bool,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_pending_interrupt(
        vcpu: hv_vcpu_t,
        type_: hv_interrupt_type_t,
        pending: bool,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_trap_debug_exceptions(vcpu: hv_vcpu_t, value: *mut bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_trap_debug_exceptions(vcpu: hv_vcpu_t, value: bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_trap_debug_reg_accesses(vcpu: hv_vcpu_t, value: *mut bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_trap_debug_reg_accesses(vcpu: hv_vcpu_t, value: bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_run(vcpu: hv_vcpu_t) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpus_exit(vcpus: *mut hv_vcpu_t, vcpu_count: u32) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_exec_time(vcpu: hv_vcpu_t, time: *mut u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_vtimer_mask(vcpu: hv_vcpu_t, vtimer_is_masked: *mut bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_vtimer_mask(vcpu: hv_vcpu_t, vtimer_is_masked: bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_get_vtimer_offset(vcpu: hv_vcpu_t, vtimer_offset: *mut u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_set_vtimer_offset(vcpu: hv_vcpu_t, vtimer_offset: u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_create(gic_config: NSObject) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_set_spi(intid: u32, level: bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_send_msi(address: hv_ipa_t, intid: u32) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_distributor_reg(
        reg: hv_gic_distributor_reg_t,
        value: *mut u64,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_set_distributor_reg(reg: hv_gic_distributor_reg_t, value: u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_redistributor_base(
        vcpu: hv_vcpu_t,
        redistributor_base_address: *mut hv_ipa_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_redistributor_reg(
        vcpu: hv_vcpu_t,
        reg: hv_gic_redistributor_reg_t,
        value: *mut u64,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_set_redistributor_reg(
        vcpu: hv_vcpu_t,
        reg: hv_gic_redistributor_reg_t,
        value: u64,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_icc_reg(
        vcpu: hv_vcpu_t,
        reg: hv_gic_icc_reg_t,
        value: *mut u64,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_set_icc_reg(vcpu: hv_vcpu_t, reg: hv_gic_icc_reg_t, value: u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_ich_reg(
        vcpu: hv_vcpu_t,
        reg: hv_gic_ich_reg_t,
        value: *mut u64,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_set_ich_reg(vcpu: hv_vcpu_t, reg: hv_gic_ich_reg_t, value: u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_icv_reg(
        vcpu: hv_vcpu_t,
        reg: hv_gic_icv_reg_t,
        value: *mut u64,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_set_icv_reg(vcpu: hv_vcpu_t, reg: hv_gic_icv_reg_t, value: u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_msi_reg(reg: hv_gic_msi_reg_t, value: *mut u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_set_msi_reg(reg: hv_gic_msi_reg_t, value: u64) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_set_state(
        gic_state_data: *const ::std::os::raw::c_void,
        gic_state_size: usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_reset() -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_config_create() -> hv_gic_config_t;
}
unsafe extern "C" {
    pub fn hv_gic_config_set_distributor_base(
        config: NSObject,
        distributor_base_address: hv_ipa_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_config_set_redistributor_base(
        config: NSObject,
        redistributor_base_address: hv_ipa_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_config_set_msi_region_base(
        config: NSObject,
        msi_region_base_address: hv_ipa_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_config_set_msi_interrupt_range(
        config: NSObject,
        msi_intid_base: u32,
        msi_intid_count: u32,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_distributor_size(distributor_size: *mut usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_distributor_base_alignment(
        distributor_base_alignment: *mut usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_redistributor_region_size(
        redistributor_region_size: *mut usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_redistributor_size(redistributor_size: *mut usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_redistributor_base_alignment(
        redistributor_base_alignment: *mut usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_msi_region_size(msi_region_size: *mut usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_msi_region_base_alignment(
        msi_region_base_alignment: *mut usize,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_spi_interrupt_range(
        spi_intid_base: *mut u32,
        spi_intid_count: *mut u32,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_get_intid(interrupt: hv_gic_intid_t, intid: *mut u32) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_state_create() -> hv_gic_state_t;
}
unsafe extern "C" {
    pub fn hv_gic_state_get_size(state: NSObject, gic_state_size: *mut usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_gic_state_get_data(
        state: NSObject,
        gic_state_data: *mut ::std::os::raw::c_void,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_sme_config_get_max_svl_bytes(value: *mut usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_config_create() -> hv_vcpu_config_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_config_get_feature_reg(
        config: NSObject,
        feature_reg: hv_feature_reg_t,
        value: *mut u64,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vcpu_config_get_ccsidr_el1_sys_reg_values(
        config: NSObject,
        cache_type: hv_cache_type_t,
        values: *mut u64,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_get_max_vcpu_count(max_vcpu_count: *mut u32) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_create(config: NSObject) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_destroy() -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_map(
        addr: *mut ::std::os::raw::c_void,
        ipa: hv_ipa_t,
        size: usize,
        flags: hv_memory_flags_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_unmap(ipa: hv_ipa_t, size: usize) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_protect(ipa: hv_ipa_t, size: usize, flags: hv_memory_flags_t) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_create() -> hv_vm_config_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_get_max_ipa_size(ipa_bit_length: *mut u32) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_get_default_ipa_size(ipa_bit_length: *mut u32) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_set_ipa_size(config: NSObject, ipa_bit_length: u32) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_get_ipa_size(config: NSObject, ipa_bit_length: *mut u32) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_get_el2_supported(el2_supported: *mut bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_get_el2_enabled(config: NSObject, el2_enabled: *mut bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_set_el2_enabled(config: NSObject, el2_enabled: bool) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_get_default_ipa_granule(granule: *mut hv_ipa_granule_t) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_get_ipa_granule(
        config: NSObject,
        granule: *mut hv_ipa_granule_t,
    ) -> hv_return_t;
}
unsafe extern "C" {
    pub fn hv_vm_config_set_ipa_granule(config: NSObject, granule: hv_ipa_granule_t)
        -> hv_return_t;
}

unsafe impl objc2::encode::RefEncode for hv_vcpu_exit_exception_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for hv_vcpu_exit_exception_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("hv_vcpu_exit_exception_t", &[]);
}
unsafe impl objc2::encode::RefEncode for hv_vcpu_exit_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for hv_vcpu_exit_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("hv_vcpu_exit_t", &[]);
}
unsafe impl objc2::encode::RefEncode for hv_vcpu_sme_state_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for hv_vcpu_sme_state_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("hv_vcpu_sme_state_t", &[]);
}
