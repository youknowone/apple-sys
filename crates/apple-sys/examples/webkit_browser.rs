//! Simple web browser using WebKit's WKWebView.
//!
//! Opens a window displaying https://www.apple.com using AppKit and WebKit.

use apple_sys::AppKit::*;
use apple_sys::CoreFoundation::{CGPoint, CGRect, CGSize, INSObject};
use apple_sys::Foundation::{INSURL, INSURLRequest, NSAutoreleasePool, NSURL, NSURLRequest};
use apple_sys::WebKit::*;
use apple_sys::objc::BOOL;

mod common;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        // Initialize NSApplication
        let app = NSApplication::sharedApplication();

        // Create window
        let rect = CGRect {
            origin: CGPoint { x: 200.0, y: 200.0 },
            size: CGSize {
                width: 800.0,
                height: 600.0,
            },
        };

        let window_alloc = NSWindow::alloc();
        let window_ptr = INSWindow::initWithContentRect_styleMask_backing_defer_(
            &window_alloc,
            rect,
            15, // titled | closable | miniaturizable | resizable
            2,  // buffered
            BOOL(false),
        );
        let window = NSWindow(window_ptr);

        let title = nsstring(c"apple-sys WebKit Browser");
        INSWindow::setTitle_(&window, title);

        // Create WKWebView with configuration
        let config = WKWebViewConfiguration::alloc();
        let config_ptr = INSObject::init(&config);
        let webview_alloc = WKWebView::alloc();
        let webview_ptr = IWKWebView::initWithFrame_configuration_(
            &webview_alloc,
            rect,
            WKWebViewConfiguration(config_ptr),
        );
        let webview = WKWebView(webview_ptr);

        // Load URL
        let url_str = nsstring(c"https://www.apple.com");
        let url = NSURL(NSURL::URLWithString_(url_str));

        let request = NSURLRequest(NSURLRequest::requestWithURL_(url));
        IWKWebView::loadRequest_(&webview, request);

        // Add webview to window
        INSWindow::setContentView_(&window, NSView(webview.0));
        INSWindow::makeKeyAndOrderFront_(&window, std::ptr::null_mut());

        // Activate app and run
        INSApplication::setActivationPolicy_(&app, 0); // regular
        INSApplication::activateIgnoringOtherApps_(&app, BOOL(true));

        println!("WebKit browser launched. Loading https://www.apple.com ...");
        println!("Close the window or press Ctrl+C to exit.");

        INSApplication::run(&app);
    }
}
