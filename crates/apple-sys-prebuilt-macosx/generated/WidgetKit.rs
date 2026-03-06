#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
unsafe extern "C" {
    pub static WGWidgetUserInfoKeyKind: NSString;
}
unsafe extern "C" {
    pub static WGWidgetUserInfoKeyFamily: NSString;
}
unsafe extern "C" {
    pub static WGWidgetUserInfoKeyActivityID: NSString;
}
unsafe extern "C" {
    pub static NSUserActivityTypeLiveActivity: NSString;
}
