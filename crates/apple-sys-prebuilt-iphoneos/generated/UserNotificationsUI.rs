#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UserNotifications::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type UNNotificationContentExtensionMediaPlayPauseButtonType = NSUInteger;
pub type UNNotificationContentExtensionResponseOption = NSUInteger;
pub trait PUNNotificationContentExtension: Sized + std::ops::Deref {
    unsafe fn didReceiveNotification_(&self, notification: UNNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveNotification : notification)
    }
    unsafe fn didReceiveNotificationResponse_completionHandler_(
        &self,
        response: UNNotificationResponse,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveNotificationResponse : response, completionHandler : completion)
    }
    unsafe fn mediaPlay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPlay)
    }
    unsafe fn mediaPause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPause)
    }
    unsafe fn mediaPlayPauseButtonType(
        &self,
    ) -> UNNotificationContentExtensionMediaPlayPauseButtonType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPlayPauseButtonType)
    }
    unsafe fn mediaPlayPauseButtonFrame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPlayPauseButtonFrame)
    }
    unsafe fn mediaPlayPauseButtonTintColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPlayPauseButtonTintColor)
    }
}
pub trait NSExtensionContext_UNNotificationContentExtension: Sized + std::ops::Deref {
    unsafe fn performNotificationDefaultAction(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, performNotificationDefaultAction)
    }
    unsafe fn dismissNotificationContentExtension(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dismissNotificationContentExtension)
    }
    unsafe fn mediaPlayingStarted(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPlayingStarted)
    }
    unsafe fn mediaPlayingPaused(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPlayingPaused)
    }
    unsafe fn notificationActions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationActions)
    }
    unsafe fn setNotificationActions_(&self, notificationActions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotificationActions : notificationActions)
    }
}
