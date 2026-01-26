//! Verify Social framework bindings by creating an SLRequest
//! and exercising SLComposeServiceViewController.

use apple_sys::Social::*;
use objc2::msg_send;

mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Social Framework ===\n");

        // SLRequest - create a request for Twitter (deprecated but tests bindings)
        println!("--- SLRequest ---");
        let url_str = nsstring(c"https://api.twitter.com/1.1/statuses/update.json");
        let url = NSURL(NSURL::URLWithString_(url_str));
        let params = NSDictionary(std::ptr::null_mut());
        let request = SLRequest::requestForServiceType_requestMethod_URL_parameters_(
            SLServiceTypeTwitter,
            0, // SLRequestMethodGET
            url,
            params,
        );
        if !request.0.is_null() {
            println!("Request: {}", nsobj_to_string(request.0));

            let method = ISLRequest::requestMethod(&request);
            println!("  requestMethod: {method}");

            let req_url = ISLRequest::URL(&request);
            println!("  URL: {}", nsobj_to_string(req_url.0));

            let req_params = ISLRequest::parameters(&request);
            println!("  parameters: {}", nsobj_to_string(req_params.0));

            let account = ISLRequest::account(&request);
            println!("  account: {}", nsobj_to_string(account.0));

            let prepared = ISLRequest::preparedURLRequest(&request);
            println!("  preparedURLRequest: {}", nsobj_to_string(prepared.0));
        } else {
            println!("Failed to create SLRequest.");
        }

        // Service type constants
        println!("\n--- Service Type Constants ---");
        println!("  Twitter: {}", nsstring_to_string(SLServiceTypeTwitter));
        println!("  Facebook: {}", nsstring_to_string(SLServiceTypeFacebook));
        println!(
            "  SinaWeibo: {}",
            nsstring_to_string(SLServiceTypeSinaWeibo)
        );
        println!(
            "  TencentWeibo: {}",
            nsstring_to_string(SLServiceTypeTencentWeibo)
        );
        println!("  LinkedIn: {}", nsstring_to_string(SLServiceTypeLinkedIn));

        // SLComposeServiceViewController
        println!("\n--- SLComposeServiceViewController ---");
        let compose = SLComposeServiceViewController::alloc();
        let compose_ptr: id = msg_send![&*compose.0, init];
        if !compose_ptr.is_null() {
            let compose = SLComposeServiceViewController(compose_ptr);
            println!("ComposeServiceVC: {}", nsobj_to_string(compose_ptr));

            let content = ISLComposeServiceViewController::contentText(&compose);
            println!("  contentText: {}", nsstring_to_string(content));

            let placeholder = ISLComposeServiceViewController::placeholder(&compose);
            println!("  placeholder: {}", nsstring_to_string(placeholder));

            let chars_remaining = ISLComposeServiceViewController::charactersRemaining(&compose);
            println!(
                "  charactersRemaining: {}",
                nsobj_to_string(chars_remaining.0)
            );

            let valid = ISLComposeServiceViewController::isContentValid(&compose);
            println!("  isContentValid: {}", valid.0);
        } else {
            println!("Failed to create SLComposeServiceViewController.");
        }
    }

    println!("\nDone.");
}
