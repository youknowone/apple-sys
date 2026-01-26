//! Open the TWAIN Data Source Manager and enumerate data sources.
//!
//! Demonstrates DSM_Entry for opening the DSM and listing
//! available scanner/image acquisition devices.

use apple_sys::TWAIN::*;
// TWAIN constants
const TWON_PROTOCOLMAJOR: u16 = 2;
const TWON_PROTOCOLMINOR: u16 = 4;

// Data Groups
const DG_CONTROL: u32 = 0x0001;

// Data Argument Types
const DAT_PARENT: u16 = 0x0004;
const DAT_IDENTITY: u16 = 0x0006;
const DAT_STATUS: u16 = 0x0008;

// Messages
const MSG_OPENDSM: u16 = 0x0301;
const MSG_CLOSEDSM: u16 = 0x0302;
const MSG_GETFIRST: u16 = 0x0004;
const MSG_GETNEXT: u16 = 0x0005;

// Return codes
const TWRC_SUCCESS: u16 = 0x0000;
const TWRC_ENDOFLIST: u16 = 0x0007;

// Supported Groups
const DG_IMAGE: u32 = 0x0002;
const DF_APP2: u32 = 0x20000;

fn tw_str32(s: &[u8; 34]) -> String {
    let end = s.iter().position(|&b| b == 0).unwrap_or(s.len());
    String::from_utf8_lossy(&s[..end]).to_string()
}

fn twrc_name(rc: u16) -> &'static str {
    match rc {
        0 => "TWRC_SUCCESS",
        1 => "TWRC_FAILURE",
        2 => "TWRC_CHECKSTATUS",
        3 => "TWRC_CANCEL",
        4 => "TWRC_DSEVENT",
        5 => "TWRC_NOTDSEVENT",
        6 => "TWRC_XFERDONE",
        7 => "TWRC_ENDOFLIST",
        _ => "UNKNOWN",
    }
}

fn main() {
    unsafe {
        println!("=== TWAIN ===\n");

        // 1. Set up application identity
        let mut app_identity: TW_IDENTITY = std::mem::zeroed();
        app_identity.Version.MajorNum = 1;
        app_identity.Version.MinorNum = 0;
        app_identity.Version.Language = 13; // TWLG_ENGLISH_USA
        app_identity.Version.Country = 1; // TWCY_USA
        app_identity.ProtocolMajor = TWON_PROTOCOLMAJOR;
        app_identity.ProtocolMinor = TWON_PROTOCOLMINOR;
        app_identity.SupportedGroups = DG_IMAGE | DF_APP2;

        // Copy strings into fixed-size arrays
        let info = b"1.0\0";
        app_identity.Version.Info[..info.len()].copy_from_slice(info);
        let mfr = b"apple-sys\0";
        app_identity.Manufacturer[..mfr.len()].copy_from_slice(mfr);
        let family = b"Example\0";
        app_identity.ProductFamily[..family.len()].copy_from_slice(family);
        let name = b"twain_scan\0";
        app_identity.ProductName[..name.len()].copy_from_slice(name);

        println!("Application identity:");
        println!("  Manufacturer:  {}", tw_str32(&app_identity.Manufacturer));
        println!("  ProductFamily: {}", tw_str32(&app_identity.ProductFamily));
        println!("  ProductName:   {}", tw_str32(&app_identity.ProductName));
        println!(
            "  Protocol:      {}.{}",
            app_identity.ProtocolMajor, app_identity.ProtocolMinor
        );

        // 2. Open the DSM (Data Source Manager)
        // Parent window handle is null for console applications
        let mut parent_handle: *mut *mut i8 = std::ptr::null_mut();
        println!("\nOpening TWAIN Data Source Manager...");
        let rc = DSM_Entry(
            &mut app_identity,
            std::ptr::null_mut(),
            DG_CONTROL,
            DAT_PARENT,
            MSG_OPENDSM,
            &mut parent_handle as *mut _ as TW_MEMREF,
        );
        println!("  DSM_Entry(MSG_OPENDSM): {}", twrc_name(rc));

        if rc == TWRC_SUCCESS {
            // 3. Enumerate available data sources
            println!("\nEnumerating data sources:");
            let mut source: TW_IDENTITY = std::mem::zeroed();
            let mut count = 0;

            let rc = DSM_Entry(
                &mut app_identity,
                std::ptr::null_mut(),
                DG_CONTROL,
                DAT_IDENTITY,
                MSG_GETFIRST,
                &mut source as *mut _ as TW_MEMREF,
            );

            if rc == TWRC_SUCCESS {
                loop {
                    count += 1;
                    println!("  Source {}:", count);
                    println!("    Manufacturer:  {}", tw_str32(&source.Manufacturer));
                    println!("    ProductFamily: {}", tw_str32(&source.ProductFamily));
                    println!("    ProductName:   {}", tw_str32(&source.ProductName));

                    let rc = DSM_Entry(
                        &mut app_identity,
                        std::ptr::null_mut(),
                        DG_CONTROL,
                        DAT_IDENTITY,
                        MSG_GETNEXT,
                        &mut source as *mut _ as TW_MEMREF,
                    );
                    if rc != TWRC_SUCCESS {
                        break;
                    }
                }
            } else if rc == TWRC_ENDOFLIST {
                println!("  No data sources found.");
            } else {
                println!("  DSM_Entry(MSG_GETFIRST): {}", twrc_name(rc));
            }

            println!("  Total sources found: {}", count);

            // 4. Close the DSM
            let rc = DSM_Entry(
                &mut app_identity,
                std::ptr::null_mut(),
                DG_CONTROL,
                DAT_PARENT,
                MSG_CLOSEDSM,
                &mut parent_handle as *mut _ as TW_MEMREF,
            );
            println!("\nDSM_Entry(MSG_CLOSEDSM): {}", twrc_name(rc));
        } else {
            println!("  Could not open DSM (no TWAIN driver installed?).");

            // Show a TW_STATUS query attempt
            let mut status: TW_STATUS = std::mem::zeroed();
            let rc2 = DSM_Entry(
                &mut app_identity,
                std::ptr::null_mut(),
                DG_CONTROL,
                DAT_STATUS,
                0x0001, // MSG_GET
                &mut status as *mut _ as TW_MEMREF,
            );
            if rc2 == TWRC_SUCCESS {
                println!("  Status condition code: {}", status.ConditionCode);
            }
        }

        // 5. Show struct sizes
        println!("\nTWAIN struct sizes:");
        println!(
            "  TW_IDENTITY:  {} bytes",
            std::mem::size_of::<TW_IDENTITY>()
        );
        println!(
            "  TW_IMAGEINFO: {} bytes",
            std::mem::size_of::<TW_IMAGEINFO>()
        );
        println!(
            "  TW_CAPABILITY: {} bytes",
            std::mem::size_of::<TW_CAPABILITY>()
        );
        println!("  TW_STATUS:    {} bytes", std::mem::size_of::<TW_STATUS>());
    }

    println!("\nDone.");
}
