//! Verify SecurityInterface bindings by exercising SFCertificatePanel,
//! SFCertificateTrustPanel, and SFChooseIdentityPanel.

use apple_sys::SecurityInterface::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SecurityInterface Certificate Panel ===\n");

        // SFCertificatePanel
        println!("--- SFCertificatePanel ---");
        let panel = SFCertificatePanel::sharedCertificatePanel();
        if !panel.0.is_null() {
            println!("Shared panel: {}", nsobj_to_string(panel.0));

            let policies = ISFCertificatePanel::policies(&panel);
            println!("  policies: {}", nsobj_to_string(policies.0));

            let shows_help = ISFCertificatePanel::showsHelp(&panel);
            println!("  showsHelp: {}", shows_help.0);

            let anchor = ISFCertificatePanel::helpAnchor(&panel);
            println!("  helpAnchor: {}", nsstring_to_string(anchor));
        } else {
            println!("Failed to get shared certificate panel.");
        }

        // SFCertificateTrustPanel
        println!("\n--- SFCertificateTrustPanel ---");
        let trust_panel = SFCertificateTrustPanel::sharedCertificateTrustPanel();
        if !trust_panel.0.is_null() {
            println!("Shared trust panel: {}", nsobj_to_string(trust_panel.0));

            let informative_text = ISFCertificateTrustPanel::informativeText(&trust_panel);
            println!(
                "  informativeText: {}",
                nsstring_to_string(informative_text)
            );
        } else {
            println!("Failed to get shared trust panel.");
        }

        // SFChooseIdentityPanel
        println!("\n--- SFChooseIdentityPanel ---");
        let identity_panel = SFChooseIdentityPanel::sharedChooseIdentityPanel();
        if !identity_panel.0.is_null() {
            println!(
                "Shared identity panel: {}",
                nsobj_to_string(identity_panel.0)
            );

            let shows_help = ISFChooseIdentityPanel::showsHelp(&identity_panel);
            println!("  showsHelp: {}", shows_help.0);

            let help_anchor = ISFChooseIdentityPanel::helpAnchor(&identity_panel);
            println!("  helpAnchor: {}", nsstring_to_string(help_anchor));

            let informative_text = ISFChooseIdentityPanel::informativeText(&identity_panel);
            println!(
                "  informativeText: {}",
                nsstring_to_string(informative_text)
            );

            let domain = ISFChooseIdentityPanel::domain(&identity_panel);
            println!("  domain: {}", nsstring_to_string(domain));

            let policies = ISFChooseIdentityPanel::policies(&identity_panel);
            println!("  policies: {}", nsobj_to_string(policies.0));
        } else {
            println!("Failed to get shared identity panel.");
        }

        // SFKeychainSavePanel
        println!("\n--- SFKeychainSavePanel ---");
        let save_panel = SFKeychainSavePanel::sharedKeychainSavePanel();
        if !save_panel.0.is_null() {
            println!("Shared save panel: {}", nsobj_to_string(save_panel.0));
        } else {
            println!("Failed to get shared keychain save panel.");
        }

        // SFKeychainSettingsPanel
        println!("\n--- SFKeychainSettingsPanel ---");
        let settings_panel = SFKeychainSettingsPanel::sharedKeychainSettingsPanel();
        if !settings_panel.0.is_null() {
            println!(
                "Shared settings panel: {}",
                nsobj_to_string(settings_panel.0)
            );
        } else {
            println!("Failed to get shared keychain settings panel.");
        }
    }

    println!("\nDone.");
}
