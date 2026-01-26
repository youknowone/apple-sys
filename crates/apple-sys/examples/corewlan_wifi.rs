//! Display WiFi network information using CoreWLAN.
//!
//! Shows current SSID, BSSID, signal strength, channel, and lists
//! all supported channels grouped by band.

use apple_sys::CoreWLAN::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreWLAN WiFi Information ===\n");

        let client = CWWiFiClient::sharedWiFiClient();
        assert!(!client.0.is_null(), "Failed to get shared WiFi client");

        let iface = ICWWiFiClient::interface(&client);
        if iface.0.is_null() {
            println!("No WiFi interface found.");
            return;
        }

        let iface_name = ICWInterface::interfaceName(&iface);
        println!("Interface: {}", nsstring_to_string(iface_name));

        let power_on = ICWInterface::powerOn(&iface);
        println!("Power:     {}", if power_on.0 { "On" } else { "Off" });

        if !power_on.0 {
            println!("\nWiFi is turned off.");
            return;
        }

        // Connection details
        let ssid = ICWInterface::ssid(&iface);
        let bssid = ICWInterface::bssid(&iface);
        let rssi = ICWInterface::rssiValue(&iface);
        let noise = ICWInterface::noiseMeasurement(&iface);
        let tx_rate = ICWInterface::transmitRate(&iface);
        let country = ICWInterface::countryCode(&iface);

        println!("SSID:      {}", nsstring_to_string(ssid));
        println!("BSSID:     {}", nsstring_to_string(bssid));
        println!("RSSI:      {} dBm", rssi);
        println!("Noise:     {} dBm", noise);
        println!("SNR:       {} dB", rssi - noise);
        println!("Tx Rate:   {tx_rate:.1} Mbps");
        println!("Country:   {}", nsstring_to_string(country));

        // Security type
        let security = ICWInterface::security(&iface);
        let security_name = match security {
            0 => "None",
            1 => "WEP",
            2 => "WPA Personal",
            3 => "WPA Personal Mixed",
            4 => "WPA2 Personal",
            6 => "Dynamic WEP",
            7 => "WPA Enterprise",
            8 => "WPA Enterprise Mixed",
            9 => "WPA2 Enterprise",
            11 => "WPA3 Personal",
            12 => "WPA3 Enterprise",
            13 => "WPA3 Transition",
            _ => "Unknown",
        };
        println!("Security:  {security_name} ({security})");

        // Channel info
        let channel = ICWInterface::wlanChannel(&iface);
        if !channel.0.is_null() {
            let ch_num = ICWChannel::channelNumber(&channel);
            let ch_band = ICWChannel::channelBand(&channel);
            let ch_width = ICWChannel::channelWidth(&channel);

            let band_name = match ch_band {
                1 => "2.4 GHz",
                2 => "5 GHz",
                3 => "6 GHz",
                _ => "Unknown",
            };
            let width_name = match ch_width {
                1 => "20 MHz",
                2 => "40 MHz",
                3 => "80 MHz",
                4 => "160 MHz",
                _ => "Unknown",
            };
            println!("\nChannel:");
            println!("  Number: {ch_num}");
            println!("  Band:   {band_name}");
            println!("  Width:  {width_name}");
        }

        // Supported channels (returned as NSSet, convert to NSArray)
        let supported_set = ICWInterface::supportedWLANChannels(&iface);
        if !supported_set.0.is_null() {
            let supported = NSSet_NSExtendedSet::<id>::allObjects(&supported_set);
            let count = INSArray::<id>::count(&supported);
            println!("\nSupported channels ({count} total):");

            let mut ch_2g = Vec::new();
            let mut ch_5g = Vec::new();
            let mut ch_6g = Vec::new();

            for i in 0..count {
                let ch_id = INSArray::<id>::objectAtIndex_(&supported, i);
                let ch = CWChannel(ch_id);
                let num = ICWChannel::channelNumber(&ch);
                let band = ICWChannel::channelBand(&ch);
                match band {
                    1 => ch_2g.push(num),
                    2 => ch_5g.push(num),
                    3 => ch_6g.push(num),
                    _ => {}
                }
            }

            ch_2g.sort();
            ch_2g.dedup();
            ch_5g.sort();
            ch_5g.dedup();
            ch_6g.sort();
            ch_6g.dedup();

            if !ch_2g.is_empty() {
                println!("  2.4 GHz: {:?}", ch_2g);
            }
            if !ch_5g.is_empty() {
                println!("  5 GHz:   {:?}", ch_5g);
            }
            if !ch_6g.is_empty() {
                println!("  6 GHz:   {:?}", ch_6g);
            }
        }
    }

    println!("\nDone.");
}
