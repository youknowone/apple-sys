//! Verify that umbrella framework sub-framework symbols are available.
//!
//! ApplicationServices is an umbrella framework whose sub-frameworks
//! (HIServices, PrintCore, ATS) have their headers in nested
//! Frameworks/ directories. This test ensures those symbols are
//! correctly extracted and included in the generated bindings.

#[cfg(all(target_os = "macos", feature = "ApplicationServices"))]
mod application_services {
    use apple_sys::ApplicationServices::*;
    use apple_sys::CoreFoundation::{Boolean, CFTypeID};

    /// HIServices: AXIsProcessTrusted should be callable.
    #[test]
    fn hiservices_ax_is_process_trusted() {
        unsafe {
            let _trusted: Boolean = AXIsProcessTrusted();
        }
    }

    /// HIServices: AXUIElementGetTypeID should return a valid CFTypeID.
    #[test]
    fn hiservices_ax_ui_element_type_id() {
        unsafe {
            let type_id: CFTypeID = AXUIElementGetTypeID();
            assert!(type_id > 0);
        }
    }

    /// PrintCore: PMCreateSession + PMRelease round-trip.
    #[test]
    fn printcore_pm_create_session() {
        unsafe {
            let mut session: PMPrintSession = std::ptr::null_mut();
            let status = PMCreateSession(&mut session);
            assert_eq!(status, 0, "PMCreateSession should return noErr (0)");
            assert!(!session.is_null(), "PMPrintSession should be non-null");
            let rel_status = PMRelease(session as PMObject);
            assert_eq!(rel_status, 0, "PMRelease should return noErr (0)");
        }
    }

    /// ATS: ATSFontContext type should be available.
    #[test]
    fn ats_types_available() {
        // ATSFontContext is a typedef from ATS sub-framework
        let _ctx: ATSFontContext = 0;
    }
}
