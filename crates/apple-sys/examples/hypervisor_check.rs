//! Check Hypervisor.framework availability and capabilities.
//!
//! Attempts to create and destroy a lightweight VM to verify
//! that the hypervisor is available on this system.

use apple_sys::Hypervisor::*;
use apple_sys::objc::NSObject;

const HV_SUCCESS: i32 = 0;
const HV_ERROR: i32 = -0x05160000 + 1; // 0xfae94001
const HV_BUSY: i32 = -0x05160000 + 2;
const HV_BAD_ARGUMENT: i32 = -0x05160000 + 3;
const HV_NO_RESOURCES: i32 = -0x05160000 + 5;
const HV_NO_DEVICE: i32 = -0x05160000 + 6;
const HV_UNSUPPORTED: i32 = -0x05160000 + 15;

fn hv_status(ret: hv_return_t) -> &'static str {
    match ret as i32 {
        HV_SUCCESS => "HV_SUCCESS",
        HV_ERROR => "HV_ERROR",
        HV_BUSY => "HV_BUSY",
        HV_BAD_ARGUMENT => "HV_BAD_ARGUMENT",
        HV_NO_RESOURCES => "HV_NO_RESOURCES",
        HV_NO_DEVICE => "HV_NO_DEVICE",
        HV_UNSUPPORTED => "HV_UNSUPPORTED",
        _ => "UNKNOWN",
    }
}

fn main() {
    unsafe {
        println!("=== Hypervisor.framework Check ===\n");

        // Try to create a VM
        println!("Attempting hv_vm_create...");
        let ret = hv_vm_create(NSObject(std::ptr::null_mut()));
        let status = hv_status(ret);
        println!("  Result: {} (0x{:x})", status, ret as u32);

        if ret as i32 == HV_SUCCESS {
            println!("  Hypervisor is available!");

            // Create and destroy a vCPU to verify full functionality
            let mut vcpu: hv_vcpu_t = 0;
            let mut vcpu_exit: *mut hv_vcpu_exit_t = std::ptr::null_mut();
            let ret = hv_vcpu_create(&mut vcpu, &mut vcpu_exit, NSObject(std::ptr::null_mut()));
            if ret as i32 == HV_SUCCESS {
                println!("  vCPU created: id={}", vcpu);
                let ret = hv_vcpu_destroy(vcpu);
                println!(
                    "  vCPU destroyed: {}",
                    if ret as i32 == HV_SUCCESS {
                        "OK"
                    } else {
                        "FAILED"
                    }
                );
            } else {
                println!("  vCPU creation: {} (0x{:x})", hv_status(ret), ret as u32);
            }

            // Destroy the VM
            let ret = hv_vm_destroy();
            println!(
                "  VM destroyed: {}",
                if ret as i32 == HV_SUCCESS {
                    "OK"
                } else {
                    "FAILED"
                }
            );
        } else {
            println!("  Hypervisor not available (may need entitlement or SIP config).");
            println!("  Add com.apple.security.hypervisor entitlement to use.");
        }
    }

    println!("\nDone.");
}
