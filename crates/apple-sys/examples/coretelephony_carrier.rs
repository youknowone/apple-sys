//! Query telephony network information using CoreTelephony.
//!
//! Creates CTTelephonyNetworkInfo and queries the current radio access
//! technology and data service identifier.

use apple_sys::CoreTelephony::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreTelephony Network Info ===\n");

        let info = CTTelephonyNetworkInfo::alloc();
        let info_ptr = INSObject::init(&info);
        if info_ptr.is_null() {
            println!("Failed to create CTTelephonyNetworkInfo instance.");
            return;
        }
        let info = CTTelephonyNetworkInfo(info_ptr);

        // Data service identifier
        let data_svc = ICTTelephonyNetworkInfo::dataServiceIdentifier(&info);
        println!("Data service identifier: {}", nsstring_to_string(data_svc));

        // serviceCurrentRadioAccessTechnology - returns NSDictionary<NSString, NSString>
        let radio_dict = ICTTelephonyNetworkInfo::serviceCurrentRadioAccessTechnology(&info);
        if !radio_dict.0.is_null() {
            let keys = NSDictionary_NSExtendedDictionary::<NSString, id>::allKeys(&radio_dict);
            let count = INSArray::<id>::count(&keys);
            println!("\nRadio access technology ({count} service(s)):");

            for i in 0..count {
                let key: id = INSArray::<id>::objectAtIndex_(&keys, i);
                let val = INSDictionary::<NSString, id>::objectForKey_(&radio_dict, key);
                println!("  {}: {}", nsobj_to_string(key), nsobj_to_string(val));
            }
        } else {
            println!("\nRadio access technology: (not available - no cellular connection)");
        }

        // serviceSubscriberCellularProviders - NSDictionary<NSString, CTCarrier>
        let carriers_dict = ICTTelephonyNetworkInfo::serviceSubscriberCellularProviders(&info);
        if !carriers_dict.0.is_null() {
            let keys = NSDictionary_NSExtendedDictionary::<NSString, id>::allKeys(&carriers_dict);
            let count = INSArray::<id>::count(&keys);
            println!("\nCellular providers ({count} carrier(s)):");

            for i in 0..count {
                let key: id = INSArray::<id>::objectAtIndex_(&keys, i);
                let carrier = INSDictionary::<NSString, id>::objectForKey_(&carriers_dict, key);
                if !carrier.is_null() {
                    let carrier = CTCarrier(carrier);
                    let name = ICTCarrier::carrierName(&carrier);
                    let mcc = ICTCarrier::mobileCountryCode(&carrier);
                    let mnc = ICTCarrier::mobileNetworkCode(&carrier);
                    let iso = ICTCarrier::isoCountryCode(&carrier);
                    let voip = ICTCarrier::allowsVOIP(&carrier);

                    println!("  Service '{}':", nsobj_to_string(key));
                    println!("    Carrier name: {}", nsstring_to_string(name));
                    println!("    MCC:          {}", nsstring_to_string(mcc));
                    println!("    MNC:          {}", nsstring_to_string(mnc));
                    println!("    ISO country:  {}", nsstring_to_string(iso));
                    println!("    Allows VoIP:  {}", voip.0);
                }
            }
        } else {
            println!("\nCellular providers: (not available)");
        }
    }

    println!("\nDone.");
}
