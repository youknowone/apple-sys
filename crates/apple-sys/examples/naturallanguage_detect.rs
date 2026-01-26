//! Detect the language of text using NaturalLanguage.framework.
//!
//! Uses NLLanguageRecognizer to identify the dominant language
//! and get probability scores for multiple language hypotheses.

use apple_sys::NaturalLanguage::*;
mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== NaturalLanguage Detection ===\n");

        let samples = [
            ("English", c"The quick brown fox jumps over the lazy dog."),
            ("Korean", c"\xEB\xB9\xA0\xEB\xA5\xB8 \xEA\xB0\x88\xEC\x83\x89 \xEC\x97\xAC\xEC\x9A\xB0\xEA\xB0\x80 \xEA\xB2\x8C\xEC\x9C\xBC\xEB\xA5\xB8 \xEA\xB0\x9C\xEB\xA5\xBC \xEB\x9B\xB0\xEC\x96\xB4\xEB\x84\x98\xEC\x8A\xB5\xEB\x8B\x88\xEB\x8B\xA4."),
            (
                "French",
                c"Le renard brun rapide saute par-dessus le chien paresseux.",
            ),
            (
                "German",
                c"Der schnelle braune Fuchs springt \xC3\xBCber den faulen Hund.",
            ),
            (
                "Spanish",
                c"El r\xC3\xA1pido zorro marr\xC3\xB3n salta sobre el perro perezoso.",
            ),
            ("Chinese", c"\xE6\x95\x8F\xE6\x8D\xB7\xE7\x9A\x84\xE6\xA3\x95\xE8\x89\xB2\xE7\x8B\x90\xE7\x8B\xB8\xE8\xB7\xB3\xE8\xBF\x87\xE4\xBA\x86\xE6\x87\x92\xE6\x83\xB0\xE7\x9A\x84\xE7\x8B\x97\xE3\x80\x82"),
        ];

        for (label, text) in &samples {
            let recognizer = NLLanguageRecognizer::alloc();
            let recognizer = NLLanguageRecognizer(INSObject::init(&recognizer));
            let ns_text = nsstring(text);
            INLLanguageRecognizer::processString_(&recognizer, ns_text);

            // Get dominant language
            let lang = INLLanguageRecognizer::dominantLanguage(&recognizer);
            let lang_str = nsstring_to_string(lang);

            // Get top 3 hypotheses
            let hypotheses = INLLanguageRecognizer::languageHypothesesWithMaximum_(&recognizer, 3);
            let keys = NSDictionary_NSExtendedDictionary::<(), ()>::allKeys(&hypotheses);
            let count = INSArray::<id>::count(&keys);

            print!(
                "  [{}] \"{}...\" -> {}",
                label,
                &text.to_str().unwrap()[..20.min(text.to_str().unwrap().len())],
                lang_str
            );

            if count > 1 {
                print!(" (");
                for i in 0..count {
                    let key = INSArray::<id>::objectAtIndex_(&keys, i);
                    let val = INSDictionary::<(), ()>::objectForKey_(&hypotheses, key);
                    let score = NSNumber(val).doubleValue();
                    let k = nsstring_to_string(NSString(key));
                    if i > 0 {
                        print!(", ");
                    }
                    print!("{}: {:.3}", k, score);
                }
                print!(")");
            }
            println!();
        }
    }

    println!("\nDone.");
}
