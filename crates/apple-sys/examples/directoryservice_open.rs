//! Open a DirectoryService session and query node count.
//!
//! Demonstrates dsOpenDirService, dsGetDirNodeCount,
//! dsCopyDirStatusName, and dsCloseDirService.

use apple_sys::DirectoryService::*;
use std::ffi::CStr;
use std::ptr;

fn status_name(status: tDirStatus) -> String {
    unsafe {
        let name = dsCopyDirStatusName(status);
        if name.is_null() {
            return format!("(unknown status {})", status);
        }
        let s = CStr::from_ptr(name).to_string_lossy().into_owned();
        libc::free(name as _);
        s
    }
}

fn main() {
    unsafe {
        println!("=== DirectoryService ===\n");

        // Open a directory service reference
        let mut dir_ref: tDirReference = 0;
        let status = dsOpenDirService(&mut dir_ref);
        println!("dsOpenDirService: {} ({})", status_name(status), status);

        if status != 0 {
            eprintln!("  Failed to open DirectoryService.");
            println!("\nDone.");
            return;
        }
        println!("  Directory reference: {}\n", dir_ref);

        // Verify the reference is valid
        let verify = dsVerifyDirRefNum(dir_ref);
        println!("dsVerifyDirRefNum: {} ({})", status_name(verify), verify);

        // Get the number of directory nodes
        let mut node_count: u32 = 0;
        let status = dsGetDirNodeCount(dir_ref, &mut node_count);
        println!("dsGetDirNodeCount: {} ({})", status_name(status), status);
        if status == 0 {
            println!("  Directory nodes available: {}", node_count);
        }

        // Get node count with change token
        let mut node_count2: u32 = 0;
        let mut change_token: u32 = 0;
        let status = dsGetDirNodeCountWithInfo(dir_ref, &mut node_count2, &mut change_token);
        if status == 0 {
            println!(
                "  Node count (with info): {}, change token: {}",
                node_count2, change_token
            );
        }

        // Allocate a data buffer and get the node list
        let buffer = dsDataBufferAllocate(dir_ref, 8192);
        if !buffer.is_null() {
            let mut count: u32 = 0;
            let mut context: tContextData = 0;
            let status = dsGetDirNodeList(dir_ref, buffer, &mut count, &mut context);
            println!("\ndsGetDirNodeList: {} ({})", status_name(status), status);
            if status == 0 {
                println!("  Nodes returned in this batch: {}", count);

                // Print the first few node names
                let limit = count.min(10);
                for i in 1..=limit {
                    let mut name_list: tDataListPtr = ptr::null_mut();
                    let err = dsGetDirNodeName(dir_ref, buffer, i, &mut name_list);
                    if err == 0 && !name_list.is_null() {
                        let node_parts = dsDataListGetNodeCount(name_list);
                        let path = dsGetPathFromList(dir_ref, name_list, c"/".as_ptr());
                        if !path.is_null() {
                            let path_str = CStr::from_ptr(path).to_string_lossy();
                            println!("  {:>3}. {} (parts={})", i, path_str, node_parts);
                            libc::free(path as _);
                        }
                        dsDataListDeallocate(dir_ref, name_list);
                    }
                }
                if count > 10 {
                    println!("  ... and {} more", count - 10);
                }
            }

            if context != 0 {
                dsReleaseContinueData(dir_ref, context);
            }
            dsDataBufferDeAllocate(dir_ref, buffer);
        }

        // Close
        let status = dsCloseDirService(dir_ref);
        println!("\ndsCloseDirService: {} ({})", status_name(status), status);
    }

    println!("\nDone.");
}
