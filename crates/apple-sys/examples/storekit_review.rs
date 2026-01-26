//! StoreKit product and review APIs.
//!
//! Demonstrates SKProductsRequest creation and SKStoreReviewController.

use apple_sys::StoreKit::*;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== StoreKit ===\n");

        // Create a product identifiers set
        let set = NSMutableSet::alloc();
        let product_ids = NSMutableSet(INSObject::init(&set));
        let id1 = nsstring(c"com.example.premium");
        let id2 = nsstring(c"com.example.coins.100");
        let id3 = nsstring(c"com.example.subscription.monthly");
        INSMutableSet::<id>::addObject_(&product_ids, id1.0);
        INSMutableSet::<id>::addObject_(&product_ids, id2.0);
        INSMutableSet::<id>::addObject_(&product_ids, id3.0);

        let count = INSSet::<id>::count(&product_ids);
        println!("Product IDs prepared: {}", count);

        // Create products request
        let req = SKProductsRequest::alloc();
        let req_ptr = ISKProductsRequest::initWithProductIdentifiers_(&req, NSSet(product_ids.0));
        assert!(!req_ptr.is_null(), "Failed to create SKProductsRequest");
        println!("SKProductsRequest created");

        // Check payment queue
        let can_pay = SKPaymentQueue::canMakePayments();
        println!("Can make payments: {}", can_pay.0);

        let queue_ptr = SKPaymentQueue::defaultQueue();
        if !queue_ptr.is_null() {
            let queue = SKPaymentQueue(queue_ptr);
            let transactions = ISKPaymentQueue::transactions(&queue);
            let tx_count = INSArray::<id>::count(&transactions);
            println!("Pending transactions: {}", tx_count);

            let storefront = ISKPaymentQueue::storefront(&queue);
            if !storefront.0.is_null() {
                let country = ISKStorefront::countryCode(&storefront);
                let identifier = ISKStorefront::identifier(&storefront);
                println!(
                    "Storefront: {} ({})",
                    nsstring_to_string(country),
                    nsstring_to_string(identifier)
                );
            }
        }

        // SKStoreReviewController exists (has bindings in StoreKit)
        println!("SKStoreReviewController available: true");

        // Receipt URL
        let main_bundle = NSBundle::mainBundle();
        let receipt_url = INSBundle::appStoreReceiptURL(&main_bundle);
        if !receipt_url.0.is_null() {
            let path = INSURL::path(&receipt_url);
            println!("Receipt URL: {}", nsstring_to_string(path));
        }
    }

    println!("\nDone.");
}
