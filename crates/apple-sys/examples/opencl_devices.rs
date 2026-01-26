//! Enumerate OpenCL platforms and devices.
//!
//! Demonstrates OpenCL's platform and device query APIs:
//! listing available compute platforms and their GPU/CPU devices.

use apple_sys::OpenCL::*;
use std::ptr;

const CL_PLATFORM_NAME: cl_platform_info = 0x0902;
const CL_PLATFORM_VENDOR: cl_platform_info = 0x0903;
const CL_PLATFORM_VERSION: cl_platform_info = 0x0901;
const CL_DEVICE_TYPE_ALL: cl_device_type = 0xFFFFFFFF;
const CL_DEVICE_NAME: cl_device_info = 0x102B;
const CL_DEVICE_TYPE: cl_device_info = 0x1000;
const CL_DEVICE_MAX_COMPUTE_UNITS: cl_device_info = 0x1002;
const CL_DEVICE_GLOBAL_MEM_SIZE: cl_device_info = 0x101F;
const CL_DEVICE_MAX_WORK_GROUP_SIZE: cl_device_info = 0x1004;
const CL_DEVICE_TYPE_CPU: cl_device_type = 1 << 1;
const CL_DEVICE_TYPE_GPU: cl_device_type = 1 << 2;
const CL_DEVICE_TYPE_ACCELERATOR: cl_device_type = 1 << 3;

unsafe fn get_platform_string(platform: cl_platform_id, param: cl_platform_info) -> String {
    let mut size: usize = 0;
    unsafe { clGetPlatformInfo(platform, param, 0, ptr::null_mut(), &mut size) };
    if size == 0 {
        return String::new();
    }
    let mut buf = vec![0u8; size];
    unsafe {
        clGetPlatformInfo(
            platform,
            param,
            size,
            buf.as_mut_ptr() as _,
            ptr::null_mut(),
        )
    };
    String::from_utf8_lossy(&buf[..size.saturating_sub(1)]).to_string()
}

unsafe fn get_device_string(device: cl_device_id, param: cl_device_info) -> String {
    let mut size: usize = 0;
    unsafe { clGetDeviceInfo(device, param, 0, ptr::null_mut(), &mut size) };
    if size == 0 {
        return String::new();
    }
    let mut buf = vec![0u8; size];
    unsafe { clGetDeviceInfo(device, param, size, buf.as_mut_ptr() as _, ptr::null_mut()) };
    String::from_utf8_lossy(&buf[..size.saturating_sub(1)]).to_string()
}

fn main() {
    unsafe {
        println!("=== OpenCL Device Enumeration ===\n");

        // 1. Get platforms
        let mut num_platforms: cl_uint = 0;
        let err = clGetPlatformIDs(0, ptr::null_mut(), &mut num_platforms);
        if err != 0 || num_platforms == 0 {
            println!("No OpenCL platforms found (err={})", err);
            return;
        }

        let mut platforms = vec![ptr::null_mut(); num_platforms as usize];
        clGetPlatformIDs(num_platforms, platforms.as_mut_ptr(), ptr::null_mut());
        println!("Found {} platform(s)\n", num_platforms);

        // 2. Enumerate each platform
        for (i, &platform) in platforms.iter().enumerate() {
            let name = get_platform_string(platform, CL_PLATFORM_NAME);
            let vendor = get_platform_string(platform, CL_PLATFORM_VENDOR);
            let version = get_platform_string(platform, CL_PLATFORM_VERSION);

            println!("Platform {}: {}", i, name);
            println!("  Vendor:  {}", vendor);
            println!("  Version: {}", version);

            // 3. Get devices for this platform
            let mut num_devices: cl_uint = 0;
            let err = clGetDeviceIDs(
                platform,
                CL_DEVICE_TYPE_ALL,
                0,
                ptr::null_mut(),
                &mut num_devices,
            );
            if err != 0 || num_devices == 0 {
                println!("  No devices found");
                continue;
            }

            let mut devices = vec![ptr::null_mut(); num_devices as usize];
            clGetDeviceIDs(
                platform,
                CL_DEVICE_TYPE_ALL,
                num_devices,
                devices.as_mut_ptr(),
                ptr::null_mut(),
            );

            for (j, &device) in devices.iter().enumerate() {
                let dev_name = get_device_string(device, CL_DEVICE_NAME);

                let mut dev_type: cl_device_type = 0;
                clGetDeviceInfo(
                    device,
                    CL_DEVICE_TYPE,
                    std::mem::size_of::<cl_device_type>(),
                    &mut dev_type as *mut _ as _,
                    ptr::null_mut(),
                );
                let type_str = match dev_type {
                    t if t & CL_DEVICE_TYPE_GPU != 0 => "GPU",
                    t if t & CL_DEVICE_TYPE_CPU != 0 => "CPU",
                    t if t & CL_DEVICE_TYPE_ACCELERATOR != 0 => "Accelerator",
                    _ => "Unknown",
                };

                let mut compute_units: cl_uint = 0;
                clGetDeviceInfo(
                    device,
                    CL_DEVICE_MAX_COMPUTE_UNITS,
                    std::mem::size_of::<cl_uint>(),
                    &mut compute_units as *mut _ as _,
                    ptr::null_mut(),
                );

                let mut mem_size: cl_ulong = 0;
                clGetDeviceInfo(
                    device,
                    CL_DEVICE_GLOBAL_MEM_SIZE,
                    std::mem::size_of::<cl_ulong>(),
                    &mut mem_size as *mut _ as _,
                    ptr::null_mut(),
                );

                let mut wg_size: usize = 0;
                clGetDeviceInfo(
                    device,
                    CL_DEVICE_MAX_WORK_GROUP_SIZE,
                    std::mem::size_of::<usize>(),
                    &mut wg_size as *mut _ as _,
                    ptr::null_mut(),
                );

                println!("  Device {}: {} ({})", j, dev_name, type_str);
                println!("    Compute units: {}", compute_units);
                println!("    Global memory: {} MB", mem_size / (1024 * 1024));
                println!("    Max work group: {}", wg_size);
            }
            println!();
        }
    }

    println!("Done.");
}
