#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
unsafe extern "C" {
    pub static mut LockedCameraCaptureVersionNumber: f64;
}
unsafe extern "C" {
    pub static LockedCameraCaptureVersionString: [::std::os::raw::c_uchar; 0usize];
}
unsafe extern "C" {
    pub static NSUserActivityTypeLockedCameraCapture: NSString;
}
