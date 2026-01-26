//! macOS app that opens a window with controls using AppKit.
//!
//! apple-sys provides framework linking and type definitions.
//! objc2 is used for Objective-C message sending.

#![allow(non_upper_case_globals)]

use apple_sys::AppKit::*;
use apple_sys::CoreFoundation::{CGPoint, CGRect, CGSize, INSObject};
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::objc::BOOL;

mod common;
use common::nsstring;

const NSApplicationActivationPolicyRegular: isize = 0;

// NSWindow style mask bits
const NSWindowStyleMaskTitled: usize = 1 << 0;
const NSWindowStyleMaskClosable: usize = 1 << 1;
const NSWindowStyleMaskMiniaturizable: usize = 1 << 2;
const NSWindowStyleMaskResizable: usize = 1 << 3;

// NSBackingStoreType
const NSBackingStoreBuffered: usize = 2;

// NSBezelStyle
const NSBezelStyleRounded: usize = 1;

// NSTextAlignment
const NSTextAlignmentCenter: usize = 1;

// NSControlStateValue
const NSControlStateValueOn: isize = 1;

/// Create a CGRect helper.
fn rect(x: f64, y: f64, w: f64, h: f64) -> CGRect {
    CGRect {
        origin: CGPoint { x, y },
        size: CGSize {
            width: w,
            height: h,
        },
    }
}

fn main() {
    unsafe {
        // Outer autorelease pool
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        // 1. Get shared NSApplication
        let app = NSApplication::sharedApplication();

        // 2. Make this a regular app (shows in Dock, can have menus)
        INSApplication::setActivationPolicy_(
            &app,
            NSApplicationActivationPolicyRegular as NSApplicationActivationPolicy,
        );

        // 3. Create a window
        let style = NSWindowStyleMaskTitled
            | NSWindowStyleMaskClosable
            | NSWindowStyleMaskMiniaturizable
            | NSWindowStyleMaskResizable;

        let window = NSWindow::alloc();
        let window = NSWindow(INSWindow::initWithContentRect_styleMask_backing_defer_(
            &window,
            rect(200.0, 200.0, 480.0, 360.0),
            style as NSWindowStyleMask,
            NSBackingStoreBuffered as NSBackingStoreType,
            BOOL(false),
        ));

        INSWindow::setTitle_(&window, nsstring(c"Hello apple-sys"));

        // 4. Get the content view
        let content_view = INSWindow::contentView(&window);

        // 5. Label (NSTextField used as a static label)
        let label = NSTextField::alloc();
        let label = NSTextField(INSControl::initWithFrame_(
            &label,
            rect(40.0, 280.0, 400.0, 24.0),
        ));
        label.setStringValue_(nsstring(c"Welcome to apple-sys!"));
        INSTextField::setEditable_(&label, BOOL(false));
        INSTextField::setBezeled_(&label, BOOL(false));
        INSTextField::setDrawsBackground_(&label, BOOL(false));
        INSTextField::setSelectable_(&label, BOOL(false));
        INSControl::setAlignment_(&label, NSTextAlignmentCenter as NSTextAlignment);
        content_view.addSubview_(NSView(label.0));

        // 6. Text field (editable)
        let tf = NSTextField::alloc();
        let tf = NSTextField(INSControl::initWithFrame_(
            &tf,
            rect(40.0, 230.0, 400.0, 24.0),
        ));
        INSTextField::setPlaceholderString_(&tf, nsstring(c"Type something here..."));
        content_view.addSubview_(NSView(tf.0));

        // 7. Push button
        let button = NSButton::alloc();
        let button = NSButton(INSControl::initWithFrame_(
            &button,
            rect(190.0, 180.0, 100.0, 32.0),
        ));
        button.setTitle_(nsstring(c"Click Me"));
        button.setBezelStyle_(NSBezelStyleRounded as _);
        content_view.addSubview_(NSView(button.0));

        // 8. Checkbox (NSButton with switchStyle)
        let checkbox = NSButton::alloc();
        let checkbox = NSButton(INSControl::initWithFrame_(
            &checkbox,
            rect(40.0, 140.0, 200.0, 24.0),
        ));
        checkbox.setTitle_(nsstring(c"Enable option"));
        checkbox.setButtonType_(3); // NSSwitchButton
        checkbox.setState_(NSControlStateValueOn as i64);
        content_view.addSubview_(NSView(checkbox.0));

        // 9. Popup button (dropdown)
        let popup = NSPopUpButton::alloc();
        let popup = NSPopUpButton(INSPopUpButton::initWithFrame_pullsDown_(
            &popup,
            rect(40.0, 100.0, 200.0, 28.0),
            BOOL(false),
        ));
        popup.addItemWithTitle_(nsstring(c"CoreFoundation"));
        popup.addItemWithTitle_(nsstring(c"AppKit"));
        popup.addItemWithTitle_(nsstring(c"UIKit"));
        content_view.addSubview_(NSView(popup.0));

        // 10. Slider
        let slider = NSSlider::alloc();
        let slider = NSSlider(INSControl::initWithFrame_(
            &slider,
            rect(40.0, 55.0, 400.0, 24.0),
        ));
        slider.setMinValue_(0.0);
        slider.setMaxValue_(100.0);
        slider.setDoubleValue_(50.0);
        content_view.addSubview_(NSView(slider.0));

        // 11. Progress indicator
        let progress = NSProgressIndicator::alloc();
        let progress = NSProgressIndicator(INSView::initWithFrame_(
            &progress,
            rect(40.0, 20.0, 400.0, 20.0),
        ));
        progress.setMinValue_(0.0);
        progress.setMaxValue_(100.0);
        progress.setDoubleValue_(65.0);
        content_view.addSubview_(NSView(progress.0));

        // 12. Show the window and run
        INSWindow::makeKeyAndOrderFront_(&window, std::ptr::null_mut());
        INSApplication::activateIgnoringOtherApps_(&app, BOOL(true));
        INSApplication::run(&app);
    }
}
