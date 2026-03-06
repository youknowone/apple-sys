#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SALRegistrationError = NSInteger;
unsafe extern "C" {
    pub static SALRegistrationErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static mut ServicesAccountLinkingVersionNumber: f64;
}
unsafe extern "C" {
    pub static ServicesAccountLinkingVersionString: [::std::os::raw::c_uchar; 0usize];
}
