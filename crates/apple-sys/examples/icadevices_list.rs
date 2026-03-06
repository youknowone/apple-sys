//! Probe ICA (Image Capture Architecture) devices.
//!
//! Demonstrates ICAOpenSession, ICACopyObjectData, and
//! ICAObjectSendMessage to interact with imaging devices
//! (cameras, scanners).

use apple_sys::ICADevices::*;
use std::ptr;

fn ica_error_string(err: i16) -> &'static str {
    match err as i32 {
        0 => "noErr",
        -9900 => "kICACommunicationErr",
        -9901 => "kICADeviceNotFoundErr",
        -9902 => "kICADeviceNotOpenErr",
        -9903 => "kICAFileCorruptedErr",
        -9904 => "kICAIOPendingErr",
        -9905 => "kICAInvalidObjectErr",
        -9906 => "kICAInvalidPropertyErr",
        -9907 => "kICAIndexOutOfRangeErr",
        -9908 => "kICAPropertyTypeNotFoundErr",
        _ => "unknown",
    }
}

/// Read a field from a packed struct safely.
macro_rules! read_packed {
    ($s:expr, $field:ident) => {
        ptr::addr_of!($s.$field).read_unaligned()
    };
}

fn main() {
    unsafe {
        println!("=== ICA Devices ===\n");

        // Try to open a session with device object 0 (root/test)
        let mut open_pb = ICAOpenSessionPB {
            header: ICAHeader { err: 0, refcon: 0 },
            deviceObject: 0,
            sessionID: 0,
        };
        let err = ICAOpenSession(&mut open_pb, None);
        let header_err = read_packed!(open_pb, header).err;
        println!("ICAOpenSession (device=0):");
        println!("  Return: {} ({})", ica_error_string(err), err);
        println!(
            "  Header err: {} ({})",
            ica_error_string(header_err),
            header_err
        );

        if err == 0 {
            let session_id = read_packed!(open_pb, sessionID);
            println!("  Session ID: {}", session_id);

            // Try to copy object data from root
            let mut data = ptr::null();
            let mut copy_pb = ICACopyObjectDataPB {
                header: ICAHeader { err: 0, refcon: 0 },
                object: 0,
                startByte: 0,
                requestedSize: 4096,
                data: &mut data,
            };
            let err = ICACopyObjectData(&mut copy_pb, None);
            println!("\n  ICACopyObjectData:");
            println!("    Return: {} ({})", ica_error_string(err), err);
            if err == 0 && !data.is_null() {
                println!("    Data retrieved successfully");
            }

            // Close the session
            let mut close_pb = ICACloseSessionPB {
                header: ICAHeader { err: 0, refcon: 0 },
                sessionID: session_id,
            };
            let err = ICACloseSession(&mut close_pb, None);
            println!("\n  ICACloseSession:");
            println!("    Return: {} ({})", ica_error_string(err), err);
        } else {
            println!("  No imaging device available (expected without hardware).");
        }

        // Try scanner session
        let mut scanner_pb = ICAScannerOpenSessionPB {
            header: ICAHeader { err: 0, refcon: 0 },
            object: 0,
            sessionID: 0,
        };
        let err = ICAScannerOpenSession(&mut scanner_pb, None);
        println!("\nICAScannerOpenSession (object=0):");
        println!("  Return: {} ({})", ica_error_string(err), err);
        if err == 0 {
            let scanner_session = read_packed!(scanner_pb, sessionID);
            println!("  Scanner session ID: {}", scanner_session);

            // Get scanner status
            let mut status_pb = ICAScannerStatusPB {
                header: ICAHeader { err: 0, refcon: 0 },
                sessionID: scanner_session,
                status: 0,
            };
            let err = ICAScannerStatus(&mut status_pb, None);
            let status_val = read_packed!(status_pb, status);
            println!("  Scanner status: {} (err={})", status_val, err);

            // Close scanner session
            let mut close_pb = ICAScannerCloseSessionPB {
                header: ICAHeader { err: 0, refcon: 0 },
                sessionID: scanner_session,
            };
            let _ = ICAScannerCloseSession(&mut close_pb, None);
        } else {
            println!("  No scanner available (expected without hardware).");
        }

        // Try sending a message to a non-existent object
        let mut msg_pb = ICAObjectSendMessagePB {
            header: ICAHeader { err: 0, refcon: 0 },
            object: 0,
            message: ICAMessage {
                messageType: 0,
                startByte: 0,
                dataPtr: ptr::null_mut(),
                dataSize: 0,
                dataType: 0,
            },
            result: 0,
        };
        let err = ICAObjectSendMessage(&mut msg_pb, None);
        let msg_result = read_packed!(msg_pb, result);
        println!("\nICAObjectSendMessage (object=0):");
        println!("  Return: {} ({})", ica_error_string(err), err);
        println!("  Result: {}", msg_result);
    }

    println!("\nDone.");
}
